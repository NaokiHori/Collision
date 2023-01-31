#if !defined(DEBUG_H)
#define DEBUG_H

#include "particle.h"
#include "event.h"
#include "cell.h"

extern void debug_check_part_cnode(const part_t *particle);
extern void debug_check_elist_enode(const elist_t *elist);
extern void debug_check_cell_part(const cell_t *cell);
extern void debug_check_overlap(const size_t nparticles, const part_t *particles);

#endif // DEBUG_H
