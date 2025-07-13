#include "mixr/stream.h"
#include "stream_internal.h"

void mxDestroyStream(MxStream* stream)
{
    Stream* s = (Stream*) stream;
    s->destroyStream(s->streamData);
    free(s);
}

MxAudioFormat mxStreamGetAudioFormat(MxStream* stream)
{
    const Stream* s = (Stream*) stream;
    return s->format;
}

size_t mxStreamGetLengthInSamples(MxStream* stream)
{
    const Stream* s = (Stream*) stream;
    //return s->lengthInSamples;
    return 0;
}

size_t mxStreamGetBuffer(MxStream* stream, uint8_t* buffer, size_t length)
{
    const Stream* s = (Stream*) stream;
    return s->getBuffer(s->streamData, buffer, length);
}

void mxStreamGetPCM(MxStream* stream, uint8_t* pcm, size_t* length)
{
    const Stream* s = (Stream*) stream;
    const size_t size = s->getDataSize(s->streamData);

    if (length != NULL)
        *length = size;

    if (pcm != NULL)
        s->getPCM(s->streamData, pcm);
}
