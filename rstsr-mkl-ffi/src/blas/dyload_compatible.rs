//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn xerbla(srname: *const c_char, info: *const c_int, lsrname: c_int) {
    dyload_lib().xerbla.unwrap()(srname, info, lsrname)
}

pub unsafe fn lsame(ca: *const c_char, cb: *const c_char, lca: MKL_INT, lcb: MKL_INT) -> c_int {
    dyload_lib().lsame.unwrap()(ca, cb, lca, lcb)
}

pub unsafe fn scabs1(c: *const MKL_Complex8) -> f32 {
    dyload_lib().scabs1.unwrap()(c)
}

pub unsafe fn sasum(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32 {
    dyload_lib().sasum.unwrap()(n, x, incx)
}

pub unsafe fn saxpy(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().saxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn saxpby(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    beta: *const f32,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().saxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn saxpyi(
    nz: *const MKL_INT,
    a: *const f32,
    x: *const f32,
    indx: *const MKL_INT,
    y: *mut f32,
) {
    dyload_lib().saxpyi.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn scasum(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32 {
    dyload_lib().scasum.unwrap()(n, x, incx)
}

pub unsafe fn scnrm2(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32 {
    dyload_lib().scnrm2.unwrap()(n, x, incx)
}

pub unsafe fn scopy(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().scopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn sdot(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f32 {
    dyload_lib().sdot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn sdoti(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *const f32) -> f32 {
    dyload_lib().sdoti.unwrap()(nz, x, indx, y)
}

pub unsafe fn sdsdot(
    n: *const MKL_INT,
    sb: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f32 {
    dyload_lib().sdsdot.unwrap()(n, sb, x, incx, y, incy)
}

pub unsafe fn sgthr(nz: *const MKL_INT, y: *const f32, x: *mut f32, indx: *const MKL_INT) {
    dyload_lib().sgthr.unwrap()(nz, y, x, indx)
}

pub unsafe fn sgthrz(nz: *const MKL_INT, y: *mut f32, x: *mut f32, indx: *const MKL_INT) {
    dyload_lib().sgthrz.unwrap()(nz, y, x, indx)
}

pub unsafe fn snrm2(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32 {
    dyload_lib().snrm2.unwrap()(n, x, incx)
}

pub unsafe fn srot(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().srot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32) {
    dyload_lib().srotg.unwrap()(a, b, c, s)
}

pub unsafe fn sroti(
    nz: *const MKL_INT,
    x: *mut f32,
    indx: *const MKL_INT,
    y: *mut f32,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().sroti.unwrap()(nz, x, indx, y, c, s)
}

pub unsafe fn srotm(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    param: *const f32,
) {
    dyload_lib().srotm.unwrap()(n, x, incx, y, incy, param)
}

pub unsafe fn srotmg(d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32) {
    dyload_lib().srotmg.unwrap()(d1, d2, x1, y1, param)
}

pub unsafe fn sscal(n: *const MKL_INT, a: *const f32, x: *mut f32, incx: *const MKL_INT) {
    dyload_lib().sscal.unwrap()(n, a, x, incx)
}

pub unsafe fn ssctr(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *mut f32) {
    dyload_lib().ssctr.unwrap()(nz, x, indx, y)
}

pub unsafe fn sswap(
    n: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
) {
    dyload_lib().sswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn isamax(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().isamax.unwrap()(n, x, incx)
}

pub unsafe fn isamin(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().isamin.unwrap()(n, x, incx)
}

pub unsafe fn caxpy(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().caxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn caxpby(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    beta: *const MKL_Complex8,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().caxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn caxpyi(
    nz: *const MKL_INT,
    a: *const MKL_Complex8,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *mut MKL_Complex8,
) {
    dyload_lib().caxpyi.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn ccopy(
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().ccopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn cdotc(
    pres: *mut MKL_Complex8,
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cdotc.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn cdotci(
    pres: *mut MKL_Complex8,
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *const MKL_Complex8,
) {
    dyload_lib().cdotci.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn cdotu(
    pres: *mut MKL_Complex8,
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cdotu.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn cdotui(
    pres: *mut MKL_Complex8,
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *const MKL_Complex8,
) {
    dyload_lib().cdotui.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn cgthr(
    nz: *const MKL_INT,
    y: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    indx: *const MKL_INT,
) {
    dyload_lib().cgthr.unwrap()(nz, y, x, indx)
}

pub unsafe fn cgthrz(
    nz: *const MKL_INT,
    y: *mut MKL_Complex8,
    x: *mut MKL_Complex8,
    indx: *const MKL_INT,
) {
    dyload_lib().cgthrz.unwrap()(nz, y, x, indx)
}

pub unsafe fn crot(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const MKL_Complex8,
) {
    dyload_lib().crot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn crotg(
    a: *mut MKL_Complex8,
    b: *const MKL_Complex8,
    c: *mut f32,
    s: *mut MKL_Complex8,
) {
    dyload_lib().crotg.unwrap()(a, b, c, s)
}

pub unsafe fn cscal(
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().cscal.unwrap()(n, a, x, incx)
}

pub unsafe fn csctr(
    nz: *const MKL_INT,
    x: *const MKL_Complex8,
    indx: *const MKL_INT,
    y: *mut MKL_Complex8,
) {
    dyload_lib().csctr.unwrap()(nz, x, indx, y)
}

pub unsafe fn csrot(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    c: *const f32,
    s: *const f32,
) {
    dyload_lib().csrot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn csscal(n: *const MKL_INT, a: *const f32, x: *mut MKL_Complex8, incx: *const MKL_INT) {
    dyload_lib().csscal.unwrap()(n, a, x, incx)
}

pub unsafe fn cswap(
    n: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
) {
    dyload_lib().cswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn icamax(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().icamax.unwrap()(n, x, incx)
}

pub unsafe fn icamin(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().icamin.unwrap()(n, x, incx)
}

pub unsafe fn dcabs1(z: *const MKL_Complex16) -> f64 {
    dyload_lib().dcabs1.unwrap()(z)
}

pub unsafe fn dasum(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64 {
    dyload_lib().dasum.unwrap()(n, x, incx)
}

pub unsafe fn daxpy(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().daxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn daxpby(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    beta: *const f64,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().daxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn daxpyi(
    nz: *const MKL_INT,
    a: *const f64,
    x: *const f64,
    indx: *const MKL_INT,
    y: *mut f64,
) {
    dyload_lib().daxpyi.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn dcopy(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dcopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn ddot(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
) -> f64 {
    dyload_lib().ddot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn dsdot(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
) -> f64 {
    dyload_lib().dsdot.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn ddoti(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *const f64) -> f64 {
    dyload_lib().ddoti.unwrap()(nz, x, indx, y)
}

pub unsafe fn dgthr(nz: *const MKL_INT, y: *const f64, x: *mut f64, indx: *const MKL_INT) {
    dyload_lib().dgthr.unwrap()(nz, y, x, indx)
}

pub unsafe fn dgthrz(nz: *const MKL_INT, y: *mut f64, x: *mut f64, indx: *const MKL_INT) {
    dyload_lib().dgthrz.unwrap()(nz, y, x, indx)
}

pub unsafe fn dnrm2(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64 {
    dyload_lib().dnrm2.unwrap()(n, x, incx)
}

pub unsafe fn drot(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().drot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64) {
    dyload_lib().drotg.unwrap()(a, b, c, s)
}

pub unsafe fn droti(
    nz: *const MKL_INT,
    x: *mut f64,
    indx: *const MKL_INT,
    y: *mut f64,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().droti.unwrap()(nz, x, indx, y, c, s)
}

pub unsafe fn drotm(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    param: *const f64,
) {
    dyload_lib().drotm.unwrap()(n, x, incx, y, incy, param)
}

pub unsafe fn drotmg(d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64) {
    dyload_lib().drotmg.unwrap()(d1, d2, x1, y1, param)
}

pub unsafe fn dscal(n: *const MKL_INT, a: *const f64, x: *mut f64, incx: *const MKL_INT) {
    dyload_lib().dscal.unwrap()(n, a, x, incx)
}

pub unsafe fn dsctr(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *mut f64) {
    dyload_lib().dsctr.unwrap()(nz, x, indx, y)
}

pub unsafe fn dswap(
    n: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
) {
    dyload_lib().dswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn dzasum(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64 {
    dyload_lib().dzasum.unwrap()(n, x, incx)
}

pub unsafe fn dznrm2(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64 {
    dyload_lib().dznrm2.unwrap()(n, x, incx)
}

pub unsafe fn idamax(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().idamax.unwrap()(n, x, incx)
}

pub unsafe fn idamin(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().idamin.unwrap()(n, x, incx)
}

pub unsafe fn zaxpy(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zaxpy.unwrap()(n, alpha, x, incx, y, incy)
}

pub unsafe fn zaxpby(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    beta: *const MKL_Complex16,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zaxpby.unwrap()(n, alpha, x, incx, beta, y, incy)
}

pub unsafe fn zaxpyi(
    nz: *const MKL_INT,
    a: *const MKL_Complex16,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *mut MKL_Complex16,
) {
    dyload_lib().zaxpyi.unwrap()(nz, a, x, indx, y)
}

pub unsafe fn zcopy(
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zcopy.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn zdotc(
    pres: *mut MKL_Complex16,
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zdotc.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn zdotci(
    pres: *mut MKL_Complex16,
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *const MKL_Complex16,
) {
    dyload_lib().zdotci.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn zdotu(
    pres: *mut MKL_Complex16,
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zdotu.unwrap()(pres, n, x, incx, y, incy)
}

pub unsafe fn zdotui(
    pres: *mut MKL_Complex16,
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *const MKL_Complex16,
) {
    dyload_lib().zdotui.unwrap()(pres, nz, x, indx, y)
}

pub unsafe fn zdrot(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const f64,
) {
    dyload_lib().zdrot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn zdscal(
    n: *const MKL_INT,
    a: *const f64,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().zdscal.unwrap()(n, a, x, incx)
}

pub unsafe fn zgthr(
    nz: *const MKL_INT,
    y: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    indx: *const MKL_INT,
) {
    dyload_lib().zgthr.unwrap()(nz, y, x, indx)
}

pub unsafe fn zgthrz(
    nz: *const MKL_INT,
    y: *mut MKL_Complex16,
    x: *mut MKL_Complex16,
    indx: *const MKL_INT,
) {
    dyload_lib().zgthrz.unwrap()(nz, y, x, indx)
}

pub unsafe fn zrot(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    c: *const f64,
    s: *const MKL_Complex16,
) {
    dyload_lib().zrot.unwrap()(n, x, incx, y, incy, c, s)
}

pub unsafe fn zrotg(
    a: *mut MKL_Complex16,
    b: *const MKL_Complex16,
    c: *mut f64,
    s: *mut MKL_Complex16,
) {
    dyload_lib().zrotg.unwrap()(a, b, c, s)
}

pub unsafe fn zscal(
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().zscal.unwrap()(n, a, x, incx)
}

pub unsafe fn zsctr(
    nz: *const MKL_INT,
    x: *const MKL_Complex16,
    indx: *const MKL_INT,
    y: *mut MKL_Complex16,
) {
    dyload_lib().zsctr.unwrap()(nz, x, indx, y)
}

pub unsafe fn zswap(
    n: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
) {
    dyload_lib().zswap.unwrap()(n, x, incx, y, incy)
}

pub unsafe fn izamax(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().izamax.unwrap()(n, x, incx)
}

pub unsafe fn izamin(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT {
    dyload_lib().izamin.unwrap()(n, x, incx)
}

pub unsafe fn sgbmv(
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
    dyload_lib().sgbmv.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sgemv(
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
    dyload_lib().sgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sger(
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
    dyload_lib().sger.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn ssbmv(
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
    dyload_lib().ssbmv.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sspmv(
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
    dyload_lib().sspmv.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn sspr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    ap: *mut f32,
) {
    dyload_lib().sspr.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn sspr2(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    y: *const f32,
    incy: *const MKL_INT,
    ap: *mut f32,
) {
    dyload_lib().sspr2.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ssymv(
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
    dyload_lib().ssymv.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn ssyr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const f32,
    incx: *const MKL_INT,
    a: *mut f32,
    lda: *const MKL_INT,
) {
    dyload_lib().ssyr.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn ssyr2(
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
    dyload_lib().ssyr2.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn stbmv(
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
    dyload_lib().stbmv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn stbsv(
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
    dyload_lib().stbsv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn stpmv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f32,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stpmv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn stpsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f32,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().stpsv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn strmv(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    b: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().strmv.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn strsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f32,
    lda: *const MKL_INT,
    x: *mut f32,
    incx: *const MKL_INT,
) {
    dyload_lib().strsv.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn sgem2vu(
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
    dyload_lib().sgem2vu.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn cgbmv(
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
    dyload_lib().cgbmv.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cgemv(
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
    dyload_lib().cgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cgerc(
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
    dyload_lib().cgerc.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn cgeru(
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
    dyload_lib().cgeru.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn chbmv(
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
    dyload_lib().chbmv.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn chemv(
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
    dyload_lib().chemv.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn cher(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    a: *mut MKL_Complex8,
    lda: *const MKL_INT,
) {
    dyload_lib().cher.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn cher2(
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
    dyload_lib().cher2.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn chpmv(
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
    dyload_lib().chpmv.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn chpr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f32,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    ap: *mut MKL_Complex8,
) {
    dyload_lib().chpr.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn chpr2(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *const MKL_Complex8,
    incy: *const MKL_INT,
    ap: *mut MKL_Complex8,
) {
    dyload_lib().chpr2.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ctbmv(
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
    dyload_lib().ctbmv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ctbsv(
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
    dyload_lib().ctbsv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ctpmv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctpmv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ctpsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex8,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctpsv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ctrmv(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    b: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctrmv.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn ctrsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex8,
    lda: *const MKL_INT,
    x: *mut MKL_Complex8,
    incx: *const MKL_INT,
) {
    dyload_lib().ctrsv.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn cgem2vc(
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
    dyload_lib().cgem2vc.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn scgemv(
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
    dyload_lib().scgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dgbmv(
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
    dyload_lib().dgbmv.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dgemv(
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
    dyload_lib().dgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dger(
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
    dyload_lib().dger.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn dsbmv(
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
    dyload_lib().dsbmv.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dspmv(
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
    dyload_lib().dspmv.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn dspr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    ap: *mut f64,
) {
    dyload_lib().dspr.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn dspr2(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    y: *const f64,
    incy: *const MKL_INT,
    ap: *mut f64,
) {
    dyload_lib().dspr2.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn dsymv(
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
    dyload_lib().dsymv.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn dsyr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const f64,
    incx: *const MKL_INT,
    a: *mut f64,
    lda: *const MKL_INT,
) {
    dyload_lib().dsyr.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn dsyr2(
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
    dyload_lib().dsyr2.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn dtbmv(
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
    dyload_lib().dtbmv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn dtbsv(
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
    dyload_lib().dtbsv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn dtpmv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f64,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtpmv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn dtpsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const f64,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtpsv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn dtrmv(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    b: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtrmv.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn dtrsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const f64,
    lda: *const MKL_INT,
    x: *mut f64,
    incx: *const MKL_INT,
) {
    dyload_lib().dtrsv.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn dgem2vu(
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
    dyload_lib().dgem2vu.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn zgbmv(
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
    dyload_lib().zgbmv.unwrap()(trans, m, n, kl, ku, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zgemv(
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
    dyload_lib().zgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zgerc(
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
    dyload_lib().zgerc.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zgeru(
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
    dyload_lib().zgeru.unwrap()(m, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zhbmv(
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
    dyload_lib().zhbmv.unwrap()(uplo, n, k, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zhemv(
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
    dyload_lib().zhemv.unwrap()(uplo, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn zher(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    a: *mut MKL_Complex16,
    lda: *const MKL_INT,
) {
    dyload_lib().zher.unwrap()(uplo, n, alpha, x, incx, a, lda)
}

pub unsafe fn zher2(
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
    dyload_lib().zher2.unwrap()(uplo, n, alpha, x, incx, y, incy, a, lda)
}

pub unsafe fn zhpmv(
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
    dyload_lib().zhpmv.unwrap()(uplo, n, alpha, ap, x, incx, beta, y, incy)
}

pub unsafe fn zhpr(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const f64,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    ap: *mut MKL_Complex16,
) {
    dyload_lib().zhpr.unwrap()(uplo, n, alpha, x, incx, ap)
}

pub unsafe fn zhpr2(
    uplo: *const c_char,
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *const MKL_Complex16,
    incy: *const MKL_INT,
    ap: *mut MKL_Complex16,
) {
    dyload_lib().zhpr2.unwrap()(uplo, n, alpha, x, incx, y, incy, ap)
}

pub unsafe fn ztbmv(
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
    dyload_lib().ztbmv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ztbsv(
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
    dyload_lib().ztbsv.unwrap()(uplo, trans, diag, n, k, a, lda, x, incx)
}

pub unsafe fn ztpmv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztpmv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ztpsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    ap: *const MKL_Complex16,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztpsv.unwrap()(uplo, trans, diag, n, ap, x, incx)
}

pub unsafe fn ztrmv(
    uplo: *const c_char,
    transa: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    b: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztrmv.unwrap()(uplo, transa, diag, n, a, lda, b, incx)
}

pub unsafe fn ztrsv(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const MKL_INT,
    a: *const MKL_Complex16,
    lda: *const MKL_INT,
    x: *mut MKL_Complex16,
    incx: *const MKL_INT,
) {
    dyload_lib().ztrsv.unwrap()(uplo, trans, diag, n, a, lda, x, incx)
}

pub unsafe fn zgem2vc(
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
    dyload_lib().zgem2vc.unwrap()(
        m, n, alpha, a, lda, x1, incx1, x2, incx2, beta, y1, incy1, y2, incy2,
    )
}

pub unsafe fn dzgemv(
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
    dyload_lib().dzgemv.unwrap()(trans, m, n, alpha, a, lda, x, incx, beta, y, incy)
}

pub unsafe fn sgemm(
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
    dyload_lib().sgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn sgemm_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().sgemm_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn sgemm_pack(
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
    dyload_lib().sgemm_pack.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn sgemm_compute(
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
    dyload_lib().sgemm_compute.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn sgemm_batch(
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
    dyload_lib().sgemm_batch.unwrap()(
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

pub unsafe fn sgemm_batch_strided(
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
    dyload_lib().sgemm_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn sgemmt(
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
    dyload_lib().sgemmt.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssymm(
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
    dyload_lib().ssymm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssyr2k(
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
    dyload_lib().ssyr2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn ssyrk(
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
    dyload_lib().ssyrk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn ssyrk_batch(
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
    dyload_lib().ssyrk_batch.unwrap()(
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

pub unsafe fn ssyrk_batch_strided(
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
    dyload_lib().ssyrk_batch_strided.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn strmm(
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
    dyload_lib().strmm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn strmm_oop(
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
    dyload_lib().strmm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn strsm(
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
    dyload_lib().strsm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn strsm_oop(
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
    dyload_lib().strsm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn strsm_batch(
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
    dyload_lib().strsm_batch.unwrap()(
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

pub unsafe fn strsm_batch_strided(
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
    dyload_lib().strsm_batch_strided.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn cgemm(
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
    dyload_lib().cgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm_batch(
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
    dyload_lib().cgemm_batch.unwrap()(
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

pub unsafe fn cgemm_batch_strided(
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
    dyload_lib().cgemm_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn scgemm(
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
    dyload_lib().scgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm3m(
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
    dyload_lib().cgemm3m.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cgemm3m_batch(
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
    dyload_lib().cgemm3m_batch.unwrap()(
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

pub unsafe fn cgemm3m_batch_strided(
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
    dyload_lib().cgemm3m_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn cgemmt(
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
    dyload_lib().cgemmt.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn chemm(
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
    dyload_lib().chemm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cher2k(
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
    dyload_lib().cher2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn cherk(
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
    dyload_lib().cherk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn csymm(
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
    dyload_lib().csymm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn csyr2k(
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
    dyload_lib().csyr2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn csyrk(
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
    dyload_lib().csyrk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn csyrk_batch(
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
    dyload_lib().csyrk_batch.unwrap()(
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

pub unsafe fn csyrk_batch_strided(
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
    dyload_lib().csyrk_batch_strided.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ctrmm(
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
    dyload_lib().ctrmm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ctrmm_oop(
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
    dyload_lib().ctrmm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ctrsm(
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
    dyload_lib().ctrsm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ctrsm_oop(
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
    dyload_lib().ctrsm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ctrsm_batch(
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
    dyload_lib().ctrsm_batch.unwrap()(
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

pub unsafe fn ctrsm_batch_strided(
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
    dyload_lib().ctrsm_batch_strided.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn dgemm(
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
    dyload_lib().dgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dgemm_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().dgemm_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn dgemm_pack(
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
    dyload_lib().dgemm_pack.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn dgemm_compute(
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
    dyload_lib().dgemm_compute.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dgemm_batch(
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
    dyload_lib().dgemm_batch.unwrap()(
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

pub unsafe fn dgemm_batch_strided(
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
    dyload_lib().dgemm_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn dgemmt(
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
    dyload_lib().dgemmt.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsymm(
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
    dyload_lib().dsymm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsyr2k(
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
    dyload_lib().dsyr2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn dsyrk(
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
    dyload_lib().dsyrk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn dsyrk_batch(
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
    dyload_lib().dsyrk_batch.unwrap()(
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

pub unsafe fn dsyrk_batch_strided(
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
    dyload_lib().dsyrk_batch_strided.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn dtrmm(
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
    dyload_lib().dtrmm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn dtrmm_oop(
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
    dyload_lib().dtrmm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn dtrsm(
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
    dyload_lib().dtrsm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn dtrsm_oop(
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
    dyload_lib().dtrsm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn dtrsm_batch(
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
    dyload_lib().dtrsm_batch.unwrap()(
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

pub unsafe fn dtrsm_batch_strided(
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
    dyload_lib().dtrsm_batch_strided.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn zgemm(
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
    dyload_lib().zgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm_batch(
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
    dyload_lib().zgemm_batch.unwrap()(
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

pub unsafe fn zgemm_batch_strided(
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
    dyload_lib().zgemm_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn dzgemm(
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
    dyload_lib().dzgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm3m(
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
    dyload_lib().zgemm3m.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zgemm3m_batch(
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
    dyload_lib().zgemm3m_batch.unwrap()(
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

pub unsafe fn zgemm3m_batch_strided(
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
    dyload_lib().zgemm3m_batch_strided.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, stridea, b, ldb, strideb, beta, c, ldc, stridec,
        batch_size,
    )
}

pub unsafe fn zgemmt(
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
    dyload_lib().zgemmt.unwrap()(uplo, transa, transb, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zhemm(
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
    dyload_lib().zhemm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zher2k(
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
    dyload_lib().zher2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zherk(
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
    dyload_lib().zherk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn zsymm(
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
    dyload_lib().zsymm.unwrap()(side, uplo, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zsyr2k(
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
    dyload_lib().zsyr2k.unwrap()(uplo, trans, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn zsyrk(
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
    dyload_lib().zsyrk.unwrap()(uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
}

pub unsafe fn zsyrk_batch(
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
    dyload_lib().zsyrk_batch.unwrap()(
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

pub unsafe fn zsyrk_batch_strided(
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
    dyload_lib().zsyrk_batch_strided.unwrap()(
        uplo, trans, n, k, alpha, a, lda, stridea, beta, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ztrmm(
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
    dyload_lib().ztrmm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ztrmm_oop(
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
    dyload_lib().ztrmm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ztrsm(
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
    dyload_lib().ztrsm.unwrap()(side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb)
}

pub unsafe fn ztrsm_oop(
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
    dyload_lib().ztrsm_oop.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn ztrsm_batch(
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
    dyload_lib().ztrsm_batch.unwrap()(
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

pub unsafe fn ztrsm_batch_strided(
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
    dyload_lib().ztrsm_batch_strided.unwrap()(
        side, uplo, transa, diag, m, n, alpha, a, lda, stridea, b, ldb, strideb, batch_size,
    )
}

pub unsafe fn gemm_s16s16s32(
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
    dyload_lib().gemm_s16s16s32.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_s8u8s32(
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
    dyload_lib().gemm_s8u8s32.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_bf16bf16f32(
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
    dyload_lib().gemm_bf16bf16f32.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_f16f16f32(
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
    dyload_lib().gemm_f16f16f32.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e5m2e5m2f32(
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
    dyload_lib().gemm_e5m2e5m2f32.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e4m3e4m3f32(
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
    dyload_lib().gemm_e4m3e4m3f32.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_s8u8s32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_s8u8s32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_s16s16s32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_s16s16s32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_bf16bf16f32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_bf16bf16f32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_f16f16f32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_f16f16f32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_e5m2e5m2f32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_e5m2e5m2f32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_e4m3e4m3f32_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().gemm_e4m3e4m3f32_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn gemm_s8u8s32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const c_void,
    ld: *const MKL_INT,
    dest: *mut c_void,
) {
    dyload_lib().gemm_s8u8s32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_s16s16s32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_INT16,
    ld: *const MKL_INT,
    dest: *mut MKL_INT16,
) {
    dyload_lib().gemm_s16s16s32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_bf16bf16f32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_BF16,
    ld: *const MKL_INT,
    dest: *mut MKL_BF16,
) {
    dyload_lib().gemm_bf16bf16f32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_f16f16f32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_F16,
    ld: *const MKL_INT,
    dest: *mut MKL_F16,
) {
    dyload_lib().gemm_f16f16f32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_e5m2e5m2f32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_E5M2,
    ld: *const MKL_INT,
    dest: *mut MKL_E5M2,
) {
    dyload_lib().gemm_e5m2e5m2f32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_e4m3e4m3f32_pack(
    identifier: *const c_char,
    trans: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
    src: *const MKL_E4M3,
    ld: *const MKL_INT,
    dest: *mut MKL_E4M3,
) {
    dyload_lib().gemm_e4m3e4m3f32_pack.unwrap()(identifier, trans, m, n, k, src, ld, dest)
}

pub unsafe fn gemm_s8u8s32_compute(
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
    dyload_lib().gemm_s8u8s32_compute.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_s16s16s32_compute(
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
    dyload_lib().gemm_s16s16s32_compute.unwrap()(
        transa, transb, offsetc, m, n, k, alpha, a, lda, ao, b, ldb, bo, beta, c, ldc, co,
    )
}

pub unsafe fn gemm_bf16bf16f32_compute(
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
    dyload_lib().gemm_bf16bf16f32_compute.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_f16f16f32_compute(
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
    dyload_lib().gemm_f16f16f32_compute.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e5m2e5m2f32_compute(
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
    dyload_lib().gemm_e5m2e5m2f32_compute.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn gemm_e4m3e4m3f32_compute(
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
    dyload_lib().gemm_e4m3e4m3f32_compute.unwrap()(
        transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc,
    )
}

pub unsafe fn hgemm(
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
    dyload_lib().hgemm.unwrap()(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

pub unsafe fn hgemm_pack_get_size(
    identifier: *const c_char,
    m: *const MKL_INT,
    n: *const MKL_INT,
    k: *const MKL_INT,
) -> usize {
    dyload_lib().hgemm_pack_get_size.unwrap()(identifier, m, n, k)
}

pub unsafe fn hgemm_pack(
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
    dyload_lib().hgemm_pack.unwrap()(identifier, trans, m, n, k, alpha, src, ld, dest)
}

pub unsafe fn hgemm_compute(
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
    dyload_lib().hgemm_compute.unwrap()(transa, transb, m, n, k, a, lda, b, ldb, beta, c, ldc)
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

pub unsafe fn saxpy_batch(
    n: *const MKL_INT,
    alpha: *const f32,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().saxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn daxpy_batch(
    n: *const MKL_INT,
    alpha: *const f64,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().daxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn caxpy_batch(
    n: *const MKL_INT,
    alpha: *const MKL_Complex8,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex8,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().caxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn zaxpy_batch(
    n: *const MKL_INT,
    alpha: *const MKL_Complex16,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex16,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zaxpy_batch.unwrap()(n, alpha, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn scopy_batch(
    n: *const MKL_INT,
    x: *mut *const f32,
    incx: *const MKL_INT,
    y: *mut *mut f32,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().scopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn dcopy_batch(
    n: *const MKL_INT,
    x: *mut *const f64,
    incx: *const MKL_INT,
    y: *mut *mut f64,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().dcopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn ccopy_batch(
    n: *const MKL_INT,
    x: *mut *const MKL_Complex8,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex8,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().ccopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn zcopy_batch(
    n: *const MKL_INT,
    x: *mut *const MKL_Complex16,
    incx: *const MKL_INT,
    y: *mut *mut MKL_Complex16,
    incy: *const MKL_INT,
    group_count: *const MKL_INT,
    group_size: *const MKL_INT,
) {
    dyload_lib().zcopy_batch.unwrap()(n, x, incx, y, incy, group_count, group_size)
}

pub unsafe fn saxpy_batch_strided(
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
    dyload_lib().saxpy_batch_strided.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn daxpy_batch_strided(
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
    dyload_lib().daxpy_batch_strided.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn caxpy_batch_strided(
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
    dyload_lib().caxpy_batch_strided.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn zaxpy_batch_strided(
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
    dyload_lib().zaxpy_batch_strided.unwrap()(
        n, alpha, x, incx, stridex, y, incy, stridey, batch_size,
    )
}

pub unsafe fn scopy_batch_strided(
    n: *const MKL_INT,
    x: *const f32,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f32,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().scopy_batch_strided.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn dcopy_batch_strided(
    n: *const MKL_INT,
    x: *const f64,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut f64,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().dcopy_batch_strided.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn ccopy_batch_strided(
    n: *const MKL_INT,
    x: *const MKL_Complex8,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex8,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().ccopy_batch_strided.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn zcopy_batch_strided(
    n: *const MKL_INT,
    x: *const MKL_Complex16,
    incx: *const MKL_INT,
    stridex: *const MKL_INT,
    y: *mut MKL_Complex16,
    incy: *const MKL_INT,
    stridey: *const MKL_INT,
    batch_size: *const MKL_INT,
) {
    dyload_lib().zcopy_batch_strided.unwrap()(n, x, incx, stridex, y, incy, stridey, batch_size)
}

pub unsafe fn sgemv_batch(
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
    dyload_lib().sgemv_batch.unwrap()(
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

pub unsafe fn sgemv_batch_strided(
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
    dyload_lib().sgemv_batch_strided.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn dgemv_batch(
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
    dyload_lib().dgemv_batch.unwrap()(
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

pub unsafe fn dgemv_batch_strided(
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
    dyload_lib().dgemv_batch_strided.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn cgemv_batch(
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
    dyload_lib().cgemv_batch.unwrap()(
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

pub unsafe fn cgemv_batch_strided(
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
    dyload_lib().cgemv_batch_strided.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn zgemv_batch(
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
    dyload_lib().zgemv_batch.unwrap()(
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

pub unsafe fn zgemv_batch_strided(
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
    dyload_lib().zgemv_batch_strided.unwrap()(
        trans, m, n, alpha, a, lda, stridea, x, incx, stridex, beta, y, incy, stridey, batch_size,
    )
}

pub unsafe fn sdgmm_batch(
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
    dyload_lib().sdgmm_batch.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn sdgmm_batch_strided(
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
    dyload_lib().sdgmm_batch_strided.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn ddgmm_batch(
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
    dyload_lib().ddgmm_batch.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn ddgmm_batch_strided(
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
    dyload_lib().ddgmm_batch_strided.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn cdgmm_batch(
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
    dyload_lib().cdgmm_batch.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn cdgmm_batch_strided(
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
    dyload_lib().cdgmm_batch_strided.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

pub unsafe fn zdgmm_batch(
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
    dyload_lib().zdgmm_batch.unwrap()(side, m, n, a, lda, x, incx, c, ldc, group_count, group_size)
}

pub unsafe fn zdgmm_batch_strided(
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
    dyload_lib().zdgmm_batch_strided.unwrap()(
        side, m, n, a, lda, stridea, x, incx, stridex, c, ldc, stridec, batch_size,
    )
}

/* #region upper case alias */

pub use caxpby as CAXPBY;
pub use caxpy as CAXPY;
pub use caxpy_batch as CAXPY_BATCH;
pub use caxpy_batch_strided as CAXPY_BATCH_STRIDED;
pub use caxpyi as CAXPYI;
pub use ccopy as CCOPY;
pub use ccopy_batch as CCOPY_BATCH;
pub use ccopy_batch_strided as CCOPY_BATCH_STRIDED;
pub use cdgmm_batch as CDGMM_BATCH;
pub use cdgmm_batch_strided as CDGMM_BATCH_STRIDED;
pub use cdotc as CDOTC;
pub use cdotci as CDOTCI;
pub use cdotu as CDOTU;
pub use cdotui as CDOTUI;
pub use cgbmv as CGBMV;
pub use cgem2vc as CGEM2VC;
pub use cgemm as CGEMM;
pub use cgemm3m as CGEMM3M;
pub use cgemm3m_batch as CGEMM3M_BATCH;
pub use cgemm3m_batch_strided as CGEMM3M_BATCH_STRIDED;
pub use cgemm_batch as CGEMM_BATCH;
pub use cgemm_batch_strided as CGEMM_BATCH_STRIDED;
pub use cgemmt as CGEMMT;
pub use cgemv as CGEMV;
pub use cgemv_batch as CGEMV_BATCH;
pub use cgemv_batch_strided as CGEMV_BATCH_STRIDED;
pub use cgerc as CGERC;
pub use cgeru as CGERU;
pub use cgthr as CGTHR;
pub use cgthrz as CGTHRZ;
pub use chbmv as CHBMV;
pub use chemm as CHEMM;
pub use chemv as CHEMV;
pub use cher as CHER;
pub use cher2 as CHER2;
pub use cher2k as CHER2K;
pub use cherk as CHERK;
pub use chpmv as CHPMV;
pub use chpr as CHPR;
pub use chpr2 as CHPR2;
pub use crot as CROT;
pub use crotg as CROTG;
pub use cscal as CSCAL;
pub use csctr as CSCTR;
pub use csrot as CSROT;
pub use csscal as CSSCAL;
pub use cswap as CSWAP;
pub use csymm as CSYMM;
pub use csyr2k as CSYR2K;
pub use csyrk as CSYRK;
pub use csyrk_batch as CSYRK_BATCH;
pub use csyrk_batch_strided as CSYRK_BATCH_STRIDED;
pub use ctbmv as CTBMV;
pub use ctbsv as CTBSV;
pub use ctpmv as CTPMV;
pub use ctpsv as CTPSV;
pub use ctrmm as CTRMM;
pub use ctrmm_oop as CTRMM_OOP;
pub use ctrmv as CTRMV;
pub use ctrsm as CTRSM;
pub use ctrsm_batch as CTRSM_BATCH;
pub use ctrsm_batch_strided as CTRSM_BATCH_STRIDED;
pub use ctrsm_oop as CTRSM_OOP;
pub use ctrsv as CTRSV;
pub use dasum as DASUM;
pub use daxpby as DAXPBY;
pub use daxpy as DAXPY;
pub use daxpy_batch as DAXPY_BATCH;
pub use daxpy_batch_strided as DAXPY_BATCH_STRIDED;
pub use daxpyi as DAXPYI;
pub use dcabs1 as DCABS1;
pub use dcopy as DCOPY;
pub use dcopy_batch as DCOPY_BATCH;
pub use dcopy_batch_strided as DCOPY_BATCH_STRIDED;
pub use ddgmm_batch as DDGMM_BATCH;
pub use ddgmm_batch_strided as DDGMM_BATCH_STRIDED;
pub use ddot as DDOT;
pub use ddoti as DDOTI;
pub use dgbmv as DGBMV;
pub use dgem2vu as DGEM2VU;
pub use dgemm as DGEMM;
pub use dgemm_batch as DGEMM_BATCH;
pub use dgemm_batch_strided as DGEMM_BATCH_STRIDED;
pub use dgemm_compute as DGEMM_COMPUTE;
pub use dgemm_pack as DGEMM_PACK;
pub use dgemm_pack_get_size as DGEMM_PACK_GET_SIZE;
pub use dgemmt as DGEMMT;
pub use dgemv as DGEMV;
pub use dgemv_batch as DGEMV_BATCH;
pub use dgemv_batch_strided as DGEMV_BATCH_STRIDED;
pub use dger as DGER;
pub use dgthr as DGTHR;
pub use dgthrz as DGTHRZ;
pub use dnrm2 as DNRM2;
pub use drot as DROT;
pub use drotg as DROTG;
pub use droti as DROTI;
pub use drotm as DROTM;
pub use drotmg as DROTMG;
pub use dsbmv as DSBMV;
pub use dscal as DSCAL;
pub use dsctr as DSCTR;
pub use dsdot as DSDOT;
pub use dspmv as DSPMV;
pub use dspr as DSPR;
pub use dspr2 as DSPR2;
pub use dswap as DSWAP;
pub use dsymm as DSYMM;
pub use dsymv as DSYMV;
pub use dsyr as DSYR;
pub use dsyr2 as DSYR2;
pub use dsyr2k as DSYR2K;
pub use dsyrk as DSYRK;
pub use dsyrk_batch as DSYRK_BATCH;
pub use dsyrk_batch_strided as DSYRK_BATCH_STRIDED;
pub use dtbmv as DTBMV;
pub use dtbsv as DTBSV;
pub use dtpmv as DTPMV;
pub use dtpsv as DTPSV;
pub use dtrmm as DTRMM;
pub use dtrmm_oop as DTRMM_OOP;
pub use dtrmv as DTRMV;
pub use dtrsm as DTRSM;
pub use dtrsm_batch as DTRSM_BATCH;
pub use dtrsm_batch_strided as DTRSM_BATCH_STRIDED;
pub use dtrsm_oop as DTRSM_OOP;
pub use dtrsv as DTRSV;
pub use dzasum as DZASUM;
pub use dzgemm as DZGEMM;
pub use dzgemv as DZGEMV;
pub use dznrm2 as DZNRM2;
pub use gemm_bf16bf16f32 as GEMM_BF16BF16F32;
pub use gemm_bf16bf16f32_compute as GEMM_BF16BF16F32_COMPUTE;
pub use gemm_bf16bf16f32_pack as GEMM_BF16BF16F32_PACK;
pub use gemm_bf16bf16f32_pack_get_size as GEMM_BF16BF16F32_PACK_GET_SIZE;
pub use gemm_e4m3e4m3f32 as GEMM_E4M3E4M3F32;
pub use gemm_e4m3e4m3f32_compute as GEMM_E4M3E4M3F32_COMPUTE;
pub use gemm_e4m3e4m3f32_pack as GEMM_E4M3E4M3F32_PACK;
pub use gemm_e4m3e4m3f32_pack_get_size as GEMM_E4M3E4M3F32_PACK_GET_SIZE;
pub use gemm_e5m2e5m2f32 as GEMM_E5M2E5M2F32;
pub use gemm_e5m2e5m2f32_compute as GEMM_E5M2E5M2F32_COMPUTE;
pub use gemm_e5m2e5m2f32_pack as GEMM_E5M2E5M2F32_PACK;
pub use gemm_e5m2e5m2f32_pack_get_size as GEMM_E5M2E5M2F32_PACK_GET_SIZE;
pub use gemm_f16f16f32 as GEMM_F16F16F32;
pub use gemm_f16f16f32_compute as GEMM_F16F16F32_COMPUTE;
pub use gemm_f16f16f32_pack as GEMM_F16F16F32_PACK;
pub use gemm_f16f16f32_pack_get_size as GEMM_F16F16F32_PACK_GET_SIZE;
pub use gemm_s16s16s32 as GEMM_S16S16S32;
pub use gemm_s16s16s32_compute as GEMM_S16S16S32_COMPUTE;
pub use gemm_s16s16s32_pack as GEMM_S16S16S32_PACK;
pub use gemm_s16s16s32_pack_get_size as GEMM_S16S16S32_PACK_GET_SIZE;
pub use gemm_s8u8s32 as GEMM_S8U8S32;
pub use gemm_s8u8s32_compute as GEMM_S8U8S32_COMPUTE;
pub use gemm_s8u8s32_pack as GEMM_S8U8S32_PACK;
pub use gemm_s8u8s32_pack_get_size as GEMM_S8U8S32_PACK_GET_SIZE;
pub use hgemm as HGEMM;
pub use hgemm_compute as HGEMM_COMPUTE;
pub use hgemm_pack as HGEMM_PACK;
pub use hgemm_pack_get_size as HGEMM_PACK_GET_SIZE;
pub use icamax as ICAMAX;
pub use icamin as ICAMIN;
pub use idamax as IDAMAX;
pub use idamin as IDAMIN;
pub use isamax as ISAMAX;
pub use isamin as ISAMIN;
pub use izamax as IZAMAX;
pub use izamin as IZAMIN;
pub use lsame as LSAME;
pub use mkl_cblas_jit_create_cgemm as MKL_CBLAS_JIT_CREATE_CGEMM;
pub use mkl_cblas_jit_create_dgemm as MKL_CBLAS_JIT_CREATE_DGEMM;
pub use mkl_cblas_jit_create_sgemm as MKL_CBLAS_JIT_CREATE_SGEMM;
pub use mkl_cblas_jit_create_zgemm as MKL_CBLAS_JIT_CREATE_ZGEMM;
pub use mkl_jit_destroy as MKL_JIT_DESTROY;
pub use mkl_jit_get_cgemm_ptr as MKL_JIT_GET_CGEMM_PTR;
pub use mkl_jit_get_dgemm_ptr as MKL_JIT_GET_DGEMM_PTR;
pub use mkl_jit_get_sgemm_ptr as MKL_JIT_GET_SGEMM_PTR;
pub use mkl_jit_get_zgemm_ptr as MKL_JIT_GET_ZGEMM_PTR;
pub use sasum as SASUM;
pub use saxpby as SAXPBY;
pub use saxpy as SAXPY;
pub use saxpy_batch as SAXPY_BATCH;
pub use saxpy_batch_strided as SAXPY_BATCH_STRIDED;
pub use saxpyi as SAXPYI;
pub use scabs1 as SCABS1;
pub use scasum as SCASUM;
pub use scgemm as SCGEMM;
pub use scgemv as SCGEMV;
pub use scnrm2 as SCNRM2;
pub use scopy as SCOPY;
pub use scopy_batch as SCOPY_BATCH;
pub use scopy_batch_strided as SCOPY_BATCH_STRIDED;
pub use sdgmm_batch as SDGMM_BATCH;
pub use sdgmm_batch_strided as SDGMM_BATCH_STRIDED;
pub use sdot as SDOT;
pub use sdoti as SDOTI;
pub use sdsdot as SDSDOT;
pub use sgbmv as SGBMV;
pub use sgem2vu as SGEM2VU;
pub use sgemm as SGEMM;
pub use sgemm_batch as SGEMM_BATCH;
pub use sgemm_batch_strided as SGEMM_BATCH_STRIDED;
pub use sgemm_compute as SGEMM_COMPUTE;
pub use sgemm_pack as SGEMM_PACK;
pub use sgemm_pack_get_size as SGEMM_PACK_GET_SIZE;
pub use sgemmt as SGEMMT;
pub use sgemv as SGEMV;
pub use sgemv_batch as SGEMV_BATCH;
pub use sgemv_batch_strided as SGEMV_BATCH_STRIDED;
pub use sger as SGER;
pub use sgthr as SGTHR;
pub use sgthrz as SGTHRZ;
pub use snrm2 as SNRM2;
pub use srot as SROT;
pub use srotg as SROTG;
pub use sroti as SROTI;
pub use srotm as SROTM;
pub use srotmg as SROTMG;
pub use ssbmv as SSBMV;
pub use sscal as SSCAL;
pub use ssctr as SSCTR;
pub use sspmv as SSPMV;
pub use sspr as SSPR;
pub use sspr2 as SSPR2;
pub use sswap as SSWAP;
pub use ssymm as SSYMM;
pub use ssymv as SSYMV;
pub use ssyr as SSYR;
pub use ssyr2 as SSYR2;
pub use ssyr2k as SSYR2K;
pub use ssyrk as SSYRK;
pub use ssyrk_batch as SSYRK_BATCH;
pub use ssyrk_batch_strided as SSYRK_BATCH_STRIDED;
pub use stbmv as STBMV;
pub use stbsv as STBSV;
pub use stpmv as STPMV;
pub use stpsv as STPSV;
pub use strmm as STRMM;
pub use strmm_oop as STRMM_OOP;
pub use strmv as STRMV;
pub use strsm as STRSM;
pub use strsm_batch as STRSM_BATCH;
pub use strsm_batch_strided as STRSM_BATCH_STRIDED;
pub use strsm_oop as STRSM_OOP;
pub use strsv as STRSV;
pub use xerbla as XERBLA;
pub use zaxpby as ZAXPBY;
pub use zaxpy as ZAXPY;
pub use zaxpy_batch as ZAXPY_BATCH;
pub use zaxpy_batch_strided as ZAXPY_BATCH_STRIDED;
pub use zaxpyi as ZAXPYI;
pub use zcopy as ZCOPY;
pub use zcopy_batch as ZCOPY_BATCH;
pub use zcopy_batch_strided as ZCOPY_BATCH_STRIDED;
pub use zdgmm_batch as ZDGMM_BATCH;
pub use zdgmm_batch_strided as ZDGMM_BATCH_STRIDED;
pub use zdotc as ZDOTC;
pub use zdotci as ZDOTCI;
pub use zdotu as ZDOTU;
pub use zdotui as ZDOTUI;
pub use zdrot as ZDROT;
pub use zdscal as ZDSCAL;
pub use zgbmv as ZGBMV;
pub use zgem2vc as ZGEM2VC;
pub use zgemm as ZGEMM;
pub use zgemm3m as ZGEMM3M;
pub use zgemm3m_batch as ZGEMM3M_BATCH;
pub use zgemm3m_batch_strided as ZGEMM3M_BATCH_STRIDED;
pub use zgemm_batch as ZGEMM_BATCH;
pub use zgemm_batch_strided as ZGEMM_BATCH_STRIDED;
pub use zgemmt as ZGEMMT;
pub use zgemv as ZGEMV;
pub use zgemv_batch as ZGEMV_BATCH;
pub use zgemv_batch_strided as ZGEMV_BATCH_STRIDED;
pub use zgerc as ZGERC;
pub use zgeru as ZGERU;
pub use zgthr as ZGTHR;
pub use zgthrz as ZGTHRZ;
pub use zhbmv as ZHBMV;
pub use zhemm as ZHEMM;
pub use zhemv as ZHEMV;
pub use zher as ZHER;
pub use zher2 as ZHER2;
pub use zher2k as ZHER2K;
pub use zherk as ZHERK;
pub use zhpmv as ZHPMV;
pub use zhpr as ZHPR;
pub use zhpr2 as ZHPR2;
pub use zrot as ZROT;
pub use zrotg as ZROTG;
pub use zscal as ZSCAL;
pub use zsctr as ZSCTR;
pub use zswap as ZSWAP;
pub use zsymm as ZSYMM;
pub use zsyr2k as ZSYR2K;
pub use zsyrk as ZSYRK;
pub use zsyrk_batch as ZSYRK_BATCH;
pub use zsyrk_batch_strided as ZSYRK_BATCH_STRIDED;
pub use ztbmv as ZTBMV;
pub use ztbsv as ZTBSV;
pub use ztpmv as ZTPMV;
pub use ztpsv as ZTPSV;
pub use ztrmm as ZTRMM;
pub use ztrmm_oop as ZTRMM_OOP;
pub use ztrmv as ZTRMV;
pub use ztrsm as ZTRSM;
pub use ztrsm_batch as ZTRSM_BATCH;
pub use ztrsm_batch_strided as ZTRSM_BATCH_STRIDED;
pub use ztrsm_oop as ZTRSM_OOP;
pub use ztrsv as ZTRSV;

/* #endregion */

/* #region lower case with underscore alias */

pub use caxpby as caxpby_;
pub use caxpy as caxpy_;
pub use caxpy_batch as caxpy_batch_;
pub use caxpy_batch_strided as caxpy_batch_strided_;
pub use caxpyi as caxpyi_;
pub use ccopy as ccopy_;
pub use ccopy_batch as ccopy_batch_;
pub use ccopy_batch_strided as ccopy_batch_strided_;
pub use cdgmm_batch as cdgmm_batch_;
pub use cdgmm_batch_strided as cdgmm_batch_strided_;
pub use cdotc as cdotc_;
pub use cdotci as cdotci_;
pub use cdotu as cdotu_;
pub use cdotui as cdotui_;
pub use cgbmv as cgbmv_;
pub use cgem2vc as cgem2vc_;
pub use cgemm as cgemm_;
pub use cgemm3m as cgemm3m_;
pub use cgemm3m_batch as cgemm3m_batch_;
pub use cgemm3m_batch_strided as cgemm3m_batch_strided_;
pub use cgemm_batch as cgemm_batch_;
pub use cgemm_batch_strided as cgemm_batch_strided_;
pub use cgemmt as cgemmt_;
pub use cgemv as cgemv_;
pub use cgemv_batch as cgemv_batch_;
pub use cgemv_batch_strided as cgemv_batch_strided_;
pub use cgerc as cgerc_;
pub use cgeru as cgeru_;
pub use cgthr as cgthr_;
pub use cgthrz as cgthrz_;
pub use chbmv as chbmv_;
pub use chemm as chemm_;
pub use chemv as chemv_;
pub use cher as cher_;
pub use cher2 as cher2_;
pub use cher2k as cher2k_;
pub use cherk as cherk_;
pub use chpmv as chpmv_;
pub use chpr as chpr_;
pub use chpr2 as chpr2_;
pub use crot as crot_;
pub use crotg as crotg_;
pub use cscal as cscal_;
pub use csctr as csctr_;
pub use csrot as csrot_;
pub use csscal as csscal_;
pub use cswap as cswap_;
pub use csymm as csymm_;
pub use csyr2k as csyr2k_;
pub use csyrk as csyrk_;
pub use csyrk_batch as csyrk_batch_;
pub use csyrk_batch_strided as csyrk_batch_strided_;
pub use ctbmv as ctbmv_;
pub use ctbsv as ctbsv_;
pub use ctpmv as ctpmv_;
pub use ctpsv as ctpsv_;
pub use ctrmm as ctrmm_;
pub use ctrmm_oop as ctrmm_oop_;
pub use ctrmv as ctrmv_;
pub use ctrsm as ctrsm_;
pub use ctrsm_batch as ctrsm_batch_;
pub use ctrsm_batch_strided as ctrsm_batch_strided_;
pub use ctrsm_oop as ctrsm_oop_;
pub use ctrsv as ctrsv_;
pub use dasum as dasum_;
pub use daxpby as daxpby_;
pub use daxpy as daxpy_;
pub use daxpy_batch as daxpy_batch_;
pub use daxpy_batch_strided as daxpy_batch_strided_;
pub use daxpyi as daxpyi_;
pub use dcabs1 as dcabs1_;
pub use dcopy as dcopy_;
pub use dcopy_batch as dcopy_batch_;
pub use dcopy_batch_strided as dcopy_batch_strided_;
pub use ddgmm_batch as ddgmm_batch_;
pub use ddgmm_batch_strided as ddgmm_batch_strided_;
pub use ddot as ddot_;
pub use ddoti as ddoti_;
pub use dgbmv as dgbmv_;
pub use dgem2vu as dgem2vu_;
pub use dgemm as dgemm_;
pub use dgemm_batch as dgemm_batch_;
pub use dgemm_batch_strided as dgemm_batch_strided_;
pub use dgemm_compute as dgemm_compute_;
pub use dgemm_pack as dgemm_pack_;
pub use dgemm_pack_get_size as dgemm_pack_get_size_;
pub use dgemmt as dgemmt_;
pub use dgemv as dgemv_;
pub use dgemv_batch as dgemv_batch_;
pub use dgemv_batch_strided as dgemv_batch_strided_;
pub use dger as dger_;
pub use dgthr as dgthr_;
pub use dgthrz as dgthrz_;
pub use dnrm2 as dnrm2_;
pub use drot as drot_;
pub use drotg as drotg_;
pub use droti as droti_;
pub use drotm as drotm_;
pub use drotmg as drotmg_;
pub use dsbmv as dsbmv_;
pub use dscal as dscal_;
pub use dsctr as dsctr_;
pub use dsdot as dsdot_;
pub use dspmv as dspmv_;
pub use dspr as dspr_;
pub use dspr2 as dspr2_;
pub use dswap as dswap_;
pub use dsymm as dsymm_;
pub use dsymv as dsymv_;
pub use dsyr as dsyr_;
pub use dsyr2 as dsyr2_;
pub use dsyr2k as dsyr2k_;
pub use dsyrk as dsyrk_;
pub use dsyrk_batch as dsyrk_batch_;
pub use dsyrk_batch_strided as dsyrk_batch_strided_;
pub use dtbmv as dtbmv_;
pub use dtbsv as dtbsv_;
pub use dtpmv as dtpmv_;
pub use dtpsv as dtpsv_;
pub use dtrmm as dtrmm_;
pub use dtrmm_oop as dtrmm_oop_;
pub use dtrmv as dtrmv_;
pub use dtrsm as dtrsm_;
pub use dtrsm_batch as dtrsm_batch_;
pub use dtrsm_batch_strided as dtrsm_batch_strided_;
pub use dtrsm_oop as dtrsm_oop_;
pub use dtrsv as dtrsv_;
pub use dzasum as dzasum_;
pub use dzgemm as dzgemm_;
pub use dzgemv as dzgemv_;
pub use dznrm2 as dznrm2_;
pub use gemm_bf16bf16f32 as gemm_bf16bf16f32_;
pub use gemm_bf16bf16f32_compute as gemm_bf16bf16f32_compute_;
pub use gemm_bf16bf16f32_pack as gemm_bf16bf16f32_pack_;
pub use gemm_bf16bf16f32_pack_get_size as gemm_bf16bf16f32_pack_get_size_;
pub use gemm_e4m3e4m3f32 as gemm_e4m3e4m3f32_;
pub use gemm_e4m3e4m3f32_compute as gemm_e4m3e4m3f32_compute_;
pub use gemm_e4m3e4m3f32_pack as gemm_e4m3e4m3f32_pack_;
pub use gemm_e4m3e4m3f32_pack_get_size as gemm_e4m3e4m3f32_pack_get_size_;
pub use gemm_e5m2e5m2f32 as gemm_e5m2e5m2f32_;
pub use gemm_e5m2e5m2f32_compute as gemm_e5m2e5m2f32_compute_;
pub use gemm_e5m2e5m2f32_pack as gemm_e5m2e5m2f32_pack_;
pub use gemm_e5m2e5m2f32_pack_get_size as gemm_e5m2e5m2f32_pack_get_size_;
pub use gemm_f16f16f32 as gemm_f16f16f32_;
pub use gemm_f16f16f32_compute as gemm_f16f16f32_compute_;
pub use gemm_f16f16f32_pack as gemm_f16f16f32_pack_;
pub use gemm_f16f16f32_pack_get_size as gemm_f16f16f32_pack_get_size_;
pub use gemm_s16s16s32 as gemm_s16s16s32_;
pub use gemm_s16s16s32_compute as gemm_s16s16s32_compute_;
pub use gemm_s16s16s32_pack as gemm_s16s16s32_pack_;
pub use gemm_s16s16s32_pack_get_size as gemm_s16s16s32_pack_get_size_;
pub use gemm_s8u8s32 as gemm_s8u8s32_;
pub use gemm_s8u8s32_compute as gemm_s8u8s32_compute_;
pub use gemm_s8u8s32_pack as gemm_s8u8s32_pack_;
pub use gemm_s8u8s32_pack_get_size as gemm_s8u8s32_pack_get_size_;
pub use hgemm as hgemm_;
pub use hgemm_compute as hgemm_compute_;
pub use hgemm_pack as hgemm_pack_;
pub use hgemm_pack_get_size as hgemm_pack_get_size_;
pub use icamax as icamax_;
pub use icamin as icamin_;
pub use idamax as idamax_;
pub use idamin as idamin_;
pub use isamax as isamax_;
pub use isamin as isamin_;
pub use izamax as izamax_;
pub use izamin as izamin_;
pub use lsame as lsame_;
pub use mkl_cblas_jit_create_cgemm as mkl_cblas_jit_create_cgemm_;
pub use mkl_cblas_jit_create_dgemm as mkl_cblas_jit_create_dgemm_;
pub use mkl_cblas_jit_create_sgemm as mkl_cblas_jit_create_sgemm_;
pub use mkl_cblas_jit_create_zgemm as mkl_cblas_jit_create_zgemm_;
pub use mkl_jit_destroy as mkl_jit_destroy_;
pub use mkl_jit_get_cgemm_ptr as mkl_jit_get_cgemm_ptr_;
pub use mkl_jit_get_dgemm_ptr as mkl_jit_get_dgemm_ptr_;
pub use mkl_jit_get_sgemm_ptr as mkl_jit_get_sgemm_ptr_;
pub use mkl_jit_get_zgemm_ptr as mkl_jit_get_zgemm_ptr_;
pub use sasum as sasum_;
pub use saxpby as saxpby_;
pub use saxpy as saxpy_;
pub use saxpy_batch as saxpy_batch_;
pub use saxpy_batch_strided as saxpy_batch_strided_;
pub use saxpyi as saxpyi_;
pub use scabs1 as scabs1_;
pub use scasum as scasum_;
pub use scgemm as scgemm_;
pub use scgemv as scgemv_;
pub use scnrm2 as scnrm2_;
pub use scopy as scopy_;
pub use scopy_batch as scopy_batch_;
pub use scopy_batch_strided as scopy_batch_strided_;
pub use sdgmm_batch as sdgmm_batch_;
pub use sdgmm_batch_strided as sdgmm_batch_strided_;
pub use sdot as sdot_;
pub use sdoti as sdoti_;
pub use sdsdot as sdsdot_;
pub use sgbmv as sgbmv_;
pub use sgem2vu as sgem2vu_;
pub use sgemm as sgemm_;
pub use sgemm_batch as sgemm_batch_;
pub use sgemm_batch_strided as sgemm_batch_strided_;
pub use sgemm_compute as sgemm_compute_;
pub use sgemm_pack as sgemm_pack_;
pub use sgemm_pack_get_size as sgemm_pack_get_size_;
pub use sgemmt as sgemmt_;
pub use sgemv as sgemv_;
pub use sgemv_batch as sgemv_batch_;
pub use sgemv_batch_strided as sgemv_batch_strided_;
pub use sger as sger_;
pub use sgthr as sgthr_;
pub use sgthrz as sgthrz_;
pub use snrm2 as snrm2_;
pub use srot as srot_;
pub use srotg as srotg_;
pub use sroti as sroti_;
pub use srotm as srotm_;
pub use srotmg as srotmg_;
pub use ssbmv as ssbmv_;
pub use sscal as sscal_;
pub use ssctr as ssctr_;
pub use sspmv as sspmv_;
pub use sspr as sspr_;
pub use sspr2 as sspr2_;
pub use sswap as sswap_;
pub use ssymm as ssymm_;
pub use ssymv as ssymv_;
pub use ssyr as ssyr_;
pub use ssyr2 as ssyr2_;
pub use ssyr2k as ssyr2k_;
pub use ssyrk as ssyrk_;
pub use ssyrk_batch as ssyrk_batch_;
pub use ssyrk_batch_strided as ssyrk_batch_strided_;
pub use stbmv as stbmv_;
pub use stbsv as stbsv_;
pub use stpmv as stpmv_;
pub use stpsv as stpsv_;
pub use strmm as strmm_;
pub use strmm_oop as strmm_oop_;
pub use strmv as strmv_;
pub use strsm as strsm_;
pub use strsm_batch as strsm_batch_;
pub use strsm_batch_strided as strsm_batch_strided_;
pub use strsm_oop as strsm_oop_;
pub use strsv as strsv_;
pub use xerbla as xerbla_;
pub use zaxpby as zaxpby_;
pub use zaxpy as zaxpy_;
pub use zaxpy_batch as zaxpy_batch_;
pub use zaxpy_batch_strided as zaxpy_batch_strided_;
pub use zaxpyi as zaxpyi_;
pub use zcopy as zcopy_;
pub use zcopy_batch as zcopy_batch_;
pub use zcopy_batch_strided as zcopy_batch_strided_;
pub use zdgmm_batch as zdgmm_batch_;
pub use zdgmm_batch_strided as zdgmm_batch_strided_;
pub use zdotc as zdotc_;
pub use zdotci as zdotci_;
pub use zdotu as zdotu_;
pub use zdotui as zdotui_;
pub use zdrot as zdrot_;
pub use zdscal as zdscal_;
pub use zgbmv as zgbmv_;
pub use zgem2vc as zgem2vc_;
pub use zgemm as zgemm_;
pub use zgemm3m as zgemm3m_;
pub use zgemm3m_batch as zgemm3m_batch_;
pub use zgemm3m_batch_strided as zgemm3m_batch_strided_;
pub use zgemm_batch as zgemm_batch_;
pub use zgemm_batch_strided as zgemm_batch_strided_;
pub use zgemmt as zgemmt_;
pub use zgemv as zgemv_;
pub use zgemv_batch as zgemv_batch_;
pub use zgemv_batch_strided as zgemv_batch_strided_;
pub use zgerc as zgerc_;
pub use zgeru as zgeru_;
pub use zgthr as zgthr_;
pub use zgthrz as zgthrz_;
pub use zhbmv as zhbmv_;
pub use zhemm as zhemm_;
pub use zhemv as zhemv_;
pub use zher as zher_;
pub use zher2 as zher2_;
pub use zher2k as zher2k_;
pub use zherk as zherk_;
pub use zhpmv as zhpmv_;
pub use zhpr as zhpr_;
pub use zhpr2 as zhpr2_;
pub use zrot as zrot_;
pub use zrotg as zrotg_;
pub use zscal as zscal_;
pub use zsctr as zsctr_;
pub use zswap as zswap_;
pub use zsymm as zsymm_;
pub use zsyr2k as zsyr2k_;
pub use zsyrk as zsyrk_;
pub use zsyrk_batch as zsyrk_batch_;
pub use zsyrk_batch_strided as zsyrk_batch_strided_;
pub use ztbmv as ztbmv_;
pub use ztbsv as ztbsv_;
pub use ztpmv as ztpmv_;
pub use ztpsv as ztpsv_;
pub use ztrmm as ztrmm_;
pub use ztrmm_oop as ztrmm_oop_;
pub use ztrmv as ztrmv_;
pub use ztrsm as ztrsm_;
pub use ztrsm_batch as ztrsm_batch_;
pub use ztrsm_batch_strided as ztrsm_batch_strided_;
pub use ztrsm_oop as ztrsm_oop_;
pub use ztrsv as ztrsv_;

/* #endregion */
