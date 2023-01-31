#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include "common.h"
#include "particle.h"


static double gen_random(const double min, const double max){
  const double val = rand() / (RAND_MAX + 1.);
  return (max - min) * val + min;
}

static void set_density(const size_t nparticles, part_t *particles){
  for(size_t n = 0; n < nparticles; n++){
    particles[n].density = 1.;
  }
}

static void set_position(const double *lengths, const size_t nparticles, part_t *particles){
  const bool write_progress = true;
  for(size_t n0 = 0; n0 < nparticles; n0++){
generate:
    {
      const double r0 = gen_random(0.5, 8.);
      double limits[NDIRS * NDIMS] = {0.};
      for(dim_t dim = 0; dim < NDIMS; dim++){
        limits[dim * NDIRS + DIR_NEG] = 0.;
        limits[dim * NDIRS + DIR_POS] = lengths[dim];
        // shift limit toward inside domain
        limits[dim * NDIRS + DIR_NEG] += r0;
        limits[dim * NDIRS + DIR_POS] -= r0;
      }
      double pos0[NDIMS] = {0.};
      for(dim_t dim = 0; dim < NDIMS; dim++){
        pos0[dim] = gen_random(
            limits[dim * NDIRS + DIR_NEG],
            limits[dim * NDIRS + DIR_POS]
        );
      }
      for(size_t n1 = 0; n1 < n0; n1++){
        const part_t *p1 = &(particles[n1]);
        const double r1 = p1->radius;
        double pos1[NDIMS] = {0.};
        for(dim_t dim = 0; dim < NDIMS; dim++){
          pos1[dim] = p1->position[dim];
        }
        double delta[NDIMS] = {0.};
        for(dim_t dim = 0; dim < NDIMS; dim++){
          delta[dim] = pos1[dim] - pos0[dim];
        }
        // compute l2
        double norm = 0.;
        for(dim_t dim = 0; dim < NDIMS; dim++){
          norm += pow(delta[dim], 2.);
        }
        norm = sqrt(norm);
        const double dist = norm - (r0 + r1);
        if(dist < 0.){
          // two particles are so close that they are overlapped
          // go back and re-generate position
          goto generate;
        }
      }
      // this position is accepted
      // register
      part_t *p0 = particles + n0;
      p0->radius = r0;
      for(dim_t dim = 0; dim < NDIMS; dim++){
        p0->position[dim] = pos0[dim];
      }
      if(write_progress){
        printf("part %8zu (% .3e) @ ", n0, r0);
        for(dim_t dim = 0; dim < NDIMS; dim++){
          printf("% .3e%c", pos0[dim], dim == NDIMS - 1 ? '\n' : ' ');
        }
      }
    }
  }
}

static void set_velocity(const size_t nparticles, part_t *particles){
  const double magnitude = 1.;
  for(size_t n = 0; n < nparticles; n++){
    part_t *p = particles + n;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      p->velocity[dim] = gen_random(
          -magnitude,
          +magnitude
      );
    }
  }
}

static void check_volume_fraction(const double *lengths, const size_t nparticles, part_t *particles){
  double total = 1.;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    total *= lengths[dim];
  }
  double pfrac = 0.;
  for(size_t n = 0; n < nparticles; n++){
    const double r = particles[n].radius;
#if NDIMS == 2
    pfrac += M_PI * r * r;
#elif NDIMS == 3
    pfrac += 4. / 3. * M_PI * r * r * r;
#elif NDIMS == 4
    pfrac += 1. / 2. * M_PI * M_PI * r * r * r * r;
#else
#error "define"
#endif
  }
  printf("%zu particles, fraction: % .2f%%\n", nparticles, 100. * pfrac / total);
}

part_t *init_particles(const double *lengths, const size_t nparticles){
  const bool use_time_seed = false;
  if(use_time_seed){
    // time-dependent seed
    srand(time(NULL));
  }else{
    // fixed seed
    srand(1 << 8);
  }
  part_t *particles = common_calloc(nparticles, sizeof(part_t));
  set_density(nparticles, particles);
  set_position(lengths, nparticles, particles);
  set_velocity(nparticles, particles);
  check_volume_fraction(lengths, nparticles, particles);
  return particles;
}

void finalise_particles(part_t *particles){
  common_free(particles);
}

