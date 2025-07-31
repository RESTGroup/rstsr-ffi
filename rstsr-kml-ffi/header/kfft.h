/*
 * @Copyright: Copyright (c) Huawei Technologies Co., Ltd. 2020-2022. All rights reserved.
 * @Description: global api declaration of kml_fft, include single and double precision
 * @Author: kpl
 * @Date: 2020-04-09 18:53:21
 */
#ifndef __KML_FFT_H__
#define __KML_FFT_H__

#include <stdio.h>
#include <stddef.h>
#include <arm_neon.h>

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

#define CPX_ELEM 2

#define KML_FFT_VERSION_LENGTH 50
extern const char kml_fft_version[KML_FFT_VERSION_LENGTH];
extern const char kml_fftf_version[KML_FFT_VERSION_LENGTH];
extern const char kml_ffth_version[KML_FFT_VERSION_LENGTH];

#define VERSION_STRUCT_LEN 100
typedef struct {
    char productName[VERSION_STRUCT_LEN];
    char productVersion[VERSION_STRUCT_LEN];
    char componentName[VERSION_STRUCT_LEN];
    char componentVersion[VERSION_STRUCT_LEN];
    char componentAppendInfo[VERSION_STRUCT_LEN];
    char softwareName[VERSION_STRUCT_LEN];
    char softwareVersion[VERSION_STRUCT_LEN];
}KFFTVersion;
int KFFTGetVersion(KFFTVersion *ver);

#define KML_FFT_DEFINE_COMPLEX(R, C) typedef R C[2]
#define KML_FFT_DEFINE_FLOAT_COMPLEX(FR, FC) typedef FR FC[2]

#define KML_FFT_CONCAT(prefix, name) prefix ## name
#define KML_FFT_MANGLE_DOUBLE(name) KML_FFT_CONCAT(kml_fft_, name)
#define KML_FFT_MANGLE_FLOAT(name) KML_FFT_CONCAT(kml_fftf_, name)
#define KML_FFT_MANGLE_FP16(name) KML_FFT_CONCAT(kml_ffth_, name)

/* common global definition */
#define KML_FFT_FORWARD (-1)
#define KML_FFT_BACKWARD 1

#define KML_FFT_NO_TIMELIMIT (-1.0)

#define KML_FFT_MEASURE 0U
#define KML_FFT_DESTROY_INPUT (1U << 0)
#define KML_FFT_PATIENT (1U << 5)
#define KML_FFT_ESTIMATE (1U << 6)
#define KML_FFT_PRESERVE_INPUT (1U << 4)

#define KML_FFT_EXTERN extern
#define KML_FFT_CDECL

enum kml_fft_r2r_kind_do_not_use_me {
    KML_FFT_R2HC = 0, KML_FFT_HC2R = 1, KML_FFT_DHT = 2,
    KML_FFT_REDFT00 = 3, KML_FFT_REDFT01 = 4, KML_FFT_REDFT10 = 5, KML_FFT_REDFT11 = 6,
    KML_FFT_RODFT00 = 7, KML_FFT_RODFT01 = 8, KML_FFT_RODFT10 = 9, KML_FFT_RODFT11 = 10
};

struct kml_fft_iodim_do_not_use_me {
    int n;             /* dimension size */
    int is;            /* input stride */
    int os;            /* output stride */
};

struct kml_fft_iodim64_do_not_use_me {
    ptrdiff_t n;             /* dimension size */
    ptrdiff_t is;            /* input stride */
    ptrdiff_t os;            /* output stride */
};

#define KML_FFT_DEFINE_API(X, R, C)                                        \
                                                                        \
KML_FFT_DEFINE_COMPLEX(R, C);                                              \
                                                                        \
typedef struct X(plan_s) *X(plan);                                      \
                                                                        \
typedef struct kml_fft_iodim_do_not_use_me X(iodim);                       \
typedef struct kml_fft_iodim64_do_not_use_me X(iodim64);                   \
                                                                        \
