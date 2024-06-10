#pragma once

#include <cstdint>
#include <vector>
#include <queue>

#include "mixr/Context.h"
#include "mixr/Common.h"

namespace mixr {

    struct Buffer {
        std::vector<uint8_t> Data;
        AudioFormat Format;
        size_t LengthInSamples;

        int ByteAlign;
        int StereoAlign;
        double SpeedCorrection;
    };

    struct Source {
        std::queue<size_t> QueuedBuffers;

        bool Playing;
        double Speed;
        float MainVolume;
        bool Looping;

        float VolumeL;
        float VolumeR;

        size_t Position;
        double FinePosition;

        size_t LastPosition;
        float LastSampleL;
        float LastSampleR;
    };

    class Impl {
    private:
        uint32_t _sampleRate;
        float _masterVolume;

        std::vector<Buffer> _buffers;
        std::vector<Source> _sources;

    public:
        explicit Impl(uint32_t sampleRate);

        size_t CreateBuffer(const AudioFormat& format, uint8_t* data, size_t dataLength);
        size_t CreateSource();

        void SetMasterVolume(float volume);

        void SourceSubmitBuffer(size_t sourceId, size_t bufferId);
        void SourceClearBuffers(size_t sourceId);
        void SourcePlay(size_t sourceId);
        void SourcePause(size_t sourceId);
        void SourceStop(size_t sourceId);
        void SourceSetSpeed(size_t sourceId, double speed);
        void SourceSetVolume(size_t sourceId, float volume);
        void SourceSetLooping(size_t sourceId, bool looping);
        void SourceSetPanning(size_t sourceId, float panning);

        void MixToStereoF32Buffer(float* buffer, size_t bufferLength);
    };

}
