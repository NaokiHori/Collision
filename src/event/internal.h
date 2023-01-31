#if !defined(INTERNAL_EVENT_H)
#define INTERNAL_EVENT_H

#include <stddef.h> // size_t
#include "struct.h"
#include "event.h"

extern void create_events(const double tmax, part_t *p0, pnode_t *pnode_root, bnode_t *bnode_root, elist_t *elist);
extern void cancel_events(elist_t *elist, const part_t *particle);
extern void cancel_root_event(elist_t *elist);
extern void cancel_all_events(elist_t *elist);

extern void min_heap_init(const size_t nitems, cell_t **cells);
extern void min_heap_update(elist_t *elist, const double time_old, const double time_new);

#endif // INTERNAL_EVENT_H
