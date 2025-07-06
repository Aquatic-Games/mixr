#include "mixr/Context.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "utils/vector.h"

typedef struct
{
    uint8_t* data;
    size_t length;
} Buffer;

typedef struct
{
    uint32_t sampleRate;
    const char* errorMsg;
    Vector buffers;
} Context;

MxResult mxCreateContext(const MxContextInfo* info, MxContext** context)
{
    Context* ctx = malloc(sizeof(Context));
    ctx->sampleRate = info->sampleRate;
    ctx->errorMsg = NULL;
    ctx->buffers = VectorCreate(sizeof(Buffer), 0);

    *context = (MxContext*) ctx;
    return MX_RESULT_OK;
}

void mxDestroyContext(MxContext* context)
{
    Context* ctx = (Context*) context;
    VectorDestroy(&ctx->buffers);
    free(ctx);
}

const char* mxGetLastErrorString(MxContext *context)
{
    const Context* ctx = (Context*) context;
    return ctx->errorMsg;
}

MxResult mxCreateBuffer(MxContext* context, const uint8_t* data, const size_t length, MxBuffer* buffer)
{
    Context* ctx = (Context*) context;
    Vector* buffers = &ctx->buffers;

    uint8_t* bufferData = malloc(length * sizeof(uint8_t*));
    if (!bufferData)
    {
        ctx->errorMsg = "Could not allocate buffer data.";
        return MX_RESULT_OUT_OF_MEMORY;
    }

    memcpy(bufferData, data, length);

    Buffer buf;
    buf.data = bufferData;
    buf.length = length;

    const size_t currentId = buffers->length;

    if (!VectorAppend(buffers, &buf))
    {
        ctx->errorMsg = "Buffers vector ran out of space. This is likely a bug.";
        return MX_RESULT_OUT_OF_MEMORY;
    }

    buffer->id = currentId;

    return MX_RESULT_OK;
}

MxResult mxDestroyBuffer(MxContext* context, MxBuffer buffer)
{

}
