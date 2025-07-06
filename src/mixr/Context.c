#include "mixr/Context.h"

#include <stdlib.h>

typedef struct
{
    uint32_t SampleRate;
} Context;

MxResult mxCreateContext(const MxContextInfo* info, MxContext** context)
{
    Context* ctx = malloc(sizeof(Context));
    ctx->SampleRate = info->sampleRate;

    *context = (MxContext*) ctx;
    return MX_RESULT_OK;
}


void mxDestroyContext(MxContext* context)
{
    Context* ctx = (Context*) context;
    free(ctx);
}
