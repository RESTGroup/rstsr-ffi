/*
 * @Copyright: Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 * @Description: Global API declaration of kml_fft_mpi, include double and float precision
 * @Author: KPL
 * @Date: 2023-01-30
*/

#ifndef __KML_FFT_MPI_H__
#define __KML_FFT_MPI_H__

#include <kfft.h>
#include <mpi.h>

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

struct kml_fft_mpi_ddim_do_not_use_me {
    ptrdiff_t n;                     /* dimension size */
    ptrdiff_t ib;                    /* input block */
    ptrdiff_t ob;                    /* output block */
};

enum SCALFFT_A2A_ALGO_E {
    A2A_ALGO_AUTO_TUNING = 0,
    A2A_ALGO_COLL_TASK_ALLTOALLV = 1,
    A2A_ALGO_P2P_TASK_LINEAR = 2,
    A2A_ALGO_P2P_TASK_PAIRWISE = 3,
    A2A_ALGO_P2P_PIPELINE_LINEAR = 4,
    A2A_ALGO_P2P_LAMMPS = 5,
    A2A_ALGO_MAX_IDX
};

enum SCALFFT_DECOMPOSE_TYPE_E {
    SCALFFT_DECOMPOSE_TYPE_SLAB = 0,
    SCALFFT_DECOMPOSE_TYPE_PENCIL = 1,
    SCALFFT_DECOMPOSE_TYPE_BRICK = 2
};

enum FFT_BACKEND_E {
    BACKEND_KFFT = 0
};

struct kml_fft_mpi_ext_options {
    enum SCALFFT_A2A_ALGO_E a2a_algo;
    enum SCALFFT_DECOMPOSE_TYPE_E decomp_type;
};

#define SCAL_MAX_DIMS 3

/*
  huge second-order macro that defines prototypes for all API
  functions.  We expand this macro for each supported precision

  XM: name-mangling macro (MPI)
  X: name-mangling macro (serial)
  R: real data type
  C: complex data type
*/

#define KML_FFT_MPI_DEFINE_API(XM, X, R, C)			\
								\
typedef struct kml_fft_mpi_ddim_do_not_use_me XM(ddim);		\
								\
KML_FFT_EXTERN void XM(init)(void);				\
KML_FFT_EXTERN void XM(cleanup)(void);				\
								\
KML_FFT_EXTERN ptrdiff_t XM(local_size_many_transposed)		\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t block0, ptrdiff_t block1, MPI_Comm comm,	\
      ptrdiff_t *local_n0, ptrdiff_t *local_0_start,		\
      ptrdiff_t *local_n1, ptrdiff_t *local_1_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_many)			\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t block0, MPI_Comm comm,				\
      ptrdiff_t *local_n0, ptrdiff_t *local_0_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_transposed)			\
    (int rnk, const ptrdiff_t *n, MPI_Comm comm,		\
      ptrdiff_t *local_n0, ptrdiff_t *local_0_start,		\
      ptrdiff_t *local_n1, ptrdiff_t *local_1_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size)				\
    (int rnk, const ptrdiff_t *n, MPI_Comm comm,		\
      ptrdiff_t *local_n0, ptrdiff_t *local_0_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_many_1d)(			\
     ptrdiff_t n0, ptrdiff_t howmany,				\
     MPI_Comm comm, int sign, unsigned flags,			\
     ptrdiff_t *local_ni, ptrdiff_t *local_i_start,		\
     ptrdiff_t *local_no, ptrdiff_t *local_o_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_1d)(			\
     ptrdiff_t n0, MPI_Comm comm, int sign, unsigned flags,	\
     ptrdiff_t *local_ni, ptrdiff_t *local_i_start,		\
     ptrdiff_t *local_no, ptrdiff_t *local_o_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_2d)(			\
     ptrdiff_t n0, ptrdiff_t n1, MPI_Comm comm,			\
     ptrdiff_t *local_n0, ptrdiff_t *local_0_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_2d_transposed)(		\
     ptrdiff_t n0, ptrdiff_t n1, MPI_Comm comm,			\
     ptrdiff_t *local_n0, ptrdiff_t *local_0_start,		\
     ptrdiff_t *local_n1, ptrdiff_t *local_1_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_3d)(			\
     ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, MPI_Comm comm,	\
     ptrdiff_t *local_n0, ptrdiff_t *local_0_start);		\
