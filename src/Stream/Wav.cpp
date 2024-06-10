#include "mixr/Stream/Wav.hpp"

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

        _stream.seekg(4, std::ios::cur); // File size

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
                    uint16_t type;
                    _stream.read((char*) &type, sizeof(type));

                    uint16_t channels;
                    _stream.read((char*) &channels, sizeof(channels));

                    uint32_t sampleRate;
                    _stream.read((char*) &sampleRate, sizeof(sampleRate));

                    _stream.seekg(6, std::ios::cur); // 6 "useless" bytes, not needed here.

                    uint16_t bitsPerSample;
                    _stream.read((char*) &bitsPerSample, sizeof(bitsPerSample));

                    _format.SampleRate = sampleRate;

                    switch (channels) {
                        case 1:
                            _format.Channels = Channels::Mono;
                            break;

                        case 2:
                            _format.Channels = Channels::Stereo;
                            break;

                        default:
                            throw std::runtime_error("Unsupported number of channels.");
                    }

                    switch (type) {
                        case 1: {
                            switch (bitsPerSample) {
                                case 8:
                                    _format.DataType = DataType::U8;
                                    break;

                                case 16:
                                    _format.DataType = DataType::I16;
                                    break;

                                case 32:
                                    _format.DataType = DataType::I32;
                                    break;

                                default:
                                    throw std::runtime_error("Unsupported data type.");
                            }

                            break;
                        }

                        case 3: {
                            switch (bitsPerSample) {
                                case 32:
                                    _format.DataType = DataType::F32;
                                    break;

                                default:
                                    throw std::runtime_error("Unsupported data type.");
                            }

                            break;
                        }

                        default:
                            throw std::runtime_error("Unsupported data type.");
                    }

                    break;
                }

                case data: {
                    _dataStartPoint = _stream.tellg();
                    _dataLength = chunkSize;

                    return;
                }

                default:
                    _stream.seekg(chunkSize, std::ios::cur);
                    break;
            }
        }
    }

    AudioFormat Wav::Format() {
        return _format;
    }

    std::vector<uint8_t> Wav::GetPCM() {
        _stream.seekg(_dataStartPoint);

        std::vector<uint8_t> data(_dataLength);
        _stream.read((char*) data.data(), _dataLength);

        return data;
    }
}