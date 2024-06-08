#pragma once

#include <cstdint>
#include <vector>

#include "mixr/Context.h"
#include "mixr/AudioFormat.h"

namespace mixr {

    struct Buffer {
        std::vector<uint8_t> Data;
        AudioFormat Format;

        uint8_t ByteAlign;
    };

    class Impl {
    private:
        uint32_t _sampleRate;

        std::vector<Buffer> _buffers;

    public:
        explicit Impl(uint32_t sampleRate);

        size_t CreateBuffer(const AudioFormat& format, uint8_t* data, size_t dataLength);
    };

}
