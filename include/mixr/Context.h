#pragma once

#include <cstdint>

namespace mixr {

    class Context {
    private:
        uint32_t _sampleRate;

    public:
        explicit Context(uint32_t sampleRate);
    };

}
