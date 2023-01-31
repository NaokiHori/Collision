#include <stdio.h>
#include <stdbool.h>
#include "event.h"
#include "cell.h"
#include "internal.h"


static const bool do_debug = false;

static size_t g_nitems = 0;

static size_t parent(const size_t n){
  return (n - 1) / 2;
}

static size_t lchild(const size_t n){
  return 2 * n + 1;
}

static size_t rchild(const size_t n){
  return 2 * n + 2;
}

static void swap(const size_t n0, const size_t n1, cell_t **cell_head){
  cell_t *cell  = cell_head[n0];
  cell_head[n0] = cell_head[n1];
  cell_head[n1] = cell;
  cell_head[n0]->elist->cell = cell_head + n0;
  cell_head[n1]->elist->cell = cell_head + n1;
}

static double get_data(const size_t nitems, const size_t n, cell_t **cell_head){
  if(n >= nitems){
    return limit_event_time;
  }
  return get_event_time(cell_head[n]->elist->enode);
}

static void upshift(const size_t nitems, size_t n, cell_t **cell_head){
  while(0 < n && n < nitems){
    const size_t n_p = parent(n);
    const double data   = get_data(nitems, n  , cell_head);
    const double data_p = get_data(nitems, n_p, cell_head);
    if(data_p > data){
      swap(n, n_p, cell_head);
      n = n_p;
    }else{
      break;
    }
  }
}

static void downshift(const size_t nitems, size_t n, cell_t **cell_head){
  while(n < nitems){
    const size_t n_l = lchild(n);
    const size_t n_r = rchild(n);
    const double data   = get_data(nitems, n  , cell_head);
    const double data_l = get_data(nitems, n_l, cell_head);
    const double data_r = get_data(nitems, n_r, cell_head);
    if(data_l < data && data_l <= data_r){
      swap(n, n_l, cell_head);
      n = n_l;
    }else if(data_r < data && data_r < data_l){
      swap(n, n_r, cell_head);
      n = n_r;
    }else{
      break;
    }
  }
}

static void check(cell_t **cell_head){
  bool is_broken = false;
  const size_t nitems = g_nitems;
  for(size_t n = 1; n < nitems; n++){
    const size_t n_p = parent(n);
    const double data   = get_data(nitems, n  , cell_head);
    const double data_p = get_data(nitems, n_p, cell_head);
    if(data_p > data){
      is_broken = true;
      printf("not heapified: %p (% .3e) > %p (% .3e)\n", cell_head[n_p], data_p, cell_head[n], data);
    }
  }
  for(size_t n = 0; n < nitems; n++){
    const size_t n_l = lchild(n);
    const size_t n_r = rchild(n);
    const double data   = get_data(nitems, n  , cell_head);
    const double data_l = get_data(nitems, n_l, cell_head);
    const double data_r = get_data(nitems, n_r, cell_head);
    if(data_l < data && data_l <= data_r){
      is_broken = true;
      printf("not heapified: %p (% .3e) < %p (% .3e)\n", cell_head[n_l], data_l, cell_head[n], data);
    }else if(data_r < data && data_r < data_l){
      is_broken = true;
      printf("not heapified: %p (% .3e) < %p (% .3e)\n", cell_head[n_r], data_r, cell_head[n], data);
    }
  }
  if(is_broken){
    exit(EXIT_FAILURE);
  }
}

void min_heap_init(const size_t nitems, cell_t **cell_head){
  g_nitems = nitems;
  for(size_t n = 0; n < g_nitems; n++){
    upshift(nitems, n, cell_head);
  }
}

void min_heap_update(elist_t *elist, const double data_old, const double data_new){
  cell_t **cell_head = elist->cell_head;
  cell_t **cell      = elist->cell;
  const size_t n = cell - cell_head;
  if(data_old > data_new){
    upshift(g_nitems, n, cell_head);
  }else{
    downshift(g_nitems, n, cell_head);
  }
  if(do_debug){
    check(cell_head);
  }
}

