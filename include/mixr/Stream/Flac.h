#pragma once

#include "AudioStream.h"

#ifdef __cplusplus
extern "C" {
#endif

void mxStreamLoadFlac(const char* path, MxAudioStream **pAudioStream);

#ifdef __cplusplus
}
#endif