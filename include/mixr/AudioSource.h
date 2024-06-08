#pragma once

#include <cstdint>

#include "Context.h"

namespace mixr {

    class AudioSource {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioSource(size_t id, Impl* impl);
    };

}
