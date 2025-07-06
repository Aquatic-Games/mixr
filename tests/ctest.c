#include <mixr/mixr.h>
#include <SDL2/SDL.h>

#include <stdio.h>

#define MX_CHECK_ERROR(Operation) {\
    MxResult res = Operation;\
    if (res != MX_RESULT_OK) {\
        printf("Operation %s failed: %s", #Operation, mxGetLastErrorString(context));\
        return 1;\
    }\
}

void Callback(void* userData, Uint8* stream, int length)
{

}

int main(int argc, char** argv)
{
    if (SDL_Init(SDL_INIT_AUDIO) < 0)
    {
        printf("Failed to initialize SDL: %s", SDL_GetError());
        return 1;
    }

    const MxContextInfo info =
    {
        .sampleRate = 44100
    };

    MxContext* context;
    printf("Creating context.\n");
    MX_CHECK_ERROR(mxCreateContext(&info, &context));

    SDL_AudioSpec spec =
    {
        .freq = info.sampleRate,
        .format = AUDIO_F32,
        .channels = 2,
        .samples = 512,
        .callback = Callback,
        .userdata = context
    };

    SDL_AudioDeviceID device = SDL_OpenAudioDevice(NULL, 0, &spec, NULL, 0);

    FILE* file = fopen("/home/aqua/Music/TESTFILES/Feeling-16bitshort.raw", "rb");
    fseek(file, 0, SEEK_END);
    size_t length = ftell(file);
    rewind(file);
    uint8_t* fbuffer = malloc(length * sizeof(uint8_t*));
    fread(fbuffer, length, 1, file);
    fclose(file);

    MxBuffer buffer;
    printf("Creating buffer.\n");
    MX_CHECK_ERROR(mxCreateBuffer(context, fbuffer, length, &buffer));

    SDL_CloseAudioDevice(device);

    printf("Destroying context.\n");
    mxDestroyContext(context);

    return 0;
}