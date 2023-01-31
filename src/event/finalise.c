#include "event.h"
#include "internal.h"


void finalise_events(elist_t *elist){
  cancel_all_events(elist);
  common_free(elist);
}

