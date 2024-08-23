#include <iostream>
#include "Impl.h"

inline float Clamp(float value, float min, float max) {
    return value <= min ? min : value >= max ? max : value;
}

inline float Lerp(float a, float b, float multiplier) {
    return multiplier * (b - a) + a;
}

namespace mixr {
    Impl::Impl(uint32_t sampleRate) {
        _sampleRate = sampleRate;
        _masterVolume = 1.0f;
    }

    size_t Impl::CreateBuffer(uint8_t* data, size_t dataLength) {
        Buffer buffer {
            .Data = std::vector<uint8_t>(data, data + dataLength)
        };

        size_t index = _buffers.size();
        _buffers.push_back(buffer);

        return index;
    }

    size_t Impl::CreateSource(const SourceDescription& description) {
        AudioFormat format = description.Format;

        uint8_t byteAlign;
        switch (format.DataType) {
            case DataType::U8:
                byteAlign = 1;
                break;
            case DataType::I16:
                byteAlign = 2;
                break;
            case DataType::I32:
            case DataType::F32:
                byteAlign = 4;
                break;
        }

        int channels;
        switch (format.Channels) {
            case Channels::Mono:
                channels = 1;
                break;

            case Channels::Stereo:
                channels = 2;
                break;
        }

        SourceType type = description.Type;
        auto chunkSize = description.ADPCM.ChunkSize;

        uint8_t* buffer = nullptr;

        switch (type)
        {
            case SourceType::PCM:
                break;
            case SourceType::ADPCM:
                buffer = new uint8_t[(chunkSize - (channels * 4)) * 4];
                break;
        }

        Source source {
            .Type = type,
            .Format = format,
            .ByteAlign = byteAlign,
            .StereoAlign = byteAlign * (channels - 1),
            // Corrects for the sample rate - if the Mixer sample rate is 48khz, and the buffer sample rate is 44.1khz,
            // then this value will be 0.91xyzw, causing the mixer to play the buffer slightly slower to correct the speed.
            .SpeedCorrection = static_cast<float>(format.SampleRate) / static_cast<float>(_sampleRate),

            .QueuedBuffers = std::queue<size_t>(),
            .MixBuffer = buffer,

            .Playing = false,
            .Speed = 1.0,
            .MainVolume = 1.0f,
            .Looping = false,

            .VolumeL = 1.0f,
            .VolumeR = 1.0f,

            .Position = 0,
            .FinePosition = 0.0,

            .LastPosition = 0,
            .LastSampleL = 0.0f,
            .LastSampleR = 0.0f,
            .LastChunk = (size_t) -1,

            .ChunkSize = chunkSize,

        };

        size_t index = _sources.size();
        _sources.push_back(source);

        return index;
    }

    void Impl::SetMasterVolume(float volume) {
        _masterVolume = volume;
    }

    void Impl::SourceSubmitBuffer(size_t sourceId, size_t bufferId) {
        Source* source = &_sources[sourceId];
        source->QueuedBuffers.push(bufferId);

        if (source->QueuedBuffers.size() == 1) {
            UpdateSource(source);
        }
    }

    void Impl::SourceClearBuffers(size_t sourceId) {
        SourceStop(sourceId);
        std::queue<size_t> empty;
        std::swap(_sources[sourceId].QueuedBuffers, empty);
    }

    void Impl::SourcePlay(size_t sourceId) {
        Source* source = &_sources[sourceId];

        if (source->QueuedBuffers.empty())
            return;

        source->Playing = true;
    }

    void Impl::SourcePause(size_t sourceId) {
        _sources[sourceId].Playing = false;
    }

    void Impl::SourceStop(size_t sourceId) {
        Source* source = &_sources[sourceId];
        source->Playing = false;
        source->Position = 0;
        source->LastPosition = 0;
        source->LastSampleL = 0.0f;
        source->LastSampleR = 0.0f;
        source->FinePosition = 0.0;
        source->LastChunk = -1;
    }

    void Impl::SourceSetSpeed(size_t sourceId, double speed) {
        _sources[sourceId].Speed = speed;
    }

    void Impl::SourceSetVolume(size_t sourceId, float volume) {
        _sources[sourceId].MainVolume = volume;
    }

    void Impl::SourceSetLooping(size_t sourceId, bool looping) {
        _sources[sourceId].Looping = looping;
    }

    void Impl::SourceSetPanning(size_t sourceId, float panning) {
        Source* source = &_sources[sourceId];

        source->VolumeL = Clamp(1 - panning, 0.0f, 1.0f);
        source->VolumeR = Clamp(1 - -panning, 0.0f, 1.0f);
    }

    void Impl::SourceSetChannelVolumes(size_t sourceId, float volumeL, float volumeR) {
        Source* source = &_sources[sourceId];

        source->VolumeL = volumeL;
        source->VolumeR = volumeR;
    }

