#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <assert.h>
#include <errno.h>
#include "common.h"
#include "viewer.h"
#include "image.h"
#include "kernel.h"
#include "snpyio.h"


static size_t load_nspheres(const char fname[]){
  errno = 0;
  FILE *fp = fopen(fname, "r");
  if(fp == NULL){
    perror(fname);
    exit(1);
  }
  size_t ndim = 0;
  size_t *shape = NULL;
  char *dtype = NULL;
  bool is_fortran_order = false;
  snpyio_r_header(&ndim, &shape, &dtype, &is_fortran_order, fp);
  size_t nspheres = 0;
  fread(&nspheres, sizeof(size_t), 1, fp);
  free(shape);
  free(dtype);
  return nspheres;
}

static double *load_pos(const size_t nspheres, const char dname[], const char fname[]){
  const size_t nchars = strlen(dname) + strlen(fname) + 1;
  char *name = common_calloc(nchars, sizeof(char));
  snprintf(name, nchars, "%s%s", dname, fname);
  errno = 0;
  FILE *fp = fopen(name, "r");
  if(fp == NULL){
    perror(name);
    exit(1);
  }
  size_t ndim = 0;
  size_t *shape = NULL;
  char *dtype = NULL;
  bool is_fortran_order = false;
  snpyio_r_header(&ndim, &shape, &dtype, &is_fortran_order, fp);
  double *pos = common_calloc(nspheres, sizeof(double));
  fread(pos, sizeof(double), nspheres, fp);
  free(shape);
  free(dtype);
  common_free(name);
  return pos;
}

static void hsv_to_rgb(const real h, const real s, const real v, real *r, real *g, real *b){
  if(s == 0.){
    *r = v;
    *g = v;
    *b = v;
    return;
  }
  uint8_t i = (uint8_t)(6. * h);
  real f = 6. * h - i;
  real p = v * (1. - s);
  real q = v * (1. - s * f);
  real t = v * (1. - s * (1. - f));
  i = i % 6;
  if(i == 0){
    *r = v;
    *g = t;
    *b = p;
  }else if(i == 1){
    *r = q;
    *g = v;
    *b = p;
  }else if(i == 2){
    *r = p;
    *g = v;
    *b = t;
  }else if(i == 3){
    *r = p;
    *g = q;
    *b = v;
  }else if(i == 4){
    *r = t;
    *g = p;
    *b = v;
  }else{
    *r = v;
    *g = p;
    *b = q;
  }
}

int main(int argc, char *argv[]){
  assert(argc == 4);
  for(int n = 0; n < argc; n++){
    printf("%s%c", argv[n], n == argc - 1 ? '\n' : ' ');
  }
  const size_t image_index = strtol(argv[3], NULL, 10);
  // number of pixels of the resulting image
  const size_t size_image[] = {3840 / 4, 2160 / 4};
  // size of screen
  const real size_screen[] = {160., 90.};
  const size_t nspheres = load_nspheres("../input/nparticles.npy");
  sphere_t *spheres = common_calloc(nspheres, sizeof(sphere_t));
  double *xs = NULL;
  double *ys = NULL;
  double *zs = NULL;
  zs = load_pos(nspheres, "../input/", "positions_2.npy");
  for(size_t n = 0; n < nspheres; n++){
    const real z = (zs[n] + 540.) / 1080.;
    const real h = z;
    const real s = 1.;
    const real v = 1.;
    real r, g, b;
    hsv_to_rgb(h, s, v, &r, &g, &b);
    spheres[n].color.r = (uint8_t)(255 * r);
    spheres[n].color.g = (uint8_t)(255 * g);
    spheres[n].color.b = (uint8_t)(255 * b);
  }
  common_free(zs);
  xs = load_pos(nspheres, argv[2], "positions_0.npy");
  ys = load_pos(nspheres, argv[2], "positions_1.npy");
  zs = load_pos(nspheres, argv[2], "positions_2.npy");
  for(size_t n = 0; n < nspheres; n++){
    spheres[n].radius   = 1.;
    spheres[n].center.x = xs[n];
    spheres[n].center.y = ys[n];
    spheres[n].center.z = zs[n];
  }
  common_free(xs);
  common_free(ys);
  common_free(zs);
  // viewer
  const real angle_a = 0.5 * M_PI / 16 * image_index;
  const real angle_p = M_PI / 16.;
  // initialise viewer (camera, screen position, etc.)
  viewer_t *viewer = viewer_init(size_image, size_screen, angle_a, angle_p);
  // initialise output image
  image_t *image = image_init(size_image);
  image_set_background(image);
  // process
  add_spheres(nspheres, spheres, viewer, image);
  // output image
  image_output(image_index, image);
  // clean-up
  viewer_finalise(viewer);
  image_finalise(image);
  common_free(spheres);
  return 0;
}

