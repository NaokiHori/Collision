#include "common.h"
#include "cell.h"
#include "particle.h"


void finalise_particles(const size_t nparticles, part_t *particles){
  for(size_t n = 0; n < nparticles; n++){
    part_t *particle = particles + n;
    common_free(particle->position);
    common_free(particle->velocity);
    remove_all_cnodes(particle->cnode);
  }
  common_free(particles);
}

