#define MINIMP3_IMPLEMENTATION

#include "mixr/Stream/Mp3.hpp"

#include <stdexcept>
#include <iostream>

namespace mixr::Stream {
    Mp3::Mp3(const std::string& path) {
        if (mp3dec_ex_open(&_mp3, path.c_str(), MP3D_SEEK_TO_SAMPLE))  {
            throw std::runtime_error("Failed to load mp3.");
        }

#if MINIMP3_FLOAT_OUTPUT
        DataType dataType = DataType::F32;
#else
        DataType dataType = DataType::I16;
#endif

        _format = {
            /* DataType= */ dataType,
            /* SampleRate= */ static_cast<uint32_t>(_mp3.info.hz),
            /* Channels= */ static_cast<uint8_t>(_mp3.info.channels)
        };

        _lengthInBytes = _mp3.samples * sizeof(mp3d_sample_t) * _format.Channels;
    }

    Mp3::~Mp3() {
        mp3dec_ex_close(&_mp3);
    }

    AudioFormat Mp3::Format() {
        return _format;
    }

    size_t Mp3::GetBuffer(uint8_t* buffer, size_t bufferLength) {
        return mp3dec_ex_read(&_mp3, (mp3d_sample_t*) buffer, bufferLength / sizeof(mp3d_sample_t)) * sizeof(mp3d_sample_t);
    }

    void Mp3::Restart() {
        mp3dec_ex_seek(&_mp3, 0);
    }

    size_t Mp3::PCMLengthInBytes() {
        return _lengthInBytes;
    }

    std::vector<uint8_t> Mp3::GetPCM() {
        std::vector<uint8_t> pcm;
        pcm.resize(_lengthInBytes);

        GetBuffer(pcm.data(), pcm.size());

        return pcm;
    }
}