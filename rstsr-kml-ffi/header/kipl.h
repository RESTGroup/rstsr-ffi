/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * Description: the main header file of KML_IPL
 * Author: dyc
 * Create: 2024-06-28
 * Notes: NA
 */
#ifndef KIPL_H_
#define KIPL_H_

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#ifdef __cplusplus
extern "C"
{
#endif

// extn
void KIPL_Csint(int nData, float *xData, float *fData, float *coef);
float KIPL_Csval(float xi, int initv, float *xData, float *coef);
void KIPL_Cssmh(int nData, float *xData, float *fData, float *weight, float smpar, float *coef);
void KIPL_Cs1gd(int ideriv, int nVec, float *xVec, int initv, float *xData, float *coef, float *derValue);

void KIPL_Bsnak(int nData, float *xData, int k, float *xKnot);
void KIPL_Bsint(int nData, float *xData, float *fData, int k, float *xKnot, float *coef);
float KIPL_Bsval(float xi, int k, float *xKnot, int ncoef, float *coef);
float KIPL_Bsder(int ideriv, float xi, int k, float *xKnot, int ncoef, float *coef);

/* evaluation accelerator */
typedef struct {
    size_t cache;      /* cache of index   */
    size_t miss_count; /* keep statistics  */
    size_t hit_count;
} kml_interp_accel;

/* interpolation object type */
typedef struct {
    const char *name;
    unsigned int min_size;
    void *(*alloc)(size_t size);
    int (*init)(void *, const float xa[], const float ya[], size_t size);
    int (*eval)(const void *, const float xa[], const float ya[], size_t size,
                float x, kml_interp_accel *, float *y);
    int (*eval_deriv)(const void *, const float xa[], const float ya[], size_t size, float x,
                      kml_interp_accel *, float *y_p);
    int (*eval_deriv2)(const void *, const float xa[], const float ya[], size_t size,
                       float x, kml_interp_accel *, float *y_pp);
    int (*eval_integ)(const void *, const float xa[], const float ya[], size_t size, kml_interp_accel *,
                      float a, float b, float *result);
    void (*free)(void *);
} kml_float_interp_type;

typedef struct {
    const char *name;
    unsigned int min_size;
    void *(*alloc)(size_t xsize, size_t ysize);
    int (*init)(void *, const float xa[], const float ya[], const float za[], size_t xsize, size_t ysize);
    int (*eval)(const void *, const float xa[], const float ya[], const float za[], size_t xsize, size_t ysize,
                float x, float y, kml_interp_accel *, kml_interp_accel *, float *z);
    int (*eval_deriv_x)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                        size_t ysize, float x, float y, kml_interp_accel *, kml_interp_accel *, float *z_p);
    int (*eval_deriv_y)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                        size_t ysize, float x, float y, kml_interp_accel *, kml_interp_accel *, float *z_p);
    int (*eval_deriv_xx)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                         size_t ysize, float x, float y, kml_interp_accel *, kml_interp_accel *, float *z_pp);
    int (*eval_deriv_xy)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                         size_t ysize, float x, float y, kml_interp_accel *, kml_interp_accel *, float *z_pp);
    int (*eval_deriv_yy)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                         size_t ysize, float x, float y, kml_interp_accel *, kml_interp_accel *, float *z_pp);
    int (*v_eval)(const void *, const float xa[], const float ya[], const float za[], size_t xsize, size_t ysize,
        const float x[], const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);
    int (*v_eval_deriv_x)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                          size_t ysize, const float x[], const float y[], size_t xin_size, size_t yin_size, float *z,
                          size_t zin_size);
    int (*v_eval_deriv_y)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                          size_t ysize, const float x[], const float y[], size_t xin_size, size_t yin_size, float *z,
                          size_t zin_size);
    int (*v_eval_deriv_xx)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                           size_t ysize, const float x[], const float y[], size_t xin_size, size_t yin_size, float *z,
                           size_t zin_size);
    int (*v_eval_deriv_xy)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                           size_t ysize, const float x[], const float y[], size_t xin_size, size_t yin_size, float *z,
                           size_t zin_size);
    int (*v_eval_deriv_yy)(const void *, const float xa[], const float ya[], const float za[], size_t xsize,
                           size_t ysize, const float x[], const float y[], size_t xin_size, size_t yin_size, float *z,
                           size_t zin_size);
    void (*free)(void *);
} kml_float_interp2d_type;

