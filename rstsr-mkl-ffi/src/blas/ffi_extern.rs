//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn xerbla_(srname: *const c_char, info: *const c_int, lsrname: c_int);
    pub fn lsame_(ca: *const c_char, cb: *const c_char, lca: MKL_INT, lcb: MKL_INT) -> c_int;
    pub fn scabs1_(c: *const MKL_Complex8) -> f32;
    pub fn sasum_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32;
    pub fn saxpy_(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn saxpby_(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn saxpyi_(
        nz: *const MKL_INT,
        a: *const f32,
        x: *const f32,
        indx: *const MKL_INT,
        y: *mut f32,
    );
    pub fn scasum_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32;
    pub fn scnrm2_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32;
    pub fn scopy_(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn sdot_(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f32;
    pub fn sdoti_(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *const f32) -> f32;
    pub fn sdsdot_(
        n: *const MKL_INT,
        sb: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f32;
    pub fn sgthr_(nz: *const MKL_INT, y: *const f32, x: *mut f32, indx: *const MKL_INT);
    pub fn sgthrz_(nz: *const MKL_INT, y: *mut f32, x: *mut f32, indx: *const MKL_INT);
    pub fn snrm2_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32;
    pub fn srot_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const f32,
    );
    pub fn srotg_(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32);
    pub fn sroti_(
        nz: *const MKL_INT,
        x: *mut f32,
        indx: *const MKL_INT,
        y: *mut f32,
        c: *const f32,
        s: *const f32,
    );
    pub fn srotm_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        param: *const f32,
    );
    pub fn srotmg_(d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32);
    pub fn sscal_(n: *const MKL_INT, a: *const f32, x: *mut f32, incx: *const MKL_INT);
    pub fn ssctr_(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *mut f32);
    pub fn sswap_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn isamax_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT;
    pub fn isamin_(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT;
    pub fn caxpy_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn caxpby_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn caxpyi_(
        nz: *const MKL_INT,
        a: *const MKL_Complex8,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *mut MKL_Complex8,
    );
    pub fn ccopy_(
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotc_(
        pres: *mut MKL_Complex8,
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotci_(
        pres: *mut MKL_Complex8,
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *const MKL_Complex8,
    );
    pub fn cdotu_(
        pres: *mut MKL_Complex8,
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotui_(
        pres: *mut MKL_Complex8,
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *const MKL_Complex8,
    );
    pub fn cgthr_(
        nz: *const MKL_INT,
        y: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        indx: *const MKL_INT,
    );
    pub fn cgthrz_(
        nz: *const MKL_INT,
        y: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        indx: *const MKL_INT,
    );
    pub fn crot_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const MKL_Complex8,
    );
    pub fn crotg_(a: *mut MKL_Complex8, b: *const MKL_Complex8, c: *mut f32, s: *mut MKL_Complex8);
    pub fn cscal_(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn csctr_(
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *mut MKL_Complex8,
    );
    pub fn csrot_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const f32,
    );
    pub fn csscal_(n: *const MKL_INT, a: *const f32, x: *mut MKL_Complex8, incx: *const MKL_INT);
    pub fn cswap_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn icamax_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT;
    pub fn icamin_(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT;
    pub fn dcabs1_(z: *const MKL_Complex16) -> f64;
    pub fn dasum_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64;
    pub fn daxpy_(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn daxpby_(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn daxpyi_(
        nz: *const MKL_INT,
        a: *const f64,
        x: *const f64,
        indx: *const MKL_INT,
        y: *mut f64,
    );
    pub fn dcopy_(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn ddot_(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
    ) -> f64;
    pub fn dsdot_(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f64;
    pub fn ddoti_(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *const f64) -> f64;
    pub fn dgthr_(nz: *const MKL_INT, y: *const f64, x: *mut f64, indx: *const MKL_INT);
    pub fn dgthrz_(nz: *const MKL_INT, y: *mut f64, x: *mut f64, indx: *const MKL_INT);
    pub fn dnrm2_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64;
    pub fn drot_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotg_(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64);
    pub fn droti_(
        nz: *const MKL_INT,
        x: *mut f64,
        indx: *const MKL_INT,
        y: *mut f64,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotm_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        param: *const f64,
    );
    pub fn drotmg_(d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64);
    pub fn dscal_(n: *const MKL_INT, a: *const f64, x: *mut f64, incx: *const MKL_INT);
    pub fn dsctr_(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *mut f64);
    pub fn dswap_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn dzasum_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64;
    pub fn dznrm2_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64;
    pub fn idamax_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT;
    pub fn idamin_(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT;
    pub fn zaxpy_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zaxpby_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zaxpyi_(
        nz: *const MKL_INT,
        a: *const MKL_Complex16,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *mut MKL_Complex16,
    );
    pub fn zcopy_(
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotc_(
        pres: *mut MKL_Complex16,
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotci_(
        pres: *mut MKL_Complex16,
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *const MKL_Complex16,
    );
    pub fn zdotu_(
        pres: *mut MKL_Complex16,
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotui_(
        pres: *mut MKL_Complex16,
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *const MKL_Complex16,
    );
    pub fn zdrot_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const f64,
    );
    pub fn zdscal_(n: *const MKL_INT, a: *const f64, x: *mut MKL_Complex16, incx: *const MKL_INT);
    pub fn zgthr_(
        nz: *const MKL_INT,
        y: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        indx: *const MKL_INT,
    );
    pub fn zgthrz_(
        nz: *const MKL_INT,
        y: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        indx: *const MKL_INT,
    );
    pub fn zrot_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const MKL_Complex16,
    );
    pub fn zrotg_(
        a: *mut MKL_Complex16,
        b: *const MKL_Complex16,
        c: *mut f64,
        s: *mut MKL_Complex16,
    );
    pub fn zscal_(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn zsctr_(
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *mut MKL_Complex16,
    );
    pub fn zswap_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn izamax_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT;
    pub fn izamin_(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT;
    pub fn sgbmv_(
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
    );
    pub fn sgemv_(
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
    );
    pub fn sger_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn ssbmv_(
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
    );
    pub fn sspmv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        ap: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn sspr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        ap: *mut f32,
    );
    pub fn sspr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
        ap: *mut f32,
    );
    pub fn ssymv_(
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
    );
    pub fn ssyr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn ssyr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn stbmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn stbsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn stpmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn stpsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn strmv_(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn strsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn sgem2vu_(
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
    );
    pub fn cgbmv_(
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
    );
    pub fn cgemv_(
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
    );
    pub fn cgerc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn cgeru_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn chbmv_(
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
    );
    pub fn chemv_(
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
    );
    pub fn cher_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn cher2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn chpmv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        ap: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn chpr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex8,
    );
    pub fn chpr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
        ap: *mut MKL_Complex8,
    );
    pub fn ctbmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctbsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctpmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctpsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctrmv_(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctrsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn cgem2vc_(
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
    );
    pub fn scgemv_(
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
    );
    pub fn dgbmv_(
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
    );
    pub fn dgemv_(
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
    );
    pub fn dger_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dsbmv_(
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
    );
    pub fn dspmv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        ap: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn dspr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        ap: *mut f64,
    );
    pub fn dspr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
        ap: *mut f64,
    );
    pub fn dsymv_(
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
    );
    pub fn dsyr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dsyr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dtbmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtbsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtpmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtpsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtrmv_(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtrsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dgem2vu_(
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
    );
    pub fn zgbmv_(
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
    );
    pub fn zgemv_(
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
    );
    pub fn zgerc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zgeru_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zhbmv_(
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
    );
    pub fn zhemv_(
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
    );
    pub fn zher_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zher2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zhpmv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        ap: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zhpr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex16,
    );
    pub fn zhpr2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
        ap: *mut MKL_Complex16,
    );
    pub fn ztbmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztbsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztpmv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztpsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztrmv_(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztrsv_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn zgem2vc_(
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
    );
    pub fn dzgemv_(
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
    );
    pub fn sgemm_(
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
    );
    pub fn sgemm_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn sgemm_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f32,
        src: *const f32,
        ld: *const MKL_INT,
        dest: *mut f32,
    );
    pub fn sgemm_compute_(
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
    );
    pub fn sgemm_batch_(
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
    );
    pub fn sgemm_batch_strided_(
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
    );
    pub fn sgemmt_(
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
    );
    pub fn ssymm_(
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
    );
    pub fn ssyr2k_(
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
    );
    pub fn ssyrk_(
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
    );
    pub fn ssyrk_batch_(
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
    );
    pub fn ssyrk_batch_strided_(
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
    );
    pub fn strmm_(
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
    );
    pub fn strmm_oop_(
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
    );
    pub fn strsm_(
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
    );
    pub fn strsm_oop_(
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
    );
    pub fn strsm_batch_(
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
    );
    pub fn strsm_batch_strided_(
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
    );
    pub fn cgemm_(
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
    );
    pub fn cgemm_batch_(
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
    );
    pub fn cgemm_batch_strided_(
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
    );
    pub fn scgemm_(
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
    );
    pub fn cgemm3m_(
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
    );
    pub fn cgemm3m_batch_(
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
    );
    pub fn cgemm3m_batch_strided_(
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
    );
    pub fn cgemmt_(
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
    );
    pub fn chemm_(
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
    );
    pub fn cher2k_(
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
    );
    pub fn cherk_(
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
    );
    pub fn csymm_(
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
    );
    pub fn csyr2k_(
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
    );
    pub fn csyrk_(
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
    );
    pub fn csyrk_batch_(
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
    );
    pub fn csyrk_batch_strided_(
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
    );
    pub fn ctrmm_(
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
    );
    pub fn ctrmm_oop_(
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
    );
    pub fn ctrsm_(
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
    );
    pub fn ctrsm_oop_(
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
    );
    pub fn ctrsm_batch_(
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
    );
    pub fn ctrsm_batch_strided_(
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
    );
    pub fn dgemm_(
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
    );
    pub fn dgemm_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn dgemm_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f64,
        src: *const f64,
        ld: *const MKL_INT,
        dest: *mut f64,
    );
    pub fn dgemm_compute_(
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
    );
    pub fn dgemm_batch_(
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
    );
    pub fn dgemm_batch_strided_(
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
    );
    pub fn dgemmt_(
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
    );
    pub fn dsymm_(
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
    );
    pub fn dsyr2k_(
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
    );
    pub fn dsyrk_(
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
    );
    pub fn dsyrk_batch_(
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
    );
    pub fn dsyrk_batch_strided_(
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
    );
    pub fn dtrmm_(
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
    );
    pub fn dtrmm_oop_(
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
    );
    pub fn dtrsm_(
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
    );
    pub fn dtrsm_oop_(
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
    );
    pub fn dtrsm_batch_(
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
    );
    pub fn dtrsm_batch_strided_(
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
    );
    pub fn zgemm_(
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
    );
    pub fn zgemm_batch_(
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
    );
    pub fn zgemm_batch_strided_(
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
    );
    pub fn dzgemm_(
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
    );
    pub fn zgemm3m_(
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
    );
    pub fn zgemm3m_batch_(
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
    );
    pub fn zgemm3m_batch_strided_(
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
    );
    pub fn zgemmt_(
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
    );
    pub fn zhemm_(
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
    );
    pub fn zher2k_(
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
    );
    pub fn zherk_(
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
    );
    pub fn zsymm_(
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
    );
    pub fn zsyr2k_(
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
    );
    pub fn zsyrk_(
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
    );
    pub fn zsyrk_batch_(
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
    );
    pub fn zsyrk_batch_strided_(
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
    );
    pub fn ztrmm_(
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
    );
    pub fn ztrmm_oop_(
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
    );
    pub fn ztrsm_(
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
    );
    pub fn ztrsm_oop_(
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
    );
    pub fn ztrsm_batch_(
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
    );
    pub fn ztrsm_batch_strided_(
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
    );
    pub fn gemm_s16s16s32_(
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
    );
    pub fn gemm_s8u8s32_(
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
    );
    pub fn gemm_bf16bf16f32_(
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
    );
    pub fn gemm_f16f16f32_(
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
    );
    pub fn gemm_e5m2e5m2f32_(
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
    );
    pub fn gemm_e4m3e4m3f32_(
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
    );
    pub fn gemm_s8u8s32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_s16s16s32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_bf16bf16f32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_f16f16f32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_e5m2e5m2f32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_e4m3e4m3f32_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_s8u8s32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const c_void,
        ld: *const MKL_INT,
        dest: *mut c_void,
    );
    pub fn gemm_s16s16s32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_INT16,
        ld: *const MKL_INT,
        dest: *mut MKL_INT16,
    );
    pub fn gemm_bf16bf16f32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_BF16,
        ld: *const MKL_INT,
        dest: *mut MKL_BF16,
    );
    pub fn gemm_f16f16f32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_F16,
        ld: *const MKL_INT,
        dest: *mut MKL_F16,
    );
    pub fn gemm_e5m2e5m2f32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_E5M2,
        ld: *const MKL_INT,
        dest: *mut MKL_E5M2,
    );
    pub fn gemm_e4m3e4m3f32_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_E4M3,
        ld: *const MKL_INT,
        dest: *mut MKL_E4M3,
    );
    pub fn gemm_s8u8s32_compute_(
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
    );
    pub fn gemm_s16s16s32_compute_(
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
    );
    pub fn gemm_bf16bf16f32_compute_(
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
    );
    pub fn gemm_f16f16f32_compute_(
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
    );
    pub fn gemm_e5m2e5m2f32_compute_(
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
    );
    pub fn gemm_e4m3e4m3f32_compute_(
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
    );
    pub fn hgemm_(
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
    );
    pub fn hgemm_pack_get_size_(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn hgemm_pack_(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const MKL_F16,
        src: *const MKL_F16,
        ld: *const MKL_INT,
        dest: *mut MKL_F16,
    );
    pub fn hgemm_compute_(
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
    );
    pub fn mkl_cblas_jit_create_dgemm_(
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
    ) -> mkl_jit_status_t;
    pub fn mkl_cblas_jit_create_sgemm_(
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
    ) -> mkl_jit_status_t;
    pub fn mkl_cblas_jit_create_cgemm_(
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
    ) -> mkl_jit_status_t;
    pub fn mkl_cblas_jit_create_zgemm_(
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
    ) -> mkl_jit_status_t;
    pub fn mkl_jit_get_dgemm_ptr_(jitter: *const c_void) -> dgemm_jit_kernel_t;
    pub fn mkl_jit_get_sgemm_ptr_(jitter: *const c_void) -> sgemm_jit_kernel_t;
    pub fn mkl_jit_get_cgemm_ptr_(jitter: *const c_void) -> cgemm_jit_kernel_t;
    pub fn mkl_jit_get_zgemm_ptr_(jitter: *const c_void) -> zgemm_jit_kernel_t;
    pub fn mkl_jit_destroy_(jitter: *mut c_void) -> mkl_jit_status_t;
    pub fn saxpy_batch_(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *mut *const f32,
        incx: *const MKL_INT,
        y: *mut *mut f32,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn daxpy_batch_(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *mut *const f64,
        incx: *const MKL_INT,
        y: *mut *mut f64,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn caxpy_batch_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *mut *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex8,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn zaxpy_batch_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *mut *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex16,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn scopy_batch_(
        n: *const MKL_INT,
        x: *mut *const f32,
        incx: *const MKL_INT,
        y: *mut *mut f32,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn dcopy_batch_(
        n: *const MKL_INT,
        x: *mut *const f64,
        incx: *const MKL_INT,
        y: *mut *mut f64,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn ccopy_batch_(
        n: *const MKL_INT,
        x: *mut *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex8,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn zcopy_batch_(
        n: *const MKL_INT,
        x: *mut *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex16,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn saxpy_batch_strided_(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn daxpy_batch_strided_(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn caxpy_batch_strided_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn zaxpy_batch_strided_(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn scopy_batch_strided_(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn dcopy_batch_strided_(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn ccopy_batch_strided_(
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn zcopy_batch_strided_(
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn sgemv_batch_(
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
    );
    pub fn sgemv_batch_strided_(
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
    );
    pub fn dgemv_batch_(
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
    );
    pub fn dgemv_batch_strided_(
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
    );
    pub fn cgemv_batch_(
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
    );
    pub fn cgemv_batch_strided_(
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
    );
    pub fn zgemv_batch_(
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
    );
    pub fn zgemv_batch_strided_(
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
    );
    pub fn sdgmm_batch_(
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
    );
    pub fn sdgmm_batch_strided_(
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
    );
    pub fn ddgmm_batch_(
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
    );
    pub fn ddgmm_batch_strided_(
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
    );
    pub fn cdgmm_batch_(
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
    );
    pub fn cdgmm_batch_strided_(
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
    );
    pub fn zdgmm_batch_(
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
    );
    pub fn zdgmm_batch_strided_(
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
    );
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
