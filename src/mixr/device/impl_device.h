#ifndef IMPL_DEVICE_H
#define IMPL_DEVICE_H

typedef MxContext (*Impl_GetContextFn)(void*);

typedef struct
{
    Impl_GetContextFn getDevice;
} Impl_Device;

#endif