typedef enum kml_fft_r2r_kind_do_not_use_me X(r2r_kind);                   \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute)(const X(plan) p);                                 \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft)(int rank, const int *n,                          \
                       C *in, C *out, int sign, unsigned flags);        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_1d)(int n, C *in, C *out, int sign,               \
                          unsigned flags);                              \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_2d)(int n0, int n1,                               \
                          C *in, C *out, int sign, unsigned flags);     \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_3d)(int n0, int n1, int n2,                       \
                          C *in, C *out, int sign, unsigned flags);     \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft)(int rank, const int *n,                     \
                            int howmany,                                \
                            C *in, const int *inembed,                  \
                            int istride, int idist,                     \
                            C *out, const int *onembed,                 \
                            int ostride, int odist,                     \
                            int sign, unsigned flags);                  \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_dft)(int rank, const X(iodim) *dims,             \
                            int howmany_rank,                           \
                            const X(iodim) *howmany_dims,               \
                            C *in, C *out,                              \
                            int sign, unsigned flags);                  \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_split_dft)(int rank, const X(iodim) *dims,       \
                                  int howmany_rank,                     \
                                  const X(iodim) *howmany_dims,         \
                                  R *ri, R *ii, R *ro, R *io,           \
                                  unsigned flags);                      \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_dft)(int rank,                                 \
                              const X(iodim64) *dims,                   \
                              int howmany_rank,                         \
                              const X(iodim64) *howmany_dims,           \
                              C *in, C *out,                            \
                              int sign, unsigned flags);                \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_split_dft)(int rank,                           \
                                    const X(iodim64) *dims,             \
                                    int howmany_rank,                   \
                                    const X(iodim64) *howmany_dims,     \
                                    R *ri, R *ii, R *ro, R *io,         \
                                    unsigned flags);                    \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft)(const X(plan) p, C *in, C *out);              \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_split_dft)(const X(plan) p, R *ri, R *ii,          \
                                      R *ro, R *io);                    \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft_r2c)(int rank, const int *n,                 \
                                int howmany,                            \
                                R *in, const int *inembed,              \
                                int istride, int idist,                 \
                                C *out, const int *onembed,             \
                                int ostride, int odist,                 \
                                unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_r2c)(int rank, const int *n,                      \
                           R *in, C *out, unsigned flags);              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_r2c_1d)(int n, R *in, C *out, unsigned flags);       \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_r2c_2d)(int n0, int n1,                           \
                              R *in, C *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_r2c_3d)(int n0, int n1,                           \
                              int n2,                                   \
                              R *in, C *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft_c2r)(int rank, const int *n,                 \
                                int howmany,                            \
                                C *in, const int *inembed,              \
                                int istride, int idist,                 \
                                R *out, const int *onembed,             \
                                int ostride, int odist,                 \
                                unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_c2r)(int rank, const int *n,                      \
                           C *in, R *out, unsigned flags);              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_c2r_1d)(int n, C *in, R *out, unsigned flags);       \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_c2r_2d)(int n0, int n1,                           \
                              C *in, R *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_c2r_3d)(int n0, int n1,                           \
                              int n2,                                   \
                              C *in, R *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_dft_r2c)(int rank, const X(iodim) *dims,         \
                                int howmany_rank,                       \
                                const X(iodim) *howmany_dims,           \
                                R *in, C *out,                          \
                                unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_dft_c2r)(int rank, const X(iodim) *dims,         \
                                int howmany_rank,                       \
                                const X(iodim) *howmany_dims,           \
                                C *in, R *out,                          \
                                unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_split_dft_r2c)(int rank, const X(iodim) *dims,   \
                                      int howmany_rank,                 \
                                      const X(iodim) *howmany_dims,     \
                                      R *in, R *ro, R *io,              \
                                      unsigned flags);                  \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_split_dft_c2r)(int rank, const X(iodim) *dims,   \
                                      int howmany_rank,                 \
                                      const X(iodim) *howmany_dims,     \
                                      R *ri, R *ii, R *out,             \
                                      unsigned flags);                  \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_dft_r2c)(int rank,                             \
                                  const X(iodim64) *dims,               \
                                  int howmany_rank,                     \
                                  const X(iodim64) *howmany_dims,       \
                                  R *in, C *out,                        \
                                  unsigned flags);                      \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_dft_c2r)(int rank,                             \
                                  const X(iodim64) *dims,               \
                                  int howmany_rank,                     \
                                  const X(iodim64) *howmany_dims,       \
                                  C *in, R *out,                        \
                                  unsigned flags);                      \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_split_dft_r2c)(int rank, const X(iodim64) *dims, \
                                        int howmany_rank,               \
                                        const X(iodim64) *howmany_dims, \
                                        R *in, R *ro, R *io,            \
                                        unsigned flags);                \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_split_dft_c2r)(int rank, const X(iodim64) *dims, \
                                        int howmany_rank,               \
                                        const X(iodim64) *howmany_dims, \
                                        R *ri, R *ii, R *out,           \
                                        unsigned flags);                \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft_r2c)(const X(plan) p, R *in, C *out);          \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft_c2r)(const X(plan) p, C *in, R *out);          \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_split_dft_r2c)(const X(plan) p,                    \
                                    R *in, R *ro, R *io);               \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_split_dft_c2r)(const X(plan) p,                    \
                                    R *ri, R *ii, R *out);              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_r2r)(int rank, const int *n,                     \
                            int howmany,                                \
                            R *in, const int *inembed,                  \
                            int istride, int idist,                     \
                            R *out, const int *onembed,                 \
                            int ostride, int odist,                     \
                            const X(r2r_kind) *kind, unsigned flags);   \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_r2r)(int rank, const int *n, R *in, R *out,           \
                       const X(r2r_kind) *kind, unsigned flags);        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_r2r_1d)(int n, R *in, R *out,                         \
                          X(r2r_kind) kind, unsigned flags);            \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_r2r_2d)(int n0, int n1, R *in, R *out,                \
                          X(r2r_kind) kind0, X(r2r_kind) kind1,         \
                          unsigned flags);                              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_r2r_3d)(int n0, int n1, int n2,                       \
                          R *in, R *out, X(r2r_kind) kind0,             \
                          X(r2r_kind) kind1, X(r2r_kind) kind2,         \
                          unsigned flags);                              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru_r2r)(int rank, const X(iodim) *dims,             \
                            int howmany_rank,                           \
                            const X(iodim) *howmany_dims,               \
                            R *in, R *out,                              \
                            const X(r2r_kind) *kind, unsigned flags);   \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_guru64_r2r)(int rank, const X(iodim64) *dims,         \
                              int howmany_rank,                         \
                              const X(iodim64) *howmany_dims,           \
                              R *in, R *out,                            \
                              const X(r2r_kind) *kind, unsigned flags); \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_r2r)(const X(plan) p, R *in, R *out);              \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(destroy_plan)(X(plan) p);                                  \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(destroy_plan_ext)(X(plan) p);                              \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(cleanup)(void);                                            \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(plan_with_nthreads)(int nthreads);                         \
                                                                        \
