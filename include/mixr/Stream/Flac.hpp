#pragma once

#include "AudioStream.hpp"

#include <FLAC++/decoder.h>
#include <memory>
#include <string>

namespace mixr::Stream {

    class MIXR_CPP_API Flac : public AudioStream {
    private:
        std::unique_ptr<FLAC::Decoder::File> _file;

    public:
        explicit Flac(const std::string& path);

        AudioFormat Format() override;

        size_t GetBuffer(uint8_t *buffer, size_t bufferLength) override;

        void Restart() override;

        size_t PCMLengthInBytes() override;
        std::vector<uint8_t> GetPCM() override;
    };

}
