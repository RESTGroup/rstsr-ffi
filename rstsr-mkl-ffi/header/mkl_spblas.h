/*******************************************************************************
* Copyright (C) 2005 Intel Corporation
*
* This software and the related documents are Intel copyrighted  materials,  and
* your use of  them is  governed by the  express license  under which  they were
* provided to you (License).  Unless the License provides otherwise, you may not
* use, modify, copy, publish, distribute,  disclose or transmit this software or
* the related documents without Intel's prior written permission.
*
* This software and the related documents  are provided as  is,  with no express
* or implied  warranties,  other  than those  that are  expressly stated  in the
* License.
*******************************************************************************/

/*
! Content:
!  Intel(R) oneAPI Math Kernel Library (oneMKL) interface for Sparse BLAS
!  level 2,3 routines
!
!******************************************************************************/

#ifndef _MKL_SPBLAS_H_
#define _MKL_SPBLAS_H_

#include "mkl_types.h"

#undef MKL_DEPRECATED
#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L /* C23 */
  #if defined(__has_c_attribute)
    #if __has_c_attribute(deprecated)
      #define MKL_DEPRECATED  [[deprecated]]
    #endif
  #endif
#elif defined(__cplusplus) && __cplusplus >= 201402L /* C++14 */
  #if defined(__has_cpp_attribute)
    #if __has_cpp_attribute(deprecated)
      #define MKL_DEPRECATED  [[deprecated]]
    #endif
  #endif
#endif

#ifndef MKL_DEPRECATED
  #if defined (__GNUC__) || defined(__clang__)
    #define MKL_DEPRECATED __attribute__((deprecated))
  #elif defined(_MSC_VER)
    #define MKL_DEPRECATED __declspec(deprecated)
  #else
    #pragma message("WARNING: Intel oneMKL NIST like Level 2 and 3 SpBLAS APIs have been declared deprecated. Use Intel oneMKL Inspector-Executor SpBLAS instead.")
    #define MKL_DEPRECATED
  #endif
