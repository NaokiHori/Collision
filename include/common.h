#if !defined(COMMON_H)
#define COMMON_H

#include <stdlib.h>
#include <math.h>
#include <time.h>

#if !defined(M_PI)
#define M_PI 3.14159265358979324
#endif

#if !defined(NDIMS)
#error "define NDIMS"
#endif

typedef unsigned short dim_t;

#define NDIRS 2

typedef enum {
  DIR_NEG = 0,
  DIR_POS = 1
} dir_t;

/* memory allocator and deallocator */
extern void *common_calloc(const size_t count, const size_t size);
extern void common_free(void *ptr);

extern time_t common_get_current_time(void);

#endif // COMMON_H
