#include <stdio.h>
#include <stdlib.h>
#include <float.h>
#include "vector.h"
#include "viewer.h"
#include "image.h"
#include "kernel.h"


static int inverse33(real *a){
  /* inverse 3x3 matrix */
  real a00 = a[0 * 3 + 0], a01 = a[0 * 3 + 1], a02 = a[0 * 3 + 2];
  real a10 = a[1 * 3 + 0], a11 = a[1 * 3 + 1], a12 = a[1 * 3 + 2];
  real a20 = a[2 * 3 + 0], a21 = a[2 * 3 + 1], a22 = a[2 * 3 + 2];
  real det =
    + a00 * a11 * a22
    + a01 * a12 * a20
    + a02 * a10 * a21
    - a02 * a11 * a20
    - a01 * a10 * a22
    - a00 * a12 * a21;
  if(det < DBL_EPSILON){
    printf("singular matrix\n");
    exit(1);
  }
  det = 1./det;
  real b00 = det * ( + a11 * a22 - a12 * a21 );
  real b01 = det * ( - a01 * a22 + a02 * a21 );
  real b02 = det * ( + a01 * a12 - a02 * a11 );
  real b10 = det * ( - a10 * a22 + a12 * a20 );
  real b11 = det * ( + a00 * a22 - a02 * a20 );
  real b12 = det * ( - a00 * a12 + a02 * a10 );
  real b20 = det * ( + a10 * a21 - a11 * a20 );
  real b21 = det * ( - a00 * a21 + a01 * a20 );
  real b22 = det * ( + a00 * a11 - a01 * a10 );
  a[0 * 3 + 0] = b00, a[0 * 3 + 1] = b01, a[0 * 3 + 2] = b02;
  a[1 * 3 + 0] = b10, a[1 * 3 + 1] = b11, a[1 * 3 + 2] = b12;
  a[2 * 3 + 0] = b20, a[2 * 3 + 1] = b21, a[2 * 3 + 2] = b22;
  return 0;
}

static void solve(real *a, real *b, real *c){
  inverse33(a);
  for(size_t n = 0; n < 3; n++){
    c[n]
      = a[n * 3 + 0] * b[0]
      + a[n * 3 + 1] * b[1]
      + a[n * 3 + 2] * b[2];
  }
}

int add_spheres(const size_t nspheres, const sphere_t *spheres, const viewer_t *viewer, image_t *image){
  // image
  const size_t nitems = image->size[0] * image->size[1];
  color_t *data = image->data;
  const vector_t camera = viewer->camera;
  const screen_t *screen = viewer->screen;
  const vector_t *pixels = screen->pixels;
  const vector_t screen_h = screen->h;
  const vector_t screen_v = screen->v;
  const vector_t screen_nh = normalise(screen_h);
  const vector_t screen_nv = normalise(screen_v);
  const vector_t screen_c = screen->c;
  real *ts = common_calloc(nitems, sizeof(real));
  for(size_t n = 0; n < nitems; n++){
    ts[n] = DBL_MAX;
  }
  for(size_t np = 0; np < nspheres; np++){
    if(np % 1000 == 0){
      printf("%s: %16zu / %16zu\n", __func__, np, nspheres);
    }
#define NEDGES 4
    const real radius = spheres[np].radius;
    const vector_t center = spheres[np].center;
    vector_t edges[NEDGES] = {
      {
        .x = center.x - radius * screen_nh.x,
        .y = center.y - radius * screen_nh.y,
        .z = center.z - radius * screen_nh.z
      },
      {
        .x = center.x + radius * screen_nh.x,
        .y = center.y + radius * screen_nh.y,
        .z = center.z + radius * screen_nh.z
      },
      {
        .x = center.x - radius * screen_nv.x,
        .y = center.y - radius * screen_nv.y,
        .z = center.z - radius * screen_nv.z
      },
      {
        .x = center.x + radius * screen_nv.x,
        .y = center.y + radius * screen_nv.y,
        .z = center.z + radius * screen_nv.z
      }
    };
    size_t imin = 0;
    size_t imax = image->size[0] - 1;
    size_t jmin = 0;
    size_t jmax = image->size[1] - 1;
    for(size_t ne = 0; ne < NEDGES; ne++){
      // vector from camera to edge
      vector_t l = {
        .x = edges[ne].x - camera.x,
        .y = edges[ne].y - camera.y,
        .z = edges[ne].z - camera.z
      };
      // initialise linear system
      real a[9] = {
        screen_h.x, screen_v.x, -l.x,
        screen_h.y, screen_v.y, -l.y,
        screen_h.z, screen_v.z, -l.z
      };
      real b[3] = {
        camera.x - screen_c.x,
        camera.y - screen_c.y,
        camera.z - screen_c.z
      };
      real params[3] = {0.};
      solve(a, b, params);
      const real param_h = params[0];
      const real param_v = params[1];
      if(ne == 0){
        const size_t tmp = param_h * image->size[0] - 1;
        imin = imin > tmp ? imin : tmp;
      }else if(ne == 1){
        const size_t tmp = param_h * image->size[0] + 1;
        imax = imax < tmp ? imax : tmp;
      }else if(ne == 2){
        const size_t tmp = param_v * image->size[1] - 1;
        jmin = jmin > tmp ? jmin : tmp;
      }else{
        const size_t tmp = param_v * image->size[1] + 1;
        jmax = jmax < tmp ? jmax : tmp;
      }
    }
    for(size_t j = jmin; j <= jmax; j++){
      for(size_t i = imin; i <= imax; i++){
        const size_t n = j * image->size[0] + i;
        vector_t ray = {
          .x = pixels[n].x - camera.x,
          .y = pixels[n].y - camera.y,
          .z = pixels[n].z - camera.z
        };
        ray = normalise(ray);
        const vector_t pos = {
          camera.x - center.x,
          camera.y - center.y,
          camera.z - center.z
        };
        const real a = 1. * dot(ray, ray);
        const real b = 2. * dot(ray, pos);
        const real c = 1. * dot(pos, pos) - pow(radius, 2.);
        const real d = pow(b, 2.) - 4. * a * c;
        if(d < 0.){
          continue;
        }
        const real t = 0.5 / a * (-b - sqrt(d));
        if(t < 0.){
          continue;
        }
        if(t > ts[n]){
          continue;
        }
        ts[n] = t;
        data[n].r = spheres[np].color.r;
        data[n].g = spheres[np].color.g;
        data[n].b = spheres[np].color.b;
      }
    }
#undef NEDGES
  }
  common_free(ts);
  return 0;
}

