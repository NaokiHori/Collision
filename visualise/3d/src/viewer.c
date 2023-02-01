#include <stdio.h>
#include <math.h>
#include "common.h"
#include "vector.h"
#include "viewer.h"


static vector_t rodrigues(const real angle, const vector_t normal, const vector_t old){
  const real nx = normal.x;
  const real ny = normal.y;
  const real nz = normal.z;
  const real sin_t = sin(angle);
  const real cos_t = cos(angle);
  const real a00 = nx * nx * (1. - cos_t) +      cos_t;
  const real a01 = nx * ny * (1. - cos_t) - nz * sin_t;
  const real a02 = nx * nz * (1. - cos_t) + ny * sin_t;
  const real a10 = ny * nx * (1. - cos_t) + nz * sin_t;
  const real a11 = ny * ny * (1. - cos_t) +      cos_t;
  const real a12 = ny * nz * (1. - cos_t) - nx * sin_t;
  const real a20 = nz * nx * (1. - cos_t) - ny * sin_t;
  const real a21 = nz * ny * (1. - cos_t) + nx * sin_t;
  const real a22 = nz * nz * (1. - cos_t) +      cos_t;
  vector_t new = {
    .x = a00 * old.x + a01 * old.y + a02 * old.z,
    .y = a10 * old.x + a11 * old.y + a12 * old.z,
    .z = a20 * old.x + a21 * old.y + a22 * old.z
  };
  return new;
}

viewer_t *viewer_init(const size_t *size_image, const real *size_screen, const real angle_a, const real angle_p){
  viewer_t *viewer = common_calloc(1, sizeof(viewer_t));
  screen_t *screen = common_calloc(1, sizeof(screen_t));
  // initial screen is located on y axis
  const vector_t focal = {.x = 0., .y = 0., .z = 0.};
  vector_t camera = {.x = 0., .y = -3840., .z = 0.};
  // three corners of screen
  //   2------+
  //   |      |
  //   0------1
  const real screen_pos = 0.95;
  const real screen_y_default = camera.y * screen_pos + focal.y * (1. - screen_pos);
  vector_t corners[3] = {
    {
      .x = -0.5 * size_screen[0],
      .y = screen_y_default,
      .z = -0.5 * size_screen[1]
    },
    {
      .x = +0.5 * size_screen[0],
      .y = screen_y_default,
      .z = -0.5 * size_screen[1]
    },
    {
      .x = -0.5 * size_screen[0],
      .y = screen_y_default,
      .z = +0.5 * size_screen[1]
    }
  };
  // azimuthal and polar rotations
  const vector_t normal_a = {
    .x = 0.,
    .y = 0.,
    .z = 1.
  };
  const vector_t normal_p = {
    .x = -cos(angle_a),
    .y = -sin(angle_a),
    .z = 0.
  };
  camera = rodrigues(angle_a, normal_a, camera);
  camera = rodrigues(angle_p, normal_p, camera);
  for(size_t n = 0; n < 3; n++){
    corners[n] = rodrigues(angle_a, normal_a, corners[n]);
    corners[n] = rodrigues(angle_p, normal_p, corners[n]);
  }
  const vector_t h = {
    .x = corners[1].x - corners[0].x,
    .y = corners[1].y - corners[0].y,
    .z = corners[1].z - corners[0].z
  };
  const vector_t v = {
    .x = corners[2].x - corners[0].x,
    .y = corners[2].y - corners[0].y,
    .z = corners[2].z - corners[0].z
  };
  const vector_t dh = {
    h.x / size_image[0],
    h.y / size_image[0],
    h.z / size_image[0]
  };
  const vector_t dv = {
    v.x / size_image[1],
    v.y / size_image[1],
    v.z / size_image[1]
  };
  // locatioins of the pixels on the screen
  const size_t nitems = size_image[0] * size_image[1];
  vector_t *pixels = common_calloc(nitems, sizeof(vector_t));
  for(size_t n = 0; n < nitems; n++){
    const size_t i = n % size_image[0];
    const size_t j = n / size_image[0];
    const real x
      = corners[0].x
      + 0.5 * (2 * i + 1) * dh.x
      + 0.5 * (2 * j + 1) * dv.x;
    const real y
      = corners[0].y
      + 0.5 * (2 * i + 1) * dh.y
      + 0.5 * (2 * j + 1) * dv.y;
    const real z
      = corners[0].z
      + 0.5 * (2 * i + 1) * dh.z
      + 0.5 * (2 * j + 1) * dv.z;
    pixels[n].x = x;
    pixels[n].y = y;
    pixels[n].z = z;
  }
  // assign members
  screen->pixels = pixels;
  screen->h = h;
  screen->v = v;
  screen->c = corners[0];
  viewer->camera = camera;
  viewer->screen = screen;
  return viewer;
}

int viewer_finalise(viewer_t *viewer){
  common_free(viewer->screen->pixels);
  common_free(viewer->screen);
  common_free(viewer);
  return 0;
}

