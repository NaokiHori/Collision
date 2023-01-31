#include <stdio.h>  // printf
#include <stdlib.h> // calloc, free
#include <stdint.h> // SIZE_MAX
#include "common.h"


void *common_calloc(const size_t count, const size_t size){
  if(count > SIZE_MAX / size){
    printf("memory allocation error: (count: %zu, size: %zu)\n", count, size);
    exit(EXIT_FAILURE);
  }
  void *ptr = calloc(count, size);
  if(ptr == NULL){
    printf("memory allocation error: (count: %zu, size: %zu)\n", count, size);
    exit(EXIT_FAILURE);
  }
  return ptr;
}

void common_free(void *ptr){
  free(ptr);
}

