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

Vector VectorCreate(size_t elemSize, size_t initialCapacity);
void VectorDestroy(Vector *vector);
bool VectorAppend(Vector *vector, const void *element);
void* VectorGet(const Vector *vector, size_t index);

#endif
