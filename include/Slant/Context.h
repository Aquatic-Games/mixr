#ifndef SLANT_CONTEXT_H
#define SLANT_CONTEXT_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// todo define this correctly on windows
#define SL_EXPORT

typedef struct SlContext SlContext;
typedef size_t SlBuffer;

// Contains various results.
typedef enum
{
    // Everything is okay.
    SL_RESULT_OK,
    
    // An unknown error occurred. This likely means the error hasn't been added to the result enum.
    // If you get this error, open an issue!
    SL_RESULT_UNKNOWN_ERROR,
    
    // An allocation failed. This can often mean the system is out of memory.
    SL_RESULT_OUT_OF_MEMORY,
    
    // A non-specific invalid value was provided, for example, if a null pointer was provided.
    SL_RESULT_INVALID_VALUE
} SlResult;

// Describes how a context should be created.
typedef struct
{
    // The sampling rate, in hz. Must be greater than 0, otherwise SL_RESULT_INVALID_VALUE will be returned.
    uint32_t sampleRate;
} SlContextInfo;

typedef struct
{
    size_t bufferSize;
} SlBufferInfo;

SL_EXPORT SlResult slCreateContext(const SlContextInfo *info, SlContext **context);
SL_EXPORT SlResult slDestroyContext(SlContext *context);

SL_EXPORT SlResult slCreateBuffer(SlContext *context, const SlBufferInfo *info, SlBuffer *buffer);
SL_EXPORT SlResult slDestroyBuffer(SlContext *context, SlBuffer buffer);

#ifdef __cplusplus
}
#endif

#endif
