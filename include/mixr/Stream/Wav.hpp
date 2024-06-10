#pragma once

#include <fstream>
#include <string>

#include "AudioStream.hpp"
#include "../Common.h"

namespace mixr::Stream {

    class Wav : public AudioStream {
    private:
        std::ifstream _stream;
        AudioFormat _format;

        size_t _dataStartPoint;
        uint32_t _dataLength;

    public:
        explicit Wav(const std::string& path);
        ~Wav() override;

        AudioFormat Format() override;
        std::vector<uint8_t> GetPCM() override;
    };

}
