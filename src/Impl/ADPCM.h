#pragma once

#include <cstdint>

// https://gist.github.com/aquagoose/526a2d6bc829f39a01d29dc2de0b8146
namespace mixr::ADPCM {
    void DecodeIMAChunk(uint8_t* inBuffer, uint8_t* outBuffer, bool stereo);
}