#pragma once

#include <cstdint>

namespace mixr {

    enum class DataType {
        U8,
        I16,
        I32,
        F32
    };

    enum class Channels {
        Mono,
        Stereo
    };

    struct AudioFormat {
        DataType DataType;
        uint32_t SampleRate;
        Channels Channels;
    };

}