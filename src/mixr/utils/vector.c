#include "vector.h"

#include <stdlib.h>
#include <string.h>

Vector mxVectorCreate(const size_t elemSize, const size_t initialCapacity)
{
    Vector vec;
    vec.length = 0;
    vec.elemSize = elemSize;
    vec.capacity = initialCapacity;
    if (initialCapacity > 0)
        vec.data = malloc(initialCapacity * elemSize);
    else
        vec.data = NULL;
    return vec;
}

void mxVectorDestroy(Vector* vector)
{
    free(vector->data);
    vector->data = NULL;
}

bool mxVectorAppend(Vector* vector, const void* element)
{
    if (vector->data == NULL)
    {
        vector->capacity = 1;
        vector->data = malloc(vector->capacity * vector->elemSize);
    }

    if (vector->length + 1 > vector->capacity)
    {
        vector->capacity <<= 1;
        void* newData = realloc(vector->data, (vector->capacity * vector->elemSize));
        if (!newData)
            return false;
        vector->data = newData;
    }

    if (!memcpy(vector->data + (vector->length * vector->elemSize), element, vector->elemSize))
        return false;

    vector->length++;

    return true;
}
