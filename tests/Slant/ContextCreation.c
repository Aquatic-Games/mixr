#include <stdio.h>
#include <Slant/Context.h>

int main(int argc, char **argv)
{
    SlContextInfo contextInfo = {
        .sampleRate = 44100
    };
    
    SlContext *context;
    SlResult result = slCreateContext(&contextInfo, &context);
    if (result != SL_RESULT_OK)
    {
        printf("Context creation failed: %d\n", result);
        return 1;
    }
    
    result = slDestroyContext(NULL);
    if (result != SL_RESULT_OK)
    {
        printf("Context destruction failed: %d\n", result);
        return 1;
    }
    
    return 0;
}