#include <stdio.h>
#include <limits.h>
#include "common.h"
#include "config.h"
#include "particle.h"
#include "snpyio.h"


static void load_params(size_t *nparticles, double **lengths){
  *nparticles = config.get_size_t("nparticles");
  *lengths  = common_calloc(NDIMS, sizeof(double));
  for(dim_t dim = 0; dim < NDIMS; dim++){
#define BUFSIZE 128
    char dsetname[BUFSIZE];
    snprintf(dsetname, BUFSIZE, "lx%u", dim);
    (*lengths)[dim]  = config.get_double(dsetname);
#undef BUFSIZE
  }
}

static void unload_params(double *lengths){
  common_free(lengths);
}

static void output(const char root[], const double *lengths, const size_t nparticles, part_t *particles){
  const char dtype_size_t[] = {"'<u8'"};
  const char dtype_double[] = {"'<f8'"};
#define BUFSIZE 128
  // iter
  {
    const size_t iter = 0;
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/iter.npy", root);
    FILE *fp = fopen(fname, "w");
    snpyio_w_header(0, NULL, dtype_size_t, false, fp);
    fwrite(&iter, sizeof(size_t), 1, fp);
    fclose(fp);
  }
  // time
  {
    const double time = 0.;
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/time.npy", root);
    FILE *fp = fopen(fname, "w");
    snpyio_w_header(0, NULL, dtype_double, false, fp);
    fwrite(&time, sizeof(double), 1, fp);
    fclose(fp);
  }
  // lengths
  {
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/lengths.npy", root);
    FILE *fp = fopen(fname, "w");
    const size_t shape[] = {NDIMS};
    snpyio_w_header(1, shape, dtype_double, false, fp);
    fwrite(lengths, sizeof(double), NDIMS, fp);
    fclose(fp);
  }
  // nparticles
  {
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/nparticles.npy", root);
    FILE *fp = fopen(fname, "w");
    snpyio_w_header(0, NULL, dtype_size_t, false, fp);
    fwrite(&nparticles, sizeof(size_t), 1, fp);
    fclose(fp);
  }
  // particles, save as SoA
  // density
  {
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/densities.npy", root);
    double *buf = common_calloc(nparticles, sizeof(double));
    for(size_t n = 0; n < nparticles; n++){
      buf[n] = particles[n].density;
    }
    FILE *fp = fopen(fname, "w");
    const size_t shape[] = {nparticles};
    snpyio_w_header(1, shape, dtype_double, false, fp);
    fwrite(buf, sizeof(double), nparticles, fp);
    fclose(fp);
    common_free(buf);
  }
  // radius
  {
    char fname[BUFSIZE] = {'\0'};
    snprintf(fname, BUFSIZE, "%s/radii.npy", root);
    double *buf = common_calloc(nparticles, sizeof(double));
    for(size_t n = 0; n < nparticles; n++){
      buf[n] = particles[n].radius;
    }
    FILE *fp = fopen(fname, "w");
    const size_t shape[] = {nparticles};
    snpyio_w_header(1, shape, dtype_double, false, fp);
    fwrite(buf, sizeof(double), nparticles, fp);
    fclose(fp);
    common_free(buf);
  }
  // position
  for(dim_t dim = 0; dim < NDIMS; dim++){
    char fname[BUFSIZE] = {'\0'};
    double *buf = common_calloc(nparticles, sizeof(double));
    for(size_t n = 0; n < nparticles; n++){
      buf[n] = particles[n].position[dim];
    }
    snprintf(fname, BUFSIZE, "%s/positions_%u.npy", root, dim);
    FILE *fp = fopen(fname, "w");
    const size_t shape[] = {nparticles};
    snpyio_w_header(1, shape, dtype_double, false, fp);
    fwrite(buf, sizeof(double), nparticles, fp);
    fclose(fp);
    common_free(buf);
  }
  // velocity
  for(dim_t dim = 0; dim < NDIMS; dim++){
    char fname[BUFSIZE] = {'\0'};
    double *buf = common_calloc(nparticles, sizeof(double));
    for(size_t n = 0; n < nparticles; n++){
      buf[n] = particles[n].velocity[dim];
    }
    snprintf(fname, BUFSIZE, "%s/velocities_%u.npy", root, dim);
    FILE *fp = fopen(fname, "w");
    const size_t shape[] = {nparticles};
    snpyio_w_header(1, shape, dtype_double, false, fp);
    fwrite(buf, sizeof(double), nparticles, fp);
    fclose(fp);
    common_free(buf);
  }
#undef BUFSIZE
}

int main(void){
  const char root[] = {"../"};
  config.load();
  size_t nparticles = 0;
  double *lengths = NULL;
  load_params(&nparticles, &lengths);
  part_t *particles = init_particles(lengths, nparticles);
  output(root, lengths, nparticles, particles);
  finalise_particles(particles);
  unload_params(lengths);
  config.unload();
  return 0;
}

