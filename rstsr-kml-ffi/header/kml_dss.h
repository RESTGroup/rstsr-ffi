/*******************************************************************************
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2022. All rights reserved.
 * Description: Part of KML library
 * Author: KML
 * Create: 2020
 ******************************************************************************/

#ifndef KML_DSS_H_INCLUDED
#define KML_DSS_H_INCLUDED

#include <stdint.h>
#include "kml_solver_defs.h"
#include "kml_export.h"
#include "kml_service.h"

#if defined(__cplusplus)
extern "C" {
#endif

// NEW API ---------------------------------------------------------------------------------
 
typedef enum {
    KMLDSS_BWR_OFF,
    KMLDSS_BWR_FIXED_THREADS,
    KMLDSS_BWR_LAST,
} KmlDssBWRMode;
 
typedef enum {
    KMLDSS_MATCHING_OFF,
    KMLDSS_MATCHING_HUNGARIAN,
    KMLDSS_MATCHING_MC64,
    KMLDSS_MATCHING_SMC64,
    KMLDSS_MATCHING_LAST,
} KmlDssMatchingType;
 
typedef enum {
    KMLDSS_RDR_KRDR,
    KMLDSS_RDR_CUSTOM,
    KMLDSS_RDR_LAST,
} KmlDssRdrType;
 
// NOLINTNEXTLINE: Use strongly typed parameters and avoid 'void *'
typedef int (*KmlDssCustomRdrAlgo)(const KmlSolverMatrixStore *store, void *perm, void *iperm, void *arg);
 
typedef enum {
    KMLDSS_PIVOTING_OFF,
    KMLDSS_PIVOTING_LOCAL,
    KMLDSS_PIVOTING_LAST,
} KmlDssPivotingType;
 
typedef enum {
    KMLDSS_MATRIX_NO_TRANS,
    KMLDSS_MATRIX_TRANS,
    KMLDSS_MATRIX_LAST,
} KmlDssMatrixTransType;
 
typedef enum {
    KMLDSS_SOLVE_ALL,
    KMLDSS_SOLVE_FORWARD,
    KMLDSS_SOLVE_DIAGONAL,
    KMLDSS_SOLVE_BACKWARD,
    KMLDSS_SOLVE_REFINE,
    KMLDSS_SOLVE_LAST,
} KmlDssSolveStage;
 
typedef enum {
    KMLDSS_REFINE_OFF,
    KMLDSS_REFINE_CLASSICAL,
    KMLDSS_REFINE_LAST,
} KmlDssRefineMethod;
 
typedef enum {
    KMLDSS_INIT_OPTION_BWR_MODE = KMLSS_BIT(0),
    KMLDSS_INIT_OPTION_NTHREADS = KMLSS_BIT(1),
} KmlDssInitOptionField;
 
typedef struct {
    uint64_t fieldMask;
    KmlDssBWRMode bwrMode;
    int32_t nThreads;
} KmlDssInitOption;
 
typedef enum {
    KMLDSS_ANALYZE_OPTION_NTHREADS_ANALYZE = KMLSS_BIT(0),
    KMLDSS_ANALYZE_OPTION_NTHREADS_FACTORIZE = KMLSS_BIT(1),
    KMLDSS_ANALYZE_OPTION_NTHREADS_SOLVE = KMLSS_BIT(2),
    KMLDSS_ANALYZE_OPTION_MATCHING_TYPE = KMLSS_BIT(3),
    KMLDSS_ANALYZE_OPTION_NTHREADS_MATCHING = KMLSS_BIT(4),
    KMLDSS_ANALYZE_OPTION_RDR_TYPE = KMLSS_BIT(5),
    KMLDSS_ANALYZE_OPTION_NTHREADS_RDR = KMLSS_BIT(6),
    KMLDSS_ANALYZE_OPTION_CUSTOM_RDR_ALGO = KMLSS_BIT(7),
    KMLDSS_ANALYZE_OPTION_CUSTOM_RDR_ARGS = KMLSS_BIT(8),
    KMLDSS_ANALYZE_OPTION_RDR_PERM = KMLSS_BIT(9),
    KMLDSS_ANALYZE_OPTION_PIVOTING_TYPE = KMLSS_BIT(10),
    KMLDSS_ANALYZE_OPTION_PIVOTING_THRESHOLD = KMLSS_BIT(11),
    KMLDSS_ANALYZE_OPTION_BATCH_NRHS = KMLSS_BIT(12),
    KMLDSS_ANALYZE_OPTION_SCHUR_SIZE = KMLSS_BIT(13),
    KMLDSS_ANALYZE_OPTION_SCHUR_FORMAT = KMLSS_BIT(14),
    KMLDSS_ANALYZE_OPTION_MATIRX_TRANS_TYPE = KMLSS_BIT(15),
} KmlDssAnalyzeOptionField;
 
typedef struct {
    uint64_t fieldMask;
    int32_t nThreadsAnalyze;
    int32_t nThreadsFactorize;
    int32_t nThreadsSolve;
    KmlDssMatchingType matchingType;
    int32_t nThreadsMatching;
    KmlDssRdrType rdrType;
    int32_t nThreadsRdr;
    KmlDssCustomRdrAlgo customRdrAlgo; // NOLINT: avoid 'void *'
    void *customRdrArgs;               // NOLINT: avoid 'void *'
    void *rdrPerm;                     // NOLINT: avoid 'void *'
    KmlDssPivotingType pivotingType;
    double pivotingThreshold;
    int64_t batchNRHS;
    int64_t schurSize;
    KmlSolverMatrixStoreFormat schurFormat;
    KmlDssMatrixTransType matrixTransType;
} KmlDssAnalyzeOption;
 
typedef enum {
    KMLDSS_FACTORIZE_OPTION_PERTURBATION_THRESHOLD = KMLSS_BIT(0),
} KmlDssFactorizeOptionField;
 
typedef struct {
    uint64_t fieldMask;
    double perturbationThreshold;
} KmlDssFactorizeOption;
 
typedef enum {
    KMLDSS_SOLVE_OPTION_SOLVE_STAGE = KMLSS_BIT(0),
    KMLDSS_SOLVE_OPTION_REFINE_METHOD = KMLSS_BIT(1),
    KMLDSS_SOLVE_OPTION_REFINE_MAX_STEP = KMLSS_BIT(2),
    KMLDSS_SOLVE_OPTION_REFINE_TOLERANCE = KMLSS_BIT(3),
} KmlDssSolveOptionField;
 
typedef struct {
    uint64_t fieldMask;
    KmlDssSolveStage stage;
    KmlDssRefineMethod refineMethod;
    int32_t refineMaxSteps;
    double refineTolerance;
} KmlDssSolveOption;
 
typedef enum {
    KMLDSS_INFO_RDR_PERM = KMLSS_BIT(0),
    KMLDSS_INFO_SCHUR_NNZ = KMLSS_BIT(1),
    KMLDSS_INFO_SCHUR_MAT = KMLSS_BIT(2),
    KMLDSS_INFO_REFINE_STEPS = KMLSS_BIT(3),
    KMLDSS_INFO_PEAK_MEM = KMLSS_BIT(4),
    KMLDSS_INFO_FILL_IN = KMLSS_BIT(5),
    KMLDSS_INFO_FILL_IN_L = KMLSS_BIT(6),
    KMLDSS_INFO_FILL_IN_U = KMLSS_BIT(7),
} KmlDssInfoField;
 
typedef struct {
    uint64_t fieldMask;
    void *rdrPerm;
    int64_t schurNnz;
    KmlSolverMatrixStore *schurMat;
    int32_t refineSteps;
    int64_t peakMem;
    int64_t nFillIn;
    int64_t nFillInL;
    int64_t nFillInU;
} KmlDssInfo;
 
typedef struct KmlDssSolver KmlDssSolver;

KML_EXPORT int KmlDssInit(KmlDssSolver **pSolver, const KmlDssInitOption *option);
KML_EXPORT int KmlDssAnalyze(KmlDssSolver *solver, const KmlSolverMatrix *matrix, const KmlDssAnalyzeOption *option);
KML_EXPORT int KmlDssFactorize(KmlDssSolver *solver, const KmlSolverMatrix *matrix,
    const KmlDssFactorizeOption *option);
KML_EXPORT int KmlDssSolve(KmlDssSolver *solver, const KmlSolverMatrix *b, KmlSolverMatrix *x,
    const KmlDssSolveOption *option);
KML_EXPORT int KmlDssClean(KmlDssSolver **pSolver);
KML_EXPORT int KmlDssQuery(KmlDssSolver *solver, KmlDssInfo *info);

// DEPRECATED API --------------------------------------------------------------------------

/* Init DSS handle for sparse matrix */
#define DECLARE_INIT(name, Atype, Itype)                                                     \
    KML_DEPRECATED_EXPORT int name(/**/                                                      \
        KmlSolverTask **pHandle,   /* Pointer to DSS solver handle placeholder */            \
        Itype n,                   /* Matrix A size */                                       \
        Atype *a,                  /* Elements of matrix A in CSR format */                  \
        Itype *ja,                 /* Columns for each elements of matrix A in CSR format */ \
        Itype *ia                  /* Index of each row of matrix A in CSR format */         \
    )

#define DECLARE_INIT_NO_EXPORT(name, Atype, Itype)                                                     \
    KML_NO_EXPORT int name(/**/                                                      \
        KmlSolverTask **pHandle,   /* Pointer to DSS solver handle placeholder */            \
        Itype n,                   /* Matrix A size */                                       \
        Atype *a,                  /* Elements of matrix A in CSR format */                  \
        Itype *ja,                 /* Columns for each elements of matrix A in CSR format */ \
        Itype *ia                  /* Index of each row of matrix A in CSR format */         \
    )

DECLARE_INIT_NO_EXPORT(KmlDssSpdInitSI, float, int);
DECLARE_INIT_NO_EXPORT(KmlDssSpdInitDI, double, int);
DECLARE_INIT_NO_EXPORT(KmlDssHpdInitCI, kml_complex_float, int);
DECLARE_INIT_NO_EXPORT(KmlDssHpdInitZI, kml_complex_double, int);
DECLARE_INIT(KmlDssSymInitSI, float, int);
DECLARE_INIT(KmlDssSymInitDI, double, int);
DECLARE_INIT(KmlDssSymInitCI, kml_complex_float, int);
DECLARE_INIT(KmlDssSymInitZI, kml_complex_double, int);
DECLARE_INIT(KmlDssGenInitSI, float, int);
DECLARE_INIT(KmlDssGenInitDI, double, int);
DECLARE_INIT(KmlDssGenInitCI, kml_complex_float, int);
DECLARE_INIT(KmlDssGenInitZI, kml_complex_double, int);

#undef DECLARE_INIT

// -----------------------------------------------------------------------------

/* Analyze sparse matrix */
KML_NO_EXPORT int KmlDssSpdAnalyzeSI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssSpdAnalyzeDI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdAnalyzeCI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdAnalyzeZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymAnalyzeSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymAnalyzeDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymAnalyzeCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymAnalyzeZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenAnalyzeSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenAnalyzeDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenAnalyzeCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenAnalyzeZI(KmlSolverTask **pHandle);

// -----------------------------------------------------------------------------

/* Factorize sparse matrix */
KML_NO_EXPORT int KmlDssSpdFactorizeSI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssSpdFactorizeDI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdFactorizeCI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdFactorizeZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymFactorizeSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymFactorizeDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymFactorizeCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymFactorizeZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenFactorizeSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenFactorizeDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenFactorizeCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenFactorizeZI(KmlSolverTask **pHandle);
// -----------------------------------------------------------------------------

/* Solve factorized sparse matrix */
#define DECLARE_SOLVE(name, Atype, Itype)                           \
    KML_DEPRECATED_EXPORT int name(/**/                             \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */      \
        Itype nb,                  /* Number of right-hand sides */ \
        Atype *x,                  /* Solutions */                  \
        Itype ldx,                 /* Leading dimension of x */     \
        const Atype *b,            /* Right-hand sides */           \
        Itype ldb                  /* Leading dimension of b */     \
    )

#define DECLARE_SOLVE_NO_EXPORT(name, Atype, Itype)                           \
    KML_NO_EXPORT int name(/**/                             \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */      \
        Itype nb,                  /* Number of right-hand sides */ \
        Atype *x,                  /* Solutions */                  \
        Itype ldx,                 /* Leading dimension of x */     \
        const Atype *b,            /* Right-hand sides */           \
        Itype ldb                  /* Leading dimension of b */     \
    )

// NOLINTBEGIN: function exceeds recommended size/complexity thresholds
DECLARE_SOLVE_NO_EXPORT(KmlDssSpdSolveSI, float, int);
DECLARE_SOLVE_NO_EXPORT(KmlDssSpdSolveDI, double, int);
DECLARE_SOLVE_NO_EXPORT(KmlDssHpdSolveCI, kml_complex_float, int);
DECLARE_SOLVE_NO_EXPORT(KmlDssHpdSolveZI, kml_complex_double, int);
DECLARE_SOLVE(KmlDssSymSolveSI, float, int);
DECLARE_SOLVE(KmlDssSymSolveDI, double, int);
DECLARE_SOLVE(KmlDssSymSolveCI, kml_complex_float, int);
DECLARE_SOLVE(KmlDssSymSolveZI, kml_complex_double, int);
DECLARE_SOLVE(KmlDssGenSolveSI, float, int);
DECLARE_SOLVE(KmlDssGenSolveDI, double, int);
DECLARE_SOLVE(KmlDssGenSolveCI, kml_complex_float, int);
DECLARE_SOLVE(KmlDssGenSolveZI, kml_complex_double, int);
// NOLINTEND: function exceeds recommended size/complexity thresholds

#undef DECLARE_SOLVE

// -----------------------------------------------------------------------------

/* Clean handle */
KML_NO_EXPORT int KmlDssSpdCleanSI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssSpdCleanDI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdCleanCI(KmlSolverTask **pHandle);
KML_NO_EXPORT int KmlDssHpdCleanZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymCleanSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymCleanDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymCleanCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssSymCleanZI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenCleanSI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenCleanDI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenCleanCI(KmlSolverTask **pHandle);
KML_DEPRECATED_EXPORT int KmlDssGenCleanZI(KmlSolverTask **pHandle);

// -----------------------------------------------------------------------------

/* Get an parameter for matrix */
#define DECLARE_GET(name, Atype, Itype)                                                                              \
    KML_DEPRECATED_EXPORT int name(/**/                                                                              \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */                                                       \
        enum KmlSolverParam param, /* Parameter enum */                                                              \
        Atype *value,              /* Returned value of parameter */                                                 \
        Itype nvalue               /* Capacity of `value` array in elements (ignored if `param` refers to scalar) */ \
    )

#define DECLARE_GET_NO_EXPORT(name, Atype, Itype)                                                                   \
    KML_NO_EXPORT int name(/**/                                                                                      \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */                                                       \
        enum KmlSolverParam param, /* Parameter enum */                                                              \
        Atype *value,              /* Returned value of parameter */                                                 \
        Itype nvalue               /* Capacity of `value` array in elements (ignored if `param` refers to scalar) */ \
    )

DECLARE_GET_NO_EXPORT(KmlDssSpdGetSIL, int64_t, int);
DECLARE_GET_NO_EXPORT(KmlDssSpdGetDIL, int64_t, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetCIL, int64_t, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetZIL, int64_t, int);
DECLARE_GET(KmlDssSymGetSIL, int64_t, int);
DECLARE_GET(KmlDssSymGetDIL, int64_t, int);
DECLARE_GET(KmlDssSymGetCIL, int64_t, int);
DECLARE_GET(KmlDssSymGetZIL, int64_t, int);
DECLARE_GET(KmlDssGenGetSIL, int64_t, int);
DECLARE_GET(KmlDssGenGetDIL, int64_t, int);
DECLARE_GET(KmlDssGenGetCIL, int64_t, int);
DECLARE_GET(KmlDssGenGetZIL, int64_t, int);

DECLARE_GET_NO_EXPORT(KmlDssSpdGetSII, int, int);
DECLARE_GET_NO_EXPORT(KmlDssSpdGetDII, int, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetCII, int, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetZII, int, int);
DECLARE_GET(KmlDssSymGetSII, int, int);
DECLARE_GET(KmlDssSymGetDII, int, int);
DECLARE_GET(KmlDssSymGetCII, int, int);
DECLARE_GET(KmlDssSymGetZII, int, int);
DECLARE_GET(KmlDssGenGetSII, int, int);
DECLARE_GET(KmlDssGenGetDII, int, int);
DECLARE_GET(KmlDssGenGetCII, int, int);
DECLARE_GET(KmlDssGenGetZII, int, int);

DECLARE_GET_NO_EXPORT(KmlDssSpdGetDID, double, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetZID, double, int);
DECLARE_GET(KmlDssSymGetDID, double, int);
DECLARE_GET(KmlDssSymGetZID, double, int);
DECLARE_GET(KmlDssGenGetDID, double, int);
DECLARE_GET(KmlDssGenGetZID, double, int);

DECLARE_GET_NO_EXPORT(KmlDssSpdGetSIS, float, int);
DECLARE_GET_NO_EXPORT(KmlDssHpdGetCIS, float, int);
DECLARE_GET(KmlDssSymGetSIS, float, int);
DECLARE_GET(KmlDssSymGetCIS, float, int);
DECLARE_GET(KmlDssGenGetSIS, float, int);
DECLARE_GET(KmlDssGenGetCIS, float, int);

#undef DECLARE_GET

// -----------------------------------------------------------------------------

/* Set an integer parameter for matrix */
#define DECLARE_SET(name, Atype, Itype)                                                                             \
    KML_DEPRECATED_EXPORT int name(/**/                                                                             \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */                                                      \
        enum KmlSolverParam param, /* Selected parameter to set */                                                  \
        const Atype *data,         /* Pointer to data */                                                            \
        Itype ndata                /* Capacity of `data` array in elements (ignored if `param` refers to scalar) */ \
    )

#define DECLARE_SET_NO_EXPORT(name, Atype, Itype)                                                                     \
    KML_NO_EXPORT int name(/**/                                                                             \
        KmlSolverTask **pHandle,   /* Pointer to DSS handle */                                                      \
        enum KmlSolverParam param, /* Selected parameter to set */                                                  \
        const Atype *data,         /* Pointer to data */                                                            \
        Itype ndata                /* Capacity of `data` array in elements (ignored if `param` refers to scalar) */ \
    )

DECLARE_SET_NO_EXPORT(KmlDssSpdSetSII, int, int);
DECLARE_SET_NO_EXPORT(KmlDssSpdSetDII, int, int);
DECLARE_SET_NO_EXPORT(KmlDssHpdSetCII, int, int);
DECLARE_SET_NO_EXPORT(KmlDssHpdSetZII, int, int);
DECLARE_SET(KmlDssSymSetSII, int, int);
DECLARE_SET(KmlDssSymSetDII, int, int);
DECLARE_SET(KmlDssSymSetCII, int, int);
DECLARE_SET(KmlDssSymSetZII, int, int);
DECLARE_SET(KmlDssGenSetSII, int, int);
DECLARE_SET(KmlDssGenSetDII, int, int);
DECLARE_SET(KmlDssGenSetCII, int, int);
DECLARE_SET(KmlDssGenSetZII, int, int);

DECLARE_SET_NO_EXPORT(KmlDssSpdSetDID, double, int);
DECLARE_SET_NO_EXPORT(KmlDssHpdSetZID, double, int);
DECLARE_SET(KmlDssSymSetDID, double, int);
DECLARE_SET(KmlDssSymSetZID, double, int);
DECLARE_SET(KmlDssGenSetDID, double, int);
DECLARE_SET(KmlDssGenSetZID, double, int);

DECLARE_SET_NO_EXPORT(KmlDssSpdSetSIS, float, int);
DECLARE_SET_NO_EXPORT(KmlDssHpdSetCIS, float, int);
DECLARE_SET(KmlDssSymSetSIS, float, int);
DECLARE_SET(KmlDssSymSetCIS, float, int);
DECLARE_SET(KmlDssGenSetSIS, float, int);
DECLARE_SET(KmlDssGenSetCIS, float, int);

#undef DECLARE_SET

// -----------------------------------------------------------------------------

#if defined(__cplusplus)
}
#endif

#endif // KML_DSS_H_INCLUDED
