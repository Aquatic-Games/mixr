#include "mixr/Stream/Wav.h"

#include "mixr/Stream/Wav.hpp"

using namespace mixr::Stream;

void mxStreamLoadWav(const char* path, MxAudioStream **pAudioStream) {
    Wav* wav = new Wav(path);

    *pAudioStream = (MxAudioStream*) wav;
}