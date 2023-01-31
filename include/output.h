#if !defined(OUTPUT_H)
#define OUTPUT_H

#include <stddef.h>
#include "particle.h"

extern void output(const size_t iter, const double time, const size_t nparticles, const part_t *particles);

#endif // OUTPUT_H
