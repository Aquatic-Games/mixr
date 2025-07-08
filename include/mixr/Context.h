#ifndef MIXR_CONTEXT_H
#define MIXR_CONTEXT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#include "Common.h"

#define MIXR_C_API

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

    MxResult mxCreateContext(const MxContextInfo *info, MxContext **context);
    void mxDestroyContext(MxContext *context);
    const char* mxGetLastErrorString(MxContext *context);
    void mxSetMasterVolume(MxContext *context, float volume);
    float mxGetMasterVolume(MxContext *context);

    MxResult mxCreateBuffer(MxContext *context, const uint8_t* data, size_t length, MxBuffer *buffer);
    MxResult mxDestroyBuffer(MxContext *context, MxBuffer buffer);

    MxResult mxCreateSource(MxContext *context, const MxSourceInfo* info, MxSource *source);
    MxResult mxDestroySource(MxContext *context, MxSource source);
    MxResult mxSourceQueueBuffer(MxContext *context, MxSource source, MxBuffer buffer);
    MxResult mxSourceClearQueue(MxContext *context, MxSource source);
    MxResult mxSourcePlay(MxContext *context, MxSource source);
    MxResult mxSourcePause(MxContext *context, MxSource source);
    MxResult mxSourceStop(MxContext *context, MxSource source);
    MxResult mxSourceGetState(MxContext *context, MxSource source, MxSourceState *state);
    MxResult mxSourceSetVolume(MxContext *context, MxSource source, float volume);
    MxResult mxSourceGetVolume(MxContext *context, MxSource source, float *volume);
    MxResult mxSourceSetSpeed(MxContext *context, MxSource source, double speed);
    MxResult mxSourceGetSpeed(MxContext *context, MxSource source, double *speed);

    void mxMixInterleavedStereo(MxContext *context, float* buffer, size_t length);

#ifdef __cplusplus
}
#endif
#endif
