#include "mixr/Context.h"

#include <stdio.h>
#include <stdlib.h>

#include "utils/vector.h"

typedef struct
{
    uint32_t sampleRate;
    Vector test;
} Context;

MxResult mxCreateContext(const MxContextInfo* info, MxContext** context)
{
    Context* ctx = malloc(sizeof(Context));
    ctx->sampleRate = info->sampleRate;
    ctx->test = VectorCreate(sizeof(const char*), 0);

    VectorAppend(&ctx->test, "Hello!");
    VectorAppend(&ctx->test, "This");
    VectorAppend(&ctx->test, "is");
    VectorAppend(&ctx->test, "a");
    VectorAppend(&ctx->test, "test!");

    printf("%lu, %lu\n", ctx->test.capacity, ctx->test.length);

    const char* test;
    if (VectorGet(&ctx->test, 3, &test))
        printf("%s\n", test);

    *context = (MxContext*) ctx;
    return MX_RESULT_OK;
}

void mxDestroyContext(MxContext* context)
{
    Context* ctx = (Context*) context;
    free(ctx);
}

const char* mxGetLastErrorString(MxContext *context)
{
    return "";
}
