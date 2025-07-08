#include "sdl_device.h"

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

DeviceImpl* mxSDLCreateDevice(const MxDeviceInfo* info)
{
    DeviceImpl* impl = malloc(sizeof(DeviceImpl));
    impl->destroyDevice = SDL_DestroyDevice;

    SDL_Init(SDL_INIT_AUDIO);

    const SDL_AudioSpec inSpec =
    {
        .freq = (int) info->sampleRate,
        .format = AUDIO_F32,
        .channels = 2,
        .samples = 512,
        .callback = AudioCallback,
        .userdata = impl
    };

    SDLDevice* deviceData = malloc(sizeof(SDLDevice));
    impl->deviceData = deviceData;

    SDL_AudioSpec outSpec;
    deviceData->id = SDL_OpenAudioDevice(NULL, 0, &inSpec, &outSpec, SDL_AUDIO_ALLOW_FREQUENCY_CHANGE | SDL_AUDIO_ALLOW_SAMPLES_CHANGE);

    if (!deviceData->id)
        printf("%s", SDL_GetError());

    const MxContextInfo contextInfo =
    {
        .sampleRate = (uint32_t) outSpec.freq
    };

    mxCreateContext(&contextInfo, &impl->context);

    SDL_PauseAudioDevice(deviceData->id, 0);

    return impl;
}
