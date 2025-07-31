/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2023-2023. All rights reserved.
 */
#ifndef KES_H
#define KES_H

#include "kes_defs.h"

#include <mpi.h>
#include <cstdbool>

#ifdef __cplusplus
extern "C" {
#endif

KES_EXPORT KesReturnCode KesMatrixCreate(KesMatrix *matA,
                                         KesScalarType scalarType,
                                         KesMatrixFormat format);

KES_EXPORT KesReturnCode KesVectorCreate(KesVector *vecX,
                                         KesScalarType scalarType,
                                         KesVectorFormat format);

KES_EXPORT KesReturnCode KesSolverCreate(KesSolver *solver,
                                         KesScalarType scalarType,
                                         KesSolverAlgorithm solverAlgorithm);

KES_EXPORT KesReturnCode KesRunSolver(const KesSolver solver,
                                      const KesMatrix matA,
                                      const KesMatrix matB,
                                      const KesMatrix matP,
                                      KesVector vecX,
                                      KesVector vecD);

KES_EXPORT void KesMatrixDestroy(KesMatrix matA);

KES_EXPORT void KesVectorDestroy(KesVector vecX);

KES_EXPORT void KesSolverDestroy(KesSolver solver);

KES_EXPORT KesReturnCode KesMatrixSetDim(KesMatrix matA,
                                         int nRows,
                                         int nCols);

KES_EXPORT KesReturnCode KesMatrixSetShellMatVec(KesMatrix matA,
                                                 UserData userData,
                                                 UserMatVec userMatVec);

KES_EXPORT void *KesMatrixGetShellUserData(KesMatrix matA);

KES_EXPORT KesReturnCode KesVectorSetComm(KesVector vecX,
                                          KesCommDirection commDirection,
                                          MPI_Comm comm);

KES_EXPORT KesReturnCode KesVectorSetDim(KesVector vecX,
                                         int nRows,
                                         int nCols);

KES_EXPORT KesReturnCode KesVectorDenseSetValues(KesVector vecX,
                                                 void *values,
                                                 int ld);

KES_EXPORT void *KesVectorDenseGetValues(KesVector vecX);

KES_EXPORT KesReturnCode KesSolverSetProblemType(KesSolver solver,
                                                 KesProblemType problemType);

KES_EXPORT KesReturnCode KesSolverSetMaxIter(KesSolver solver,
                                             int maxIter);

KES_EXPORT KesReturnCode KesSolverSetMaxOrthoIter(KesSolver solver,
                                                  int maxOrthoIter);

KES_EXPORT KesReturnCode KesSolverSetThreshold(KesSolver solver,
                                               double threshold);

KES_EXPORT KesReturnCode KesSetDenseEigenSolverType(KesSolver solver,
                                                    KesDenseEigenSolverType denseEigenSolverType);

KES_EXPORT KesReturnCode KesSetConvergeDetectorType(KesSolver solver,
                                                    KesConvergeDetectorType convergeDetectorType);

KES_EXPORT KesReturnCode KesSetOrthogonalizerType(KesSolver solver,
                                                  KesOrthogonalizerType orthogonalizerType);

KES_EXPORT KesReturnCode KesSetOrthoProjectorType(KesSolver solver,
                                                  KesOrthoProjectorType orthoProjectorType);

KES_EXPORT KesReturnCode KesGetSolverIter(KesSolver solver,
                                          int *nIter);

KES_EXPORT KesReturnCode KesGetSolverConv(KesSolver solver,
                                          int *nConv);

KES_EXPORT KesReturnCode KesVectorReplicate(const KesVector vecX,
                                            KesVector *replicateX,
                                            bool toAllocateMem,
                                            bool toCopyData);

KES_EXPORT KesReturnCode KesEigResidual(const KesMatrix matA,
                                        const KesMatrix matB,
                                        const KesVector vecX,
                                        const KesVector vecD,
                                        KesVector vecR);

KES_EXPORT KesReturnCode KesVectorNorm(const KesVector vecX,
                                       KesVector normX);

#ifdef __cplusplus
}
#endif

#endif
