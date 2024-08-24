#pragma once

#include <cstdint>

#include "Common.h"
#include "AudioBuffer.h"

namespace mixr {

    class MIXR_CPP_API AudioSource {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioSource(size_t id, Impl* impl);
        ~AudioSource();

        inline size_t ID() {
            return _id;
        }

        void SubmitBuffer(AudioBuffer* buffer);
        void ClearBuffers();

        void SetSpeed(double speed);
        void SetVolume(float volume);
        void SetLooping(bool looping);
        void SetPanning(float panning);
        void SetChannelVolumes(float volumeL, float volumeR);

        void Play();
        void Pause();
        void Stop();
    };

}
