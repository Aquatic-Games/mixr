#include "mixr/Context.h"
#include "Impl/Impl.h"

namespace mixr {
    Context::Context(uint32_t sampleRate) {
        _impl = std::make_unique<Impl>(sampleRate);
    }
}