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

    size_t temp;
} Context;

MxResult mxCreateContext(const MxContextInfo* info, MxContext** context)
{
    Context* ctx = malloc(sizeof(Context));
    ctx->sampleRate = info->sampleRate;
    ctx->errorMsg = NULL;
    ctx->buffers = VectorCreate(sizeof(Buffer), 0);
    ctx->temp = 0;

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

    uint8_t* bufferData = malloc(length);
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

float GetSample(const uint8_t* buffer, const size_t pos, const MxDataType type)
{
    switch (type)
    {
        /*case MX_DATA_TYPE_U8:
            return (float) (int8_t) (buffer[pos] - INT8_MAX) / INT8_MAX;*/
        case MX_DATA_TYPE_I8:
            return (float) (int8_t) buffer[pos] / INT8_MAX;
        case MX_DATA_TYPE_I16:
            return (float) (int16_t) (buffer[pos] | (buffer[pos + 1] << 8)) / INT16_MAX;
        case MX_DATA_TYPE_I32:
            return (float) (buffer[pos] | (buffer[pos + 1] << 8) | (buffer[pos + 2] << 16) | (buffer[pos + 3] << 24)) / INT32_MAX;
        case MX_DATA_TYPE_F32:
            uint32_t sample = (buffer[pos] | (buffer[pos + 1] << 8) | (buffer[pos + 2] << 16) | (buffer[pos + 3] << 24));
            return *(float*) &sample;
    }

    return 0;
}

void mxMixInterleavedStereo(MxContext *context, float* buffer, const size_t length)
{
    Context* ctx = (Context*) context;
    const Buffer* buf = VectorGet(&ctx->buffers, 0);
    const uint8_t* data = buf->data;

    for (size_t i = 0; i < length; i += 2)
    {
        int position = ctx->temp * 4;

        buffer[i] = GetSample(data, position, MX_DATA_TYPE_I16);
        buffer[i + 1] = GetSample(data, position + 2, MX_DATA_TYPE_I16);

        ctx->temp += 1;
    }
}
