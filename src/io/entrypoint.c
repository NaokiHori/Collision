#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <errno.h>
#include <assert.h>
#include "common.h"
#include "io.h"
#include "snpyio.h"


const char dtype_size_t[] = {"'<u8'"};
const char dtype_double[] = {"'<f8'"};

FILE *my_fopen(const char *path, const char *mode){
  errno = 0;
  FILE *fp = fopen(path, mode);
  if(fp == NULL){
    perror(path);
    if(0 == strcmp("r", mode)){
      exit(EXIT_FAILURE);
    }
  }
  return fp;
}

int my_fclose(FILE *fp){
  fclose(fp);
  return 0;
}

static char *concatenate(const char dname[], const char dsetname[]){
  const char suffix[] = {".npy"};
  const size_t nitems
    = strlen(dname)
    + 1
    + strlen(dsetname)
    + strlen(suffix)
    + 1;
  char *fname = common_calloc(nitems, sizeof(char));
  snprintf(fname, nitems, "%s/%s%s", dname, dsetname, suffix);
  fname[nitems-1] = '\0';
  return fname;
}

char *make_directory(const size_t iter){
  const char prefix[] = {"output/save/iter"};
  const int ndigits = 10;
  const size_t nitems =
    strlen(prefix)
    +
    1 // "/"
    +
    ndigits
    +
    1; // NUL
  char *dname = common_calloc(nitems, sizeof(char));
  snprintf(dname, nitems, "%s%0*zu", prefix, ndigits, iter);
  dname[nitems-1] = '\0';
  errno = 0;
  if(mkdir(dname, S_IRWXU | S_IRWXG | S_IRWXO) != 0){
    perror(dname);
    if(EEXIST != errno){
      common_free(dname);
      dname = NULL;
    }
  }
  return dname;
}

void load_scalar(const char dname[], const char dsetname[], const size_t size, const char dtype[], void *buf){
  char *fname = concatenate(dname, dsetname);
  FILE *fp = my_fopen(fname, "r");
  size_t ndim_ = 0;
  size_t *shape_ = NULL;
  char *dtype_ = NULL;
  bool is_fortran_order_ = false;
  snpyio_r_header(&ndim_, &shape_, &dtype_, &is_fortran_order_, fp);
  assert(0 == ndim_);
  assert(NULL == shape_);
  assert(0 == strcmp(dtype_, dtype));
  assert(false == is_fortran_order_);
  errno = 0;
  if(1 != fread(buf, size, 1, fp)){
    perror(fname);
    exit(EXIT_FAILURE);
  }
  my_fclose(fp);
  common_free(fname);
  common_free(shape_);
  common_free(dtype_);
}

void dump_scalar(const char dname[], const char dsetname[], const size_t size, const char dtype[], const void *buf){
  char *fname = concatenate(dname, dsetname);
  FILE *fp = my_fopen(fname, "w");
  const size_t ndim = 0;
  const size_t *shape = NULL;
  const bool is_fortran_order = false;
  snpyio_w_header(ndim, shape, dtype, is_fortran_order, fp);
  fwrite(buf, size, 1, fp);
  my_fclose(fp);
  common_free(fname);
}

void load_vector(const char dname[], const char dsetname[], const size_t nitems, const size_t size, const char dtype[], void *buf){
  char *fname = concatenate(dname, dsetname);
  FILE *fp = my_fopen(fname, "r");
  size_t ndim_ = 0;
  size_t *shape_ = NULL;
  char *dtype_ = NULL;
  bool is_fortran_order_ = false;
  snpyio_r_header(&ndim_, &shape_, &dtype_, &is_fortran_order_, fp);
  assert(1 == ndim_);
  assert(nitems == shape_[0]);
  assert(0 == strcmp(dtype_, dtype));
  assert(false == is_fortran_order_);
  errno = 0;
  if(nitems != fread(buf, size, nitems, fp)){
    perror(fname);
    exit(EXIT_FAILURE);
  }
  my_fclose(fp);
  common_free(fname);
  common_free(shape_);
  common_free(dtype_);
}

void dump_vector(const char dname[], const char dsetname[], const size_t nitems, const size_t size, const char dtype[], const void *buf){
  char *fname = concatenate(dname, dsetname);
  FILE *fp = my_fopen(fname, "w");
  const size_t ndim = 1;
  const size_t shape[1] = {nitems};
  const bool is_fortran_order = false;
  snpyio_w_header(ndim, shape, dtype, is_fortran_order, fp);
  fwrite(buf, size, nitems, fp);
  my_fclose(fp);
  common_free(fname);
}

void load_vectors(const char dname[], const char prefix[], const size_t nparticles, double **buf){
  for(dim_t dim = 0; dim < NDIMS; dim++){
    const size_t nitems = strlen(prefix) + 3;
    char *dsetname = common_calloc(nitems, sizeof(char));
    snprintf(dsetname, nitems, "%s_%u", prefix, dim);
    char *fname = concatenate(dname, dsetname);
    FILE *fp = my_fopen(fname, "r");
    size_t ndim = 0;
    size_t *shape = NULL;
    char *dtype = NULL;
    bool is_fortran_order = false;
    snpyio_r_header(&ndim, &shape, &dtype, &is_fortran_order, fp);
    errno = 0;
    if(nparticles != fread(buf[dim], sizeof(double), nparticles, fp)){
      perror(fname);
      exit(EXIT_FAILURE);
    }
    my_fclose(fp);
    common_free(dsetname);
    common_free(fname);
    common_free(shape);
    common_free(dtype);
  }
}

void dump_vectors(const char dname[], const char prefix[], const size_t nparticles, double **buf){
  for(dim_t dim = 0; dim < NDIMS; dim++){
    const size_t nitems = strlen(prefix) + 3;
    char *dsetname = common_calloc(nitems, sizeof(char));
    snprintf(dsetname, nitems, "%s_%u", prefix, dim);
    char *fname = concatenate(dname, dsetname);
    FILE *fp = my_fopen(fname, "w");
    const size_t ndim = 1;
    const size_t shape[] = {nparticles};
    const char dtype[] = "'float64'";
    const bool is_fortran_order = false;
    snpyio_w_header(ndim, shape, dtype, is_fortran_order, fp);
    fwrite(buf[dim], sizeof(double), nparticles, fp);
    my_fclose(fp);
    common_free(dsetname);
    common_free(fname);
  }
}