    inline float GetSample(const uint8_t* data, size_t index, DataType dataType) {
        switch (dataType) {
            case DataType::U8:
                return (float) (int8_t) (data[index] - INT8_MAX) / (float) INT8_MAX;
            case DataType::I16:
                return (float) (int16_t) (data[index + 0] | data[index + 1] << 8) / (float) INT16_MAX;
            case DataType::I32:
                return (float) (data[index + 0] | (data[index + 1] << 8) | (data[index + 2] << 16) | (data[index + 3] << 24)) / (float) INT32_MAX;
            case DataType::F32: {
                uint32_t value = (data[index + 0] | (data[index + 1] << 8) | (data[index + 2] << 16) | (data[index + 3] << 24));
                return *(float*) &value;
            }
        }

        return 0;
    }

    void Impl::MixToStereoF32Buffer(float* buffer, size_t bufferLength) {
        for (int i = 0; i < bufferLength; i += 2) {
            buffer[i + 0] = 0;
            buffer[i + 1] = 0;

            // TODO: Optimize this by placing all playing sources into a vector which gets enumerated over.
            for (int s = 0; s < _sources.size(); s++) {
                Source* source = &_sources[s];

                if (!source->Playing || source->MainVolume <= 0.01f)
                    continue;

                Buffer* buf = &_buffers[source->QueuedBuffers.front()];

                uint8_t* bufferData = buf->Data.data();
                AudioFormat* format = &source->Format;

                size_t bytePosition = source->Position * (source->ByteAlign + source->StereoAlign);

                float sampleL, sampleR;

                switch (source->Type) {
                    case SourceType::PCM: {
                        sampleL = GetSample(bufferData, bytePosition, format->DataType);
                        sampleR = GetSample(bufferData, bytePosition + source->StereoAlign, format->DataType);

                        break;
                    }

                    // This makes me uncomfortable..
                    // The fact that it works and *fast* annoys me because now I don't want to change it even though
                    // it could really do with some threading and being less awful.
                    case SourceType::ADPCM: {
                        bool stereo = source->Format.Channels == Channels::Stereo;
                        size_t chunkSize = source->ChunkSize;
                        size_t dataSize = (chunkSize - (stereo ? 8 : 4)) * 4;

                        size_t chunk = bytePosition / dataSize;

                        if (chunk < source->NumChunks && chunk != source->LastChunk) {
                            source->LastChunk = chunk;
                            Utils::ADPCM::DecodeIMAChunk(buf->Data.data() + (chunk * chunkSize), chunkSize, source->MixBuffer, stereo);
                        }

                        size_t newBytePosition = bytePosition % dataSize;

                        uint8_t* sBuf = source->MixBuffer;

                        sampleL = (float) (int16_t) (sBuf[newBytePosition + 0] | sBuf[newBytePosition + 1] << 8) / (float) INT16_MAX;
                        sampleR = (float) (int16_t) (sBuf[newBytePosition + 2] | sBuf[newBytePosition + 3] << 8) / (float) INT16_MAX;

                        break;
                    }
                }

                float lastSampleL = source->LastSampleL;
                float lastSampleR = source->LastSampleR;

                float outSampleL = Lerp(lastSampleL, sampleL, (float) source->FinePosition) * source->VolumeL * source->MainVolume;
                float outSampleR = Lerp(lastSampleR, sampleR, (float) source->FinePosition) * source->VolumeR * source->MainVolume;

                buffer[i + 0] += Clamp(outSampleL, -1.0f, 1.0f);
                buffer[i + 1] += Clamp(outSampleR, -1.0f, 1.0f);

                source->FinePosition += source->SpeedCorrection * source->Speed;

                int intPos = (int) source->FinePosition;
                source->Position += intPos;
                source->FinePosition -= intPos;

                if (source->Position != source->LastPosition) {
                    source->LastPosition = source->Position;
                    source->LastSampleL = sampleL;
                    source->LastSampleR = sampleR;
                }

                if (source->Position >= source->LengthInSamples) {
                    // Hmm. This doesn't seem ideal.
                    // Right now the library relies on the queue having elements, since it gets the buffer by checking
                    // the front of the queue. Perhaps the "current buffer" should be stored in an index instead.
                    if (source->QueuedBuffers.size() > 1) {
                        source->QueuedBuffers.pop();
                        source->Position = 0;
                        UpdateSource(source);
                    } else if (source->Looping) {
                        source->Position -= source->LengthInSamples;
                    } else {
                        SourceStop(s);
                    }
                }
            }

            buffer[i + 0] = Clamp(buffer[i + 0] * _masterVolume, -1.0f, 1.0f);
            buffer[i + 1] = Clamp(buffer[i + 1] * _masterVolume, -1.0f, 1.0f);
        }
    }

    void Impl::UpdateSource(Source* source) {
        auto dataLength = _buffers[source->QueuedBuffers.front()].Data.size();
        auto byteAlign = source->ByteAlign;
        auto channels = 0;

        switch (source->Format.Channels) {
            case Channels::Mono:
                channels = 1;
                break;

            case Channels::Stereo:
                channels = 2;
                break;
        }

        switch (source->Type) {
            case SourceType::PCM:
                source->LengthInSamples = dataLength / (byteAlign * channels);
                break;

            case SourceType::ADPCM: {
                auto numChunks = dataLength / source->ChunkSize;
                source->NumChunks = numChunks;
                int bytesToRemovePerChunk = channels * 4;
                source->LengthInSamples = ((dataLength * 4) - (bytesToRemovePerChunk * numChunks)) / (byteAlign * channels);
                break;
            }
        }
    }
}