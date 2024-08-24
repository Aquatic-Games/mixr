#pragma once

#include <fstream>
#include <string>

#include "AudioStream.hpp"
#include "../Common.h"

namespace mixr::Stream {

    enum ADPCMType {
        IMA
    };

    struct ADPCMInfo {
        ADPCMType Type;
        size_t ChunkSize;
    };

    class Wav : public AudioStream {
    private:
        std::ifstream _stream;
        AudioFormat _format;
        bool _isAdpcm;
        ADPCMInfo _adpcmInfo;

        size_t _dataStartPoint;
        uint32_t _dataLength;

    public:
        explicit Wav(const std::string& path);
        ~Wav() override;

        AudioFormat Format() override;

        size_t PCMLengthInBytes() override;
        std::vector<uint8_t> GetPCM() override;

        bool IsADPCM();
        ADPCMInfo ADPCMInfo();
    };

}
