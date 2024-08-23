#pragma once

#include <cstdint>
#include <cstddef>

namespace mixr {

    class Impl;

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

    enum class SourceType {
        PCM,
        ADPCM
    };

    struct ADPCMDescription {
        size_t ChunkSize;
    };

    struct SourceDescription {
        SourceType Type;
        AudioFormat Format;

        union {
            ADPCMDescription ADPCM;
        };
    };

}