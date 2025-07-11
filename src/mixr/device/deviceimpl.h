#ifndef IMPL_DEVICE_H
#define IMPL_DEVICE_H

#include "mixr/device.h"

typedef struct
{
    void* deviceData;
    MxContext* context;

    void (*destroyDevice)(void* device);
} DeviceImpl;

#endif
