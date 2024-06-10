#pragma once

#include <cstdint>

#include "Common.h"
#include "AudioBuffer.h"

namespace mixr {

    class AudioSource {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioSource(size_t id, Impl* impl);

        inline size_t ID() {
            return _id;
        }

        void SubmitBuffer(AudioBuffer* buffer);

        void SetSpeed(double speed);
        void SetVolume(float volume);
        void SetLooping(bool looping);
        void SetPanning(float panning);

        void Play();
        void Pause();
        void Stop();
    };

}
