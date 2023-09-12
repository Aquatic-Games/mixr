#ifndef MIXR_H
#define MIXR_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  MX_DATA_TYPE_I8,
  MX_DATA_TYPE_U8,
  MX_DATA_TYPE_I16,
  MX_DATA_TYPE_U16,
  MX_DATA_TYPE_I32,
  MX_DATA_TYPE_F32,
  MX_DATA_TYPE_F64,
} MxDataType;

typedef enum {
  MX_PLAY_STATE_STOPPED,
  MX_PLAY_STATE_PAUSED,
  MX_PLAY_STATE_PLAYING,
} MxPlayState;

typedef enum {
  MX_RESULT_OK,
  MX_RESULT_INVALID_BUFFER,
  MX_RESULT_INVALID_VOICE,
  MX_RESULT_INVALID_VALUE,
  MX_RESULT_INVALID_OPERATION,
  MX_RESULT_FILE_NOT_FOUND,
  MX_RESULT_OTHER,
} MxResult;

typedef struct MxAudioSystem MxAudioSystem;

typedef struct MxStream MxStream;

typedef struct {
  MxDataType dataType;
  uint32_t sampleRate;
  uint8_t channels;
} MxAudioFormat;

typedef struct {
  MxAudioFormat format;
} MxBufferDescription;

typedef struct {
  uintptr_t id;
} MxAudioBuffer;

typedef struct {
  double volume;
  double speed;
  double panning;
  bool looping;
  uintptr_t loopStart;
  uintptr_t loopEnd;
  uintptr_t startSample;
} MxPlayProperties;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

MxAudioSystem *mxCreateSystem(uint32_t sample_rate, uint16_t voices);

void mxDestroySystem(MxAudioSystem *system);

MxResult mxCreateBuffer(MxAudioSystem *system,
                        MxBufferDescription description,
                        const void *data,
                        uintptr_t length,
                        MxAudioBuffer *buffer);

MxResult mxDestroyBuffer(MxAudioSystem *system, MxAudioBuffer buffer);

MxResult mxUpdateBuffer(MxAudioSystem *system,
                        MxAudioBuffer buffer,
                        MxAudioFormat format,
                        const void *data,
                        uintptr_t length);

MxResult mxPlayBuffer(MxAudioSystem *system,
                      MxAudioBuffer buffer,
                      uint16_t voice,
                      MxPlayProperties properties);

MxResult mxQueueBuffer(MxAudioSystem *system, MxAudioBuffer buffer, uint16_t voice);

MxResult mxGetPlayProperties(MxAudioSystem *system, uint16_t voice, MxPlayProperties *properties);

MxResult mxSetPlayProperties(MxAudioSystem *system, uint16_t voice, MxPlayProperties properties);

MxResult mxGetVoiceState(MxAudioSystem *system, uint16_t voice, MxPlayState *state);

MxResult mxSetVoiceState(MxAudioSystem *system, uint16_t voice, MxPlayState state);

MxResult mxGetPositionSamples(MxAudioSystem *system, uint16_t voice, uintptr_t *position);

MxResult mxSetPositionSamples(MxAudioSystem *system, uint16_t voice, uintptr_t position);

MxResult mxGetPosition(MxAudioSystem *system, uint16_t voice, double *position);

MxResult mxSetPosition(MxAudioSystem *system, uint16_t voice, double position);

void mxSetBufferFinishedCallback(MxAudioSystem *system, void (*callback)(MxAudioBuffer, uint16_t));

void mxReadBufferStereoF32(MxAudioSystem *system, float *buffer, uintptr_t length);

uint16_t mxGetNumVoices(MxAudioSystem *system);

MxResult mxStreamLoadFile(const char *path, MxStream **stream);

MxResult mxStreamLoadWavFile(const char *path, MxStream **stream);

MxResult mxStreamLoadVorbisFile(const char *path, MxStream **stream);

void mxStreamFree(MxStream *stream);

void mxStreamGetFormat(MxStream *stream, MxAudioFormat *format);

uintptr_t mxStreamGetBuffer(MxStream *stream, uint8_t *buffer, uintptr_t length);

void mxStreamGetPcm(MxStream *stream, void *data, uintptr_t *length);

void mxStreamSeek(MxStream *stream, double position);

void mxStreamSeekSamples(MxStream *stream, uintptr_t position);

void mxStreamRestart(MxStream *stream);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* MIXR_H */
