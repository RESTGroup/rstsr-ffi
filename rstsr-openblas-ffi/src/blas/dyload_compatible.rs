//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn xerbla_(arg1: *mut c_char, info: *mut blas_int, arg2: blas_int) -> c_int {
    dyload_lib().xerbla_.unwrap()(arg1, info, arg2)
}

pub unsafe fn openblas_set_num_threads_(arg1: *mut c_int) {
    dyload_lib().openblas_set_num_threads_.unwrap()(arg1)
}

pub unsafe fn sdot_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> f32 {
    dyload_lib().sdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sdsdot_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
) -> f32 {
    dyload_lib().sdsdot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dsdot_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> f64 {
    dyload_lib().dsdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn ddot_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) -> f64 {
    dyload_lib().ddot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qdot_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) -> xdouble {
    dyload_lib().qdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sbdot_(
    arg1: *mut blas_int,
    arg2: *mut bfloat16,
    arg3: *mut blas_int,
    arg4: *mut bfloat16,
    arg5: *mut blas_int,
) -> f32 {
    dyload_lib().sbdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sbstobf16_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut bfloat16,
    arg5: *mut blas_int,
) {
    dyload_lib().sbstobf16_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sbdtobf16_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut bfloat16,
    arg5: *mut blas_int,
) {
    dyload_lib().sbdtobf16_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sbf16tos_(
    arg1: *mut blas_int,
    arg2: *mut bfloat16,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) {
    dyload_lib().sbf16tos_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dbf16tod_(
    arg1: *mut blas_int,
    arg2: *mut bfloat16,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) {
    dyload_lib().dbf16tod_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cdotu_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> openblas_complex_float {
    dyload_lib().cdotu_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cdotc_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> openblas_complex_float {
    dyload_lib().cdotc_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zdotu_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) -> openblas_complex_double {
    dyload_lib().zdotu_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zdotc_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) -> openblas_complex_double {
    dyload_lib().zdotc_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xdotu_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) -> openblas_complex_xdouble {
    dyload_lib().xdotu_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xdotc_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) -> openblas_complex_xdouble {
    dyload_lib().xdotc_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn saxpy_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
) {
    dyload_lib().saxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn daxpy_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
) {
    dyload_lib().daxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qaxpy_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
) {
    dyload_lib().qaxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn caxpy_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
) {
    dyload_lib().caxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zaxpy_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
) {
    dyload_lib().zaxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xaxpy_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
) {
    dyload_lib().xaxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn caxpyc_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
) {
    dyload_lib().caxpyc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zaxpyc_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
) {
    dyload_lib().zaxpyc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xaxpyc_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
) {
    dyload_lib().xaxpyc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn scopy_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) {
    dyload_lib().scopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dcopy_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) {
    dyload_lib().dcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qcopy_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) {
    dyload_lib().qcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn ccopy_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) {
    dyload_lib().ccopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zcopy_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) {
    dyload_lib().zcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xcopy_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) {
    dyload_lib().xcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sswap_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) {
    dyload_lib().sswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dswap_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) {
    dyload_lib().dswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qswap_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) {
    dyload_lib().qswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cswap_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) {
    dyload_lib().cswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zswap_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) {
    dyload_lib().zswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xswap_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
) {
    dyload_lib().xswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sasum_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().sasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scasum_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dasum_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qasum_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzasum_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxasum_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ssum_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().ssum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scsum_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scsum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dsum_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dsum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qsum_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qsum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzsum_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzsum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxsum_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxsum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn isamax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().isamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn idamax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().idamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn iqamax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().iqamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn icamax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().icamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn izamax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().izamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ixamax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ixamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ismax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ismax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn idmax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().idmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn iqmax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().iqmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn icmax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().icmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn izmax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().izmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ixmax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ixmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn isamin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().isamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn idamin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().idamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn iqamin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().iqamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn icamin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().icamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn izamin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().izamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ixamin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ixamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ismin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ismin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn idmin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().idmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn iqmin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().iqmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn icmin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> blas_int {
    dyload_lib().icmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn izmin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> blas_int {
    dyload_lib().izmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn ixmin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> blas_int {
    dyload_lib().ixmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn samax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().samax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn damax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().damax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qamax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scamax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzamax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxamax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn samin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().samin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn damin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().damin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qamin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scamin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzamin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxamin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxamin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn smax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().smax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dmax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qmax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scmax_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzmax_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxmax_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxmax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn smin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().smin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dmin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qmin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scmin_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzmin_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dzmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxmin_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxmin_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn sscal_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut f32, arg4: *mut blas_int) {
    dyload_lib().sscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dscal_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut blas_int) {
    dyload_lib().dscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn qscal_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
) {
    dyload_lib().qscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn cscal_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut f32, arg4: *mut blas_int) {
    dyload_lib().cscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn zscal_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut blas_int) {
    dyload_lib().zscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn xscal_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
) {
    dyload_lib().xscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn csscal_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut f32, arg4: *mut blas_int) {
    dyload_lib().csscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn zdscal_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut blas_int) {
    dyload_lib().zdscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn xqscal_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
) {
    dyload_lib().xqscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn snrm2_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().snrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scnrm2_(arg1: *mut blas_int, arg2: *mut f32, arg3: *mut blas_int) -> f32 {
    dyload_lib().scnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dnrm2_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qnrm2_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dznrm2_(arg1: *mut blas_int, arg2: *mut f64, arg3: *mut blas_int) -> f64 {
    dyload_lib().dznrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn qxnrm2_(arg1: *mut blas_int, arg2: *mut xdouble, arg3: *mut blas_int) -> xdouble {
    dyload_lib().qxnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn srot_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
) {
    dyload_lib().srot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn drot_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
) {
    dyload_lib().drot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn qrot_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
) {
    dyload_lib().qrot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn csrot_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
) {
    dyload_lib().csrot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zdrot_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
) {
    dyload_lib().zdrot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xqrot_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
) {
    dyload_lib().xqrot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn srotg_(arg1: *mut f32, arg2: *mut f32, arg3: *mut f32, arg4: *mut f32) {
    dyload_lib().srotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn drotg_(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64) {
    dyload_lib().drotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn qrotg_(
    arg1: *mut xdouble,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
) {
    dyload_lib().qrotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn crotg_(arg1: *mut f32, arg2: *mut f32, arg3: *mut f32, arg4: *mut f32) {
    dyload_lib().crotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn zrotg_(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64) {
    dyload_lib().zrotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn xrotg_(
    arg1: *mut xdouble,
    arg2: *mut xdouble,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
) {
    dyload_lib().xrotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn srotmg_(
    arg1: *mut f32,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut f32,
) {
    dyload_lib().srotmg_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn drotmg_(
    arg1: *mut f64,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut f64,
) {
    dyload_lib().drotmg_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn srotm_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
) {
    dyload_lib().srotm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn drotm_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
) {
    dyload_lib().drotm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qrotm_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
) {
    dyload_lib().qrotm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sger_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().sger_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dger_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().dger_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qger_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().qger_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cgeru_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().cgeru_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cgerc_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().cgerc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zgeru_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zgeru_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zgerc_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zgerc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xgeru_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xgeru_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xgerc_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xgerc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn sbgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut bfloat16,
    arg6: *mut blas_int,
    arg7: *mut bfloat16,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().sbgemv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
    )
}

pub unsafe fn sgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().sgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().dgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn qgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().qgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn cgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().cgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().zgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn xgemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().xgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn strsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().strsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dtrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().dtrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn qtrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
) {
    dyload_lib().qtrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ctrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().ctrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ztrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().ztrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xtrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
) {
    dyload_lib().xtrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn strmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().strmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dtrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().dtrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn qtrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
) {
    dyload_lib().qtrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ctrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().ctrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ztrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().ztrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xtrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
) {
    dyload_lib().xtrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn stpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().stpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dtpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().dtpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn qtpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().qtpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ctpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().ctpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ztpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().ztpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xtpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().xtpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn stpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().stpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dtpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().dtpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn qtpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().qtpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ctpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().ctpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ztpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().ztpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xtpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().xtpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn stbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().stbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dtbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().dtbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qtbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().qtbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ctbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().ctbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ztbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().ztbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xtbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xtbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn stbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().stbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dtbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().dtbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qtbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().qtbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ctbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().ctbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ztbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().ztbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xtbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xtbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ssymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().ssymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn dsymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().dsymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn qsymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().qsymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn csymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().csymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zsymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().zsymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn xsymv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().xsymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn sspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().sspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().dspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().qspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().cspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xspmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ssyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().ssyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dsyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().dsyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn qsyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().qsyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn csyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().csyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zsyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().zsyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xsyr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().xsyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ssyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().ssyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dsyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().dsyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qsyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().qsyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn csyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().csyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zsyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zsyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xsyr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xsyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn sspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
) {
    dyload_lib().sspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
) {
    dyload_lib().dspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
) {
    dyload_lib().qspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
) {
    dyload_lib().cspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
) {
    dyload_lib().zspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xspr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
) {
    dyload_lib().xspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
) {
    dyload_lib().sspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
) {
    dyload_lib().dspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn qspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
) {
    dyload_lib().qspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
) {
    dyload_lib().cspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
) {
    dyload_lib().zspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xspr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
) {
    dyload_lib().xspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cher_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().cher_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zher_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().zher_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xher_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
) {
    dyload_lib().xher_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn chpr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
) {
    dyload_lib().chpr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zhpr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
) {
    dyload_lib().zhpr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xhpr_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
) {
    dyload_lib().xhpr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cher2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().cher2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zher2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zher2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xher2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xher2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn chpr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
) {
    dyload_lib().chpr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zhpr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
) {
    dyload_lib().zhpr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xhpr2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
) {
    dyload_lib().xhpr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn chemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().chemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zhemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().zhemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn xhemv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().xhemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn chpmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().chpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zhpmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zhpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xhpmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
) {
    dyload_lib().xhpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn snorm_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().snorm_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dnorm_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dnorm_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cnorm_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().cnorm_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn znorm_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().znorm_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().sgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().dgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn qgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) {
    dyload_lib().qgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().cgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().zgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn xgbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) {
    dyload_lib().xgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn ssbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().ssbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dsbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().dsbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn qsbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().qsbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn csbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().csbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zsbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().zsbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn xsbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().xsbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn chbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().chbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zhbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().zhbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn xhbmv_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut xdouble,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().xhbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn sbgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut bfloat16,
    arg8: *mut blas_int,
    arg9: *mut bfloat16,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().sbgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn sgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().sgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().dgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn qgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) {
    dyload_lib().qgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().cgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().zgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn xgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) {
    dyload_lib().xgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cgemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().cgemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zgemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().zgemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn xgemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) {
    dyload_lib().xgemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn sgemmt_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().sgemmt_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dgemmt_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().dgemmt_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cgemmt_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) {
    dyload_lib().cgemmt_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zgemmt_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) {
    dyload_lib().zgemmt_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn sge2mm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().sge2mm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dge2mm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().dge2mm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cge2mm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().cge2mm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zge2mm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().zge2mm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn strsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().strsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dtrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().dtrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn qtrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().qtrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ctrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().ctrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ztrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().ztrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn xtrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().xtrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn strmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().strmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dtrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().dtrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn qtrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().qtrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ctrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut blas_int,
) {
    dyload_lib().ctrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ztrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut blas_int,
) {
    dyload_lib().ztrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn xtrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut blas_int,
) {
    dyload_lib().xtrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ssymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().ssymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().dsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn qsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().qsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn csymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().csymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn csymm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().csymm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zsymm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zsymm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xsymm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xsymm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn ssyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().ssyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn dsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().dsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn qsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().qsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn csyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().csyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().zsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn xsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().xsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn ssyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().ssyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().dsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn qsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().qsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn csyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().csyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn chemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().chemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zhemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zhemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xhemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xhemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn chemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().chemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zhemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zhemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xhemm3m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xhemm3m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn cherk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
) {
    dyload_lib().cherk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zherk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
) {
    dyload_lib().zherk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn xherk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
) {
    dyload_lib().xherk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn cher2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
    arg10: *mut f32,
    arg11: *mut f32,
    arg12: *mut blas_int,
) {
    dyload_lib().cher2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zher2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
    arg10: *mut f64,
    arg11: *mut f64,
    arg12: *mut blas_int,
) {
    dyload_lib().zher2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn xher2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut xdouble,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut xdouble,
    arg9: *mut blas_int,
    arg10: *mut xdouble,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
) {
    dyload_lib().xher2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn cher2m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut f32,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().cher2m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zher2m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut f64,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().zher2m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn xher2m_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut xdouble,
    arg13: *mut blas_int,
) -> c_int {
    dyload_lib().xher2m_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn sgemt_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().sgemt_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dgemt_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().dgemt_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cgemt_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut f32,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().cgemt_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zgemt_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut f64,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().zgemt_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn sgema_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().sgema_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dgema_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().dgema_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn cgema_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().cgema_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zgema_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().zgema_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn sgems_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().sgems_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dgems_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().dgems_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn cgems_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().cgems_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zgems_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
) -> c_int {
    dyload_lib().zgems_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn sgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
    arg13: *mut f32,
    arg14: *mut f32,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().sgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn dgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
    arg13: *mut f64,
    arg14: *mut f64,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().dgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn qgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
    arg13: *mut xdouble,
    arg14: *mut xdouble,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().qgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn cgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut f32,
    arg10: *mut blas_int,
    arg11: *mut f32,
    arg12: *mut blas_int,
    arg13: *mut f32,
    arg14: *mut f32,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().cgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn zgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut f64,
    arg10: *mut blas_int,
    arg11: *mut f64,
    arg12: *mut blas_int,
    arg13: *mut f64,
    arg14: *mut f64,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().zgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn xgemc_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut xdouble,
    arg10: *mut blas_int,
    arg11: *mut xdouble,
    arg12: *mut blas_int,
    arg13: *mut xdouble,
    arg14: *mut xdouble,
    arg15: *mut blas_int,
) -> c_int {
    dyload_lib().xgemc_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15,
    )
}

pub unsafe fn sgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().sgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().dgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().qgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().cgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().zgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xgetf2_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().xgetf2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().sgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().dgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().qgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().cgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().zgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xgetrf_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().xgetrf_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn slaswp_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().slaswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dlaswp_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().dlaswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn qlaswp_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().qlaswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn claswp_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().claswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zlaswp_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().zlaswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn xlaswp_(
    arg1: *mut blas_int,
    arg2: *mut xdouble,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut blas_int,
) -> c_int {
    dyload_lib().xlaswp_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn sgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().sgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().dgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn qgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().qgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f32,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().cgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut f64,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().zgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn xgetrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
    arg7: *mut xdouble,
    arg8: *mut blas_int,
    arg9: *mut blas_int,
) -> c_int {
    dyload_lib().xgetrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn sgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().sgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().dgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn qgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().qgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().cgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().zgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xgesv_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().xgesv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn spotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().spotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dpotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dpotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qpotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().qpotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cpotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().cpotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zpotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().zpotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xpotf2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().xpotf2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn spotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().spotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dpotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dpotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qpotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().qpotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cpotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().cpotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zpotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().zpotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xpotrf_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().xpotrf_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn spotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().spotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dpotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dpotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qpotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().qpotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cpotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().cpotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zpotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().zpotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xpotri_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().xpotri_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn spotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().spotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dpotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().dpotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn qpotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().qpotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cpotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().cpotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zpotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().zpotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn xpotrs_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut xdouble,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) -> c_int {
    dyload_lib().xpotrs_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn slauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().slauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dlauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dlauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qlauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().qlauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn clauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().clauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zlauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().zlauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xlauu2_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().xlauu2_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn slauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().slauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dlauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().dlauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn qlauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().qlauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn clauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().clauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zlauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().zlauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn xlauum_(
    arg1: *mut c_char,
    arg2: *mut blas_int,
    arg3: *mut xdouble,
    arg4: *mut blas_int,
    arg5: *mut blas_int,
) -> c_int {
    dyload_lib().xlauum_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn strti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().strti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dtrti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().dtrti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qtrti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().qtrti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn ctrti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().ctrti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn ztrti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().ztrti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xtrti2_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().xtrti2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn strtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().strtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dtrtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().dtrtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn qtrtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().qtrtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn ctrtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().ctrtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn ztrtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().ztrtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn xtrtri_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut xdouble,
    arg5: *mut blas_int,
    arg6: *mut blas_int,
) -> c_int {
    dyload_lib().xtrtri_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn slamch_(arg1: *mut c_char) -> f32 {
    dyload_lib().slamch_.unwrap()(arg1)
}

pub unsafe fn dlamch_(arg1: *mut c_char) -> f64 {
    dyload_lib().dlamch_.unwrap()(arg1)
}

pub unsafe fn qlamch_(arg1: *mut c_char) -> xdouble {
    dyload_lib().qlamch_.unwrap()(arg1)
}

pub unsafe fn slamc3_(arg1: *mut f32, arg2: *mut f32) -> f32 {
    dyload_lib().slamc3_.unwrap()(arg1, arg2)
}

pub unsafe fn dlamc3_(arg1: *mut f64, arg2: *mut f64) -> f64 {
    dyload_lib().dlamc3_.unwrap()(arg1, arg2)
}

pub unsafe fn qlamc3_(arg1: *mut xdouble, arg2: *mut xdouble) -> xdouble {
    dyload_lib().qlamc3_.unwrap()(arg1, arg2)
}

pub unsafe fn saxpby_(
    arg1: *mut blas_int,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().saxpby_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn daxpby_(
    arg1: *mut blas_int,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().daxpby_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn caxpby_(
    arg1: *mut blas_int,
    arg2: *mut c_void,
    arg3: *mut f32,
    arg4: *mut blas_int,
    arg5: *mut c_void,
    arg6: *mut f32,
    arg7: *mut blas_int,
) {
    dyload_lib().caxpby_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zaxpby_(
    arg1: *mut blas_int,
    arg2: *mut c_void,
    arg3: *mut f64,
    arg4: *mut blas_int,
    arg5: *mut c_void,
    arg6: *mut f64,
    arg7: *mut blas_int,
) {
    dyload_lib().zaxpby_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn somatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().somatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn domatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().domatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn comatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut f32,
    arg9: *mut blas_int,
) {
    dyload_lib().comatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zomatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut f64,
    arg9: *mut blas_int,
) {
    dyload_lib().zomatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn simatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) {
    dyload_lib().simatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dimatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) {
    dyload_lib().dimatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cimatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f32,
    arg6: *mut f32,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) {
    dyload_lib().cimatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zimatcopy_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut blas_int,
    arg4: *mut blas_int,
    arg5: *mut f64,
    arg6: *mut f64,
    arg7: *mut blas_int,
    arg8: *mut blas_int,
) {
    dyload_lib().zimatcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn sgeadd_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().sgeadd_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dgeadd_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().dgeadd_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn cgeadd_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f32,
    arg4: *mut f32,
    arg5: *mut blas_int,
    arg6: *mut f32,
    arg7: *mut f32,
    arg8: *mut blas_int,
) {
    dyload_lib().cgeadd_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zgeadd_(
    arg1: *mut blas_int,
    arg2: *mut blas_int,
    arg3: *mut f64,
    arg4: *mut f64,
    arg5: *mut blas_int,
    arg6: *mut f64,
    arg7: *mut f64,
    arg8: *mut blas_int,
) {
    dyload_lib().zgeadd_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}
