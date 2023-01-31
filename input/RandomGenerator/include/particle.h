#if !defined(PARTICLE_H)
#define PARTICLE_H

#include <stdlib.h>
#include "common.h"


typedef struct part_t_ {
  size_t index;
  double density;
  double radius;
  double position[NDIMS];
  double velocity[NDIMS];
} part_t;

extern part_t *init_particles(const double *lengths, const size_t nparticles);
extern void finalise_particles(part_t *particle);

#endif // PARTICLE_H
