#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef _WIN32
#define MIXR_C_API __declspec(dllexport)
#else
#define MIXR_C_API
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

    MIXR_C_API void mxCreateContext(uint32_t sampleRate, MxContext **pContext);
    MIXR_C_API void mxDestroyContext(MxContext *context);

    // TODO: Move this to a separate file
    // TODO: This implementation is temporary. This should be done properly at some point.
    MIXR_C_API void mxCreateSDLDevice(uint32_t sampleRate, uint16_t periodSize, MxDevice **pDevice);

    MIXR_C_API void mxDeviceGetContext(MxDevice *device, MxContext **pContext);
    MIXR_C_API void mxDestroyDevice(MxDevice *device);

    MIXR_C_API MxAudioBuffer mxContextCreateBuffer(MxContext *context, uint8_t *data, size_t dataLength);
    MIXR_C_API void mxContextDestroyBuffer(MxContext *context, MxAudioBuffer buffer);

    MIXR_C_API MxAudioSource mxContextCreateSource(MxContext *context, MxSourceDescription *description);
    MIXR_C_API void mxContextDestroySource(MxContext *context, MxAudioSource source);

    MIXR_C_API void mxContextUpdateBuffer(MxContext *context, MxAudioBuffer buffer, uint8_t *data, size_t dataLength);

    MIXR_C_API void mxSourceSubmitBuffer(MxContext *context, MxAudioSource source, MxAudioBuffer buffer);
    MIXR_C_API void mxSourceClearBuffers(MxContext *context, MxAudioSource source);
    MIXR_C_API void mxSourcePlay(MxContext *context, MxAudioSource source);
    MIXR_C_API void mxSourcePause(MxContext *context, MxAudioSource source);
    MIXR_C_API void mxSourceStop(MxContext *context, MxAudioSource source);
    MIXR_C_API void mxSourceSetSpeed(MxContext *context, MxAudioSource source, double speed);
    MIXR_C_API void mxSourceSetVolume(MxContext *context, MxAudioSource source, float volume);
    MIXR_C_API void mxSourceSetLooping(MxContext *context, MxAudioSource source, bool looping);
    MIXR_C_API void mxSourceSetPanning(MxContext *context, MxAudioSource source, float panning);
    MIXR_C_API void mxSourceSetChannelVolumes(MxContext *context, MxAudioSource source, float volumeL, float volumeR);

    MIXR_C_API void mxContextSetMasterVolume(MxContext *context, float volume);

#ifdef __cplusplus
}
#endif