#endif

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/*Converters lower case*/
MKL_DEPRECATED void mkl_ddnscsr(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, double *Adns, const MKL_INT *lda, double *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void mkl_sdnscsr(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, float *Adns, const MKL_INT *lda, float *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void mkl_cdnscsr(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, MKL_Complex8 *Adns, const MKL_INT *lda, MKL_Complex8 *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void mkl_zdnscsr(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, MKL_Complex16 *Adns, const MKL_INT *lda, MKL_Complex16 *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);

/*Converters upper case*/
MKL_DEPRECATED void MKL_DDNSCSR(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, double *Adns, const MKL_INT *lda, double *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void MKL_SDNSCSR(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, float *Adns, const MKL_INT *lda, float *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void MKL_CDNSCSR(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, MKL_Complex8 *Adns, const MKL_INT *lda, MKL_Complex8 *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);
MKL_DEPRECATED void MKL_ZDNSCSR(const MKL_INT *job, const MKL_INT *m, const MKL_INT *n, MKL_Complex16 *Adns, const MKL_INT *lda, MKL_Complex16 *Acsr, MKL_INT *AJ, MKL_INT *AI, MKL_INT *info);


/*Sparse BLAS Level2 (CSR-CSR or CSR-DNS) lower case */
MKL_DEPRECATED void mkl_dcsrmultcsr(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, double *a,  MKL_INT *ja, MKL_INT *ia, double *b, MKL_INT *jb, MKL_INT *ib,  double *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void mkl_dcsradd(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, double *a, MKL_INT *ja, MKL_INT *ia,  const double *beta, double *b, MKL_INT *jb, MKL_INT *ib,  double *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void mkl_scsrmultcsr(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, float *a, MKL_INT *ja, MKL_INT *ia, float *b, MKL_INT *jb, MKL_INT *ib,  float *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void mkl_scsradd(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, float *a, MKL_INT *ja, MKL_INT *ia,  const float *beta, float *b, MKL_INT *jb, MKL_INT *ib, float *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void mkl_ccsrmultcsr(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, const MKL_INT *k, MKL_Complex8 *a, MKL_INT *ja, MKL_INT *ia, MKL_Complex8 *b, MKL_INT *jb, MKL_INT *ib, MKL_Complex8 *c, MKL_INT *jc, MKL_INT *ic, const MKL_INT *nnzmax, MKL_INT *ierr);
MKL_DEPRECATED void mkl_ccsradd(const char *transa,  const MKL_INT *job, const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, MKL_Complex8 *a, MKL_INT *ja, MKL_INT *ia, const MKL_Complex8 *beta, MKL_Complex8 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex8 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void mkl_zcsrmultcsr(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, MKL_Complex16 *a, MKL_INT *ja,  MKL_INT *ia, MKL_Complex16 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex16 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void mkl_zcsradd(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, MKL_Complex16 *a, MKL_INT *ja, MKL_INT *ia,  const MKL_Complex16 *beta, MKL_Complex16 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex16 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);


/*Sparse BLAS Level2 (CSR-CSR or CSR-DNS) upper case */
MKL_DEPRECATED void MKL_DCSRMULTCSR(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, double *a, MKL_INT *ja, MKL_INT *ia, double *b, MKL_INT *jb, MKL_INT *ib,  double *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void MKL_DCSRADD(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, double *a, MKL_INT *ja, MKL_INT *ia, const double *beta, double *b, MKL_INT *jb, MKL_INT *ib,  double *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void MKL_SCSRMULTCSR(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, float *a, MKL_INT *ja, MKL_INT *ia, float *b, MKL_INT *jb, MKL_INT *ib,  float *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void MKL_SCSRADD(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, float *a, MKL_INT *ja, MKL_INT *ia,  const float *beta, float *b, MKL_INT *jb, MKL_INT *ib,  float *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void MKL_CCSRMULTCSR(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, MKL_Complex8 *a, MKL_INT *ja, MKL_INT *ia, MKL_Complex8 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex8 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void MKL_CCSRADD(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, MKL_Complex8 *a, MKL_INT *ja, MKL_INT *ia,  const MKL_Complex8 *beta, MKL_Complex8 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex8 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);

MKL_DEPRECATED void MKL_ZCSRMULTCSR(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n,  const MKL_INT *k, MKL_Complex16 *a, MKL_INT *ja, MKL_INT *ia, MKL_Complex16 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex16 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);
MKL_DEPRECATED void MKL_ZCSRADD(const char *transa,  const MKL_INT *job,  const MKL_INT *sort,  const MKL_INT *m,  const MKL_INT *n, MKL_Complex16 *a, MKL_INT *ja, MKL_INT *ia,  const MKL_Complex16 *beta, MKL_Complex16 *b, MKL_INT *jb, MKL_INT *ib,  MKL_Complex16 *c,  MKL_INT *jc,  MKL_INT *ic,  const MKL_INT *nnzmax,  MKL_INT *ierr);




/*****************************************************************************************/
/************** Basic types and constants for inspector-executor SpBLAS API **************/
/*****************************************************************************************/

    /* status of the routines */
    typedef enum
    {
        SPARSE_STATUS_SUCCESS           = 0,    /* the operation was successful */
        SPARSE_STATUS_NOT_INITIALIZED   = 1,    /* empty handle or matrix arrays */
        SPARSE_STATUS_ALLOC_FAILED      = 2,    /* internal error: memory allocation failed */
        SPARSE_STATUS_INVALID_VALUE     = 3,    /* invalid input value */
        SPARSE_STATUS_EXECUTION_FAILED  = 4,    /* e.g. 0-diagonal element for triangular solver, etc. */
        SPARSE_STATUS_INTERNAL_ERROR    = 5,    /* internal error */
        SPARSE_STATUS_NOT_SUPPORTED     = 6     /* e.g. operation for double precision doesn't support other types */
    } sparse_status_t;

    /* sparse matrix operations */
    typedef enum
    {
        SPARSE_OPERATION_NON_TRANSPOSE       = 10,
        SPARSE_OPERATION_TRANSPOSE           = 11,
        SPARSE_OPERATION_CONJUGATE_TRANSPOSE = 12
    } sparse_operation_t;

    /* supported matrix types */
    typedef enum
    {
        SPARSE_MATRIX_TYPE_GENERAL            = 20,   /*    General case                    */
        SPARSE_MATRIX_TYPE_SYMMETRIC          = 21,   /*    Triangular part of              */
        SPARSE_MATRIX_TYPE_HERMITIAN          = 22,   /*    the matrix is to be processed   */
        SPARSE_MATRIX_TYPE_TRIANGULAR         = 23,
        SPARSE_MATRIX_TYPE_DIAGONAL           = 24,   /* diagonal matrix; only diagonal elements will be processed */
        SPARSE_MATRIX_TYPE_BLOCK_TRIANGULAR   = 25,
        SPARSE_MATRIX_TYPE_BLOCK_DIAGONAL     = 26    /* block-diagonal matrix; only diagonal blocks will be processed */
    } sparse_matrix_type_t;

    /* sparse matrix indexing: C-style or Fortran-style */
    typedef enum
    {
        SPARSE_INDEX_BASE_ZERO  = 0,           /* C-style */
        SPARSE_INDEX_BASE_ONE   = 1            /* Fortran-style */
    } sparse_index_base_t;

    /* applies to triangular matrices only ( SPARSE_MATRIX_TYPE_SYMMETRIC, SPARSE_MATRIX_TYPE_HERMITIAN, SPARSE_MATRIX_TYPE_TRIANGULAR ) */
    typedef enum
    {
        SPARSE_FILL_MODE_LOWER  = 40,           /* lower triangular part of the matrix is stored */
        SPARSE_FILL_MODE_UPPER  = 41,            /* upper triangular part of the matrix is stored */
        SPARSE_FILL_MODE_FULL   = 42            /* upper triangular part of the matrix is stored */
    } sparse_fill_mode_t;

    /* applies to triangular matrices only ( SPARSE_MATRIX_TYPE_SYMMETRIC, SPARSE_MATRIX_TYPE_HERMITIAN, SPARSE_MATRIX_TYPE_TRIANGULAR ) */
    typedef enum
    {
        SPARSE_DIAG_NON_UNIT    = 50,           /* triangular matrix with non-unit diagonal */
        SPARSE_DIAG_UNIT        = 51            /* triangular matrix with unit diagonal */
    } sparse_diag_type_t;

    /* applicable for Level 3 operations with dense matrices; describes storage scheme for dense matrix (row major or column major) */
    typedef enum
    {
        SPARSE_LAYOUT_ROW_MAJOR    = 101,       /* C-style */
        SPARSE_LAYOUT_COLUMN_MAJOR = 102        /* Fortran-style */
    } sparse_layout_t;

    /* verbose mode; if verbose mode activated, handle should collect and report profiling / optimization info */
    typedef enum
    {
        SPARSE_VERBOSE_OFF      = 70,
        SPARSE_VERBOSE_BASIC    = 71,           /* output contains high-level information about optimization algorithms, issues, etc. */
        SPARSE_VERBOSE_EXTENDED = 72            /* provide detailed output information */
    } verbose_mode_t;

    /* memory optimization hints from user: describe how much memory could be used on optimization stage */
    typedef enum
    {
        SPARSE_MEMORY_NONE          = 80,       /* no memory should be allocated for matrix values and structures; auxiliary structures could be created only for workload balancing, parallelization, etc. */
        SPARSE_MEMORY_AGGRESSIVE    = 81        /* matrix could be converted to any internal format */
    } sparse_memory_usage_t;

    typedef enum
    {
        SPARSE_STAGE_FULL_MULT            = 90,
        SPARSE_STAGE_NNZ_COUNT            = 91,
        SPARSE_STAGE_FINALIZE_MULT        = 92,
        SPARSE_STAGE_FULL_MULT_NO_VAL     = 93,
        SPARSE_STAGE_FINALIZE_MULT_NO_VAL = 94
    } sparse_request_t;

    /* applies to SOR interface; define type of (S)SOR operation to perform */
    typedef enum
    {
        SPARSE_SOR_FORWARD   = 110, /* (omega∗L + D)∗x^1 = (D - omega*D - omega*U)∗alpha*x^0 + omega*b */
        SPARSE_SOR_BACKWARD  = 111, /* (omega∗U + D)∗x^1 = (D - omega*D - omega*L)∗alpha*x^0 + omega*b */
        SPARSE_SOR_SYMMETRIC = 112  /* SSOR, for e.g. with omega == 1 && alpha == 1, equal to solving a system:
                                       (L + D)∗x^1 = b - U*x; (U + D)∗x = b - L*x^1 */
    } sparse_sor_type_t;

/*************************************************************************************************/
/*** Opaque structure for sparse matrix in internal format, further D - means double precision ***/
/*************************************************************************************************/

    struct  sparse_matrix;
    typedef struct sparse_matrix *sparse_matrix_t;

    /* descriptor of main sparse matrix properties */
    struct matrix_descr {
        sparse_matrix_type_t  type;       /* matrix type: general, diagonal or triangular / symmetric / hermitian */
        sparse_fill_mode_t    mode;       /* upper or lower triangular part of the matrix ( for triangular / symmetric / hermitian case) */
        sparse_diag_type_t    diag;       /* unit or non-unit diagonal ( for triangular / symmetric / hermitian case) */
    };

/*****************************************************************************************/
/*************************************** Creation routines *******************************/
/*****************************************************************************************/

/*
    Matrix handle is used for storing information about the matrix and matrix values

    Create matrix from one of the existing sparse formats by creating the handle with matrix info and copy matrix values if requested.
    Collect high-level info about the matrix. Need to use this interface for the case with several calls in program for performance reasons,
    where optimizations are not required.

    coordinate format,
    SPARSE_MATRIX_TYPE_GENERAL by default, pointers to input arrays are stored in the handle

    *** User data is not marked const since the mkl_sparse_order() or mkl_sparse_?_set_values()
    functionality could change user data.  However, this is only done by a user call.
    Internally const-ness of user data is maintained other than through explicit
    use of these interfaces.

*/
    sparse_status_t mkl_sparse_s_create_coo(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             nnz,
                                                   MKL_INT             *row_indx,
                                                   MKL_INT             *col_indx,
                                                   float               *values );

    sparse_status_t mkl_sparse_d_create_coo(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             nnz,
                                                   MKL_INT             *row_indx,
                                                   MKL_INT             *col_indx,
                                                   double              *values );

    sparse_status_t mkl_sparse_c_create_coo(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             nnz,
                                                   MKL_INT             *row_indx,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_coo(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             nnz,
                                                   MKL_INT             *row_indx,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex16       *values );

    sparse_status_t mkl_sparse_s_create_coo_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           nnz,
                                                      MKL_INT64           *row_indx,
                                                      MKL_INT64           *col_indx,
                                                      float               *values );

    sparse_status_t mkl_sparse_d_create_coo_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           nnz,
                                                      MKL_INT64           *row_indx,
                                                      MKL_INT64           *col_indx,
                                                      double              *values );

    sparse_status_t mkl_sparse_c_create_coo_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           nnz,
                                                      MKL_INT64           *row_indx,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_coo_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           nnz,
                                                      MKL_INT64           *row_indx,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex16       *values );


/*
    compressed sparse row format (4-arrays version),
    SPARSE_MATRIX_TYPE_GENERAL by default, pointers to input arrays are stored in the handle

    *** User data is not marked const since the mkl_sparse_order() or mkl_sparse_?_set_values()
    functionality could change user data.  However, this is only done by a user call.
    Internally const-ness of user data is maintained other than through explicit
    use of these interfaces.

*/
    sparse_status_t mkl_sparse_s_create_csr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   float               *values );

    sparse_status_t mkl_sparse_d_create_csr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   double              *values );

    sparse_status_t mkl_sparse_c_create_csr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_csr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex16       *values );

    sparse_status_t mkl_sparse_s_create_csr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      float               *values );

    sparse_status_t mkl_sparse_d_create_csr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      double              *values );

    sparse_status_t mkl_sparse_c_create_csr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_csr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex16       *values );

/*
    compressed sparse column format (4-arrays version),
    SPARSE_MATRIX_TYPE_GENERAL by default, pointers to input arrays are stored in the handle

    *** User data is not marked const since the mkl_sparse_order() or mkl_sparse_?_set_values()
    functionality could change user data.  However, this is only done by a user call.
    Internally const-ness of user data is maintained other than through explicit
    use of these interfaces.

*/
    sparse_status_t mkl_sparse_s_create_csc(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *cols_start,
                                                   MKL_INT             *cols_end,
                                                   MKL_INT             *row_indx,
                                                   float               *values );

    sparse_status_t mkl_sparse_d_create_csc(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *cols_start,
                                                   MKL_INT             *cols_end,
                                                   MKL_INT             *row_indx,
                                                   double              *values );

    sparse_status_t mkl_sparse_c_create_csc( sparse_matrix_t           *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *cols_start,
                                                   MKL_INT             *cols_end,
                                                   MKL_INT             *row_indx,
                                                   MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_csc( sparse_matrix_t           *A,
                                             const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                                   MKL_INT             *cols_start,
                                                   MKL_INT             *cols_end,
                                                   MKL_INT             *row_indx,
                                                   MKL_Complex16       *values );

    sparse_status_t mkl_sparse_s_create_csc_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *cols_start,
                                                      MKL_INT64           *cols_end,
                                                      MKL_INT64           *row_indx,
                                                      float               *values );

    sparse_status_t mkl_sparse_d_create_csc_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *cols_start,
                                                      MKL_INT64           *cols_end,
                                                      MKL_INT64           *row_indx,
                                                      double              *values );

    sparse_status_t mkl_sparse_c_create_csc_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *cols_start,
                                                      MKL_INT64           *cols_end,
                                                      MKL_INT64           *row_indx,
                                                      MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_csc_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing, /* indexing: C-style or Fortran-style */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                      MKL_INT64           *cols_start,
                                                      MKL_INT64           *cols_end,
                                                      MKL_INT64           *row_indx,
                                                      MKL_Complex16       *values );

/*
    compressed block sparse row format (4-arrays version, square blocks),
    SPARSE_MATRIX_TYPE_GENERAL by default, pointers to input arrays are stored in the handle

    *** User data is not marked const since the mkl_sparse_order() or mkl_sparse_?_set_values()
    functionality could change user data.  However, this is only done by a user call.
    Internally const-ness of user data is maintained other than through explicit
    use of these interfaces.

*/
    sparse_status_t mkl_sparse_s_create_bsr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                             const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             block_size,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   float               *values );

    sparse_status_t mkl_sparse_d_create_bsr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                             const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             block_size,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   double              *values );

    sparse_status_t mkl_sparse_c_create_bsr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                             const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             block_size,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_bsr(       sparse_matrix_t     *A,
                                             const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                             const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                             const MKL_INT             rows,
                                             const MKL_INT             cols,
                                             const MKL_INT             block_size,
                                                   MKL_INT             *rows_start,
                                                   MKL_INT             *rows_end,
                                                   MKL_INT             *col_indx,
                                                   MKL_Complex16       *values );

    sparse_status_t mkl_sparse_s_create_bsr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                                const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           block_size,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      float               *values );

    sparse_status_t mkl_sparse_d_create_bsr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                                const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           block_size,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      double              *values );

    sparse_status_t mkl_sparse_c_create_bsr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                                const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           block_size,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex8        *values );

    sparse_status_t mkl_sparse_z_create_bsr_64(       sparse_matrix_t     *A,
                                                const sparse_index_base_t indexing,       /* indexing: C-style or Fortran-style */
                                                const sparse_layout_t     block_layout,   /* block storage: row-major or column-major */
                                                const MKL_INT64           rows,
                                                const MKL_INT64           cols,
                                                const MKL_INT64           block_size,
                                                      MKL_INT64           *rows_start,
                                                      MKL_INT64           *rows_end,
                                                      MKL_INT64           *col_indx,
                                                      MKL_Complex16       *values );


/*
    Create copy of the existing handle; matrix properties could be changed.
    For example it could be used for extracting triangular or diagonal parts from existing matrix.
*/
    sparse_status_t mkl_sparse_copy( const sparse_matrix_t     source,
                                     const struct matrix_descr descr,        /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                     sparse_matrix_t           *dest );

    sparse_status_t mkl_sparse_copy_64( const sparse_matrix_t     source,
                                        const struct matrix_descr descr,        /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        sparse_matrix_t           *dest );


/*
    destroy matrix handle; if sparse matrix was stored inside the handle it also deallocates the matrix
    It is user's responsibility not to delete the handle with the matrix, if this matrix is shared with other handles
*/
    sparse_status_t mkl_sparse_destroy( sparse_matrix_t  A );

    sparse_status_t mkl_sparse_destroy_64( sparse_matrix_t  A );
/*
    return extended error information from last operation;
    e.g. info about wrong input parameter, memory sizes that couldn't be allocated
*/
    sparse_status_t mkl_sparse_get_error_info( sparse_matrix_t  A, MKL_INT *info ); /* unsupported currently */

    sparse_status_t mkl_sparse_get_error_info_64( sparse_matrix_t  A, MKL_INT64 *info ); /* unsupported currently */

/*****************************************************************************************/
/************************ Converters of internal representation  *************************/
/*****************************************************************************************/

    /* converters from current format to another */
    sparse_status_t mkl_sparse_convert_csr( const sparse_matrix_t    source,         /* convert original matrix to CSR representation */
                                            const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                            sparse_matrix_t          *dest );

    sparse_status_t mkl_sparse_convert_bsr( const sparse_matrix_t    source,         /* convert original matrix to BSR representation */
                                            const MKL_INT            block_size,
                                            const sparse_layout_t    block_layout,   /* block storage: row-major or column-major */
                                            const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                            sparse_matrix_t          *dest );

    sparse_status_t mkl_sparse_convert_csc( const sparse_matrix_t    source,         /* convert original matrix to CSC representation */
                                            const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                            sparse_matrix_t          *dest );

    sparse_status_t mkl_sparse_convert_coo( const sparse_matrix_t    source,         /* convert original matrix to COO representation */
                                            const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                            sparse_matrix_t          *dest );


    sparse_status_t mkl_sparse_convert_csr_64( const sparse_matrix_t    source,         /* convert original matrix to CSR representation */
                                               const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                               sparse_matrix_t          *dest );

    sparse_status_t mkl_sparse_convert_bsr_64( const sparse_matrix_t    source,         /* convert original matrix to BSR representation */
                                               const MKL_INT64          block_size,
                                               const sparse_layout_t    block_layout,   /* block storage: row-major or column-major */
                                               const sparse_operation_t operation,      /* as is, transposed or conjugate transposed */
                                               sparse_matrix_t          *dest );

    sparse_status_t mkl_sparse_convert_csc_64( const sparse_matrix_t source,       /* convert original matrix to CSC representation */
                                               const sparse_operation_t operation, /* as is, transposed or conjugate transposed */
                                               sparse_matrix_t *dest);

    sparse_status_t mkl_sparse_convert_coo_64( const sparse_matrix_t source,       /* convert original matrix to COO representation */
                                               const sparse_operation_t operation, /* as is, transposed or conjugate transposed */
                                               sparse_matrix_t *dest);

    /* converters from current sparse format to dense */
    sparse_status_t mkl_sparse_s_convert_dense( const sparse_matrix_t source,
                                                const struct matrix_descr descr,
                                                const sparse_layout_t dense_layout,
                                                const MKL_INT lda,
                                                float *adns );

    sparse_status_t mkl_sparse_d_convert_dense( const sparse_matrix_t source,
                                                const struct matrix_descr descr,
                                                const sparse_layout_t dense_layout,
                                                const MKL_INT lda,
                                                double *adns );

    sparse_status_t mkl_sparse_c_convert_dense( const sparse_matrix_t source,
                                                const struct matrix_descr descr,
                                                const sparse_layout_t dense_layout,
                                                const MKL_INT lda,
                                                MKL_Complex8 *adns );

    sparse_status_t mkl_sparse_z_convert_dense( const sparse_matrix_t source,
                                                const struct matrix_descr descr,
                                                const sparse_layout_t dense_layout,
                                                const MKL_INT lda,
                                                MKL_Complex16 *adns );

    sparse_status_t mkl_sparse_s_convert_dense_64( const sparse_matrix_t source,
                                                   const struct matrix_descr descr,
                                                   const sparse_layout_t dense_layout,
                                                   const MKL_INT64 lda,
                                                   float *adns );

    sparse_status_t mkl_sparse_d_convert_dense_64( const sparse_matrix_t source,
                                                   const struct matrix_descr descr,
                                                   const sparse_layout_t dense_layout,
                                                   const MKL_INT64 lda,
                                                   double *adns );

    sparse_status_t mkl_sparse_c_convert_dense_64( const sparse_matrix_t source,
                                                   const struct matrix_descr descr,
                                                   const sparse_layout_t dense_layout,
                                                   const MKL_INT64 lda,
                                                   MKL_Complex8 *adns );

    sparse_status_t mkl_sparse_z_convert_dense_64( const sparse_matrix_t source,
                                                   const struct matrix_descr descr,
                                                   const sparse_layout_t dense_layout,
                                                   const MKL_INT64 lda,
                                                   MKL_Complex16 *adns );

    /* converters from dense format to given sparse format */
    sparse_status_t mkl_sparse_s_dense2csr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const float *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2csr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const double *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2csr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex8 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2csr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex16 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2csr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const float *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2csr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const double *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2csr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex8 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2csr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex16 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2csc( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const float *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2csc( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const double *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2csc( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex8 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2csc( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex16 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2csc_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const float *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2csc_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const double *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2csc_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex8 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2csc_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex16 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2coo( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const float *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2coo( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const double *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2coo( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex8 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2coo( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex16 *adns,
                                            const sparse_index_base_t indexing,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2coo_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const float *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2coo_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const double *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2coo_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex8 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2coo_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex16 *adns,
                                               const sparse_index_base_t indexing,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2bsr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const float *adns,
                                            const sparse_index_base_t indexing,
                                            const MKL_INT block_size,
                                            const sparse_layout_t block_layout,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2bsr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const double *adns,
                                            const sparse_index_base_t indexing,
                                            const MKL_INT block_size,
                                            const sparse_layout_t block_layout,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2bsr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex8 *adns,
                                            const sparse_index_base_t indexing,
                                            const MKL_INT block_size,
                                            const sparse_layout_t block_layout,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2bsr( const MKL_INT rows,
                                            const MKL_INT cols,
                                            const sparse_layout_t dense_layout,
                                            const MKL_INT lda,
                                            const MKL_Complex16 *adns,
                                            const sparse_index_base_t indexing,
                                            const MKL_INT block_size,
                                            const sparse_layout_t block_layout,
                                            const struct matrix_descr descr,
                                            sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_s_dense2bsr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const float *adns,
                                               const sparse_index_base_t indexing,
                                               const MKL_INT64 block_size,
                                               const sparse_layout_t block_layout,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_d_dense2bsr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const double *adns,
                                               const sparse_index_base_t indexing,
                                               const MKL_INT64 block_size,
                                               const sparse_layout_t block_layout,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_c_dense2bsr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex8 *adns,
                                               const sparse_index_base_t indexing,
                                               const MKL_INT64 block_size,
                                               const sparse_layout_t block_layout,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

    sparse_status_t mkl_sparse_z_dense2bsr_64( const MKL_INT64 rows,
                                               const MKL_INT64 cols,
                                               const sparse_layout_t dense_layout,
                                               const MKL_INT64 lda,
                                               const MKL_Complex16 *adns,
                                               const sparse_index_base_t indexing,
                                               const MKL_INT64 block_size,
                                               const sparse_layout_t block_layout,
                                               const struct matrix_descr descr,
                                               sparse_matrix_t *dest );

/**********************************************************************************/
/************************ Export internal representation  *************************/
/**********************************************************************************/
    sparse_status_t mkl_sparse_s_export_bsr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                *block_size,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             float                  **values );

    sparse_status_t mkl_sparse_d_export_bsr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                *block_size,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             double                 **values );

    sparse_status_t mkl_sparse_c_export_bsr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                *block_size,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_bsr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                *block_size,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             MKL_Complex16          **values );

    sparse_status_t mkl_sparse_s_export_bsr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              *block_size,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                float                  **values );

    sparse_status_t mkl_sparse_d_export_bsr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              *block_size,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                double                 **values );

    sparse_status_t mkl_sparse_c_export_bsr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              *block_size,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_bsr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                sparse_layout_t        *block_layout,  /* block storage: row-major or column-major */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              *block_size,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                MKL_Complex16          **values );


    sparse_status_t mkl_sparse_s_export_csr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             float                  **values );

    sparse_status_t mkl_sparse_d_export_csr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             double                 **values );

    sparse_status_t mkl_sparse_c_export_csr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_csr( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **rows_start,
                                             MKL_INT                **rows_end,
                                             MKL_INT                **col_indx,
                                             MKL_Complex16          **values );

    sparse_status_t mkl_sparse_s_export_csr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                float                  **values );

    sparse_status_t mkl_sparse_d_export_csr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                double                 **values );

    sparse_status_t mkl_sparse_c_export_csr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_csr_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **rows_start,
                                                MKL_INT64              **rows_end,
                                                MKL_INT64              **col_indx,
                                                MKL_Complex16          **values );


    sparse_status_t mkl_sparse_s_export_csc( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **cols_start,
                                             MKL_INT                **cols_end,
                                             MKL_INT                **row_indx,
                                             float                  **values );

    sparse_status_t mkl_sparse_d_export_csc( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **cols_start,
                                             MKL_INT                **cols_end,
                                             MKL_INT                **row_indx,
                                             double                 **values );

    sparse_status_t mkl_sparse_c_export_csc( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **cols_start,
                                             MKL_INT                **cols_end,
                                             MKL_INT                **row_indx,
                                             MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_csc( const sparse_matrix_t  source,
                                             sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT                *rows,
                                             MKL_INT                *cols,
                                             MKL_INT                **cols_start,
                                             MKL_INT                **cols_end,
                                             MKL_INT                **row_indx,
                                             MKL_Complex16          **values );

    sparse_status_t mkl_sparse_s_export_csc_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **cols_start,
                                                MKL_INT64              **cols_end,
                                                MKL_INT64              **row_indx,
                                                float                  **values );

    sparse_status_t mkl_sparse_d_export_csc_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **cols_start,
                                                MKL_INT64              **cols_end,
                                                MKL_INT64              **row_indx,
                                                double                 **values );

    sparse_status_t mkl_sparse_c_export_csc_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **cols_start,
                                                MKL_INT64              **cols_end,
                                                MKL_INT64              **row_indx,
                                                MKL_Complex8           **values );

    sparse_status_t mkl_sparse_z_export_csc_64( const sparse_matrix_t  source,
                                                sparse_index_base_t    *indexing,      /* indexing: C-style or Fortran-style */
                                                MKL_INT64              *rows,
                                                MKL_INT64              *cols,
                                                MKL_INT64              **cols_start,
                                                MKL_INT64              **cols_end,
                                                MKL_INT64              **row_indx,
                                                MKL_Complex16          **values );

    sparse_status_t mkl_sparse_s_export_coo( const sparse_matrix_t source,
                                             sparse_index_base_t *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT *rows,
                                             MKL_INT *cols,
                                             MKL_INT *nnz,
                                             MKL_INT **rows_indx,
                                             MKL_INT **cols_indx,
                                             float **values);

    sparse_status_t mkl_sparse_d_export_coo( const sparse_matrix_t source,
                                             sparse_index_base_t *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT *rows,
                                             MKL_INT *cols,
                                             MKL_INT *nnz,
                                             MKL_INT **rows_indx,
                                             MKL_INT **cols_indx,
                                             double **values);

    sparse_status_t mkl_sparse_c_export_coo( const sparse_matrix_t source,
                                             sparse_index_base_t *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT *rows,
                                             MKL_INT *cols,
                                             MKL_INT *nnz,
                                             MKL_INT **rows_indx,
                                             MKL_INT **cols_indx,
                                             MKL_Complex8 **values);

    sparse_status_t mkl_sparse_z_export_coo( const sparse_matrix_t source,
                                             sparse_index_base_t *indexing,      /* indexing: C-style or Fortran-style */
                                             MKL_INT *rows,
                                             MKL_INT *cols,
                                             MKL_INT *nnz,
                                             MKL_INT **rows_indx,
                                             MKL_INT **cols_indx,
                                             MKL_Complex16 **values);

    sparse_status_t mkl_sparse_s_export_coo_64( const sparse_matrix_t source,
                                                sparse_index_base_t *indexing, /* indexing: C-style or Fortran-style */
                                                MKL_INT64 *rows,
                                                MKL_INT64 *cols,
                                                MKL_INT64 *nnz,
                                                MKL_INT64 **rows_indx,
                                                MKL_INT64 **cols_indx,
                                                float **values);

    sparse_status_t mkl_sparse_d_export_coo_64( const sparse_matrix_t source,
                                                sparse_index_base_t *indexing, /* indexing: C-style or Fortran-style */
                                                MKL_INT64 *rows,
                                                MKL_INT64 *cols,
                                                MKL_INT64 *nnz,
                                                MKL_INT64 **rows_indx,
                                                MKL_INT64 **cols_indx,
                                                double **values);

    sparse_status_t mkl_sparse_c_export_coo_64( const sparse_matrix_t source,
                                                sparse_index_base_t *indexing, /* indexing: C-style or Fortran-style */
                                                MKL_INT64 *rows,
                                                MKL_INT64 *cols,
                                                MKL_INT64 *nnz,
                                                MKL_INT64 **rows_indx,
                                                MKL_INT64 **cols_indx,
                                                MKL_Complex8 **values);

    sparse_status_t mkl_sparse_z_export_coo_64( const sparse_matrix_t source,
                                                sparse_index_base_t *indexing, /* indexing: C-style or Fortran-style */
                                                MKL_INT64 *rows,
                                                MKL_INT64 *cols,
                                                MKL_INT64 *nnz,
                                                MKL_INT64 **rows_indx,
                                                MKL_INT64 **cols_indx,
                                                MKL_Complex16 **values);

    /*****************************************************************************************/
    /************************** Step-by-step modification routines ***************************/
    /*****************************************************************************************/


    /* update existing value in the matrix ( for internal storage only, should not work with user-allocated matrices) */
    sparse_status_t mkl_sparse_s_set_value( const sparse_matrix_t A,
                                            const MKL_INT         row,
                                            const MKL_INT         col,
                                            const float           value );

    sparse_status_t mkl_sparse_d_set_value( const sparse_matrix_t A,
                                            const MKL_INT         row,
                                            const MKL_INT         col,
                                            const double          value );

    sparse_status_t mkl_sparse_c_set_value( const sparse_matrix_t A,
                                            const MKL_INT         row,
                                            const MKL_INT         col,
                                            const MKL_Complex8    value );

    sparse_status_t mkl_sparse_z_set_value( const sparse_matrix_t A,
                                            const MKL_INT         row,
                                            const MKL_INT         col,
                                            const MKL_Complex16   value );

    sparse_status_t mkl_sparse_s_set_value_64( const sparse_matrix_t A,
                                               const MKL_INT64       row,
                                               const MKL_INT64       col,
                                               const float           value );

    sparse_status_t mkl_sparse_d_set_value_64( const sparse_matrix_t A,
                                               const MKL_INT64       row,
                                               const MKL_INT64       col,
                                               const double          value );

    sparse_status_t mkl_sparse_c_set_value_64( const sparse_matrix_t A,
                                               const MKL_INT64       row,
                                               const MKL_INT64       col,
                                               const MKL_Complex8    value );

    sparse_status_t mkl_sparse_z_set_value_64( const sparse_matrix_t A,
                                               const MKL_INT64       row,
                                               const MKL_INT64       col,
                                               const MKL_Complex16   value );


    /* update existing values in the matrix for internal storage only
       can be used to either update all or selected values */
    sparse_status_t mkl_sparse_s_update_values( const sparse_matrix_t A,
                                                const MKL_INT         nvalues,
                                                const MKL_INT        *indx,
                                                const MKL_INT        *indy,
                                                      float          *values );

    sparse_status_t mkl_sparse_d_update_values( const sparse_matrix_t A,
                                                const MKL_INT         nvalues,
                                                const MKL_INT        *indx,
                                                const MKL_INT        *indy,
                                                      double         *values );

    sparse_status_t mkl_sparse_c_update_values( const sparse_matrix_t A,
                                                const MKL_INT         nvalues,
                                                const MKL_INT        *indx,
                                                const MKL_INT        *indy,
                                                      MKL_Complex8   *values );

    sparse_status_t mkl_sparse_z_update_values( const sparse_matrix_t A,
                                                const MKL_INT         nvalues,
                                                const MKL_INT        *indx,
                                                const MKL_INT        *indy,
                                                      MKL_Complex16  *values );

    sparse_status_t mkl_sparse_s_update_values_64( const sparse_matrix_t A,
                                                   const MKL_INT64       nvalues,
                                                   const MKL_INT64      *indx,
                                                   const MKL_INT64      *indy,
                                                         float          *values );

    sparse_status_t mkl_sparse_d_update_values_64( const sparse_matrix_t A,
                                                   const MKL_INT64       nvalues,
                                                   const MKL_INT64      *indx,
                                                   const MKL_INT64      *indy,
                                                         double         *values );

    sparse_status_t mkl_sparse_c_update_values_64( const sparse_matrix_t A,
                                                   const MKL_INT64       nvalues,
                                                   const MKL_INT64      *indx,
                                                   const MKL_INT64      *indy,
                                                         MKL_Complex8   *values );

    sparse_status_t mkl_sparse_z_update_values_64( const sparse_matrix_t A,
                                                   const MKL_INT64       nvalues,
                                                   const MKL_INT64      *indx,
                                                   const MKL_INT64      *indy,
                                                         MKL_Complex16  *values );


