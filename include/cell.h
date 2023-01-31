#if !defined(CELL_H)
#define CELL_H

#include <stdbool.h>
#include <stddef.h>
#include "struct.h"


struct cnode_t_ {
  cell_t *cell;
  struct cnode_t_ *next;
};

extern int insert_cnode(cnode_t **cnode_root, cell_t *cell);
extern int remove_cnode(cnode_t **cnode_root, const cell_t *cell);
extern void remove_all_cnodes(cnode_t *cnode_root);

struct cell_t_ {
  pnode_t *pnode;
  bnode_t *bnode;
  elist_t *elist;
};

extern void init_cells(size_t *ncells_total, cell_t ***cells, const size_t nparticles, part_t *particles);
extern void finalise_cells(const size_t ncells_total, cell_t **cells);

#endif // CELL_H
