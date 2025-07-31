/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2020. All rights reserved.
 * Description: DesignWare kunpeng libmath interface definition.
 * Author:
 * Create: 2020-08-28
 */

#ifndef KML_LIBM_H
#define KML_LIBM_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

#ifndef _Complex_I
#define _Complex_I (__extension__ 1.0iF)
#endif
#undef I
#define I _Complex_I

#ifndef FP_NAN
#define FP_NAN 0
#endif

#ifndef FP_INFINITE
#define FP_INFINITE 1
#endif

#ifndef FP_ZERO
#define FP_ZERO 2
#endif

#ifndef FP_SUBNORMAL
#define FP_SUBNORMAL 3
#endif

#ifndef FP_NORMAL
#define FP_NORMAL 4
#endif

#undef copysign
#undef copysignf
#undef fma
#undef fmaf
#undef fpclassify
#undef fpclassifyf
#undef isfinite
#undef isfinitef
#undef isgreater
#undef isgreaterf
#undef isgreaterequal
#undef isgreaterequalf
#undef isinf
#undef isinff
#undef isless
#undef islessf
#undef islessequal
#undef islessequalf
#undef islessgreater
#undef islessgreaterf
#undef isnan
#undef isnanf
#undef isnormal
#undef isnormalf
#undef isunordered
#undef isunorderedf
#undef signbit
#undef signbitf
#undef scalbn
#undef scalbnf
#undef scalbln
#undef scalblnf
#undef modf
#undef modff
#undef frexp
#undef frexpf
#undef ldexp
#undef ldexpf

#define copysign(x, y)            __builtin_copysign(x, y)
#define copysignf(x, y)           __builtin_copysignf(x, y)
#define fma(x,  y,  z)            __builtin_fma(x, y, z)
#define fmaf(x,  y,  z)           __builtin_fmaf(x, y, z)
#define fpclassify(x)             __builtin_fpclassify(FP_NAN, FP_INFINITE, FP_NORMAL, FP_SUBNORMAL, FP_ZERO, x)
#define fpclassifyf(x)            __builtin_fpclassify(FP_NAN, FP_INFINITE, FP_NORMAL, FP_SUBNORMAL, FP_ZERO, x)
#define frexp(x, exp)             __builtin_frexp(x, exp)
#define frexpf(x, exp)            __builtin_frexpf(x, exp)
#define isfinite(x)               __builtin_isfinite(x)
#define isfinitef(x)              __builtin_isfinite(x)
#define isgreater(x,  y)          __builtin_isgreater(x, y)
#define isgreaterf(x,  y)         __builtin_isgreater(x, y)
#define isgreaterequal(x,  y)     __builtin_isgreaterequal(x, y)
#define isgreaterequalf(x,  y)    __builtin_isgreaterequal(x, y)
#define isinf(x)                  __builtin_isinf(x)
#define isinff(x)                 __builtin_isinf(x)
#define isless(x,  y)             __builtin_isless(x, y)
#define islessf(x,  y)            __builtin_isless(x, y)
#define islessequal(x,  y)        __builtin_islessequal(x, y)
#define islessequalf(x,  y)       __builtin_islessequal(x, y)
#define islessgreater(x,  y)      __builtin_islessgreater(x, y)
#define islessgreaterf(x,  y)     __builtin_islessgreater(x, y)
#define isnan(x)                  __builtin_isnan(x)
#define isnanf(x)                 __builtin_isnan(x)
#define isnormal(x)               __builtin_isnormal(x)
#define isnormalf(x)              __builtin_isnormal(x)
#define isunordered(x,  y)        __builtin_isunordered(x, y)
#define isunorderedf(x,  y)       __builtin_isunordered(x, y)
#define ldexp(x, exp)             __builtin_ldexp(x, exp)
#define ldexpf(x, exp)            __builtin_ldexpf(x, exp)
#define modf(x, iptr)             __builtin_modf(x, iptr)
#define modff(x, iptr)            __builtin_modff(x, iptr)
#define scalbln(x, n)             __builtin_scalbln(x, n)
#define scalblnf(x, n)            __builtin_scalblnf(x, n)
#define scalbn(x, n)              __builtin_scalbn(x, n)
#define scalbnf(x, n)             __builtin_scalbnf(x, n)
#define signbit(x)                __builtin_signbit(x)
#define signbitf(x)               __builtin_signbitf(x)

