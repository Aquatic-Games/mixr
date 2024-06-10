#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxContext MxContext;
    typedef struct MxDevice MxDevice;

    typedef size_t MxAudioBuffer;
    typedef size_t MxAudioSource;

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

    // TODO: Move this to a separate file
    // TODO: This implementation is temporary. This should be done properly at some point.
    void mxCreateSDLDevice(uint32_t sampleRate, uint16_t periodSize, MxDevice **pDevice);

    void mxDeviceGetContext(MxDevice *device, MxContext **pContext);
    void mxDestroyDevice(MxDevice *device);

    MxAudioBuffer mxContextCreateBuffer(MxContext *context, MxAudioFormat *format, uint8_t* data, size_t dataLength);
    MxAudioSource mxContextCreateSource(MxContext *context);

    void mxContextSetMasterVolume(MxContext *context, float volume);

    void mxSourceSubmitBuffer(MxContext *context, MxAudioSource source, MxAudioBuffer buffer);
    void mxSourcePlay(MxContext *context, MxAudioSource source);
    void mxSourceStop(MxContext *context, MxAudioSource source);
    void mxSourceSetSpeed(MxContext *context, MxAudioSource source, double speed);
    void mxSourceSetVolume(MxContext *context, MxAudioSource source, float volume);
    void mxSourceSetLooping(MxContext *context, MxAudioSource source, bool looping);
    void mxSourceSetPanning(MxContext *context, MxAudioSource source, float panning);

#ifdef __cplusplus
}
#endif