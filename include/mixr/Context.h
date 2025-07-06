#ifndef MIXR_CONTEXT_H
#define MIXR_CONTEXT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#include "Common.h"

#define MIXR_C_API

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct
    {
        uint32_t sampleRate;
    } MxContextInfo;

    typedef struct MxContext MxContext;

    typedef struct
    {
        size_t id;
    } MxBuffer;

    MxResult mxCreateContext(const MxContextInfo *info, MxContext **context);
    void mxDestroyContext(MxContext *context);
    const char* mxGetLastErrorString(MxContext *context);

    MxResult mxCreateBuffer(MxContext *context, const uint8_t* data, size_t length, MxBuffer *buffer);
    MxResult mxDestroyBuffer(MxContext *context, MxBuffer buffer);

    void mxMixInterleavedStereo(MxContext *context, float* buffer, size_t length);

#ifdef __cplusplus
}
#endif
#endif
