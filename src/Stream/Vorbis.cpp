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
    }

    Vorbis::~Vorbis() {
        stb_vorbis_close(_vorbis);
    }

    AudioFormat Vorbis::Format() {
        return _format;
    }

    size_t Vorbis::GetBuffer(uint8_t* buffer, size_t bufferLength) {
        size_t numSamples = stb_vorbis_get_samples_float_interleaved(_vorbis, _format.Channels == Channels::Stereo ? 2 : 1, (float*) buffer, bufferLength / sizeof(float));

        return numSamples * (_format.Channels == Channels::Stereo ? 2 : 1) * sizeof(float);
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