#ifndef ALSA_DEVICE_H
#define ALSA_DEVICE_H

#include "../deviceimpl.h"
#include <alsa/asoundlib.h>

typedef struct
{
    snd_pcm_t* device;
} AlsaDevice;

MxResult mxALSACreateDevice(const MxDeviceInfo *info, DeviceImpl **impl);

#endif