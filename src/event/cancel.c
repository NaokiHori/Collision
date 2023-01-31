#include <stdbool.h>
#include <float.h>
#include "common.h"
#include "event.h"
#include "internal.h"


static void finalise_enode(enode_t *enode){
  if(enode->event->pp != NULL){
    part_t *p = NULL;
    p = enode->event->pp->new_p0;
    common_free(p->position);
    common_free(p->velocity);
    common_free(p);
    p = enode->event->pp->new_p1;
    common_free(p->position);
    common_free(p->velocity);
    common_free(p);
  }else{
    part_t *p = enode->event->pb->new_p;
    common_free(p->position);
    common_free(p->velocity);
    common_free(p);
  }
  common_free(enode->event->pp);
  common_free(enode->event->pb);
  common_free(enode->event);
  common_free(enode);
}

void cancel_events(elist_t *elist, const part_t *particle){
  // remove all events involving the given particle
  enode_t **enode_root = &(elist->enode);
  // keep time of root event which is needed to update heap,
  //   since root may be changed for several times
  const double time_old = get_event_time(*enode_root);
  while(*enode_root){
    event_t *event = (*enode_root)->event;
    // check this event node is to be removed
    bool remove = false;
    if(event->pp != NULL){
      // inter-particle event
      const event_pp_t *pp = event->pp;
      if(particle == pp->p0){
        remove = true;
      }else if(particle == pp->p1){
        remove = true;
      }
    }else{
      // particle-boundary event
      const event_pb_t *pb = event->pb;
      if(particle == pb->p){
        remove = true;
      }
    }
    if(remove){
      enode_t *enode_next = (*enode_root)->next;
      finalise_enode(*enode_root);
      *enode_root = enode_next;
    }else{
      enode_root = &((*enode_root)->next);
    }
  }
  // update heap when the root node is modified,
  //   which can be detected by checking the difference of times
  const double time_new = get_event_time(elist->enode);
  if(time_old != time_new){
    min_heap_update(elist, time_old, time_new);
  }
}

void cancel_root_event(elist_t *elist){
  // remove the first event
  // apparently the root node is updated,
  //   which is always followed by updating the heap
  enode_t **enode_root = &(elist->enode);
  enode_t *enode_next = (*enode_root)->next;
  const double time_old = get_event_time(*enode_root);
  const double time_new = get_event_time( enode_next);
  finalise_enode(*enode_root);
  *enode_root = enode_next;
  min_heap_update(elist, time_old, time_new);
}

void cancel_all_events(elist_t *elist){
  enode_t *enode_root = elist->enode;
  while(enode_root){
    enode_t *enode_next = enode_root->next;
    finalise_enode(enode_root);
    enode_root = enode_next;
  }
}

