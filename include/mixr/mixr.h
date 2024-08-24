#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef _MSC_VER
#define MX_C_API_EXPORT __declspec(dllexport)
#else
#define MX_C_API_EXPORT
#endif

#ifdef __cplusplus
extern "C" {
#endif

    typedef struct MxContext MxContext;
    typedef struct MxDevice MxDevice;

    typedef size_t MxAudioBuffer;
    typedef size_t MxAudioSource;

    typedef enum {
        MX_DATA_TYPE_I8,
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

    MX_C_API_EXPORT void mxCreateContext(uint32_t sampleRate, MxContext **pContext);
    MX_C_API_EXPORT void mxDestroyContext(MxContext *context);

    // TODO: Move this to a separate file
    // TODO: This implementation is temporary. This should be done properly at some point.
    MX_C_API_EXPORT void mxCreateSDLDevice(uint32_t sampleRate, uint16_t periodSize, MxDevice **pDevice);

    MX_C_API_EXPORT void mxDeviceGetContext(MxDevice *device, MxContext **pContext);
    MX_C_API_EXPORT void mxDestroyDevice(MxDevice *device);

    MX_C_API_EXPORT MxAudioBuffer mxContextCreateBuffer(MxContext *context, uint8_t *data, size_t dataLength);
    MX_C_API_EXPORT MxAudioSource mxContextCreateSource(MxContext *context, MxSourceDescription *description);

    MX_C_API_EXPORT void mxContextSetMasterVolume(MxContext *context, float volume);

    MX_C_API_EXPORT void mxSourceSubmitBuffer(MxContext *context, MxAudioSource source, MxAudioBuffer buffer);
    MX_C_API_EXPORT void mxSourceClearBuffers(MxContext *context, MxAudioSource source);
    MX_C_API_EXPORT void mxSourcePlay(MxContext *context, MxAudioSource source);
    MX_C_API_EXPORT void mxSourcePause(MxContext *context, MxAudioSource source);
    MX_C_API_EXPORT void mxSourceStop(MxContext *context, MxAudioSource source);
    MX_C_API_EXPORT void mxSourceSetSpeed(MxContext *context, MxAudioSource source, double speed);
    MX_C_API_EXPORT void mxSourceSetVolume(MxContext *context, MxAudioSource source, float volume);
    MX_C_API_EXPORT void mxSourceSetLooping(MxContext *context, MxAudioSource source, bool looping);
    MX_C_API_EXPORT void mxSourceSetPanning(MxContext *context, MxAudioSource source, float panning);
    MX_C_API_EXPORT void mxSourceSetChannelVolumes(MxContext *context, MxAudioSource source, float volumeL, float volumeR);

#ifdef __cplusplus
}
#endif