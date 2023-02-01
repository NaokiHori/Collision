#if !defined(VECTOR_H)
#define VECTOR_H

#include "common.h"

typedef struct {
  real x;
  real y;
  real z;
} vector_t;

extern real dot(const vector_t v0, const vector_t v1);
extern vector_t normalise(const vector_t v0);

#endif // VECTOR_H
