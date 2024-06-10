#include "mixr/Stream/Wav.h"

#include <stdexcept>

namespace mixr::Stream {
    Wav::Wav(const std::string& path) {
        constexpr uint32_t riff = 0x46464952;
        constexpr uint32_t wave = 0x45564157;
        constexpr uint32_t fmt  = 0x20746D66;
        constexpr uint32_t data = 0x61746164;

        _stream = std::ifstream(path, std::ios::in | std::ios::binary);

        uint32_t magic;
        _stream.read((char*) &magic, sizeof(magic));
        if (magic != riff) {
            throw std::runtime_error("Given file is not a wav file: Missing RIFF header.");
        }

        _stream.read(nullptr, 4); // File size

        uint32_t waveHeader;
        _stream.read((char*) &waveHeader, sizeof(waveHeader));
        if (waveHeader != wave) {
            throw std::runtime_error("Given file is not a valid wav file: Expected WAVE, was not found.");
        }

        while (true) {
            uint32_t header;
            _stream.read((char*) &header, sizeof(header));

            uint32_t chunkSize;
            _stream.read((char*) &chunkSize, sizeof(chunkSize));

            switch (header) {
                case fmt: {
                    break;
                }

                case data: {
                    return;
                }

                default:
                    _stream.seekg(chunkSize, std::ios::cur);
                    break;
            }
        }
    }
}