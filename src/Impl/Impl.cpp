#include "Impl.h"

namespace mixr {
    Impl::Impl(uint32_t sampleRate) {
        _sampleRate = sampleRate;
    }

    size_t Impl::CreateBuffer(const AudioFormat& format, uint8_t* data, size_t dataLength) {
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

        Buffer buffer {
            .Data = std::vector<uint8_t>(data, data + dataLength),
            .Format = format,

            .ByteAlign = byteAlign,
            // Corrects for the sample rate - if the Mixer sample rate is 48khz, and the buffer sample rate is 44.1khz,
            // then this value will be 0.91xyzw, causing the mixer to play the buffer slightly slower to correct the speed.
            .SpeedCorrection = static_cast<float>(format.SampleRate) / static_cast<float>(_sampleRate)
        };

        size_t index = _buffers.size();
        _buffers.push_back(buffer);

        return index;
    }

    size_t Impl::CreateSource() {
        Source source {
            .QueuedBuffers = std::queue<size_t>()
        };

        size_t index = _sources.size();
        _sources.push_back(source);

        return index;
    }

    void Impl::SubmitBufferToSource(size_t sourceId, size_t bufferId) {
        _sources[sourceId].QueuedBuffers.push(bufferId);
    }

    void Impl::MixToStereoF32Buffer(float* buffer, size_t bufferLength) {
        for (int i = 0; i < bufferLength; i += 2) {
            buffer[i + 0] = 0;
            buffer[i + 1] = 0;
        }
    }
}