/*******************************************************************************
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2021. All rights reserved.
 * Description: Part of KML library
 * Author: KML
 * Create: 2022
 ******************************************************************************/

#ifndef KML_SCAISS_H_INCLUDED
#define KML_SCAISS_H_INCLUDED

#include <mpi.h>
#include "kml_export.h"
#include "kml_scasolver_defs.h"

// NOLINTBEGIN
#if defined(__cplusplus)
extern "C" {
#endif

#ifndef OMIT_CG_SPD_SI
/**
 * @defgroup cg_spd_si Conjugate Gradient routines for SPD matrix,
 * single precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgInitStripesSI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const float * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgInitWithoutMatSI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgAnalyzeSI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgFactorizeSI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSolveSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSolveDxSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgCleanSI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSetUserPreconditionerSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, float *x));

/** setup solver-agnostic user spmv.
 * @ingroup cg_spd_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSetUserSpmvSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const float *x, float *y));

/** Set an integer parameter in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSetSII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, const float *data, int nd);

/** Get an integer parameter in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgGetSII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgGetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, float *data, int nd);

/** Set an integer parameter for preconditioner in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, const int *data,
    int nd);
/** Set a floating point parameter for subpreconditioner in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSubPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const float *data, int nd);

/** Set an integer parameter for subpreconditioner in IssCgSI handle
 * @ingroup cg_spd_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSubPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);
#endif // OMIT_CG_SPD_SI

#ifndef OMIT_CG_SPD_DI
/**
 * @defgroup cg_spd_di Conjugate Gradient routines for SPD matrix, double
 * precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgInitStripesDI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const double * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgInitWithoutMatDI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgAnalyzeDI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgFactorizeDI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSolveDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSolveDxDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgCleanDI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSetUserPreconditionerDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, double *x));

/** setup solver-agnostic user spmv.
 * @ingroup cg_spd_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCgSetUserSpmvDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const double *x, double *y));

/** Set an integer parameter in IssCgDI handle
 * @ingroup cg_spd_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSetDII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssCgDI handle
 * @ingroup cg_spd_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSetDID(KmlScasolverTask **handle, enum KmlSolverParam param, const double *data, int nd);

/** Get an integer parameter in IssCgDI handle
 * @ingroup cg_spd_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgGetDII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssCgDI handle
 * @ingroup cg_spd_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgGetDID(KmlScasolverTask **handle, enum KmlSolverParam param, double *data, int nd);

/** Set an integer parameter for preconditioner in IssCgDI handle
 * @ingroup cg_spd_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, const int *data,
    int nd);
/** Set an integer parameter for subpreconditioner in IssCgDI handle
 * @ingroup cg_spd_dl
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSubPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);

/** Set a floating point parameter for subpreconditioner in IssCgDI handle
 * @ingroup cg_spd_dl
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCgSubPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const double *data, int nd);
#endif // OMIT_CG_SPD_DI

#ifndef OMIT_BICGSTAB_SI
/**
 * @defgroup bicgstab_si Biconjugate Gradient Stabilized routines for
 * single precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabInitStripesSI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const float * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabInitWithoutMatSI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabAnalyzeSI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabFactorizeSI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSolveSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSolveDxSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b,
    int ldb);

/** clean - clean internal data structure.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabCleanSI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSetUserPreconditionerSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, float *x));

/** setup solver-agnostic user spmv.
 * @ingroup bicgstab_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSetUserSpmvSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const float *x, float *y));

/** Set an integer parameter in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSetSII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, const float *data, int nd);

/** Get an integer parameter in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabGetSII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabGetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, float *data, int nd);

/** Set an integer parameter for preconditioner in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const int *data, int nd);
/** Set a floating point parameter for subpreconditioner in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSubPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    int subn, const float *data, int nd);

/** Set an integer parameter for subpreconditioner in IssBicgstabSI handle
 * @ingroup bicgstab_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSubPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    int subn, const int *data, int nd);
#endif // OMIT_BICGSTAB_SI

#ifndef OMIT_BICGSTAB_DI
/**
 * @defgroup bicgstab_di Conjugate Gradient routines for SPD matrix, double
 * precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabInitStripesDI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const double * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabInitWithoutMatDI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabAnalyzeDI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabFactorizeDI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSolveDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b,
    int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSolveDxDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b,
    int ldb);

/** clean - clean internal data structure.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabCleanDI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSetUserPreconditionerDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, double *x));

/** setup solver-agnostic user spmv.
 * @ingroup bicgstab_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissBicgstabSetUserSpmvDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const double *x, double *y));

/** Set an integer parameter in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSetDII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSetDID(KmlScasolverTask **handle, enum KmlSolverParam param, const double *data,
    int nd);

/** Get an integer parameter in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabGetDII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabGetDID(KmlScasolverTask **handle, enum KmlSolverParam param, double *data, int nd);

/** Set an integer parameter for preconditioner in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const int *data, int nd);
/** Set an integer parameter for subpreconditioner in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSubPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    int subn, const int *data, int nd);

/** Set a floating point parameter for subpreconditioner in IssBicgstabDI handle
 * @ingroup bicgstab_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissBicgstabSubPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    int subn, const double *data, int nd);
#endif // OMIT_BICGSTAB_DI

#ifndef OMIT_GMRES_SI
/**
 * @defgroup gmres_si Generalized Minimal Residual routines for
 * single precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresInitStripesSI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const float * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresInitWithoutMatSI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresAnalyzeSI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresFactorizeSI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSolveSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSolveDxSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresCleanSI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSetUserPreconditionerSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, float *x));

/** setup solver-agnostic user spmv.
 * @ingroup gmres_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSetUserSpmvSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const float *x, float *y));

/** Set an integer parameter in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSetSII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, const float *data, int nd);

/** Get an integer parameter in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresGetSII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresGetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, float *data, int nd);

/** Set an integer parameter for preconditioner in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const int *data, int nd);

/** Set a floating point parameter for preconditioner in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const float *data, int nd);
/** Set a floating point parameter for subpreconditioner in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSubPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const float *data, int nd);

/** Set an integer parameter for subpreconditioner in IssGmresSI handle
 * @ingroup gmres_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSubPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);
#endif // OMIT_GMRES_SI

#ifndef OMIT_GMRES_DI
/**
 * @defgroup gmres_di Conjugate Gradient routines for SPD matrix, double
 * precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresInitStripesDI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const double * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresInitWithoutMatDI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresAnalyzeDI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresFactorizeDI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSolveDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSolveDxDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresCleanDI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSetUserPreconditionerDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, double *x));

/** setup solver-agnostic user spmv.
 * @ingroup gmres_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissGmresSetUserSpmvDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const double *x, double *y));

/** Set an integer parameter in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSetDII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSetDID(KmlScasolverTask **handle, enum KmlSolverParam param, const double *data, int nd);

/** Get an integer parameter in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresGetDII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresGetDID(KmlScasolverTask **handle, enum KmlSolverParam param, double *data, int nd);

/** Set an integer parameter for preconditioner in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const int *data, int nd);

/** Set a floating point parameter for preconditioner in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const double *data, int nd);
/** Set an integer parameter for subpreconditioner in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSubPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);

/** Set a floating point parameter for subpreconditioner in IssGmresDI handle
 * @ingroup gmres_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissGmresSubPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const double *data, int nd);
#endif // OMIT_GMRES_DI

#ifndef OMIT_CSI_SI
/**
 * @defgroup csi_si Generalized Minimal Residual routines for
 * single precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup csi_si
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiInitStripesSI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const float * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup csi_si
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiInitWithoutMatSI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiAnalyzeSI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiFactorizeSI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSolveSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSolveDxSI(KmlScasolverTask **handle, int nb, float *x, int ldx, const float *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiCleanSI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSetUserPreconditionerSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, float *x));

/** setup solver-agnostic user spmv.
 * @ingroup csi_si
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSetUserSpmvSI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const float *x, float *y));

/** Set an integer parameter in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSetSII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, const float *data, int nd);

/** Get an integer parameter in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiGetSII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiGetSIS(KmlScasolverTask **handle, enum KmlSolverParam param, float *data, int nd);

/** Set an integer parameter for preconditioner in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, const int *data,
    int nd);

/** Set a floating point parameter for preconditioner in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const float *data, int nd);
/** Set a floating point parameter for subpreconditioner in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSubPcSetSIS(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const float *data, int nd);

/** Set an integer parameter for subpreconditioner in IssCsiSI handle
 * @ingroup csi_si
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSubPcSetSII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);
#endif // OMIT_CSI_SI

#ifndef OMIT_CSI_DI
/**
 * @defgroup csi_di Conjugate Gradient routines for SPD matrix, double
 * precision, 32-bit indexing
 ******************************************************************************/

/** init - add sparse matrix.
 * @ingroup csi_di
 *
 * @param handle - SCAISS matrix handle
 * @param n - matrix size
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param stripeRow - array of nstripes elements, i-th element is the global number of the first row of the i-th matrix
 * strip
 * @param a - non-zero elements of the input matrix strip in CSR format
 * @param ja - column indices of the non-zero elements of the input matrix strip
 * @param ia - row indices for the input matrix strip in CSR format
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiInitStripesDI(KmlScasolverTask **handle, int n, int nstripes, const int *stripeWidth,
    const int *stripeRow, const double * const *a, const int * const *ja, const int * const *ia, MPI_Comm comm);

/** init - without matrix.
 * @ingroup csi_di
 *
 * @param handle - SCAISS matrix handle
 * @param nstripes - the number of strips stored within the callee MPI process memory
 * @param stripeWidth - array of nstripes elements, i-th element is the number of rows in i-th matrix strip
 * @param comm - MPI communicator handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiInitWithoutMatDI(KmlScasolverTask **handle, int nstripes, const int *stripeWidth,
    MPI_Comm comm);

/** analyze[reserved for future] - analyze sparse matrix and change
 * storage representation if required.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiAnalyzeDI(KmlScasolverTask **handle);

/** factorize - compute preconditioner if required.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiFactorizeDI(KmlScasolverTask **handle);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (duplicated dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSolveDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** solve - solve system $Ax = b$ with `n-by-nb` matrices `x` and `b`.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @param nb - number of right-hand side vectors in `b`
 * @param x - (distributed dense matrix) solutions on output, initial guess on input
 * @param ldx - leading dimension of `x`
 * @param b - (distributed dense matrix) right hand side
 * @param ldb - leading dimension of `b`
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSolveDxDI(KmlScasolverTask **handle, int nb, double *x, int ldx, const double *b, int ldb);

/** clean - clean internal data structure.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiCleanDI(KmlScasolverTask **handle);

/** setup solver-agnostic user preconditioner.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided preconditioner computing $x=M^{-1}x$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSetUserPreconditionerDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, double *x));

/** setup solver-agnostic user spmv.
 * @ingroup csi_di
 *
 * @param handle - SCAISS handle
 * @param ustruct - user data
 * @param fptr - user-provided spmv computing $x=Ax$
 * @return a KmlSolverStatus value
 */
KML_EXPORT int KmlScaissCsiSetUserSpmvDI(KmlScasolverTask **handle, void *ustruct,
    int (*fptr)(void *ustruct, const double *x, double *y));

/** Set an integer parameter in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSetDII(KmlScasolverTask **handle, enum KmlSolverParam param, const int *data, int nd);

/** Set a floating point parameter in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSetDID(KmlScasolverTask **handle, enum KmlSolverParam param, const double *data, int nd);

/** Get an integer parameter in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiGetDII(KmlScasolverTask **handle, enum KmlSolverParam param, int *data, int nd);

/** Get a floating point parameter in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiGetDID(KmlScasolverTask **handle, enum KmlSolverParam param, double *data, int nd);

/** Set an integer parameter for preconditioner in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, const int *data,
    int nd);

/** Set a floating point parameter for preconditioner in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param,
    const double *data, int nd);
/** Set an integer parameter for subpreconditioner in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSubPcSetDII(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const int *data, int nd);

/** Set a floating point parameter for subpreconditioner in IssCsiDI handle
 * @ingroup csi_di
 *
 * @param handle SCAISS handle
 * @param param Selector of the parameter
 * @param subn Index of subpreconditioner
 * @param data Values of the parameter
 * @param nd Number of the values
 * @return Error code of the last error
 */
KML_EXPORT int KmlScaissCsiSubPcSetDID(KmlScasolverTask **handle, enum KmlSolverPreconditionerParam param, int subn,
    const double *data, int nd);
#endif // OMIT_CSI_DI

#if defined(__cplusplus)
}
#endif
// NOLINTEND

#endif // KML_SCAISS_H_INCLUDED
