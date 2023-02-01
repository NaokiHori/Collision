#if !defined(IMAGE_H)
#define IMAGE_H

#include "color.h"

typedef struct {
  size_t size[2];
  color_t *data;
} image_t;

extern image_t *image_init(const size_t *size_image);
extern int image_finalise(image_t *image);

extern int image_set_background(image_t *image);

extern int image_centering(image_t *image);
extern int image_output(const size_t index, const image_t *image);

#endif // IMAGE_H
