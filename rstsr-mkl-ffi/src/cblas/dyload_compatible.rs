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

pub unsafe fn cblas_sdot(
    N: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
) -> f32 {
    dyload_lib().cblas_sdot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_sdoti(N: MKL_INT, X: *const f32, indx: *const MKL_INT, Y: *const f32) -> f32 {
    dyload_lib().cblas_sdoti.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_ddot(
    N: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    Y: *const f64,
    incY: MKL_INT,
) -> f64 {
    dyload_lib().cblas_ddot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_ddoti(N: MKL_INT, X: *const f64, indx: *const MKL_INT, Y: *const f64) -> f64 {
    dyload_lib().cblas_ddoti.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_dsdot(
    N: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
) -> f64 {
    dyload_lib().cblas_dsdot.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_sdsdot(
    N: MKL_INT,
    sb: f32,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
) -> f32 {
    dyload_lib().cblas_sdsdot.unwrap()(N, sb, X, incX, Y, incY)
}

pub unsafe fn cblas_cdotu_sub(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    dotu: *mut c_void,
) {
    dyload_lib().cblas_cdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
}

pub unsafe fn cblas_cdotui_sub(
    N: MKL_INT,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *const c_void,
    dotui: *mut c_void,
) {
    dyload_lib().cblas_cdotui_sub.unwrap()(N, X, indx, Y, dotui)
}

pub unsafe fn cblas_cdotc_sub(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    dotc: *mut c_void,
) {
    dyload_lib().cblas_cdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
}

pub unsafe fn cblas_cdotci_sub(
    N: MKL_INT,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *const c_void,
    dotui: *mut c_void,
) {
    dyload_lib().cblas_cdotci_sub.unwrap()(N, X, indx, Y, dotui)
}

pub unsafe fn cblas_zdotu_sub(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    dotu: *mut c_void,
) {
    dyload_lib().cblas_zdotu_sub.unwrap()(N, X, incX, Y, incY, dotu)
}

pub unsafe fn cblas_zdotui_sub(
    N: MKL_INT,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *const c_void,
    dotui: *mut c_void,
) {
    dyload_lib().cblas_zdotui_sub.unwrap()(N, X, indx, Y, dotui)
}

pub unsafe fn cblas_zdotc_sub(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    dotc: *mut c_void,
) {
    dyload_lib().cblas_zdotc_sub.unwrap()(N, X, incX, Y, incY, dotc)
}

pub unsafe fn cblas_zdotci_sub(
    N: MKL_INT,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *const c_void,
    dotui: *mut c_void,
) {
    dyload_lib().cblas_zdotci_sub.unwrap()(N, X, indx, Y, dotui)
}

pub unsafe fn cblas_snrm2(N: MKL_INT, X: *const f32, incX: MKL_INT) -> f32 {
    dyload_lib().cblas_snrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_sasum(N: MKL_INT, X: *const f32, incX: MKL_INT) -> f32 {
    dyload_lib().cblas_sasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dnrm2(N: MKL_INT, X: *const f64, incX: MKL_INT) -> f64 {
    dyload_lib().cblas_dnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dasum(N: MKL_INT, X: *const f64, incX: MKL_INT) -> f64 {
    dyload_lib().cblas_dasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_scnrm2(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> f32 {
    dyload_lib().cblas_scnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_scasum(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> f32 {
    dyload_lib().cblas_scasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dznrm2(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> f64 {
    dyload_lib().cblas_dznrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dzasum(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> f64 {
    dyload_lib().cblas_dzasum.unwrap()(N, X, incX)
}

pub unsafe fn cblas_isamax(N: MKL_INT, X: *const f32, incX: MKL_INT) -> usize {
    dyload_lib().cblas_isamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_idamax(N: MKL_INT, X: *const f64, incX: MKL_INT) -> usize {
    dyload_lib().cblas_idamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_icamax(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> usize {
    dyload_lib().cblas_icamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_izamax(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> usize {
    dyload_lib().cblas_izamax.unwrap()(N, X, incX)
}

pub unsafe fn cblas_isamin(N: MKL_INT, X: *const f32, incX: MKL_INT) -> usize {
    dyload_lib().cblas_isamin.unwrap()(N, X, incX)
}

pub unsafe fn cblas_idamin(N: MKL_INT, X: *const f64, incX: MKL_INT) -> usize {
    dyload_lib().cblas_idamin.unwrap()(N, X, incX)
}

pub unsafe fn cblas_icamin(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> usize {
    dyload_lib().cblas_icamin.unwrap()(N, X, incX)
}

pub unsafe fn cblas_izamin(N: MKL_INT, X: *const c_void, incX: MKL_INT) -> usize {
    dyload_lib().cblas_izamin.unwrap()(N, X, incX)
}

pub unsafe fn cblas_sswap(N: MKL_INT, X: *mut f32, incX: MKL_INT, Y: *mut f32, incY: MKL_INT) {
    dyload_lib().cblas_sswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_scopy(N: MKL_INT, X: *const f32, incX: MKL_INT, Y: *mut f32, incY: MKL_INT) {
    dyload_lib().cblas_scopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_saxpy(
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_saxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_saxpby(
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_saxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_saxpyi(
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    indx: *const MKL_INT,
    Y: *mut f32,
) {
    dyload_lib().cblas_saxpyi.unwrap()(N, alpha, X, indx, Y)
}

pub unsafe fn cblas_sgthr(N: MKL_INT, Y: *const f32, X: *mut f32, indx: *const MKL_INT) {
    dyload_lib().cblas_sgthr.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_sgthrz(N: MKL_INT, Y: *mut f32, X: *mut f32, indx: *const MKL_INT) {
    dyload_lib().cblas_sgthrz.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_ssctr(N: MKL_INT, X: *const f32, indx: *const MKL_INT, Y: *mut f32) {
    dyload_lib().cblas_ssctr.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
    dyload_lib().cblas_srotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_dswap(N: MKL_INT, X: *mut f64, incX: MKL_INT, Y: *mut f64, incY: MKL_INT) {
    dyload_lib().cblas_dswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_dcopy(N: MKL_INT, X: *const f64, incX: MKL_INT, Y: *mut f64, incY: MKL_INT) {
    dyload_lib().cblas_dcopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_daxpy(
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_daxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_daxpby(
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_daxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_daxpyi(
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    indx: *const MKL_INT,
    Y: *mut f64,
) {
    dyload_lib().cblas_daxpyi.unwrap()(N, alpha, X, indx, Y)
}

pub unsafe fn cblas_dgthr(N: MKL_INT, Y: *const f64, X: *mut f64, indx: *const MKL_INT) {
    dyload_lib().cblas_dgthr.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_dgthrz(N: MKL_INT, Y: *mut f64, X: *mut f64, indx: *const MKL_INT) {
    dyload_lib().cblas_dgthrz.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_dsctr(N: MKL_INT, X: *const f64, indx: *const MKL_INT, Y: *mut f64) {
    dyload_lib().cblas_dsctr.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
    dyload_lib().cblas_drotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_cswap(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_cswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_ccopy(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_ccopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_caxpy(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_caxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_caxpby(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_caxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_caxpyi(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *mut c_void,
) {
    dyload_lib().cblas_caxpyi.unwrap()(N, alpha, X, indx, Y)
}

pub unsafe fn cblas_cgthr(N: MKL_INT, Y: *const c_void, X: *mut c_void, indx: *const MKL_INT) {
    dyload_lib().cblas_cgthr.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_cgthrz(N: MKL_INT, Y: *mut c_void, X: *mut c_void, indx: *const MKL_INT) {
    dyload_lib().cblas_cgthrz.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_csctr(N: MKL_INT, X: *const c_void, indx: *const MKL_INT, Y: *mut c_void) {
    dyload_lib().cblas_csctr.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_crotg(a: *mut c_void, b: *const c_void, c: *mut f32, s: *mut c_void) {
    dyload_lib().cblas_crotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_zswap(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zswap.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_zcopy(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zcopy.unwrap()(N, X, incX, Y, incY)
}

pub unsafe fn cblas_zaxpy(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zaxpy.unwrap()(N, alpha, X, incX, Y, incY)
}

pub unsafe fn cblas_zaxpby(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zaxpby.unwrap()(N, alpha, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zaxpyi(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    indx: *const MKL_INT,
    Y: *mut c_void,
) {
    dyload_lib().cblas_zaxpyi.unwrap()(N, alpha, X, indx, Y)
}

pub unsafe fn cblas_zgthr(N: MKL_INT, Y: *const c_void, X: *mut c_void, indx: *const MKL_INT) {
    dyload_lib().cblas_zgthr.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_zgthrz(N: MKL_INT, Y: *mut c_void, X: *mut c_void, indx: *const MKL_INT) {
    dyload_lib().cblas_zgthrz.unwrap()(N, Y, X, indx)
}

pub unsafe fn cblas_zsctr(N: MKL_INT, X: *const c_void, indx: *const MKL_INT, Y: *mut c_void) {
    dyload_lib().cblas_zsctr.unwrap()(N, X, indx, Y)
}

pub unsafe fn cblas_zrotg(a: *mut c_void, b: *const c_void, c: *mut f64, s: *mut c_void) {
    dyload_lib().cblas_zrotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32) {
    dyload_lib().cblas_srotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_sroti(
    N: MKL_INT,
    X: *mut f32,
    indx: *const MKL_INT,
    Y: *mut f32,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_sroti.unwrap()(N, X, indx, Y, c, s)
}

pub unsafe fn cblas_srotm(
    N: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
    Y: *mut f32,
    incY: MKL_INT,
    P: *const f32,
) {
    dyload_lib().cblas_srotm.unwrap()(N, X, incX, Y, incY, P)
}

pub unsafe fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64) {
    dyload_lib().cblas_drotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_drotm(
    N: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
    Y: *mut f64,
    incY: MKL_INT,
    P: *const f64,
) {
    dyload_lib().cblas_drotm.unwrap()(N, X, incX, Y, incY, P)
}

pub unsafe fn cblas_droti(
    N: MKL_INT,
    X: *mut f64,
    indx: *const MKL_INT,
    Y: *mut f64,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_droti.unwrap()(N, X, indx, Y, c, s)
}

pub unsafe fn cblas_sscal(N: MKL_INT, alpha: f32, X: *mut f32, incX: MKL_INT) {
    dyload_lib().cblas_sscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_dscal(N: MKL_INT, alpha: f64, X: *mut f64, incX: MKL_INT) {
    dyload_lib().cblas_dscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_cscal(N: MKL_INT, alpha: *const c_void, X: *mut c_void, incX: MKL_INT) {
    dyload_lib().cblas_cscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_zscal(N: MKL_INT, alpha: *const c_void, X: *mut c_void, incX: MKL_INT) {
    dyload_lib().cblas_zscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_csscal(N: MKL_INT, alpha: f32, X: *mut c_void, incX: MKL_INT) {
    dyload_lib().cblas_csscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_zdscal(N: MKL_INT, alpha: f64, X: *mut c_void, incX: MKL_INT) {
    dyload_lib().cblas_zdscal.unwrap()(N, alpha, X, incX)
}

pub unsafe fn cblas_srot(
    N: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
    Y: *mut f32,
    incY: MKL_INT,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_srot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_drot(
    N: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
    Y: *mut f64,
    incY: MKL_INT,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_drot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_crot(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    c: f32,
    s: *const c_void,
) {
    dyload_lib().cblas_crot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_zrot(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    c: f64,
    s: *const c_void,
) {
    dyload_lib().cblas_zrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_csrot(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_csrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_zdrot(
    N: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_zdrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_sgemv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_sgemv.unwrap()(Layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sgbmv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    KL: MKL_INT,
    KU: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_sgbmv.unwrap()(
        Layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_strmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const f32,
    lda: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_strmv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_stbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f32,
    lda: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_stbmv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const f32,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_stpmv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_strsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const f32,
    lda: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_strsv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_stbsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f32,
    lda: MKL_INT,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_stbsv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stpsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const f32,
    X: *mut f32,
    incX: MKL_INT,
) {
    dyload_lib().cblas_stpsv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dgemv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_dgemv.unwrap()(Layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dgbmv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    KL: MKL_INT,
    KU: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_dgbmv.unwrap()(
        Layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_dtrmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const f64,
    lda: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtrmv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f64,
    lda: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtbmv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const f64,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtpmv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dtrsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const f64,
    lda: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtrsv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtbsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f64,
    lda: MKL_INT,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtbsv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtpsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const f64,
    X: *mut f64,
    incX: MKL_INT,
) {
    dyload_lib().cblas_dtpsv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_cgemv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_cgemv.unwrap()(Layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_cgbmv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    KL: MKL_INT,
    KU: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_cgbmv.unwrap()(
        Layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_ctrmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctrmv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctbmv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const c_void,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctpmv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ctrsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctrsv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctbsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctbsv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctpsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const c_void,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ctpsv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_zgemv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zgemv.unwrap()(Layout, TransA, M, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zgbmv(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    KL: MKL_INT,
    KU: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zgbmv.unwrap()(
        Layout, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_ztrmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztrmv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztbmv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const c_void,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztpmv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ztrsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztrsv.unwrap()(Layout, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztbsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    K: MKL_INT,
    A: *const c_void,
    lda: MKL_INT,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztbsv.unwrap()(Layout, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztpsv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: MKL_INT,
    Ap: *const c_void,
    X: *mut c_void,
    incX: MKL_INT,
) {
    dyload_lib().cblas_ztpsv.unwrap()(Layout, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ssymv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_ssymv.unwrap()(Layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_ssbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_ssbmv.unwrap()(Layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sspmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    Ap: *const f32,
    X: *const f32,
    incX: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
) {
    dyload_lib().cblas_sspmv.unwrap()(Layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sger(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
    A: *mut f32,
    lda: MKL_INT,
) {
    dyload_lib().cblas_sger.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_ssyr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    A: *mut f32,
    lda: MKL_INT,
) {
    dyload_lib().cblas_ssyr.unwrap()(Layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_sspr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    Ap: *mut f32,
) {
    dyload_lib().cblas_sspr.unwrap()(Layout, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_ssyr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
    A: *mut f32,
    lda: MKL_INT,
) {
    dyload_lib().cblas_ssyr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_sspr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    Y: *const f32,
    incY: MKL_INT,
    A: *mut f32,
) {
    dyload_lib().cblas_sspr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_dsymv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_dsymv.unwrap()(Layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dsbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_dsbmv.unwrap()(Layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dspmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    Ap: *const f64,
    X: *const f64,
    incX: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
) {
    dyload_lib().cblas_dspmv.unwrap()(Layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dger(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    Y: *const f64,
    incY: MKL_INT,
    A: *mut f64,
    lda: MKL_INT,
) {
    dyload_lib().cblas_dger.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dsyr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    A: *mut f64,
    lda: MKL_INT,
) {
    dyload_lib().cblas_dsyr.unwrap()(Layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_dspr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    Ap: *mut f64,
) {
    dyload_lib().cblas_dspr.unwrap()(Layout, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_dsyr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    Y: *const f64,
    incY: MKL_INT,
    A: *mut f64,
    lda: MKL_INT,
) {
    dyload_lib().cblas_dsyr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dspr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    Y: *const f64,
    incY: MKL_INT,
    A: *mut f64,
) {
    dyload_lib().cblas_dspr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_chemv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_chemv.unwrap()(Layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_chbmv.unwrap()(Layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    Ap: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_chpmv.unwrap()(Layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_cgeru(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_cgeru.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cgerc(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_cgerc.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cher(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const c_void,
    incX: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_cher.unwrap()(Layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_chpr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f32,
    X: *const c_void,
    incX: MKL_INT,
    A: *mut c_void,
) {
    dyload_lib().cblas_chpr.unwrap()(Layout, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_cher2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_cher2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_chpr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_chpr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_zhemv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zhemv.unwrap()(Layout, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhbmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zhbmv.unwrap()(Layout, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhpmv(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    Ap: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
) {
    dyload_lib().cblas_zhpmv.unwrap()(Layout, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zgeru(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_zgeru.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zgerc(
    Layout: CBLAS_LAYOUT,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_zgerc.unwrap()(Layout, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zher(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const c_void,
    incX: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_zher.unwrap()(Layout, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_zhpr(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: f64,
    X: *const c_void,
    incX: MKL_INT,
    A: *mut c_void,
) {
    dyload_lib().cblas_zhpr.unwrap()(Layout, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_zher2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    A: *mut c_void,
    lda: MKL_INT,
) {
    dyload_lib().cblas_zher2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zhpr2(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    Y: *const c_void,
    incY: MKL_INT,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_zhpr2.unwrap()(Layout, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_sgemm(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_sgemm.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_sgemm_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const f32,
    A_Array: *mut *const f32,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const f32,
    ldb_Array: *const MKL_INT,
    beta_Array: *const f32,
    C_Array: *mut *mut f32,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_sgemm_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_sgemm_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_sgemm_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_sgemmt(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_sgemmt.unwrap()(
        Layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ssymm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ssymm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_ssyrk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ssyrk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_ssyrk_batch_strided(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    stridea: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_ssyrk_batch_strided.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, stridea, beta, C, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_ssyrk_batch(
    Layout: CBLAS_LAYOUT,
    Uplo_array: *const CBLAS_UPLO,
    Trans_array: *const CBLAS_TRANSPOSE,
    N_array: *const MKL_INT,
    K_array: *const MKL_INT,
    alpha_array: *const f32,
    A_array: *mut *const f32,
    lda_array: *const MKL_INT,
    beta_array: *const f32,
    C_array: *mut *mut f32,
    ldc_array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_ssyrk_batch.unwrap()(
        Layout,
        Uplo_array,
        Trans_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_ssyr2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ssyr2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_strmm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *mut f32,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_strmm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_strmm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_strmm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_strsm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *mut f32,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_strsm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_strsm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_strsm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_strsm_batch(
    Layout: CBLAS_LAYOUT,
    Side_Array: *const CBLAS_SIDE,
    Uplo_Array: *const CBLAS_UPLO,
    TransA_Array: *const CBLAS_TRANSPOSE,
    Diag_Array: *const CBLAS_DIAG,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    alpha_Array: *const f32,
    A_Array: *mut *const f32,
    lda_Array: *const MKL_INT,
    B_Array: *mut *mut f32,
    ldb_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_strsm_batch.unwrap()(
        Layout,
        Side_Array,
        Uplo_Array,
        TransA_Array,
        Diag_Array,
        M_Array,
        N_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_strsm_batch_strided(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *mut f32,
    ldb: MKL_INT,
    strideb: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_strsm_batch_strided.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, stridea, B, ldb, strideb, batch_size,
    )
}

pub unsafe fn cblas_dgemm(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dgemm.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dgemm_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const f64,
    A_Array: *mut *const f64,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const f64,
    ldb_Array: *const MKL_INT,
    beta_Array: *const f64,
    C_Array: *mut *mut f64,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_dgemm_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_dgemm_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_dgemm_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_dgemmt(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dgemmt.unwrap()(
        Layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dsymm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dsymm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_dsyrk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dsyrk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_dsyrk_batch(
    Layout: CBLAS_LAYOUT,
    Uplo_array: *const CBLAS_UPLO,
    Trans_array: *const CBLAS_TRANSPOSE,
    N_array: *const MKL_INT,
    K_array: *const MKL_INT,
    alpha_array: *const f64,
    A_array: *mut *const f64,
    lda_array: *const MKL_INT,
    beta_array: *const f64,
    C_array: *mut *mut f64,
    ldc_array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_dsyrk_batch.unwrap()(
        Layout,
        Uplo_array,
        Trans_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_dsyrk_batch_strided(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    stridea: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_dsyrk_batch_strided.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, stridea, beta, C, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_dsyr2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dsyr2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dtrmm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *mut f64,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_dtrmm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dtrmm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dtrmm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dtrsm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *mut f64,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_dtrsm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dtrsm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dtrsm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dtrsm_batch(
    Layout: CBLAS_LAYOUT,
    Side_Array: *const CBLAS_SIDE,
    Uplo_Array: *const CBLAS_UPLO,
    Transa_Array: *const CBLAS_TRANSPOSE,
    Diag_Array: *const CBLAS_DIAG,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    alpha_Array: *const f64,
    A_Array: *mut *const f64,
    lda_Array: *const MKL_INT,
    B_Array: *mut *mut f64,
    ldb_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_dtrsm_batch.unwrap()(
        Layout,
        Side_Array,
        Uplo_Array,
        Transa_Array,
        Diag_Array,
        M_Array,
        N_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_dtrsm_batch_strided(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *mut f64,
    ldb: MKL_INT,
    strideb: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_dtrsm_batch_strided.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, stridea, B, ldb, strideb, batch_size,
    )
}

pub unsafe fn cblas_cgemm(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_cgemm.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemm3m(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_cgemm3m.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemm_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const c_void,
    ldb_Array: *const MKL_INT,
    beta_Array: *const c_void,
    C_Array: *mut *mut c_void,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_cgemm_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_cgemm_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_cgemm_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_cgemm3m_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const c_void,
    ldb_Array: *const MKL_INT,
    beta_Array: *const c_void,
    C_Array: *mut *mut c_void,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_cgemm3m_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_cgemm3m_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_cgemm3m_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_cgemmt(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_cgemmt.unwrap()(
        Layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_csymm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_csymm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_csyrk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_csyrk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_csyrk_batch(
    Layout: CBLAS_LAYOUT,
    Uplo_array: *const CBLAS_UPLO,
    Trans_array: *const CBLAS_TRANSPOSE,
    N_array: *const MKL_INT,
    K_array: *const MKL_INT,
    alpha_array: *const c_void,
    A_array: *mut *const c_void,
    lda_array: *const MKL_INT,
    beta_array: *const c_void,
    C_array: *mut *mut c_void,
    ldc_array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_csyrk_batch.unwrap()(
        Layout,
        Uplo_array,
        Trans_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_csyrk_batch_strided(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_csyrk_batch_strided.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, stridea, beta, C, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_csyr2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_csyr2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ctrmm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_ctrmm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ctrmm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ctrmm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ctrsm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_ctrsm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ctrsm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ctrsm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ctrsm_batch(
    Layout: CBLAS_LAYOUT,
    Side_Array: *const CBLAS_SIDE,
    Uplo_Array: *const CBLAS_UPLO,
    Transa_Array: *const CBLAS_TRANSPOSE,
    Diag_Array: *const CBLAS_DIAG,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *mut c_void,
    ldb_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_ctrsm_batch.unwrap()(
        Layout,
        Side_Array,
        Uplo_Array,
        Transa_Array,
        Diag_Array,
        M_Array,
        N_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_ctrsm_batch_strided(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_ctrsm_batch_strided.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, stridea, B, ldb, strideb, batch_size,
    )
}

pub unsafe fn cblas_zgemm(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zgemm.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemm3m(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zgemm3m.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemm_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const c_void,
    ldb_Array: *const MKL_INT,
    beta_Array: *const c_void,
    C_Array: *mut *mut c_void,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zgemm_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zgemm_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zgemm_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_zgemm3m_batch(
    Layout: CBLAS_LAYOUT,
    TransA_Array: *const CBLAS_TRANSPOSE,
    TransB_Array: *const CBLAS_TRANSPOSE,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    K_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *const c_void,
    ldb_Array: *const MKL_INT,
    beta_Array: *const c_void,
    C_Array: *mut *mut c_void,
    ldc_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zgemm3m_batch.unwrap()(
        Layout,
        TransA_Array,
        TransB_Array,
        M_Array,
        N_Array,
        K_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        beta_Array,
        C_Array,
        ldc_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zgemm3m_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zgemm3m_batch_strided.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, stridea, B, ldb, strideb, beta, C, ldc,
        stridec, batch_size,
    )
}

pub unsafe fn cblas_zgemmt(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zgemmt.unwrap()(
        Layout, Uplo, TransA, TransB, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zsymm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zsymm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zsyrk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zsyrk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zsyrk_batch(
    Layout: CBLAS_LAYOUT,
    Uplo_array: *const CBLAS_UPLO,
    Trans_array: *const CBLAS_TRANSPOSE,
    N_array: *const MKL_INT,
    K_array: *const MKL_INT,
    alpha_array: *const c_void,
    A_array: *mut *const c_void,
    lda_array: *const MKL_INT,
    beta_array: *const c_void,
    C_array: *mut *mut c_void,
    ldc_array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zsyrk_batch.unwrap()(
        Layout,
        Uplo_array,
        Trans_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zsyrk_batch_strided(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zsyrk_batch_strided.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, stridea, beta, C, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_zsyr2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zsyr2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ztrmm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_ztrmm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ztrmm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ztrmm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ztrsm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
) {
    dyload_lib().cblas_ztrsm.unwrap()(Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ztrsm_oop(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_ztrsm_oop.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ztrsm_batch(
    Layout: CBLAS_LAYOUT,
    Side_Array: *const CBLAS_SIDE,
    Uplo_Array: *const CBLAS_UPLO,
    Transa_Array: *const CBLAS_TRANSPOSE,
    Diag_Array: *const CBLAS_DIAG,
    M_Array: *const MKL_INT,
    N_Array: *const MKL_INT,
    alpha_Array: *const c_void,
    A_Array: *mut *const c_void,
    lda_Array: *const MKL_INT,
    B_Array: *mut *mut c_void,
    ldb_Array: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_ztrsm_batch.unwrap()(
        Layout,
        Side_Array,
        Uplo_Array,
        Transa_Array,
        Diag_Array,
        M_Array,
        N_Array,
        alpha_Array,
        A_Array,
        lda_Array,
        B_Array,
        ldb_Array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_ztrsm_batch_strided(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    B: *mut c_void,
    ldb: MKL_INT,
    strideb: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_ztrsm_batch_strided.unwrap()(
        Layout, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, stridea, B, ldb, strideb, batch_size,
    )
}

pub unsafe fn cblas_chemm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_chemm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_cherk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const c_void,
    lda: MKL_INT,
    beta: f32,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_cherk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_cher2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: f32,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_cher2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zhemm(
    Layout: CBLAS_LAYOUT,
    Side: CBLAS_SIDE,
    Uplo: CBLAS_UPLO,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: *const c_void,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zhemm.unwrap()(Layout, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zherk(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    A: *const c_void,
    lda: MKL_INT,
    beta: f64,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zherk.unwrap()(Layout, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zher2k(
    Layout: CBLAS_LAYOUT,
    Uplo: CBLAS_UPLO,
    Trans: CBLAS_TRANSPOSE,
    N: MKL_INT,
    K: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    B: *const c_void,
    ldb: MKL_INT,
    beta: f64,
    C: *mut c_void,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_zher2k.unwrap()(
        Layout, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_sgemm_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_sgemm_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_sgemm_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    src: *const f32,
    ld: MKL_INT,
    dest: *mut f32,
) {
    dyload_lib().cblas_sgemm_pack.unwrap()(Layout, identifier, Trans, M, N, K, alpha, src, ld, dest)
}

pub unsafe fn cblas_sgemm_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f32,
    lda: MKL_INT,
    B: *const f32,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_sgemm_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dgemm_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_dgemm_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_dgemm_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f64,
    src: *const f64,
    ld: MKL_INT,
    dest: *mut f64,
) {
    dyload_lib().cblas_dgemm_pack.unwrap()(Layout, identifier, Trans, M, N, K, alpha, src, ld, dest)
}

pub unsafe fn cblas_dgemm_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    A: *const f64,
    lda: MKL_INT,
    B: *const f64,
    ldb: MKL_INT,
    beta: f64,
    C: *mut f64,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_dgemm_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_hgemm(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: MKL_F16,
    A: *const MKL_F16,
    lda: MKL_INT,
    B: *const MKL_F16,
    ldb: MKL_INT,
    beta: MKL_F16,
    C: *mut MKL_F16,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_hgemm.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_hgemm_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_hgemm_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_hgemm_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: MKL_F16,
    src: *const MKL_F16,
    ld: MKL_INT,
    dest: *mut MKL_F16,
) {
    dyload_lib().cblas_hgemm_pack.unwrap()(Layout, identifier, Trans, M, N, K, alpha, src, ld, dest)
}

pub unsafe fn cblas_hgemm_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    A: *const MKL_F16,
    lda: MKL_INT,
    B: *const MKL_F16,
    ldb: MKL_INT,
    beta: MKL_F16,
    C: *mut MKL_F16,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_hgemm_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_s16s16s32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    OffsetC: CBLAS_OFFSET,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_INT16,
    lda: MKL_INT,
    ao: MKL_INT16,
    B: *const MKL_INT16,
    ldb: MKL_INT,
    bo: MKL_INT16,
    beta: f32,
    C: *mut MKL_INT32,
    ldc: MKL_INT,
    cb: *const MKL_INT32,
) {
    dyload_lib().cblas_gemm_s16s16s32.unwrap()(
        Layout, TransA, TransB, OffsetC, M, N, K, alpha, A, lda, ao, B, ldb, bo, beta, C, ldc, cb,
    )
}

pub unsafe fn cblas_gemm_s8u8s32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    OffsetC: CBLAS_OFFSET,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const c_void,
    lda: MKL_INT,
    ao: MKL_INT8,
    B: *const c_void,
    ldb: MKL_INT,
    bo: MKL_INT8,
    beta: f32,
    C: *mut MKL_INT32,
    ldc: MKL_INT,
    cb: *const MKL_INT32,
) {
    dyload_lib().cblas_gemm_s8u8s32.unwrap()(
        Layout, TransA, TransB, OffsetC, M, N, K, alpha, A, lda, ao, B, ldb, bo, beta, C, ldc, cb,
    )
}

pub unsafe fn cblas_gemm_bf16bf16f32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_BF16,
    lda: MKL_INT,
    B: *const MKL_BF16,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_bf16bf16f32.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_f16f16f32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_F16,
    lda: MKL_INT,
    B: *const MKL_F16,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_f16f16f32.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_e5m2e5m2f32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_E5M2,
    lda: MKL_INT,
    B: *const MKL_E5M2,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_e5m2e5m2f32.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_e4m3e4m3f32(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_E4M3,
    lda: MKL_INT,
    B: *const MKL_E4M3,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_e4m3e4m3f32.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_s8u8s32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_s8u8s32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_s16s16s32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_s16s16s32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_bf16bf16f32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_bf16bf16f32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_f16f16f32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_f16f16f32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_e5m2e5m2f32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_e5m2e5m2f32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_e4m3e4m3f32_pack_get_size(
    identifier: CBLAS_IDENTIFIER,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
) -> usize {
    dyload_lib().cblas_gemm_e4m3e4m3f32_pack_get_size.unwrap()(identifier, M, N, K)
}

pub unsafe fn cblas_gemm_s8u8s32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const c_void,
    ld: MKL_INT,
    dest: *mut c_void,
) {
    dyload_lib().cblas_gemm_s8u8s32_pack.unwrap()(Layout, identifier, Trans, M, N, K, src, ld, dest)
}

pub unsafe fn cblas_gemm_s16s16s32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const MKL_INT16,
    ld: MKL_INT,
    dest: *mut MKL_INT16,
) {
    dyload_lib().cblas_gemm_s16s16s32_pack.unwrap()(
        Layout, identifier, Trans, M, N, K, src, ld, dest,
    )
}

pub unsafe fn cblas_gemm_bf16bf16f32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const MKL_BF16,
    ld: MKL_INT,
    dest: *mut MKL_BF16,
) {
    dyload_lib().cblas_gemm_bf16bf16f32_pack.unwrap()(
        Layout, identifier, Trans, M, N, K, src, ld, dest,
    )
}

pub unsafe fn cblas_gemm_f16f16f32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const MKL_F16,
    ld: MKL_INT,
    dest: *mut MKL_F16,
) {
    dyload_lib().cblas_gemm_f16f16f32_pack.unwrap()(
        Layout, identifier, Trans, M, N, K, src, ld, dest,
    )
}

pub unsafe fn cblas_gemm_e5m2e5m2f32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const MKL_E5M2,
    ld: MKL_INT,
    dest: *mut MKL_E5M2,
) {
    dyload_lib().cblas_gemm_e5m2e5m2f32_pack.unwrap()(
        Layout, identifier, Trans, M, N, K, src, ld, dest,
    )
}

pub unsafe fn cblas_gemm_e4m3e4m3f32_pack(
    Layout: CBLAS_LAYOUT,
    identifier: CBLAS_IDENTIFIER,
    Trans: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    src: *const MKL_E4M3,
    ld: MKL_INT,
    dest: *mut MKL_E4M3,
) {
    dyload_lib().cblas_gemm_e4m3e4m3f32_pack.unwrap()(
        Layout, identifier, Trans, M, N, K, src, ld, dest,
    )
}

pub unsafe fn cblas_gemm_s8u8s32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    offsetc: CBLAS_OFFSET,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const c_void,
    lda: MKL_INT,
    ao: MKL_INT8,
    B: *const c_void,
    ldb: MKL_INT,
    bo: MKL_INT8,
    beta: f32,
    C: *mut MKL_INT32,
    ldc: MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().cblas_gemm_s8u8s32_compute.unwrap()(
        Layout, TransA, TransB, offsetc, M, N, K, alpha, A, lda, ao, B, ldb, bo, beta, C, ldc, co,
    )
}

pub unsafe fn cblas_gemm_s16s16s32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    offsetc: CBLAS_OFFSET,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_INT16,
    lda: MKL_INT,
    ao: MKL_INT16,
    B: *const MKL_INT16,
    ldb: MKL_INT,
    bo: MKL_INT16,
    beta: f32,
    C: *mut MKL_INT32,
    ldc: MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().cblas_gemm_s16s16s32_compute.unwrap()(
        Layout, TransA, TransB, offsetc, M, N, K, alpha, A, lda, ao, B, ldb, bo, beta, C, ldc, co,
    )
}

pub unsafe fn cblas_gemm_bf16bf16f32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_BF16,
    lda: MKL_INT,
    B: *const MKL_BF16,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_bf16bf16f32_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_f16f16f32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_F16,
    lda: MKL_INT,
    B: *const MKL_F16,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_f16f16f32_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_e5m2e5m2f32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_E5M2,
    lda: MKL_INT,
    B: *const MKL_E5M2,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_e5m2e5m2f32_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_gemm_e4m3e4m3f32_compute(
    Layout: CBLAS_LAYOUT,
    TransA: MKL_INT,
    TransB: MKL_INT,
    M: MKL_INT,
    N: MKL_INT,
    K: MKL_INT,
    alpha: f32,
    A: *const MKL_E4M3,
    lda: MKL_INT,
    B: *const MKL_E4M3,
    ldb: MKL_INT,
    beta: f32,
    C: *mut f32,
    ldc: MKL_INT,
) {
    dyload_lib().cblas_gemm_e4m3e4m3f32_compute.unwrap()(
        Layout, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_dgemm(
    jitter: *mut *mut c_void,
    layout: MKL_LAYOUT,
    transa: MKL_TRANSPOSE,
    transb: MKL_TRANSPOSE,
    m: MKL_INT,
    n: MKL_INT,
    k: MKL_INT,
    alpha: f64,
    lda: MKL_INT,
    ldb: MKL_INT,
    beta: f64,
    ldc: MKL_INT,
) -> mkl_jit_status_t {
    dyload_lib().mkl_cblas_jit_create_dgemm.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_sgemm(
    jitter: *mut *mut c_void,
    layout: MKL_LAYOUT,
    transa: MKL_TRANSPOSE,
    transb: MKL_TRANSPOSE,
    m: MKL_INT,
    n: MKL_INT,
    k: MKL_INT,
    alpha: f32,
    lda: MKL_INT,
    ldb: MKL_INT,
    beta: f32,
    ldc: MKL_INT,
) -> mkl_jit_status_t {
    dyload_lib().mkl_cblas_jit_create_sgemm.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_cgemm(
    jitter: *mut *mut c_void,
    layout: MKL_LAYOUT,
    transa: MKL_TRANSPOSE,
    transb: MKL_TRANSPOSE,
    m: MKL_INT,
    n: MKL_INT,
    k: MKL_INT,
    alpha: *const c_void,
    lda: MKL_INT,
    ldb: MKL_INT,
    beta: *const c_void,
    ldc: MKL_INT,
) -> mkl_jit_status_t {
    dyload_lib().mkl_cblas_jit_create_cgemm.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_zgemm(
    jitter: *mut *mut c_void,
    layout: MKL_LAYOUT,
    transa: MKL_TRANSPOSE,
    transb: MKL_TRANSPOSE,
    m: MKL_INT,
    n: MKL_INT,
    k: MKL_INT,
    alpha: *const c_void,
    lda: MKL_INT,
    ldb: MKL_INT,
    beta: *const c_void,
    ldc: MKL_INT,
) -> mkl_jit_status_t {
    dyload_lib().mkl_cblas_jit_create_zgemm.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_jit_get_dgemm_ptr(jitter: *const c_void) -> dgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_dgemm_ptr.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_sgemm_ptr(jitter: *const c_void) -> sgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_sgemm_ptr.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_cgemm_ptr(jitter: *const c_void) -> cgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_cgemm_ptr.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_zgemm_ptr(jitter: *const c_void) -> zgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_zgemm_ptr.unwrap()(jitter)
}

pub unsafe fn mkl_jit_destroy(jitter: *mut c_void) -> mkl_jit_status_t {
    dyload_lib().mkl_jit_destroy.unwrap()(jitter)
}

pub unsafe fn cblas_saxpy_batch(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_saxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_daxpy_batch(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_daxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_caxpy_batch(
    n: *const MKL_INT,
    alpha: *const c_void,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    y: *mut *mut c_void,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_caxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_zaxpy_batch(
    n: *const MKL_INT,
    alpha: *const c_void,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    y: *mut *mut c_void,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zaxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_saxpy_batch_strided(
    N: MKL_INT,
    alpha: f32,
    X: *const f32,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut f32,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_saxpy_batch_strided.unwrap()(
        N, alpha, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_daxpy_batch_strided(
    N: MKL_INT,
    alpha: f64,
    X: *const f64,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut f64,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_daxpy_batch_strided.unwrap()(
        N, alpha, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_caxpy_batch_strided(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_caxpy_batch_strided.unwrap()(
        N, alpha, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_zaxpy_batch_strided(
    N: MKL_INT,
    alpha: *const c_void,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zaxpy_batch_strided.unwrap()(
        N, alpha, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_scopy_batch(
    n: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_scopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_dcopy_batch(
    n: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_dcopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_ccopy_batch(
    n: *const MKL_INT,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    y: *mut *mut c_void,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_ccopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_zcopy_batch(
    n: *const MKL_INT,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    y: *mut *mut c_void,
    incy: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zcopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn cblas_scopy_batch_strided(
    N: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut f32,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_scopy_batch_strided.unwrap()(
        N, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_dcopy_batch_strided(
    N: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut f64,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_dcopy_batch_strided.unwrap()(
        N, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_ccopy_batch_strided(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_ccopy_batch_strided.unwrap()(
        N, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_zcopy_batch_strided(
    N: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zcopy_batch_strided.unwrap()(
        N, X, incX, stridex, Y, incY, stridey, batch_size,
    )
}

pub unsafe fn cblas_sgemv_batch(
    Layout: CBLAS_LAYOUT,
    TransA: *const CBLAS_TRANSPOSE,
    M: *const MKL_INT,
    N: *const MKL_INT,
    alpha: *const f32,
    A: *mut *const f32,
    lda: *const MKL_INT,
    X: *mut *const f32,
    incX: *const MKL_INT,
    beta: *const f32,
    Y: *mut *mut f32,
    incY: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_sgemv_batch.unwrap()(
        Layout,
        TransA,
        M,
        N,
        alpha,
        A,
        lda,
        X,
        incX,
        beta,
        Y,
        incY,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_sgemv_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f32,
    A: *const f32,
    lda: MKL_INT,
    stridea: MKL_INT,
    X: *const f32,
    incX: MKL_INT,
    stridex: MKL_INT,
    beta: f32,
    Y: *mut f32,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_sgemv_batch_strided.unwrap()(
        Layout, TransA, M, N, alpha, A, lda, stridea, X, incX, stridex, beta, Y, incY, stridey,
        batch_size,
    )
}

pub unsafe fn cblas_dgemv_batch(
    Layout: CBLAS_LAYOUT,
    TransA: *const CBLAS_TRANSPOSE,
    M: *const MKL_INT,
    N: *const MKL_INT,
    alpha: *const f64,
    A: *mut *const f64,
    lda: *const MKL_INT,
    X: *mut *const f64,
    incX: *const MKL_INT,
    beta: *const f64,
    Y: *mut *mut f64,
    incY: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_dgemv_batch.unwrap()(
        Layout,
        TransA,
        M,
        N,
        alpha,
        A,
        lda,
        X,
        incX,
        beta,
        Y,
        incY,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_dgemv_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: f64,
    A: *const f64,
    lda: MKL_INT,
    stridea: MKL_INT,
    X: *const f64,
    incX: MKL_INT,
    stridex: MKL_INT,
    beta: f64,
    Y: *mut f64,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_dgemv_batch_strided.unwrap()(
        Layout, TransA, M, N, alpha, A, lda, stridea, X, incX, stridex, beta, Y, incY, stridey,
        batch_size,
    )
}

pub unsafe fn cblas_cgemv_batch(
    Layout: CBLAS_LAYOUT,
    TransA: *const CBLAS_TRANSPOSE,
    M: *const MKL_INT,
    N: *const MKL_INT,
    alpha: *const c_void,
    A: *mut *const c_void,
    lda: *const MKL_INT,
    X: *mut *const c_void,
    incX: *const MKL_INT,
    beta: *const c_void,
    Y: *mut *mut c_void,
    incY: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_cgemv_batch.unwrap()(
        Layout,
        TransA,
        M,
        N,
        alpha,
        A,
        lda,
        X,
        incX,
        beta,
        Y,
        incY,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_cgemv_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_cgemv_batch_strided.unwrap()(
        Layout, TransA, M, N, alpha, A, lda, stridea, X, incX, stridex, beta, Y, incY, stridey,
        batch_size,
    )
}

pub unsafe fn cblas_zgemv_batch(
    Layout: CBLAS_LAYOUT,
    TransA: *const CBLAS_TRANSPOSE,
    M: *const MKL_INT,
    N: *const MKL_INT,
    alpha: *const c_void,
    A: *mut *const c_void,
    lda: *const MKL_INT,
    X: *mut *const c_void,
    incX: *const MKL_INT,
    beta: *const c_void,
    Y: *mut *mut c_void,
    incY: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zgemv_batch.unwrap()(
        Layout,
        TransA,
        M,
        N,
        alpha,
        A,
        lda,
        X,
        incX,
        beta,
        Y,
        incY,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zgemv_batch_strided(
    Layout: CBLAS_LAYOUT,
    TransA: CBLAS_TRANSPOSE,
    M: MKL_INT,
    N: MKL_INT,
    alpha: *const c_void,
    A: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    X: *const c_void,
    incX: MKL_INT,
    stridex: MKL_INT,
    beta: *const c_void,
    Y: *mut c_void,
    incY: MKL_INT,
    stridey: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zgemv_batch_strided.unwrap()(
        Layout, TransA, M, N, alpha, A, lda, stridea, X, incX, stridex, beta, Y, incY, stridey,
        batch_size,
    )
}

pub unsafe fn cblas_sdgmm_batch(
    layout: CBLAS_LAYOUT,
    side: *const CBLAS_SIDE,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const f32,
    lda: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    c: *mut *mut f32,
    ldc: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_sdgmm_batch.unwrap()(
        layout,
        side,
        m,
        n,
        a,
        lda,
        x,
        incx,
        c,
        ldc,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_sdgmm_batch_strided(
    layout: CBLAS_LAYOUT,
    side: CBLAS_SIDE,
    m: MKL_INT,
    n: MKL_INT,
    a: *const f32,
    lda: MKL_INT,
    stridea: MKL_INT,
    x: *const f32,
    incx: MKL_INT,
    stridex: MKL_INT,
    c: *mut f32,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_sdgmm_batch_strided.unwrap()(
        layout, side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_ddgmm_batch(
    layout: CBLAS_LAYOUT,
    side: *const CBLAS_SIDE,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const f64,
    lda: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    c: *mut *mut f64,
    ldc: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_ddgmm_batch.unwrap()(
        layout,
        side,
        m,
        n,
        a,
        lda,
        x,
        incx,
        c,
        ldc,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_ddgmm_batch_strided(
    layout: CBLAS_LAYOUT,
    side: CBLAS_SIDE,
    m: MKL_INT,
    n: MKL_INT,
    a: *const f64,
    lda: MKL_INT,
    stridea: MKL_INT,
    x: *const f64,
    incx: MKL_INT,
    stridex: MKL_INT,
    c: *mut f64,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_ddgmm_batch_strided.unwrap()(
        layout, side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_cdgmm_batch(
    layout: CBLAS_LAYOUT,
    side: *const CBLAS_SIDE,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const c_void,
    lda: *const MKL_INT,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    c: *mut *mut c_void,
    ldc: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_cdgmm_batch.unwrap()(
        layout,
        side,
        m,
        n,
        a,
        lda,
        x,
        incx,
        c,
        ldc,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_cdgmm_batch_strided(
    layout: CBLAS_LAYOUT,
    side: CBLAS_SIDE,
    m: MKL_INT,
    n: MKL_INT,
    a: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    x: *const c_void,
    incx: MKL_INT,
    stridex: MKL_INT,
    c: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_cdgmm_batch_strided.unwrap()(
        layout, side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn cblas_zdgmm_batch(
    layout: CBLAS_LAYOUT,
    side: *const CBLAS_SIDE,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const c_void,
    lda: *const MKL_INT,
    x: *mut *const c_void,
    incx: *const MKL_INT,
    c: *mut *mut c_void,
    ldc: *const MKL_INT,
    group_count: MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cblas_zdgmm_batch.unwrap()(
        layout,
        side,
        m,
        n,
        a,
        lda,
        x,
        incx,
        c,
        ldc,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zdgmm_batch_strided(
    layout: CBLAS_LAYOUT,
    side: CBLAS_SIDE,
    m: MKL_INT,
    n: MKL_INT,
    a: *const c_void,
    lda: MKL_INT,
    stridea: MKL_INT,
    x: *const c_void,
    incx: MKL_INT,
    stridex: MKL_INT,
    c: *mut c_void,
    ldc: MKL_INT,
    stridec: MKL_INT,
    batch_size: MKL_INT,
) {
    dyload_lib().cblas_zdgmm_batch_strided.unwrap()(
        layout, side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}
