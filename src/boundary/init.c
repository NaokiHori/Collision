#include <stdbool.h>
#include "common.h"
#include "boundary.h"
#include "internal.h"


bnode_t *init_boundaries(const bool *are_edge, const double *limits, cell_t **neighbours){
  bnode_t *bnode_root = NULL;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    for(dir_t dir = 0; dir < NDIRS; dir++){
      const bool is_edge = are_edge[dim * NDIRS + dir];
      const double position = limits[dim * NDIRS + dir];
      cell_t *neighbour = neighbours[dim * NDIRS + dir];
      for(dir_t sft = 0; sft < NDIRS; sft++){
        const bool is_outer = (dir ^ sft) == 0;
        if(is_edge && is_outer){
          // inside wall, I do not have to consider,
          //   since particles should be reflected beforehand
          continue;
        }else{
          bund_t *boundary = common_calloc(1, sizeof(bund_t));
          boundary->dim = dim;
          boundary->dir = dir;
          boundary->sft = sft;
          boundary->position = position;
          boundary->is_edge = is_edge;
          boundary->is_outer = is_outer;
          boundary->neighbour = neighbour;
          insert_bnode(&bnode_root, boundary);
        }
      }
    }
  }
  return bnode_root;
}

