#include <stdio.h>
#include "event.h"
#include "boundary.h"
#include "internal.h"


static void insert_event(elist_t *elist, event_t *event){
  // create new node from event_t
  enode_t *enode_new = common_calloc(1, sizeof(enode_t));
  enode_new->event = event;
  enode_new->next = NULL;
  // insert the new node to the linked list
  enode_t **enode_root = &(elist->enode);
  // check time when the root event occurs
  // NOTE: get_event_time returns large value when root is NULL
  const double time_old = get_event_time(*enode_root);
  const double time_new = get_event_time( enode_new );
  // compare new node and root node
  if(time_new < time_old){
    // new node becomes root, since
    //   1. root node is NULL (empty list)
    //   2. new event happens earlier than the root event
    enode_new->next = *enode_root;
    *enode_root = enode_new;
    min_heap_update(elist, time_old, time_new);
  }else{
    // list is not empty,
    //   and the new event occurs later than root event
    while(*enode_root){
      const double time_next = get_event_time((*enode_root)->next);
      if(time_new < time_next){
        enode_t *enode_next = (*enode_root)->next;
        (*enode_root)->next = enode_new;
        enode_new->next = enode_next;
        break;
      }
      enode_root = &((*enode_root)->next);
    }
  }
}

void create_events(const double tmax, part_t *p0, pnode_t *pnode_root, bnode_t *bnode_root, elist_t *elist){
  // inter-particle events
  for(pnode_t *pnode = pnode_root; pnode; pnode = pnode->next){
    part_t *p1 = pnode->particle;
    if(p0 == p1){
      // same particles,
      //   obviously no need to consider events between them
      continue;
    }
    if(p0->time != p1->time){
      printf("%s:%d\n", __FILE__, __LINE__);
      printf("Trying to consider events between particles ");
      printf("whose local times differ\n");
      exit(EXIT_FAILURE);
    }
    double time = limit_event_time;
    part_t *new_p0 = NULL;
    part_t *new_p1 = NULL;
    const int retval = check_next_pp_event(tmax, &time, p0, p1, &new_p0, &new_p1);
    if(retval != 0){
      continue;
    }
    event_t *new_event = common_calloc(1, sizeof(event_t));
    new_event->pp = common_calloc(1, sizeof(event_pp_t));
    new_event->pb = NULL;
    new_event->time = time;
    new_event->pp->p0 = p0;
    new_event->pp->p1 = p1;
    new_event->pp->new_p0 = new_p0;
    new_event->pp->new_p1 = new_p1;
    insert_event(elist, new_event);
  }
  // particle-boundary events
  for(bnode_t *bnode = bnode_root; bnode; bnode = bnode->next){
    const bund_t *b = bnode->boundary;
    double time = limit_event_time;
    part_t *new_p = NULL;
    const int retval = check_next_pb_event(tmax, &time, p0, b, &new_p);
    if(retval != 0){
      continue;
    }
    event_t *new_event = common_calloc(1, sizeof(event_t));
    new_event->pp = NULL;
    new_event->pb = common_calloc(1, sizeof(event_pb_t));
    new_event->time = time;
    new_event->pb->p = p0;
    new_event->pb->b = b;
    new_event->pb->new_p = new_p;
    insert_event(elist, new_event);
  }
}

