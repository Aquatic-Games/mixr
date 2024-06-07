#pragma once

#include <cstdint>

namespace mixr {

    class Context {
    private:
        class Impl;

        Impl _impl;

    public:
        Context(uint32_t sampleRate);
    };

}
