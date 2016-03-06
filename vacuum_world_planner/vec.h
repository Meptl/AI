#ifndef VEC_H
#define VEC_H

typedef struct vector {
    int *values;
    int length;
    int max_size;
} vector;

// An integer vector(takes in action enums)
vector *vec_init(int size);
void vec_free(vector *vec);
void vec_push(vector *vec, int x);
void vec_pop(vector *vec);

#endif