KML_FFT_EXTERN ptrdiff_t XM(local_size_3d_transposed)(		\
     ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, MPI_Comm comm,	\
     ptrdiff_t *local_n0, ptrdiff_t *local_0_start,		\
     ptrdiff_t *local_n1, ptrdiff_t *local_1_start);		\
								\
KML_FFT_EXTERN X(plan) XM(plan_many_transpose)			\
    (ptrdiff_t n0, ptrdiff_t n1,				\
      ptrdiff_t howmany, ptrdiff_t block0, ptrdiff_t block1,	\
      R *in, R *out, MPI_Comm comm, unsigned flags);		\
KML_FFT_EXTERN X(plan) XM(plan_transpose)				\
    (ptrdiff_t n0, ptrdiff_t n1,				\
      R *in, R *out, MPI_Comm comm, unsigned flags);		\
								\
KML_FFT_EXTERN X(plan) XM(plan_many_dft)				\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t block, ptrdiff_t tblock, C *in, C *out,		\
      MPI_Comm comm, int sign, unsigned flags);			\
KML_FFT_EXTERN X(plan) XM(plan_dft)				\
    (int rnk, const ptrdiff_t *n, C *in, C *out,		\
      MPI_Comm comm, int sign, unsigned flags);			\
KML_FFT_EXTERN X(plan) XM(plan_dft_1d)				\
    (ptrdiff_t n0, C *in, C *out,				\
      MPI_Comm comm, int sign, unsigned flags);			\
KML_FFT_EXTERN X(plan) XM(plan_dft_2d)				\
    (ptrdiff_t n0, ptrdiff_t n1, C *in, C *out,		\
      MPI_Comm comm, int sign, unsigned flags);			\
KML_FFT_EXTERN X(plan) XM(plan_dft_3d)				\
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, C *in, C *out,	\
      MPI_Comm comm, int sign, unsigned flags);			\
								\
KML_FFT_EXTERN X(plan) XM(plan_many_r2r)				\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t iblock, ptrdiff_t oblock, R *in, R *out,	\
      MPI_Comm comm, const X(r2r_kind) *kind, unsigned flags);	\
KML_FFT_EXTERN X(plan) XM(plan_r2r)				\
    (int rnk, const ptrdiff_t *n, R *in, R *out,		\
      MPI_Comm comm, const X(r2r_kind) *kind, unsigned flags);	\
KML_FFT_EXTERN X(plan) XM(plan_r2r_2d)				\
    (ptrdiff_t n0, ptrdiff_t n1, R *in, R *out, MPI_Comm comm,	\
      X(r2r_kind) kind0, X(r2r_kind) kind1, unsigned flags);	\
KML_FFT_EXTERN X(plan) XM(plan_r2r_3d)				\
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2,			\
      R *in, R *out, MPI_Comm comm, X(r2r_kind) kind0,		\
      X(r2r_kind) kind1, X(r2r_kind) kind2, unsigned flags);	\
								\
KML_FFT_EXTERN X(plan) XM(plan_many_dft_r2c)			\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t iblock, ptrdiff_t oblock, R *in, C *out,	\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_r2c)				\
    (int rnk, const ptrdiff_t *n, R *in, C *out,		\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_r2c_2d)				\
    (ptrdiff_t n0, ptrdiff_t n1, R *in, C *out,		\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_r2c_3d)				\
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, R *in, C *out,	\
      MPI_Comm comm, unsigned flags);				\
								\
KML_FFT_EXTERN X(plan) XM(plan_many_dft_c2r)			\
    (int rnk, const ptrdiff_t *n, ptrdiff_t howmany,		\
      ptrdiff_t iblock, ptrdiff_t oblock, C *in, R *out,	\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_c2r)				\
    (int rnk, const ptrdiff_t *n, C *in, R *out,		\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_c2r_2d)				\
    (ptrdiff_t n0, ptrdiff_t n1, C *in, R *out,		\
      MPI_Comm comm, unsigned flags);				\