/* general interpolation object */
typedef struct {
    const kml_float_interp_type *type;
    float xmin;
    float xmax;
    size_t size;
    void *state;
} kml_float_interp;

typedef struct {
    kml_float_interp *interp;
    float *x;
    float *y;
    size_t size;
} kml_float_spline;

typedef struct {
    const kml_float_interp2d_type *type; /* interpolation type */
    float xmin;                          /* minimum value of x for which data have been provided */
    float xmax;                          /* maximum value of x for which data have been provided */
    float ymin;                          /* minimum value of y for which data have been provided */
    float ymax;                          /* maximum value of y for which data have been provided */
    size_t xsize;                        /* number of x values provided */
    size_t ysize;                        /* number of y values provided */
    void *state;                         /* internal state object specific to the interpolation type */
} kml_float_interp2d;

typedef struct {
    kml_float_interp2d interp_object; /* low-level interpolation object */
    float *xarr;                      /* x data array */
    float *yarr;                      /* y data array */
    float *zarr;                      /* z data array */
} kml_float_spline2d;

/* available types */
extern const kml_float_interp_type *kml_float_interp_cspline;
extern const kml_float_interp_type *kml_float_interp_cspline_periodic;
extern const kml_float_interp2d_type *kml_float_interp2d_bilinear;
extern const kml_float_interp2d_type *kml_float_interp2d_bicubic;

// accel
kml_interp_accel *kml_interp_accel_alloc(void);
int kml_interp_accel_reset(kml_interp_accel *a);
void kml_interp_accel_free(kml_interp_accel *a);

// interp
kml_float_interp *kml_float_interp_alloc(const kml_float_interp_type *T, size_t size);
int kml_float_interp_init(kml_float_interp *interp, const float x_array[], const float y_array[], size_t size);
void kml_float_interp_free(kml_float_interp *interp);
const char *kml_float_interp_name(const kml_float_interp *interp);
unsigned int kml_float_interp_min_size(const kml_float_interp *interp);
unsigned int kml_float_interp_type_min_size(const kml_float_interp_type *T);

int kml_float_interp_eval_e(const kml_float_interp *interp, const float xa[],
                            const float ya[], float x, kml_interp_accel *a, float *y);

float kml_float_interp_eval(const kml_float_interp *interp, const float xa[], const float ya[], float x,
                            kml_interp_accel *a);

int kml_float_interp_eval_deriv_e(const kml_float_interp *interp, const float xa[], const float ya[], float x,
                                  kml_interp_accel *a, float *dydx);

float kml_float_interp_eval_deriv(const kml_float_interp *interp, const float xa[], const float ya[], float x,
                                  kml_interp_accel *a);

int kml_float_interp_eval_deriv2_e(const kml_float_interp *interp, const float xa[], const float ya[], float x,
                                   kml_interp_accel *a, float *d2);

float kml_float_interp_eval_deriv2(const kml_float_interp *interp, const float xa[],
                                   const float ya[], float x, kml_interp_accel *a);

int kml_float_interp_eval_integ_e(const kml_float_interp *interp, const float xa[], const float ya[],
                                  float a, float b, kml_interp_accel *acc, float *result);

float kml_float_interp_eval_integ(const kml_float_interp *interp, const float xa[], const float ya[],
                                  float a, float b, kml_interp_accel *acc);

// spline
kml_float_spline *kml_float_spline_alloc(const kml_float_interp_type *t, size_t size);
int kml_float_spline_init(kml_float_spline *spline, const float x_array[], const float y_array[], size_t size);
void kml_float_spline_free(kml_float_spline *spline);
const char *kml_float_spline_name(const kml_float_spline *spline);
unsigned int kml_float_spline_min_size(const kml_float_spline *spline);

int kml_float_spline_eval_e(const kml_float_spline *spline, float x,
                            kml_interp_accel *a, float *y);

float kml_float_spline_eval(const kml_float_spline *spline, float x, kml_interp_accel *a);

int kml_float_spline_eval_deriv_e(const kml_float_spline *spline, float x,
                                  kml_interp_accel *a, float *dydx);

