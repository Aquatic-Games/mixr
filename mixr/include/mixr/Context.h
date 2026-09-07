#ifndef MIXR_CONTEXT_H
#define MIXR_CONTEXT_H

#ifdef __cplusplus
extern "C" {
#endif

#include "Common.h"

#include <stdint.h>

typedef struct MxContext MxContext;

typedef struct MxContextInfo
{
    uint32_t sampleRate;
} MxContextInfo;

MxResult mxCreateContext(const MxContextInfo *info, MxContext **context);

#ifdef __cplusplus
}
#endif

#endif
