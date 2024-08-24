#pragma once

#include "AudioStream.hpp"

#include <FLAC++/decoder.h>
#include <memory>
#include <string>

namespace mixr::Stream {

    class Flac : public AudioStream {
    private:
        std::unique_ptr<FLAC::Decoder::File> _file;

    public:
        explicit Flac(const std::string& path);

        AudioFormat Format() override;
        std::vector<uint8_t> GetPCM() override;
    };

}
