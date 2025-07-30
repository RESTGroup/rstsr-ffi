/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 */

#ifndef KML_MG_API_H
#define KML_MG_API_H

#include <mg_types.h>
#include <mg_status.h>

#ifdef __cplusplus
extern "C" {
#endif

KmlMgStatus KmlMgMatrixCreate(KmlMgMatrixH *pMatrix, const KmlMgMatrixStore *store, const KmlMgMatrixOptions *options);
KmlMgStatus KmlMgMatrixSetValues(KmlMgMatrixH matrix, void *values);
void KmlMgMatrixDestroy(KmlMgMatrixH matrix);
KmlMgStatus KmlMgMatrixQuery(KmlMgMatrixH matrix, KmlMgMatrixInfo *info);

KmlMgStatus KmlMgSolverCreate(KmlMgSolverH *pSolver, MPI_Comm comm, const KmlMgSolverOptions *options);
KmlMgStatus KmlMgSolverSetPreconditioner(KmlMgSolverH solver, const KmlMgPreconditionerOptions *options);
KmlMgStatus KmlMgSolverSetPreconditionerMatrix(KmlMgSolverH solver, KmlMgMatrixH A);
KmlMgStatus KmlMgSolverAnalyze(KmlMgSolverH solver, KmlMgMatrixH A);
KmlMgStatus KmlMgSolverPrepare(KmlMgSolverH solver, KmlMgMatrixH A);
KmlMgStatus KmlMgSolverApply(KmlMgSolverH solver, KmlMgMatrixH B, KmlMgMatrixH X, const KmlMgApplyOptions *options);
void KmlMgSolverDestroy(KmlMgSolverH solver);
KmlMgStatus KmlMgSolverQuery(KmlMgSolverH solver, KmlMgSolverInfo *info);

#ifdef __cplusplus
}
#endif

#endif
