#pragma once

#include <cstdint>
#include <vector>
#include <deque>

#include "mixr/Context.h"
#include "mixr/Common.h"

#include "mixr/Utils/ADPCM.h"

namespace mixr {

    struct Buffer {
        std::vector<uint8_t> Data;
    };

    struct Source {
        // ------ Source Info -----
        SourceType Type;
        AudioFormat Format;
        int ByteAlign;
        int StereoAlign;
        double SpeedCorrection;

        // ----- Buffering -----
        std::deque<size_t> QueuedBuffers;
        uint8_t* MixBuffer;

        // ----- Playing Info ------
        bool Playing;
        double Speed;
        float MainVolume;
        bool Looping;

        size_t LengthInSamples;

        float VolumeL;
        float VolumeR;

        size_t Position;
        double FinePosition;

        // ----- Interpolation -----
        size_t LastPosition;
        float LastSampleL;
        float LastSampleR;

        // ----- IMA ADPCM -----
        size_t LastChunk;
        size_t ChunkSize;
        size_t NumChunks;
    };

    class Impl {
    private:
        uint32_t _sampleRate;
        float _masterVolume;

        std::vector<Buffer> _buffers;
        std::vector<Source> _sources;

        std::deque<size_t> _availableBuffers;
        std::deque<size_t> _availableSources;

        void UpdateSource(Source* source);

    public:
        explicit Impl(uint32_t sampleRate);

        size_t CreateBuffer(uint8_t* data, size_t dataLength);
        void DestroyBuffer(size_t bufferId);

        size_t CreateSource(const SourceDescription& description);
        void DestroySource(size_t sourceId);

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
        void SourceSetChannelVolumes(size_t sourceId, float volumeL, float volumeR);

        void MixToStereoF32Buffer(float* buffer, size_t bufferLength);
    };

}