#define KM_VERSION_STRUCT_LEN 100
typedef struct {
    char productName[KM_VERSION_STRUCT_LEN];
    char productVersion[KM_VERSION_STRUCT_LEN];
    char componentName[KM_VERSION_STRUCT_LEN];
    char componentVersion[KM_VERSION_STRUCT_LEN];
    char componentAppendInfo[KM_VERSION_STRUCT_LEN];
    char softwareName[KM_VERSION_STRUCT_LEN];
    char softwareVersion[KM_VERSION_STRUCT_LEN];
} KMVersion;

int KMGetVersion(KMVersion *ver);

// Trigonometric Functions
double acos(double);
float acosf(float);
double acosd(double);
float acosdf(float);
double acospi(double x);
float acospif(float x);
double asin(double);
float asinf(float);
float asindf(float);
double asind(double);
double asinpi(double x);
float asinpif(float x);
double atan(double);
float atanf(float);
double atanpi(double x);
float atanpif(float x);
double atan2(double, double);
float atan2f(float, float);
long double atan2l(long double, long double);
long double atan2dl(long double, long double);
double atan2d(double, double);
float atan2df(float, float);
double atan2pi(double y, double x);
float atan2pif(float y, float x);
double atand(double);
float atandf(float);
double cos(double);
float cosf(float);
long double cosl(long double);
float cosdf(float);
long double cosdl(long double);
double cosd(double);
double cospi(double x);
float cospif(float x);
double cot(double);
float cotf(float);
double cotd(double);
float cotdf(float);
double sin(double);
float sinf(float);
long double sinl(long double);
double sind(double x);
float sindf(float x);
long double sindl(long double);
void sincos(double, double *, double *);
void sincosf(float, float *, float *);
void sincosd(double, double *, double *);
void sincosdf(float, float *, float *);
float sindf(float);
double sind(double);
double sinpi(double x);
float sinpif(float x);
double tan(double);
float tanf(float);
double tand(double);
float tandf(float);
double tanpi(double x);
float tanpif(float x);
// Hyperbolic Functions
double acosh(double);
float acoshf(float);
double asinh(double);
float asinhf(float);
double atanh(double);
float atanhf(float);
double cosh(double);
float coshf(float);
double sinh(double);
float sinhf(float);
void sincosh(double, double *, double *);
void sincoshf(float, float *, float *);
void sinhcosh(double x, double *sinhval, double *coshval);
void sinhcoshf(float x, float *sinhval, float *coshval);
double tanh(double);
float tanhf(float);
// Exponential Functions
double cbrt(double);
float cbrtf(float);
double exp(double);
float expf(float);
long double expl(long double);
double exp10(double);
float exp10f(float);
double exp2(double);
float exp2f(float);
double expm1(double);
float expm1f(float);
double frexp(double, int *);
float frexpf(float, int*);
double hypot(double, double);
float hypotf(float, float);
double invsqrt(double);
float invsqrtf(float);
int ilogb(double);
int ilogbf(float);
double ldexp(double, int);
float ldexpf(float, int);
double log(double);
float logf(float);
long double logl(long double);
double log10(double);
float log10f(float);
double log1p(double);
float log1pf(float);
double log2(double);
float log2f(float);
double logb(double);
float logbf(float);
double pow(double, double);
float powf(float, float);
double pow2o3(double x);
float pow2o3f(float x);
double pow3o2(double x);
float pow3o2f(float x);
double powr(double x, double y);
float powrf(float x, float y);
long double powl(long double, long double);
double scalb(double, double);
float scalbf(float, float);
double scalbln(double, long int);
float scalblnf(float, long int);
double scalbn(double, int);
float scalbnf(float, int);
double sqrt(double);
float sqrtf(float);
long double sqrtl(long double);
// Complex Functions
double cabs(double _Complex);
float cabsf(float _Complex);
double _Complex cacos(double _Complex);
float _Complex cacosf(float _Complex);
double _Complex cacosh(double _Complex);
float _Complex cacoshf(float _Complex);
double carg(double _Complex);
float cargf(float _Complex);
double _Complex casin(double _Complex);
float _Complex casinf(float _Complex);
double _Complex casinh(double _Complex);
float _Complex casinhf(float _Complex);
double _Complex catan(double _Complex);
float _Complex catanf(float _Complex);
double _Complex catanh(double _Complex);
float _Complex catanhf(float _Complex);
double _Complex ccos(double _Complex);
_Complex float ccosf(_Complex float);
_Complex double cexp(_Complex double);
_Complex float cexpf(_Complex float);
double _Complex cexp2(double _Complex);
float _Complex cexp2f(float _Complex);
double _Complex cexp10(double _Complex);
float _Complex cexp10f(float _Complex);
double cimag(double _Complex);
float cimagf(float _Complex);
double _Complex cis(double);
float _Complex cisf(float);
double _Complex cisd(double);
float _Complex cisdf(float);
_Complex double clog(_Complex double);
float _Complex clogf(float _Complex);
_Complex double clog2(_Complex double);
float _Complex clog2f(float _Complex);
double _Complex clog10(double _Complex);
float _Complex clog10f(float _Complex);
double _Complex conj(double _Complex);
float _Complex conjf(float _Complex);
double _Complex ccosh(double _Complex);
float _Complex ccoshf(float _Complex);
double _Complex cpow(double _Complex, double _Complex);
float _Complex cpowf(float _Complex, float _Complex);
double _Complex cproj(double _Complex);
float _Complex cprojf(float _Complex);
double creal(double _Complex);
float crealf(float _Complex);
_Complex double csin(_Complex double);
_Complex float csinf(_Complex float);
double _Complex csinh(double _Complex);
float _Complex csinhf(float _Complex);
double _Complex csqrt(double _Complex);
float _Complex csqrtf(float _Complex);
double _Complex ctan(double _Complex);
float _Complex ctanf(float _Complex);
double _Complex ctanh(double _Complex);
float _Complex ctanhf(float _Complex);
_Complex double cceil(_Complex double);
_Complex float cceilf(_Complex float);
// Nearest Integer Functions
double ceil(double);
float ceilf(float);
double floor(double);
float floorf(float);
long long int llrint(double);
long long int llrintf(float);
long long int llround(double);
long long int llroundf(float);
long int lrint(double);
long int lrintf(float);
long int lround(double);
long int lroundf(float);
double modf(double, double *);
float modff(float, float *);
double nearbyint(double);
float nearbyintf(float);
double rint(double);
float rintf(float);
double round(double);
float roundf(float);
double trunc(double);
float truncf(float);
// Remainder Functions
float fmodf(float, float);
double fmod(double, double);
double remainder(double, double);
float remainderf(float, float);
double drem(double, double);
float dremf(float, float);
double remquo(double, double, int *);
float remquof(float, float, int *);
// Special Functions
double fmax(double x, double y);
float fmaxf(float x, float y);
double fmin(double x, double y);
float fminf(float x, float y);
double fmaxmag(double x, double y);
float fmaxmagf(float x, float y);
double fminmag(double x, double y);
float fminmagf(float x, float y);
double nextafter(double, double);
float nextafterf(float, float);
double significand(double);
float significandf(float);
double nexttoward(double, long double);
float nexttowardf(float, long double);
double annuity(double, double);
float annuityf(float, float);
double compound(double, double);
float compoundf(float, float);
float fabsf(float x);
double fabs(double x);
double erf(double);
float erff(float);
double erfcx(double);
float erfcxf(float);
double erfc(double);
float erfcf(float);
double erfinv(double);
float erfinvf(float);
int finitef(float x);
int finite(double x);
float gammaf(float);
double gamma(double);
double gamma_r(double, int *);
float gammaf_r(float, int *);
double j0(double);
float j0f(float);
double j1(double);
float j1f(float);
double jn(int, double);
float jnf(int, float);
float nanf(const char* tagp);
double nan(const char* tagp);
double lgamma(double);
float lgammaf(float);
double lgamma_r(double, int *);
float lgammaf_r(float, int *);
float tgammaf(float);
double tgamma(double);
double y0(double);
float y0f(float);
double y1(double);
float y1f(float);
double yn(int, double);
float ynf(int, float);
// other functions non standard
double _Complex cdiv(double _Complex, double _Complex);
float _Complex cdivf(float _Complex, float _Complex);
#ifdef __cplusplus
}
#endif

#endif
