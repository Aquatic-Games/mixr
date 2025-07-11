#ifndef ALSA_DEVICE_H
#define ALSA_DEVICE_H

#include "../deviceimpl.h"
#include <alsa/asoundlib.h>
#include <pthread.h>

typedef struct
{
    snd_pcm_t* device;
    size_t bufferSizeFrames;
    size_t bufferSize;
    pthread_t thread;
    bool threadAlive;
} AlsaDevice;

MxResult mxALSACreateDevice(const MxDeviceInfo *info, DeviceImpl **impl);

#endif