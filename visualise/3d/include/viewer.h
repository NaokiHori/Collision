#if !defined(VIEWER_H)
#define VIEWER_H

#include "vector.h"

typedef struct {
  // origin
  vector_t origin;
  // horizontal and vertical vectors
  vector_t h, v;
  // left-bottom corner
  vector_t c;
  // all pixels on screen
  vector_t *pixels;
  // horizontal and vertical pixel sizes
  vector_t dh, dv;
} screen_t;

typedef struct {
  // camera position
  vector_t camera;
  // screen
  screen_t *screen;
} viewer_t;

extern viewer_t *viewer_init(const size_t *size_image, const real *size_screen, const real angle_a, const real angle_p);
extern int viewer_finalise(viewer_t *viewer);

#endif // VIEWER_H