float kml_float_spline_eval_deriv(const kml_float_spline *spline, float x, kml_interp_accel *a);

int kml_float_spline_eval_deriv2_e(const kml_float_spline *spline, float x, kml_interp_accel *a,
                                   float *d2);

float kml_float_spline_eval_deriv2(const kml_float_spline *spline,
                                   float x, kml_interp_accel *a);

int kml_float_spline_eval_integ_e(const kml_float_spline *spline,
                                  float a, float b,
                                  kml_interp_accel *acc,
                                  float *result);

float kml_float_spline_eval_integ(const kml_float_spline *spline,
                                  float a, float b,
                                  kml_interp_accel *acc);

// interp2d
kml_float_interp2d *kml_float_interp2d_alloc(const kml_float_interp2d_type *T, const size_t xsize, const size_t ysize);
int kml_float_interp2d_init(kml_float_interp2d *interp, const float xarr[], const float yarr[],
                            const float zarr[], const size_t xsize, const size_t ysize);
void kml_float_interp2d_free(kml_float_interp2d *interp);
const char *kml_float_interp2d_name(const kml_float_interp2d *interp);
size_t kml_float_interp2d_min_size(const kml_float_interp2d *interp);
size_t kml_float_interp2d_type_min_size(const kml_float_interp2d_type *T);

int kml_float_interp2d_set(const kml_float_interp2d *interp, float zarr[],
                           const size_t i, const size_t j, const float z);

float kml_float_interp2d_get(const kml_float_interp2d *interp, const float zarr[], const size_t i, const size_t j);

size_t kml_float_interp2d_idx(const kml_float_interp2d *interp, const size_t i, const size_t j);

float kml_float_interp2d_eval(const kml_float_interp2d *interp, const float xarr[],
                              const float yarr[], const float zarr[], const float x,
                              const float y, kml_interp_accel *xa, kml_interp_accel *ya);

float kml_float_interp2d_eval_extrap(const kml_float_interp2d *interp, const float xarr[], const float yarr[],
                                     const float zarr[], const float x, const float y, kml_interp_accel *xa,
                                     kml_interp_accel *ya);

