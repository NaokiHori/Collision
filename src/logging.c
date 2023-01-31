#include <stdio.h>
#include "cell.h"
#include "event.h"
#include "particle.h"
#include "logging.h"
#include "io.h"


static void check_number_of_events(const double time, const size_t ncells, cell_t * const *cells){
  size_t nevents = 0;
  for(size_t n = 0; n < ncells; n++){
    const cell_t *cell = *(cells + n);
    const elist_t *elist = cell->elist;
    const enode_t *enode_root = elist->enode;
    for(const enode_t *enode = enode_root; enode; enode = enode->next){
      nevents += 1;
    }
  }
  FILE *fp = my_fopen("output/log/nevents.dat", "a");
  if(fp == NULL){
    return ;
  }
  fprintf(fp, "% .3e %16zu\n", time, nevents);
  fclose(fp);
}

static void check_number_of_particles(const double time, const size_t ncells, cell_t * const *cells){
  size_t nparticles = 0;
  for(size_t n = 0; n < ncells; n++){
    const cell_t *cell = *(cells + n);
    const pnode_t *pnode_root = cell->pnode;
    for(const pnode_t *pnode = pnode_root; pnode; pnode = pnode->next){
      nparticles += 1;
    }
  }
  FILE *fp = my_fopen("output/log/nparticles.dat", "a");
  if(fp == NULL){
    return ;
  }
  fprintf(fp, "% .3e %16zu\n", time, nparticles);
  fclose(fp);
}

static void check_momenta(const double time, const size_t nparticles, const part_t *particles){
  static double momenta_t0[NDIMS] = {0.};
  double momenta[NDIMS] = {0.};
  for(size_t n = 0; n < nparticles; n++){
    const part_t *p = particles + n;
    const double d = p->density;
    const double r = p->radius;
    const double m = compute_p_mass(d, r);
    const double *vel = p->velocity;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      momenta[dim] += m * vel[dim];
    }
  }
  for(dim_t dim = 0; dim < NDIMS; dim++){
    if(momenta_t0[dim] == 0.){
      momenta_t0[dim] = momenta[dim];
    }
  }
  FILE *fp = my_fopen("output/log/momenta.dat", "a");
  if(fp == NULL){
    return ;
  }
  fprintf(fp, "% .3e ", time);
  for(dim_t dim = 0; dim < NDIMS; dim++){
    fprintf(fp, "% .3e%c",
        momenta[dim] - momenta_t0[dim],
        dim == NDIMS - 1 ? '\n' : ' '
    );
  }
  fclose(fp);
}

static void check_energy(const double time, const size_t nparticles, const part_t *particles){
  static double energy_t0 = 0.;
  double energy = 0.;
  for(size_t n = 0; n < nparticles; n++){
    const part_t *p = particles + n;
    const double d = p->density;
    const double r = p->radius;
    const double m = compute_p_mass(d, r);
    const double *vel = p->velocity;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      energy += 0.5 * m * vel[dim] * vel[dim];
    }
  }
  if(energy_t0 == 0.){
    energy_t0 = energy;
  }
  FILE *fp = my_fopen("output/log/energy.dat", "a");
  if(fp == NULL){
    return ;
  }
  fprintf(fp, "% .3e % .3e % .3e\n", time, energy, energy - energy_t0);
  fclose(fp);
}

void logging(const double time, const size_t nparticles, const part_t *particles, const size_t ncells, cell_t **cells){
  check_number_of_events(time, ncells, cells);
  check_number_of_particles(time, ncells, cells);
  check_momenta(time, nparticles, particles);
  check_energy(time, nparticles, particles);
}

