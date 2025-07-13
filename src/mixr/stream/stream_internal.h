#ifndef MIXR_INT_STREAM_H
#define MIXR_INT_STREAM_H

#include "mixr/stream.h"

typedef struct
{
    void* streamData;
    MxAudioFormat format;

    void (*destroyStream)(void* stream);
    size_t (*getBuffer)(void* stream, uint8_t* buffer, size_t length);
    size_t (*getDataSize)(void* stream);

    void (*getPCM)(void* stream, uint8_t* buffer);
} Stream;

#endif
