#ifndef MIXR_WAV_H
#define MIXR_WAV_H

#include "stream.h"

#ifdef __cplusplus
extern "C" {
#endif

    MIXR_C_API MxResult mxStreamLoadWav(const char *path, MxStream **stream);

#ifdef __cplusplus
}
#endif

#endif
