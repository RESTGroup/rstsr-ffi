/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020-2024. All rights reserved.
 * Description: KML function
 * Author: KML
 * Create: 2020
 */
#ifndef KLAPACK_EXTERNAL_H
#define KLAPACK_EXTERNAL_H

#include "kml_service.h"

// NOLINTBEGIN: standard API
#ifdef __cplusplus
extern "C" {
#endif

KML_EXPORT void cgesdd_(const char *jobz, const int *m, const int *n, kml_complex_float *a, const int *lda, float *s,
    kml_complex_float *u, const int *ldu, kml_complex_float *vt, const int *ldvt, kml_complex_float *work,
    const int *lwork, float *rwork, int *iwork, int *info);

KML_EXPORT void dgesdd_(const char *jobz, const int *m, const int *n, double *a, const int *lda, double *s, double *u,
    const int *ldu, double *vt, const int *ldvt, double *work, const int *lwork, int *iwork, int *info);

KML_EXPORT void sgesdd_(const char *jobz, const int *m, const int *n, float *a, const int *lda, float *s, float *u,
    const int *ldu, float *vt, const int *ldvt, float *work, const int *lwork, int *iwork, int *info);

KML_EXPORT void zgesdd_(const char *jobz, const int *m, const int *n, kml_complex_double *a, const int *lda, double *s,
    kml_complex_double *u, const int *ldu, kml_complex_double *vt, const int *ldvt, kml_complex_double *work,
    const int *lwork, double *rwork, int *iwork, int *info);

KML_EXPORT void cgelsd_(const int *m, const int *n, const int *nrhs, kml_complex_float *a, const int *lda,
    kml_complex_float *b, const int *ldb, float *s, const float *rcond, int *rank, kml_complex_float *work,
    const int *lwork, float *rwork, int *iwork, int *info);

KML_EXPORT void dgelsd_(const int *m, const int *n, const int *nrhs, double *a, const int *lda, double *b,
    const int *ldb, double *s, const double *rcond, int *rank, double *work, const int *lwork, int *iwork, int *info);

KML_EXPORT void sgelsd_(const int *m, const int *n, const int *nrhs, float *a, const int *lda, float *b, const int *ldb,
    float *s, const float *rcond, int *rank, float *work, const int *lwork, int *iwork, int *info);

KML_EXPORT void zgelsd_(const int *m, const int *n, const int *nrhs, kml_complex_double *a, const int *lda,
    kml_complex_double *b, const int *ldb, double *s, const double *rcond, int *rank, kml_complex_double *work,
    const int *lwork, double *rwork, int *iwork, int *info);

KML_EXPORT void cppsv_(const char *uplo, const int *n, const int *nrhs, kml_complex_float *ap, kml_complex_float *b,
    const int *ldb, int *info);

KML_EXPORT void dppsv_(const char *uplo, const int *n, const int *nrhs, double *ap, double *b, const int *ldb,
    int *info);

KML_EXPORT void sppsv_(const char *uplo, const int *n, const int *nrhs, float *ap, float *b, const int *ldb, int *info);

KML_EXPORT void zppsv_(const char *uplo, const int *n, const int *nrhs, kml_complex_double *ap, kml_complex_double *b,
    const int *ldb, int *info);

KML_EXPORT void cgetrf_(const int *m, const int *n, kml_complex_float *a, const int *lda, int *ipiv, int *info);

KML_EXPORT void dgetrf_(const int *m, const int *n, double *a, const int *lda, int *ipiv, int *info);

KML_EXPORT void sgetrf_(const int *m, const int *n, float *a, const int *lda, int *ipiv, int *info);

KML_EXPORT void zgetrf_(const int *m, const int *n, kml_complex_double *a, const int *lda, int *ipiv, int *info);

KML_EXPORT void cgetri_(const int *n, kml_complex_float *a, const int *lda, const int *ipiv, kml_complex_float *work,
    const int *lwork, int *info);

KML_EXPORT void dgetri_(const int *n, double *a, const int *lda, const int *ipiv, double *work, const int *lwork,
    int *info);

KML_EXPORT void sgetri_(const int *n, float *a, const int *lda, const int *ipiv, float *work, const int *lwork,
    int *info);

KML_EXPORT void zgetri_(const int *n, kml_complex_double *a, const int *lda, const int *ipiv, kml_complex_double *work,
    const int *lwork, int *info);

KML_EXPORT void cgetrs_(const char *trans, const int *n, const int *nrhs, const kml_complex_float *a, const int *lda,
    const int *ipiv, kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dgetrs_(const char *trans, const int *n, const int *nrhs, const double *a, const int *lda,
    const int *ipiv, double *b, const int *ldb, int *info);

KML_EXPORT void sgetrs_(const char *trans, const int *n, const int *nrhs, const float *a, const int *lda,
    const int *ipiv, float *b, const int *ldb, int *info);

KML_EXPORT void zgetrs_(const char *trans, const int *n, const int *nrhs, const kml_complex_double *a, const int *lda,
    const int *ipiv, kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void cgeqrf_(const int *m, const int *n, kml_complex_float *a, const int *lda, kml_complex_float *tau,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dgeqrf_(const int *m, const int *n, double *a, const int *lda, double *tau, double *work,
    const int *lwork, int *info);

KML_EXPORT void sgeqrf_(const int *m, const int *n, float *a, const int *lda, float *tau, float *work, const int *lwork,
    int *info);

KML_EXPORT void zgeqrf_(const int *m, const int *n, kml_complex_double *a, const int *lda, kml_complex_double *tau,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cgelqf_(const int *m, const int *n, kml_complex_float *a, const int *lda, kml_complex_float *tau,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dgelqf_(const int *m, const int *n, double *a, const int *lda, double *tau, double *work,
    const int *lwork, int *info);

KML_EXPORT void sgelqf_(const int *m, const int *n, float *a, const int *lda, float *tau, float *work, const int *lwork,
    int *info);

KML_EXPORT void zgelqf_(const int *m, const int *n, kml_complex_double *a, const int *lda, kml_complex_double *tau,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cgerqf_(const int *m, const int *n, kml_complex_float *a, const int *lda, kml_complex_float *tau,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dgerqf_(const int *m, const int *n, double *a, const int *lda, double *tau, double *work,
    const int *lwork, int *info);

KML_EXPORT void sgerqf_(const int *m, const int *n, float *a, const int *lda, float *tau, float *work, const int *lwork,
    int *info);

KML_EXPORT void zgerqf_(const int *m, const int *n, kml_complex_double *a, const int *lda, kml_complex_double *tau,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cgeqlf_(const int *m, const int *n, kml_complex_float *a, const int *lda, kml_complex_float *tau,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dgeqlf_(const int *m, const int *n, double *a, const int *lda, double *tau, double *work,
    const int *lwork, int *info);

KML_EXPORT void sgeqlf_(const int *m, const int *n, float *a, const int *lda, float *tau, float *work, const int *lwork,
    int *info);

KML_EXPORT void zgeqlf_(const int *m, const int *n, kml_complex_double *a, const int *lda, kml_complex_double *tau,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void sgels_(const char *trans, const int *m, const int *n, const int *nrhs, float *a, const int *lda,
    float *b, const int *ldb, float *work, const int *lwork, int *info);

KML_EXPORT void dgels_(const char *trans, const int *m, const int *n, const int *nrhs, double *a, const int *lda,
    double *b, const int *ldb, double *work, const int *lwork, int *info);

KML_EXPORT void cgels_(const char *trans, const int *m, const int *n, const int *nrhs, kml_complex_float *a,
    const int *lda, kml_complex_float *b, const int *ldb, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zgels_(const char *trans, const int *m, const int *n, const int *nrhs, kml_complex_double * const a,
    const int *lda, kml_complex_double *b, const int *ldb, kml_complex_double * const work, const int *lwork,
    int *info);

KML_EXPORT void sgelss_(const int *m, const int *n, const int *nrhs, float *a, const int *lda, float *b, const int *ldb,
    float *s, const float *rcond, int *rank, float *work, const int *lwork, int *info);

KML_EXPORT void dgelss_(const int *m, const int *n, const int *nrhs, double *a, const int *lda, double *b,
    const int *ldb, double *s, const double *rcond, int *rank, double *work, const int *lwork, int *info);

KML_EXPORT void cgelss_(const int *m, const int *n, const int *nrhs, kml_complex_float *a, const int *lda,
    kml_complex_float *b, const int *ldb, float *s, const float *rcond, int *rank, kml_complex_float *work,
    const int *lwork, float *rwork, int *info);

KML_EXPORT void zgelss_(const int *m, const int *n, const int *nrhs, kml_complex_double *a, const int *lda,
    kml_complex_double *b, const int *ldb, double *s, const double *rcond, int *rank, kml_complex_double *work,
    const int *lwork, double *rwork, int *info);

KML_EXPORT void sorgtr_(const char *uplo, const int *n, float *a, const int *lda, const float *tau, float *work,
    const int *lwork, int *info);

KML_EXPORT void dorgtr_(const char *uplo, const int *n, double *a, const int *lda, const double *tau, double *work,
    const int *lwork, int *info);

KML_EXPORT void cungtr_(const char *uplo, const int *n, kml_complex_float *a, const int *lda,
    const kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zungtr_(const char *uplo, const int *n, kml_complex_double *a, const int *lda,
    const kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void sorglq_(const int *m, const int *n, const int *k, float *a, const int *lda, const float *tau,
    float *work, const int *lwork, int *info);

KML_EXPORT void dorglq_(const int *m, const int *n, const int *k, double *a, const int *lda, const double *tau,
    double *work, const int *lwork, int *info);

KML_EXPORT void cunglq_(const int *m, const int *n, const int *k, kml_complex_float *a, const int *lda,
    const kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunglq_(const int *m, const int *n, const int *k, kml_complex_double *a, const int *lda,
    const kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void sorgql_(const int *m, const int *n, const int *k, float *a, const int *lda, const float *tau,
    float *work, const int *lwork, int *info);

KML_EXPORT void dorgql_(const int *m, const int *n, const int *k, double *a, const int *lda, const double *tau,
    double *work, const int *lwork, int *info);

KML_EXPORT void cungql_(const int *m, const int *n, const int *k, kml_complex_float *a, const int *lda,
    const kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zungql_(const int *m, const int *n, const int *k, kml_complex_double *a, const int *lda,
    const kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void sorgqr_(const int *m, const int *n, const int *k, float *a, const int *lda, const float *tau,
    float *work, const int *lwork, int *info);

KML_EXPORT void dorgqr_(const int *m, const int *n, const int *k, double *a, const int *lda, const double *tau,
    double *work, const int *lwork, int *info);

KML_EXPORT void cungqr_(const int *m, const int *n, const int *k, kml_complex_float *a, const int *lda,
    const kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zungqr_(const int *m, const int *n, const int *k, kml_complex_double *a, const int *lda,
    const kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void sorgrq_(const int *m, const int *n, const int *k, float *a, const int *lda, const float *tau,
    float *work, const int *lwork, int *info);

KML_EXPORT void dorgrq_(const int *m, const int *n, const int *k, double *a, const int *lda, const double *tau,
    double *work, const int *lwork, int *info);

KML_EXPORT void cungrq_(const int *m, const int *n, const int *k, kml_complex_float *a, const int *lda,
    const kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zungrq_(const int *m, const int *n, const int *k, kml_complex_double *a, const int *lda,
    const kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cpptrf_(const char *uplo, const int *n, kml_complex_float *ap, int *info);

KML_EXPORT void dpptrf_(const char *uplo, const int *n, double *ap, int *info);

KML_EXPORT void spptrf_(const char *uplo, const int *n, float *ap, int *info);

KML_EXPORT void zpptrf_(const char *uplo, const int *n, kml_complex_double *ap, int *info);

KML_EXPORT void cpptrs_(const char *uplo, const int *n, const int *nrhs, const kml_complex_float *ap,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dpptrs_(const char *uplo, const int *n, const int *nrhs, const double *ap, double *b, const int *ldb,
    int *info);

KML_EXPORT void spptrs_(const char *uplo, const int *n, const int *nrhs, const float *ap, float *b, const int *ldb,
    int *info);

KML_EXPORT void zpptrs_(const char *uplo, const int *n, const int *nrhs, const kml_complex_double *ap,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void cpptri_(const char *uplo, const int *n, kml_complex_float *ap, int *info);

KML_EXPORT void dpptri_(const char *uplo, const int *n, double *ap, int *info);

KML_EXPORT void spptri_(const char *uplo, const int *n, float *ap, int *info);

KML_EXPORT void zpptri_(const char *uplo, const int *n, kml_complex_double *ap, int *info);

KML_EXPORT void dormqr_(const char *side, const char *trans, const int *m, const int *n, const int *k, const double *a,
    const int *lda, const double *tau, double *c, const int *ldc, double *work, const int *lwork, int *info);

KML_EXPORT void sormqr_(const char *side, const char *trans, const int *m, const int *n, const int *k, const float *a,
    const int *lda, const float *tau, float *c, const int *ldc, float *work, const int *lwork, int *info);

KML_EXPORT void dormlq_(const char *side, const char *trans, const int *m, const int *n, const int *k, const double *a,
    const int *lda, const double *tau, double *c, const int *ldc, double *work, const int *lwork, int *info);

KML_EXPORT void sormlq_(const char *side, const char *trans, const int *m, const int *n, const int *k, const float *a,
    const int *lda, const float *tau, float *c, const int *ldc, float *work, const int *lwork, int *info);

KML_EXPORT void dormql_(const char *side, const char *trans, const int *m, const int *n, const int *k, const double *a,
    const int *lda, const double *tau, double *c, const int *ldc, double *work, const int *lwork, int *info);

KML_EXPORT void sormql_(const char *side, const char *trans, const int *m, const int *n, const int *k, const float *a,
    const int *lda, const float *tau, float *c, const int *ldc, float *work, const int *lwork, int *info);

KML_EXPORT void dormrq_(const char *side, const char *trans, const int *m, const int *n, const int *k, const double *a,
    const int *lda, const double *tau, double *c, const int *ldc, double *work, const int *lwork, int *info);

KML_EXPORT void sormrq_(const char *side, const char *trans, const int *m, const int *n, const int *k, const float *a,
    const int *lda, const float *tau, float *c, const int *ldc, float *work, const int *lwork, int *info);

KML_EXPORT void cunmqr_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_float *a, const int *lda, const kml_complex_float *tau, kml_complex_float *c, const int *ldc,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunmqr_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_double *a, const int *lda, const kml_complex_double *tau, kml_complex_double *c, const int *ldc,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cunmlq_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_float *a, const int *lda, const kml_complex_float *tau, kml_complex_float *c, const int *ldc,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunmlq_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_double *a, const int *lda, const kml_complex_double *tau, kml_complex_double *c, const int *ldc,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cunmql_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_float *a, const int *lda, const kml_complex_float *tau, kml_complex_float *c, const int *ldc,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunmql_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_double *a, const int *lda, const kml_complex_double *tau, kml_complex_double *c, const int *ldc,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cunmrq_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_float *a, const int *lda, const kml_complex_float *tau, kml_complex_float *c, const int *ldc,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunmrq_(const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_double *a, const int *lda, const kml_complex_double *tau, kml_complex_double *c, const int *ldc,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cpotrf_(const char *uplo, const int *n, kml_complex_float *a, const int *lda, int *info);

KML_EXPORT void dpotrf_(const char *uplo, const int *n, double *a, const int *lda, int *info);

KML_EXPORT void spotrf_(const char *uplo, const int *n, float *a, const int *lda, int *info);

KML_EXPORT void zpotrf_(const char *uplo, const int *n, kml_complex_double *a, const int *lda, int *info);

KML_EXPORT float clange_(const char *norm, const int *m, const int *n, const kml_complex_float *a, const int *lda,
    float *work);

KML_EXPORT double dlange_(const char *norm, const int *m, const int *n, const double *a, const int *lda, double *work);

KML_EXPORT float slange_(const char *norm, const int *m, const int *n, const float *a, const int *lda, float *work);

KML_EXPORT double zlange_(const char *norm, const int *m, const int *n, const kml_complex_double *a, const int *lda,
    double *work);

KML_EXPORT void claset_(const char *uplo, const int *m, const int *n, const kml_complex_float *alpha,
    const kml_complex_float *beta, kml_complex_float *a, const int *lda);

KML_EXPORT void dlaset_(const char *uplo, const int *m, const int *n, const double *alpha, const double *beta,
    double *a, const int *lda);

KML_EXPORT void slaset_(const char *uplo, const int *m, const int *n, const float *alpha, const float *beta, float *a,
    const int *lda);

KML_EXPORT void zlaset_(const char *uplo, const int *m, const int *n, const kml_complex_double *alpha,
    const kml_complex_double *beta, kml_complex_double *a, const int *lda);

KML_EXPORT void clacpy_(const char *uplo, const int *m, const int *n, const kml_complex_float *a, const int *lda,
    kml_complex_float *b, const int *ldb);

KML_EXPORT void dlacpy_(const char *uplo, const int *m, const int *n, const double *a, const int *lda, double *b,
    const int *ldb);

KML_EXPORT void slacpy_(const char *uplo, const int *m, const int *n, const float *a, const int *lda, float *b,
    const int *ldb);

KML_EXPORT void zlacpy_(const char *uplo, const int *m, const int *n, const kml_complex_double *a, const int *lda,
    kml_complex_double *b, const int *ldb);

KML_EXPORT void claswp_(const int *n, kml_complex_float *a, const int *lda, const int *k1, const int *k2,
    const int *ipiv, const int *incx);

KML_EXPORT void dlaswp_(const int *n, double *a, const int *lda, const int *k1, const int *k2, const int *ipiv,
    const int *incx);

KML_EXPORT void slaswp_(const int *n, float *a, const int *lda, const int *k1, const int *k2, const int *ipiv,
    const int *incx);

KML_EXPORT void zlaswp_(const int *n, kml_complex_double *a, const int *lda, const int *k1, const int *k2,
    const int *ipiv, const int *incx);

KML_EXPORT void csptrf_(const char *uplo, const int *n, kml_complex_float *ap, int *ipiv, int *info);

KML_EXPORT void dsptrf_(const char *uplo, const int *n, double *ap, int *ipiv, int *info);

KML_EXPORT void ssptrf_(const char *uplo, const int *n, float *ap, int *ipiv, int *info);

KML_EXPORT void zsptrf_(const char *uplo, const int *n, kml_complex_double *ap, int *ipiv, int *info);

KML_EXPORT void chptrf_(const char *uplo, const int *n, kml_complex_float *ap, int *ipiv, int *info);

KML_EXPORT void zhptrf_(const char *uplo, const int *n, kml_complex_double *ap, int *ipiv, int *info);

KML_EXPORT void cgesv_(const int *n, const int *nrhs, kml_complex_float *a, const int *lda, int *ipiv,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dgesv_(const int *n, const int *nrhs, double *a, const int *lda, int *ipiv, double *b, const int *ldb,
    int *info);

KML_EXPORT void sgesv_(const int *n, const int *nrhs, float *a, const int *lda, int *ipiv, float *b, const int *ldb,
    int *info);

KML_EXPORT void zgesv_(const int *n, const int *nrhs, kml_complex_double *a, const int *lda, int *ipiv,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void cptsv_(const int *n, const int *nrhs, float *d, kml_complex_float *e, kml_complex_float *b,
    const int *ldb, int *info);

KML_EXPORT void dptsv_(const int *n, const int *nrhs, double *d, double *e, double *b, const int *ldb, int *info);

KML_EXPORT void sptsv_(const int *n, const int *nrhs, float *d, float *e, float *b, const int *ldb, int *info);

KML_EXPORT void zptsv_(const int *n, const int *nrhs, double *d, kml_complex_double *e, kml_complex_double *b,
    const int *ldb, int *info);

KML_EXPORT void cpttrf_(const int *n, float *d, kml_complex_float *e, int *info);

KML_EXPORT void dpttrf_(const int *n, double *d, double *e, int *info);

KML_EXPORT void spttrf_(const int *n, float *d, float *e, int *info);

KML_EXPORT void zpttrf_(const int *n, double *d, kml_complex_double *e, int *info);

KML_EXPORT void cpttrs_(const char *uplo, const int *n, const int *nrhs, const float *d, const kml_complex_float *e,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dpttrs_(const int *n, const int *nrhs, const double *d, const double *e, double *b, const int *ldb,
    int *info);

KML_EXPORT void spttrs_(const int *n, const int *nrhs, const float *d, const float *e, float *b, const int *ldb,
    int *info);

KML_EXPORT void zpttrs_(const char *uplo, const int *n, const int *nrhs, const double *d, const kml_complex_double *e,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void cptts2_(const int *iuplo, const int *n, const int *nrhs, const float *d, const kml_complex_float *e,
    kml_complex_float *b, const int *ldb);

KML_EXPORT void dptts2_(const int *n, const int *nrhs, const double *d, const double *e, double *b, const int *ldb);

KML_EXPORT void sptts2_(const int *n, const int *nrhs, const float *d, const float *e, float *b, const int *ldb);

KML_EXPORT void zptts2_(const int *uplo, const int *n, const int *nrhs, const double *d, const kml_complex_double *e,
    kml_complex_double *b, const int *ldb);

KML_EXPORT void cpotri_(const char *uplo, const int *n, kml_complex_float *a, const int *lda, int *info);

KML_EXPORT void dpotri_(const char *uplo, const int *n, double *a, const int *lda, int *info);

KML_EXPORT void spotri_(const char *uplo, const int *n, float *a, const int *lda, int *info);

KML_EXPORT void zpotri_(const char *uplo, const int *n, kml_complex_double *a, const int *lda, int *info);

KML_EXPORT void chetrd_(const char *uplo, const int *n, kml_complex_float *a, const int *lda, float *d, float *e,
    kml_complex_float *tau, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dsytrd_(const char *uplo, const int *n, double *a, const int *lda, double *d, double *e, double *tau,
    double *work, const int *lwork, int *info);

KML_EXPORT void ssytrd_(const char *uplo, const int *n, float *a, const int *lda, float *d, float *e, float *tau,
    float *work, const int *lwork, int *info);

KML_EXPORT void zhetrd_(const char *uplo, const int *n, kml_complex_double *a, const int *lda, double *d, double *e,
    kml_complex_double *tau, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cposv_(const char *uplo, const int *n, const int *nrhs, kml_complex_float *a, const int *lda,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dposv_(const char *uplo, const int *n, const int *nrhs, double *a, const int *lda, double *b,
    const int *ldb, int *info);

KML_EXPORT void sposv_(const char *uplo, const int *n, const int *nrhs, float *a, const int *lda, float *b,
    const int *ldb, int *info);

KML_EXPORT void zposv_(const char *uplo, const int *n, const int *nrhs, kml_complex_double *a, const int *lda,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void dsyev_(const char *jobz, const char *uplo, const int *n, double *a, const int *lda, double *w,
    double *work, const int *lwork, int *info);

KML_EXPORT void ssyev_(const char *jobz, const char *uplo, const int *n, float *a, const int *lda, float *w,
    float *work, const int *lwork, int *info);

KML_EXPORT void cheev_(const char *jobz, const char *uplo, const int *n, kml_complex_float *a, const int *lda, float *w,
    kml_complex_float *work, const int *lwork, float *rwork, int *info);

KML_EXPORT void zheev_(const char *jobz, const char *uplo, const int *n, kml_complex_double *a, const int *lda,
    double *w, kml_complex_double *work, const int *lwork, double *rwork, int *info);

KML_EXPORT void dsyevd_(const char *jobz, const char *uplo, const int *n, double *a, const int *lda, double *w,
    double *work, const int *lwork, int *iwork, const int *liwork, int *info);

KML_EXPORT void ssyevd_(const char *jobz, const char *uplo, const int *n, float *a, const int *lda, float *w,
    float *work, const int *lwork, int *iwork, const int *liwork, int *info);

KML_EXPORT void cheevd_(const char *jobz, const char *uplo, const int *n, kml_complex_float *a, const int *lda,
    float *w, kml_complex_float *work, const int *lwork, float *rwork, const int *lrwork, int *iwork, const int *liwork,
    int *info);

KML_EXPORT void zheevd_(const char *jobz, const char *uplo, const int *n, kml_complex_double *a, const int *lda,
    double *w, kml_complex_double *work, const int *lwork, double *rwork, const int *lrwork, int *iwork,
    const int *liwork, int *info);

KML_EXPORT void dstedc_(const char *compz, const int *n, double *d, double *e, double *z, const int *ldz, double *work,
    const int *lwork, int *iwork, const int *liwork, int *info);

KML_EXPORT void sstedc_(const char *compz, const int *n, float *d, float *e, float *z, const int *ldz, float *work,
    const int *lwork, int *iwork, const int *liwork, int *info);

KML_EXPORT void cstedc_(const char *compz, const int *n, float *d, float *e, kml_complex_float *z, const int *ldz,
    kml_complex_float *work, const int *lwork, float *rwork, const int *lrwork, int *iwork, const int *liwork,
    int *info);

KML_EXPORT void zstedc_(const char *compz, const int *n, double *d, double *e, kml_complex_double *z, const int *ldz,
    kml_complex_double *work, const int *lwork, double *rwork, const int *lrwork, int *iwork, const int *liwork,
    int *info);

KML_EXPORT void csteqr_(const char *compz, const int *n, float *d, float *e, kml_complex_float *z, const int *ldz,
    float *work, int *info);

KML_EXPORT void dsteqr_(const char *compz, const int *n, double *d, double *e, double *z, const int *ldz, double *work,
    int *info);

KML_EXPORT void ssteqr_(const char *compz, const int *n, float *d, float *e, float *z, const int *ldz, float *work,
    int *info);

KML_EXPORT void zsteqr_(const char *compz, const int *n, double *d, double *e, kml_complex_double *z, const int *ldz,
    double *work, int *info);

KML_EXPORT void cgttrf_(const int *n, kml_complex_float *dl, kml_complex_float *d, kml_complex_float *du,
    kml_complex_float *du2, int *ipiv, int *info);

KML_EXPORT void dgttrf_(const int *n, double *dl, double *d, double *du, double *du2, int *ipiv, int *info);

KML_EXPORT void sgttrf_(const int *n, float *dl, float *d, float *du, float *du2, int *ipiv, int *info);

KML_EXPORT void zgttrf_(const int *n, kml_complex_double *dl, kml_complex_double *d, kml_complex_double *du,
    kml_complex_double *du2, int *ipiv, int *info);

KML_EXPORT void cgttrs_(const char *trans, const int *n, const int *nrhs, const kml_complex_float *dl,
    const kml_complex_float *d, const kml_complex_float *du, const kml_complex_float *du2, const int *ipiv,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dgttrs_(const char *trans, const int *n, const int *nrhs, const double *dl, const double *d,
    const double *du, const double *du2, const int *ipiv, double *b, const int *ldb, int *info);

KML_EXPORT void sgttrs_(const char *trans, const int *n, const int *nrhs, const float *dl, const float *d,
    const float *du, const float *du2, const int *ipiv, float *b, const int *ldb, int *info);

KML_EXPORT void zgttrs_(const char *trans, const int *n, const int *nrhs, const kml_complex_double *dl,
    const kml_complex_double *d, const kml_complex_double *du, const kml_complex_double *du2, const int *ipiv,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void cgtts2_(const int *trans, const int *n, const int *nrhs, const kml_complex_float *dl,
    const kml_complex_float *d, const kml_complex_float *du, const kml_complex_float *du2, const int *ipiv,
    kml_complex_float *b, const int *ldb);

KML_EXPORT void dgtts2_(const int *trans, const int *n, const int *nrhs, const double *dl, const double *d,
    const double *du, const double *du2, const int *ipiv, double *b, const int *ldb);

KML_EXPORT void sgtts2_(const int *trans, const int *n, const int *nrhs, const float *dl, const float *d,
    const float *du, const float *du2, const int *ipiv, float *b, const int *ldb);

KML_EXPORT void zgtts2_(const int *trans, const int *n, const int *nrhs, const kml_complex_double *dl,
    const kml_complex_double *d, const kml_complex_double *du, const kml_complex_double *du2, const int *ipiv,
    kml_complex_double *b, const int *ldb);

KML_EXPORT void cgtsv_(const int *n, const int *nrhs, kml_complex_float *dl, kml_complex_float *d,
    kml_complex_float *du, kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dgtsv_(const int *n, const int *nrhs, double *dl, double *d, double *du, double *b, const int *ldb,
    int *info);

KML_EXPORT void sgtsv_(const int *n, const int *nrhs, float *dl, float *d, float *du, float *b, const int *ldb,
    int *info);

KML_EXPORT void zgtsv_(const int *n, const int *nrhs, kml_complex_double *dl, kml_complex_double *d,
    kml_complex_double *du, kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void clasr_(const char *side, const char *pivot, const char *direct, const int *m, const int *n,
    const float *c, const float *s, kml_complex_float *a, const int *lda);

KML_EXPORT void slasr_(const char *side, const char *pivot, const char *direct, const int *m, const int *n,
    const float *c, const float *s, float *a, const int *lda);

KML_EXPORT void dlasr_(const char *side, const char *pivot, const char *direct, const int *m, const int *n,
    const double *c, const double *s, double *a, const int *lda);

KML_EXPORT void zlasr_(const char *side, const char *pivot, const char *direct, const int *m, const int *n,
    const double *c, const double *s, kml_complex_double *a, const int *lda);
KML_EXPORT void cpotrs_(const char *uplo, const int *n, const int *nrhs, const kml_complex_float *a, const int *lda,
    kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dpotrs_(const char *uplo, const int *n, const int *nrhs, const double *a, const int *lda, double *b,
    const int *ldb, int *info);

KML_EXPORT void spotrs_(const char *uplo, const int *n, const int *nrhs, const float *a, const int *lda, float *b,
    const int *ldb, int *info);

KML_EXPORT void zpotrs_(const char *uplo, const int *n, const int *nrhs, const kml_complex_double *a, const int *lda,
    kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void dsytrd_2stage_(const char *vect, const char *uplo, const int *n, double *a, const int *lda, double *d,
    double *e, double *tau, double *hous2, const int *lhous2, double *work, const int *lwork, int *info);

KML_EXPORT void ssytrd_2stage_(const char *vect, const char *uplo, const int *n, float *a, const int *lda, float *d,
    float *e, float *tau, float *hous2, const int *lhous2, float *work, const int *lwork, int *info);

KML_EXPORT void chetrd_2stage_(const char *vect, const char *uplo, const int *n, kml_complex_float *a, const int *lda,
    float *d, float *e, kml_complex_float *tau, kml_complex_float *hous2, const int *lhous2, kml_complex_float *work,
    const int *lwork, int *info);

KML_EXPORT void zhetrd_2stage_(const char *vect, const char *uplo, const int *n, kml_complex_double *a, const int *lda,
    double *d, double *e, kml_complex_double *tau, kml_complex_double *hous2, const int *lhous2,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void ctrtrs_(const char *uplo, const char *trans, const char *diag, const int *n, const int *nrhs,
    const kml_complex_float *a, const int *lda, kml_complex_float *b, const int *ldb, int *info);

KML_EXPORT void dtrtrs_(const char *uplo, const char *trans, const char *diag, const int *n, const int *nrhs,
    const double *a, const int *lda, double *b, const int *ldb, int *info);

KML_EXPORT void strtrs_(const char *uplo, const char *trans, const char *diag, const int *n, const int *nrhs,
    const float *a, const int *lda, float *b, const int *ldb, int *info);

KML_EXPORT void ztrtrs_(const char *uplo, const char *trans, const char *diag, const int *n, const int *nrhs,
    const kml_complex_double *a, const int *lda, kml_complex_double *b, const int *ldb, int *info);

KML_EXPORT void dsgesv_(const int *n, const int *nrhs, double *a, const int *lda, int *ipiv, const double *b,
    const int *ldb, double *x, const int *ldx, double *work, float *swork, int *iter, int *info);

KML_EXPORT void zcgesv_(const int *n, const int *nrhs, kml_complex_double *a, const int *lda, int *ipiv,
    const kml_complex_double *b, const int *ldb, kml_complex_double *x, const int *ldx, kml_complex_double *work,
    kml_complex_float *swork, double *rwork, int *iter, int *info);

KML_EXPORT void clascl_(const char *type, const int *kl, const int *ku, const float *cfrom, const float *cto,
    const int *m, const int *n, kml_complex_float *a, const int *lda, int *info);

KML_EXPORT void dlascl_(const char *type, const int *kl, const int *ku, const double *cfrom, const double *cto,
    const int *m, const int *n, double *a, const int *lda, int *info);

KML_EXPORT void slascl_(const char *type, const int *kl, const int *ku, const float *cfrom, const float *cto,
    const int *m, const int *n, float *a, const int *lda, int *info);

KML_EXPORT void zlascl_(const char *type, const int *kl, const int *ku, const double *cfrom, const double *cto,
    const int *m, const int *n, kml_complex_double *a, const int *lda, int *info);

KML_EXPORT void dormbr_(const char *vect, const char *side, const char *trans, const int *m, const int *n, const int *k,
    const double *a, const int *lda, const double *tau, double *c, const int *ldc, double *work, const int *lwork,
    int *info);

KML_EXPORT void sormbr_(const char *vect, const char *side, const char *trans, const int *m, const int *n, const int *k,
    const float *a, const int *lda, const float *tau, float *c, const int *ldc, float *work, const int *lwork,
    int *info);

KML_EXPORT void cunmbr_(const char *vect, const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_float *a, const int *lda, const kml_complex_float *tau, kml_complex_float *c, const int *ldc,
    kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void zunmbr_(const char *vect, const char *side, const char *trans, const int *m, const int *n, const int *k,
    const kml_complex_double *a, const int *lda, const kml_complex_double *tau, kml_complex_double *c, const int *ldc,
    kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void ctrtri_(const char *uplo, const char *diag, const int *n, kml_complex_float *a, const int *lda,
    int *info);

KML_EXPORT void dtrtri_(const char *uplo, const char *diag, const int *n, double *a, const int *lda, int *info);

KML_EXPORT void strtri_(const char *uplo, const char *diag, const int *n, float *a, const int *lda, int *info);

KML_EXPORT void ztrtri_(const char *uplo, const char *diag, const int *n, kml_complex_double *a, const int *lda,
    int *info);

KML_EXPORT void cgesvd_(const char *jobu, const char *jobvt, const int *m, const int *n, kml_complex_float *a,
    const int *lda, float *s, kml_complex_float *u, const int *ldu, kml_complex_float *vt, const int *ldvt,
    kml_complex_float *work, const int *lwork, float *rwork, int *info);

KML_EXPORT void dgesvd_(const char *jobu, const char *jobvt, const int *m, const int *n, double *a, const int *lda,
    double *s, double *u, const int *ldu, double *vt, const int *ldvt, double *work, const int *lwork, int *info);

KML_EXPORT void sgesvd_(const char *jobu, const char *jobvt, const int *m, const int *n, float *a, const int *lda,
    float *s, float *u, const int *ldu, float *vt, const int *ldvt, float *work, const int *lwork, int *info);

KML_EXPORT void zgesvd_(const char *jobu, const char *jobvt, const int *m, const int *n, kml_complex_double *a,
    const int *lda, double *s, kml_complex_double *u, const int *ldu, kml_complex_double *vt, const int *ldvt,
    kml_complex_double *work, const int *lwork, double *rwork, int *info);

KML_EXPORT void cgebrd_(const int *m, const int *n, kml_complex_float *a, const int *lda, float *d, float *e,
    kml_complex_float *tauq, kml_complex_float *taup, kml_complex_float *work, const int *lwork, int *info);

KML_EXPORT void dgebrd_(const int *m, const int *n, double *a, const int *lda, double *d, double *e, double *tauq,
    double *taup, double *work, const int *lwork, int *info);

KML_EXPORT void sgebrd_(const int *m, const int *n, float *a, const int *lda, float *d, float *e, float *tauq,
    float *taup, float *work, const int *lwork, int *info);

KML_EXPORT void zgebrd_(const int *m, const int *n, kml_complex_double *a, const int *lda, double *d, double *e,
    kml_complex_double *tauq, kml_complex_double *taup, kml_complex_double *work, const int *lwork, int *info);

KML_EXPORT void cbdsqr_(const char *uplo, const int *n, const int *ncvt, const int *nru, const int *ncc, float *d,
    float *e, kml_complex_float *vt, const int *ldvt, kml_complex_float *u, const int *ldu, kml_complex_float *c,
    const int *ldc, float *rwork, int *info);

KML_EXPORT void dbdsqr_(const char *uplo, const int *n, const int *ncvt, const int *nru, const int *ncc, double *d,
    double *e, double *vt, const int *ldvt, double *u, const int *ldu, double *c, const int *ldc, double *work,
    int *info);

KML_EXPORT void sbdsqr_(const char *uplo, const int *n, const int *ncvt, const int *nru, const int *ncc, float *d,
    float *e, float *vt, const int *ldvt, float *u, const int *ldu, float *c, const int *ldc, float *work, int *info);

KML_EXPORT void zbdsqr_(const char *uplo, const int *n, const int *ncvt, const int *nru, const int *ncc, double *d,
    double *e, kml_complex_double *vt, const int *ldvt, kml_complex_double *u, const int *ldu, kml_complex_double *c,
    const int *ldc, double *rwork, int *info);

KML_EXPORT void dbdsdc_(const char *uplo, const char *compq, const int *n, double *d, double *e, double *u,
    const int *ldu, double *vt, const int *ldvt, double *q, int *iq, double *work, int *iwork, int *info);

KML_EXPORT void sbdsdc_(const char *uplo, const char *compq, const int *n, float *d, float *e, float *u, const int *ldu,
    float *vt, const int *ldvt, float *q, int *iq, float *work, int *iwork, int *info);

typedef struct KMLVersion KLAPACKVersion;
KML_EXPORT int KLAPACKGetVersion(KLAPACKVersion *ver);

#ifdef __cplusplus
}
#endif
// NOLINTEND: standard API

#endif /* KLAPACK_EXTERNAL_H */
