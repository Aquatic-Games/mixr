#pragma once

#include <cstdint>
#include <vector>

namespace mixr {

    class AudioBuffer;
    class AudioSource;

    class Context {
    private:
        class Buffer;
        class Source;

        uint32_t _sampleRate;

        std::vector<Buffer> _buffers;
        std::vector<Source> _sources;

    public:
        explicit Context(uint32_t sampleRate);

        AudioBuffer CreateBuffer();
    };

}
