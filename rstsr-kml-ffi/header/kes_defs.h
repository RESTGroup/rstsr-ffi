/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 */
#ifndef KES_DEFS_H
#define KES_DEFS_H

#include "kes_macros.h"

#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    KES_GOOD = 0,
    KES_INVALID_PARAMETER = -1,
    KES_INVALID_RANGE = -2,
    KES_OUT_OF_MEMORY = -3,
    KES_NOT_SUPPORTED_YET = -4,
    KES_INVALID_TYPE = -5,
    KES_INVALID_CALL = -6,
    KES_NULL_PTR = -7,
} KesReturnCode;

typedef enum {
    KES_SCALAR_FP64 = 0,
    KES_SCALAR_FP32 = 1,
    KES_SCALAR_FP64C = 2,
    KES_SCALAR_FP32C = 3,
} KesScalarType;

typedef enum {
    KES_STANDARD = 0,
    KES_GENERALIZED = 1,
} KesProblemType;

typedef enum {
    KES_EMPTY_MATRIX = 0,
    KES_SHELL = 1,
} KesMatrixFormat;

typedef enum {
    KES_EMPTY_VECTOR = 0,
    KES_DENSE = 1,
} KesVectorFormat;

typedef enum {
    KES_LOBPCG = 0,
    KES_LOBPCG_BATCH = 1,
    KES_PPCG = 2,
} KesSolverAlgorithm;

typedef enum {
    KES_RESIDUALS = 0,
    KES_RELATIIVE_RESIDUALS = 1,
    KES_EIGENVALUES = 2,
    KES_RESIDUALS_OR_EIGENVALUES = 3,
} KesConvergeDetectorType;

typedef enum {
    KES_GRAM_SCHMIDT = 0,
} KesOrthoProjectorType;

typedef enum {
    KES_CHOLESKY_QR = 0,
    KES_SVD_QR = 1,
} KesOrthogonalizerType;

typedef enum {
    KES_LAPACK = 0,
    KES_SCALAPACK = 1,
} KesDenseEigenSolverType;

typedef enum {
    KES_LAPACK_CHOL = 0,
    KES_SCALAPACK_CHOL = 1,
} KesCholeskyFactorizerType;

typedef enum {
    KES_ROW = 0,
    KES_COL = 1,
} KesCommDirection;

/*
    opague object handle to a KES MATRIX
*/
typedef struct KesMatrixBase *KesMatrix;

/*
    opague object handle to a KES VECTOR
*/
typedef struct KesVectorBase *KesVector;

/*
    opague object handle to a KES SOLVER
*/
typedef struct KesSolverBase *KesSolver;

/*
    user data pointer
*/
typedef void *UserData;

/*
    user MatVec pointer
*/
typedef void (*UserMatVec)(void *, void *, void *, int, int, int);

#ifdef __cplusplus
}
#endif
#endif