KML_FFT_EXTERN X(plan) XM(plan_dft_c2r_3d)				\
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, C *in, R *out,	\
      MPI_Comm comm, unsigned flags);				\
								\
KML_FFT_EXTERN void XM(execute_dft)(X(plan) p, C *in, C *out);	\
KML_FFT_EXTERN void XM(execute_dft_r2c)(X(plan) p, R *in, C *out);	\
KML_FFT_EXTERN void XM(execute_dft_c2r)(X(plan) p, C *in, R *out);	\
KML_FFT_EXTERN void XM(execute_r2r)(X(plan) p, R *in, R *out); \
                                                                    \
typedef struct kml_fft_mpi_ext_options XM(options);                 \
                                                                    \
KML_FFT_EXTERN ptrdiff_t XM(local_size_ext)                         \
    (int rnk, const ptrdiff_t *n, MPI_Comm comm,                    \
     enum SCALFFT_DECOMPOSE_TYPE_E decomp_type,                        \
     ptrdiff_t *low, ptrdiff_t *high);                              \
KML_FFT_EXTERN ptrdiff_t XM(local_size_transposed_ext)              \
    (int rank, const ptrdiff_t *n, MPI_Comm comm,                   \
    enum SCALFFT_DECOMPOSE_TYPE_E decomp_type,                         \
    const int *order,                                               \
    ptrdiff_t *low, ptrdiff_t *high);                               \
KML_FFT_EXTERN ptrdiff_t XM(local_size_3d_ext)                      \
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, MPI_Comm comm,       \
     enum SCALFFT_DECOMPOSE_TYPE_E decomp_type,                        \
     ptrdiff_t *low, ptrdiff_t *high);                              \
KML_FFT_EXTERN ptrdiff_t XM(local_size_3d_transposed_ext)           \
    (ptrdiff_t n0, ptrdiff_t n1, ptrdiff_t n2, MPI_Comm comm,       \
     enum SCALFFT_DECOMPOSE_TYPE_E decomp_type,                        \
     const int *order,                                              \
     ptrdiff_t *low, ptrdiff_t *high);                              \
KML_FFT_EXTERN void XM(split_world)                                 \
    (const int *world,                                              \
     const int *proc_grid,                                          \
     int (*box_low)[SCAL_MAX_DIMS],                                 \
     int (*box_high)[SCAL_MAX_DIMS]);                               \
                                                                    \
KML_FFT_EXTERN X(plan) XM(plan_dft_3d_ext)                          \
    (int n0, int n1, int n2,                                        \
     const int *inbox_order, const int *outbox_order,               \
     MPI_Comm comm, const XM(options) options);                     \
KML_FFT_EXTERN X(plan) XM(plan_dft_r2c_3d_ext)                      \
    (int n0, int n1, int n2,                                        \
     const int *inbox_order, const int *outbox_order,               \
     MPI_Comm comm, const XM(options) options);                     \
KML_FFT_EXTERN X(plan) XM(plan_dft_c2r_3d_ext)                      \
    (int n0, int n1, int n2,                                        \
     const int *inbox_order, const int *outbox_order,               \
     MPI_Comm comm, const XM(options) options);                     \
KML_FFT_EXTERN X(plan) XM(plan_r2r_3d_ext)                          \
    (int n0, int n1, int n2,                                        \
     const int *inbox_order, const int *outbox_order,               \
     X(r2r_kind) kind0, X(r2r_kind) kind1, X(r2r_kind) kind2,       \
     MPI_Comm comm, const XM(options) options);                     \
KML_FFT_EXTERN X(plan) XM(plan_create)                              \
    (int backend,                                                   \
     const int *inbox_low,                                          \
     const int *inbox_high,                                         \
     const int *inbox_order,                                        \
     const int *outbox_low,                                         \
     const int *outbox_high,                                        \
     const int *outbox_order,                                       \
     MPI_Comm comm,                                                 \
     const XM(options) options);                                    \
