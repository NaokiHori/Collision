#include <stdio.h>
#include <stdlib.h>
#include "common.h"


void *common_calloc(size_t count, size_t size){
  void *ptr = calloc(count, size);
  if(ptr == NULL){
    perror(__func__);
    exit(EXIT_FAILURE);
  }
  return ptr;
}

void common_free(void *ptr){
  free(ptr);
}

