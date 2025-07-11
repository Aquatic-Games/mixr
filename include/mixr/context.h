#ifndef MIXR_CONTEXT_H
#define MIXR_CONTEXT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#include "common.h"

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct
    {
        uint32_t sampleRate;
    } MxContextInfo;

    typedef struct
    {
        MxAudioFormat format;
    } MxSourceInfo;

    typedef enum
    {
        MX_SOURCE_STATE_STOPPED,
        MX_SOURCE_STATE_PAUSED,
        MX_SOURCE_STATE_PLAYING,
    } MxSourceState;

    typedef struct MxContext MxContext;

    typedef struct
    {
        size_t id;
    } MxBuffer;

    typedef struct
    {
        size_t id;
    } MxSource;

    MIXR_C_API MxResult mxCreateContext(const MxContextInfo *info, MxContext **context);
    MIXR_C_API void mxDestroyContext(MxContext *context);
    MIXR_C_API void mxSetMasterVolume(MxContext *context, float volume);
    MIXR_C_API float mxGetMasterVolume(MxContext *context);

    MIXR_C_API MxResult mxCreateBuffer(MxContext *context, const uint8_t* data, size_t length, MxBuffer *buffer);
    MIXR_C_API MxResult mxDestroyBuffer(MxContext *context, MxBuffer buffer);

    MIXR_C_API MxResult mxCreateSource(MxContext *context, const MxSourceInfo* info, MxSource *source);
    MIXR_C_API MxResult mxDestroySource(MxContext *context, MxSource source);
    MIXR_C_API MxResult mxSourceQueueBuffer(MxContext *context, MxSource source, MxBuffer buffer);
    MIXR_C_API MxResult mxSourceClearQueue(MxContext *context, MxSource source);
    MIXR_C_API MxResult mxSourcePlay(MxContext *context, MxSource source);
    MIXR_C_API MxResult mxSourcePause(MxContext *context, MxSource source);
    MIXR_C_API MxResult mxSourceStop(MxContext *context, MxSource source);
    MIXR_C_API MxResult mxSourceGetState(MxContext *context, MxSource source, MxSourceState *state);
    MIXR_C_API MxResult mxSourceSetVolume(MxContext *context, MxSource source, float volume);
    MIXR_C_API MxResult mxSourceGetVolume(MxContext *context, MxSource source, float *volume);
    MIXR_C_API MxResult mxSourceSetSpeed(MxContext *context, MxSource source, double speed);
    MIXR_C_API MxResult mxSourceGetSpeed(MxContext *context, MxSource source, double *speed);

    MIXR_C_API void mxMixInterleavedStereo(MxContext *context, float* buffer, size_t length);

#ifdef __cplusplus
}
#endif
#endif
