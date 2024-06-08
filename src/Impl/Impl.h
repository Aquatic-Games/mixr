#pragma once

#include <cstdint>
#include <vector>
#include <queue>

#include "mixr/Context.h"
#include "mixr/AudioFormat.h"

namespace mixr {

    struct Buffer {
        std::vector<uint8_t> Data;
        AudioFormat Format;

        uint8_t ByteAlign;
    };

    struct Source {
        std::queue<size_t> QueuedBuffers;
    };

    class Impl {
    private:
        uint32_t _sampleRate;

        std::vector<Buffer> _buffers;
        std::vector<Source> _sources;

    public:
        explicit Impl(uint32_t sampleRate);

        size_t CreateBuffer(const AudioFormat& format, uint8_t* data, size_t dataLength);
        size_t CreateSource();
    };

}
