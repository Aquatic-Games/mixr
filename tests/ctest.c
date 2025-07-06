#include <mixr/mixr.h>

#include <stdio.h>

#define MX_CHECK_ERROR(Operation) {\
    MxResult res = Operation;\
    if (res != MX_RESULT_OK) {\
        printf("Operation %s failed.", #Operation);\
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