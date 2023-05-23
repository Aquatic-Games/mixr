#ifndef MIXR_H
#define MIXR_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  AUDIO_RESULT_OK,
  AUDIO_RESULT_INVALID_BUFFER,
  AUDIO_RESULT_INVALID_VOICE,
  AUDIO_RESULT_INVALID_VALUE,
  AUDIO_RESULT_INVALID_OPERATION,
  AUDIO_RESULT_OTHER,
} AudioResult;

typedef enum {
  DATA_TYPE_I8,
  DATA_TYPE_U8,
  DATA_TYPE_I16,
  DATA_TYPE_U16,
  DATA_TYPE_I32,
  DATA_TYPE_F32,
  DATA_TYPE_F64,
} DataType;

typedef struct AudioSystem AudioSystem;

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

typedef void *Stream;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

AudioSystem *mxCreateSystem(uint32_t sample_rate, uint16_t voices);

void mxDestroySystem(AudioSystem *system);

AudioResult mxCreateBuffer(AudioSystem *system,
                           BufferDescription description,
                           const void *data,
                           uintptr_t length,
                           AudioBuffer *buffer);

AudioResult mxDestroyBuffer(AudioSystem *system, AudioBuffer buffer);

AudioResult mxPlayBuffer(AudioSystem *system,
                         AudioBuffer buffer,
                         uint16_t voice,
                         PlayProperties properties);

void mxReadBufferStereoF32(AudioSystem *system, float *buffer, uintptr_t length);

Stream mxStreamLoadWav(const char *path);

void mxStreamFree(Stream stream);

void mxStreamWavGetFormat(Stream stream, AudioFormat *format);

void mxStreamWavGetPcm(Stream stream, void *data, uintptr_t *length);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* MIXR_H */
