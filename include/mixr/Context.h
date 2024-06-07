#pragma once

#include <cstdint>
#include <memory>

namespace mixr {

    class Context {
    private:
        class Impl;

        std::unique_ptr<Impl> _impl;

    public:
        Context(uint32_t sampleRate);
    };

}
