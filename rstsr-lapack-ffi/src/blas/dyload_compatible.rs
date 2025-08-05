//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn xerbla_(arg1: *mut c_char, arg2: *mut c_void) {
    dyload_lib().xerbla_.unwrap()(arg1, arg2)
}

pub unsafe fn srot_(
    arg1: *const blas_int,
    arg2: *mut f32,
    arg3: *const blas_int,
    arg4: *mut f32,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const f32,
) {
    dyload_lib().srot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn srotg_(arg1: *mut f32, arg2: *mut f32, arg3: *mut f32, arg4: *mut f32) {
    dyload_lib().srotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn srotm_(
    arg1: *const blas_int,
    arg2: *mut f32,
    arg3: *const blas_int,
    arg4: *mut f32,
    arg5: *const blas_int,
    arg6: *const f32,
) {
    dyload_lib().srotm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn srotmg_(
    arg1: *mut f32,
    arg2: *mut f32,
    arg3: *mut f32,
    arg4: *const f32,
    arg5: *mut f32,
) {
    dyload_lib().srotmg_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sswap_(
    arg1: *const blas_int,
    arg2: *mut f32,
    arg3: *const blas_int,
    arg4: *mut f32,
    arg5: *const blas_int,
) {
    dyload_lib().sswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn scopy_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *mut f32,
    arg5: *const blas_int,
) {
    dyload_lib().scopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn saxpy_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const f32,
    arg4: *const blas_int,
    arg5: *mut f32,
    arg6: *const blas_int,
) {
    dyload_lib().saxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sdotsub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *mut f32,
) {
    dyload_lib().sdotsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sdsdotsub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const f32,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *mut f32,
) {
    dyload_lib().sdsdotsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn sscal_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *mut f32,
    arg4: *const blas_int,
) {
    dyload_lib().sscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn snrm2sub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *mut f32,
) {
    dyload_lib().snrm2sub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn sasumsub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *mut f32,
) {
    dyload_lib().sasumsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn isamaxsub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *mut blas_int,
) {
    dyload_lib().isamaxsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn drot_(
    arg1: *const blas_int,
    arg2: *mut f64,
    arg3: *const blas_int,
    arg4: *mut f64,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const f64,
) {
    dyload_lib().drot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn drotg_(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64) {
    dyload_lib().drotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn drotm_(
    arg1: *const blas_int,
    arg2: *mut f64,
    arg3: *const blas_int,
    arg4: *mut f64,
    arg5: *const blas_int,
    arg6: *const f64,
) {
    dyload_lib().drotm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn drotmg_(
    arg1: *mut f64,
    arg2: *mut f64,
    arg3: *mut f64,
    arg4: *const f64,
    arg5: *mut f64,
) {
    dyload_lib().drotmg_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dswap_(
    arg1: *const blas_int,
    arg2: *mut f64,
    arg3: *const blas_int,
    arg4: *mut f64,
    arg5: *const blas_int,
) {
    dyload_lib().dswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dcopy_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *mut f64,
    arg5: *const blas_int,
) {
    dyload_lib().dcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn daxpy_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const f64,
    arg4: *const blas_int,
    arg5: *mut f64,
    arg6: *const blas_int,
) {
    dyload_lib().daxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dsdotsub_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *mut f64,
) {
    dyload_lib().dsdotsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn ddotsub_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *mut f64,
) {
    dyload_lib().ddotsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dscal_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *mut f64,
    arg4: *const blas_int,
) {
    dyload_lib().dscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dnrm2sub_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *mut f64,
) {
    dyload_lib().dnrm2sub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dasumsub_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *mut f64,
) {
    dyload_lib().dasumsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn idamaxsub_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *mut blas_int,
) {
    dyload_lib().idamaxsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn crotg_(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut f32, arg4: *mut c_void) {
    dyload_lib().crotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn csrot_(
    arg1: *const blas_int,
    X: *mut c_void,
    arg2: *const blas_int,
    arg3: *mut c_void,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const f32,
) {
    dyload_lib().csrot_.unwrap()(arg1, X, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cswap_(
    arg1: *const blas_int,
    arg2: *mut c_void,
    arg3: *const blas_int,
    arg4: *mut c_void,
    arg5: *const blas_int,
) {
    dyload_lib().cswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn ccopy_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut c_void,
    arg5: *const blas_int,
) {
    dyload_lib().ccopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn caxpy_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const c_void,
    arg4: *const blas_int,
    arg5: *mut c_void,
    arg6: *const blas_int,
) {
    dyload_lib().caxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cdotcsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().cdotcsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cdotusub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().cdotusub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn cscal_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *mut c_void,
    arg4: *const blas_int,
) {
    dyload_lib().cscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn icamaxsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut blas_int,
) {
    dyload_lib().icamaxsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn csscal_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *mut c_void,
    arg4: *const blas_int,
) {
    dyload_lib().csscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn scnrm2sub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut f32,
) {
    dyload_lib().scnrm2sub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn scasumsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut f32,
) {
    dyload_lib().scasumsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn scabs1sub_(arg1: *const c_void, arg2: *mut f32) {
    dyload_lib().scabs1sub_.unwrap()(arg1, arg2)
}

pub unsafe fn zrotg_(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut f64, arg4: *mut c_void) {
    dyload_lib().zrotg_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn zdrot_(
    arg1: *const blas_int,
    X: *mut c_void,
    arg2: *const blas_int,
    arg3: *mut c_void,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const f64,
) {
    dyload_lib().zdrot_.unwrap()(arg1, X, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zswap_(
    arg1: *const blas_int,
    arg2: *mut c_void,
    arg3: *const blas_int,
    arg4: *mut c_void,
    arg5: *const blas_int,
) {
    dyload_lib().zswap_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zcopy_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut c_void,
    arg5: *const blas_int,
) {
    dyload_lib().zcopy_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zaxpy_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const c_void,
    arg4: *const blas_int,
    arg5: *mut c_void,
    arg6: *const blas_int,
) {
    dyload_lib().zaxpy_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zdotcsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().zdotcsub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zdotusub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().zdotusub_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zdscal_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *mut c_void,
    arg4: *const blas_int,
) {
    dyload_lib().zdscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn zscal_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *mut c_void,
    arg4: *const blas_int,
) {
    dyload_lib().zscal_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dznrm2sub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut f64,
) {
    dyload_lib().dznrm2sub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dzasumsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut f64,
) {
    dyload_lib().dzasumsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn izamaxsub_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *mut blas_int,
) {
    dyload_lib().izamaxsub_.unwrap()(arg1, arg2, arg3, arg4)
}

pub unsafe fn dcabs1sub_(arg1: *const c_void, arg2: *mut f64) {
    dyload_lib().dcabs1sub_.unwrap()(arg1, arg2)
}

pub unsafe fn sgemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *const f32,
    arg8: *const blas_int,
    arg9: *const f32,
    arg10: *mut f32,
    arg11: *const blas_int,
) {
    dyload_lib().sgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn sgbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const f32,
    arg8: *const blas_int,
    arg9: *const f32,
    arg10: *const blas_int,
    arg11: *const f32,
    arg12: *mut f32,
    arg13: *const blas_int,
) {
    dyload_lib().sgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn ssymv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *const f32,
    arg9: *mut f32,
    arg10: *const blas_int,
) {
    dyload_lib().ssymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn ssbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *const f32,
    arg8: *const blas_int,
    arg9: *const f32,
    arg10: *mut f32,
    arg11: *const blas_int,
) {
    dyload_lib().ssbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn sspmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *const f32,
    arg8: *mut f32,
    arg9: *const blas_int,
) {
    dyload_lib().sspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn strmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *mut f32,
    arg8: *const blas_int,
) {
    dyload_lib().strmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn stbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *mut f32,
    arg9: *const blas_int,
) {
    dyload_lib().stbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn strsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const blas_int,
    arg7: *mut f32,
    arg8: *const blas_int,
) {
    dyload_lib().strsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn stbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *mut f32,
    arg9: *const blas_int,
) {
    dyload_lib().stbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn stpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *mut f32,
    arg7: *const blas_int,
) {
    dyload_lib().stpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn stpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *mut f32,
    arg7: *const blas_int,
) {
    dyload_lib().stpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn sger_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *mut f32,
    arg9: *const blas_int,
) {
    dyload_lib().sger_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ssyr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *mut f32,
    arg7: *const blas_int,
) {
    dyload_lib().ssyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn sspr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *mut f32,
) {
    dyload_lib().sspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn sspr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *mut f32,
) {
    dyload_lib().sspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ssyr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const f32,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *mut f32,
    arg9: *const blas_int,
) {
    dyload_lib().ssyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dgemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const f64,
    arg5: *const f64,
    arg6: *const blas_int,
    arg7: *const f64,
    arg8: *const blas_int,
    arg9: *const f64,
    arg10: *mut f64,
    arg11: *const blas_int,
) {
    dyload_lib().dgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dgbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const f64,
    arg8: *const blas_int,
    arg9: *const f64,
    arg10: *const blas_int,
    arg11: *const f64,
    arg12: *mut f64,
    arg13: *const blas_int,
) {
    dyload_lib().dgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dsymv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *const f64,
    arg9: *mut f64,
    arg10: *const blas_int,
) {
    dyload_lib().dsymv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn dsbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const f64,
    arg5: *const f64,
    arg6: *const blas_int,
    arg7: *const f64,
    arg8: *const blas_int,
    arg9: *const f64,
    arg10: *mut f64,
    arg11: *const blas_int,
) {
    dyload_lib().dsbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dspmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const f64,
    arg6: *const blas_int,
    arg7: *const f64,
    arg8: *mut f64,
    arg9: *const blas_int,
) {
    dyload_lib().dspmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dtrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const blas_int,
    arg7: *mut f64,
    arg8: *const blas_int,
) {
    dyload_lib().dtrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dtbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *mut f64,
    arg9: *const blas_int,
) {
    dyload_lib().dtbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dtrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const blas_int,
    arg7: *mut f64,
    arg8: *const blas_int,
) {
    dyload_lib().dtrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dtbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *mut f64,
    arg9: *const blas_int,
) {
    dyload_lib().dtbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dtpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *mut f64,
    arg7: *const blas_int,
) {
    dyload_lib().dtpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dtpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *mut f64,
    arg7: *const blas_int,
) {
    dyload_lib().dtpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dger_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *mut f64,
    arg9: *const blas_int,
) {
    dyload_lib().dger_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn dsyr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *mut f64,
    arg7: *const blas_int,
) {
    dyload_lib().dsyr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn dspr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *mut f64,
) {
    dyload_lib().dspr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn dspr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *mut f64,
) {
    dyload_lib().dspr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn dsyr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const f64,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *mut f64,
    arg9: *const blas_int,
) {
    dyload_lib().dsyr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cgemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().cgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn cgbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().cgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn chemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().chemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn chbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().chbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn chpmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().chpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ctrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *mut c_void,
    arg8: *const blas_int,
) {
    dyload_lib().ctrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ctbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().ctbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ctpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().ctpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ctrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *mut c_void,
    arg8: *const blas_int,
) {
    dyload_lib().ctrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ctbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().ctbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ctpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().ctpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn cgerc_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().cgerc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cgeru_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().cgeru_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn cher_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().cher_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn cher2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().cher2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn chpr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f32,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().chpr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn chpr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
) {
    dyload_lib().chpr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn zgemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().zgemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zgbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().zgbmv_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zhemv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().zhemv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zhbmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().zhbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zhpmv_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().zhpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ztrmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *mut c_void,
    arg8: *const blas_int,
) {
    dyload_lib().ztrmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ztbmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().ztbmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ztpmv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().ztpmv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn ztrsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const blas_int,
    arg7: *mut c_void,
    arg8: *const blas_int,
) {
    dyload_lib().ztrsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn ztbsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().ztbsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn ztpsv_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().ztpsv_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zgerc_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().zgerc_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zgeru_(
    arg1: *const blas_int,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().zgeru_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zher_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
    arg7: *const blas_int,
) {
    dyload_lib().zher_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}

pub unsafe fn zher2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
    arg9: *const blas_int,
) {
    dyload_lib().zher2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}

pub unsafe fn zhpr_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const f64,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *mut c_void,
) {
    dyload_lib().zhpr_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn zhpr2_(
    arg1: *mut c_char,
    arg2: *const blas_int,
    arg3: *const c_void,
    arg4: *const c_void,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *mut c_void,
) {
    dyload_lib().zhpr2_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
}

pub unsafe fn sgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const f32,
    arg8: *const blas_int,
    arg9: *const f32,
    arg10: *const blas_int,
    arg11: *const f32,
    arg12: *mut f32,
    arg13: *const blas_int,
) {
    dyload_lib().sgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn sgemmtr_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f32,
    arg7: *const f32,
    arg8: *const blas_int,
    arg9: *const f32,
    arg10: *const blas_int,
    arg11: *const f32,
    arg12: *mut f32,
    arg13: *const blas_int,
) {
    dyload_lib().sgemmtr_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn ssymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *const f32,
    arg9: *const blas_int,
    arg10: *const f32,
    arg11: *mut f32,
    arg12: *const blas_int,
) {
    dyload_lib().ssymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn ssyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *const f32,
    arg9: *mut f32,
    arg10: *const blas_int,
) {
    dyload_lib().ssyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn ssyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const f32,
    arg7: *const blas_int,
    arg8: *const f32,
    arg9: *const blas_int,
    arg10: *const f32,
    arg11: *mut f32,
    arg12: *const blas_int,
) {
    dyload_lib().ssyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn strmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const f32,
    arg8: *const f32,
    arg9: *const blas_int,
    arg10: *mut f32,
    arg11: *const blas_int,
) {
    dyload_lib().strmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn strsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const f32,
    arg8: *const f32,
    arg9: *const blas_int,
    arg10: *mut f32,
    arg11: *const blas_int,
) {
    dyload_lib().strsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const f64,
    arg8: *const blas_int,
    arg9: *const f64,
    arg10: *const blas_int,
    arg11: *const f64,
    arg12: *mut f64,
    arg13: *const blas_int,
) {
    dyload_lib().dgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dgemmtr_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const f64,
    arg7: *const f64,
    arg8: *const blas_int,
    arg9: *const f64,
    arg10: *const blas_int,
    arg11: *const f64,
    arg12: *mut f64,
    arg13: *const blas_int,
) {
    dyload_lib().dgemmtr_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn dsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *const f64,
    arg9: *const blas_int,
    arg10: *const f64,
    arg11: *mut f64,
    arg12: *const blas_int,
) {
    dyload_lib().dsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *const f64,
    arg9: *mut f64,
    arg10: *const blas_int,
) {
    dyload_lib().dsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn dsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const f64,
    arg7: *const blas_int,
    arg8: *const f64,
    arg9: *const blas_int,
    arg10: *const f64,
    arg11: *mut f64,
    arg12: *const blas_int,
) {
    dyload_lib().dsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn dtrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const f64,
    arg8: *const f64,
    arg9: *const blas_int,
    arg10: *mut f64,
    arg11: *const blas_int,
) {
    dyload_lib().dtrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn dtrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const f64,
    arg8: *const f64,
    arg9: *const blas_int,
    arg10: *mut f64,
    arg11: *const blas_int,
) {
    dyload_lib().dtrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn cgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().cgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn cgemmtr_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().cgemmtr_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn csymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().csymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn chemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().chemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn csyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().csyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn cherk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const f32,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().cherk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn csyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().csyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn cher2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const f32,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().cher2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn ctrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().ctrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ctrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().ctrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn zgemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().zgemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zgemmtr_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *const blas_int,
    arg5: *const blas_int,
    arg6: *const c_void,
    arg7: *const c_void,
    arg8: *const blas_int,
    arg9: *const c_void,
    arg10: *const blas_int,
    arg11: *const c_void,
    arg12: *mut c_void,
    arg13: *const blas_int,
) {
    dyload_lib().zgemmtr_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
    )
}

pub unsafe fn zsymm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().zsymm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zhemm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().zhemm_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zsyrk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().zsyrk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zherk_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const f64,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const f64,
    arg9: *mut c_void,
    arg10: *const blas_int,
) {
    dyload_lib().zherk_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub unsafe fn zsyr2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const c_void,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().zsyr2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn zher2k_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *const blas_int,
    arg4: *const blas_int,
    arg5: *const c_void,
    arg6: *const c_void,
    arg7: *const blas_int,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *const f64,
    arg11: *mut c_void,
    arg12: *const blas_int,
) {
    dyload_lib().zher2k_.unwrap()(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
    )
}

pub unsafe fn ztrmm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().ztrmm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn ztrsm_(
    arg1: *mut c_char,
    arg2: *mut c_char,
    arg3: *mut c_char,
    arg4: *mut c_char,
    arg5: *const blas_int,
    arg6: *const blas_int,
    arg7: *const c_void,
    arg8: *const c_void,
    arg9: *const blas_int,
    arg10: *mut c_void,
    arg11: *const blas_int,
) {
    dyload_lib().ztrsm_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11)
}

pub unsafe fn sdot_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const blas_int,
) -> f32 {
    dyload_lib().sdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn sdsdot_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const f32,
    arg4: *const blas_int,
    arg5: *const f32,
    arg6: *const blas_int,
) -> f32 {
    dyload_lib().sdsdot_.unwrap()(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub unsafe fn snrm2_(arg1: *const blas_int, arg2: *const f32, arg3: *const blas_int) -> f32 {
    dyload_lib().snrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn sasum_(arg1: *const blas_int, arg2: *const f32, arg3: *const blas_int) -> f32 {
    dyload_lib().sasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn isamax_(arg1: *const blas_int, arg2: *const f32, arg3: *const blas_int) -> blas_int {
    dyload_lib().isamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dsdot_(
    arg1: *const blas_int,
    arg2: *const f32,
    arg3: *const blas_int,
    arg4: *const f32,
    arg5: *const blas_int,
) -> f64 {
    dyload_lib().dsdot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn ddot_(
    arg1: *const blas_int,
    arg2: *const f64,
    arg3: *const blas_int,
    arg4: *const f64,
    arg5: *const blas_int,
) -> f64 {
    dyload_lib().ddot_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dnrm2_(arg1: *const blas_int, arg2: *const f64, arg3: *const blas_int) -> f64 {
    dyload_lib().dnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dasum_(arg1: *const blas_int, arg2: *const f64, arg3: *const blas_int) -> f64 {
    dyload_lib().dasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn idamax_(arg1: *const blas_int, arg2: *const f64, arg3: *const blas_int) -> blas_int {
    dyload_lib().idamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn cdotc_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
) -> c_void {
    dyload_lib().cdotc_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn cdotu_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
) -> c_void {
    dyload_lib().cdotu_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn icamax_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
) -> blas_int {
    dyload_lib().icamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scnrm2_(arg1: *const blas_int, arg2: *const c_void, arg3: *const blas_int) -> f32 {
    dyload_lib().scnrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scasum_(arg1: *const blas_int, arg2: *const c_void, arg3: *const blas_int) -> f32 {
    dyload_lib().scasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn scabs1_(arg1: *const c_void) -> f32 {
    dyload_lib().scabs1_.unwrap()(arg1)
}

pub unsafe fn zdotc_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
) -> c_void {
    dyload_lib().zdotc_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn zdotu_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
    arg4: *const c_void,
    arg5: *const blas_int,
) -> c_void {
    dyload_lib().zdotu_.unwrap()(arg1, arg2, arg3, arg4, arg5)
}

pub unsafe fn dznrm2_(arg1: *const blas_int, arg2: *const c_void, arg3: *const blas_int) -> f64 {
    dyload_lib().dznrm2_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dzasum_(arg1: *const blas_int, arg2: *const c_void, arg3: *const blas_int) -> f64 {
    dyload_lib().dzasum_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn izamax_(
    arg1: *const blas_int,
    arg2: *const c_void,
    arg3: *const blas_int,
) -> blas_int {
    dyload_lib().izamax_.unwrap()(arg1, arg2, arg3)
}

pub unsafe fn dcabs1_(arg1: *const c_void) -> f64 {
    dyload_lib().dcabs1_.unwrap()(arg1)
}
