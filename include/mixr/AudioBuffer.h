#pragma once

#include <cstdint>

#include "Common.h"

namespace mixr {

    class AudioBuffer {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioBuffer(size_t id, Impl* impl);

        inline size_t ID() {
            return _id;
        }
    };

}
