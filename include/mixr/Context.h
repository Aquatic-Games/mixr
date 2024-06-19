#pragma once

#include <cstdint>
#include <memory>

#include "Common.h"
#include "AudioBuffer.h"
#include "AudioSource.h"

namespace mixr {

    class AudioDevice;

    class Context {
        friend class AudioDevice;

    private:
        std::unique_ptr<Impl> _impl;

    public:
        explicit Context(uint32_t sampleRate);
        ~Context();

        std::unique_ptr<AudioBuffer> CreateBuffer(const BufferDescription& description, void* data, size_t dataLength);
        std::unique_ptr<AudioSource> CreateSource();

        void SetMasterVolume(float volume);
    };

}
