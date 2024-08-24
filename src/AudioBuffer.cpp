#include "mixr/AudioBuffer.h"
#include "Impl/Impl.h"

namespace mixr {
    AudioBuffer::AudioBuffer(size_t id, Impl* impl) {
        _id = id;
        _impl = impl;
    }

    AudioBuffer::~AudioBuffer() {
        _impl->DestroyBuffer(_id);
    }
}