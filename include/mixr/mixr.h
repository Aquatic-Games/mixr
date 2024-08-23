#pragma once

#include <stdint.h>
#include <stddef.h>

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

    typedef enum {
        MX_SOURCE_TYPE_PCM,
        MX_SOURCE_TYPE_ADPCM
    } MxSourceType;

    typedef struct {
        size_t ChunkSize;
    } MxADPCMDescription;

    typedef struct {
        MxSourceType Type;
        MxAudioFormat Format;

        union {
            MxADPCMDescription ADPCM;
        };
    } MxSourceDescription;

    void mxCreateContext(uint32_t sampleRate, MxContext **pContext);
    void mxDestroyContext(MxContext *context);

    // TODO: Move this to a separate file
    // TODO: This implementation is temporary. This should be done properly at some point.
    void mxCreateSDLDevice(uint32_t sampleRate, uint16_t periodSize, MxDevice **pDevice);

    void mxDeviceGetContext(MxDevice *device, MxContext **pContext);
    void mxDestroyDevice(MxDevice *device);

    MxAudioBuffer mxContextCreateBuffer(MxContext *context, uint8_t *data, size_t dataLength);
    MxAudioSource mxContextCreateSource(MxContext *context, MxSourceDescription *description);

    void mxContextSetMasterVolume(MxContext *context, float volume);

    void mxSourceSubmitBuffer(MxContext *context, MxAudioSource source, MxAudioBuffer buffer);
    void mxSourceClearBuffers(MxContext *context, MxAudioSource source);
    void mxSourcePlay(MxContext *context, MxAudioSource source);
    void mxSourcePause(MxContext *context, MxAudioSource source);
    void mxSourceStop(MxContext *context, MxAudioSource source);
    void mxSourceSetSpeed(MxContext *context, MxAudioSource source, double speed);
    void mxSourceSetVolume(MxContext *context, MxAudioSource source, float volume);
    void mxSourceSetLooping(MxContext *context, MxAudioSource source, bool looping);
    void mxSourceSetPanning(MxContext *context, MxAudioSource source, float panning);
    void mxSourceSetChannelVolumes(MxContext *context, MxAudioSource source, float volumeL, float volumeR);

#ifdef __cplusplus
}
#endif