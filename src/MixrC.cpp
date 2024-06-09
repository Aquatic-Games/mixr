#include "mixr/mixr.h"

#include "Impl/Impl.h"

using namespace mixr;

void mxCreateContext(uint32_t sampleRate, MxContext** pContext) {
    Impl* impl = new Impl(sampleRate);
    *pContext = (MxContext*) impl;
}

void mxDestroyContext(MxContext* context) {
    delete (Impl*) context;
}