#include <mixr/mixr.h>
#include <SDL2/SDL.h>
#include <unistd.h>

#include <stdio.h>

#define MX_CHECK_ERROR(Operation) {\
    MxResult res = Operation;\
    if (res != MX_RESULT_OK) {\
        printf("Operation %s failed: %s", #Operation, mxGetLastErrorString());\
        return 1;\
    }\
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        printf("Invalid number of arguments. Expected 1.\n");
        return 1;
    }

    const MxDeviceInfo devInfo =
    {
        .sampleRate = 48000
    };

    MxDevice* device;
    MX_CHECK_ERROR(mxCreateDevice(&devInfo, &device));
    MxContext* context = mxDeviceGetContext(device);
    //mxSetMasterVolume(context, 0.25f);

    MxStream* stream;
    //MX_CHECK_ERROR(mxStreamLoadWav(argv[1], &stream));
    MX_CHECK_ERROR(mxStreamLoadFlac(argv[1], &stream));

    size_t bufSize;
    mxStreamGetPCM(stream, NULL, &bufSize);
    uint8_t* buf = malloc(bufSize);
    mxStreamGetPCM(stream, buf, &bufSize);

    MxBuffer buffer;
    printf("Creating buffer.\n");
    MX_CHECK_ERROR(mxCreateBuffer(context, buf, bufSize, &buffer));

    const MxSourceInfo srcInfo =
    {
        .format = mxStreamGetAudioFormat(stream)
    };

    mxDestroyStream(stream);

    MxSource source;
    printf("Creating source.\n");
    MX_CHECK_ERROR(mxCreateSource(context, &srcInfo, &source));
    MX_CHECK_ERROR(mxSourceQueueBuffer(context, source, buffer));
    mxSourceSetSpeed(context, source, 1.0);
    mxSourceSetVolume(context, source, 1.0f);
    mxSourcePlay(context, source);

    MxSourceState state;
    MX_CHECK_ERROR(mxSourceGetState(context, source, &state));

    uint32_t secs = 0;

    while (state == MX_SOURCE_STATE_PLAYING)
    {
        printf("%d\n", secs);
        secs++;
        sleep(1);
        MX_CHECK_ERROR(mxSourceGetState(context, source, &state));
    }

    mxDestroyDevice(device);

    return 0;
}