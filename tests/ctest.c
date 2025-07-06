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

int main(int argc, char** argv)
{
    const MxContextInfo info =
    {
        .sampleRate = 44100
    };

    MxContext* context;
    printf("Creating context.\n");
    MX_CHECK_ERROR(mxCreateContext(&info, &context));

    printf("Destroying context.\n");
    mxDestroyContext(context);

    return 0;
}