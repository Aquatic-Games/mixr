#include "mixr/Context.h"
#include "mixr/AudioBuffer.h"
#include "mixr/AudioSource.h"
#include "Impl/Impl.h"

namespace mixr {
    Context::Context(uint32_t sampleRate) {
        _impl = std::make_unique<Impl>(sampleRate);
    }

    Context::~Context() = default;

    std::unique_ptr<AudioBuffer> Context::CreateBuffer(const BufferDescription& description, void* data, size_t dataLength) {
        size_t index = _impl->CreateBuffer(description, static_cast<uint8_t*>(data), dataLength);
        return std::make_unique<AudioBuffer>(index, _impl.get());
    }

    std::unique_ptr<AudioSource> Context::CreateSource() {
        size_t index = _impl->CreateSource();
        return std::make_unique<AudioSource>(index, _impl.get());
    }

    void Context::SetMasterVolume(float volume) {
        _impl->SetMasterVolume(volume);
    }
}