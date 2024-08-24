#pragma once

#include "Context.h"

namespace mixr {

    class MIXR_CPP_API AudioDevice {
    private:
        std::unique_ptr<mixr::Context> _context;

    protected:
        explicit AudioDevice(uint32_t sampleRate);
        virtual ~AudioDevice() = default;

        void GetBuffer(uint8_t* buffer, size_t dataLength);

    public:
        Context* Context();
    };

}
