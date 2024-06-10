#pragma once

#include <vector>
#include <cstdint>

#include "../Common.h"

namespace mixr::Stream {

    class AudioStream {
    public:
        virtual ~AudioStream() = default;

        virtual AudioFormat Format() = 0;
        virtual std::vector<uint8_t> GetBuffer() = 0;
    };

}
