#include "Slant/Context.h"

#include <stdlib.h>

typedef struct
{
    uint32_t sampleRate;
} SlantContext;

SlResult slCreateContext(const SlContextInfo *info, SlContext **context)
{
    if (info->sampleRate < 1)
        return SL_RESULT_INVALID_VALUE;
    
    SlantContext *ctx = (SlantContext *) malloc(sizeof(SlantContext));
    if (!ctx)
        return SL_RESULT_OUT_OF_MEMORY;
    
    ctx->sampleRate = info->sampleRate;
    
    *context = (SlContext *) ctx;
    return SL_RESULT_OK;
}

SlResult slDestroyContext(SlContext *context)
{
    if (!context)
        return SL_RESULT_INVALID_VALUE;
    
    SlantContext *ctx = (SlantContext *) context;
    free(ctx);
}
