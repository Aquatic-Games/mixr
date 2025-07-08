#include "sdl_device.h"

#include "../../internal.h"

void AudioCallback(void* userData, Uint8* buffer, int length)
{
    MxContext* context = ((DeviceImpl*) userData)->context;
    mxMixInterleavedStereo(context, (float*) buffer, length / 4);
}

void SDL_DestroyDevice(void* device)
{
    SDLDevice* sdlDevice = (SDLDevice*) device;
    SDL_CloseAudioDevice(sdlDevice->id);
    free(sdlDevice);
}

MxResult mxSDLCreateDevice(const MxDeviceInfo* info, DeviceImpl** impl)
{
    DeviceImpl* dev = malloc(sizeof(DeviceImpl));
    *impl = dev;
    dev->destroyDevice = SDL_DestroyDevice;

    if (SDL_Init(SDL_INIT_AUDIO) < 0)
    {
        mxSetErrorString(SDL_GetError());
        return MX_RESULT_GENERAL_FAILURE;
    }

    const SDL_AudioSpec inSpec =
    {
        .freq = (int) info->sampleRate,
        .format = AUDIO_F32,
        .channels = 2,
        .samples = 512,
        .callback = AudioCallback,
        .userdata = dev
    };

    SDLDevice* deviceData = malloc(sizeof(SDLDevice));
    dev->deviceData = deviceData;

    SDL_AudioSpec outSpec;
    deviceData->id = SDL_OpenAudioDevice(NULL, 0, &inSpec, &outSpec, SDL_AUDIO_ALLOW_FREQUENCY_CHANGE | SDL_AUDIO_ALLOW_SAMPLES_CHANGE);

    if (!deviceData->id)
    {
        mxSetErrorString(SDL_GetError());
        return MX_RESULT_UNKNOWN_ERROR;
    }

    const MxContextInfo contextInfo =
    {
        .sampleRate = (uint32_t) outSpec.freq
    };

    mxCreateContext(&contextInfo, &dev->context);

    SDL_PauseAudioDevice(deviceData->id, 0);

    return MX_RESULT_OK;
}
