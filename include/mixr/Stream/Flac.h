#pragma once

#include "AudioStream.h"
#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    MX_C_API_EXPORT void mxStreamLoadFlac(const char* path, MxAudioStream **pAudioStream);

#ifdef __cplusplus
}
#endif