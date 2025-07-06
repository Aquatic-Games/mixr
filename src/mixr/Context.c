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

typedef struct SourceQueueNode
{
    size_t buffer;
    struct SourceQueueNode* next;
    struct SourceQueueNode* prev;
} SourceQueueNode;

typedef struct
{
    MxAudioFormat format;
    // The number of bytes per channel.
    size_t channelStride;
    // In mixr, one "sample" is a full stereo buffer.
    // EXAMPLE: format.dataType = F32. format.channels = 2. channelStride = 4, sampleStride = 8.
    //          format.dataType = I16. format.channels = 1. channelStride = 2, sampleStride = 2.
    size_t sampleStride;
    // Corrects for the main sample rate of the context.
    // EXAMPLE: context.sampleRate = 48000. format.sampleRate = 44100. speedCorrection ~= 0.9875
    double speedCorrection;

    SourceQueueNode* queue;
    size_t queueLength;

    size_t bufferLength;
    uint8_t* sourceBuffer;

    bool playing;

    size_t position;
    double finePosition;
} Source;

typedef struct
{
    uint32_t sampleRate;
    const char* errorMsg;

    Vector buffers;
    Vector sources;
} Context;

MxResult mxCreateContext(const MxContextInfo* info, MxContext** context)
{
    Context* ctx = malloc(sizeof(Context));
    ctx->sampleRate = info->sampleRate;
    ctx->errorMsg = NULL;
    ctx->buffers = VectorCreate(sizeof(Buffer), 0);
    ctx->sources = VectorCreate(sizeof(Source), 0);

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

MxResult mxCreateSource(MxContext* context, const MxSourceInfo* info, MxSource* source)
{
    Context* ctx = (Context*) context;

    Source src;

    src.format = info->format;

    switch (info->format.dataType)
    {
        case MX_DATA_TYPE_I8:
            src.channelStride = 1;
            break;
        case MX_DATA_TYPE_I16:
            src.channelStride = 2;
            break;
        case MX_DATA_TYPE_I32:
        case MX_DATA_TYPE_F32:
            src.channelStride = 4;
            break;
        default:
            // TODO: Return error.
            src.channelStride = 0;
    }

    if (info->format.channels > 2)
    {
        ctx->errorMsg = "Maximum 2 channels allowed for a source.";
        return MX_RESULT_INVALID_NUM_CHANNELS;
    }

    src.sampleStride = src.channelStride * info->format.channels;
    src.speedCorrection = (float) info->format.sampleRate / (float) ctx->sampleRate;

    src.queue = NULL;

    // TODO: Buffers. For now we're not using them.
    //src.bufferLength = 512;
    //src.sourceBuffer = calloc(0, src.bufferLength);

    src.playing = false;
    src.position = 0;
    src.finePosition = 0.0f;

    size_t currentId = ctx->sources.length;

    if (!VectorAppend(&ctx->sources, &src))
    {
        ctx->errorMsg = "Sources vector ran out of space. This is likely a bug.";
        return MX_RESULT_OUT_OF_MEMORY;
    }

    source->id = currentId;

    return MX_RESULT_OK;
}

MxResult mxDestroySource(MxContext* context, MxSource source)
{

}

MxResult mxSourceQueueBuffer(MxContext* context, MxSource source, MxBuffer buffer)
{
    Context* ctx = (Context*) context;

    if (source.id >= ctx->sources.length)
    {
        ctx->errorMsg = "An invalid source was provided.";
        return MX_RESULT_INVALID_SOURCE;
    }

    if (buffer.id >= ctx->buffers.length)
    {
        ctx->errorMsg = "An invalid buffer was provided.";
        return MX_RESULT_INVALID_BUFFER;
    }

    Source* src = VectorGet(&ctx->sources, source.id);

    SourceQueueNode* node = malloc(sizeof(SourceQueueNode));
    node->buffer = buffer.id;
    node->next = NULL;

    if (src->queue)
    {
        if (src->queue->prev)
        {
            src->queue->prev->next = node;
            src->queue->prev = node;
        }
        else
        {
            src->queue->next = node;
            src->queue->prev = node;
        }

        src->queueLength++;
    }
    else
    {
        node->prev = NULL;
        src->queue = node;
        src->queueLength = 1;
    }

    return MX_RESULT_OK;
}

MxResult mxSourcePlay(MxContext* context, MxSource source)
{
    Context* ctx = (Context*) context;

    if (source.id >= ctx->sources.length)
    {
        ctx->errorMsg = "An invalid source was provided.";
        return MX_RESULT_INVALID_SOURCE;
    }

    Source* src = VectorGet(&ctx->sources, source.id);
    if (!src->queue)
    {
        ctx->errorMsg = "Nothing in the source queue. Cannot play.";
        return MX_RESULT_SOURCE_QUEUE_EMPTY;
    }

    src->playing = true;
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
    const Vector* buffers = &ctx->buffers;
    const Vector* sources = &ctx->sources;

    // TODO: Source buffers, as mentioned above. This is the code for when we use buffers.
    /*for (size_t s = 0; s < sources->length; s++)
    {
        Source* source = VectorGet(sources, s);

        if (!source->playing)
            continue;

        const Buffer* buf = VectorGet(buffers, source->queue->buffer);
        const uint8_t* data = buf->data;

        for (size_t i = 0; i < length; i += 2)
        {
            //int position = ctx->temp * 4;

            //buffer[i] = GetSample(data, position, MX_DATA_TYPE_I16);
            //buffer[i + 1] = GetSample(data, position + 2, MX_DATA_TYPE_I16);
        }
    }*/

    for (size_t i = 0; i < length; i += 2)
    {
        buffer[i + 0] = 0;
        buffer[i + 1] = 0;

        for (size_t s = 0; s < sources->length; s++)
        {
            Source* source = VectorGet(sources, s);

            if (!source->playing)
                continue;

            const MxDataType dataType = source->format.dataType;
            const Buffer* buf = VectorGet(buffers, source->queue->buffer);
            const uint8_t* data = buf->data;

            size_t position = source->position * source->sampleStride;

            float sampleL = GetSample(data, position, dataType);
            float sampleR = GetSample(data, position + source->channelStride, dataType);

            buffer[i + 0] += sampleL;
            buffer[i + 1] += sampleR;

            source->finePosition += source->speedCorrection;
            const size_t intPos = (size_t) source->finePosition;
            source->position += intPos;
            source->finePosition -= (double) intPos;
        }
    }
}
