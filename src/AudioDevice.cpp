#include "mixr/AudioDevice.h"
#include "Impl/Impl.h"

namespace mixr {
    AudioDevice::AudioDevice(uint32_t sampleRate) {
        _context = std::make_unique<Context>(sampleRate);
    }

    void AudioDevice::GetBuffer(uint8_t* buffer, size_t dataLength) {
        _context->_impl->MixToStereoF32Buffer((float*) buffer, dataLength / 4);
    }
}