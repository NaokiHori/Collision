#include "particle.h"
#include "event.h"
#include "debug.h"
#include "internal.h"


void init_events(const double tmax, const size_t ncells_total, cell_t **cells){
  for(size_t n = 0; n < ncells_total; n++){
    cell_t *cell = cells[n];
    elist_t *elist = cell->elist;
    bnode_t *bnode_root = cell->bnode;
    pnode_t *pnode_root = cell->pnode;
    for(pnode_t *pnode = pnode_root; pnode; pnode = pnode->next){
      part_t *particle = pnode->particle;
      create_events(tmax, particle, pnode->next, bnode_root, elist);
    }
  }
  min_heap_init(ncells_total, cells);
}

