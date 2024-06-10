#pragma once

#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxAudioStream MxAudioStream;

    MxAudioFormat mxStreamGetFormat(MxAudioStream *stream);
    void mxStreamGetPCM(MxAudioStream *stream, uint8_t *data, size_t *dataLength);

    void mxDestroyStream(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif
