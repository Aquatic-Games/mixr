#pragma once

#include "AudioStream.h"

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

    void mxStreamLoadWav(const char* path, MxAudioStream **pAudioStream);

    bool mxWavIsADPCM(MxAudioStream *stream);
    MxADPCMInfo mxWavGetADPCMInfo(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif