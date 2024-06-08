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

            .ByteAlign = byteAlign
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
}