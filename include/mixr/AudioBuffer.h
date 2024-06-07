#pragma once

#include <cstdint>

namespace mixr {

    class AudioBuffer {
    private:
        class Impl;

        size_t _id;
        Impl* _impl;
    };

}
