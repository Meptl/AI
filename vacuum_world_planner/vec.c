#include <stdio.h>
#include <stdlib.h>

#include "vec.h"

// An action array that expands
vector *vec_init(int size) {
    vector *vec = malloc(sizeof(*vec));
    vec->length = 0;
    vec->max_size = size;
    vec->values = malloc(sizeof(int) * size);

    return vec;
}

void vec_free(vector *vec) {
    free(vec->values);
    free(vec);
}

void vec_push(vector *vec, int x) {
    // Exponential array growth.
    if (vec->length == vec->max_size) {
        vec->max_size = vec->max_size * 2;
        // If I'm out of memory, I'll just crash.
        int *tmp = realloc(vec->values, sizeof(int) * vec->max_size);
        if (tmp) {
            vec->values = tmp;
        }
        else {
            fprintf(stderr, "Out of memory\n");
            exit(-1);
        }
    }
    vec->values[vec->length++] = x;
}

void vec_pop(vector *vec) {
    if (vec->length == 0) {
        fprintf(stderr, "Popped empty vec\n");
        return;
    }
    vec->length--;
}
