//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn openblas_set_num_threads(num_threads: c_int) {
    dyload_lib().openblas_set_num_threads.unwrap()(num_threads)
}

pub unsafe fn goto_set_num_threads(num_threads: c_int) {
    dyload_lib().goto_set_num_threads.unwrap()(num_threads)
}

pub unsafe fn openblas_set_num_threads_local(num_threads: c_int) -> c_int {
    dyload_lib().openblas_set_num_threads_local.unwrap()(num_threads)
}

pub unsafe fn openblas_get_num_threads() -> c_int {
    dyload_lib().openblas_get_num_threads.unwrap()()
}

pub unsafe fn openblas_get_num_procs() -> c_int {
    dyload_lib().openblas_get_num_procs.unwrap()()
}

pub unsafe fn openblas_get_config() -> *mut c_char {
    dyload_lib().openblas_get_config.unwrap()()
}

pub unsafe fn openblas_get_corename() -> *mut c_char {
    dyload_lib().openblas_get_corename.unwrap()()
}

pub unsafe fn openblas_set_threads_callback_function(callback: openblas_threads_callback) {
    dyload_lib().openblas_set_threads_callback_function.unwrap()(callback)
}

pub unsafe fn openblas_get_parallel() -> c_int {
    dyload_lib().openblas_get_parallel.unwrap()()
}

