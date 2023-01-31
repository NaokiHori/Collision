#include <stdio.h>  // printf
#include <stdlib.h> // calloc, free
#include <stdint.h> // SIZE_MAX
#include <time.h>   // time_t, time
#include "common.h"


void *common_calloc(const size_t count, const size_t size){
  if(count > SIZE_MAX / size){
    printf("memory allocation error: (count: %zu, size: %zu)\n", count, size);
    exit(EXIT_FAILURE);
  }
  if(count == 0 || size == 0){
    return NULL;
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

time_t common_get_current_time(void){
  return time(NULL);
}

