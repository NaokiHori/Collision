#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "common.h"
#include "image.h"


image_t *image_init(const size_t *size_image){
  // image size, should have the same aspect ratio as the screen.size
  image_t *image = common_calloc(1, sizeof(image_t));
  image->size[0] = size_image[0];
  image->size[1] = size_image[1];
  image->data = common_calloc(size_image[0] * size_image[1], sizeof(color_t));
  return image;
}

int image_finalise(image_t *image){
  common_free(image->data);
  common_free(image);
  return 0;
}

int image_set_background(image_t *image){
  const size_t size[2] = {
    image->size[0],
    image->size[1]
  };
  color_t *data = image->data;
  for(size_t j = 0; j < size[1]; j++){
    for(size_t i = 0; i < size[0]; i++){
      const size_t index = j * size[0] + i;
      data[index].r = 0;
      data[index].g = 0;
      data[index].b = 0;
    }
  }
  return 0;
}

int image_output(const size_t index, const image_t *image){
  const bool is_binary = true;
  const size_t size[2] = {
    image->size[0],
    image->size[1]
  };
  const color_t *data = image->data;
  char fname[128];
  snprintf(fname, 128, "images/image%010zu.ppm", index);
  FILE *fp = fopen(fname, "w");
  if(fp == NULL){
    perror(fname);
    return 1;
  }
  if(is_binary){
    fprintf(fp, "P6\n");
  }else{
    fprintf(fp, "P3\n");
  }
  fprintf(fp, "%zu %zu\n", size[0], size[1]);
  fprintf(fp, "255\n");
  for(size_t jinv = 0; jinv < size[1]; jinv++){
    for(size_t i = 0; i < size[0]; i++){
      const size_t j = size[1] - jinv - 1;
      const size_t index = j * size[0] + i;
      const color_t pixel = data[index];
      if(is_binary){
        uint8_t buf[3] = {pixel.r, pixel.g, pixel.b};
        fwrite(buf, sizeof(uint8_t), 3, fp);
      }else{
        fprintf(fp, "%3u %3u %3u\n", pixel.r, pixel.g, pixel.b);
      }
    }
  }
  fclose(fp);
  return 0;
}

