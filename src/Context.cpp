#include "mixr/Context.h"
#include "mixr/AudioBuffer.h"
#include "Impl/Impl.h"

namespace mixr {
    Context::Context(uint32_t sampleRate) {
        _impl = std::make_unique<Impl>(sampleRate);
    }

    std::unique_ptr<AudioBuffer> Context::CreateBuffer(const AudioFormat& format, void* data, size_t dataLength) {
        size_t index = _impl->CreateBuffer(format, static_cast<uint8_t*>(data), dataLength);
        return std::make_unique<AudioBuffer>(index, _impl.get());
    }

    Context::~Context() = default;
}