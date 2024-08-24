#pragma once

#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxAudioStream MxAudioStream;

    MIXR_C_API MxAudioFormat mxStreamGetFormat(MxAudioStream *stream);

    MIXR_C_API size_t mxStreamGetPCMLengthInBytes(MxAudioStream *stream);
    MIXR_C_API void mxStreamGetPCM(MxAudioStream *stream, uint8_t *data, size_t *dataLength);

    MIXR_C_API void mxDestroyStream(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif
