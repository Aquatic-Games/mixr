#ifndef MIXR_DEVICE_H
#define MIXR_DEVICE_H

#include "Common.h"
#include "Context.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct
    {
        uint32_t sampleRate;
    } MxDeviceInfo;

    typedef struct MxDevice MxDevice;

    MxResult mxCreateDevice(const MxDeviceInfo *info, MxDevice *device);
    void mxDestroyDevice(MxDevice *device);
    MxContext *mxDeviceGetContext(MxDevice *device);

#ifdef __cplusplus
}
#endif
#endif
