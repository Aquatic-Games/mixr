#include "internal.h"

static const char* errorString;

void mxSetErrorString(const char* string)
{
    errorString = string;
}

const char* mxGetLastErrorString()
{
    return errorString;
}