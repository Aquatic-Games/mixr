#ifndef MIXR_VECTOR_H
#define MIXR_VECTOR_H

#include <stddef.h>
#include <stdbool.h>

typedef struct
{
    void* data;
    size_t length;
    size_t capacity;
    size_t elemSize;
} Vector;

Vector mxVectorCreate(size_t elemSize, size_t initialCapacity);
void mxVectorDestroy(Vector *vector);
bool mxVectorAppend(Vector *vector, const void *element);
static void* mxVectorGet(const Vector *vector, size_t index)
{
    return vector->data + (index * vector->elemSize);
}

#endif
