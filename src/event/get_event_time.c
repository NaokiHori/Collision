#include <float.h>
#include "event.h"


// large number which is larger than all event time
const double limit_event_time = DBL_MAX;

double get_event_time(const enode_t *enode){
  if(enode){
    return enode->event->time;
  }else{
    return limit_event_time;
  }
}

