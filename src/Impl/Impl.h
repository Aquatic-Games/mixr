#pragma once

#include <cstdint>

#include "mixr/Context.h"

namespace mixr {

    class Context::Impl {
    private:
        uint32_t _sampleRate;

    public:
        Impl(uint32_t sampleRate);
    };

} // mixr
