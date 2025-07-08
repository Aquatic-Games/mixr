#include <mixr/mixr.h>
#include <mixr/Device.h>
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

void Callback(void* userData, Uint8* stream, int length)
{
    MxContext* context = (MxContext*) userData;
    mxMixInterleavedStereo(context, (float*) stream, length / 4);
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

    FILE* file = fopen(argv[1], "rb");
    fseek(file, 0, SEEK_END);
    size_t length = ftell(file);
    rewind(file);
    uint8_t* fbuffer = malloc(length * sizeof(uint8_t*));
    fread(fbuffer, length, 1, file);
    fclose(file);

    MxBuffer buffer;
    printf("Creating buffer.\n");
    MX_CHECK_ERROR(mxCreateBuffer(context, fbuffer, length, &buffer));
    free(fbuffer);

    const MxSourceInfo srcInfo =
    {
        .format = { .dataType = MX_DATA_TYPE_I16, .sampleRate = 44100, .channels = 2 }
    };

    MxSource source;
    printf("Creating source.\n");
    MX_CHECK_ERROR(mxCreateSource(context, &srcInfo, &source));
    MX_CHECK_ERROR(mxSourceQueueBuffer(context, source, buffer));
    mxSourceSetSpeed(context, source, 0.5);
    mxSourceSetVolume(context, source, 1.0f);
    mxSourcePlay(context, source);

    MxSourceState state;
    MX_CHECK_ERROR(mxSourceGetState(context, source, &state));

    while (state == MX_SOURCE_STATE_PLAYING)
    {
        sleep(1);
        MX_CHECK_ERROR(mxSourceGetState(context, source, &state));
    }

    mxDestroyDevice(device);

    return 0;
}