#if !defined(IO_H)
#define IO_H

#include <stdio.h>

extern const char dtype_size_t[];
extern const char dtype_double[];

// file manipulations
extern FILE *my_fopen(const char *path, const char *mode);
extern int my_fclose(FILE *fp);
// input
extern void load_scalar(const char dname[], const char dsetname[], const size_t size, const char dtype[], void *buf);
extern void load_vector(const char dname[], const char dsetname[], const size_t nitems, const size_t size, const char dtype[], void *buf);
extern void load_vectors(const char dname[], const char prefix[], const size_t nparticles, double **buf);
// output
extern char *make_directory(const size_t iter);
extern void dump_scalar(const char dname[], const char dsetname[], const size_t size, const char dtype[], const void *buf);
extern void dump_vector(const char dname[], const char dsetname[], const size_t nitems, const size_t size, const char dtype[], const void *buf);
extern void dump_vectors(const char dname[], const char prefix[], const size_t nparticles, double **buf);

#endif // IO_H
