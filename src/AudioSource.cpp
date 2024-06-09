#include "mixr/AudioSource.h"
#include "mixr/AudioBuffer.h"
#include "Impl/Impl.h"

namespace mixr {
    AudioSource::AudioSource(size_t id, Impl* impl) {
        _id = id;
        _impl = impl;
    }

    void AudioSource::SubmitBuffer(AudioBuffer* buffer) {
        _impl->SourceSubmitBuffer(_id, buffer->ID());
    }

    void AudioSource::Play() {
        _impl->SourcePlay(_id);
    }

    void AudioSource::Stop() {
        _impl->SourceStop(_id);
    }
}