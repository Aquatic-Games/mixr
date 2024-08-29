#pragma once

#include <cstdint>
#include <cstddef>

#ifdef _WIN32
#define MIXR_CPP_API __declspec(dllexport)
#else
#define MIXR_CPP_API
#endif

namespace mixr {

    class Impl;

    enum class DataType {
        I8,
        U8,
        I16,
        I32,
        F32
    };

    struct AudioFormat {
        DataType DataType;
        uint32_t SampleRate;
        uint8_t Channels;
    };

    enum class SourceType {
        PCM,
        ADPCM
    };

    enum class SourceState {
        Stopped,
        Paused,
        Playing
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