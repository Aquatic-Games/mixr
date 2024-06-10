#include "mixr/mixr.h"

#include "../Impl/Impl.h"

using namespace mixr;

void mxCreateContext(uint32_t sampleRate, MxContext** pContext) {
    Impl* impl = new Impl(sampleRate);
    *pContext = (MxContext*) impl;
}

void mxDestroyContext(MxContext *context) {
    delete (Impl*) context;
}

MxAudioBuffer mxContextCreateBuffer(MxContext *context, MxAudioFormat *format, uint8_t* data, size_t dataLength) {
    Impl* impl = (Impl*) context;

    AudioFormat fmt {
        .DataType = (DataType) format->DataType,
        .SampleRate = format->SampleRate,
        .Channels = (Channels) format->Channels
    };

    size_t index = impl->CreateBuffer(fmt, data, dataLength);
    return index;
}

MxAudioSource mxContextCreateSource(MxContext *context) {
    Impl* impl = (Impl*) context;

    size_t index = impl->CreateSource();
    return index;
}

void mxContextSetMasterVolume(MxContext *context, float volume) {
    Impl* impl = (Impl*) context;

    impl->SetMasterVolume(volume);
}

void mxSourceSubmitBuffer(MxContext *context, MxAudioSource source, MxAudioBuffer buffer) {
    Impl* impl = (Impl*) context;

    impl->SourceSubmitBuffer(source, buffer);
}

void mxSourcePlay(MxContext *context, MxAudioSource source) {
    Impl* impl = (Impl*) context;

    impl->SourcePlay(source);
}

void mxSourcePause(MxContext *context, MxAudioSource source) {
    Impl* impl = (Impl*) context;

    impl->SourcePause(source);
}

void mxSourceStop(MxContext *context, MxAudioSource source) {
    Impl* impl = (Impl*) context;

    impl->SourceStop(source);
}

void mxSourceSetSpeed(MxContext *context, MxAudioSource source, double speed) {
    Impl* impl = (Impl*) context;

    impl->SourceSetSpeed(source, speed);
}

void mxSourceSetVolume(MxContext *context, MxAudioSource source, float volume) {
    Impl* impl = (Impl*) context;

    impl->SourceSetVolume(source, volume);
}

void mxSourceSetLooping(MxContext *context, MxAudioSource source, bool looping) {
    Impl* impl = (Impl*) context;

    impl->SourceSetLooping(source, looping);
}

void mxSourceSetPanning(MxContext *context, MxAudioSource source, float panning) {
    Impl* impl = (Impl*) context;

    impl->SourceSetPanning(source, panning);
}