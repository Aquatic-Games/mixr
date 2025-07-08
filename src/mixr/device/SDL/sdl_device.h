#ifndef SDL_DEVICE_H
#define SDL_DEVICE_H

#include "../deviceimpl.h"
#include <SDL2/SDL.h>

typedef struct
{
    SDL_AudioDeviceID id;
} SDLDevice;

DeviceImpl* mxSDLCreateDevice(const MxDeviceInfo *info);

#endif