/*****************************************************************************************/
/****************************** Verbose mode routine *************************************/
/*****************************************************************************************/

    /* allow to switch on/off verbose mode */
    sparse_status_t mkl_sparse_set_verbose_mode ( verbose_mode_t verbose ); /* unsupported currently */

    sparse_status_t mkl_sparse_set_verbose_mode_64 ( verbose_mode_t verbose ); /* unsupported currently */

/*****************************************************************************************/
/****************************** Optimization routines ************************************/
/*****************************************************************************************/

    /* Describe expected operations with amount of iterations */
    sparse_status_t mkl_sparse_set_mv_hint    ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const MKL_INT             expected_calls );

    sparse_status_t mkl_sparse_set_mv_hint_64 ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const MKL_INT64           expected_calls );



    sparse_status_t mkl_sparse_set_dotmv_hint    ( const sparse_matrix_t     A,
                                                   const sparse_operation_t  operation, /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                   const struct matrix_descr descr,     /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                   const MKL_INT             expectedCalls );

    sparse_status_t mkl_sparse_set_dotmv_hint_64 ( const sparse_matrix_t     A,
                                                   const sparse_operation_t  operation, /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                   const struct matrix_descr descr,     /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                   const MKL_INT64           expectedCalls );



    sparse_status_t mkl_sparse_set_mm_hint    ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                                const MKL_INT             dense_matrix_size, /* amount of columns in dense matrix */
                                                const MKL_INT             expected_calls );

    sparse_status_t mkl_sparse_set_mm_hint_64 ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                                const MKL_INT64           dense_matrix_size, /* amount of columns in dense matrix */
                                                const MKL_INT64           expected_calls );



    sparse_status_t mkl_sparse_set_sv_hint    ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const MKL_INT             expected_calls );

    sparse_status_t mkl_sparse_set_sv_hint_64 ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const MKL_INT64           expected_calls );



    sparse_status_t mkl_sparse_set_sm_hint    ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                                const MKL_INT             dense_matrix_size, /* amount of columns in dense matrix */
                                                const MKL_INT             expected_calls );

    sparse_status_t mkl_sparse_set_sm_hint_64 ( const sparse_matrix_t     A,
                                                const sparse_operation_t  operation,
                                                const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                                const MKL_INT64           dense_matrix_size, /* amount of columns in dense matrix */
                                                const MKL_INT64           expected_calls );



    sparse_status_t mkl_sparse_set_symgs_hint    ( const sparse_matrix_t     A,
                                                   const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                   const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                   const MKL_INT             expected_calls );

    sparse_status_t mkl_sparse_set_symgs_hint_64 ( const sparse_matrix_t     A,
                                                   const sparse_operation_t  operation,  /* SPARSE_OPERATION_NON_TRANSPOSE is default value for infinite amount of calls */
                                                   const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                   const MKL_INT64           expected_calls );



    sparse_status_t mkl_sparse_set_lu_smoother_hint( const sparse_matrix_t     A,
                                                     const sparse_operation_t  operation,
                                                     const struct matrix_descr descr, /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                     const MKL_INT             expectedCalls );

    sparse_status_t mkl_sparse_set_lu_smoother_hint_64( const sparse_matrix_t     A,
                                                        const sparse_operation_t  operation,
                                                        const struct matrix_descr descr, /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                        const MKL_INT64           expectedCalls );



    sparse_status_t mkl_sparse_set_sorv_hint   ( const sparse_sor_type_t   type,  /* choice of forward, backward sweep or SSOR operation */
                                                 const sparse_matrix_t     A,
                                                 const struct matrix_descr descr, /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                 const MKL_INT             expectedCalls );

    sparse_status_t mkl_sparse_set_sorv_hint_64( const sparse_sor_type_t   type,  /* choice of forward, backward sweep or SSOR operation */
                                                 const sparse_matrix_t     A,
                                                 const struct matrix_descr descr, /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                                 const MKL_INT64           expectedCalls );



    /* Describe memory usage model */
    sparse_status_t mkl_sparse_set_memory_hint ( const sparse_matrix_t       A,
                                                 const sparse_memory_usage_t policy );    /* SPARSE_MEMORY_AGGRESSIVE is default value */

    sparse_status_t mkl_sparse_set_memory_hint_64 ( const sparse_matrix_t       A,
                                                    const sparse_memory_usage_t policy );    /* SPARSE_MEMORY_AGGRESSIVE is default value */




