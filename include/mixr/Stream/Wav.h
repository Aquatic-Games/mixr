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

    MIXR_C_API void mxStreamLoadWav(const char* path, MxAudioStream **pAudioStream);

    MIXR_C_API bool mxWavIsADPCM(MxAudioStream *stream);
    MIXR_C_API MxADPCMInfo mxWavGetADPCMInfo(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif