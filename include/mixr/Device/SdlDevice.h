#pragma once

#include <cstdint>

#include "../AudioDevice.h"

namespace mixr::Device {

    class MIXR_CPP_API SdlDevice : public AudioDevice {
    private:
        uint32_t _device;

        static void AudioCallback(void* userData, uint8_t* buffer, int length);

    public:
        explicit SdlDevice(uint32_t sampleRate, uint16_t periodSize = 512);
        ~SdlDevice() override;
    };

}
