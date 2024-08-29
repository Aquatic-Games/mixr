#include "mixr/Stream/Vorbis.hpp"

#include <stdexcept>

namespace mixr::Stream {
    Vorbis::Vorbis(const std::string& path) {
        int error;
        _vorbis = stb_vorbis_open_filename(path.c_str(), &error, nullptr);

        if (!_vorbis) {
            throw std::runtime_error("Failed to load vorbis: Error code " + std::to_string(error));
        }

        stb_vorbis_info info = stb_vorbis_get_info(_vorbis);

        _format = {
            /* DataType= */ DataType::F32,
            /* SampleRate= */ info.sample_rate,
            /* Channels= */ info.channels == 2 ? Channels::Stereo : Channels::Mono
        };

        _lengthInBytes = stb_vorbis_stream_length_in_samples(_vorbis) * info.sample_rate * info.channels * sizeof(float);
        _currentBufferPos = 0;
    }

    Vorbis::~Vorbis() {
        stb_vorbis_close(_vorbis);
    }

    AudioFormat Vorbis::Format() {
        return _format;
    }

    size_t Vorbis::GetBuffer(uint8_t* buffer, size_t bufferLength) {
        size_t currentIndex = 0;

        while (currentIndex < bufferLength) {
            if (_currentBufferPos == 0)
            {
                float** output;
                int channels;
                size_t frameSize = stb_vorbis_get_frame_float(_vorbis, &channels, &output);

                if (frameSize == 0) {
                    break;
                }

                _buffer.clear();
                _buffer.reserve(frameSize * channels * sizeof(float));
                for (size_t i = 0; i < frameSize; i++) {
                    for (int c = 0; c < channels; c++) {
                        uint32_t sample = *(uint32_t*) &output[c][i];

                        _buffer.push_back(sample & 0xFF);
                        _buffer.push_back(sample >> 8);
                        _buffer.push_back(sample >> 16);
                        _buffer.push_back(sample >> 24);
                    }
                }
            }

            auto copySize = (_buffer.size() - _currentBufferPos);
            if (currentIndex + copySize > bufferLength) {
                copySize = bufferLength - currentIndex;
            }

            std::copy(_buffer.data() + _currentBufferPos, _buffer.data() + _currentBufferPos + copySize, buffer + currentIndex);

            currentIndex += copySize;

            _currentBufferPos += copySize;

            if (_currentBufferPos >= _buffer.size()) {
                _currentBufferPos -= _buffer.size();
            }
        }

        return currentIndex;
    }

    void Vorbis::Restart() {
        stb_vorbis_seek_start(_vorbis);
    }

    size_t Vorbis::PCMLengthInBytes() {
        return _lengthInBytes;
    }

    std::vector<uint8_t> Vorbis::GetPCM() {
        std::vector<uint8_t> data;
        data.resize(_lengthInBytes);

        GetBuffer(data.data(), _lengthInBytes);

        return data;
    }
}