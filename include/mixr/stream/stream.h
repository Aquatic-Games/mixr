#ifndef MIXR_STREAM_H
#define MIXR_STREAM_H

#include <stdlib.h>

#include "../common.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxStream MxStream;

    MIXR_C_API void mxDestroyStream(MxStream *stream);

    MIXR_C_API MxAudioFormat mxStreamGetAudioFormat(MxStream *stream);
    MIXR_C_API size_t mxStreamGetLength(MxStream *stream);

    MIXR_C_API void mxStreamGetBuffer(MxStream *stream, uint8_t *buffer, size_t length);
    MIXR_C_API void mxStreamGetPCM(MxStream *stream, uint8_t *pcm);

#ifdef __cplusplus
}
#endif

#endif