pub unsafe fn cblas_sdsdot(
    n: blas_int,
    alpha: f32,
    x: *const f32,
    incx: blas_int,
    y: *const f32,
    incy: blas_int,
) -> f32 {
    dyload_lib().cblas_sdsdot.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_dsdot(
    n: blas_int,
    x: *const f32,
    incx: blas_int,
    y: *const f32,
    incy: blas_int,
) -> f64 {
    dyload_lib().cblas_dsdot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_sdot(
    n: blas_int,
    x: *const f32,
    incx: blas_int,
    y: *const f32,
    incy: blas_int,
) -> f32 {
    dyload_lib().cblas_sdot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_ddot(
    n: blas_int,
    x: *const f64,
    incx: blas_int,
    y: *const f64,
    incy: blas_int,
) -> f64 {
    dyload_lib().cblas_ddot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_cdotu(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
) -> openblas_complex_float {
    dyload_lib().cblas_cdotu.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_cdotc(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
) -> openblas_complex_float {
    dyload_lib().cblas_cdotc.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_zdotu(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
) -> openblas_complex_double {
    dyload_lib().cblas_zdotu.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_zdotc(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
) -> openblas_complex_double {
    dyload_lib().cblas_zdotc.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_cdotu_sub(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_cdotu_sub.unwrap()(n, x, incx, y, incy, ret)
}

pub unsafe fn cblas_cdotc_sub(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_cdotc_sub.unwrap()(n, x, incx, y, incy, ret)
}

pub unsafe fn cblas_zdotu_sub(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_zdotu_sub.unwrap()(n, x, incx, y, incy, ret)
}

pub unsafe fn cblas_zdotc_sub(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *const c_void,
    incy: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_zdotc_sub.unwrap()(n, x, incx, y, incy, ret)
}

pub unsafe fn cblas_sasum(n: blas_int, x: *const f32, incx: blas_int) -> f32 {
    dyload_lib().cblas_sasum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dasum(n: blas_int, x: *const f64, incx: blas_int) -> f64 {
    dyload_lib().cblas_dasum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_scasum(n: blas_int, x: *const c_void, incx: blas_int) -> f32 {
    dyload_lib().cblas_scasum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dzasum(n: blas_int, x: *const c_void, incx: blas_int) -> f64 {
    dyload_lib().cblas_dzasum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_ssum(n: blas_int, x: *const f32, incx: blas_int) -> f32 {
    dyload_lib().cblas_ssum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dsum(n: blas_int, x: *const f64, incx: blas_int) -> f64 {
    dyload_lib().cblas_dsum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_scsum(n: blas_int, x: *const c_void, incx: blas_int) -> f32 {
    dyload_lib().cblas_scsum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dzsum(n: blas_int, x: *const c_void, incx: blas_int) -> f64 {
    dyload_lib().cblas_dzsum.unwrap()(n, x, incx)
}

pub unsafe fn cblas_snrm2(N: blas_int, X: *const f32, incX: blas_int) -> f32 {
    dyload_lib().cblas_snrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dnrm2(N: blas_int, X: *const f64, incX: blas_int) -> f64 {
    dyload_lib().cblas_dnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_scnrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f32 {
    dyload_lib().cblas_scnrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_dznrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f64 {
    dyload_lib().cblas_dznrm2.unwrap()(N, X, incX)
}

pub unsafe fn cblas_isamax(n: blas_int, x: *const f32, incx: blas_int) -> usize {
    dyload_lib().cblas_isamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_idamax(n: blas_int, x: *const f64, incx: blas_int) -> usize {
    dyload_lib().cblas_idamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_icamax(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_icamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_izamax(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_izamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_isamin(n: blas_int, x: *const f32, incx: blas_int) -> usize {
    dyload_lib().cblas_isamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_idamin(n: blas_int, x: *const f64, incx: blas_int) -> usize {
    dyload_lib().cblas_idamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_icamin(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_icamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_izamin(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_izamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_samax(n: blas_int, x: *const f32, incx: blas_int) -> f32 {
    dyload_lib().cblas_samax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_damax(n: blas_int, x: *const f64, incx: blas_int) -> f64 {
    dyload_lib().cblas_damax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_scamax(n: blas_int, x: *const c_void, incx: blas_int) -> f32 {
    dyload_lib().cblas_scamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dzamax(n: blas_int, x: *const c_void, incx: blas_int) -> f64 {
    dyload_lib().cblas_dzamax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_samin(n: blas_int, x: *const f32, incx: blas_int) -> f32 {
    dyload_lib().cblas_samin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_damin(n: blas_int, x: *const f64, incx: blas_int) -> f64 {
    dyload_lib().cblas_damin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_scamin(n: blas_int, x: *const c_void, incx: blas_int) -> f32 {
    dyload_lib().cblas_scamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_dzamin(n: blas_int, x: *const c_void, incx: blas_int) -> f64 {
    dyload_lib().cblas_dzamin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_ismax(n: blas_int, x: *const f32, incx: blas_int) -> usize {
    dyload_lib().cblas_ismax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_idmax(n: blas_int, x: *const f64, incx: blas_int) -> usize {
    dyload_lib().cblas_idmax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_icmax(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_icmax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_izmax(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_izmax.unwrap()(n, x, incx)
}

pub unsafe fn cblas_ismin(n: blas_int, x: *const f32, incx: blas_int) -> usize {
    dyload_lib().cblas_ismin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_idmin(n: blas_int, x: *const f64, incx: blas_int) -> usize {
    dyload_lib().cblas_idmin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_icmin(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_icmin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_izmin(n: blas_int, x: *const c_void, incx: blas_int) -> usize {
    dyload_lib().cblas_izmin.unwrap()(n, x, incx)
}

pub unsafe fn cblas_saxpy(
    n: blas_int,
    alpha: f32,
    x: *const f32,
    incx: blas_int,
    y: *mut f32,
    incy: blas_int,
) {
    dyload_lib().cblas_saxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_daxpy(
    n: blas_int,
    alpha: f64,
    x: *const f64,
    incx: blas_int,
    y: *mut f64,
    incy: blas_int,
) {
    dyload_lib().cblas_daxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_caxpy(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_caxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_zaxpy(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zaxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_caxpyc(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_caxpyc.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_zaxpyc(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zaxpyc.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn cblas_scopy(n: blas_int, x: *const f32, incx: blas_int, y: *mut f32, incy: blas_int) {
    dyload_lib().cblas_scopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_dcopy(n: blas_int, x: *const f64, incx: blas_int, y: *mut f64, incy: blas_int) {
    dyload_lib().cblas_dcopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_ccopy(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_ccopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_zcopy(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zcopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_sswap(n: blas_int, x: *mut f32, incx: blas_int, y: *mut f32, incy: blas_int) {
    dyload_lib().cblas_sswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_dswap(n: blas_int, x: *mut f64, incx: blas_int, y: *mut f64, incy: blas_int) {
    dyload_lib().cblas_dswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_cswap(
    n: blas_int,
    x: *mut c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_cswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_zswap(
    n: blas_int,
    x: *mut c_void,
    incx: blas_int,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zswap.unwrap()(n, x, incx, y, incy)
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
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incY: blas_int,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_csrot.unwrap()(n, x, incx, y, incY, c, s)
}

pub unsafe fn cblas_zdrot(
    n: blas_int,
    x: *const c_void,
    incx: blas_int,
    y: *mut c_void,
    incY: blas_int,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_zdrot.unwrap()(n, x, incx, y, incY, c, s)
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

pub unsafe fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32) {
    dyload_lib().cblas_srotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64) {
    dyload_lib().cblas_drotmg.unwrap()(d1, d2, b1, b2, P)
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

pub unsafe fn cblas_sgemv(
    order: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *const f32,
    lda: blas_int,
    x: *const f32,
    incx: blas_int,
    beta: f32,
    y: *mut f32,
    incy: blas_int,
) {
    dyload_lib().cblas_sgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cblas_dgemv(
    order: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *const f64,
    lda: blas_int,
    x: *const f64,
    incx: blas_int,
    beta: f64,
    y: *mut f64,
    incy: blas_int,
) {
    dyload_lib().cblas_dgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cblas_cgemv(
    order: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const c_void,
    a: *const c_void,
    lda: blas_int,
    x: *const c_void,
    incx: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_cgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cblas_zgemv(
    order: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const c_void,
    a: *const c_void,
    lda: blas_int,
    x: *const c_void,
    incx: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cblas_sger(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_sger.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dger(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dger.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cgeru(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_cgeru.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cgerc(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_cgerc.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zgeru(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zgeru.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zgerc(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zgerc.unwrap()(order, M, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_strsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_strsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtrsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctrsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztrsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztrsv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_strmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f32,
    lda: blas_int,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_strmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_dtrmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const f64,
    lda: blas_int,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ctrmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ztrmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    A: *const c_void,
    lda: blas_int,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztrmv.unwrap()(order, Uplo, TransA, Diag, N, A, lda, X, incX)
}

pub unsafe fn cblas_ssyr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    A: *mut f32,
    lda: blas_int,
) {
    dyload_lib().cblas_ssyr.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_dsyr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    A: *mut f64,
    lda: blas_int,
) {
    dyload_lib().cblas_dsyr.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_cher(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_cher.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_zher(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
    lda: blas_int,
) {
    dyload_lib().cblas_zher.unwrap()(order, Uplo, N, alpha, X, incX, A, lda)
}

pub unsafe fn cblas_ssyr2(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ssyr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_dsyr2(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dsyr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_cher2(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_cher2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_zher2(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zher2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A, lda)
}

pub unsafe fn cblas_sgbmv(
    order: CBLAS_ORDER,
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
        order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_dgbmv(
    order: CBLAS_ORDER,
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
        order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_cgbmv(
    order: CBLAS_ORDER,
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
        order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_zgbmv(
    order: CBLAS_ORDER,
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
        order, TransA, M, N, KL, KU, alpha, A, lda, X, incX, beta, Y, incY,
    )
}

pub unsafe fn cblas_ssbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ssbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dsbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dsbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_stbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_stbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dtbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ctbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ztbmv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stbsv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_stbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_dtbsv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dtbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ctbsv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ctbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_ztbsv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ztbsv.unwrap()(order, Uplo, TransA, Diag, N, K, A, lda, X, incX)
}

pub unsafe fn cblas_stpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f32,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dtpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f64,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ctpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ztpmv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztpmv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_stpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f32,
    X: *mut f32,
    incX: blas_int,
) {
    dyload_lib().cblas_stpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_dtpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const f64,
    X: *mut f64,
    incX: blas_int,
) {
    dyload_lib().cblas_dtpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ctpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ctpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ztpsv(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    Diag: CBLAS_DIAG,
    N: blas_int,
    Ap: *const c_void,
    X: *mut c_void,
    incX: blas_int,
) {
    dyload_lib().cblas_ztpsv.unwrap()(order, Uplo, TransA, Diag, N, Ap, X, incX)
}

pub unsafe fn cblas_ssymv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_ssymv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dsymv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dsymv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chemv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_chemv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhemv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zhemv.unwrap()(order, Uplo, N, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sspmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_sspmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_dspmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_dspmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sspr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Ap: *mut f32,
) {
    dyload_lib().cblas_sspr.unwrap()(order, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_dspr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Ap: *mut f64,
) {
    dyload_lib().cblas_dspr.unwrap()(order, Uplo, N, alpha, X, incX, Ap)
}

pub unsafe fn cblas_chpr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
) {
    dyload_lib().cblas_chpr.unwrap()(order, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_zhpr(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const c_void,
    incX: blas_int,
    A: *mut c_void,
) {
    dyload_lib().cblas_zhpr.unwrap()(order, Uplo, N, alpha, X, incX, A)
}

pub unsafe fn cblas_sspr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f32,
    X: *const f32,
    incX: blas_int,
    Y: *const f32,
    incY: blas_int,
    A: *mut f32,
) {
    dyload_lib().cblas_sspr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_dspr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: f64,
    X: *const f64,
    incX: blas_int,
    Y: *const f64,
    incY: blas_int,
    A: *mut f64,
) {
    dyload_lib().cblas_dspr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, A)
}

pub unsafe fn cblas_chpr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_chpr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_zhpr2(
    order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    N: blas_int,
    alpha: *const c_void,
    X: *const c_void,
    incX: blas_int,
    Y: *const c_void,
    incY: blas_int,
    Ap: *mut c_void,
) {
    dyload_lib().cblas_zhpr2.unwrap()(order, Uplo, N, alpha, X, incX, Y, incY, Ap)
}

pub unsafe fn cblas_chbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_chbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhbmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zhbmv.unwrap()(order, Uplo, N, K, alpha, A, lda, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_chpmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_chpmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_zhpmv(
    order: CBLAS_ORDER,
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
    dyload_lib().cblas_zhpmv.unwrap()(order, Uplo, N, alpha, Ap, X, incX, beta, Y, incY)
}

pub unsafe fn cblas_sgemm(
    Order: CBLAS_ORDER,
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
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dgemm(
    Order: CBLAS_ORDER,
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
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemm(
    Order: CBLAS_ORDER,
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
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemm3m(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_cgemm3m.unwrap()(
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemm(
    Order: CBLAS_ORDER,
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
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemm3m(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_zgemm3m.unwrap()(
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_sgemmt(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
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
    dyload_lib().cblas_sgemmt.unwrap()(
        Order, Uplo, TransA, TransB, M, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dgemmt(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
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
    dyload_lib().cblas_dgemmt.unwrap()(
        Order, Uplo, TransA, TransB, M, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cgemmt(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
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
    dyload_lib().cblas_cgemmt.unwrap()(
        Order, Uplo, TransA, TransB, M, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zgemmt(
    Order: CBLAS_ORDER,
    Uplo: CBLAS_UPLO,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
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
    dyload_lib().cblas_zgemmt.unwrap()(
        Order, Uplo, TransA, TransB, M, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_ssymm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ssymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_dsymm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_dsymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_csymm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_csymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zsymm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_zsymm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_ssyrk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ssyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_dsyrk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_dsyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_csyrk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_csyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zsyrk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_zsyrk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_ssyr2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_dsyr2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_csyr2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zsyr2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_strmm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_strmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dtrmm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_dtrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ctrmm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ctrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ztrmm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ztrmm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_strsm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_strsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_dtrsm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_dtrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ctrsm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ctrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_ztrsm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_ztrsm.unwrap()(Order, Side, Uplo, TransA, Diag, M, N, alpha, A, lda, B, ldb)
}

pub unsafe fn cblas_chemm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_chemm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_zhemm(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_zhemm.unwrap()(Order, Side, Uplo, M, N, alpha, A, lda, B, ldb, beta, C, ldc)
}

pub unsafe fn cblas_cherk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_cherk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_zherk(
    Order: CBLAS_ORDER,
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
    dyload_lib().cblas_zherk.unwrap()(Order, Uplo, Trans, N, K, alpha, A, lda, beta, C, ldc)
}

pub unsafe fn cblas_cher2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_zher2k(
    Order: CBLAS_ORDER,
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
        Order, Uplo, Trans, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_xerbla(p: blas_int, rout: *const c_char, form: *const c_char) {
    dyload_lib().cblas_xerbla.unwrap()(p, rout, form)
}

pub unsafe fn cblas_saxpby(
    n: blas_int,
    alpha: f32,
    x: *const f32,
    incx: blas_int,
    beta: f32,
    y: *mut f32,
    incy: blas_int,
) {
    dyload_lib().cblas_saxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn cblas_daxpby(
    n: blas_int,
    alpha: f64,
    x: *const f64,
    incx: blas_int,
    beta: f64,
    y: *mut f64,
    incy: blas_int,
) {
    dyload_lib().cblas_daxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn cblas_caxpby(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_caxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn cblas_zaxpby(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incx: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incy: blas_int,
) {
    dyload_lib().cblas_zaxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn cblas_somatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f32,
    a: *const f32,
    clda: blas_int,
    b: *mut f32,
    cldb: blas_int,
) {
    dyload_lib().cblas_somatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, b, cldb)
}

pub unsafe fn cblas_domatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f64,
    a: *const f64,
    clda: blas_int,
    b: *mut f64,
    cldb: blas_int,
) {
    dyload_lib().cblas_domatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, b, cldb)
}

pub unsafe fn cblas_comatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f32,
    a: *const f32,
    clda: blas_int,
    b: *mut f32,
    cldb: blas_int,
) {
    dyload_lib().cblas_comatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, b, cldb)
}

pub unsafe fn cblas_zomatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f64,
    a: *const f64,
    clda: blas_int,
    b: *mut f64,
    cldb: blas_int,
) {
    dyload_lib().cblas_zomatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, b, cldb)
}

pub unsafe fn cblas_simatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f32,
    a: *mut f32,
    clda: blas_int,
    cldb: blas_int,
) {
    dyload_lib().cblas_simatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, cldb)
}

pub unsafe fn cblas_dimatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f64,
    a: *mut f64,
    clda: blas_int,
    cldb: blas_int,
) {
    dyload_lib().cblas_dimatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, cldb)
}

pub unsafe fn cblas_cimatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f32,
    a: *mut f32,
    clda: blas_int,
    cldb: blas_int,
) {
    dyload_lib().cblas_cimatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, cldb)
}

pub unsafe fn cblas_zimatcopy(
    CORDER: CBLAS_ORDER,
    CTRANS: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f64,
    a: *mut f64,
    clda: blas_int,
    cldb: blas_int,
) {
    dyload_lib().cblas_zimatcopy.unwrap()(CORDER, CTRANS, crows, ccols, calpha, a, clda, cldb)
}

pub unsafe fn cblas_sgeadd(
    CORDER: CBLAS_ORDER,
    crows: blas_int,
    ccols: blas_int,
    calpha: f32,
    a: *const f32,
    clda: blas_int,
    cbeta: f32,
    c: *mut f32,
    cldc: blas_int,
) {
    dyload_lib().cblas_sgeadd.unwrap()(CORDER, crows, ccols, calpha, a, clda, cbeta, c, cldc)
}

pub unsafe fn cblas_dgeadd(
    CORDER: CBLAS_ORDER,
    crows: blas_int,
    ccols: blas_int,
    calpha: f64,
    a: *const f64,
    clda: blas_int,
    cbeta: f64,
    c: *mut f64,
    cldc: blas_int,
) {
    dyload_lib().cblas_dgeadd.unwrap()(CORDER, crows, ccols, calpha, a, clda, cbeta, c, cldc)
}

pub unsafe fn cblas_cgeadd(
    CORDER: CBLAS_ORDER,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f32,
    a: *const f32,
    clda: blas_int,
    cbeta: *const f32,
    c: *mut f32,
    cldc: blas_int,
) {
    dyload_lib().cblas_cgeadd.unwrap()(CORDER, crows, ccols, calpha, a, clda, cbeta, c, cldc)
}

pub unsafe fn cblas_zgeadd(
    CORDER: CBLAS_ORDER,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f64,
    a: *const f64,
    clda: blas_int,
    cbeta: *const f64,
    c: *mut f64,
    cldc: blas_int,
) {
    dyload_lib().cblas_zgeadd.unwrap()(CORDER, crows, ccols, calpha, a, clda, cbeta, c, cldc)
}

pub unsafe fn cblas_sgemm_batch(
    Order: CBLAS_ORDER,
    TransA_array: *const CBLAS_TRANSPOSE,
    TransB_array: *const CBLAS_TRANSPOSE,
    M_array: *const blas_int,
    N_array: *const blas_int,
    K_array: *const blas_int,
    alpha_array: *const f32,
    A_array: *mut *const f32,
    lda_array: *const blas_int,
    B_array: *mut *const f32,
    ldb_array: *const blas_int,
    beta_array: *const f32,
    C_array: *mut *mut f32,
    ldc_array: *const blas_int,
    group_count: blas_int,
    group_size: *const blas_int,
) {
    dyload_lib().cblas_sgemm_batch.unwrap()(
        Order,
        TransA_array,
        TransB_array,
        M_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        B_array,
        ldb_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_dgemm_batch(
    Order: CBLAS_ORDER,
    TransA_array: *const CBLAS_TRANSPOSE,
    TransB_array: *const CBLAS_TRANSPOSE,
    M_array: *const blas_int,
    N_array: *const blas_int,
    K_array: *const blas_int,
    alpha_array: *const f64,
    A_array: *mut *const f64,
    lda_array: *const blas_int,
    B_array: *mut *const f64,
    ldb_array: *const blas_int,
    beta_array: *const f64,
    C_array: *mut *mut f64,
    ldc_array: *const blas_int,
    group_count: blas_int,
    group_size: *const blas_int,
) {
    dyload_lib().cblas_dgemm_batch.unwrap()(
        Order,
        TransA_array,
        TransB_array,
        M_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        B_array,
        ldb_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_cgemm_batch(
    Order: CBLAS_ORDER,
    TransA_array: *const CBLAS_TRANSPOSE,
    TransB_array: *const CBLAS_TRANSPOSE,
    M_array: *const blas_int,
    N_array: *const blas_int,
    K_array: *const blas_int,
    alpha_array: *const c_void,
    A_array: *mut *const c_void,
    lda_array: *const blas_int,
    B_array: *mut *const c_void,
    ldb_array: *const blas_int,
    beta_array: *const c_void,
    C_array: *mut *mut c_void,
    ldc_array: *const blas_int,
    group_count: blas_int,
    group_size: *const blas_int,
) {
    dyload_lib().cblas_cgemm_batch.unwrap()(
        Order,
        TransA_array,
        TransB_array,
        M_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        B_array,
        ldb_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_zgemm_batch(
    Order: CBLAS_ORDER,
    TransA_array: *const CBLAS_TRANSPOSE,
    TransB_array: *const CBLAS_TRANSPOSE,
    M_array: *const blas_int,
    N_array: *const blas_int,
    K_array: *const blas_int,
    alpha_array: *const c_void,
    A_array: *mut *const c_void,
    lda_array: *const blas_int,
    B_array: *mut *const c_void,
    ldb_array: *const blas_int,
    beta_array: *const c_void,
    C_array: *mut *mut c_void,
    ldc_array: *const blas_int,
    group_count: blas_int,
    group_size: *const blas_int,
) {
    dyload_lib().cblas_zgemm_batch.unwrap()(
        Order,
        TransA_array,
        TransB_array,
        M_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        B_array,
        ldb_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cblas_sbstobf16(
    n: blas_int,
    in_: *const f32,
    incin: blas_int,
    out: *mut bfloat16,
    incout: blas_int,
) {
    dyload_lib().cblas_sbstobf16.unwrap()(n, in_, incin, out, incout)
}

pub unsafe fn cblas_sbdtobf16(
    n: blas_int,
    in_: *const f64,
    incin: blas_int,
    out: *mut bfloat16,
    incout: blas_int,
) {
    dyload_lib().cblas_sbdtobf16.unwrap()(n, in_, incin, out, incout)
}

pub unsafe fn cblas_sbf16tos(
    n: blas_int,
    in_: *const bfloat16,
    incin: blas_int,
    out: *mut f32,
    incout: blas_int,
) {
    dyload_lib().cblas_sbf16tos.unwrap()(n, in_, incin, out, incout)
}

pub unsafe fn cblas_dbf16tod(
    n: blas_int,
    in_: *const bfloat16,
    incin: blas_int,
    out: *mut f64,
    incout: blas_int,
) {
    dyload_lib().cblas_dbf16tod.unwrap()(n, in_, incin, out, incout)
}

pub unsafe fn cblas_sbdot(
    n: blas_int,
    x: *const bfloat16,
    incx: blas_int,
    y: *const bfloat16,
    incy: blas_int,
) -> f32 {
    dyload_lib().cblas_sbdot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cblas_sbgemv(
    order: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *const bfloat16,
    lda: blas_int,
    x: *const bfloat16,
    incx: blas_int,
    beta: f32,
    y: *mut f32,
    incy: blas_int,
) {
    dyload_lib().cblas_sbgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cblas_sbgemm(
    Order: CBLAS_ORDER,
    TransA: CBLAS_TRANSPOSE,
    TransB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const bfloat16,
    lda: blas_int,
    B: *const bfloat16,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_sbgemm.unwrap()(
        Order, TransA, TransB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_sbgemm_batch(
    Order: CBLAS_ORDER,
    TransA_array: *const CBLAS_TRANSPOSE,
    TransB_array: *const CBLAS_TRANSPOSE,
    M_array: *const blas_int,
    N_array: *const blas_int,
    K_array: *const blas_int,
    alpha_array: *const f32,
    A_array: *mut *const bfloat16,
    lda_array: *const blas_int,
    B_array: *mut *const bfloat16,
    ldb_array: *const blas_int,
    beta_array: *const f32,
    C_array: *mut *mut f32,
    ldc_array: *const blas_int,
    group_count: blas_int,
    group_size: *const blas_int,
) {
    dyload_lib().cblas_sbgemm_batch.unwrap()(
        Order,
        TransA_array,
        TransB_array,
        M_array,
        N_array,
        K_array,
        alpha_array,
        A_array,
        lda_array,
        B_array,
        ldb_array,
        beta_array,
        C_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn omp_set_num_threads(arg1: c_int) {
    dyload_lib().omp_set_num_threads.unwrap()(arg1)
}

pub unsafe fn omp_get_max_threads() -> c_int {
    dyload_lib().omp_get_max_threads.unwrap()()
}
