#include "common.h" // M_PI
#include "particle.h"


double compute_p_volume(const double r){
#if NDIMS == 2
  return M_PI * r * r;
#elif NDIMS == 3
  return 4. / 3. * M_PI * r * r * r;
#elif NDIMS == 4
  return 1. / 2. * M_PI * M_PI * r * r * r * r;
#else
#error "define"
#endif
}

double compute_p_mass(const double d, const double r){
  return d * compute_p_volume(r);
}

