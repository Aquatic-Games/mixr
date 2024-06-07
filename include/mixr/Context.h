#pragma once

#include <cstdint>
#include <memory>

#include "AudioFormat.h"
#include "AudioBuffer.h"

namespace mixr {

    class Context {
    private:
        class Impl;

        std::unique_ptr<Impl> _impl;

    public:
        Context(uint32_t sampleRate);

        std::unique_ptr<AudioBuffer> CreateBuffer(const AudioFormat& format, void* data, size_t dataLength);
    };

}
