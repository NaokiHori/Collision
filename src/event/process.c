#include <stdio.h>
#include <float.h>
#include "common.h"
#include "config.h"
#include "event.h"
#include "particle.h"
#include "boundary.h"
#include "internal.h"
#include "debug.h"


static const bool do_debug_print = false;

static void update_particle_events(const double tmax, const double time, part_t *particle){
  for(cnode_t *cnode = particle->cnode; cnode; cnode = cnode->next){
    // for each cell storing this particle
    cell_t *cell = cnode->cell;
    // 1. update ALL particle positions contained by this cell,
    //   so that I can create new events based on new positions
    bool found = false;
    for(pnode_t *pnode = cell->pnode; pnode; pnode = pnode->next){
      integrate_particle_in_time(time, pnode->particle);
      if(particle == pnode->particle){
        found = true;
      }
    }
    if(!found){
      printf("%s:%d\n", __FILE__, __LINE__);
      printf("Although particle should belong to the cell, ");
      printf("particle list of this cell does not include it\n");
      exit(EXIT_FAILURE);
    }
    // 2. update events involving the given particle
    // first cancel out-dated events
    cancel_events(cell->elist, particle);
    // and renew them
    create_events(tmax, particle, cell->pnode, cell->bnode, cell->elist);
  }
}

static double process_pp_event(const double tmax, cell_t *cell){
  // inter-particle event (collision)
  // 1. compute new velocities after collision
  //   by ONE cell holding these two particles
  // 2. update events related to these particles
  //   for ALL cells which contain them
  event_t *event = cell->elist->enode->event;
  // consider two colliding particles
  // new positions and velocities have already been computed
  //   and stored by new_p[01]
  part_t *p0     = event->pp->p0;
  part_t *p1     = event->pp->p1;
  part_t *new_p0 = event->pp->new_p0;
  part_t *new_p1 = event->pp->new_p1;
  if(do_debug_print){
    printf("processing event (cell %p): p-p event: %p vs %p\n", cell, p0, p1);
  }
  const double time = event->time;
  // copy new positions and velocities
  //   which had already been computed
  //   when this event was planned
  for(dim_t dim = 0; dim < NDIMS; dim++){
    p0->position[dim] = new_p0->position[dim];
    p1->position[dim] = new_p1->position[dim];
    p0->velocity[dim] = new_p0->velocity[dim];
    p1->velocity[dim] = new_p1->velocity[dim];
  }
  // now new information is assigned to particles
  // thus local time is also updated to the event time
  p0->time = time;
  p1->time = time;
  // update other neighbour particles and events
  update_particle_events(tmax, time, p0);
  update_particle_events(tmax, time, p1);
  return time;
}

static void assign_particle(const double tmax, cell_t *cell, part_t *particle){
  // assign particle to the specified cell
  // first update all particle positions
  //   which exists originally in this cell
  const double time = particle->time;
  for(pnode_t *pnode = cell->pnode; pnode; pnode = pnode->next){
    integrate_particle_in_time(time, pnode->particle);
  }
  // since I delay the particle removal process,
  //   it is possible that this cell already contains the particle,
  //   which can be detected by checking the return value of "insert_pnode"
  // do nothing and return in such cases
  // 1. add particle to the list attached to the cell
  if(insert_pnode(&(cell->pnode), particle) != 0){
    // failed to insert, indicating particle is already registered to the cell
    return;
  }
  // 2. add cell to the list attach to the particle
  if(insert_cnode(&(particle->cnode), cell) != 0){
    // failed to insert, indicating cell is already registered to the particle
    printf("%s:%d\n", __FILE__, __LINE__);
    printf("Although particle did NOT know it belonged to the cell, ");
    printf("cell DID know the particle already\n");
    exit(EXIT_FAILURE);
  }
  // 3. schedule events related to this particle in this cell
  create_events(tmax, particle, cell->pnode, cell->bnode, cell->elist);
}

static double process_pb_event(const double tmax, cell_t *cell){
  // particle-boundary event
  event_t *event = cell->elist->enode->event;
  double time = event->time;
  part_t *particle = event->pb->p;
  const bund_t *boundary = event->pb->b;
  // update particles in this cell
  for(pnode_t *pnode = cell->pnode; pnode; pnode = pnode->next){
    integrate_particle_in_time(time, pnode->particle);
  }
  if(boundary->is_outer){
    // particle is getting out of this cell
    // remove its information completely
    if(do_debug_print){
      printf("processing event (cell %p): p-b event, getting out: %p\n", cell, particle);
    }
    cancel_events(cell->elist, particle);
    if(remove_pnode(&(cell->pnode), particle) != 0){
      // failed to remove this particle from cell
      printf("%s:%d\n", __FILE__, __LINE__);
      printf("Although the particle %p was expected to exist in the list of this cell %p, ", particle, cell);
      printf("I could not find it\n");
      exit(EXIT_FAILURE);
    }
    if(remove_cnode(&(particle->cnode), cell) != 0){
      // failed to remove this cell from particle
      printf("%s:%d\n", __FILE__, __LINE__);
      printf("Although the cell %p was expected to exist in the list of this particle %p, ", cell, particle);
      printf("I could not find it\n");
      exit(EXIT_FAILURE);
    }
    if(particle->cnode == NULL){
      // this particle belongs to no cell
      printf("%s:%d\n", __FILE__, __LINE__);
      printf("Particle %p belongs to no cell\n", particle);
      exit(EXIT_FAILURE);
    }
  }else{ // inner boundary events
    // 1. collision
    // 2. passing to one of the neighbouring cell
    if(boundary->is_edge){
      // particle-wall collision
      if(do_debug_print){
        printf("processing event (cell %p): p-b event, collision: %p\n", cell, particle);
      }
      for(dim_t dim = 0; dim < NDIMS; dim++){
        particle->position[dim] = event->pb->new_p->position[dim];
        particle->velocity[dim] = event->pb->new_p->velocity[dim];
      }
      update_particle_events(tmax, time, particle);
    }else{
      // assign particle to the neighbouring cell
      if(do_debug_print){
        printf("processing event (cell %p): p-b event: assign %p to %p\n", cell, particle, boundary->neighbour);
      }
      // now this particle is moving accross the cell boundary
      // assign particle to the neighbouring cell
      assign_particle(tmax, boundary->neighbour, particle);
      // since the particle motion is not changed,
      //   updating events in the original cell is a waste of time
      // so I just remove root event here
      //   instead of rescheduling all events involving the given particle
      cancel_root_event(cell->elist);
    }
  }
  return time;
}

double process_event(const double tmax, cell_t **cells){
  // extract first cell,
  //   in which the upcoming event is contained
  cell_t *cell = cells[0];
  enode_t *enode = cell->elist->enode;
  if(enode == NULL){
    // no more event before tmax
    return limit_event_time;
  }
  if(enode->event->pp != NULL){
    return process_pp_event(tmax, cell);
  }else{
    return process_pb_event(tmax, cell);
  }
}

