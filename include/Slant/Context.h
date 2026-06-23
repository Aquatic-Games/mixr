#ifndef SLANT_CONTEXT_H
#define SLANT_CONTEXT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// todo define this correctly on windows
#define SL_EXPORT

typedef struct SlContext SlContext;

typedef enum
{
    // Everything is okay.
    SL_RESULT_OK,
    
    // An unknown error occurred. This likely means the error hasn't been added to the result enum.
    // If you get this error, open an issue!
    SL_RESULT_UNKNOWN_ERROR,
    
    // A non-specific invalid value was provided, for example, if a null pointer was provided.
    SL_RESULT_INVALID_VALUE
} SlResult;

typedef struct
{
    uint32_t sampleRate;
} SlContextInfo;

SL_EXPORT SlResult slCreateContext(SlContextInfo *info, SlContext **context);
SL_EXPORT SlResult slDestroyContext(SlContext *context);

#ifdef __cplusplus
}
#endif

#endif
