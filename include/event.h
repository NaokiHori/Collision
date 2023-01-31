#if !defined(EVENT_H)
#define EVENT_H

#include <stddef.h>
#include <stdbool.h>
#include "common.h"
#include "struct.h"
#include "particle.h"


typedef struct {
  part_t *p0;
  part_t *p1;
  part_t *new_p0;
  part_t *new_p1;
} event_pp_t;

typedef struct {
  part_t *p;
  const bund_t *b;
  part_t *new_p;
} event_pb_t;

typedef struct {
  double time;
  event_pp_t *pp;
  event_pb_t *pb;
} event_t;

// singly-linked list
//   to store events which are planned in each cell
struct enode_t_ {
  event_t *event;
  struct enode_t_ *next;
};

struct elist_t_ {
  cell_t **cell_head;
  cell_t **cell;
  enode_t *enode;
};

extern const double limit_event_time;
extern double get_event_time(const enode_t *enode);

extern void init_events(const double tmax, const size_t ncells_total, cell_t **cells);
extern double process_event(const double tmax, cell_t **cells);
extern void finalise_events(elist_t *elist);

#endif // EVENT_H
