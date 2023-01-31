#if !defined(BOUNDARY_H)
#define BOUNDARY_H

#include <stdlib.h>
#include <stdbool.h>
#include "common.h"
#include "struct.h"

struct bund_t_ {
  // dimension of this boundary (x-wall, y-wall, etc.)
  dim_t dim;
  // cell has negative and positive boundaries,
  //   which is distinguished by this member
  dir_t dir;
  // each cell boundary has two events
  //   1. assign particle to the neighbouring cell,
  //   2. stop tracking the particle since it is moving out,
  // which is distinguished by this member
  dir_t sft;
  // physical position where this boundary is located
  double position;
  // whether this boundary sits at the edge of the whole domain
  bool is_edge;
  // whether this boundary is inner or outer (closely linked to "sft" member)
  bool is_outer;
  // pointer to the cell sharing this boundary
  cell_t *neighbour;
};

struct bnode_t_ {
  // pointer to actual boundary data
  bund_t *boundary;
  // pointer to next node
  struct bnode_t_ *next;
};

extern bnode_t *init_boundaries(const bool *are_edge, const double *limits, cell_t **neighbours);
extern void finalise_boundaries(bnode_t *bnode_root);

#endif // BOUNDARY_H
