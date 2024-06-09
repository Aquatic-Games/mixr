#pragma once

#include <cstdint>
#include <memory>

#include "Common.h"
#include "AudioBuffer.h"
#include "AudioSource.h"

namespace mixr {

    class Context {
    private:
        std::unique_ptr<Impl> _impl;

    public:
        explicit Context(uint32_t sampleRate);
        ~Context();

        std::unique_ptr<AudioBuffer> CreateBuffer(const AudioFormat& format, void* data, size_t dataLength);
        std::unique_ptr<AudioSource> CreateSource();
    };

}
