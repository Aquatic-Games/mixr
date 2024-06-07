#include "mixr/AudioBuffer.h"

namespace mixr {
    AudioBuffer::AudioBuffer(size_t id, Context* context) {
        _id = id;
        _context = context;
    }

    AudioBuffer::~AudioBuffer() {

    }
}