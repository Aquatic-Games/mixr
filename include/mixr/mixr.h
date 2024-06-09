#pragma once

#include <stdint.h>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxContext MxContext;

    void mxCreateContext(uint32_t sampleRate, MxContext **pContext);
    void mxDestroyContext(MxContext* context);

#ifdef __cplusplus
}
#endif