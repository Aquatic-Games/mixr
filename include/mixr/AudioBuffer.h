#pragma once

#include "Context.h"

namespace mixr {

    class AudioBuffer {
    private:
        size_t _id;
        Context* _context;

        AudioBuffer(size_t id, Context* context);
        ~AudioBuffer();
    };

}
