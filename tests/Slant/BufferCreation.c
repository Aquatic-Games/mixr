#include <Slant/Context.h>
#include <stdio.h>

int main(int argc, char **argv)
{
    const SlContextInfo contextInfo = {
        .sampleRate = 44100
    };
    
    SlContext *context;
    SlResult result = slCreateContext(&contextInfo, &context);
    if (result != SL_RESULT_OK)
    {
        printf("Context creation failed: %d\n", result);
        return 1;
    }
    
    const SlBufferInfo bufferInfo = {
        .bufferSize = 1000
    };
    
    SlBuffer buffer;
    result = slCreateBuffer(context, &bufferInfo, &buffer);
    if (result != SL_RESULT_OK)
    {
        printf("Buffer creation failed: %d\n", result);
        return 1;
    }
    
    slDestroyContext(context);
    
    return 0;
}