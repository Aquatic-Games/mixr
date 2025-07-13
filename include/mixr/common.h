#ifndef MIXR_COMMON_H
#define MIXR_COMMON_H

#include <stdint.h>

#if defined(_WIN32)
#define MIXR_C_API __declspec(dllexport)
#else
#define MIXR_C_API
#endif

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
    MX_RESULT_SOURCE_QUEUE_EMPTY,
    MX_RESULT_UNSUPPORTED_FORMAT,
    MX_RESULT_FILE_NOT_FOUND,
    MX_RESULT_MALFORMED_FILE,
    MX_RESULT_UNEXPECTED_EOF
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

MIXR_C_API const char* mxGetLastErrorString();

static MIXR_C_API uint8_t mxDataTypeToBits(const MxDataType type)
{
    switch (type)
    {
        case MX_DATA_TYPE_I8:
            return 8;
        case MX_DATA_TYPE_I16:
            return 16;
        case MX_DATA_TYPE_I32:
            return 32;
        case MX_DATA_TYPE_F32:
            return 32;
    }
}

static MIXR_C_API uint8_t mxDataTypeToBytes(const MxDataType type)
{
    return mxDataTypeToBits(type) / 8;
}

#ifdef __cplusplus
}
#endif

#endif
