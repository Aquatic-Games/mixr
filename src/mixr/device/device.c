#include <stdlib.h>

#include "mixr/device.h"
#include "deviceimpl.h"
#include "../internal.h"

#if defined(__linux__)
#include "ALSA/alsa_device.h"
#endif

MxResult mxCreateDevice(const MxDeviceInfo* info, MxDevice** device)
{
    DeviceImpl* impl;
    MxResult result = MX_RESULT_UNKNOWN_ERROR;

#if defined(_WIN32)
    mxSetErrorString("Windows device not yet implemented.");
    return MX_RESULT_GENERAL_FAILURE;
#elif defined(__linux__)
    result = mxALSACreateDevice(info, &impl);
#endif

    *device = (MxDevice*) impl;
    return result;
}

void mxDestroyDevice(MxDevice* device)
{
    DeviceImpl* impl = (DeviceImpl*) device;
    impl->destroyDevice(impl->deviceData);
    mxDestroyContext(impl->context);
    free(impl);
}

MxContext* mxDeviceGetContext(MxDevice* device)
{
    const DeviceImpl* impl = (DeviceImpl*) device;
    return impl->context;
}
