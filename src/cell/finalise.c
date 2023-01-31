#include "common.h"
#include "cell.h"
#include "particle.h"
#include "boundary.h"
#include "event.h"


void finalise_cells(const size_t ncells_total, cell_t **cells){
  for(size_t n = 0; n < ncells_total; n++){
    cell_t *cell = cells[n];
    remove_all_pnodes(cell->pnode);
    finalise_boundaries(cell->bnode);
    finalise_events(cell->elist);
    common_free(cell);
  }
  common_free(cells);
}

