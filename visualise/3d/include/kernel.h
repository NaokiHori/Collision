#if !defined(KERNEL_H)
#define KERNEL_H

#include "vector.h"
#include "color.h"

typedef struct {
  real radius;
  vector_t center;
  color_t color;
} sphere_t;

extern int add_spheres(const size_t nspheres, const sphere_t *spheres, const viewer_t *viewer, image_t *image);

#endif // KERNEL_H
