#include "mixr/mixr.h"
#include "../Impl/Impl.h"

#include <SDL.h>
#include <stdexcept>

using namespace mixr;

struct SDLDevice {
    SDL_AudioDeviceID Device;
    Impl* Impl;
};

void AudioCallback(void* userData, Uint8* buffer, int length) {
    ((SDLDevice*) userData)->Impl->MixToStereoF32Buffer((float*) buffer, length / 4);
}

void mxCreateSDLDevice(uint32_t sampleRate, uint16_t periodSize, MxDevice **pDevice) {
    if (SDL_Init(SDL_INIT_AUDIO) < 0) {
        throw std::runtime_error("Failed to initialize SDL.");
    }

    SDLDevice* deviceStruct = new SDLDevice();

    SDL_AudioSpec spec {};
    spec.freq = static_cast<int>(sampleRate);
    spec.samples = periodSize;
    spec.channels = 2;
    spec.format = AUDIO_F32;
    spec.userdata = deviceStruct;
    spec.callback = AudioCallback;

    auto device = SDL_OpenAudioDevice(nullptr, 0, &spec, nullptr, 0);
    Impl* impl = new Impl(sampleRate);

    deviceStruct->Device = device;
    deviceStruct->Impl = impl;

    SDL_PauseAudioDevice(device, 0);

    *pDevice = (MxDevice*) deviceStruct;
}

void mxDeviceGetContext(MxDevice *device, MxContext **pContext) {
    SDLDevice* sdlDevice = (SDLDevice*) device;
    *pContext = (MxContext*) sdlDevice->Impl;
}

void mxDestroyDevice(MxDevice *device) {
    SDLDevice* sdlDevice = (SDLDevice*) device;

    SDL_CloseAudioDevice(sdlDevice->Device);
    SDL_Quit();

    delete sdlDevice->Impl;
    delete sdlDevice;
}