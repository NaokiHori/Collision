#include <stdio.h>
#include <stdbool.h>
#include "common.h"
#include "config.h"
#include "cell.h"
#include "particle.h"
#include "boundary.h"
#include "event.h"
#include "io.h"


static size_t *index_to_ranks(size_t index, const size_t *sizes){
  size_t *ranks = common_calloc(NDIMS, sizeof(size_t));
  for(dim_t dim = 0; dim < NDIMS; dim++){
    ranks[dim] = index % sizes[dim];
    index =      index / sizes[dim];
  }
  return ranks;
}

static size_t *nitems_to_strides(const size_t *nitems){
  size_t *strides = common_calloc(NDIMS, sizeof(size_t));
  for(dim_t dim0 = 0; dim0 < NDIMS; dim0++){
    strides[dim0] = 1;
    for(dim_t dim1 = 0; dim1 < dim0; dim1++){
      strides[dim0] *= nitems[dim1];
    }
  }
  return strides;
}

static void init_cell_configs(double **cell_faces, const size_t *ncells, size_t *ncells_total, cell_t ***cells){
  size_t *strides = nitems_to_strides(ncells);
  *ncells_total = 1;
  for(dim_t dim = 0; dim < NDIMS; dim++){
    *ncells_total *= ncells[dim];
  }
  *cells = common_calloc(*ncells_total, sizeof(cell_t *));
  for(size_t n = 0; n < *ncells_total; n++){
    (*cells)[n] = common_calloc(1, sizeof(cell_t));
  }
  for(size_t n = 0; n < *ncells_total; n++){
    // convert n (index) to ranks x0, x1, ...: physical location
    size_t *ranks = index_to_ranks(n, ncells);
    // whether this cell sits at the edge of domain
    bool *are_edge = common_calloc(NDIRS * NDIMS, sizeof(bool));
    for(dim_t dim = 0; dim < NDIMS; dim++){
      are_edge[dim * NDIRS + DIR_NEG] = ranks[dim] ==               0;
      are_edge[dim * NDIRS + DIR_POS] = ranks[dim] == ncells[dim] - 1;
    }
    // range of domain this cell is responsible for
    double *limits = common_calloc(NDIRS * NDIMS, sizeof(double));
    for(dim_t dim = 0; dim < NDIMS; dim++){
      limits[dim * NDIRS + DIR_NEG] = cell_faces[dim][ranks[dim] + 0];
      limits[dim * NDIRS + DIR_POS] = cell_faces[dim][ranks[dim] + 1];
    }
    // my neighbours
    // NOTE: this is not correct for edge cells
    // this information, however, is not used in such case
    //   and thus I do not take it into accout
    cell_t **neighbours = common_calloc(NDIRS * NDIMS, sizeof(cell_t *));
    for(dim_t dim0 = 0; dim0 < NDIMS; dim0++){
      for(dir_t dir = 0; dir < NDIRS; dir++){
        size_t rank_neighbour = 0;
        for(dim_t dim1 = 0; dim1 < NDIMS; dim1++){
          // check neighbour rank
          size_t nrank = 0;
          if(dim0 != dim1){
            // when the dimension is not identical, use the same rank as mine
            nrank = ranks[dim1];
          }else{ // dim0 == dim1
            // consider displacement
            const size_t ncells0 = ncells[dim0];
            const size_t myrank0 =  ranks[dim0];
            if(dir == DIR_NEG){
              // basically -1 in dim0
              // exception: when myrank in dim0 is 0
              nrank =
                myrank0 != 0 ? myrank0 - 1
                             : myrank0;
            }else{ // dir == DIR_POS
              // basically +1 in dim0
              // exception: when myrank in dim0 is ncells0 - 1
              nrank =
                myrank0 != ncells0 - 1 ? myrank0 + 1
                                       : myrank0;
            }
          }
          // convert ND ranks to 1D (np.ravel)
          rank_neighbour += nrank * strides[dim1];
        }
        // store pointer to the neighbour cell instead of index
        neighbours[dim0 * NDIRS + dir] = (*cells)[rank_neighbour];
      }
    }
    cell_t *cell = (*cells)[n];
    cell->bnode = init_boundaries(are_edge, limits, neighbours);
    cell->elist = common_calloc(1, sizeof(elist_t));
    cell->elist->cell_head = (*cells) + 0;
    cell->elist->cell      = (*cells) + n;
    common_free(ranks);
    common_free(are_edge);
    common_free(neighbours);
    common_free(limits);
  }
  common_free(strides);
}

