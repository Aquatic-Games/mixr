#pragma once

#include <cstdint>
#include <vector>

#include "AudioFormat.h"

namespace mixr {

    class AudioBuffer;
    class AudioSource;

    class Context {
        friend class AudioBuffer;
        friend class AudioSource;

    private:
        class Buffer;
        class Source;

        uint32_t _sampleRate;

        std::vector<Buffer> _buffers;
        //std::vector<Source> _sources;

    public:
        explicit Context(uint32_t sampleRate);

        AudioBuffer CreateBuffer(const AudioFormat& format, void* data);
    };

}
