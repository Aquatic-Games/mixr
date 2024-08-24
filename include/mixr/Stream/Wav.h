#pragma once

#include "AudioStream.h"
#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef enum {
        MX_ADPCM_TYPE_IMA
    } MxADPCMType;

    typedef struct {
        MxADPCMType Type;
        size_t ChunkSize;
    } MxADPCMInfo;

    MX_C_API_EXPORT void mxStreamLoadWav(const char* path, MxAudioStream **pAudioStream);

    MX_C_API_EXPORT bool mxWavIsADPCM(MxAudioStream *stream);
    MX_C_API_EXPORT MxADPCMInfo mxWavGetADPCMInfo(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif