#if !defined(PARTICLE_H)
#define PARTICLE_H

#include <stdlib.h>
#include <stdbool.h>
#include "common.h"
#include "struct.h"


// datatype to store particle info,
//   which are SHARED among all cells
struct part_t_ {
  // local time attached to this particle
  double time;
  // singly-linked list of cells
  //   which hold this particle
  cnode_t *cnode;
  // others
  double density;
  double radius;
  double *position;
  double *velocity;
};

extern void init_particles(const double time, size_t *nparticles, part_t **particles);
extern void finalise_particles(const size_t nparticles, part_t *particles);

extern void integrate_particle_in_time(const double t_new, part_t *particle);

extern int check_next_pp_event(const double tmax, double *time, const part_t *p0, const part_t *p1, part_t **new_p0, part_t **new_p1);
extern int check_next_pb_event(const double tmax, double *time, const part_t *p, const bund_t *b, part_t **new_p);

extern double compute_p_volume(const double r);
extern double compute_p_mass(const double d, const double r);

// singly-linked list
//   to store particles which belong to the cell,
// which is NOT SHARED (each cell has independent info)
struct pnode_t_ {
  // pointer to actual particle data
  part_t *particle;
  // pointer to next node
  struct pnode_t_ *next;
};

// pnode list operations
extern int insert_pnode(pnode_t **pnode_root, part_t *particle);
extern int remove_pnode(pnode_t **pnode_root, const part_t *particle);
extern void remove_all_pnodes(pnode_t *pnode_root);

#endif // PARTICLE_H
