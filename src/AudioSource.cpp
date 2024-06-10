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

    void AudioSource::ClearBuffers() {
        _impl->SourceClearBuffers(_id);
    }

    void AudioSource::SetSpeed(double speed) {
        _impl->SourceSetSpeed(_id, speed);
    }

    void AudioSource::SetVolume(float volume) {
        _impl->SourceSetVolume(_id, volume);
    }

    void AudioSource::SetLooping(bool looping) {
        _impl->SourceSetLooping(_id, looping);
    }

    void AudioSource::SetPanning(float panning) {
        _impl->SourceSetPanning(_id, panning);
    }

    void AudioSource::Play() {
        _impl->SourcePlay(_id);
    }

    void AudioSource::Pause() {
        _impl->SourcePause(_id);
    }

    void AudioSource::Stop() {
        _impl->SourceStop(_id);
    }
}