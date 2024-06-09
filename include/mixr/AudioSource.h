#pragma once

#include <cstdint>

#include "Common.h"
#include "AudioBuffer.h"

namespace mixr {

    class AudioSource {
    private:
        size_t _id;
        Impl* _impl;

    public:
        AudioSource(size_t id, Impl* impl);

        inline size_t ID() {
            return _id;
        }

        void SubmitBuffer(AudioBuffer* buffer);
    };

}
