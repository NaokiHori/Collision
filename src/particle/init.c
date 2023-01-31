#include "common.h"
#include "config.h"
#include "particle.h"
#include "io.h"


void init_particles(const double time, size_t *nparticles, part_t **particles){
  const char *dname = config.get_string("input_directory");
  load_scalar(dname, "nparticles", sizeof(size_t), dtype_size_t, nparticles);
  *particles = common_calloc(*nparticles, sizeof(part_t));
  for(size_t n = 0; n < *nparticles; n++){
    part_t *p = *particles + n;
    p->position = common_calloc(NDIMS, sizeof(double));
    p->velocity = common_calloc(NDIMS, sizeof(double));
  }
  // assign fixed values
  for(size_t n = 0; n < *nparticles; n++){
    part_t *p = *particles + n;
    p->time = time;
  }
  // densities
  {
    double *buf = common_calloc(*nparticles, sizeof(double));
    load_vector(dname, "densities", *nparticles, sizeof(double), dtype_double, buf);
    for(size_t n = 0; n < *nparticles; n++){
      (*particles)[n].density = buf[n];
    }
    common_free(buf);
  }
  // radii
  {
    double *buf = common_calloc(*nparticles, sizeof(double));
    load_vector(dname, "radii", *nparticles, sizeof(double), dtype_double, buf);
    for(size_t n = 0; n < *nparticles; n++){
      (*particles)[n].radius = buf[n];
    }
    common_free(buf);
  }
  // positions
  {
    double **buf = common_calloc(NDIMS, sizeof(double *));
    for(dim_t dim = 0; dim < NDIMS; dim++){
      buf[dim] = common_calloc(*nparticles, sizeof(double));
    }
    load_vectors(dname, "positions", *nparticles, buf);
    for(dim_t dim = 0; dim < NDIMS; dim++){
      for(size_t n = 0; n < *nparticles; n++){
        (*particles)[n].position[dim] = buf[dim][n];
      }
    }
    for(dim_t dim = 0; dim < NDIMS; dim++){
      common_free(buf[dim]);
    }
    common_free(buf);
  }
  // velocities
  {
    double **buf = common_calloc(NDIMS, sizeof(double *));
    for(dim_t dim = 0; dim < NDIMS; dim++){
      buf[dim] = common_calloc(*nparticles, sizeof(double));
    }
    load_vectors(dname, "velocities", *nparticles, buf);
    for(dim_t dim = 0; dim < NDIMS; dim++){
      for(size_t n = 0; n < *nparticles; n++){
        (*particles)[n].velocity[dim] = buf[dim][n];
      }
    }
    for(dim_t dim = 0; dim < NDIMS; dim++){
      common_free(buf[dim]);
    }
    common_free(buf);
  }
}

