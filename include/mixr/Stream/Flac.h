#pragma once

#include "AudioStream.h"
#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    MIXR_C_API void mxStreamLoadFlac(const char* path, MxAudioStream **pAudioStream);

#ifdef __cplusplus
}
#endif