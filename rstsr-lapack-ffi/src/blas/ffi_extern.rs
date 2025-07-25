//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn xerbla_(arg1: *mut c_char, arg2: *mut c_void);
    pub fn srot_(
        arg1: *const blas_int,
        arg2: *mut f32,
        arg3: *const blas_int,
        arg4: *mut f32,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const f32,
    );
    pub fn srotg_(arg1: *mut f32, arg2: *mut f32, arg3: *mut f32, arg4: *mut f32);
    pub fn srotm_(
        arg1: *const blas_int,
        arg2: *mut f32,
        arg3: *const blas_int,
        arg4: *mut f32,
        arg5: *const blas_int,
        arg6: *const f32,
    );
    pub fn srotmg_(
        arg1: *mut f32,
        arg2: *mut f32,
        arg3: *mut f32,
        arg4: *const f32,
        arg5: *mut f32,
    );
    pub fn sswap_(
        arg1: *const blas_int,
        arg2: *mut f32,
        arg3: *const blas_int,
        arg4: *mut f32,
        arg5: *const blas_int,
    );
    pub fn scopy_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *mut f32,
        arg5: *const blas_int,
    );
    pub fn saxpy_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const f32,
        arg4: *const blas_int,
        arg5: *mut f32,
        arg6: *const blas_int,
    );
    pub fn sdotsub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *mut f32,
    );
    pub fn sdsdotsub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const f32,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *const blas_int,
        arg7: *mut f32,
    );
    pub fn sscal_(arg1: *const blas_int, arg2: *const f32, arg3: *mut f32, arg4: *const blas_int);
    pub fn snrm2sub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *mut f32,
    );
    pub fn sasumsub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *mut f32,
    );
    pub fn isamaxsub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *mut blas_int,
    );
    pub fn drot_(
        arg1: *const blas_int,
        arg2: *mut f64,
        arg3: *const blas_int,
        arg4: *mut f64,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const f64,
    );
    pub fn drotg_(arg1: *mut f64, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
    pub fn drotm_(
        arg1: *const blas_int,
        arg2: *mut f64,
        arg3: *const blas_int,
        arg4: *mut f64,
        arg5: *const blas_int,
        arg6: *const f64,
    );
    pub fn drotmg_(
        arg1: *mut f64,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *const f64,
        arg5: *mut f64,
    );
    pub fn dswap_(
        arg1: *const blas_int,
        arg2: *mut f64,
        arg3: *const blas_int,
        arg4: *mut f64,
        arg5: *const blas_int,
    );
    pub fn dcopy_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const blas_int,
        arg4: *mut f64,
        arg5: *const blas_int,
    );
    pub fn daxpy_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const f64,
        arg4: *const blas_int,
        arg5: *mut f64,
        arg6: *const blas_int,
    );
    pub fn dsdotsub_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *const blas_int,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *mut f64,
    );
    pub fn ddotsub_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const blas_int,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *mut f64,
    );
    pub fn dscal_(arg1: *const blas_int, arg2: *const f64, arg3: *mut f64, arg4: *const blas_int);
    pub fn dnrm2sub_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const blas_int,
        arg4: *mut f64,
    );
    pub fn dasumsub_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const blas_int,
        arg4: *mut f64,
    );
    pub fn idamaxsub_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *const blas_int,
        arg4: *mut blas_int,
    );
    pub fn crotg_(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut f32, arg4: *mut c_void);
    pub fn csrot_(
        arg1: *const blas_int,
        X: *mut c_void,
        arg2: *const blas_int,
        arg3: *mut c_void,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *const f32,
    );
    pub fn cswap_(
        arg1: *const blas_int,
        arg2: *mut c_void,
        arg3: *const blas_int,
        arg4: *mut c_void,
        arg5: *const blas_int,
    );
    pub fn ccopy_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut c_void,
        arg5: *const blas_int,
    );
    pub fn caxpy_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const c_void,
        arg4: *const blas_int,
        arg5: *mut c_void,
        arg6: *const blas_int,
    );
    pub fn cdotcsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn cdotusub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn cscal_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *mut c_void,
        arg4: *const blas_int,
    );
    pub fn icamaxsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut blas_int,
    );
    pub fn csscal_(
        arg1: *const blas_int,
        arg2: *const f32,
        arg3: *mut c_void,
        arg4: *const blas_int,
    );
    pub fn scnrm2sub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut f32,
    );
    pub fn scasumsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut f32,
    );
    pub fn scabs1sub_(arg1: *const c_void, arg2: *mut f32);
    pub fn zrotg_(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut f64, arg4: *mut c_void);
    pub fn zdrot_(
        arg1: *const blas_int,
        X: *mut c_void,
        arg2: *const blas_int,
        arg3: *mut c_void,
        arg4: *const blas_int,
        arg5: *const f64,
        arg6: *const f64,
    );
    pub fn zswap_(
        arg1: *const blas_int,
        arg2: *mut c_void,
        arg3: *const blas_int,
        arg4: *mut c_void,
        arg5: *const blas_int,
    );
    pub fn zcopy_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut c_void,
        arg5: *const blas_int,
    );
    pub fn zaxpy_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const c_void,
        arg4: *const blas_int,
        arg5: *mut c_void,
        arg6: *const blas_int,
    );
    pub fn zdotcsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn zdotusub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn zdscal_(
        arg1: *const blas_int,
        arg2: *const f64,
        arg3: *mut c_void,
        arg4: *const blas_int,
    );
    pub fn zscal_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *mut c_void,
        arg4: *const blas_int,
    );
    pub fn dznrm2sub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut f64,
    );
    pub fn dzasumsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut f64,
    );
    pub fn izamaxsub_(
        arg1: *const blas_int,
        arg2: *const c_void,
        arg3: *const blas_int,
        arg4: *mut blas_int,
    );
    pub fn dcabs1sub_(arg1: *const c_void, arg2: *mut f64);
    pub fn sgemv_(
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
    );
    pub fn sgbmv_(
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
    );
    pub fn ssymv_(
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
    );
    pub fn ssbmv_(
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
    );
    pub fn sspmv_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const f32,
        arg6: *const blas_int,
        arg7: *const f32,
        arg8: *mut f32,
        arg9: *const blas_int,
    );
    pub fn strmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *const blas_int,
        arg7: *mut f32,
        arg8: *const blas_int,
    );
    pub fn stbmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const blas_int,
        arg8: *mut f32,
        arg9: *const blas_int,
    );
    pub fn strsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *const blas_int,
        arg7: *mut f32,
        arg8: *const blas_int,
    );
    pub fn stbsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const blas_int,
        arg8: *mut f32,
        arg9: *const blas_int,
    );
    pub fn stpmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *mut f32,
        arg7: *const blas_int,
    );
    pub fn stpsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f32,
        arg6: *mut f32,
        arg7: *const blas_int,
    );
    pub fn sger_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const blas_int,
        arg8: *mut f32,
        arg9: *const blas_int,
    );
    pub fn ssyr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *mut f32,
        arg7: *const blas_int,
    );
    pub fn sspr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *mut f32,
    );
    pub fn sspr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const blas_int,
        arg8: *mut f32,
    );
    pub fn ssyr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const f32,
        arg5: *const blas_int,
        arg6: *const f32,
        arg7: *const blas_int,
        arg8: *mut f32,
        arg9: *const blas_int,
    );
    pub fn dgemv_(
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
    );
    pub fn dgbmv_(
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
    );
    pub fn dsymv_(
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
    );
    pub fn dsbmv_(
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
    );
    pub fn dspmv_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const f64,
        arg6: *const blas_int,
        arg7: *const f64,
        arg8: *mut f64,
        arg9: *const blas_int,
    );
    pub fn dtrmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f64,
        arg6: *const blas_int,
        arg7: *mut f64,
        arg8: *const blas_int,
    );
    pub fn dtbmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const blas_int,
        arg8: *mut f64,
        arg9: *const blas_int,
    );
    pub fn dtrsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f64,
        arg6: *const blas_int,
        arg7: *mut f64,
        arg8: *const blas_int,
    );
    pub fn dtbsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const blas_int,
        arg8: *mut f64,
        arg9: *const blas_int,
    );
    pub fn dtpmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f64,
        arg6: *mut f64,
        arg7: *const blas_int,
    );
    pub fn dtpsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const f64,
        arg6: *mut f64,
        arg7: *const blas_int,
    );
    pub fn dger_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const blas_int,
        arg8: *mut f64,
        arg9: *const blas_int,
    );
    pub fn dsyr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *mut f64,
        arg7: *const blas_int,
    );
    pub fn dspr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *mut f64,
    );
    pub fn dspr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const blas_int,
        arg8: *mut f64,
    );
    pub fn dsyr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const f64,
        arg5: *const blas_int,
        arg6: *const f64,
        arg7: *const blas_int,
        arg8: *mut f64,
        arg9: *const blas_int,
    );
    pub fn cgemv_(
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
    );
    pub fn cgbmv_(
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
    );
    pub fn chemv_(
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
    );
    pub fn chbmv_(
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
    );
    pub fn chpmv_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *const c_void,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ctrmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *mut c_void,
        arg8: *const blas_int,
    );
    pub fn ctbmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ctpmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn ctrsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *mut c_void,
        arg8: *const blas_int,
    );
    pub fn ctbsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ctpsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn cgerc_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn cgeru_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn cher_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn cher2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn chpr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f32,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn chpr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
    );
    pub fn zgemv_(
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
    );
    pub fn zgbmv_(
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
    );
    pub fn zhemv_(
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
    );
    pub fn zhbmv_(
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
    );
    pub fn zhpmv_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *const c_void,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ztrmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *mut c_void,
        arg8: *const blas_int,
    );
    pub fn ztbmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ztpmv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn ztrsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *const blas_int,
        arg7: *mut c_void,
        arg8: *const blas_int,
    );
    pub fn ztbsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn ztpsv_(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *mut c_char,
        arg4: *const blas_int,
        arg5: *const c_void,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn zgerc_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn zgeru_(
        arg1: *const blas_int,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn zher_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
        arg7: *const blas_int,
    );
    pub fn zher2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
        arg9: *const blas_int,
    );
    pub fn zhpr_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const f64,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *mut c_void,
    );
    pub fn zhpr2_(
        arg1: *mut c_char,
        arg2: *const blas_int,
        arg3: *const c_void,
        arg4: *const c_void,
        arg5: *const blas_int,
        arg6: *const c_void,
        arg7: *const blas_int,
        arg8: *mut c_void,
    );
    pub fn sgemm_(
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
    );
    pub fn sgemmtr_(
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
    );
    pub fn ssymm_(
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
    );
    pub fn ssyrk_(
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
    );
    pub fn ssyr2k_(
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
    );
    pub fn strmm_(
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
    );
    pub fn strsm_(
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
    );
    pub fn dgemm_(
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
    );
    pub fn dgemmtr_(
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
    );
    pub fn dsymm_(
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
    );
    pub fn dsyrk_(
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
    );
    pub fn dsyr2k_(
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
    );
    pub fn dtrmm_(
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
    );
    pub fn dtrsm_(
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
    );
    pub fn cgemm_(
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
    );
    pub fn cgemmtr_(
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
    );
    pub fn csymm_(
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
    );
    pub fn chemm_(
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
    );
    pub fn csyrk_(
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
    );
    pub fn cherk_(
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
    );
    pub fn csyr2k_(
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
    );
    pub fn cher2k_(
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
    );
    pub fn ctrmm_(
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
    );
    pub fn ctrsm_(
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
    );
    pub fn zgemm_(
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
    );
    pub fn zgemmtr_(
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
    );
    pub fn zsymm_(
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
    );
    pub fn zhemm_(
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
    );
    pub fn zsyrk_(
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
    );
    pub fn zherk_(
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
    );
    pub fn zsyr2k_(
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
    );
    pub fn zher2k_(
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
    );
    pub fn ztrmm_(
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
    );
    pub fn ztrsm_(
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
    );
}