static void check_affiliation(const double radius, const double center, const size_t ncells, const double *faces, size_t *nitems, size_t **ranks){
  typedef struct node_t_ {
    size_t rank;
    struct node_t_ *next;
  } node_t;
  node_t *node_root = NULL;
  const double partmin = center - radius;
  const double partmax = center + radius;
  *nitems = 0;
  for(size_t rank = 0; rank < ncells; rank++){
    const double cellmin = faces[rank    ];
    const double cellmax = faces[rank + 1];
    if( (cellmin <= partmax) && (partmin <= cellmax) ){
      *nitems += 1;
      node_t *new_node = common_calloc(1, sizeof(node_t));
      new_node->rank = rank;
      new_node->next = node_root;
      node_root = new_node;
    }
  }
  *ranks = common_calloc(*nitems, sizeof(size_t));
  for(size_t n = 0; n < *nitems; n++){
    (*ranks)[n] = node_root->rank;
    node_t *next = node_root->next;
    common_free(node_root);
    node_root = next;
  }
}

static void assign_particles_to_cells(const size_t nparticles, part_t *particles, double **cell_faces, const size_t *ncells, cell_t **cells){
  size_t *strides = nitems_to_strides(ncells);
  for(size_t np = 0; np < nparticles; np++){
    part_t *particle = particles + np;
    // for each particle, check affiliation in each dimension
    size_t nitems_[NDIMS] = {0};
    size_t *ranks_[NDIMS] = {NULL};
    for(dim_t dim = 0; dim < NDIMS; dim++){
      check_affiliation(
          particle->radius,
          particle->position[dim],
          ncells[dim],
          cell_faces[dim],
          &(nitems_[dim]),
          &(ranks_[dim])
      );
    }
    // convert ND result to 1D
    size_t nitems = 1;
    for(dim_t dim = 0; dim < NDIMS; dim++){
      nitems *= nitems_[dim];
    }
    for(size_t n = 0; n < nitems; n++){
      // ranks in terms of items in each dimension
      size_t *ranks = index_to_ranks(n, nitems_);
      // compute index to obtain target cell
      size_t index = 0;
      for(dim_t dim = 0; dim < NDIMS; dim++){
        index += ranks_[dim][ranks[dim]] * strides[dim];
      }
      cell_t *cell = cells[index];
      insert_pnode(&(cell->pnode), particle);
      insert_cnode(&(particle->cnode), cell);
      common_free(ranks);
    }
    for(dim_t dim = 0; dim < NDIMS; dim++){
      common_free(ranks_[dim]);
    }
  }
  common_free(strides);
}

void init_cells(size_t *ncells_total, cell_t ***cells, const size_t nparticles, part_t *particles){
  // domain size in each dimension,
  //   which are loaded from files
  //   and used to compute cell faces
  double lengths[NDIMS] = {0.};
  load_vector(
      config.get_string("input_directory"),
      "lengths",
      NDIMS,
      sizeof(double),
      dtype_double,
      lengths
  );
  // (approximate) size of cell,
  //   assuming typical particle size is unity
  const double cell_size = 2.;
  // number of cells in each dimension,
  //   which are loaded from environmental variables
  size_t ncells[NDIMS] = {0};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    ncells[dim] = lengths[dim] / cell_size;
  }
  double *cell_faces[NDIMS] = {NULL};
  for(dim_t dim = 0; dim < NDIMS; dim++){
    cell_faces[dim] = common_calloc(ncells[dim] + 1, sizeof(double));
    for(size_t n = 0; n < ncells[dim] + 1; n++){
      cell_faces[dim][n] = 1. * n * lengths[dim] / ncells[dim];
    }
  }
  // initialise cells
  init_cell_configs(cell_faces, ncells, ncells_total, cells);
  assign_particles_to_cells(nparticles, particles, cell_faces, ncells, *cells);
  // clean-up local heaps
  for(dim_t dim = 0; dim < NDIMS; dim++){
    common_free(cell_faces[dim]);
  }
  printf("ncells_total: %zu\n", *ncells_total);
}

