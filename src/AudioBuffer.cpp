#include "mixr/AudioBuffer.h"

namespace mixr {
    AudioBuffer::AudioBuffer(size_t id, Impl* impl) {
        _id = id;
        _impl = impl;
    }
}