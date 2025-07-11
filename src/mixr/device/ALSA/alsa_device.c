#include "alsa_device.h"
#include "../../internal.h"

// TODO: Set error string with params
#define MX_ALSA_CHECK(Operation) {\
    int res = Operation;\
    if (res < 0) {\
        printf("ALSA operation %s failed: %s\n", #Operation, snd_strerror(res));\
        mxSetErrorString("ALSA error: see stdout.");\
        return MX_RESULT_UNKNOWN_ERROR;\
    }\
}

void* mxALSAThread(void* impl)
{
    DeviceImpl* dev = (DeviceImpl*) impl;
    AlsaDevice* alsa = (AlsaDevice*) dev->deviceData;
    uint8_t* buffer = malloc(alsa->bufferSize);

    while (alsa->threadAlive)
    {
        mxMixInterleavedStereo(dev->context, (float*) buffer, alsa->bufferSize / 4);
        snd_pcm_sframes_t frames = snd_pcm_writei(alsa->device, buffer, alsa->bufferSizeFrames);

        if (frames < 0)
            frames = snd_pcm_recover(alsa->device, frames, 0);
        if (frames < 0)
        {
            printf("FAILURE: %s", snd_strerror(frames));
            return NULL;
        }

        if (frames > 0 && frames < alsa->bufferSizeFrames)
        {
            printf("FAILURE.");
            return NULL;
        }
    }

    free(buffer);

    return NULL;
}

void mxALSADestroyDevice(void* device)
{
    AlsaDevice* dev = (AlsaDevice*) device;
    dev->threadAlive = false;
    // TODO: This can be done better. This will wait for ALSA to finish as well which may cause the thread to block for
    //       a few ms.
    pthread_join(dev->thread, NULL);
    snd_pcm_close(dev->device);
}

MxResult mxALSACreateDevice(const MxDeviceInfo *info, DeviceImpl **impl)
{
    DeviceImpl* dev = malloc(sizeof(DeviceImpl));
    *impl = dev;
    dev->destroyDevice = mxALSADestroyDevice;

    AlsaDevice* device = malloc(sizeof(AlsaDevice));
    dev->deviceData = device;

    snd_pcm_t* alsa;
    MX_ALSA_CHECK(snd_pcm_open(&alsa, "default", SND_PCM_STREAM_PLAYBACK, 0));
    device->device = alsa;

    snd_pcm_hw_params_t* params;
    snd_pcm_hw_params_alloca(&params);

    MX_ALSA_CHECK(snd_pcm_hw_params_any(alsa, params));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_access(alsa, params, SND_PCM_ACCESS_RW_INTERLEAVED));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_format(alsa, params, SND_PCM_FORMAT_FLOAT));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_rate(alsa, params, info->sampleRate, 0));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_channels(alsa, params, 2));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_period_size(alsa, params, 256, 0));
    MX_ALSA_CHECK(snd_pcm_hw_params_set_periods(alsa, params, 2, 0));

    MX_ALSA_CHECK(snd_pcm_hw_params(alsa, params));

    snd_pcm_uframes_t bufferSizeFrames;
    MX_ALSA_CHECK(snd_pcm_hw_params_get_buffer_size(params, &bufferSizeFrames));
    device->bufferSizeFrames = bufferSizeFrames;

    //snd_pcm_hw_params_free(params);

    device->bufferSize = snd_pcm_frames_to_bytes(alsa, (snd_pcm_sframes_t) bufferSizeFrames);

    printf("frames: %lu, size: %lu\n", bufferSizeFrames, device->bufferSize);

    MxContextInfo contextInfo;
    contextInfo.sampleRate = info->sampleRate;

    MxContext* context;
    const MxResult result = mxCreateContext(&contextInfo, &context);
    if (result != MX_RESULT_OK)
        return result;

    dev->context = context;

    device->threadAlive = true;
    if (pthread_create(&device->thread, NULL, mxALSAThread, dev))
    {
        mxSetErrorString("pthread failed to create a thread.");
        return MX_RESULT_GENERAL_FAILURE;
    }

    return MX_RESULT_OK;
}