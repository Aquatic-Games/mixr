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

    MxResult mxCreateContext(const MxContextInfo *info, MxContext **context);
    void mxDestroyContext(MxContext *context);

    const char* mxGetLastErrorString(MxContext *context);

#ifdef __cplusplus
}
#endif
#endif