KML_FFT_EXTERN void                                                     \
KML_FFT_CDECL X(plan_with_nthreads_local)(int nthreads);                \
                                                                        \
KML_FFT_EXTERN int                                                      \
KML_FFT_CDECL X(init_threads)(void);                                       \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(cleanup_threads)(void);                                    \
                                                                        \
KML_FFT_EXTERN void *                                                      \
KML_FFT_CDECL X(malloc)(size_t n);                                         \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(free)(void *p);                                            \
                                                                        \
KML_FFT_EXTERN const char X(version)[]

/* end of KML_FFT_DEFINE_API macro */

KML_FFT_DEFINE_API(KML_FFT_MANGLE_DOUBLE, double, kml_fft_complex);
KML_FFT_DEFINE_API(KML_FFT_MANGLE_FLOAT, float, kml_fftf_complex);
KML_FFT_DEFINE_API(KML_FFT_MANGLE_FP16, __fp16, kml_ffth_complex);

#define KML_FFT_DEFINE_SCALE_API(X, FR, FC, R, C)                                        \
                                                                        \
KML_FFT_DEFINE_FLOAT_COMPLEX(FR, FC);                                              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft_scale)(int rank, const int *n,                     \
                                     int howmany,                                \
                                     C *in, const int *inembed,                  \
                                     int istride, int idist,                     \
                                     FC *out, const int *onembed,                 \
                                     int ostride, int odist,                     \
                                     int sign, unsigned flags);                  \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale)(int rank, const int *n,                          \
                                 C *in, FC *out, int sign, unsigned flags);        \
                                                                              \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_1d)(int n, C *in, FC *out, int sign,               \
                                   unsigned flags);                              \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_2d)(int n0, int n1,                               \
                                   C *in, FC *out, int sign, unsigned flags);     \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft_scale_r2c)(int rank, const int *n,                 \
                                         int howmany,                            \
                                         R *in, const int *inembed,              \
                                         int istride, int idist,                 \
                                         FC *out, const int *onembed,             \
                                         int ostride, int odist,                 \
                                         unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_r2c)(int rank, const int *n,                      \
                                    R *in, FC *out, unsigned flags);              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_r2c_1d)(int n, R *in, FC *out, unsigned flags);       \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_r2c_2d)(int n0, int n1,                           \
                                       R *in, FC *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_many_dft_scale_c2r)(int rank, const int *n,                 \
                                         int howmany,                            \
                                         C *in, const int *inembed,              \
                                         int istride, int idist,                 \
                                         FR *out, const int *onembed,             \
                                         int ostride, int odist,                 \
                                         unsigned flags);                        \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_c2r)(int rank, const int *n,                      \
                                    C *in, FR *out, unsigned flags);              \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_c2r_1d)(int n, C *in, FR *out, unsigned flags);       \
                                                                        \
KML_FFT_EXTERN X(plan)                                                     \
KML_FFT_CDECL X(plan_dft_scale_c2r_2d)(int n0, int n1,                           \
                                       C *in, FR *out, unsigned flags);           \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft_scale)(const X(plan) p, C *in, FC *out);              \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft_scale_r2c)(const X(plan) p, R *in, FC *out);          \
                                                                        \
KML_FFT_EXTERN void                                                        \
KML_FFT_CDECL X(execute_dft_scale_c2r)(const X(plan) p, C *in, FR *out);          \
                                                                        \
/* end of KML_FFT_DEFINE_SCALE_API macro */

KML_FFT_DEFINE_SCALE_API(KML_FFT_MANGLE_FP16, float, kml_ffth_float_complex, __fp16, kml_ffth_complex)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* __KML_FFT_H__ */
