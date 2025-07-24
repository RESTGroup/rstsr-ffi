//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn cblas_dcabs1(z: *const c_void) -> f64 {
    dyload_lib().cblas_dcabs1.unwrap()(z)
}

pub unsafe fn cblas_scabs1(c: *const c_void) -> f32 {
    dyload_lib().cblas_scabs1.unwrap()(c)
}

pub unsafe fn cblas_sdsdot(
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
) -> f32 {
    dyload_lib().cblas_sdsdot.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_dsdot(
    N: blas_int,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
) -> f64 {
    dyload_lib().cblas_dsdot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_sdot(
    N: blas_int,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
) -> f32 {
    dyload_lib().cblas_sdot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_ddot(
    N: blas_int,
    X: *const f64,
    incX: blas_int,
    Y: *const f64,
    incY: blas_int,
) -> f64 {
    dyload_lib().cblas_ddot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_cdotu_sub(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    dotu: *mut c_void,
) {
    dyload_lib().cblas_cdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
}

pub unsafe fn cblas_cdotc_sub(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    dotc: *mut c_void,
) {
    dyload_lib().cblas_cdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
}

pub unsafe fn cblas_zdotu_sub(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    dotu: *mut c_void,
) {
    dyload_lib().cblas_zdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
}

pub unsafe fn cblas_zdotc_sub(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    dotc: *mut c_void,
) {
    dyload_lib().cblas_zdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
}

pub unsafe fn cblas_snrm2(N: blas_int, X: *const f32, incX: blas_int) -> f32 {
    dyload_lib().cblas_snrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_sasum(N: blas_int, X: *const f32, incX: blas_int) -> f32 {
    dyload_lib().cblas_sasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dnrm2(N: blas_int, X: *const f64, incX: blas_int) -> f64 {
    dyload_lib().cblas_dnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dasum(N: blas_int, X: *const f64, incX: blas_int) -> f64 {
    dyload_lib().cblas_dasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_scnrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f32 {
    dyload_lib().cblas_scnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_scasum(N: blas_int, X: *const c_void, incX: blas_int) -> f32 {
    dyload_lib().cblas_scasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dznrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f64 {
    dyload_lib().cblas_dznrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dzasum(N: blas_int, X: *const c_void, incX: blas_int) -> f64 {
    dyload_lib().cblas_dzasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_isamax(N: blas_int, X: *const f32, incX: blas_int) -> usize {
    dyload_lib().cblas_isamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_idamax(N: blas_int, X: *const f64, incX: blas_int) -> usize {
    dyload_lib().cblas_idamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_icamax(N: blas_int, X: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_icamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_izamax(N: blas_int, X: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_izamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_sswap(N: blas_int, X: *mut f32, incX: blas_int, Y: *mut f32, incY: blas_int) {
    dyload_lib().cblas_sswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_scopy(N: blas_int, X: *const f32, incX: blas_int, Y: *mut f32, incY: blas_int) {
    dyload_lib().cblas_scopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_saxpy(
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_saxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_dswap(N: blas_int, X: *mut f64, incX: blas_int, Y: *mut f64, incY: blas_int) {
    dyload_lib().cblas_dswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_dcopy(N: blas_int, X: *const f64, incX: blas_int, Y: *mut f64, incY: blas_int) {
    dyload_lib().cblas_dcopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_daxpy(
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_daxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_cswap(
    N: blas_int,
    X: *mut c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_cswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_ccopy(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_ccopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_caxpy(
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_caxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_zswap(
    N: blas_int,
    X: *mut c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_zcopy(
    N: blas_int,
    X: *const c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zcopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_zaxpy(
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zaxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32) {
    dyload_lib().cblas_srotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_srotm(
    N: blas_int,
    X: *mut f32,
    incX: blas_int,
    Y: *mut f32,
    incY: blas_int,
    P: *const f32,
) {
    dyload_lib().cblas_srotm.unwrap()(N, X, incX, Y, incY, P)
}

pub unsafe fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64) {
    dyload_lib().cblas_drotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_drotm(
    N: blas_int,
    X: *mut f64,
    incX: blas_int,
    Y: *mut f64,
    incY: blas_int,
    P: *const f64,
) {
    dyload_lib().cblas_drotm.unwrap()(N, X, incX, Y, incY, P)
}

pub unsafe fn cblas_sscal(N: blas_int, alpha: f32, X: *mut f32, incX: blas_int) {
    dyload_lib().cblas_sscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_dscal(N: blas_int, alpha: f64, X: *mut f64, incX: blas_int) {
    dyload_lib().cblas_dscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_cscal(N: blas_int, alpha: *const c_void, X: *mut c_void, incX: blas_int) {
    dyload_lib().cblas_cscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_zscal(N: blas_int, alpha: *const c_void, X: *mut c_void, incX: blas_int) {
    dyload_lib().cblas_zscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_csscal(N: blas_int, alpha: f32, X: *mut c_void, incX: blas_int) {
    dyload_lib().cblas_csscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_zdscal(N: blas_int, alpha: f64, X: *mut c_void, incX: blas_int) {
    dyload_lib().cblas_zdscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
    dyload_lib().cblas_srotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
    dyload_lib().cblas_drotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_crotg(a: *mut c_void, b: *mut c_void, c: *mut f32, s: *mut c_void) {
    dyload_lib().cblas_crotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_zrotg(a: *mut c_void, b: *mut c_void, c: *mut f64, s: *mut c_void) {
    dyload_lib().cblas_zrotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_srot(
    N: blas_int,
    X: *mut f32,
    incX: blas_int,
    Y: *mut f32,
    incY: blas_int,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_srot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_drot(
    N: blas_int,
    X: *mut f64,
    incX: blas_int,
    Y: *mut f64,
    incY: blas_int,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_drot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_csrot(
    N: blas_int,
    X: *mut c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_csrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_zdrot(
    N: blas_int,
    X: *mut c_void,
    incX: blas_int,
    Y: *mut c_void,
    incY: blas_int,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_zdrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_sgemv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    X: *const f32,
    incX: blas_int,
    beta: f32,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_sgemv.unwrap()(layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sgbmv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    KL: blas_int,
    KU: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    X: *const f32,
    incX: blas_int,
    beta: f32,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_sgbmv.unwrap()(
        layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_strmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_strmv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_stbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stbmv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f32,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stpmv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_strsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_strsv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_stbsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stbsv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stpsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f32,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stpsv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dgemv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    X: *const f64,
    incX: blas_int,
    beta: f64,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dgemv.unwrap()(layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dgbmv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    KL: blas_int,
    KU: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    X: *const f64,
    incX: blas_int,
    beta: f64,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dgbmv.unwrap()(
        layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_dtrmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtrmv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtbmv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f64,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtpmv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dtrsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtrsv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtbsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtbsv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtpsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f64,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtpsv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_cgemv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_cgemv.unwrap()(layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_cgbmv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    KL: blas_int,
    KU: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_cgbmv.unwrap()(
        layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_ctrmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctrmv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctbmv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctpmv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ctrsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctrsv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctbsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctbsv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctpsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctpsv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_zgemv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zgemv.unwrap()(layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zgbmv(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    KL: blas_int,
    KU: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zgbmv.unwrap()(
        layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_ztrmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztrmv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztbmv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztpmv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ztrsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztrsv.unwrap()(layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztbsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    K: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztbsv.unwrap()(layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztpsv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztpsv.unwrap()(layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ssymv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    X: *const f32,
    incX: blas_int,
    beta: f32,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_ssymv.unwrap()(layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_ssbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    X: *const f32,
    incX: blas_int,
    beta: f32,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_ssbmv.unwrap()(layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sspmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    Ap: *const f32,
    X: *const f32,
    incX: blas_int,
    beta: f32,
    Y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_sspmv.unwrap()(layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sger(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
    A: *mut f32,
    lda: blas_int,
) {
    dyload_lib().cblas_sger.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_ssyr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    A: *mut f32,
    lda: blas_int,
) {
    dyload_lib().cblas_ssyr.unwrap()(layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_sspr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Ap: *mut f32,
) {
    dyload_lib().cblas_sspr.unwrap()(layout, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_ssyr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
    A: *mut f32,
    lda: blas_int,
) {
    dyload_lib().cblas_ssyr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_sspr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
    A: *mut f32,
) {
    dyload_lib().cblas_sspr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_dsymv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    X: *const f64,
    incX: blas_int,
    beta: f64,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dsymv.unwrap()(layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dsbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    X: *const f64,
    incX: blas_int,
    beta: f64,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dsbmv.unwrap()(layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dspmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    Ap: *const f64,
    X: *const f64,
    incX: blas_int,
    beta: f64,
    Y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dspmv.unwrap()(layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dger(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Y: *const f64,
    incY: blas_int,
    A: *mut f64,
    lda: blas_int,
) {
    dyload_lib().cblas_dger.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dsyr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    A: *mut f64,
    lda: blas_int,
) {
    dyload_lib().cblas_dsyr.unwrap()(layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_dspr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Ap: *mut f64,
) {
    dyload_lib().cblas_dspr.unwrap()(layout, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_dsyr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Y: *const f64,
    incY: blas_int,
    A: *mut f64,
    lda: blas_int,
) {
    dyload_lib().cblas_dsyr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dspr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Y: *const f64,
    incY: blas_int,
    A: *mut f64,
) {
    dyload_lib().cblas_dspr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_chemv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_chemv.unwrap()(layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_chbmv.unwrap()(layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    Ap: *const c_void,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_chpmv.unwrap()(layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_cgeru(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_cgeru.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cgerc(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_cgerc.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cher(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_cher.unwrap()(layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_chpr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
) {
    dyload_lib().cblas_chpr.unwrap()(layout, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_cher2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_cher2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_chpr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_chpr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_zhemv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zhemv.unwrap()(layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhbmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zhbmv.unwrap()(layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhpmv(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    Ap: *const c_void,
    X: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    Y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zhpmv.unwrap()(layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zgeru(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_zgeru.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zgerc(
    layout: CBLAS_LAYOUT,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_zgerc.unwrap()(layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zher(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_zher.unwrap()(layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_zhpr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
) {
    dyload_lib().cblas_zhpr.unwrap()(layout, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_zher2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_zher2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zhpr2(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_zhpr2.unwrap()(layout, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_sgemm(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *const f32,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_sgemm.unwrap()(
        layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_sgemmtr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *const f32,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_sgemmtr.unwrap()(
        layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ssymm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *const f32,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_ssymm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_ssyrk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_ssyrk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_ssyr2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *const f32,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_ssyr2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_strmm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_strmm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_strsm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: f32,
    A: *const f32,
    lda: blas_int,
    B: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_strsm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dgemm(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *const f64,
    ldb: blas_int,
    beta: f64,
    C: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dgemm.unwrap()(
        layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dgemmtr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *const f64,
    ldb: blas_int,
    beta: f64,
    C: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dgemmtr.unwrap()(
        layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dsymm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *const f64,
    ldb: blas_int,
    beta: f64,
    C: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dsymm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_dsyrk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    beta: f64,
    C: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dsyrk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_dsyr2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *const f64,
    ldb: blas_int,
    beta: f64,
    C: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dsyr2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dtrmm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dtrmm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dtrsm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: f64,
    A: *const f64,
    lda: blas_int,
    B: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dtrsm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_cgemm(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_cgemm.unwrap()(
        layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemmtr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_cgemmtr.unwrap()(
        layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_csymm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_csymm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_csyrk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_csyrk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_csyr2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_csyr2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ctrmm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *mut c_void,
    ldb: blas_int,
) {
    dyload_lib().cblas_ctrmm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ctrsm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *mut c_void,
    ldb: blas_int,
) {
    dyload_lib().cblas_ctrsm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_zgemm(
    layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zgemm.unwrap()(
        layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemmtr(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zgemmtr.unwrap()(
        layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zsymm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zsymm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zsyrk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zsyrk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zsyr2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zsyr2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ztrmm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *mut c_void,
    ldb: blas_int,
) {
    dyload_lib().cblas_ztrmm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ztrsm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *mut c_void,
    ldb: blas_int,
) {
    dyload_lib().cblas_ztrsm.unwrap()(layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_chemm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_chemm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_cherk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const c_void,
    lda: blas_int,
    beta: f32,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_cherk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_cher2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: f32,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_cher2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zhemm(
    layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: blas_int,
    N: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: *const c_void,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zhemm.unwrap()(layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zherk(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: f64,
    A: *const c_void,
    lda: blas_int,
    beta: f64,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zherk.unwrap()(layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zher2k(
    layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: blas_int,
    K: blas_int,
    alpha: *const c_void,
    A: *const c_void,
    lda: blas_int,
    B: *const c_void,
    ldb: blas_int,
    beta: f64,
    C: *mut c_void,
    ldc: blas_int,
) {
    dyload_lib().cblas_zher2k.unwrap()(
        layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_xerbla(p: blas_int, rout: *const c_char, form: *const c_char) {
    dyload_lib().cblas_xerbla.unwrap()(p, rout, form)
}
