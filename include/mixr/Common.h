#ifndef MIXR_COMMON_H
#define MIXR_COMMON_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    MX_RESULT_OK,
    MX_RESULT_UNKNOWN_ERROR,
    MX_RESULT_GENERAL_FAILURE,
    MX_RESULT_INVALID_BUFFER,
    MX_RESULT_INVALID_SOURCE,
    MX_RESULT_INVALID_NUM_CHANNELS,
    MX_RESULT_OUT_OF_MEMORY,
    MX_RESULT_SOURCE_QUEUE_EMPTY
} MxResult;

typedef enum
{
    //MX_DATA_TYPE_U8,
    MX_DATA_TYPE_I8,
    MX_DATA_TYPE_I16,
    MX_DATA_TYPE_I32,
    MX_DATA_TYPE_F32
} MxDataType;

typedef struct
{
    MxDataType dataType;
    uint32_t sampleRate;
    uint8_t channels;
} MxAudioFormat;

const char* mxGetLastErrorString();

#ifdef __cplusplus
}
#endif

#endif
