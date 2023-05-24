#ifndef MIXR_H
#define MIXR_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  DATA_TYPE_I8,
  DATA_TYPE_U8,
  DATA_TYPE_I16,
  DATA_TYPE_U16,
  DATA_TYPE_I32,
  DATA_TYPE_F32,
  DATA_TYPE_F64,
} DataType;

typedef enum {
  MIXR_RESULT_OK,
  MIXR_RESULT_INVALID_BUFFER,
  MIXR_RESULT_INVALID_VOICE,
  MIXR_RESULT_INVALID_VALUE,
  MIXR_RESULT_INVALID_OPERATION,
  MIXR_RESULT_OTHER,
} MixrResult;

typedef enum {
  PLAY_STATE_STOPPED,
  PLAY_STATE_PAUSED,
  PLAY_STATE_PLAYING,
} PlayState;

typedef struct AudioSystem AudioSystem;

typedef struct Stream Stream;

typedef struct {
  DataType dataType;
  uint32_t sampleRate;
  uint8_t channels;
} AudioFormat;

typedef struct {
  AudioFormat format;
} BufferDescription;

typedef struct {
  uintptr_t id;
} AudioBuffer;

typedef struct {
  double volume;
  double speed;
  double panning;
  bool looping;
  uintptr_t loopStart;
  uintptr_t loopEnd;
  uintptr_t startSample;
} PlayProperties;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

AudioSystem *mxCreateSystem(uint32_t sample_rate, uint16_t voices);

void mxDestroySystem(AudioSystem *system);

MixrResult mxCreateBuffer(AudioSystem *system,
                          BufferDescription description,
                          const void *data,
                          uintptr_t length,
                          AudioBuffer *buffer);

MixrResult mxDestroyBuffer(AudioSystem *system, AudioBuffer buffer);

MixrResult mxUpdateBuffer(AudioSystem *system,
                          AudioBuffer buffer,
                          AudioFormat format,
                          const void *data,
                          uintptr_t length);

MixrResult mxPlayBuffer(AudioSystem *system,
                        AudioBuffer buffer,
                        uint16_t voice,
                        PlayProperties properties);

MixrResult mxGetPlayProperties(AudioSystem *system, uint16_t voice, PlayProperties *properties);

MixrResult mxSetPlayProperties(AudioSystem *system, uint16_t voice, PlayProperties properties);

MixrResult mxGetVoiceState(AudioSystem *system, uint16_t voice, PlayState *state);

MixrResult mxSetVoiceState(AudioSystem *system, uint16_t voice, PlayState state);

MixrResult mxGetPositionSamples(AudioSystem *system, uint16_t voice, uintptr_t *position);

MixrResult mxSetPositionSamples(AudioSystem *system, uint16_t voice, uintptr_t position);

MixrResult mxGetPosition(AudioSystem *system, uint16_t voice, double *position);

MixrResult mxSetPosition(AudioSystem *system, uint16_t voice, double position);

void mxReadBufferStereoF32(AudioSystem *system, float *buffer, uintptr_t length);

MixrResult mxStreamLoadWav(const char *path, Stream **stream);

void mxStreamFree(Stream *stream);

void mxStreamGetFormat(Stream *stream, AudioFormat *format);

void mxStreamGetPcm(Stream *stream, void *data, uintptr_t *length);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* MIXR_H */