/*
    Optimize matrix described by the handle. It uses hints (optimization and memory) that should be set up before this call.
    If hints were not explicitly defined, default vales are:
    SPARSE_OPERATION_NON_TRANSPOSE for matrix-vector multiply with infinite number of expected iterations.
*/
    sparse_status_t mkl_sparse_optimize ( sparse_matrix_t  A );

    sparse_status_t mkl_sparse_optimize_64 ( sparse_matrix_t  A );

/*****************************************************************************************/
/****************************** Computational routines ***********************************/
/*****************************************************************************************/

    sparse_status_t mkl_sparse_order( const sparse_matrix_t A );

    sparse_status_t mkl_sparse_order_64( const sparse_matrix_t A );

/*
    Perform computations based on created matrix handle

    Level 2
*/
    /*   Computes y = alpha * A * x + beta * y   */
    sparse_status_t mkl_sparse_s_mv ( const sparse_operation_t  operation,
                                      const float               alpha,
                                      const sparse_matrix_t     A,
                                      const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                      const float               *x,
                                      const float               beta,
                                      float                     *y );

    sparse_status_t mkl_sparse_d_mv ( const sparse_operation_t  operation,
                                      const double              alpha,
                                      const sparse_matrix_t     A,
                                      const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                      const double              *x,
                                      const double              beta,
                                      double                    *y );

    sparse_status_t mkl_sparse_c_mv ( const sparse_operation_t  operation,
                                      const MKL_Complex8        alpha,
                                      const sparse_matrix_t     A,
                                      const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                      const MKL_Complex8        *x,
                                      const MKL_Complex8        beta,
                                      MKL_Complex8              *y );

    sparse_status_t mkl_sparse_z_mv ( const sparse_operation_t  operation,
                                      const MKL_Complex16       alpha,
                                      const sparse_matrix_t     A,
                                      const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                      const MKL_Complex16       *x,
                                      const MKL_Complex16       beta,
                                      MKL_Complex16             *y );

    sparse_status_t mkl_sparse_s_mv_64 ( const sparse_operation_t  operation,
                                         const float               alpha,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                         const float               *x,
                                         const float               beta,
                                         float                     *y );

    sparse_status_t mkl_sparse_d_mv_64 ( const sparse_operation_t  operation,
                                         const double              alpha,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                         const double              *x,
                                         const double              beta,
                                         double                    *y );

    sparse_status_t mkl_sparse_c_mv_64 ( const sparse_operation_t  operation,
                                         const MKL_Complex8        alpha,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                         const MKL_Complex8        *x,
                                         const MKL_Complex8        beta,
                                         MKL_Complex8              *y );

    sparse_status_t mkl_sparse_z_mv_64 ( const sparse_operation_t  operation,
                                         const MKL_Complex16       alpha,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                         const MKL_Complex16       *x,
                                         const MKL_Complex16       beta,
                                         MKL_Complex16             *y );



    /*    Computes y = alpha * A * x + beta * y  and d = <x, y> , the l2 inner product */
    sparse_status_t mkl_sparse_s_dotmv( const sparse_operation_t  transA,
                                        const float               alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const float               *x,
                                        const float               beta,
                                        float                     *y,
                                        float                     *d);

    sparse_status_t mkl_sparse_d_dotmv( const sparse_operation_t  transA,
                                        const double              alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const double              *x,
                                        const double              beta,
                                        double                    *y,
                                        double                    *d);

    sparse_status_t mkl_sparse_c_dotmv( const sparse_operation_t  transA,
                                        const MKL_Complex8        alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const MKL_Complex8        *x,
                                        const MKL_Complex8        beta,
                                        MKL_Complex8              *y,
                                        MKL_Complex8              *d);

    sparse_status_t mkl_sparse_z_dotmv( const sparse_operation_t  transA,
                                        const MKL_Complex16       alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const MKL_Complex16       *x,
                                        const MKL_Complex16       beta,
                                        MKL_Complex16             *y,
                                        MKL_Complex16             *d);

    sparse_status_t mkl_sparse_s_dotmv_64( const sparse_operation_t  transA,
                                           const float               alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const float               *x,
                                           const float               beta,
                                           float                     *y,
                                           float                     *d);

    sparse_status_t mkl_sparse_d_dotmv_64( const sparse_operation_t  transA,
                                           const double              alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const double              *x,
                                           const double              beta,
                                           double                    *y,
                                           double                    *d);

    sparse_status_t mkl_sparse_c_dotmv_64( const sparse_operation_t  transA,
                                           const MKL_Complex8        alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const MKL_Complex8        *x,
                                           const MKL_Complex8        beta,
                                           MKL_Complex8              *y,
                                           MKL_Complex8              *d);

    sparse_status_t mkl_sparse_z_dotmv_64( const sparse_operation_t  transA,
                                           const MKL_Complex16       alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const MKL_Complex16       *x,
                                           const MKL_Complex16       beta,
                                           MKL_Complex16             *y,
                                           MKL_Complex16             *d);


    /*   Solves triangular system y = alpha * A^{-1} * x   */
    sparse_status_t mkl_sparse_s_trsv ( const sparse_operation_t  operation,
                                        const float               alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const float               *x,
                                        float                     *y );

    sparse_status_t mkl_sparse_d_trsv ( const sparse_operation_t  operation,
                                        const double              alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const double              *x,
                                        double                    *y );

    sparse_status_t mkl_sparse_c_trsv ( const sparse_operation_t  operation,
                                        const MKL_Complex8        alpha,
                                        const sparse_matrix_t    A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const MKL_Complex8        *x,
                                        MKL_Complex8              *y );

    sparse_status_t mkl_sparse_z_trsv ( const sparse_operation_t  operation,
                                        const MKL_Complex16       alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const MKL_Complex16       *x,
                                        MKL_Complex16             *y );

    sparse_status_t mkl_sparse_s_trsv_64 ( const sparse_operation_t  operation,
                                           const float               alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const float               *x,
                                           float                     *y );

    sparse_status_t mkl_sparse_d_trsv_64 ( const sparse_operation_t  operation,
                                           const double              alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const double              *x,
                                           double                    *y );

    sparse_status_t mkl_sparse_c_trsv_64 ( const sparse_operation_t  operation,
                                           const MKL_Complex8        alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const MKL_Complex8        *x,
                                           MKL_Complex8              *y );

    sparse_status_t mkl_sparse_z_trsv_64 ( const sparse_operation_t  operation,
                                           const MKL_Complex16       alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const MKL_Complex16       *x,
                                           MKL_Complex16             *y );



    /*   Applies symmetric Gauss-Seidel preconditioner to symmetric system A * x = b, */
    /*   that is, it solves:                                                          */
    /*      x0       = alpha*x                                                        */
    /*      (L+D)*x1 = b - U*x0                                                       */
    /*      (D+U)*x  = b - L*x1                                                       */
    /*                                                                                */
    /*   SYMGS_MV also returns y = A*x                                                */
    sparse_status_t mkl_sparse_s_symgs ( const sparse_operation_t  op,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,
                                         const float               alpha,
                                         const float               *b,
                                         float                     *x);

    sparse_status_t mkl_sparse_d_symgs ( const sparse_operation_t  op,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,
                                         const double              alpha,
                                         const double              *b,
                                         double                    *x);

    sparse_status_t mkl_sparse_c_symgs ( const sparse_operation_t  op,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,
                                         const MKL_Complex8        alpha,
                                         const MKL_Complex8        *b,
                                         MKL_Complex8              *x);

    sparse_status_t mkl_sparse_z_symgs ( const sparse_operation_t  op,
                                         const sparse_matrix_t     A,
                                         const struct matrix_descr descr,
                                         const MKL_Complex16       alpha,
                                         const MKL_Complex16       *b,
                                         MKL_Complex16             *x);

    sparse_status_t mkl_sparse_s_symgs_64 ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const float               alpha,
                                            const float               *b,
                                            float                     *x);

    sparse_status_t mkl_sparse_d_symgs_64 ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const double              alpha,
                                            const double              *b,
                                            double                    *x);

    sparse_status_t mkl_sparse_c_symgs_64 ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const MKL_Complex8        alpha,
                                            const MKL_Complex8        *b,
                                            MKL_Complex8              *x);

    sparse_status_t mkl_sparse_z_symgs_64 ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const MKL_Complex16       alpha,
                                            const MKL_Complex16       *b,
                                            MKL_Complex16             *x);


    sparse_status_t mkl_sparse_s_symgs_mv ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const float               alpha,
                                            const float               *b,
                                            float                     *x,
                                            float                     *y);

    sparse_status_t mkl_sparse_d_symgs_mv ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const double              alpha,
                                            const double              *b,
                                            double                    *x,
                                            double                    *y);

    sparse_status_t mkl_sparse_c_symgs_mv ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const MKL_Complex8        alpha,
                                            const MKL_Complex8        *b,
                                            MKL_Complex8              *x,
                                            MKL_Complex8              *y);

    sparse_status_t mkl_sparse_z_symgs_mv ( const sparse_operation_t  op,
                                            const sparse_matrix_t     A,
                                            const struct matrix_descr descr,
                                            const MKL_Complex16       alpha,
                                            const MKL_Complex16       *b,
                                            MKL_Complex16             *x,
                                            MKL_Complex16             *y);

    sparse_status_t mkl_sparse_s_symgs_mv_64 ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const float               alpha,
                                               const float               *b,
                                               float                     *x,
                                               float                     *y);

    sparse_status_t mkl_sparse_d_symgs_mv_64 ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const double              alpha,
                                               const double              *b,
                                               double                    *x,
                                               double                    *y);

    sparse_status_t mkl_sparse_c_symgs_mv_64 ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const MKL_Complex8        alpha,
                                               const MKL_Complex8        *b,
                                               MKL_Complex8              *x,
                                               MKL_Complex8              *y);

    sparse_status_t mkl_sparse_z_symgs_mv_64 ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const MKL_Complex16       alpha,
                                               const MKL_Complex16       *b,
                                               MKL_Complex16             *x,
                                               MKL_Complex16             *y);


    /*   Computes an action of a preconditioner
         which corresponds to the approximate matrix decomposition A ~ (L+D)*E*(U+D)
         for the system Ax = b.

         L is lower triangular part of A
         U is upper triangular part of A
         D is diagonal values of A
         E is approximate diagonal inverse

         That is, it solves:
             r = rhs - A*x0
             (L + D)*E*(U + D)*dx = r
             x1 = x0 + dx                                        */

    sparse_status_t mkl_sparse_s_lu_smoother ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const float               *diag,
                                               const float               *approx_diag_inverse,
                                               float                     *x,
                                               const float               *rhs);

    sparse_status_t mkl_sparse_d_lu_smoother ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const double              *diag,
                                               const double              *approx_diag_inverse,
                                               double                    *x,
                                               const double              *rhs);

    sparse_status_t mkl_sparse_c_lu_smoother ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const MKL_Complex8        *diag,
                                               const MKL_Complex8        *approx_diag_inverse,
                                               MKL_Complex8              *x,
                                               const MKL_Complex8        *rhs);

    sparse_status_t mkl_sparse_z_lu_smoother ( const sparse_operation_t  op,
                                               const sparse_matrix_t     A,
                                               const struct matrix_descr descr,
                                               const MKL_Complex16       *diag,
                                               const MKL_Complex16       *approx_diag_inverse,
                                               MKL_Complex16             *x,
                                               const MKL_Complex16       *rhs);

    sparse_status_t mkl_sparse_s_lu_smoother_64 ( const sparse_operation_t  op,
                                                  const sparse_matrix_t     A,
                                                  const struct matrix_descr descr,
                                                  const float               *diag,
                                                  const float               *approx_diag_inverse,
                                                  float                     *x,
                                                  const float               *rhs);

    sparse_status_t mkl_sparse_d_lu_smoother_64 ( const sparse_operation_t  op,
                                                  const sparse_matrix_t     A,
                                                  const struct matrix_descr descr,
                                                  const double              *diag,
                                                  const double              *approx_diag_inverse,
                                                  double                    *x,
                                                  const double              *rhs);

    sparse_status_t mkl_sparse_c_lu_smoother_64 ( const sparse_operation_t  op,
                                                  const sparse_matrix_t     A,
                                                  const struct matrix_descr descr,
                                                  const MKL_Complex8        *diag,
                                                  const MKL_Complex8        *approx_diag_inverse,
                                                  MKL_Complex8              *x,
                                                  const MKL_Complex8        *rhs);

    sparse_status_t mkl_sparse_z_lu_smoother_64 ( const sparse_operation_t  op,
                                                  const sparse_matrix_t     A,
                                                  const struct matrix_descr descr,
                                                  const MKL_Complex16       *diag,
                                                  const MKL_Complex16       *approx_diag_inverse,
                                                  MKL_Complex16             *x,
                                                  const MKL_Complex16       *rhs);


    /* Level 3 */

    /*   Computes y = alpha * A * x + beta * y   */
    sparse_status_t mkl_sparse_s_mm( const sparse_operation_t  operation,
                                     const float               alpha,
                                     const sparse_matrix_t     A,
                                     const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                     const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                     const float               *x,
                                     const MKL_INT             columns,
                                     const MKL_INT             ldx,
                                     const float               beta,
                                     float                     *y,
                                     const MKL_INT             ldy );

    sparse_status_t mkl_sparse_d_mm( const sparse_operation_t  operation,
                                     const double              alpha,
                                     const sparse_matrix_t     A,
                                     const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                     const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                     const double              *x,
                                     const MKL_INT             columns,
                                     const MKL_INT             ldx,
                                     const double              beta,
                                     double                    *y,
                                     const MKL_INT             ldy );

    sparse_status_t mkl_sparse_c_mm( const sparse_operation_t  operation,
                                     const MKL_Complex8        alpha,
                                     const sparse_matrix_t     A,
                                     const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                     const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                     const MKL_Complex8        *x,
                                     const MKL_INT             columns,
                                     const MKL_INT             ldx,
                                     const MKL_Complex8        beta,
                                     MKL_Complex8              *y,
                                     const MKL_INT             ldy );

    sparse_status_t mkl_sparse_z_mm( const sparse_operation_t  operation,
                                     const MKL_Complex16       alpha,
                                     const sparse_matrix_t     A,
                                     const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                     const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                     const MKL_Complex16       *x,
                                     const MKL_INT             columns,
                                     const MKL_INT             ldx,
                                     const MKL_Complex16       beta,
                                     MKL_Complex16             *y,
                                     const MKL_INT             ldy );

    sparse_status_t mkl_sparse_s_mm_64( const sparse_operation_t  operation,
                                        const float               alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const float               *x,
                                        const MKL_INT64           columns,
                                        const MKL_INT64           ldx,
                                        const float               beta,
                                        float                     *y,
                                        const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_d_mm_64( const sparse_operation_t  operation,
                                        const double              alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const double              *x,
                                        const MKL_INT64           columns,
                                        const MKL_INT64           ldx,
                                        const double              beta,
                                        double                    *y,
                                        const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_c_mm_64( const sparse_operation_t  operation,
                                        const MKL_Complex8        alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const MKL_Complex8        *x,
                                        const MKL_INT64           columns,
                                        const MKL_INT64           ldx,
                                        const MKL_Complex8        beta,
                                        MKL_Complex8              *y,
                                        const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_z_mm_64( const sparse_operation_t  operation,
                                        const MKL_Complex16       alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,          /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,         /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const MKL_Complex16       *x,
                                        const MKL_INT64           columns,
                                        const MKL_INT64           ldx,
                                        const MKL_Complex16       beta,
                                        MKL_Complex16             *y,
                                        const MKL_INT64           ldy );


    /*   Solves triangular system y = alpha * A^{-1} * x   */
    sparse_status_t mkl_sparse_s_trsm ( const sparse_operation_t  operation,
                                        const float               alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const float               *x,
                                        const MKL_INT             columns,
                                        const MKL_INT             ldx,
                                        float                     *y,
                                        const MKL_INT             ldy );

    sparse_status_t mkl_sparse_d_trsm ( const sparse_operation_t  operation,
                                        const double              alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const double              *x,
                                        const MKL_INT             columns,
                                        const MKL_INT             ldx,
                                        double                    *y,
                                        const MKL_INT             ldy );

    sparse_status_t mkl_sparse_c_trsm ( const sparse_operation_t  operation,
                                        const MKL_Complex8        alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const MKL_Complex8        *x,
                                        const MKL_INT             columns,
                                        const MKL_INT             ldx,
                                        MKL_Complex8              *y,
                                        const MKL_INT             ldy );

    sparse_status_t mkl_sparse_z_trsm ( const sparse_operation_t  operation,
                                        const MKL_Complex16       alpha,
                                        const sparse_matrix_t     A,
                                        const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                        const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                        const MKL_Complex16       *x,
                                        const MKL_INT             columns,
                                        const MKL_INT             ldx,
                                        MKL_Complex16             *y,
                                        const MKL_INT             ldy );

    sparse_status_t mkl_sparse_s_trsm_64 ( const sparse_operation_t  operation,
                                           const float               alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                           const float               *x,
                                           const MKL_INT64           columns,
                                           const MKL_INT64           ldx,
                                           float                     *y,
                                           const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_d_trsm_64 ( const sparse_operation_t  operation,
                                           const double              alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                           const double              *x,
                                           const MKL_INT64           columns,
                                           const MKL_INT64           ldx,
                                           double                    *y,
                                           const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_c_trsm_64 ( const sparse_operation_t  operation,
                                           const MKL_Complex8        alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                           const MKL_Complex8        *x,
                                           const MKL_INT64           columns,
                                           const MKL_INT64           ldx,
                                           MKL_Complex8              *y,
                                           const MKL_INT64           ldy );

    sparse_status_t mkl_sparse_z_trsm_64 ( const sparse_operation_t  operation,
                                           const MKL_Complex16       alpha,
                                           const sparse_matrix_t     A,
                                           const struct matrix_descr descr,      /* sparse_matrix_type_t + sparse_fill_mode_t + sparse_diag_type_t */
                                           const sparse_layout_t     layout,     /* storage scheme for the dense matrix: C-style or Fortran-style */
                                           const MKL_Complex16       *x,
                                           const MKL_INT64           columns,
                                           const MKL_INT64           ldx,
                                           MKL_Complex16             *y,
                                           const MKL_INT64           ldy );


    /* Sparse-sparse functionality */


    /*   Computes sum of sparse matrices: C = alpha * op(A) + B, result is sparse   */
    sparse_status_t mkl_sparse_s_add( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      const float              alpha,
                                      const sparse_matrix_t    B,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_d_add( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      const double             alpha,
                                      const sparse_matrix_t    B,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_c_add( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      const MKL_Complex8       alpha,
                                      const sparse_matrix_t    B,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_z_add( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      const MKL_Complex16      alpha,
                                      const sparse_matrix_t    B,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_s_add_64( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         const float              alpha,
                                         const sparse_matrix_t    B,
                                         sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_d_add_64( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         const double             alpha,
                                         const sparse_matrix_t    B,
                                         sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_c_add_64( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         const MKL_Complex8       alpha,
                                         const sparse_matrix_t    B,
                                         sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_z_add_64( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         const MKL_Complex16      alpha,
                                         const sparse_matrix_t    B,
                                         sparse_matrix_t          *C );



    /*   Computes product of sparse matrices: C = op(A) * B, result is sparse   */
    sparse_status_t mkl_sparse_spmm ( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      const sparse_matrix_t    B,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_spmm_64 ( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         const sparse_matrix_t    B,
                                         sparse_matrix_t          *C );

    /*   Computes product of sparse matrices: C = opA(A) * opB(B), result is sparse   */
    sparse_status_t mkl_sparse_sp2m ( const sparse_operation_t  transA,
                                      const struct matrix_descr descrA,
                                      const sparse_matrix_t     A,
                                      const sparse_operation_t  transB,
                                      const struct matrix_descr descrB,
                                      const sparse_matrix_t     B,
                                      const sparse_request_t    request,
                                      sparse_matrix_t           *C );

    sparse_status_t mkl_sparse_sp2m_64 ( const sparse_operation_t  transA,
                                         const struct matrix_descr descrA,
                                         const sparse_matrix_t     A,
                                         const sparse_operation_t  transB,
                                         const struct matrix_descr descrB,
                                         const sparse_matrix_t     B,
                                         const sparse_request_t    request,
                                         sparse_matrix_t           *C );


    /*   Computes product of sparse matrices: C = op(A) * (op(A))^{T for real or H for complex}, result is sparse   */
    sparse_status_t mkl_sparse_syrk ( const sparse_operation_t operation,
                                      const sparse_matrix_t    A,
                                      sparse_matrix_t          *C );

    sparse_status_t mkl_sparse_syrk_64 ( const sparse_operation_t operation,
                                         const sparse_matrix_t    A,
                                         sparse_matrix_t          *C );

    /*   Computes product of sparse matrices: C = op(A) * B * (op(A))^{T for real or H for complex}, result is sparse   */
    sparse_status_t mkl_sparse_sypr ( const sparse_operation_t  transA,
                                      const sparse_matrix_t     A,
                                      const sparse_matrix_t     B,
                                      const struct matrix_descr descrB,
                                      sparse_matrix_t           *C,
                                      const sparse_request_t    request );

    sparse_status_t mkl_sparse_sypr_64 ( const sparse_operation_t  transA,
                                         const sparse_matrix_t     A,
                                         const sparse_matrix_t     B,
                                         const struct matrix_descr descrB,
                                         sparse_matrix_t           *C,
                                         const sparse_request_t    request );


    /*   Computes product of sparse matrices: C = op(A) * B * (op(A))^{T for real or H for complex}, result is dense */
    sparse_status_t mkl_sparse_s_syprd ( const sparse_operation_t op,
                                         const sparse_matrix_t    A,
                                         const float              *B,
                                         const sparse_layout_t    layoutB,
                                         const MKL_INT            ldb,
                                         const float              alpha,
                                         const float              beta,
                                         float                    *C,
                                         const sparse_layout_t    layoutC,
                                         const MKL_INT            ldc );

    sparse_status_t mkl_sparse_d_syprd ( const sparse_operation_t op,
                                         const sparse_matrix_t    A,
                                         const double             *B,
                                         const sparse_layout_t    layoutB,
                                         const MKL_INT            ldb,
                                         const double             alpha,
                                         const double             beta,
                                         double                   *C,
                                         const sparse_layout_t    layoutC,
                                         const MKL_INT            ldc );

    sparse_status_t mkl_sparse_c_syprd ( const sparse_operation_t op,
                                         const sparse_matrix_t    A,
                                         const MKL_Complex8       *B,
                                         const sparse_layout_t    layoutB,
                                         const MKL_INT            ldb,
                                         const MKL_Complex8       alpha,
                                         const MKL_Complex8       beta,
                                         MKL_Complex8             *C,
                                         const sparse_layout_t    layoutC,
                                         const MKL_INT            ldc );

    sparse_status_t mkl_sparse_z_syprd ( const sparse_operation_t op,
                                         const sparse_matrix_t    A,
                                         const MKL_Complex16      *B,
                                         const sparse_layout_t    layoutB,
                                         const MKL_INT            ldb,
                                         const MKL_Complex16      alpha,
                                         const MKL_Complex16      beta,
                                         MKL_Complex16            *C,
                                         const sparse_layout_t    layoutC,
                                         const MKL_INT            ldc );

    sparse_status_t mkl_sparse_s_syprd_64 ( const sparse_operation_t op,
                                            const sparse_matrix_t    A,
                                            const float              *B,
                                            const sparse_layout_t    layoutB,
                                            const MKL_INT64          ldb,
                                            const float              alpha,
                                            const float              beta,
                                            float                    *C,
                                            const sparse_layout_t    layoutC,
                                            const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_d_syprd_64 ( const sparse_operation_t op,
                                            const sparse_matrix_t    A,
                                            const double             *B,
                                            const sparse_layout_t    layoutB,
                                            const MKL_INT64          ldb,
                                            const double             alpha,
                                            const double             beta,
                                            double                   *C,
                                            const sparse_layout_t    layoutC,
                                            const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_c_syprd_64 ( const sparse_operation_t op,
                                            const sparse_matrix_t    A,
                                            const MKL_Complex8       *B,
                                            const sparse_layout_t    layoutB,
                                            const MKL_INT64          ldb,
                                            const MKL_Complex8       alpha,
                                            const MKL_Complex8       beta,
                                            MKL_Complex8             *C,
                                            const sparse_layout_t    layoutC,
                                            const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_z_syprd_64 ( const sparse_operation_t op,
                                            const sparse_matrix_t    A,
                                            const MKL_Complex16      *B,
                                            const sparse_layout_t    layoutB,
                                            const MKL_INT64          ldb,
                                            const MKL_Complex16      alpha,
                                            const MKL_Complex16      beta,
                                            MKL_Complex16            *C,
                                            const sparse_layout_t    layoutC,
                                            const MKL_INT64          ldc );


    /*   Computes product of sparse matrices: C = op(A) * B, result is dense   */
    sparse_status_t mkl_sparse_s_spmmd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const sparse_matrix_t    B,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        float                    *C,
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_d_spmmd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const sparse_matrix_t    B,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        double                   *C,
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_c_spmmd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const sparse_matrix_t    B,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        MKL_Complex8             *C,
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_z_spmmd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const sparse_matrix_t    B,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        MKL_Complex16            *C,
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_s_spmmd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const sparse_matrix_t    B,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           float                    *C,
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_d_spmmd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const sparse_matrix_t    B,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           double                   *C,
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_c_spmmd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const sparse_matrix_t    B,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           MKL_Complex8             *C,
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_z_spmmd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const sparse_matrix_t    B,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           MKL_Complex16            *C,
                                           const MKL_INT64          ldc );



    /*   Computes product of sparse matrices: C = opA(A) * opB(B), result is dense*/
    sparse_status_t mkl_sparse_s_sp2md ( const sparse_operation_t  transA,
                                         const struct matrix_descr descrA,
                                         const sparse_matrix_t     A,
                                         const sparse_operation_t  transB,
                                         const struct matrix_descr descrB,
                                         const sparse_matrix_t     B,
                                         const float               alpha,
                                         const float               beta,
                                         float                     *C,
                                         const sparse_layout_t     layout,
                                         const MKL_INT             ldc );

    sparse_status_t mkl_sparse_d_sp2md ( const sparse_operation_t  transA,
                                         const struct matrix_descr descrA,
                                         const sparse_matrix_t     A,
                                         const sparse_operation_t  transB,
                                         const struct matrix_descr descrB,
                                         const sparse_matrix_t     B,
                                         const double              alpha,
                                         const double              beta,
                                         double                    *C,
                                         const sparse_layout_t     layout,
                                         const MKL_INT             ldc );

    sparse_status_t mkl_sparse_c_sp2md ( const sparse_operation_t  transA,
                                         const struct matrix_descr descrA,
                                         const sparse_matrix_t     A,
                                         const sparse_operation_t  transB,
                                         const struct matrix_descr descrB,
                                         const sparse_matrix_t     B,
                                         const MKL_Complex8        alpha,
                                         const MKL_Complex8        beta,
                                         MKL_Complex8              *C,
                                         const sparse_layout_t     layout,
                                         const MKL_INT             ldc );

    sparse_status_t mkl_sparse_z_sp2md ( const sparse_operation_t  transA,
                                         const struct matrix_descr descrA,
                                         const sparse_matrix_t     A,
                                         const sparse_operation_t  transB,
                                         const struct matrix_descr descrB,
                                         const sparse_matrix_t     B,
                                         const MKL_Complex16       alpha,
                                         const MKL_Complex16       beta,
                                         MKL_Complex16             *C,
                                         const sparse_layout_t     layout,
                                         const MKL_INT             ldc );

    sparse_status_t mkl_sparse_s_sp2md_64 ( const sparse_operation_t  transA,
                                            const struct matrix_descr descrA,
                                            const sparse_matrix_t     A,
                                            const sparse_operation_t  transB,
                                            const struct matrix_descr descrB,
                                            const sparse_matrix_t     B,
                                            const float               alpha,
                                            const float               beta,
                                            float                     *C,
                                            const sparse_layout_t     layout,
                                            const MKL_INT64           ldc );

    sparse_status_t mkl_sparse_d_sp2md_64 ( const sparse_operation_t  transA,
                                            const struct matrix_descr descrA,
                                            const sparse_matrix_t     A,
                                            const sparse_operation_t  transB,
                                            const struct matrix_descr descrB,
                                            const sparse_matrix_t     B,
                                            const double              alpha,
                                            const double              beta,
                                            double                    *C,
                                            const sparse_layout_t     layout,
                                            const MKL_INT64           ldc );

    sparse_status_t mkl_sparse_c_sp2md_64 ( const sparse_operation_t  transA,
                                            const struct matrix_descr descrA,
                                            const sparse_matrix_t     A,
                                            const sparse_operation_t  transB,
                                            const struct matrix_descr descrB,
                                            const sparse_matrix_t     B,
                                            const MKL_Complex8        alpha,
                                            const MKL_Complex8        beta,
                                            MKL_Complex8              *C,
                                            const sparse_layout_t     layout,
                                            const MKL_INT64           ldc );

    sparse_status_t mkl_sparse_z_sp2md_64 ( const sparse_operation_t  transA,
                                            const struct matrix_descr descrA,
                                            const sparse_matrix_t     A,
                                            const sparse_operation_t  transB,
                                            const struct matrix_descr descrB,
                                            const sparse_matrix_t     B,
                                            const MKL_Complex16       alpha,
                                            const MKL_Complex16       beta,
                                            MKL_Complex16             *C,
                                            const sparse_layout_t     layout,
                                            const MKL_INT64           ldc );


    /*   Computes product of sparse matrices: C = op(A) * (op(A))^{T for real or H for complex}, result is dense */
    sparse_status_t mkl_sparse_s_syrkd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const float              alpha,
                                        const float              beta,
                                        float                    *C,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_d_syrkd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const double             alpha,
                                        const double             beta,
                                        double                   *C,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_c_syrkd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const MKL_Complex8       alpha,
                                        const MKL_Complex8       beta,
                                        MKL_Complex8             *C,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_z_syrkd( const sparse_operation_t operation,
                                        const sparse_matrix_t    A,
                                        const MKL_Complex16      alpha,
                                        const MKL_Complex16      beta,
                                        MKL_Complex16            *C,
                                        const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                        const MKL_INT            ldc );

    sparse_status_t mkl_sparse_s_syrkd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const float              alpha,
                                           const float              beta,
                                           float                    *C,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_d_syrkd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const double             alpha,
                                           const double             beta,
                                           double                   *C,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_c_syrkd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const MKL_Complex8       alpha,
                                           const MKL_Complex8       beta,
                                           MKL_Complex8             *C,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           const MKL_INT64          ldc );

    sparse_status_t mkl_sparse_z_syrkd_64( const sparse_operation_t operation,
                                           const sparse_matrix_t    A,
                                           const MKL_Complex16      alpha,
                                           const MKL_Complex16      beta,
                                           MKL_Complex16            *C,
                                           const sparse_layout_t    layout,       /* storage scheme for the output dense matrix: C-style or Fortran-style */
                                           const MKL_INT64          ldc );


    /* Computes forward or backward sweep of successive over-relaxation (SOR),
       or Symmetric successive over-relaxation (SSOR) */
    sparse_status_t mkl_sparse_s_sorv ( const sparse_sor_type_t   type,   /* choice of forward, backward sweep or SSOR operation */
                                        const struct matrix_descr descrA,
                                        const sparse_matrix_t     A,
                                              float               omega,
                                              float               alpha,  /* alpha equals to 0 mean zero initial guess */
                                              float*              x,      /* solution vector and alpha * x is initial guess */
                                        const float*              b );    /* right-hand side */

    sparse_status_t mkl_sparse_d_sorv ( const sparse_sor_type_t   type,   /* choice of forward, backward sweep or SSOR operation */
                                        const struct matrix_descr descrA,
                                        const sparse_matrix_t     A,
                                              double              omega,
                                              double              alpha,  /* alpha equals to 0 mean zero initial guess */
                                              double*             x,      /* solution vector and alpha * x is initial guess */
                                        const double*             b );    /* right-hand side */

    sparse_status_t mkl_sparse_s_sorv_64 ( const sparse_sor_type_t   type,   /* choice of forward, backward sweep or SSOR operation */
                                           const struct matrix_descr descrA,
                                           const sparse_matrix_t     A,
                                                 float               omega,
                                                 float               alpha,  /* alpha equals to 0 mean zero initial guess */
                                                 float*              x,      /* solution vector and alpha * x is initial guess */
                                           const float*              b );    /* right-hand side */

    sparse_status_t mkl_sparse_d_sorv_64 ( const sparse_sor_type_t   type,   /* choice of forward, backward sweep or SSOR operation */
                                           const struct matrix_descr descrA,
                                           const sparse_matrix_t     A,
                                                 double              omega,
                                                 double              alpha,  /* alpha equals to 0 mean zero initial guess */
                                                 double*             x,      /* solution vector and alpha * x is initial guess */
                                           const double*             b );    /* right-hand side */

#ifdef MKL_DEPRECATED
#undef MKL_DEPRECATED
#endif

#ifdef __cplusplus
}
#endif /*__cplusplus */
#endif /*_MKL_SPBLAS_H_ */
