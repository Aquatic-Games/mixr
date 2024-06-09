#pragma once

#include <stdint.h>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxContext MxContext;

    typedef enum {
        MX_DATA_TYPE_U8,
        MX_DATA_TYPE_I16,
        MX_DATA_TYPE_I32,
        MX_DATA_TYPE_F32
    } MxDataType;

    typedef enum {
        MX_CHANNELS_MONO,
        MX_CHANNELS_STEREO
    } MxChannels;

    typedef struct {
        MxDataType DataType;
        uint32_t SampleRate;
        MxChannels Channels;
    } MxAudioFormat;

    void mxCreateContext(uint32_t sampleRate, MxContext **pContext);
    void mxDestroyContext(MxContext *context);

    size_t mxContextCreateBuffer(MxContext *context, MxAudioFormat *format, uint8_t* data, size_t dataLength);

    size_t mxContextCreateSource(MxContext *context);

#ifdef __cplusplus
}
#endif