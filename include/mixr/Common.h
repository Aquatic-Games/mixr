#ifndef MIXR_COMMON_H
#define MIXR_COMMON_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    MX_RESULT_OK,
    MX_RESULT_INVALID_BUFFER,
    MX_RESULT_INVALID_SOURCE,
    MX_RESULT_OUT_OF_MEMORY
} MxResult;

#ifdef __cplusplus
}
#endif

#endif