KML_FFT_EXTERN X(plan) XM(plan_create_r2c)                          \
    (int backend,                                                   \
     const int *inbox_low,                                          \
     const int *inbox_high,                                         \
     const int *inbox_order,                                        \
     const int *outbox_low,                                         \
     const int *outbox_high,                                        \
     const int *outbox_order,                                       \
     MPI_Comm comm,                                                 \
     const XM(options) options);                                    \
KML_FFT_EXTERN X(plan) XM(plan_create_c2r)                          \
    (int backend,                                                   \
     const int *inbox_low,                                          \
     const int *inbox_high,                                         \
     const int *inbox_order,                                        \
     const int *outbox_low,                                         \
     const int *outbox_high,                                        \
     const int *outbox_order,                                       \
     MPI_Comm comm,                                                 \
     const XM(options) options);                                    \
KML_FFT_EXTERN X(plan) XM(plan_create_r2r)                          \
    (int backend,                                                   \
     const int *inbox_low,                                          \
     const int *inbox_high,                                         \
     const int *inbox_order,                                        \
     const int *outbox_low,                                         \
     const int *outbox_high,                                        \
     const int *outbox_order,                                       \
     const X(r2r_kind) *kind,                                       \
     MPI_Comm comm,                                                 \
     const XM(options) options);                                    \
                                                                    \
KML_FFT_EXTERN void XM(execute_dft_ext)                             \
    (X(plan) p, C *in, C *out, int scale, int sign_flag);           \
KML_FFT_EXTERN void XM(execute_dft_r2c_ext)                         \
    (X(plan) p, R *in, C *out, int scale);                          \
KML_FFT_EXTERN void XM(execute_dft_c2r_ext)                         \
    (X(plan) p, C *in, R *out, int scale);                          \
KML_FFT_EXTERN void XM(execute_r2r_ext)                             \
    (X(plan) p, R *in, R *out, int scale);                          \
KML_FFT_EXTERN void XM(forward_c2c)                                 \
    (X(plan) p, C *in, C *out, int scale);                          \
KML_FFT_EXTERN void XM(backward_c2c)                                \
    (X(plan) p, C *in, C *out, int scale);                          \
KML_FFT_EXTERN void XM(forward_r2c)                                 \
    (X(plan) p, R *in, C *out, int scale);                          \
KML_FFT_EXTERN void XM(backward_c2r)                                \
    (X(plan) p, C *in, R *out, int scale);

/* end of KML_FFT_MPI_DEFINE_API macro */

#define KML_FFT_MPI_MANGLE_DOUBLE(name) KML_FFT_MANGLE_DOUBLE(KML_FFT_CONCAT(mpi_, name))
#define KML_FFT_MPI_MANGLE_FLOAT(name) KML_FFT_MANGLE_FLOAT(KML_FFT_CONCAT(mpi_, name))
#define KML_FFT_MPI_MANGLE_FP16(name) KML_FFT_MANGLE_FP16(KML_FFT_CONCAT(mpi_, name))

KML_FFT_MPI_DEFINE_API(KML_FFT_MPI_MANGLE_DOUBLE, KML_FFT_MANGLE_DOUBLE, double, kml_fft_complex)
KML_FFT_MPI_DEFINE_API(KML_FFT_MPI_MANGLE_FLOAT, KML_FFT_MANGLE_FLOAT, float, kml_fftf_complex)
KML_FFT_MPI_DEFINE_API(KML_FFT_MPI_MANGLE_FP16, KML_FFT_MANGLE_FP16, __fp16, kml_ffth_complex)

#define KML_FFT_MPI_DEFAULT_BLOCK (0)

/* MPI-specific flags */
#define KML_FFT_MPI_SCRAMBLED_IN (1U << 27)
#define KML_FFT_MPI_SCRAMBLED_OUT (1U << 28)
#define KML_FFT_MPI_TRANSPOSED_IN (1U << 29)
#define KML_FFT_MPI_TRANSPOSED_OUT (1U << 30)

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* __KML_FFT_MPI_H__ */
