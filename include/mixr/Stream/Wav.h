#pragma once

#include "AudioStream.h"

#ifdef __cplusplus
extern "C" {
#endif

    void mxStreamLoadWav(const char* path, MxAudioStream **pAudioStream);

#ifdef __cplusplus
}
#endif