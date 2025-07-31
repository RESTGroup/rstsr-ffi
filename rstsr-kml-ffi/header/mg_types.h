/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 */

#ifndef KML_MG_TYPE_H
#define KML_MG_TYPE_H

#include <stdint.h>
#include <mpi.h>

#ifdef __cplusplus
extern "C" {
#endif

/* The i-th bit */
#define KML_MG_BIT(i) (1UL << (i))

typedef enum {
    KML_MG_VALUE_FP16 = 0, /**< half */
    KML_MG_VALUE_FP32, /**< float */
    KML_MG_VALUE_FP64, /**< double */
    KML_MG_VALUE_LAST,
} KmlMgValueType;

typedef enum {
    KML_MG_INDEX_INT32 = 0, /**< int32_t */
    KML_MG_INDEX_INT64, /**< int64_t */
    KML_MG_INDEX_LAST,
} KmlMgIndexType;

typedef enum {
    KML_MG_STENCIL_2D5 = 0, // 95232,     bin2dec('000000000010111010000000000')
    KML_MG_STENCIL_2D9,     // 261632,    bin2dec('000000000111111111000000000')
    KML_MG_STENCIL_3D7,     // 4289552,   bin2dec('000010000010111010000010000')
    KML_MG_STENCIL_3D15,    // 38534802,  bin2dec('010010010111111111010010010')
    KML_MG_STENCIL_3D19,    // 49020602,  bin2dec('010111010111111111010111010')
    KML_MG_STENCIL_3D27,    // 134217727, bin2dec('111111111111111111111111111')
    KML_MG_STENCIL_LAST,
} KmlMgStencilType;

const int KML_MG_STENCIL_DIAG_SIZE[6] = {5, 9, 7, 15, 19, 27};

typedef enum {
    KML_MG_SPARSE_AOS = 0,
    KML_MG_SPARSE_SOA,
    KML_MG_SPARSE_LAST,
} KmlMgSparseType;

typedef struct {
    void* values;
    int64_t numDim; /**< number of vectors */
    int is2D;
    int needCorner;
} KmlMgMatrixStoreDense;

typedef struct {
    void* values;
    KmlMgSparseType type;
    KmlMgStencilType stencil;
} KmlMgMatrixStoreSparse;

typedef enum {
    KML_MG_MATRIX_OPTIONS_INDEX_TYPE = KML_MG_BIT(0), /**< Index type */
    KML_MG_MATRIX_OPTIONS_VALUE_TYPE = KML_MG_BIT(1), /**< Value type */
} KmlMgMatrixOptionsField;

typedef struct {
    uint64_t fieldMask;
    KmlMgIndexType indexType;
    KmlMgValueType valueType;
} KmlMgMatrixOptions;

typedef enum {
    KML_MG_MATRIX_STORE_DENSE_ROW_MAJOR, /**< Dense(Vector) row-major */
    KML_MG_MATRIX_STORE_DENSE_COL_MAJOR, /**< Dense(Vector) col-major */
    KML_MG_MATRIX_STORE_DENSE_ROW_MAJOR_HALO, /**< Dense(Vector) row-major with halo space */
    KML_MG_MATRIX_STORE_DENSE_COL_MAJOR_HALO, /**< Dense(Vector) col-major with halo space */
    KML_MG_MATRIX_STORE_SPARSE, /**< Sparse(Matrix) */
    KML_MG_MATRIX_STORE_SPARSE_HALO, /**< Sparse(Matrix) with halo space */
    KML_MG_MATRIX_STORE_LAST,
} KmlMgMatrixStoreType;

typedef struct {
    MPI_Comm comm;
    int64_t globalx;
    int64_t globaly;
    int64_t globalz;
    int64_t procx;
    int64_t procy;
    int64_t procz;
    int64_t xbeg;
    int64_t xend;
    int64_t ybeg;
    int64_t yend;
    int64_t zbeg;
    int64_t zend;
    int64_t xhalo;
    int64_t yhalo;
    int64_t zhalo;
    int crossPolar;
    KmlMgMatrixStoreType type;
    union {
        KmlMgMatrixStoreDense dense;
        KmlMgMatrixStoreSparse sparse;
    };
} KmlMgMatrixStore;

typedef enum {
    KML_MG_MATRIX_INFO_STORE = KML_MG_BIT(0),
} KmlMgMatrixInfoField;

typedef struct {
    uint64_t fieldMask;
    KmlMgMatrixStore store;
} KmlMgMatrixInfo;

typedef enum {
    KML_MG_SMOOTHER_POINT_JACOBI,
    KML_MG_SMOOTHER_POINT_GS,
    KML_MG_SMOOTHER_LINE_JACOBI,
    KML_MG_SMOOTHER_LINE_GS,
    KML_MG_SMOOTHER_PLANE_ILU,
    KML_MG_SMOOTHER_SHUFFLE_PLANE_ILU,
    KML_MG_SMOOTHER_BLOCK_ILU,
    KML_MG_SMOOTHER_ADI,
    KML_MG_SMOOTHER_BLOCK_LU,
    KML_MG_SMOOTHER_LU,
    KML_MG_SMOOTHER_CUSTOM,
    KML_MG_SMOOTHER_LAST,
} KmlMgSmootherType;

typedef enum {
    KML_MG_CYCLE_V,
    KML_MG_CYCLE_W,
    KML_MG_CYCLE_LAST,
} KmlMgCycleType;

typedef enum {
    KML_MG_COARSEN_FULL_XYZ,
    KML_MG_COARSEN_SEMI_XY,
    KML_MG_COARSEN_SEMI_XZ,
    KML_MG_COARSEN_SEMI_YZ,
    KML_MG_COARSEN_SEMI_Z,
    KML_MG_COARSEN_FULL_XZ,
    KML_MG_COARSEN_LAST,
} KmlMgCoarsenType;

typedef enum {
    KML_MG_REST_INTE_VTX_1D1,
    KML_MG_REST_INTE_VTX_1D3,
    KML_MG_REST_INTE_VTX_2D1,
    KML_MG_REST_INTE_VTX_2D5,
    KML_MG_REST_INTE_VTX_2D9,
    KML_MG_REST_INTE_CELL_2D4,
    KML_MG_REST_INTE_CELL_2D16,
    KML_MG_REST_INTE_CELL_3D8,
    KML_MG_REST_INTE_CELL_3D64,
    KML_MG_REST_INTE_CELL_LAST,
} KmlMgRestInteType;

typedef enum {
    KML_MG_PRECONDITIONER_OPTIONS_INDEX_TYPE = KML_MG_BIT(0),
    KML_MG_PRECONDITIONER_OPTIONS_KSP_VALUE_TYPE = KML_MG_BIT(1),
    KML_MG_PRECONDITIONER_OPTIONS_VALUE_TYPE = KML_MG_BIT(2),
    KML_MG_PRECONDITIONER_OPTIONS_CALC_TYPE = KML_MG_BIT(3),
    KML_MG_PRECONDITIONER_OPTIONS_NUM_LEVEL = KML_MG_BIT(4),
    KML_MG_PRECONDITIONER_OPTIONS_SHIFT_LEVEL = KML_MG_BIT(5),
    KML_MG_PRECONDITIONER_OPTIONS_CYCLE_TYPE = KML_MG_BIT(6),
    KML_MG_PRECONDITIONER_OPTIONS_SWEEP = KML_MG_BIT(7),
    KML_MG_PRECONDITIONER_OPTIONS_XPSS = KML_MG_BIT(8),
    KML_MG_PRECONDITIONER_OPTIONS_SMOOTHER = KML_MG_BIT(9),
    KML_MG_PRECONDITIONER_OPTIONS_WEIGHT = KML_MG_BIT(10),
    KML_MG_PRECONDITIONER_OPTIONS_COARSEN = KML_MG_BIT(11),
    KML_MG_PRECONDITIONER_OPTIONS_RESTRICT = KML_MG_BIT(12),
    KML_MG_PRECONDITIONER_OPTIONS_PROLONG = KML_MG_BIT(13),
    KML_MG_PRECONDITIONER_OPTIONS_ALPHA = KML_MG_BIT(14),
    KML_MG_PRECONDITIONER_OPTIONS_CONTROL = KML_MG_BIT(15),
} KmlMgPreconditionerOptionsField;

typedef struct {
    uint64_t fieldMask;
    KmlMgIndexType indexType;
    KmlMgValueType kspValueType;
    KmlMgValueType valueType;
    KmlMgValueType calcType;
    int64_t numLevels;
    int64_t shiftLevelId;
    KmlMgCycleType cycle;
    void* gridSweep; // type is indexType
    void* xpss; // X-Process Shrink Strides(X方向进程减少的步长), type is indexType
    KmlMgSmootherType* smoother;
    void* weight; // smoother weight, type is calcType
    KmlMgCoarsenType* coarsen; // 粗化类型
    KmlMgRestInteType* rest; // 限制算子
    KmlMgRestInteType* interp; // 插值算子
    void* alpha; // Gakerkin 算子调整的系数, type is calcType
    int* control;
} KmlMgPreconditionerOptions;

typedef enum {
    KML_MG_TOLERANCE_ABS,
    KML_MG_TOLERANCE_REL,
    KML_MG_TOLERANCE_LAST,
} KmlMgToleranceType;

typedef enum {
    KML_MG_SOLVER_NONE,
    KML_MG_SOLVER_BICG,
    KML_MG_SOLVER_CG,
    KML_MG_SOLVER_CR,
    KML_MG_SOLVER_GCR,
    KML_MG_SOLVER_GMRES,
    KML_MG_SOLVER_LAST,
} KmlMgSolverKsp;

typedef enum {
    KML_MG_SOLVER_OPTIONS_INDEX_TYPE = KML_MG_BIT(0), /**< Index type */
    KML_MG_SOLVER_OPTIONS_KSP_VALUE_TYPE = KML_MG_BIT(1), /**< Value type */
    KML_MG_SOLVER_OPTIONS_PC_VALUE_TYPE = KML_MG_BIT(2),
    KML_MG_SOLVER_OPTIONS_PC_CALC_TYPE = KML_MG_BIT(3),
    KML_MG_SOLVER_OPTIONS_KSP = KML_MG_BIT(4),
} KmlMgSolverOptionsField;

typedef struct {
    uint64_t fieldMask;
    KmlMgIndexType indexType;
    KmlMgValueType kspValueType;
    KmlMgValueType pcValueType;
    KmlMgValueType pcCalcType;
    KmlMgSolverKsp ksp;
} KmlMgSolverOptions;

typedef enum {
    KML_MG_APPLY_OPTIONS_USE_ZERO_GUESS = KML_MG_BIT(0),
    KML_MG_APPLY_OPTIONS_TOLERANCE_TYPE = KML_MG_BIT(1),
    KML_MG_APPLY_OPTIONS_TOLERANCE = KML_MG_BIT(2),
    KML_MG_APPLY_OPTIONS_MAX_ITERATION = KML_MG_BIT(3),
    KML_MG_APPLY_OPTIONS_RESTART_STEP = KML_MG_BIT(4),
    KML_MG_APPLY_OPTIONS_SOLVE_RESI = KML_MG_BIT(5),
} KmlMgApplyOptionsField;

typedef struct {
    uint64_t fieldMask;
    int useZeroGuess;
    KmlMgToleranceType toleranceType;
    double tolerance;
    int64_t maxIteration;
    int64_t restartStep; // only for GCR & GMRES
    int solveResi; // only for GCR
} KmlMgApplyOptions;

typedef enum {
    KML_MG_SOLVER_INFO_KSP_TYPE = KML_MG_BIT(0),
    KML_MG_SOLVER_INFO_USE_ZERO_GUESS = KML_MG_BIT(1),
    KML_MG_SOLVER_INFO_TOLERANCE_TYPE = KML_MG_BIT(2),
    KML_MG_SOLVER_INFO_TOLERANCE = KML_MG_BIT(3),
    KML_MG_SOLVER_INFO_MAX_ITERATION = KML_MG_BIT(4),
    KML_MG_SOLVER_INFO_RESTART_STEP = KML_MG_BIT(5),
    KML_MG_SOLVER_INFO_SOLVE_RESI = KML_MG_BIT(6),
} KmlMgSolverInfoField;

typedef struct {
    uint64_t fieldMask;
    KmlMgSolverKsp ksp;
    int useZeroGuess;
    KmlMgToleranceType toleranceType;
    double tolerance;
    int64_t maxIteration;
    int64_t restartStep; // only for GCR & GMRES
    int solveResi; // only for GCR
} KmlMgSolverInfo;

typedef struct KmlMgMatrix* KmlMgMatrixH;

typedef struct KmlMgSolver* KmlMgSolverH;

#ifdef __cplusplus
}
#endif

#endif
