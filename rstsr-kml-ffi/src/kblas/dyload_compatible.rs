//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn BlasSetNumThreads(numThreads: c_int) {
    dyload_lib().BlasSetNumThreads.unwrap()(numThreads)
}

pub unsafe fn BlasSetNumThreadsLocal(numThreadsLocal: c_int) {
    dyload_lib().BlasSetNumThreadsLocal.unwrap()(numThreadsLocal)
}

pub unsafe fn BlasGetNumThreads() -> c_int {
    dyload_lib().BlasGetNumThreads.unwrap()()
}

pub unsafe fn BlasGetNumThreadsLocal() -> c_int {
    dyload_lib().BlasGetNumThreadsLocal.unwrap()()
}

pub unsafe fn BlasGetNumProcs() -> c_int {
    dyload_lib().BlasGetNumProcs.unwrap()()
}

pub unsafe fn BlasGetParallel() -> c_int {
    dyload_lib().BlasGetParallel.unwrap()()
}

pub unsafe fn KBLASGetVersion(ver: *mut KBLASVersion) -> c_int {
    dyload_lib().KBLASGetVersion.unwrap()(ver)
}

pub unsafe fn cblas_sdsdot(
    n: blas_int,
    alpha: f32,
    x: *const f32,
    incX: blas_int,
    y: *const f32,
    incY: blas_int,
) -> f32 {
    dyload_lib().cblas_sdsdot.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_dsdot(
    n: blas_int,
    x: *const f32,
    incX: blas_int,
    y: *const f32,
    incY: blas_int,
) -> f64 {
    dyload_lib().cblas_dsdot.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_sdot(
    n: blas_int,
    x: *const f32,
    incX: blas_int,
    y: *const f32,
    incY: blas_int,
) -> f32 {
    dyload_lib().cblas_sdot.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_ddot(
    n: blas_int,
    x: *const f64,
    incX: blas_int,
    y: *const f64,
    incY: blas_int,
) -> f64 {
    dyload_lib().cblas_ddot.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_cdotu(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
) -> blas_complex_float {
    dyload_lib().cblas_cdotu.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_cdotc(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
) -> blas_complex_float {
    dyload_lib().cblas_cdotc.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_zdotu(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
) -> blas_complex_double {
    dyload_lib().cblas_zdotu.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_zdotc(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
) -> blas_complex_double {
    dyload_lib().cblas_zdotc.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_cdotu_sub(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_cdotu_sub.unwrap()(n, x, incX, y, incY, ret)
}

pub unsafe fn cblas_cdotc_sub(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_cdotc_sub.unwrap()(n, x, incX, y, incY, ret)
}

pub unsafe fn cblas_zdotu_sub(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_zdotu_sub.unwrap()(n, x, incX, y, incY, ret)
}

pub unsafe fn cblas_zdotc_sub(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *const c_void,
    incY: blas_int,
    ret: *mut c_void,
) {
    dyload_lib().cblas_zdotc_sub.unwrap()(n, x, incX, y, incY, ret)
}

pub unsafe fn cblas_sasum(n: blas_int, x: *const f32, incX: blas_int) -> f32 {
    dyload_lib().cblas_sasum.unwrap()(n, x, incX)
}

pub unsafe fn cblas_dasum(n: blas_int, x: *const f64, incX: blas_int) -> f64 {
    dyload_lib().cblas_dasum.unwrap()(n, x, incX)
}

pub unsafe fn cblas_scasum(n: blas_int, x: *const c_void, incX: blas_int) -> f32 {
    dyload_lib().cblas_scasum.unwrap()(n, x, incX)
}

pub unsafe fn cblas_dzasum(n: blas_int, x: *const c_void, incX: blas_int) -> f64 {
    dyload_lib().cblas_dzasum.unwrap()(n, x, incX)
}

pub unsafe fn cblas_snrm2(n: blas_int, x: *const f32, incX: blas_int) -> f32 {
    dyload_lib().cblas_snrm2.unwrap()(n, x, incX)
}

pub unsafe fn cblas_dnrm2(n: blas_int, x: *const f64, incX: blas_int) -> f64 {
    dyload_lib().cblas_dnrm2.unwrap()(n, x, incX)
}

pub unsafe fn cblas_scnrm2(n: blas_int, x: *const c_void, incX: blas_int) -> f32 {
    dyload_lib().cblas_scnrm2.unwrap()(n, x, incX)
}

pub unsafe fn cblas_dznrm2(n: blas_int, x: *const c_void, incX: blas_int) -> f64 {
    dyload_lib().cblas_dznrm2.unwrap()(n, x, incX)
}

pub unsafe fn cblas_isamax(n: blas_int, x: *const f32, incX: blas_int) -> usize {
    dyload_lib().cblas_isamax.unwrap()(n, x, incX)
}

pub unsafe fn cblas_idamax(n: blas_int, x: *const f64, incX: blas_int) -> usize {
    dyload_lib().cblas_idamax.unwrap()(n, x, incX)
}

pub unsafe fn cblas_icamax(n: blas_int, x: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_icamax.unwrap()(n, x, incX)
}

pub unsafe fn cblas_izamax(n: blas_int, x: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_izamax.unwrap()(n, x, incX)
}

pub unsafe fn cblas_isamin(n: blas_int, x: *const f32, incX: blas_int) -> usize {
    dyload_lib().cblas_isamin.unwrap()(n, x, incX)
}

pub unsafe fn cblas_idamin(n: blas_int, x: *const f64, incX: blas_int) -> usize {
    dyload_lib().cblas_idamin.unwrap()(n, x, incX)
}

pub unsafe fn cblas_icamin(n: blas_int, x: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_icamin.unwrap()(n, x, incX)
}

pub unsafe fn cblas_izamin(n: blas_int, x: *const c_void, incX: blas_int) -> usize {
    dyload_lib().cblas_izamin.unwrap()(n, x, incX)
}

pub unsafe fn cblas_saxpy(
    n: blas_int,
    alpha: f32,
    x: *const f32,
    incX: blas_int,
    y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_saxpy.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_daxpy(
    n: blas_int,
    alpha: f64,
    x: *const f64,
    incX: blas_int,
    y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_daxpy.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_caxpy(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_caxpy.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_zaxpy(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zaxpy.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_caxpyc(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_caxpyc.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_zaxpyc(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zaxpyc.unwrap()(n, alpha, x, incX, y, incY)
}

pub unsafe fn cblas_somatcopy(
    cblasOrder: CBLAS_ORDER,
    cblasTrans: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f32,
    a: *const f32,
    clda: blas_int,
    b: *mut f32,
    cldb: blas_int,
) {
    dyload_lib().cblas_somatcopy.unwrap()(
        cblasOrder, cblasTrans, crows, ccols, calpha, a, clda, b, cldb,
    )
}

pub unsafe fn cblas_domatcopy(
    cblasOrder: CBLAS_ORDER,
    cblasTrans: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: f64,
    a: *const f64,
    clda: blas_int,
    b: *mut f64,
    cldb: blas_int,
) {
    dyload_lib().cblas_domatcopy.unwrap()(
        cblasOrder, cblasTrans, crows, ccols, calpha, a, clda, b, cldb,
    )
}

pub unsafe fn cblas_comatcopy(
    cblasOrder: CBLAS_ORDER,
    cblasTrans: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f32,
    a: *const f32,
    clda: blas_int,
    b: *mut f32,
    cldb: blas_int,
) {
    dyload_lib().cblas_comatcopy.unwrap()(
        cblasOrder, cblasTrans, crows, ccols, calpha, a, clda, b, cldb,
    )
}

pub unsafe fn cblas_zomatcopy(
    cblasOrder: CBLAS_ORDER,
    cblasTrans: CBLAS_TRANSPOSE,
    crows: blas_int,
    ccols: blas_int,
    calpha: *const f64,
    a: *const f64,
    clda: blas_int,
    b: *mut f64,
    cldb: blas_int,
) {
    dyload_lib().cblas_zomatcopy.unwrap()(
        cblasOrder, cblasTrans, crows, ccols, calpha, a, clda, b, cldb,
    )
}

pub unsafe fn cblas_simatcopy(
    ordering: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    rows: blas_int,
    cols: blas_int,
    alpha: f32,
    AB: *mut f32,
    lda: blas_int,
    ldb: blas_int,
) {
    dyload_lib().cblas_simatcopy.unwrap()(ordering, trans, rows, cols, alpha, AB, lda, ldb)
}

pub unsafe fn cblas_dimatcopy(
    ordering: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    rows: blas_int,
    cols: blas_int,
    alpha: f64,
    AB: *mut f64,
    lda: blas_int,
    ldb: blas_int,
) {
    dyload_lib().cblas_dimatcopy.unwrap()(ordering, trans, rows, cols, alpha, AB, lda, ldb)
}

pub unsafe fn cblas_cimatcopy(
    ordering: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    rows: blas_int,
    cols: blas_int,
    alpha: *const f32,
    AB: *mut f32,
    lda: blas_int,
    ldb: blas_int,
) {
    dyload_lib().cblas_cimatcopy.unwrap()(ordering, trans, rows, cols, alpha, AB, lda, ldb)
}

pub unsafe fn cblas_zimatcopy(
    ordering: CBLAS_ORDER,
    trans: CBLAS_TRANSPOSE,
    rows: blas_int,
    cols: blas_int,
    alpha: *const f64,
    AB: *mut f64,
    lda: blas_int,
    ldb: blas_int,
) {
    dyload_lib().cblas_zimatcopy.unwrap()(ordering, trans, rows, cols, alpha, AB, lda, ldb)
}

pub unsafe fn cblas_simatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_simatadd.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_dimatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dimatadd.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_cimatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_cimatadd.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_zimatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_zimatadd.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_somatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    beta: f32,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_somatadd.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_domatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    beta: f64,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_domatadd.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_comatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    beta: *const f32,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_comatadd.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_zomatadd(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    beta: *const f64,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_zomatadd.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_simatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_simatsub.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_dimatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dimatsub.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_cimatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_cimatsub.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_zimatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_zimatsub.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_somatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    beta: f32,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_somatsub.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_domatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    beta: f64,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_domatsub.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_comatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    beta: *const f32,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_comatsub.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_zomatsub(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    beta: *const f64,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_zomatsub.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, beta, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_simatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_simatmul.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_dimatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dimatmul.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_cimatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_cimatmul.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_zimatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_zimatmul.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_somatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_somatmul.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_domatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_domatmul.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_comatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_comatmul.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_chomatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f16,
    a: *mut f16,
    lda: blas_int,
    b: *mut f16,
    ldb: blas_int,
    c: *mut f16,
    ldc: blas_int,
) {
    dyload_lib().cblas_chomatmul.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_zomatmul(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_zomatmul.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_simatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_simatdiv.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_dimatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_dimatdiv.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_cimatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    bc: *mut f32,
    ldb: blas_int,
) {
    dyload_lib().cblas_cimatdiv.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_zimatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    bc: *mut f64,
    ldb: blas_int,
) {
    dyload_lib().cblas_zimatdiv.unwrap()(ordering, transa, m, n, alpha, a, lda, bc, ldb)
}

pub unsafe fn cblas_somatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f32,
    a: *mut f32,
    lda: blas_int,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_somatdiv.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_domatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: f64,
    a: *mut f64,
    lda: blas_int,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_domatdiv.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_comatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f32,
    a: *mut f32,
    lda: blas_int,
    b: *mut f32,
    ldb: blas_int,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_comatdiv.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_zomatdiv(
    ordering: CBLAS_ORDER,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    alpha: *const f64,
    a: *mut f64,
    lda: blas_int,
    b: *mut f64,
    ldb: blas_int,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_zomatdiv.unwrap()(
        ordering, transa, transb, m, n, alpha, a, lda, b, ldb, c, ldc,
    )
}

pub unsafe fn cblas_scopy(n: blas_int, x: *const f32, incX: blas_int, y: *mut f32, incY: blas_int) {
    dyload_lib().cblas_scopy.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_dcopy(n: blas_int, x: *const f64, incX: blas_int, y: *mut f64, incY: blas_int) {
    dyload_lib().cblas_dcopy.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_ccopy(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_ccopy.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_zcopy(
    n: blas_int,
    x: *const c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zcopy.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_sswap(n: blas_int, x: *mut f32, incX: blas_int, y: *mut f32, incY: blas_int) {
    dyload_lib().cblas_sswap.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_dswap(n: blas_int, x: *mut f64, incX: blas_int, y: *mut f64, incY: blas_int) {
    dyload_lib().cblas_dswap.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_cswap(
    n: blas_int,
    x: *mut c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_cswap.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_zswap(
    n: blas_int,
    x: *mut c_void,
    incX: blas_int,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zswap.unwrap()(n, x, incX, y, incY)
}

pub unsafe fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
    dyload_lib().cblas_srotg.unwrap()(a, b, c, s)
}

pub unsafe fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
    dyload_lib().cblas_drotg.unwrap()(a, b, c, s)
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
    X: *mut f32,
    incX: blas_int,
    Y: *mut f32,
    incY: blas_int,
    c: f32,
    s: f32,
) {
    dyload_lib().cblas_csrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_zdrot(
    N: blas_int,
    X: *mut f64,
    incX: blas_int,
    Y: *mut f64,
    incY: blas_int,
    c: f64,
    s: f64,
) {
    dyload_lib().cblas_zdrot.unwrap()(N, X, incX, Y, incY, c, s)
}

pub unsafe fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32) {
    dyload_lib().cblas_srotmg.unwrap()(d1, d2, b1, b2, P)
}

pub unsafe fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64) {
    dyload_lib().cblas_drotmg.unwrap()(d1, d2, b1, b2, P)
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
    incX: blas_int,
    beta: f32,
    y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_sgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incX, beta, y, incY)
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
    incX: blas_int,
    beta: f64,
    y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_dgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incX, beta, y, incY)
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
    incX: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_cgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incX, beta, y, incY)
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
    incX: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zgemv.unwrap()(order, trans, m, n, alpha, a, lda, x, incX, beta, y, incY)
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
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f32,
    a: *const f32,
    lda: blas_int,
    b: *const f32,
    ldb: blas_int,
    beta: f32,
    c: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_sgemm.unwrap()(
        order, transA, transB, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn cblas_dgemm(
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f64,
    a: *const f64,
    lda: blas_int,
    b: *const f64,
    ldb: blas_int,
    beta: f64,
    c: *mut f64,
    ldc: blas_int,
) {
    dyload_lib().cblas_dgemm.unwrap()(
        order, transA, transB, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
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

pub unsafe fn cblas_gemm_s8s8s32(
    layout: CBLAS_LAYOUT,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    offsetc: CBLAS_OFFSET,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f32,
    a: *const BLASINT8,
    lda: blas_int,
    oa: BLASINT8,
    b: *const BLASINT8,
    ldb: blas_int,
    ob: BLASINT8,
    beta: f32,
    c: *mut i32,
    ldc: blas_int,
    oc: *const i32,
) {
    dyload_lib().cblas_gemm_s8s8s32.unwrap()(
        layout, transa, transb, offsetc, m, n, k, alpha, a, lda, oa, b, ldb, ob, beta, c, ldc, oc,
    )
}

pub unsafe fn cblas_gemm_u8u8s32(
    layout: CBLAS_LAYOUT,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    offsetc: CBLAS_OFFSET,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f32,
    a: *const BLASUINT8,
    lda: blas_int,
    oa: BLASINT8,
    b: *const BLASUINT8,
    ldb: blas_int,
    ob: BLASINT8,
    beta: f32,
    c: *mut i32,
    ldc: blas_int,
    oc: *const i32,
) {
    dyload_lib().cblas_gemm_u8u8s32.unwrap()(
        layout, transa, transb, offsetc, m, n, k, alpha, a, lda, oa, b, ldb, ob, beta, c, ldc, oc,
    )
}

pub unsafe fn cblas_gemm_s8u8s32(
    layout: CBLAS_LAYOUT,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    offsetc: CBLAS_OFFSET,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f32,
    a: *const BLASINT8,
    lda: blas_int,
    oa: BLASINT8,
    b: *const BLASUINT8,
    ldb: blas_int,
    ob: BLASINT8,
    beta: f32,
    c: *mut i32,
    ldc: blas_int,
    oc: *const i32,
) {
    dyload_lib().cblas_gemm_s8u8s32.unwrap()(
        layout, transa, transb, offsetc, m, n, k, alpha, a, lda, oa, b, ldb, ob, beta, c, ldc, oc,
    )
}

pub unsafe fn cblas_gemm_u8s8s32(
    layout: CBLAS_LAYOUT,
    transa: CBLAS_TRANSPOSE,
    transb: CBLAS_TRANSPOSE,
    offsetc: CBLAS_OFFSET,
    m: blas_int,
    n: blas_int,
    k: blas_int,
    alpha: f32,
    a: *const BLASUINT8,
    lda: blas_int,
    oa: BLASINT8,
    b: *const BLASINT8,
    ldb: blas_int,
    ob: BLASINT8,
    beta: f32,
    c: *mut i32,
    ldc: blas_int,
    oc: *const i32,
) {
    dyload_lib().cblas_gemm_u8s8s32.unwrap()(
        layout, transa, transb, offsetc, m, n, k, alpha, a, lda, oa, b, ldb, ob, beta, c, ldc, oc,
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

pub unsafe fn cblas_hgemm(
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: f16,
    A: *const f16,
    lda: blas_int,
    B: *const f16,
    ldb: blas_int,
    beta: f16,
    C: *mut f16,
    ldc: blas_int,
) {
    dyload_lib().cblas_hgemm.unwrap()(
        order, transA, transB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_shgemm(
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
    M: blas_int,
    N: blas_int,
    K: blas_int,
    alpha: f32,
    A: *const f16,
    lda: blas_int,
    B: *const f16,
    ldb: blas_int,
    beta: f32,
    C: *mut f32,
    ldc: blas_int,
) {
    dyload_lib().cblas_shgemm.unwrap()(
        order, transA, transB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_chgemm(
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
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
    dyload_lib().cblas_chgemm.unwrap()(
        order, transA, transB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
    )
}

pub unsafe fn cblas_cshgemm(
    order: CBLAS_ORDER,
    transA: CBLAS_TRANSPOSE,
    transB: CBLAS_TRANSPOSE,
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
    dyload_lib().cblas_cshgemm.unwrap()(
        order, transA, transB, M, N, K, alpha, A, lda, B, ldb, beta, C, ldc,
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
    incX: blas_int,
    beta: f32,
    y: *mut f32,
    incY: blas_int,
) {
    dyload_lib().cblas_saxpby.unwrap()(n, alpha, x, incX, beta, y, incY)
}

pub unsafe fn cblas_daxpby(
    n: blas_int,
    alpha: f64,
    x: *const f64,
    incX: blas_int,
    beta: f64,
    y: *mut f64,
    incY: blas_int,
) {
    dyload_lib().cblas_daxpby.unwrap()(n, alpha, x, incX, beta, y, incY)
}

pub unsafe fn cblas_caxpby(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_caxpby.unwrap()(n, alpha, x, incX, beta, y, incY)
}

pub unsafe fn cblas_zaxpby(
    n: blas_int,
    alpha: *const c_void,
    x: *const c_void,
    incX: blas_int,
    beta: *const c_void,
    y: *mut c_void,
    incY: blas_int,
) {
    dyload_lib().cblas_zaxpby.unwrap()(n, alpha, x, incX, beta, y, incY)
}
