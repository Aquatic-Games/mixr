#include "mixr/Context.h"
#include "mixr/AudioBuffer.h"

#include "Buffer.h"

namespace mixr {
    Context::Context(uint32_t sampleRate) {
        _sampleRate = sampleRate;
    }

    AudioBuffer Context::CreateBuffer(const AudioFormat& format, void* data) {
        return AudioBuffer();
    }
}