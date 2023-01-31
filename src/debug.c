#include <stdio.h>
#include <stdbool.h>
#include <float.h>
#include "particle.h"
#include "event.h"
#include "cell.h"
#include "debug.h"


void debug_check_part_cnode(const part_t *particle){
  // check to which cell the given particle belongs
  const char fname[] = {"output/log/part_cnode.log"};
  static bool never_called = true;
  FILE *fp = fopen(fname, never_called ? "w" : "a");
  never_called = false;
  fprintf(fp, "part %p:\n", particle);
  const cnode_t *cnode_root = particle->cnode;
  for(const cnode_t *cnode = cnode_root; cnode; cnode = cnode->next){
    fprintf(fp, "  cell %p\n", cnode->cell);
  }
  fclose(fp);
}

void debug_check_elist_enode(const elist_t *elist){
  // check all events in the given elist
  const char fname[] = {"output/log/elist_enode.log"};
  static bool never_called = true;
  FILE *fp = fopen(fname, never_called ? "w" : "a");
  never_called = false;
  fprintf(fp, "elist %p:\n", elist);
  for(const enode_t *enode = elist->enode; enode; enode = enode->next){
    const event_t *event = enode->event;
    if(event->pp != NULL){
      const double time = event->time;
      fprintf(fp, "  p p event %p @ % .3e\n", event, time);
    }else{
      const double time = event->time;
      fprintf(fp, "  p b event %p @ % .3e\n", event, time);
    }
  }
  fclose(fp);
}

void debug_check_cell_part(const cell_t *cell){
  // check all particles in the given cell
  const char fname[] = {"output/log/cell_part.log"};
  static bool never_called = true;
  FILE *fp = fopen(fname, never_called ? "w" : "a");
  never_called = false;
  fprintf(fp, "cell %p:\n", cell);
  const pnode_t *pnode_root = cell->pnode;
  for(const pnode_t *pnode = pnode_root; pnode; pnode = pnode->next){
    const part_t *particle = pnode->particle;
    fprintf(fp, "  particle %p\n", particle);
  }
}

void debug_check_overlap(const size_t nparticles, const part_t *particles){
  // check overlap
  const char fname[] = {"output/log/overlap.log"};
  static bool never_called = true;
  FILE *fp = fopen(fname, never_called ? "w" : "a");
  never_called = false;
  double minval = DBL_MAX;
  for(size_t n0 = 0; n0 < nparticles; n0++){
    const part_t *p0 = particles + n0;
    const double r0 = p0->radius;
    const double *pos0 = p0->position;
    for(size_t n1 = n0 + 1; n1 < nparticles; n1++){
      const part_t *p1 = particles + n1;
      const double r1 = p1->radius;
      const double *pos1 = p1->position;
      double val = - pow(r0 + r1, 2.);
      for(dim_t dim = 0; dim < NDIMS; dim++){
        val += pow(pos1[dim] - pos0[dim], 2.);
      }
      minval = fmin(minval, val);
    }
  }
  fprintf(fp, "% .3e\n", minval);
  fclose(fp);
}

