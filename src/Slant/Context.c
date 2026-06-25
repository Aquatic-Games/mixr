#include "Slant/Context.h"

#include <stdlib.h>

#define SL_ARRAYLIST(type, name) type *name;\
    size_t name##Length;\
    size_t name##Capacity;

#define SL_ARRAYLIST_NEW(type, var, capacity) var = (type *) malloc(capacity * sizeof(type));\
    if (!var) return SL_RESULT_OUT_OF_MEMORY;\
    var##Length = 0;\
    var##Capacity = capacity;

#define SL_ARRAYLIST_APPEND(type, var, value) {\
    if (var##Length + 1 >= var##Capacity) {\
        var##Capacity <<= 1;\
        type *newList = realloc(var, var##Capacity * sizeof(type));\
        if (!newList) return SL_RESULT_OUT_OF_MEMORY;\
        var = newList;\
    }\
    var[var##Length] = value;\
    var##Length++;\
}

typedef struct
{
    
} SlBufferImpl;

typedef struct
{
    uint32_t sampleRate;
    SL_ARRAYLIST(SlBufferImpl, buffers)
} SlContextImpl;

SlResult slCreateContext(const SlContextInfo *info, SlContext **context)
{
    if (info->sampleRate < 1)
        return SL_RESULT_INVALID_VALUE;
    
    SlContextImpl *ctx = (SlContextImpl *) malloc(sizeof(SlContextImpl));
    if (!ctx)
        return SL_RESULT_OUT_OF_MEMORY;
    
    ctx->sampleRate = info->sampleRate;
    SL_ARRAYLIST_NEW(SlBufferImpl, ctx->buffers, 16);
    
    *context = (SlContext *) ctx;
    return SL_RESULT_OK;
}

SlResult slDestroyContext(SlContext *context)
{
    if (!context)
        return SL_RESULT_INVALID_VALUE;
    
    SlContextImpl *ctx = (SlContextImpl *) context;
    free(ctx->buffers);
    free(ctx);
    
    return SL_RESULT_OK;
}

SlResult slCreateBuffer(SlContext *context, const SlBufferInfo *info, SlBuffer *buffer)
{
    if (!context || !info)
        return SL_RESULT_INVALID_VALUE;
    
    SlContextImpl *ctx = (SlContextImpl *) context;
    
    SlBufferImpl bufferImpl = {
        
    };
    
    size_t bufferID = ctx->buffersLength;
    SL_ARRAYLIST_APPEND(SlBufferImpl, ctx->buffers, bufferImpl);
    
    *buffer = (SlBuffer) bufferID;
    return SL_RESULT_OK;
}

/*SlResult slDestroyBuffer(SlContext *context, SlBuffer buffer)
{
    if (!context)
        return SL_RESULT_INVALID_VALUE;
    
    SlContextImpl *ctx = (SlContextImpl *) context;
    
}*/