int kml_float_interp2d_eval_e(const kml_float_interp2d *interp, const float xarr[],
                              const float yarr[], const float zarr[], const float x, const float y,
                              kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_e(const kml_float_interp2d *interp, const float xarr[],
                                const float yarr[], const float zarr[], const float x[], const float y[],
                                size_t xin_size, size_t yin_size, float *z, size_t zin_size);

/* DISABLE_DEPRECATED */
int kml_float_interp2d_eval_e_extrap(const kml_float_interp2d *interp, const float xarr[],
                                     const float yarr[], const float zarr[], const float x, const float y,
                                     kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_float_interp2d_eval_extrap_e(const kml_float_interp2d *interp, const float xarr[],
                                     const float yarr[], const float zarr[], const float x, const float y,
                                     kml_interp_accel *xa, kml_interp_accel *ya, float *z);

float kml_float_interp2d_eval_deriv_x(const kml_float_interp2d *interp, const float xarr[],
                                      const float yarr[], const float zarr[], const float x, const float y,
                                      kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_interp2d_eval_deriv_x_e(const kml_float_interp2d *interp, const float xarr[],
                                      const float yarr[], const float zarr[], const float x, const float y,
                                      kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_deriv_x_e(const kml_float_interp2d *interp, const float xarr[],
                                        const float yarr[], const float zarr[], const float x[], const float y[],
                                        size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_interp2d_eval_deriv_y(const kml_float_interp2d *interp, const float xarr[],
                                      const float yarr[], const float zarr[], const float x, const float y,
                                      kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_interp2d_eval_deriv_y_e(const kml_float_interp2d *interp, const float xarr[],
                                      const float yarr[], const float zarr[], const float x, const float y,
                                      kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_deriv_y_e(const kml_float_interp2d *interp, const float xarr[],
                                        const float yarr[], const float zarr[], const float x[], const float y[],
                                        size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_interp2d_eval_deriv_xx(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_interp2d_eval_deriv_xx_e(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_deriv_xx_e(const kml_float_interp2d *interp, const float xarr[],
                                         const float yarr[], const float zarr[], const float x[], const float y[],
                                         size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_interp2d_eval_deriv_yy(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_interp2d_eval_deriv_yy_e(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_deriv_yy_e(const kml_float_interp2d *interp, const float xarr[],
                                         const float yarr[], const float zarr[], const float x[], const float y[],
                                         size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_interp2d_eval_deriv_xy(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_interp2d_eval_deriv_xy_e(const kml_float_interp2d *interp, const float xarr[],
                                       const float yarr[], const float zarr[], const float x, const float y,
                                       kml_interp_accel *xa, kml_interp_accel *ya, float *z);

int kml_v_float_interp2d_eval_deriv_xy_e(const kml_float_interp2d *interp, const float xarr[],
                                         const float yarr[], const float zarr[], const float x[], const float y[],
                                         size_t xin_size, size_t yin_size, float *z, size_t zin_size);

// spline2d
kml_float_spline2d *kml_float_spline2d_alloc(const kml_float_interp2d_type *t, size_t xsize, size_t ysize);
int kml_float_spline2d_init(kml_float_spline2d *interp, const float xarr[],
                            const float yarr[], const float zarr[],
                            size_t xsize, size_t ysize);
void kml_float_spline2d_free(kml_float_spline2d *interp);
const char *kml_float_spline2d_name(const kml_float_spline2d *interp);
size_t kml_float_spline2d_min_size(const kml_float_spline2d *interp);

int kml_float_spline2d_set(const kml_float_spline2d *interp, float zarr[],
                           const size_t i, const size_t j, const float z);

float kml_float_spline2d_get(const kml_float_spline2d *interp, const float zarr[],
                             const size_t i, const size_t j);

float kml_float_spline2d_eval(const kml_float_spline2d *interp, const float x,
                              const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_e(const kml_float_spline2d *interp, const float x,
                              const float y, kml_interp_accel *xa, kml_interp_accel *ya,
                              float *z);

int kml_v_float_spline2d_eval_e(const kml_float_spline2d *interp, const float x[],
                                const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_spline2d_eval_extrap(const kml_float_spline2d *interp, const float x,
                                     const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_extrap_e(const kml_float_spline2d *interp, const float x,
                                     const float y, kml_interp_accel *xa, kml_interp_accel *ya,
                                     float *z);

float kml_float_spline2d_eval_deriv_x(const kml_float_spline2d *interp, const float x,
                                      const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_deriv_x_e(const kml_float_spline2d *interp, const float x,
                                      const float y, kml_interp_accel *xa,
                                      kml_interp_accel *ya, float *z);

int kml_v_float_spline2d_eval_deriv_x_e(const kml_float_spline2d *interp, const float x[],
                                        const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_spline2d_eval_deriv_y(const kml_float_spline2d *interp, const float x,
                                      const float y, kml_interp_accel *xa,
                                      kml_interp_accel *ya);

int kml_float_spline2d_eval_deriv_y_e(const kml_float_spline2d *interp, const float x,
                                      const float y, kml_interp_accel *xa,
                                      kml_interp_accel *ya, float *z);

int kml_v_float_spline2d_eval_deriv_y_e(const kml_float_spline2d *interp, const float x[],
                                        const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_spline2d_eval_deriv_xx(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_deriv_xx_e(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa,
                                       kml_interp_accel *ya, float *z);

int kml_v_float_spline2d_eval_deriv_xx_e(const kml_float_spline2d *interp, const float x[],
                                         const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_spline2d_eval_deriv_yy(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_deriv_yy_e(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa,
                                       kml_interp_accel *ya, float *z);

int kml_v_float_spline2d_eval_deriv_yy_e(const kml_float_spline2d *interp, const float x[],
                                         const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

float kml_float_spline2d_eval_deriv_xy(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa, kml_interp_accel *ya);

int kml_float_spline2d_eval_deriv_xy_e(const kml_float_spline2d *interp, const float x,
                                       const float y, kml_interp_accel *xa,
                                       kml_interp_accel *ya, float *z);

int kml_v_float_spline2d_eval_deriv_xy_e(const kml_float_spline2d *interp, const float x[],
                                         const float y[], size_t xin_size, size_t yin_size, float *z, size_t zin_size);

#ifdef __cplusplus
}
#endif

#endif /* KIPL_H */