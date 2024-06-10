#pragma once

#include <fstream>
#include <string>

#include "AudioStream.h"

namespace mixr::Stream {

    class Wav : public AudioStream {
    private:
        std::ifstream _stream;

    public:
        explicit Wav(const std::string& path);
    };

}
