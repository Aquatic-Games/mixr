#include <stdlib.h>

#include "mixr/Device.h"
#include "deviceimpl.h"

#include "SDL/sdl_device.h"

MxResult mxCreateDevice(const MxDeviceInfo* info, MxDevice** device)
{
    DeviceImpl* impl;
    const MxResult result = mxSDLCreateDevice(info, &impl);
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
