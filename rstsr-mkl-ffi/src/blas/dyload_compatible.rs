//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn xerbla_(srname: *const c_char, info: *const c_int, lsrname: c_int) {
    dyload_lib().xerbla_.unwrap()(srname, info, lsrname)
}

pub unsafe fn lsame_(ca: *const c_char, cb: *const c_char, lca: MKL_INT, lcb: MKL_INT) -> c_int {
    dyload_lib().lsame_.unwrap()(ca, cb, lca, lcb)
}

pub unsafe fn scabs1_(c: *const MKL_Complex8) -> f32 {
    dyload_lib().scabs1_.unwrap()(c)
}

pub unsafe fn sasum_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32 {
    dyload_lib().sasum_.unwrap()(n, x, incx)
}

pub unsafe fn saxpy_(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().saxpy_.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn saxpby_(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().saxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn saxpyi_(
    nz: *const MKL_INT,
    a: *const f32,
    x: *const f32,
    indx: *const MKL_INT,
    y: *mut f32,
) {
    dyload_lib().saxpyi_.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn scasum_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32 {
    dyload_lib().scasum_.unwrap()(n, x, incx)
}

pub unsafe fn scnrm2_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32 {
    dyload_lib().scnrm2_.unwrap()(n, x, incx)
}

pub unsafe fn scopy_(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().scopy_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn sdot_(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f32 {
    dyload_lib().sdot_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn sdoti_(
    nz: *const MKL_INT,
    x: *const f32,
    indx: *const MKL_INT,
    y: *const f32,
) -> f32 {
    dyload_lib().sdoti_.unwrap()(nz, x, indx, y)
}

pub unsafe fn sdsdot_(
    n: *const MKL_INT,
    sb: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f32 {
    dyload_lib().sdsdot_.unwrap()(n, sb, x, incx, y, incy)
}

pub unsafe fn sgthr_(nz: *const MKL_INT, y: *const f32, x: *mut f32, indx: *const MKL_INT) {
    dyload_lib().sgthr_.unwrap()(nz, y, x, indx)
}

pub unsafe fn sgthrz_(nz: *const MKL_INT, y: *mut f32, x: *mut f32, indx: *const MKL_INT) {
    dyload_lib().sgthrz_.unwrap()(nz, y, x, indx)
}

pub unsafe fn snrm2_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32 {
    dyload_lib().snrm2_.unwrap()(n, x, incx)
}

pub unsafe fn srot_(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().srot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn srotg_(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
    dyload_lib().srotg_.unwrap()(a, b, c, s)
}

pub unsafe fn sroti_(
    nz: *const MKL_INT,
    x: *mut f32,
    indx: *const MKL_INT,
    y: *mut f32,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().sroti_.unwrap()(nz, x, indx, y, c, s)
}

pub unsafe fn srotm_(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    param: *const f32,
) {
    dyload_lib().srotm_.unwrap()(n, x, incx, y, incy, param)
}

pub unsafe fn srotmg_(d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) {
    dyload_lib().srotmg_.unwrap()(d1, d2, x1, y1, param)
}

pub unsafe fn sscal_(n: *const MKL_INT, a: *const f32, x: *mut f32, incx: *const MKL_INT) {
    dyload_lib().sscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn ssctr_(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *mut f32) {
    dyload_lib().ssctr_.unwrap()(nz, x, indx, y)
}

pub unsafe fn sswap_(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().sswap_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn isamax_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().isamax_.unwrap()(n, x, incx)
}

pub unsafe fn isamin_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().isamin_.unwrap()(n, x, incx)
}

pub unsafe fn caxpy_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().caxpy_.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn caxpby_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().caxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn caxpyi_(
    nz: *const MKL_INT,
    a: *const MKL_Complex8,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *mut MKL_Complex8,
) {
    dyload_lib().caxpyi_.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn ccopy_(
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().ccopy_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cdotc_(
    pres: *mut MKL_Complex8,
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cdotc_.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn cdotci_(
    pres: *mut MKL_Complex8,
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *const MKL_Complex8,
) {
    dyload_lib().cdotci_.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn cdotu_(
    pres: *mut MKL_Complex8,
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cdotu_.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn cdotui_(
    pres: *mut MKL_Complex8,
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *const MKL_Complex8,
) {
    dyload_lib().cdotui_.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn cgthr_(
    nz: *const MKL_INT,
    y: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    indx: *const MKL_INT,
) {
    dyload_lib().cgthr_.unwrap()(nz, y, x, indx)
}

pub unsafe fn cgthrz_(
    nz: *const MKL_INT,
    y: *mut MKL_Complex8,
    x: *mut MKL_Complex8,
    indx: *const MKL_INT,
) {
    dyload_lib().cgthrz_.unwrap()(nz, y, x, indx)
}

pub unsafe fn crot_(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const MKL_Complex8,
) {
    dyload_lib().crot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn crotg_(
    a: *mut MKL_Complex8,
    b: *const MKL_Complex8,
    c: *mut f32,
    s: *mut MKL_Complex8,
) {
    dyload_lib().crotg_.unwrap()(a, b, c, s)
}

pub unsafe fn cscal_(
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().cscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn csctr_(
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *mut MKL_Complex8,
) {
    dyload_lib().csctr_.unwrap()(nz, x, indx, y)
}

pub unsafe fn csrot_(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().csrot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn csscal_(
    n: *const MKL_INT,
    a: *const f32,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().csscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn cswap_(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cswap_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn icamax_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().icamax_.unwrap()(n, x, incx)
}

pub unsafe fn icamin_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().icamin_.unwrap()(n, x, incx)
}

pub unsafe fn dcabs1_(z: *const MKL_Complex16) -> f64 {
    dyload_lib().dcabs1_.unwrap()(z)
}

pub unsafe fn dasum_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64 {
    dyload_lib().dasum_.unwrap()(n, x, incx)
}

pub unsafe fn daxpy_(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().daxpy_.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn daxpby_(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().daxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn daxpyi_(
    nz: *const MKL_INT,
    a: *const f64,
    x: *const f64,
    indx: *const MKL_INT,
    y: *mut f64,
) {
    dyload_lib().daxpyi_.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn dcopy_(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dcopy_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn ddot_(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
) -> f64 {
    dyload_lib().ddot_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn dsdot_(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f64 {
    dyload_lib().dsdot_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn ddoti_(
    nz: *const MKL_INT,
    x: *const f64,
    indx: *const MKL_INT,
    y: *const f64,
) -> f64 {
    dyload_lib().ddoti_.unwrap()(nz, x, indx, y)
}

pub unsafe fn dgthr_(nz: *const MKL_INT, y: *const f64, x: *mut f64, indx: *const MKL_INT) {
    dyload_lib().dgthr_.unwrap()(nz, y, x, indx)
}

pub unsafe fn dgthrz_(nz: *const MKL_INT, y: *mut f64, x: *mut f64, indx: *const MKL_INT) {
    dyload_lib().dgthrz_.unwrap()(nz, y, x, indx)
}

pub unsafe fn dnrm2_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64 {
    dyload_lib().dnrm2_.unwrap()(n, x, incx)
}

pub unsafe fn drot_(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().drot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn drotg_(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
    dyload_lib().drotg_.unwrap()(a, b, c, s)
}

pub unsafe fn droti_(
    nz: *const MKL_INT,
    x: *mut f64,
    indx: *const MKL_INT,
    y: *mut f64,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().droti_.unwrap()(nz, x, indx, y, c, s)
}

pub unsafe fn drotm_(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    param: *const f64,
) {
    dyload_lib().drotm_.unwrap()(n, x, incx, y, incy, param)
}

pub unsafe fn drotmg_(d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) {
    dyload_lib().drotmg_.unwrap()(d1, d2, x1, y1, param)
}

pub unsafe fn dscal_(n: *const MKL_INT, a: *const f64, x: *mut f64, incx: *const MKL_INT) {
    dyload_lib().dscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn dsctr_(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *mut f64) {
    dyload_lib().dsctr_.unwrap()(nz, x, indx, y)
}

pub unsafe fn dswap_(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dswap_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn dzasum_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64 {
    dyload_lib().dzasum_.unwrap()(n, x, incx)
}

pub unsafe fn dznrm2_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64 {
    dyload_lib().dznrm2_.unwrap()(n, x, incx)
}

pub unsafe fn idamax_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().idamax_.unwrap()(n, x, incx)
}

pub unsafe fn idamin_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().idamin_.unwrap()(n, x, incx)
}

pub unsafe fn zaxpy_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zaxpy_.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn zaxpby_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zaxpby_.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn zaxpyi_(
    nz: *const MKL_INT,
    a: *const MKL_Complex16,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *mut MKL_Complex16,
) {
    dyload_lib().zaxpyi_.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn zcopy_(
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zcopy_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn zdotc_(
    pres: *mut MKL_Complex16,
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zdotc_.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn zdotci_(
    pres: *mut MKL_Complex16,
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *const MKL_Complex16,
) {
    dyload_lib().zdotci_.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn zdotu_(
    pres: *mut MKL_Complex16,
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zdotu_.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn zdotui_(
    pres: *mut MKL_Complex16,
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *const MKL_Complex16,
) {
    dyload_lib().zdotui_.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn zdrot_(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().zdrot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn zdscal_(
    n: *const MKL_INT,
    a: *const f64,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().zdscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn zgthr_(
    nz: *const MKL_INT,
    y: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    indx: *const MKL_INT,
) {
    dyload_lib().zgthr_.unwrap()(nz, y, x, indx)
}

pub unsafe fn zgthrz_(
    nz: *const MKL_INT,
    y: *mut MKL_Complex16,
    x: *mut MKL_Complex16,
    indx: *const MKL_INT,
) {
    dyload_lib().zgthrz_.unwrap()(nz, y, x, indx)
}

pub unsafe fn zrot_(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const MKL_Complex16,
) {
    dyload_lib().zrot_.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn zrotg_(
    a: *mut MKL_Complex16,
    b: *const MKL_Complex16,
    c: *mut f64,
    s: *mut MKL_Complex16,
) {
    dyload_lib().zrotg_.unwrap()(a, b, c, s)
}

pub unsafe fn zscal_(
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().zscal_.unwrap()(n, a, x, incx)
}

pub unsafe fn zsctr_(
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *mut MKL_Complex16,
) {
    dyload_lib().zsctr_.unwrap()(nz, x, indx, y)
}

pub unsafe fn zswap_(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zswap_.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn izamax_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().izamax_.unwrap()(n, x, incx)
}

pub unsafe fn izamin_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().izamin_.unwrap()(n, x, incx)
}

pub unsafe fn sgbmv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    kl: *const MKL_INT,
    ku: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().sgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().sgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sger_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
    a: *mut f32,
    lda: *const MKL_INT,
) {
    dyload_lib().sger_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn ssbmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().ssbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sspmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    ap: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().sspmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn sspr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    ap: *mut f32,
) {
    dyload_lib().sspr_.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn sspr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
    ap: *mut f32,
) {
    dyload_lib().sspr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ssymv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().ssymv_.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn ssyr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    a: *mut f32,
    lda: *const MKL_INT,
) {
    dyload_lib().ssyr_.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn ssyr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
    a: *mut f32,
    lda: *const MKL_INT,
) {
    dyload_lib().ssyr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn stbmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn stbsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn stpmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f32,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn stpsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f32,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn strmv_(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    b: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().strmv_.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn strsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().strsv_.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn sgem2vu_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    x1: *const f32,
    incx1: *const MKL_INT,
    x2: *const f32,
    incx2: *const MKL_INT,
    beta: *const f32,
    y1: *mut f32,
    incy1: *const MKL_INT,
    y2: *mut f32,
    incy2: *const MKL_INT,
) {
    dyload_lib().sgem2vu_.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn cgbmv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    kl: *const MKL_INT,
    ku: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cgerc_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
    a: *mut MKL_Complex8,
    lda: *const MKL_INT,
) {
    dyload_lib().cgerc_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn cgeru_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
    a: *mut MKL_Complex8,
    lda: *const MKL_INT,
) {
    dyload_lib().cgeru_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn chbmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().chbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn chemv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().chemv_.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cher_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    a: *mut MKL_Complex8,
    lda: *const MKL_INT,
) {
    dyload_lib().cher_.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn cher2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
    a: *mut MKL_Complex8,
    lda: *const MKL_INT,
) {
    dyload_lib().cher2_.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn chpmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    ap: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().chpmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn chpr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    ap: *mut MKL_Complex8,
) {
    dyload_lib().chpr_.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn chpr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
    ap: *mut MKL_Complex8,
) {
    dyload_lib().chpr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ctbmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ctbsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ctpmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ctpsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ctrmv_(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctrmv_.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn ctrsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctrsv_.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn cgem2vc_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x1: *const MKL_Complex8,
    incx1: *const MKL_INT,
    x2: *const MKL_Complex8,
    incx2: *const MKL_INT,
    beta: *const MKL_Complex8,
    y1: *mut MKL_Complex8,
    incy1: *const MKL_INT,
    y2: *mut MKL_Complex8,
    incy2: *const MKL_INT,
) {
    dyload_lib().cgem2vc_.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn scgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const f32,
    lda: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().scgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dgbmv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    kl: *const MKL_INT,
    ku: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dger_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
    a: *mut f64,
    lda: *const MKL_INT,
) {
    dyload_lib().dger_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn dsbmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dsbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dspmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    ap: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dspmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn dspr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    ap: *mut f64,
) {
    dyload_lib().dspr_.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn dspr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
    ap: *mut f64,
) {
    dyload_lib().dspr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn dsymv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dsymv_.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dsyr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    a: *mut f64,
    lda: *const MKL_INT,
) {
    dyload_lib().dsyr_.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn dsyr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
    a: *mut f64,
    lda: *const MKL_INT,
) {
    dyload_lib().dsyr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn dtbmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn dtbsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn dtpmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f64,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn dtpsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f64,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn dtrmv_(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    b: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtrmv_.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn dtrsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtrsv_.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn dgem2vu_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    x1: *const f64,
    incx1: *const MKL_INT,
    x2: *const f64,
    incx2: *const MKL_INT,
    beta: *const f64,
    y1: *mut f64,
    incy1: *const MKL_INT,
    y2: *mut f64,
    incy2: *const MKL_INT,
) {
    dyload_lib().dgem2vu_.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn zgbmv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    kl: *const MKL_INT,
    ku: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zgbmv_.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zgerc_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
    a: *mut MKL_Complex16,
    lda: *const MKL_INT,
) {
    dyload_lib().zgerc_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zgeru_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
    a: *mut MKL_Complex16,
    lda: *const MKL_INT,
) {
    dyload_lib().zgeru_.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zhbmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zhbmv_.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zhemv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zhemv_.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zher_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    a: *mut MKL_Complex16,
    lda: *const MKL_INT,
) {
    dyload_lib().zher_.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn zher2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
    a: *mut MKL_Complex16,
    lda: *const MKL_INT,
) {
    dyload_lib().zher2_.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zhpmv_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    ap: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zhpmv_.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn zhpr_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    ap: *mut MKL_Complex16,
) {
    dyload_lib().zhpr_.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn zhpr2_(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
    ap: *mut MKL_Complex16,
) {
    dyload_lib().zhpr2_.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ztbmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztbmv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ztbsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztbsv_.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ztpmv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztpmv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ztpsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztpsv_.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ztrmv_(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztrmv_.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn ztrsv_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztrsv_.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn zgem2vc_(
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x1: *const MKL_Complex16,
    incx1: *const MKL_INT,
    x2: *const MKL_Complex16,
    incx2: *const MKL_INT,
    beta: *const MKL_Complex16,
    y1: *mut MKL_Complex16,
    incy1: *const MKL_INT,
    y2: *mut MKL_Complex16,
    incy2: *const MKL_INT,
) {
    dyload_lib().zgem2vc_.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn dzgemv_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const f64,
    lda: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().dzgemv_.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().sgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn sgemm_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().sgemm_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn sgemm_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    src: *const f32,
    ld: *const MKL_INT,
    dest: *mut f32,
) {
    dyload_lib().sgemm_pack_.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn sgemm_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().sgemm_compute_.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn sgemm_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const f32,
    a_array: *mut *const f32,
    lda_array: *const MKL_INT,
    b_array: *mut *const f32,
    ldb_array: *const MKL_INT,
    beta_array: *const f32,
    c_array: *mut *mut f32,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().sgemm_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn sgemm_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().sgemm_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn sgemmt_(
    uplo: *const c_char,
    transa: *const c_char,
    transb: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().sgemmt_.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssymm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().ssymm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssyr2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().ssyr2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssyrk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().ssyrk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn ssyrk_batch_(
    uplo_array: *const c_char,
    trans_array: *const c_char,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const f32,
    a_array: *mut *const f32,
    lda_array: *const MKL_INT,
    beta_array: *const f32,
    c_array: *mut *mut f32,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ssyrk_batch_.unwrap()(
        uplo_array,
        trans_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn ssyrk_batch_strided_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ssyrk_batch_strided_.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn strmm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *mut f32,
    ldb: *const MKL_INT,
) {
    dyload_lib().strmm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn strmm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().strmm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn strsm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *mut f32,
    ldb: *const MKL_INT,
) {
    dyload_lib().strsm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn strsm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const f32,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().strsm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn strsm_batch_(
    side_array: *const c_char,
    uplo_array: *const c_char,
    transa_array: *const c_char,
    diag_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    alpha_array: *const f32,
    a_array: *mut *const f32,
    lda_array: *const MKL_INT,
    b_array: *mut *mut f32,
    ldb: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().strsm_batch_.unwrap()(
        side_array,
        uplo_array,
        transa_array,
        diag_array,
        m_array,
        n_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb,
        group_count,
        group_size,
    )
}

pub unsafe fn strsm_batch_strided_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *mut f32,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().strsm_batch_strided_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn cgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().cgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex8,
    a_array: *mut *const MKL_Complex8,
    lda_array: *const MKL_INT,
    b_array: *mut *const MKL_Complex8,
    ldb_array: *const MKL_INT,
    beta_array: *const MKL_Complex8,
    c_array: *mut *mut MKL_Complex8,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cgemm_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cgemm_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().cgemm_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn scgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const f32,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().scgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm3m_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().cgemm3m_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm3m_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex8,
    a_array: *mut *const MKL_Complex8,
    lda_array: *const MKL_INT,
    b_array: *mut *const MKL_Complex8,
    ldb_array: *const MKL_INT,
    beta_array: *const MKL_Complex8,
    c_array: *mut *mut MKL_Complex8,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cgemm3m_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn cgemm3m_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().cgemm3m_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn cgemmt_(
    uplo: *const c_char,
    transa: *const c_char,
    transb: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().cgemmt_.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn chemm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().chemm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cher2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().cher2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cherk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    beta: *const f32,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().cherk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn csymm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().csymm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn csyr2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().csyr2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn csyrk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().csyrk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn csyrk_batch_(
    uplo_array: *const c_char,
    trans_array: *const c_char,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex8,
    a_array: *mut *const MKL_Complex8,
    lda_array: *const MKL_INT,
    beta_array: *const MKL_Complex8,
    c_array: *mut *mut MKL_Complex8,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().csyrk_batch_.unwrap()(
        uplo_array,
        trans_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn csyrk_batch_strided_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().csyrk_batch_strided_.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ctrmm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *mut MKL_Complex8,
    ldb: *const MKL_INT,
) {
    dyload_lib().ctrmm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ctrmm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().ctrmm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ctrsm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *mut MKL_Complex8,
    ldb: *const MKL_INT,
) {
    dyload_lib().ctrsm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ctrsm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *const MKL_Complex8,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex8,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
) {
    dyload_lib().ctrsm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ctrsm_batch_(
    side_array: *const c_char,
    uplo_array: *const c_char,
    transa_array: *const c_char,
    diag_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    alpha_array: *const MKL_Complex8,
    a_array: *mut *const MKL_Complex8,
    lda_array: *const MKL_INT,
    b_array: *mut *mut MKL_Complex8,
    ldb: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ctrsm_batch_.unwrap()(
        side_array,
        uplo_array,
        transa_array,
        diag_array,
        m_array,
        n_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb,
        group_count,
        group_size,
    )
}

pub unsafe fn ctrsm_batch_strided_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *mut MKL_Complex8,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ctrsm_batch_strided_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn dgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dgemm_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().dgemm_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn dgemm_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    src: *const f64,
    ld: *const MKL_INT,
    dest: *mut f64,
) {
    dyload_lib().dgemm_pack_.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn dgemm_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dgemm_compute_.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dgemm_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const f64,
    a_array: *mut *const f64,
    lda_array: *const MKL_INT,
    b_array: *mut *const f64,
    ldb_array: *const MKL_INT,
    beta_array: *const f64,
    c_array: *mut *mut f64,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dgemm_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn dgemm_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dgemm_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn dgemmt_(
    uplo: *const c_char,
    transa: *const c_char,
    transb: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dgemmt_.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsymm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dsymm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsyr2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dsyr2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsyrk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dsyrk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn dsyrk_batch_(
    uplo_array: *const c_char,
    trans_array: *const c_char,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const f64,
    a_array: *mut *const f64,
    lda_array: *const MKL_INT,
    beta_array: *const f64,
    c_array: *mut *mut f64,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dsyrk_batch_.unwrap()(
        uplo_array,
        trans_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn dsyrk_batch_strided_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dsyrk_batch_strided_.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn dtrmm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *mut f64,
    ldb: *const MKL_INT,
) {
    dyload_lib().dtrmm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn dtrmm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dtrmm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn dtrsm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *mut f64,
    ldb: *const MKL_INT,
) {
    dyload_lib().dtrsm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn dtrsm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const f64,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut f64,
    ldc: *const MKL_INT,
) {
    dyload_lib().dtrsm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn dtrsm_batch_(
    side_array: *const c_char,
    uplo_array: *const c_char,
    transa_array: *const c_char,
    diag_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    alpha_array: *const f64,
    a_array: *mut *const f64,
    lda_array: *const MKL_INT,
    b_array: *mut *mut f64,
    ldb: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dtrsm_batch_.unwrap()(
        side_array,
        uplo_array,
        transa_array,
        diag_array,
        m_array,
        n_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb,
        group_count,
        group_size,
    )
}

pub unsafe fn dtrsm_batch_strided_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *mut f64,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dtrsm_batch_strided_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn zgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex16,
    a_array: *mut *const MKL_Complex16,
    lda_array: *const MKL_INT,
    b_array: *mut *const MKL_Complex16,
    ldb_array: *const MKL_INT,
    beta_array: *const MKL_Complex16,
    c_array: *mut *mut MKL_Complex16,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zgemm_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn zgemm_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zgemm_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn dzgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const f64,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().dzgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm3m_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zgemm3m_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm3m_batch_(
    transa_array: *const c_char,
    transb_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex16,
    a_array: *mut *const MKL_Complex16,
    lda_array: *const MKL_INT,
    b_array: *mut *const MKL_Complex16,
    ldb_array: *const MKL_INT,
    beta_array: *const MKL_Complex16,
    c_array: *mut *mut MKL_Complex16,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zgemm3m_batch_.unwrap()(
        transa_array,
        transb_array,
        m_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn zgemm3m_batch_strided_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zgemm3m_batch_strided_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn zgemmt_(
    uplo: *const c_char,
    transa: *const c_char,
    transb: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zgemmt_.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zhemm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zhemm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zher2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const f64,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zher2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zherk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f64,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    beta: *const f64,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zherk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn zsymm_(
    side: *const c_char,
    uplo: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zsymm_.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zsyr2k_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zsyr2k_.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zsyrk_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().zsyrk_.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn zsyrk_batch_(
    uplo_array: *const c_char,
    trans_array: *const c_char,
    n_array: *const MKL_INT,
    k_array: *const MKL_INT,
    alpha_array: *const MKL_Complex16,
    a_array: *mut *const MKL_Complex16,
    lda_array: *const MKL_INT,
    beta_array: *const MKL_Complex16,
    c_array: *mut *mut MKL_Complex16,
    ldc_array: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zsyrk_batch_.unwrap()(
        uplo_array,
        trans_array,
        n_array,
        k_array,
        alpha_array,
        a_array,
        lda_array,
        beta_array,
        c_array,
        ldc_array,
        group_count,
        group_size,
    )
}

pub unsafe fn zsyrk_batch_strided_(
    uplo: *const c_char,
    trans: *const c_char,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zsyrk_batch_strided_.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ztrmm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *mut MKL_Complex16,
    ldb: *const MKL_INT,
) {
    dyload_lib().ztrmm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ztrmm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().ztrmm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ztrsm_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *mut MKL_Complex16,
    ldb: *const MKL_INT,
) {
    dyload_lib().ztrsm_.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ztrsm_oop_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *const MKL_Complex16,
    ldb: *const MKL_INT,
    beta: *const MKL_Complex16,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
) {
    dyload_lib().ztrsm_oop_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ztrsm_batch_(
    side_array: *const c_char,
    uplo_array: *const c_char,
    transa_array: *const c_char,
    diag_array: *const c_char,
    m_array: *const MKL_INT,
    n_array: *const MKL_INT,
    alpha_array: *const MKL_Complex16,
    a_array: *mut *const MKL_Complex16,
    lda_array: *const MKL_INT,
    b_array: *mut *mut MKL_Complex16,
    ldb: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ztrsm_batch_.unwrap()(
        side_array,
        uplo_array,
        transa_array,
        diag_array,
        m_array,
        n_array,
        alpha_array,
        a_array,
        lda_array,
        b_array,
        ldb,
        group_count,
        group_size,
    )
}

pub unsafe fn ztrsm_batch_strided_(
    side: *const c_char,
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    b: *mut MKL_Complex16,
    ldb: *const MKL_INT,
    strideb: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ztrsm_batch_strided_.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn gemm_s16s16s32_(
    transa: *const c_char,
    transb: *const c_char,
    offsetc: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_INT16,
    lda: *const MKL_INT,
    ao: *const MKL_INT16,
    b: *const MKL_INT16,
    ldb: *const MKL_INT,
    bo: *const MKL_INT16,
    beta: *const f32,
    c: *mut MKL_INT32,
    ldc: *const MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().gemm_s16s16s32_.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_s8u8s32_(
    transa: *const c_char,
    transb: *const c_char,
    offsetc: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_INT8,
    lda: *const MKL_INT,
    ao: *const MKL_INT8,
    b: *const MKL_UINT8,
    ldb: *const MKL_INT,
    bo: *const MKL_INT8,
    beta: *const f32,
    c: *mut MKL_INT32,
    ldc: *const MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().gemm_s8u8s32_.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_bf16bf16f32_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_BF16,
    lda: *const MKL_INT,
    b: *const MKL_BF16,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_bf16bf16f32_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_f16f16f32_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_F16,
    lda: *const MKL_INT,
    b: *const MKL_F16,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_f16f16f32_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e5m2e5m2f32_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_E5M2,
    lda: *const MKL_INT,
    b: *const MKL_E5M2,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_e5m2e5m2f32_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e4m3e4m3f32_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_E4M3,
    lda: *const MKL_INT,
    b: *const MKL_E4M3,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_e4m3e4m3f32_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_s8u8s32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_s8u8s32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_s16s16s32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_s16s16s32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_bf16bf16f32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_bf16bf16f32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_f16f16f32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_f16f16f32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_e5m2e5m2f32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_e5m2e5m2f32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_e4m3e4m3f32_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_e4m3e4m3f32_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_s8u8s32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const c_void,
    ld: *const MKL_INT,
    dest: *mut c_void,
) {
    dyload_lib().gemm_s8u8s32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_s16s16s32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_INT16,
    ld: *const MKL_INT,
    dest: *mut MKL_INT16,
) {
    dyload_lib().gemm_s16s16s32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_bf16bf16f32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_BF16,
    ld: *const MKL_INT,
    dest: *mut MKL_BF16,
) {
    dyload_lib().gemm_bf16bf16f32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_f16f16f32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_F16,
    ld: *const MKL_INT,
    dest: *mut MKL_F16,
) {
    dyload_lib().gemm_f16f16f32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_e5m2e5m2f32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_E5M2,
    ld: *const MKL_INT,
    dest: *mut MKL_E5M2,
) {
    dyload_lib().gemm_e5m2e5m2f32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_e4m3e4m3f32_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_E4M3,
    ld: *const MKL_INT,
    dest: *mut MKL_E4M3,
) {
    dyload_lib().gemm_e4m3e4m3f32_pack_.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_s8u8s32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    offsetc: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_INT8,
    lda: *const MKL_INT,
    ao: *const MKL_INT8,
    b: *const MKL_UINT8,
    ldb: *const MKL_INT,
    bo: *const MKL_INT8,
    beta: *const f32,
    c: *mut MKL_INT32,
    ldc: *const MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().gemm_s8u8s32_compute_.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_s16s16s32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    offsetc: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_INT16,
    lda: *const MKL_INT,
    ao: *const MKL_INT16,
    b: *const MKL_INT16,
    ldb: *const MKL_INT,
    bo: *const MKL_INT16,
    beta: *const f32,
    c: *mut MKL_INT32,
    ldc: *const MKL_INT,
    co: *const MKL_INT32,
) {
    dyload_lib().gemm_s16s16s32_compute_.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_bf16bf16f32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_BF16,
    lda: *const MKL_INT,
    b: *const MKL_BF16,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_bf16bf16f32_compute_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_f16f16f32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_F16,
    lda: *const MKL_INT,
    b: *const MKL_F16,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_f16f16f32_compute_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e5m2e5m2f32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_E5M2,
    lda: *const MKL_INT,
    b: *const MKL_E5M2,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_e5m2e5m2f32_compute_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e4m3e4m3f32_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const f32,
    a: *const MKL_E4M3,
    lda: *const MKL_INT,
    b: *const MKL_E4M3,
    ldb: *const MKL_INT,
    beta: *const f32,
    c: *mut f32,
    ldc: *const MKL_INT,
) {
    dyload_lib().gemm_e4m3e4m3f32_compute_.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn hgemm_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_F16,
    a: *const MKL_F16,
    lda: *const MKL_INT,
    b: *const MKL_F16,
    ldb: *const MKL_INT,
    beta: *const MKL_F16,
    c: *mut MKL_F16,
    ldc: *const MKL_INT,
) {
    dyload_lib().hgemm_.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn hgemm_pack_get_size_(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().hgemm_pack_get_size_.unwrap()(identifier, m, n, k)
}

pub unsafe fn hgemm_pack_(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    alpha: *const MKL_F16,
    src: *const MKL_F16,
    ld: *const MKL_INT,
    dest: *mut MKL_F16,
) {
    dyload_lib().hgemm_pack_.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn hgemm_compute_(
    transa: *const c_char,
    transb: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    a: *const MKL_F16,
    lda: *const MKL_INT,
    b: *const MKL_F16,
    ldb: *const MKL_INT,
    beta: *const MKL_F16,
    c: *mut MKL_F16,
    ldc: *const MKL_INT,
) {
    dyload_lib().hgemm_compute_.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn mkl_cblas_jit_create_dgemm_(
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
    dyload_lib().mkl_cblas_jit_create_dgemm_.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_sgemm_(
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
    dyload_lib().mkl_cblas_jit_create_sgemm_.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_cgemm_(
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
    dyload_lib().mkl_cblas_jit_create_cgemm_.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_cblas_jit_create_zgemm_(
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
    dyload_lib().mkl_cblas_jit_create_zgemm_.unwrap()(
        jitter, layout, transa, transb, m, n, k, alpha, lda, ldb, beta, ldc,
    )
}

pub unsafe fn mkl_jit_get_dgemm_ptr_(jitter: *const c_void) -> dgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_dgemm_ptr_.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_sgemm_ptr_(jitter: *const c_void) -> sgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_sgemm_ptr_.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_cgemm_ptr_(jitter: *const c_void) -> cgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_cgemm_ptr_.unwrap()(jitter)
}

pub unsafe fn mkl_jit_get_zgemm_ptr_(jitter: *const c_void) -> zgemm_jit_kernel_t {
    dyload_lib().mkl_jit_get_zgemm_ptr_.unwrap()(jitter)
}

pub unsafe fn mkl_jit_destroy_(jitter: *mut c_void) -> mkl_jit_status_t {
    dyload_lib().mkl_jit_destroy_.unwrap()(jitter)
}

pub unsafe fn saxpy_batch_(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().saxpy_batch_.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn daxpy_batch_(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().daxpy_batch_.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn caxpy_batch_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex8,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().caxpy_batch_.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn zaxpy_batch_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex16,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zaxpy_batch_.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn scopy_batch_(
    n: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().scopy_batch_.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn dcopy_batch_(
    n: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dcopy_batch_.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn ccopy_batch_(
    n: *const MKL_INT,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex8,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ccopy_batch_.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn zcopy_batch_(
    n: *const MKL_INT,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex16,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zcopy_batch_.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn saxpy_batch_strided_(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().saxpy_batch_strided_.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn daxpy_batch_strided_(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().daxpy_batch_strided_.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn caxpy_batch_strided_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().caxpy_batch_strided_.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn zaxpy_batch_strided_(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zaxpy_batch_strided_.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn scopy_batch_strided_(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().scopy_batch_strided_.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn dcopy_batch_strided_(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dcopy_batch_strided_.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn ccopy_batch_strided_(
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ccopy_batch_strided_.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn zcopy_batch_strided_(
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zcopy_batch_strided_.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn sgemv_batch_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *mut *const f32,
    lda: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().sgemv_batch_.unwrap()(
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
        group_count,
        group_size,
    )
}

pub unsafe fn sgemv_batch_strided_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f32,
    a: *const f32,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().sgemv_batch_strided_.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn dgemv_batch_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *mut *const f64,
    lda: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dgemv_batch_.unwrap()(
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
        group_count,
        group_size,
    )
}

pub unsafe fn dgemv_batch_strided_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const f64,
    a: *const f64,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dgemv_batch_strided_.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn cgemv_batch_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *mut *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut *mut MKL_Complex8,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cgemv_batch_.unwrap()(
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
        group_count,
        group_size,
    )
}

pub unsafe fn cgemv_batch_strided_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().cgemv_batch_strided_.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn zgemv_batch_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *mut *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut *mut MKL_Complex16,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zgemv_batch_.unwrap()(
        trans,
        m,
        n,
        alpha,
        a,
        lda,
        x,
        incx,
        beta,
        y,
        incy,
        group_count,
        group_size,
    )
}

pub unsafe fn zgemv_batch_strided_(
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zgemv_batch_strided_.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn sdgmm_batch_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const f32,
    lda: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    c: *mut *mut f32,
    ldc: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().sdgmm_batch_.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn sdgmm_batch_strided_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    c: *mut f32,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().sdgmm_batch_strided_.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ddgmm_batch_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const f64,
    lda: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    c: *mut *mut f64,
    ldc: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ddgmm_batch_.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn ddgmm_batch_strided_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    c: *mut f64,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ddgmm_batch_strided_.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn cdgmm_batch_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    c: *mut *mut MKL_Complex8,
    ldc: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().cdgmm_batch_.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn cdgmm_batch_strided_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    c: *mut MKL_Complex8,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().cdgmm_batch_strided_.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn zdgmm_batch_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *mut *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    c: *mut *mut MKL_Complex16,
    ldc: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zdgmm_batch_.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn zdgmm_batch_strided_(
    side: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    stridea: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    c: *mut MKL_Complex16,
    ldc: *const MKL_INT,
    stridec: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zdgmm_batch_strided_.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

/* #region upper case alias */

pub use caxpby_ as CAXPBY;
pub use caxpy_ as CAXPY;
pub use caxpy_batch_ as CAXPY_BATCH;
pub use caxpy_batch_strided_ as CAXPY_BATCH_STRIDED;
pub use caxpyi_ as CAXPYI;
pub use ccopy_ as CCOPY;
pub use ccopy_batch_ as CCOPY_BATCH;
pub use ccopy_batch_strided_ as CCOPY_BATCH_STRIDED;
pub use cdgmm_batch_ as CDGMM_BATCH;
pub use cdgmm_batch_strided_ as CDGMM_BATCH_STRIDED;
pub use cdotc_ as CDOTC;
pub use cdotci_ as CDOTCI;
pub use cdotu_ as CDOTU;
pub use cdotui_ as CDOTUI;
pub use cgbmv_ as CGBMV;
pub use cgem2vc_ as CGEM2VC;
pub use cgemm3m_ as CGEMM3M;
pub use cgemm3m_batch_ as CGEMM3M_BATCH;
pub use cgemm3m_batch_strided_ as CGEMM3M_BATCH_STRIDED;
pub use cgemm_ as CGEMM;
pub use cgemm_batch_ as CGEMM_BATCH;
pub use cgemm_batch_strided_ as CGEMM_BATCH_STRIDED;
pub use cgemmt_ as CGEMMT;
pub use cgemv_ as CGEMV;
pub use cgemv_batch_ as CGEMV_BATCH;
pub use cgemv_batch_strided_ as CGEMV_BATCH_STRIDED;
pub use cgerc_ as CGERC;
pub use cgeru_ as CGERU;
pub use cgthr_ as CGTHR;
pub use cgthrz_ as CGTHRZ;
pub use chbmv_ as CHBMV;
pub use chemm_ as CHEMM;
pub use chemv_ as CHEMV;
pub use cher2_ as CHER2;
pub use cher2k_ as CHER2K;
pub use cher_ as CHER;
pub use cherk_ as CHERK;
pub use chpmv_ as CHPMV;
pub use chpr2_ as CHPR2;
pub use chpr_ as CHPR;
pub use crot_ as CROT;
pub use crotg_ as CROTG;
pub use cscal_ as CSCAL;
pub use csctr_ as CSCTR;
pub use csrot_ as CSROT;
pub use csscal_ as CSSCAL;
pub use cswap_ as CSWAP;
pub use csymm_ as CSYMM;
pub use csyr2k_ as CSYR2K;
pub use csyrk_ as CSYRK;
pub use csyrk_batch_ as CSYRK_BATCH;
pub use csyrk_batch_strided_ as CSYRK_BATCH_STRIDED;
pub use ctbmv_ as CTBMV;
pub use ctbsv_ as CTBSV;
pub use ctpmv_ as CTPMV;
pub use ctpsv_ as CTPSV;
pub use ctrmm_ as CTRMM;
pub use ctrmm_oop_ as CTRMM_OOP;
pub use ctrmv_ as CTRMV;
pub use ctrsm_ as CTRSM;
pub use ctrsm_batch_ as CTRSM_BATCH;
pub use ctrsm_batch_strided_ as CTRSM_BATCH_STRIDED;
pub use ctrsm_oop_ as CTRSM_OOP;
pub use ctrsv_ as CTRSV;
pub use dasum_ as DASUM;
pub use daxpby_ as DAXPBY;
pub use daxpy_ as DAXPY;
pub use daxpy_batch_ as DAXPY_BATCH;
pub use daxpy_batch_strided_ as DAXPY_BATCH_STRIDED;
pub use daxpyi_ as DAXPYI;
pub use dcabs1_ as DCABS1;
pub use dcopy_ as DCOPY;
pub use dcopy_batch_ as DCOPY_BATCH;
pub use dcopy_batch_strided_ as DCOPY_BATCH_STRIDED;
pub use ddgmm_batch_ as DDGMM_BATCH;
pub use ddgmm_batch_strided_ as DDGMM_BATCH_STRIDED;
pub use ddot_ as DDOT;
pub use ddoti_ as DDOTI;
pub use dgbmv_ as DGBMV;
pub use dgem2vu_ as DGEM2VU;
pub use dgemm_ as DGEMM;
pub use dgemm_batch_ as DGEMM_BATCH;
pub use dgemm_batch_strided_ as DGEMM_BATCH_STRIDED;
pub use dgemm_compute_ as DGEMM_COMPUTE;
pub use dgemm_pack_ as DGEMM_PACK;
pub use dgemm_pack_get_size_ as DGEMM_PACK_GET_SIZE;
pub use dgemmt_ as DGEMMT;
pub use dgemv_ as DGEMV;
pub use dgemv_batch_ as DGEMV_BATCH;
pub use dgemv_batch_strided_ as DGEMV_BATCH_STRIDED;
pub use dger_ as DGER;
pub use dgthr_ as DGTHR;
pub use dgthrz_ as DGTHRZ;
pub use dnrm2_ as DNRM2;
pub use drot_ as DROT;
pub use drotg_ as DROTG;
pub use droti_ as DROTI;
pub use drotm_ as DROTM;
pub use drotmg_ as DROTMG;
pub use dsbmv_ as DSBMV;
pub use dscal_ as DSCAL;
pub use dsctr_ as DSCTR;
pub use dsdot_ as DSDOT;
pub use dspmv_ as DSPMV;
pub use dspr2_ as DSPR2;
pub use dspr_ as DSPR;
pub use dswap_ as DSWAP;
pub use dsymm_ as DSYMM;
pub use dsymv_ as DSYMV;
pub use dsyr2_ as DSYR2;
pub use dsyr2k_ as DSYR2K;
pub use dsyr_ as DSYR;
pub use dsyrk_ as DSYRK;
pub use dsyrk_batch_ as DSYRK_BATCH;
pub use dsyrk_batch_strided_ as DSYRK_BATCH_STRIDED;
pub use dtbmv_ as DTBMV;
pub use dtbsv_ as DTBSV;
pub use dtpmv_ as DTPMV;
pub use dtpsv_ as DTPSV;
pub use dtrmm_ as DTRMM;
pub use dtrmm_oop_ as DTRMM_OOP;
pub use dtrmv_ as DTRMV;
pub use dtrsm_ as DTRSM;
pub use dtrsm_batch_ as DTRSM_BATCH;
pub use dtrsm_batch_strided_ as DTRSM_BATCH_STRIDED;
pub use dtrsm_oop_ as DTRSM_OOP;
pub use dtrsv_ as DTRSV;
pub use dzasum_ as DZASUM;
pub use dzgemm_ as DZGEMM;
pub use dzgemv_ as DZGEMV;
pub use dznrm2_ as DZNRM2;
pub use gemm_bf16bf16f32_ as GEMM_BF16BF16F32;
pub use gemm_bf16bf16f32_compute_ as GEMM_BF16BF16F32_COMPUTE;
pub use gemm_bf16bf16f32_pack_ as GEMM_BF16BF16F32_PACK;
pub use gemm_bf16bf16f32_pack_get_size_ as GEMM_BF16BF16F32_PACK_GET_SIZE;
pub use gemm_e4m3e4m3f32_ as GEMM_E4M3E4M3F32;
pub use gemm_e4m3e4m3f32_compute_ as GEMM_E4M3E4M3F32_COMPUTE;
pub use gemm_e4m3e4m3f32_pack_ as GEMM_E4M3E4M3F32_PACK;
pub use gemm_e4m3e4m3f32_pack_get_size_ as GEMM_E4M3E4M3F32_PACK_GET_SIZE;
pub use gemm_e5m2e5m2f32_ as GEMM_E5M2E5M2F32;
pub use gemm_e5m2e5m2f32_compute_ as GEMM_E5M2E5M2F32_COMPUTE;
pub use gemm_e5m2e5m2f32_pack_ as GEMM_E5M2E5M2F32_PACK;
pub use gemm_e5m2e5m2f32_pack_get_size_ as GEMM_E5M2E5M2F32_PACK_GET_SIZE;
pub use gemm_f16f16f32_ as GEMM_F16F16F32;
pub use gemm_f16f16f32_compute_ as GEMM_F16F16F32_COMPUTE;
pub use gemm_f16f16f32_pack_ as GEMM_F16F16F32_PACK;
pub use gemm_f16f16f32_pack_get_size_ as GEMM_F16F16F32_PACK_GET_SIZE;
pub use gemm_s16s16s32_ as GEMM_S16S16S32;
pub use gemm_s16s16s32_compute_ as GEMM_S16S16S32_COMPUTE;
pub use gemm_s16s16s32_pack_ as GEMM_S16S16S32_PACK;
pub use gemm_s16s16s32_pack_get_size_ as GEMM_S16S16S32_PACK_GET_SIZE;
pub use gemm_s8u8s32_ as GEMM_S8U8S32;
pub use gemm_s8u8s32_compute_ as GEMM_S8U8S32_COMPUTE;
pub use gemm_s8u8s32_pack_ as GEMM_S8U8S32_PACK;
pub use gemm_s8u8s32_pack_get_size_ as GEMM_S8U8S32_PACK_GET_SIZE;
pub use hgemm_ as HGEMM;
pub use hgemm_compute_ as HGEMM_COMPUTE;
pub use hgemm_pack_ as HGEMM_PACK;
pub use hgemm_pack_get_size_ as HGEMM_PACK_GET_SIZE;
pub use icamax_ as ICAMAX;
pub use icamin_ as ICAMIN;
pub use idamax_ as IDAMAX;
pub use idamin_ as IDAMIN;
pub use isamax_ as ISAMAX;
pub use isamin_ as ISAMIN;
pub use izamax_ as IZAMAX;
pub use izamin_ as IZAMIN;
pub use lsame_ as LSAME;
pub use mkl_cblas_jit_create_cgemm_ as MKL_CBLAS_JIT_CREATE_CGEMM;
pub use mkl_cblas_jit_create_dgemm_ as MKL_CBLAS_JIT_CREATE_DGEMM;
pub use mkl_cblas_jit_create_sgemm_ as MKL_CBLAS_JIT_CREATE_SGEMM;
pub use mkl_cblas_jit_create_zgemm_ as MKL_CBLAS_JIT_CREATE_ZGEMM;
pub use mkl_jit_destroy_ as MKL_JIT_DESTROY;
pub use mkl_jit_get_cgemm_ptr_ as MKL_JIT_GET_CGEMM_PTR;
pub use mkl_jit_get_dgemm_ptr_ as MKL_JIT_GET_DGEMM_PTR;
pub use mkl_jit_get_sgemm_ptr_ as MKL_JIT_GET_SGEMM_PTR;
pub use mkl_jit_get_zgemm_ptr_ as MKL_JIT_GET_ZGEMM_PTR;
pub use sasum_ as SASUM;
pub use saxpby_ as SAXPBY;
pub use saxpy_ as SAXPY;
pub use saxpy_batch_ as SAXPY_BATCH;
pub use saxpy_batch_strided_ as SAXPY_BATCH_STRIDED;
pub use saxpyi_ as SAXPYI;
pub use scabs1_ as SCABS1;
pub use scasum_ as SCASUM;
pub use scgemm_ as SCGEMM;
pub use scgemv_ as SCGEMV;
pub use scnrm2_ as SCNRM2;
pub use scopy_ as SCOPY;
pub use scopy_batch_ as SCOPY_BATCH;
pub use scopy_batch_strided_ as SCOPY_BATCH_STRIDED;
pub use sdgmm_batch_ as SDGMM_BATCH;
pub use sdgmm_batch_strided_ as SDGMM_BATCH_STRIDED;
pub use sdot_ as SDOT;
pub use sdoti_ as SDOTI;
pub use sdsdot_ as SDSDOT;
pub use sgbmv_ as SGBMV;
pub use sgem2vu_ as SGEM2VU;
pub use sgemm_ as SGEMM;
pub use sgemm_batch_ as SGEMM_BATCH;
pub use sgemm_batch_strided_ as SGEMM_BATCH_STRIDED;
pub use sgemm_compute_ as SGEMM_COMPUTE;
pub use sgemm_pack_ as SGEMM_PACK;
pub use sgemm_pack_get_size_ as SGEMM_PACK_GET_SIZE;
pub use sgemmt_ as SGEMMT;
pub use sgemv_ as SGEMV;
pub use sgemv_batch_ as SGEMV_BATCH;
pub use sgemv_batch_strided_ as SGEMV_BATCH_STRIDED;
pub use sger_ as SGER;
pub use sgthr_ as SGTHR;
pub use sgthrz_ as SGTHRZ;
pub use snrm2_ as SNRM2;
pub use srot_ as SROT;
pub use srotg_ as SROTG;
pub use sroti_ as SROTI;
pub use srotm_ as SROTM;
pub use srotmg_ as SROTMG;
pub use ssbmv_ as SSBMV;
pub use sscal_ as SSCAL;
pub use ssctr_ as SSCTR;
pub use sspmv_ as SSPMV;
pub use sspr2_ as SSPR2;
pub use sspr_ as SSPR;
pub use sswap_ as SSWAP;
pub use ssymm_ as SSYMM;
pub use ssymv_ as SSYMV;
pub use ssyr2_ as SSYR2;
pub use ssyr2k_ as SSYR2K;
pub use ssyr_ as SSYR;
pub use ssyrk_ as SSYRK;
pub use ssyrk_batch_ as SSYRK_BATCH;
pub use ssyrk_batch_strided_ as SSYRK_BATCH_STRIDED;
pub use stbmv_ as STBMV;
pub use stbsv_ as STBSV;
pub use stpmv_ as STPMV;
pub use stpsv_ as STPSV;
pub use strmm_ as STRMM;
pub use strmm_oop_ as STRMM_OOP;
pub use strmv_ as STRMV;
pub use strsm_ as STRSM;
pub use strsm_batch_ as STRSM_BATCH;
pub use strsm_batch_strided_ as STRSM_BATCH_STRIDED;
pub use strsm_oop_ as STRSM_OOP;
pub use strsv_ as STRSV;
pub use xerbla_ as XERBLA;
pub use zaxpby_ as ZAXPBY;
pub use zaxpy_ as ZAXPY;
pub use zaxpy_batch_ as ZAXPY_BATCH;
pub use zaxpy_batch_strided_ as ZAXPY_BATCH_STRIDED;
pub use zaxpyi_ as ZAXPYI;
pub use zcopy_ as ZCOPY;
pub use zcopy_batch_ as ZCOPY_BATCH;
pub use zcopy_batch_strided_ as ZCOPY_BATCH_STRIDED;
pub use zdgmm_batch_ as ZDGMM_BATCH;
pub use zdgmm_batch_strided_ as ZDGMM_BATCH_STRIDED;
pub use zdotc_ as ZDOTC;
pub use zdotci_ as ZDOTCI;
pub use zdotu_ as ZDOTU;
pub use zdotui_ as ZDOTUI;
pub use zdrot_ as ZDROT;
pub use zdscal_ as ZDSCAL;
pub use zgbmv_ as ZGBMV;
pub use zgem2vc_ as ZGEM2VC;
pub use zgemm3m_ as ZGEMM3M;
pub use zgemm3m_batch_ as ZGEMM3M_BATCH;
pub use zgemm3m_batch_strided_ as ZGEMM3M_BATCH_STRIDED;
pub use zgemm_ as ZGEMM;
pub use zgemm_batch_ as ZGEMM_BATCH;
pub use zgemm_batch_strided_ as ZGEMM_BATCH_STRIDED;
pub use zgemmt_ as ZGEMMT;
pub use zgemv_ as ZGEMV;
pub use zgemv_batch_ as ZGEMV_BATCH;
pub use zgemv_batch_strided_ as ZGEMV_BATCH_STRIDED;
pub use zgerc_ as ZGERC;
pub use zgeru_ as ZGERU;
pub use zgthr_ as ZGTHR;
pub use zgthrz_ as ZGTHRZ;
pub use zhbmv_ as ZHBMV;
pub use zhemm_ as ZHEMM;
pub use zhemv_ as ZHEMV;
pub use zher2_ as ZHER2;
pub use zher2k_ as ZHER2K;
pub use zher_ as ZHER;
pub use zherk_ as ZHERK;
pub use zhpmv_ as ZHPMV;
pub use zhpr2_ as ZHPR2;
pub use zhpr_ as ZHPR;
pub use zrot_ as ZROT;
pub use zrotg_ as ZROTG;
pub use zscal_ as ZSCAL;
pub use zsctr_ as ZSCTR;
pub use zswap_ as ZSWAP;
pub use zsymm_ as ZSYMM;
pub use zsyr2k_ as ZSYR2K;
pub use zsyrk_ as ZSYRK;
pub use zsyrk_batch_ as ZSYRK_BATCH;
pub use zsyrk_batch_strided_ as ZSYRK_BATCH_STRIDED;
pub use ztbmv_ as ZTBMV;
pub use ztbsv_ as ZTBSV;
pub use ztpmv_ as ZTPMV;
pub use ztpsv_ as ZTPSV;
pub use ztrmm_ as ZTRMM;
pub use ztrmm_oop_ as ZTRMM_OOP;
pub use ztrmv_ as ZTRMV;
pub use ztrsm_ as ZTRSM;
pub use ztrsm_batch_ as ZTRSM_BATCH;
pub use ztrsm_batch_strided_ as ZTRSM_BATCH_STRIDED;
pub use ztrsm_oop_ as ZTRSM_OOP;
pub use ztrsv_ as ZTRSV;

/* #endregion */

/* #region lower case with underscore alias */

pub use caxpby_ as caxpby;
pub use caxpy_ as caxpy;
pub use caxpy_batch_ as caxpy_batch;
pub use caxpy_batch_strided_ as caxpy_batch_strided;
pub use caxpyi_ as caxpyi;
pub use ccopy_ as ccopy;
pub use ccopy_batch_ as ccopy_batch;
pub use ccopy_batch_strided_ as ccopy_batch_strided;
pub use cdgmm_batch_ as cdgmm_batch;
pub use cdgmm_batch_strided_ as cdgmm_batch_strided;
pub use cdotc_ as cdotc;
pub use cdotci_ as cdotci;
pub use cdotu_ as cdotu;
pub use cdotui_ as cdotui;
pub use cgbmv_ as cgbmv;
pub use cgem2vc_ as cgem2vc;
pub use cgemm3m_ as cgemm3m;
pub use cgemm3m_batch_ as cgemm3m_batch;
pub use cgemm3m_batch_strided_ as cgemm3m_batch_strided;
pub use cgemm_ as cgemm;
pub use cgemm_batch_ as cgemm_batch;
pub use cgemm_batch_strided_ as cgemm_batch_strided;
pub use cgemmt_ as cgemmt;
pub use cgemv_ as cgemv;
pub use cgemv_batch_ as cgemv_batch;
pub use cgemv_batch_strided_ as cgemv_batch_strided;
pub use cgerc_ as cgerc;
pub use cgeru_ as cgeru;
pub use cgthr_ as cgthr;
pub use cgthrz_ as cgthrz;
pub use chbmv_ as chbmv;
pub use chemm_ as chemm;
pub use chemv_ as chemv;
pub use cher2_ as cher2;
pub use cher2k_ as cher2k;
pub use cher_ as cher;
pub use cherk_ as cherk;
pub use chpmv_ as chpmv;
pub use chpr2_ as chpr2;
pub use chpr_ as chpr;
pub use crot_ as crot;
pub use crotg_ as crotg;
pub use cscal_ as cscal;
pub use csctr_ as csctr;
pub use csrot_ as csrot;
pub use csscal_ as csscal;
pub use cswap_ as cswap;
pub use csymm_ as csymm;
pub use csyr2k_ as csyr2k;
pub use csyrk_ as csyrk;
pub use csyrk_batch_ as csyrk_batch;
pub use csyrk_batch_strided_ as csyrk_batch_strided;
pub use ctbmv_ as ctbmv;
pub use ctbsv_ as ctbsv;
pub use ctpmv_ as ctpmv;
pub use ctpsv_ as ctpsv;
pub use ctrmm_ as ctrmm;
pub use ctrmm_oop_ as ctrmm_oop;
pub use ctrmv_ as ctrmv;
pub use ctrsm_ as ctrsm;
pub use ctrsm_batch_ as ctrsm_batch;
pub use ctrsm_batch_strided_ as ctrsm_batch_strided;
pub use ctrsm_oop_ as ctrsm_oop;
pub use ctrsv_ as ctrsv;
pub use dasum_ as dasum;
pub use daxpby_ as daxpby;
pub use daxpy_ as daxpy;
pub use daxpy_batch_ as daxpy_batch;
pub use daxpy_batch_strided_ as daxpy_batch_strided;
pub use daxpyi_ as daxpyi;
pub use dcabs1_ as dcabs1;
pub use dcopy_ as dcopy;
pub use dcopy_batch_ as dcopy_batch;
pub use dcopy_batch_strided_ as dcopy_batch_strided;
pub use ddgmm_batch_ as ddgmm_batch;
pub use ddgmm_batch_strided_ as ddgmm_batch_strided;
pub use ddot_ as ddot;
pub use ddoti_ as ddoti;
pub use dgbmv_ as dgbmv;
pub use dgem2vu_ as dgem2vu;
pub use dgemm_ as dgemm;
pub use dgemm_batch_ as dgemm_batch;
pub use dgemm_batch_strided_ as dgemm_batch_strided;
pub use dgemm_compute_ as dgemm_compute;
pub use dgemm_pack_ as dgemm_pack;
pub use dgemm_pack_get_size_ as dgemm_pack_get_size;
pub use dgemmt_ as dgemmt;
pub use dgemv_ as dgemv;
pub use dgemv_batch_ as dgemv_batch;
pub use dgemv_batch_strided_ as dgemv_batch_strided;
pub use dger_ as dger;
pub use dgthr_ as dgthr;
pub use dgthrz_ as dgthrz;
pub use dnrm2_ as dnrm2;
pub use drot_ as drot;
pub use drotg_ as drotg;
pub use droti_ as droti;
pub use drotm_ as drotm;
pub use drotmg_ as drotmg;
pub use dsbmv_ as dsbmv;
pub use dscal_ as dscal;
pub use dsctr_ as dsctr;
pub use dsdot_ as dsdot;
pub use dspmv_ as dspmv;
pub use dspr2_ as dspr2;
pub use dspr_ as dspr;
pub use dswap_ as dswap;
pub use dsymm_ as dsymm;
pub use dsymv_ as dsymv;
pub use dsyr2_ as dsyr2;
pub use dsyr2k_ as dsyr2k;
pub use dsyr_ as dsyr;
pub use dsyrk_ as dsyrk;
pub use dsyrk_batch_ as dsyrk_batch;
pub use dsyrk_batch_strided_ as dsyrk_batch_strided;
pub use dtbmv_ as dtbmv;
pub use dtbsv_ as dtbsv;
pub use dtpmv_ as dtpmv;
pub use dtpsv_ as dtpsv;
pub use dtrmm_ as dtrmm;
pub use dtrmm_oop_ as dtrmm_oop;
pub use dtrmv_ as dtrmv;
pub use dtrsm_ as dtrsm;
pub use dtrsm_batch_ as dtrsm_batch;
pub use dtrsm_batch_strided_ as dtrsm_batch_strided;
pub use dtrsm_oop_ as dtrsm_oop;
pub use dtrsv_ as dtrsv;
pub use dzasum_ as dzasum;
pub use dzgemm_ as dzgemm;
pub use dzgemv_ as dzgemv;
pub use dznrm2_ as dznrm2;
pub use gemm_bf16bf16f32_ as gemm_bf16bf16f32;
pub use gemm_bf16bf16f32_compute_ as gemm_bf16bf16f32_compute;
pub use gemm_bf16bf16f32_pack_ as gemm_bf16bf16f32_pack;
pub use gemm_bf16bf16f32_pack_get_size_ as gemm_bf16bf16f32_pack_get_size;
pub use gemm_e4m3e4m3f32_ as gemm_e4m3e4m3f32;
pub use gemm_e4m3e4m3f32_compute_ as gemm_e4m3e4m3f32_compute;
pub use gemm_e4m3e4m3f32_pack_ as gemm_e4m3e4m3f32_pack;
pub use gemm_e4m3e4m3f32_pack_get_size_ as gemm_e4m3e4m3f32_pack_get_size;
pub use gemm_e5m2e5m2f32_ as gemm_e5m2e5m2f32;
pub use gemm_e5m2e5m2f32_compute_ as gemm_e5m2e5m2f32_compute;
pub use gemm_e5m2e5m2f32_pack_ as gemm_e5m2e5m2f32_pack;
pub use gemm_e5m2e5m2f32_pack_get_size_ as gemm_e5m2e5m2f32_pack_get_size;
pub use gemm_f16f16f32_ as gemm_f16f16f32;
pub use gemm_f16f16f32_compute_ as gemm_f16f16f32_compute;
pub use gemm_f16f16f32_pack_ as gemm_f16f16f32_pack;
pub use gemm_f16f16f32_pack_get_size_ as gemm_f16f16f32_pack_get_size;
pub use gemm_s16s16s32_ as gemm_s16s16s32;
pub use gemm_s16s16s32_compute_ as gemm_s16s16s32_compute;
pub use gemm_s16s16s32_pack_ as gemm_s16s16s32_pack;
pub use gemm_s16s16s32_pack_get_size_ as gemm_s16s16s32_pack_get_size;
pub use gemm_s8u8s32_ as gemm_s8u8s32;
pub use gemm_s8u8s32_compute_ as gemm_s8u8s32_compute;
pub use gemm_s8u8s32_pack_ as gemm_s8u8s32_pack;
pub use gemm_s8u8s32_pack_get_size_ as gemm_s8u8s32_pack_get_size;
pub use hgemm_ as hgemm;
pub use hgemm_compute_ as hgemm_compute;
pub use hgemm_pack_ as hgemm_pack;
pub use hgemm_pack_get_size_ as hgemm_pack_get_size;
pub use icamax_ as icamax;
pub use icamin_ as icamin;
pub use idamax_ as idamax;
pub use idamin_ as idamin;
pub use isamax_ as isamax;
pub use isamin_ as isamin;
pub use izamax_ as izamax;
pub use izamin_ as izamin;
pub use lsame_ as lsame;
pub use mkl_cblas_jit_create_cgemm_ as mkl_cblas_jit_create_cgemm;
pub use mkl_cblas_jit_create_dgemm_ as mkl_cblas_jit_create_dgemm;
pub use mkl_cblas_jit_create_sgemm_ as mkl_cblas_jit_create_sgemm;
pub use mkl_cblas_jit_create_zgemm_ as mkl_cblas_jit_create_zgemm;
pub use mkl_jit_destroy_ as mkl_jit_destroy;
pub use mkl_jit_get_cgemm_ptr_ as mkl_jit_get_cgemm_ptr;
pub use mkl_jit_get_dgemm_ptr_ as mkl_jit_get_dgemm_ptr;
pub use mkl_jit_get_sgemm_ptr_ as mkl_jit_get_sgemm_ptr;
pub use mkl_jit_get_zgemm_ptr_ as mkl_jit_get_zgemm_ptr;
pub use sasum_ as sasum;
pub use saxpby_ as saxpby;
pub use saxpy_ as saxpy;
pub use saxpy_batch_ as saxpy_batch;
pub use saxpy_batch_strided_ as saxpy_batch_strided;
pub use saxpyi_ as saxpyi;
pub use scabs1_ as scabs1;
pub use scasum_ as scasum;
pub use scgemm_ as scgemm;
pub use scgemv_ as scgemv;
pub use scnrm2_ as scnrm2;
pub use scopy_ as scopy;
pub use scopy_batch_ as scopy_batch;
pub use scopy_batch_strided_ as scopy_batch_strided;
pub use sdgmm_batch_ as sdgmm_batch;
pub use sdgmm_batch_strided_ as sdgmm_batch_strided;
pub use sdot_ as sdot;
pub use sdoti_ as sdoti;
pub use sdsdot_ as sdsdot;
pub use sgbmv_ as sgbmv;
pub use sgem2vu_ as sgem2vu;
pub use sgemm_ as sgemm;
pub use sgemm_batch_ as sgemm_batch;
pub use sgemm_batch_strided_ as sgemm_batch_strided;
pub use sgemm_compute_ as sgemm_compute;
pub use sgemm_pack_ as sgemm_pack;
pub use sgemm_pack_get_size_ as sgemm_pack_get_size;
pub use sgemmt_ as sgemmt;
pub use sgemv_ as sgemv;
pub use sgemv_batch_ as sgemv_batch;
pub use sgemv_batch_strided_ as sgemv_batch_strided;
pub use sger_ as sger;
pub use sgthr_ as sgthr;
pub use sgthrz_ as sgthrz;
pub use snrm2_ as snrm2;
pub use srot_ as srot;
pub use srotg_ as srotg;
pub use sroti_ as sroti;
pub use srotm_ as srotm;
pub use srotmg_ as srotmg;
pub use ssbmv_ as ssbmv;
pub use sscal_ as sscal;
pub use ssctr_ as ssctr;
pub use sspmv_ as sspmv;
pub use sspr2_ as sspr2;
pub use sspr_ as sspr;
pub use sswap_ as sswap;
pub use ssymm_ as ssymm;
pub use ssymv_ as ssymv;
pub use ssyr2_ as ssyr2;
pub use ssyr2k_ as ssyr2k;
pub use ssyr_ as ssyr;
pub use ssyrk_ as ssyrk;
pub use ssyrk_batch_ as ssyrk_batch;
pub use ssyrk_batch_strided_ as ssyrk_batch_strided;
pub use stbmv_ as stbmv;
pub use stbsv_ as stbsv;
pub use stpmv_ as stpmv;
pub use stpsv_ as stpsv;
pub use strmm_ as strmm;
pub use strmm_oop_ as strmm_oop;
pub use strmv_ as strmv;
pub use strsm_ as strsm;
pub use strsm_batch_ as strsm_batch;
pub use strsm_batch_strided_ as strsm_batch_strided;
pub use strsm_oop_ as strsm_oop;
pub use strsv_ as strsv;
pub use xerbla_ as xerbla;
pub use zaxpby_ as zaxpby;
pub use zaxpy_ as zaxpy;
pub use zaxpy_batch_ as zaxpy_batch;
pub use zaxpy_batch_strided_ as zaxpy_batch_strided;
pub use zaxpyi_ as zaxpyi;
pub use zcopy_ as zcopy;
pub use zcopy_batch_ as zcopy_batch;
pub use zcopy_batch_strided_ as zcopy_batch_strided;
pub use zdgmm_batch_ as zdgmm_batch;
pub use zdgmm_batch_strided_ as zdgmm_batch_strided;
pub use zdotc_ as zdotc;
pub use zdotci_ as zdotci;
pub use zdotu_ as zdotu;
pub use zdotui_ as zdotui;
pub use zdrot_ as zdrot;
pub use zdscal_ as zdscal;
pub use zgbmv_ as zgbmv;
pub use zgem2vc_ as zgem2vc;
pub use zgemm3m_ as zgemm3m;
pub use zgemm3m_batch_ as zgemm3m_batch;
pub use zgemm3m_batch_strided_ as zgemm3m_batch_strided;
pub use zgemm_ as zgemm;
pub use zgemm_batch_ as zgemm_batch;
pub use zgemm_batch_strided_ as zgemm_batch_strided;
pub use zgemmt_ as zgemmt;
pub use zgemv_ as zgemv;
pub use zgemv_batch_ as zgemv_batch;
pub use zgemv_batch_strided_ as zgemv_batch_strided;
pub use zgerc_ as zgerc;
pub use zgeru_ as zgeru;
pub use zgthr_ as zgthr;
pub use zgthrz_ as zgthrz;
pub use zhbmv_ as zhbmv;
pub use zhemm_ as zhemm;
pub use zhemv_ as zhemv;
pub use zher2_ as zher2;
pub use zher2k_ as zher2k;
pub use zher_ as zher;
pub use zherk_ as zherk;
pub use zhpmv_ as zhpmv;
pub use zhpr2_ as zhpr2;
pub use zhpr_ as zhpr;
pub use zrot_ as zrot;
pub use zrotg_ as zrotg;
pub use zscal_ as zscal;
pub use zsctr_ as zsctr;
pub use zswap_ as zswap;
pub use zsymm_ as zsymm;
pub use zsyr2k_ as zsyr2k;
pub use zsyrk_ as zsyrk;
pub use zsyrk_batch_ as zsyrk_batch;
pub use zsyrk_batch_strided_ as zsyrk_batch_strided;
pub use ztbmv_ as ztbmv;
pub use ztbsv_ as ztbsv;
pub use ztpmv_ as ztpmv;
pub use ztpsv_ as ztpsv;
pub use ztrmm_ as ztrmm;
pub use ztrmm_oop_ as ztrmm_oop;
pub use ztrmv_ as ztrmv;
pub use ztrsm_ as ztrsm;
pub use ztrsm_batch_ as ztrsm_batch;
pub use ztrsm_batch_strided_ as ztrsm_batch_strided;
pub use ztrsm_oop_ as ztrsm_oop;
pub use ztrsv_ as ztrsv;

/* #endregion */
