#pragma once

#include "AudioStream.hpp"

#include <string>
#include <iostream>

#include <stb_vorbis.c>

namespace mixr::Stream {

    class MIXR_CPP_API Vorbis : public AudioStream {
    private:
        stb_vorbis* _vorbis;
        AudioFormat _format;
        size_t _lengthInBytes;
        std::vector<uint8_t> _buffer;
        size_t _currentBufferPos;

    public:
        explicit Vorbis(const std::string& path);
        ~Vorbis() override;

        AudioFormat Format() override;

        size_t GetBuffer(uint8_t* buffer, size_t bufferLength) override;

        void Restart() override;

        size_t PCMLengthInBytes() override;

        std::vector<uint8_t> GetPCM() override;
    };


}