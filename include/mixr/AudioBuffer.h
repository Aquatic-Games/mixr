#pragma once

#include <cstdint>

#include "Context.h"

namespace mixr {

    class AudioBuffer {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioBuffer(size_t id, Impl* impl);
    };

}
