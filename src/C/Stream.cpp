#include "mixr/Stream/AudioStream.h"

#include "mixr/Stream/AudioStream.hpp"

using namespace mixr;
using namespace mixr::Stream;

MxAudioFormat mxStreamGetFormat(MxAudioStream *stream) {
    AudioStream* aStream = (AudioStream*) stream;

    AudioFormat format = aStream->Format();

    return {
        .DataType = (MxDataType) format.DataType,
        .SampleRate = format.SampleRate,
        .Channels = (MxChannels) format.Channels
    };
}

size_t mxStreamGetPCMLengthInBytes(MxAudioStream *stream) {
    AudioStream* aStream = (AudioStream*) stream;

    return aStream->PCMLengthInBytes();
}

void mxStreamGetPCM(MxAudioStream *stream, uint8_t *data, size_t *dataLength) {
    AudioStream* aStream = (AudioStream*) stream;

    *dataLength = aStream->PCMLengthInBytes();

    if (data) {
        auto pcmData = aStream->GetPCM();
        auto dataPtr = pcmData.data();
        std::copy(dataPtr, dataPtr + *dataLength, data);
    }
}

void mxDestroyStream(MxAudioStream *stream) {
    AudioStream* aStream = (AudioStream*) stream;
    delete aStream;
}