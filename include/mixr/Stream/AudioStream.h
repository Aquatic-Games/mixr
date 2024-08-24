#pragma once

#include "../mixr.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxAudioStream MxAudioStream;

    MX_C_API_EXPORT MxAudioFormat mxStreamGetFormat(MxAudioStream *stream);

    MX_C_API_EXPORT size_t mxStreamGetPCMLengthInBytes(MxAudioStream *stream);
    MX_C_API_EXPORT void mxStreamGetPCM(MxAudioStream *stream, uint8_t *data, size_t *dataLength);

    MX_C_API_EXPORT void mxDestroyStream(MxAudioStream *stream);

#ifdef __cplusplus
}
#endif
