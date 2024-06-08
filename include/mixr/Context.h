#pragma once

#include <cstdint>
#include <memory>

#include "AudioFormat.h"

namespace mixr {

    class Impl;

    class AudioBuffer;
    class AudioSource;

    class Context {
    private:
        std::unique_ptr<Impl> _impl;

    public:
        explicit Context(uint32_t sampleRate);
        ~Context();

        std::unique_ptr<AudioBuffer> CreateBuffer(const AudioFormat& format, void* data, size_t dataLength);
    };

}
