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

void mxStreamGetPCM(MxAudioStream *stream, uint8_t *data, size_t *dataLength) {
    AudioStream* aStream = (AudioStream*) stream;

    auto aData = aStream->GetPCM();
    *dataLength = aData.size();

    if (data) {
        auto dataPtr = aData.data();
        std::copy(dataPtr, dataPtr + *dataLength, data);
    }
}

void mxDestroyStream(MxAudioStream *stream) {
    AudioStream* aStream = (AudioStream*) stream;
    delete aStream;
}