#include "mixr/mixr.h"

#include "Impl/Impl.h"

using namespace mixr;

void mxCreateContext(uint32_t sampleRate, MxContext** pContext) {
    Impl* impl = new Impl(sampleRate);
    *pContext = (MxContext*) impl;
}

void mxDestroyContext(MxContext *context) {
    delete (Impl*) context;
}

size_t mxContextCreateBuffer(MxContext *context, MxAudioFormat *format, uint8_t* data, size_t dataLength) {
    Impl* impl = (Impl*) context;

    AudioFormat fmt {
        .DataType = (DataType) format->DataType,
        .SampleRate = format->SampleRate,
        .Channels = (Channels) format->Channels
    };

    size_t index = impl->CreateBuffer(fmt, data, dataLength);
    return index;
}