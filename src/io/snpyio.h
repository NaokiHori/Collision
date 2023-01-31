// https://github.com/NaokiHori/SimpleNpyIO

#if !defined(SNPYIO_H)
#define SNPYIO_H

#include <stdio.h>   // size_t, FILE
#include <stdbool.h> // bool

extern size_t snpyio_w_header(const size_t  ndim, const size_t  *shape, const char dtype[], const bool  is_fortran_order, FILE *fp);
extern size_t snpyio_r_header(      size_t *ndim,       size_t **shape,       char **dtype,       bool *is_fortran_order, FILE *fp);

#endif // SNPYIO_H
