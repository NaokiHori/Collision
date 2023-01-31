#include "common.h"
#include "particle.h"
#include "output.h"
#include "io.h"


void output(const size_t iter, const double time, const size_t nparticles, const part_t *particles){
  char *dname = make_directory(iter);
  if(dname == NULL){
    return;
  }
  // save as SoA instead of AoS
  double *densities = common_calloc(nparticles, sizeof(double));
  double *radii     = common_calloc(nparticles, sizeof(double));
  double **poss = common_calloc(NDIMS, sizeof(double *));
  double **vels = common_calloc(NDIMS, sizeof(double *));
  for(dim_t dim = 0; dim < NDIMS; dim++){
    poss[dim] = common_calloc(nparticles, sizeof(double));
    vels[dim] = common_calloc(nparticles, sizeof(double));
  }
  // convert linked list to arrays
  for(size_t n = 0; n < nparticles; n++){
    const part_t *p = particles + n;
    densities[n] = p->density;
    radii[n]     = p->radius;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      poss[dim][n] = p->position[dim];
      vels[dim][n] = p->velocity[dim];
    }
  }
  dump_scalar(dname, "iter", sizeof(size_t), dtype_size_t, &iter);
  dump_scalar(dname, "time", sizeof(double), dtype_double, &time);
  dump_scalar(dname, "nparticles", sizeof(size_t), dtype_size_t, &nparticles);
  dump_vector(dname, "densities", nparticles, sizeof(double), dtype_double, densities);
  dump_vector(dname, "radii", nparticles, sizeof(double), dtype_double, radii);
  dump_vectors(dname, "positions", nparticles, poss);
  dump_vectors(dname, "velocities", nparticles, vels);
  // clean-up
  common_free(densities);
  common_free(radii);
  for(dim_t dim = 0; dim < NDIMS; dim++){
    common_free(poss[dim]);
    common_free(vels[dim]);
  }
  common_free(poss);
  common_free(vels);
  common_free(dname);
}

