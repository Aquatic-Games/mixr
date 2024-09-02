#pragma once

#include "AudioStream.hpp"

#include <string>

#include <minimp3_ex.h>

namespace mixr::Stream {

    class MIXR_CPP_API Mp3 : public AudioStream {
    private:
        mp3dec_ex_t _mp3;

        AudioFormat _format;
        size_t _lengthInBytes;

    public:
        explicit Mp3(const std::string& path);
        ~Mp3() override;

        AudioFormat Format() override;

        size_t GetBuffer(uint8_t* buffer, size_t bufferLength) override;

        void Restart() override;

        size_t PCMLengthInBytes() override;

        std::vector<uint8_t> GetPCM() override;
    };

}