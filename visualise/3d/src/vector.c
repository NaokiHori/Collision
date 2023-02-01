#include <math.h>
#include "vector.h"


real dot(const vector_t v0, const vector_t v1){
  // inner product of two vectors a and b
  real retval = 0.;
  retval += v0.x * v1.x;
  retval += v0.y * v1.y;
  retval += v0.z * v1.z;
  return retval;
}

vector_t normalise(const vector_t v0){
  real norm = sqrt(dot(v0, v0));
  vector_t v1 = {
    .x = v0.x/norm,
    .y = v0.y/norm,
    .z = v0.z/norm
  };
  return v1;
}

