//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn xerbla(srname: *const c_char, info: *const c_int, lsrname: c_int);
    pub fn lsame(ca: *const c_char, cb: *const c_char, lca: MKL_INT, lcb: MKL_INT) -> c_int;
    pub fn scabs1(c: *const MKL_Complex8) -> f32;
    pub fn sasum(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32;
    pub fn saxpy(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn saxpby(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn saxpyi(
        nz: *const MKL_INT,
        a: *const f32,
        x: *const f32,
        indx: *const MKL_INT,
        y: *mut f32,
    );
    pub fn scasum(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32;
    pub fn scnrm2(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> f32;
    pub fn scopy(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn sdot(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f32;
    pub fn sdoti(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *const f32) -> f32;
    pub fn sdsdot(
        n: *const MKL_INT,
        sb: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f32;
    pub fn sgthr(nz: *const MKL_INT, y: *const f32, x: *mut f32, indx: *const MKL_INT);
    pub fn sgthrz(nz: *const MKL_INT, y: *mut f32, x: *mut f32, indx: *const MKL_INT);
    pub fn snrm2(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> f32;
    pub fn srot(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const f32,
    );
    pub fn srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32);
    pub fn sroti(
        nz: *const MKL_INT,
        x: *mut f32,
        indx: *const MKL_INT,
        y: *mut f32,
        c: *const f32,
        s: *const f32,
    );
    pub fn srotm(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        param: *const f32,
    );
    pub fn srotmg(d1: *mut f32, d2: *mut f32, x1: *mut f32, y1: *const f32, param: *mut f32);
    pub fn sscal(n: *const MKL_INT, a: *const f32, x: *mut f32, incx: *const MKL_INT);
    pub fn ssctr(nz: *const MKL_INT, x: *const f32, indx: *const MKL_INT, y: *mut f32);
    pub fn sswap(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn isamax(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT;
    pub fn isamin(n: *const MKL_INT, x: *const f32, incx: *const MKL_INT) -> MKL_INT;
    pub fn caxpy(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn caxpby(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn caxpyi(
        nz: *const MKL_INT,
        a: *const MKL_Complex8,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *mut MKL_Complex8,
    );
    pub fn ccopy(
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotc(
        pres: *mut MKL_Complex8,
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotci(
        pres: *mut MKL_Complex8,
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *const MKL_Complex8,
    );
    pub fn cdotu(
        pres: *mut MKL_Complex8,
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn cdotui(
        pres: *mut MKL_Complex8,
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *const MKL_Complex8,
    );
    pub fn cgthr(
        nz: *const MKL_INT,
        y: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        indx: *const MKL_INT,
    );
    pub fn cgthrz(
        nz: *const MKL_INT,
        y: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        indx: *const MKL_INT,
    );
    pub fn crot(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const MKL_Complex8,
    );
    pub fn crotg(a: *mut MKL_Complex8, b: *const MKL_Complex8, c: *mut f32, s: *mut MKL_Complex8);
    pub fn cscal(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn csctr(
        nz: *const MKL_INT,
        x: *const MKL_Complex8,
        indx: *const MKL_INT,
        y: *mut MKL_Complex8,
    );
    pub fn csrot(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const f32,
    );
    pub fn csscal(n: *const MKL_INT, a: *const f32, x: *mut MKL_Complex8, incx: *const MKL_INT);
    pub fn cswap(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
    );
    pub fn icamax(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT;
    pub fn icamin(n: *const MKL_INT, x: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT;
    pub fn dcabs1(z: *const MKL_Complex16) -> f64;
    pub fn dasum(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64;
    pub fn daxpy(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn daxpby(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn daxpyi(
        nz: *const MKL_INT,
        a: *const f64,
        x: *const f64,
        indx: *const MKL_INT,
        y: *mut f64,
    );
    pub fn dcopy(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn ddot(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
    ) -> f64;
    pub fn dsdot(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
    ) -> f64;
    pub fn ddoti(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *const f64) -> f64;
    pub fn dgthr(nz: *const MKL_INT, y: *const f64, x: *mut f64, indx: *const MKL_INT);
    pub fn dgthrz(nz: *const MKL_INT, y: *mut f64, x: *mut f64, indx: *const MKL_INT);
    pub fn dnrm2(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> f64;
    pub fn drot(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64);
    pub fn droti(
        nz: *const MKL_INT,
        x: *mut f64,
        indx: *const MKL_INT,
        y: *mut f64,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotm(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        param: *const f64,
    );
    pub fn drotmg(d1: *mut f64, d2: *mut f64, x1: *mut f64, y1: *const f64, param: *mut f64);
    pub fn dscal(n: *const MKL_INT, a: *const f64, x: *mut f64, incx: *const MKL_INT);
    pub fn dsctr(nz: *const MKL_INT, x: *const f64, indx: *const MKL_INT, y: *mut f64);
    pub fn dswap(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn dzasum(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64;
    pub fn dznrm2(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> f64;
    pub fn idamax(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT;
    pub fn idamin(n: *const MKL_INT, x: *const f64, incx: *const MKL_INT) -> MKL_INT;
    pub fn zaxpy(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zaxpby(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zaxpyi(
        nz: *const MKL_INT,
        a: *const MKL_Complex16,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *mut MKL_Complex16,
    );
    pub fn zcopy(
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotc(
        pres: *mut MKL_Complex16,
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotci(
        pres: *mut MKL_Complex16,
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *const MKL_Complex16,
    );
    pub fn zdotu(
        pres: *mut MKL_Complex16,
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn zdotui(
        pres: *mut MKL_Complex16,
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *const MKL_Complex16,
    );
    pub fn zdrot(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const f64,
    );
    pub fn zdscal(n: *const MKL_INT, a: *const f64, x: *mut MKL_Complex16, incx: *const MKL_INT);
    pub fn zgthr(
        nz: *const MKL_INT,
        y: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        indx: *const MKL_INT,
    );
    pub fn zgthrz(
        nz: *const MKL_INT,
        y: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        indx: *const MKL_INT,
    );
    pub fn zrot(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const MKL_Complex16,
    );
    pub fn zrotg(
        a: *mut MKL_Complex16,
        b: *const MKL_Complex16,
        c: *mut f64,
        s: *mut MKL_Complex16,
    );
    pub fn zscal(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn zsctr(
        nz: *const MKL_INT,
        x: *const MKL_Complex16,
        indx: *const MKL_INT,
        y: *mut MKL_Complex16,
    );
    pub fn zswap(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
    );
    pub fn izamax(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT;
    pub fn izamin(n: *const MKL_INT, x: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT;
    pub fn sgbmv(
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
    pub fn sgemv(
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
    pub fn sger(
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
    pub fn ssbmv(
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
    pub fn sspmv(
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
    pub fn sspr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        ap: *mut f32,
    );
    pub fn sspr2(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        y: *const f32,
        incy: *const MKL_INT,
        ap: *mut f32,
    );
    pub fn ssymv(
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
    pub fn ssyr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const f32,
        incx: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn ssyr2(
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
    pub fn stbmv(
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
    pub fn stbsv(
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
    pub fn stpmv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn stpsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn strmv(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn strsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
    );
    pub fn sgem2vu(
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
    pub fn cgbmv(
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
    pub fn cgemv(
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
    pub fn cgerc(
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
    pub fn cgeru(
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
    pub fn chbmv(
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
    pub fn chemv(
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
    pub fn cher(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn cher2(
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
    pub fn chpmv(
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
    pub fn chpr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f32,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex8,
    );
    pub fn chpr2(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *const MKL_Complex8,
        incy: *const MKL_INT,
        ap: *mut MKL_Complex8,
    );
    pub fn ctbmv(
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
    pub fn ctbsv(
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
    pub fn ctpmv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctpsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctrmv(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn ctrsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
    );
    pub fn cgem2vc(
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
    pub fn scgemv(
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
    pub fn dgbmv(
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
    pub fn dgemv(
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
    pub fn dger(
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
    pub fn dsbmv(
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
    pub fn dspmv(
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
    pub fn dspr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        ap: *mut f64,
    );
    pub fn dspr2(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        y: *const f64,
        incy: *const MKL_INT,
        ap: *mut f64,
    );
    pub fn dsymv(
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
    pub fn dsyr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const f64,
        incx: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dsyr2(
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
    pub fn dtbmv(
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
    pub fn dtbsv(
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
    pub fn dtpmv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtpsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtrmv(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dtrsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
    );
    pub fn dgem2vu(
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
    pub fn zgbmv(
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
    pub fn zgemv(
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
    pub fn zgerc(
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
    pub fn zgeru(
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
    pub fn zhbmv(
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
    pub fn zhemv(
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
    pub fn zher(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zher2(
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
    pub fn zhpmv(
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
    pub fn zhpr(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const f64,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex16,
    );
    pub fn zhpr2(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *const MKL_Complex16,
        incy: *const MKL_INT,
        ap: *mut MKL_Complex16,
    );
    pub fn ztbmv(
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
    pub fn ztbsv(
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
    pub fn ztpmv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztpsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztrmv(
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn ztrsv(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
    );
    pub fn zgem2vc(
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
    pub fn dzgemv(
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
    pub fn sgemm(
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
    pub fn sgemm_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn sgemm_pack(
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
    pub fn sgemm_compute(
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
    pub fn sgemm_batch(
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
    pub fn sgemm_batch_strided(
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
    pub fn sgemmt(
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
    pub fn ssymm(
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
    pub fn ssyr2k(
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
    pub fn ssyrk(
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
    pub fn ssyrk_batch(
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
    pub fn ssyrk_batch_strided(
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
    pub fn strmm(
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
    pub fn strmm_oop(
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
    pub fn strsm(
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
    pub fn strsm_oop(
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
    pub fn strsm_batch(
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
    pub fn strsm_batch_strided(
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
    pub fn cgemm(
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
    pub fn cgemm_batch(
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
    pub fn cgemm_batch_strided(
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
    pub fn scgemm(
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
    pub fn cgemm3m(
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
    pub fn cgemm3m_batch(
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
    pub fn cgemm3m_batch_strided(
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
    pub fn cgemmt(
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
    pub fn chemm(
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
    pub fn cher2k(
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
    pub fn cherk(
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
    pub fn csymm(
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
    pub fn csyr2k(
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
    pub fn csyrk(
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
    pub fn csyrk_batch(
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
    pub fn csyrk_batch_strided(
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
    pub fn ctrmm(
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
    pub fn ctrmm_oop(
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
    pub fn ctrsm(
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
    pub fn ctrsm_oop(
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
    pub fn ctrsm_batch(
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
    pub fn ctrsm_batch_strided(
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
    pub fn dgemm(
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
    pub fn dgemm_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn dgemm_pack(
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
    pub fn dgemm_compute(
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
    pub fn dgemm_batch(
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
    pub fn dgemm_batch_strided(
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
    pub fn dgemmt(
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
    pub fn dsymm(
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
    pub fn dsyr2k(
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
    pub fn dsyrk(
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
    pub fn dsyrk_batch(
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
    pub fn dsyrk_batch_strided(
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
    pub fn dtrmm(
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
    pub fn dtrmm_oop(
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
    pub fn dtrsm(
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
    pub fn dtrsm_oop(
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
    pub fn dtrsm_batch(
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
    pub fn dtrsm_batch_strided(
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
    pub fn zgemm(
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
    pub fn zgemm_batch(
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
    pub fn zgemm_batch_strided(
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
    pub fn dzgemm(
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
    pub fn zgemm3m(
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
    pub fn zgemm3m_batch(
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
    pub fn zgemm3m_batch_strided(
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
    pub fn zgemmt(
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
    pub fn zhemm(
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
    pub fn zher2k(
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
    pub fn zherk(
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
    pub fn zsymm(
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
    pub fn zsyr2k(
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
    pub fn zsyrk(
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
    pub fn zsyrk_batch(
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
    pub fn zsyrk_batch_strided(
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
    pub fn ztrmm(
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
    pub fn ztrmm_oop(
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
    pub fn ztrsm(
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
    pub fn ztrsm_oop(
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
    pub fn ztrsm_batch(
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
    pub fn ztrsm_batch_strided(
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
    pub fn gemm_s16s16s32(
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
    pub fn gemm_s8u8s32(
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
    pub fn gemm_bf16bf16f32(
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
    pub fn gemm_f16f16f32(
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
    pub fn gemm_e5m2e5m2f32(
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
    pub fn gemm_e4m3e4m3f32(
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
    pub fn gemm_s8u8s32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_s16s16s32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_bf16bf16f32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_f16f16f32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_e5m2e5m2f32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_e4m3e4m3f32_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn gemm_s8u8s32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const c_void,
        ld: *const MKL_INT,
        dest: *mut c_void,
    );
    pub fn gemm_s16s16s32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_INT16,
        ld: *const MKL_INT,
        dest: *mut MKL_INT16,
    );
    pub fn gemm_bf16bf16f32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_BF16,
        ld: *const MKL_INT,
        dest: *mut MKL_BF16,
    );
    pub fn gemm_f16f16f32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_F16,
        ld: *const MKL_INT,
        dest: *mut MKL_F16,
    );
    pub fn gemm_e5m2e5m2f32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_E5M2,
        ld: *const MKL_INT,
        dest: *mut MKL_E5M2,
    );
    pub fn gemm_e4m3e4m3f32_pack(
        identifier: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        src: *const MKL_E4M3,
        ld: *const MKL_INT,
        dest: *mut MKL_E4M3,
    );
    pub fn gemm_s8u8s32_compute(
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
    pub fn gemm_s16s16s32_compute(
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
    pub fn gemm_bf16bf16f32_compute(
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
    pub fn gemm_f16f16f32_compute(
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
    pub fn gemm_e5m2e5m2f32_compute(
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
    pub fn gemm_e4m3e4m3f32_compute(
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
    pub fn hgemm(
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
    pub fn hgemm_pack_get_size(
        identifier: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
    ) -> usize;
    pub fn hgemm_pack(
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
    pub fn hgemm_compute(
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
    pub fn mkl_cblas_jit_create_dgemm(
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
    pub fn mkl_cblas_jit_create_sgemm(
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
    pub fn mkl_cblas_jit_create_cgemm(
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
    pub fn mkl_cblas_jit_create_zgemm(
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
    pub fn mkl_jit_get_dgemm_ptr(jitter: *const c_void) -> dgemm_jit_kernel_t;
    pub fn mkl_jit_get_sgemm_ptr(jitter: *const c_void) -> sgemm_jit_kernel_t;
    pub fn mkl_jit_get_cgemm_ptr(jitter: *const c_void) -> cgemm_jit_kernel_t;
    pub fn mkl_jit_get_zgemm_ptr(jitter: *const c_void) -> zgemm_jit_kernel_t;
    pub fn mkl_jit_destroy(jitter: *mut c_void) -> mkl_jit_status_t;
    pub fn saxpy_batch(
        n: *const MKL_INT,
        alpha: *const f32,
        x: *mut *const f32,
        incx: *const MKL_INT,
        y: *mut *mut f32,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn daxpy_batch(
        n: *const MKL_INT,
        alpha: *const f64,
        x: *mut *const f64,
        incx: *const MKL_INT,
        y: *mut *mut f64,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn caxpy_batch(
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *mut *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex8,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn zaxpy_batch(
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *mut *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex16,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn scopy_batch(
        n: *const MKL_INT,
        x: *mut *const f32,
        incx: *const MKL_INT,
        y: *mut *mut f32,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn dcopy_batch(
        n: *const MKL_INT,
        x: *mut *const f64,
        incx: *const MKL_INT,
        y: *mut *mut f64,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn ccopy_batch(
        n: *const MKL_INT,
        x: *mut *const MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex8,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn zcopy_batch(
        n: *const MKL_INT,
        x: *mut *const MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut *mut MKL_Complex16,
        incy: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
    );
    pub fn saxpy_batch_strided(
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
    pub fn daxpy_batch_strided(
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
    pub fn caxpy_batch_strided(
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
    pub fn zaxpy_batch_strided(
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
    pub fn scopy_batch_strided(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn dcopy_batch_strided(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn ccopy_batch_strided(
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn zcopy_batch_strided(
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        stridex: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        stridey: *const MKL_INT,
        batch_size: *const MKL_INT,
    );
    pub fn sgemv_batch(
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
    pub fn sgemv_batch_strided(
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
    pub fn dgemv_batch(
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
    pub fn dgemv_batch_strided(
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
    pub fn cgemv_batch(
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
    pub fn cgemv_batch_strided(
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
    pub fn zgemv_batch(
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
    pub fn zgemv_batch_strided(
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
    pub fn sdgmm_batch(
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
    pub fn sdgmm_batch_strided(
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
    pub fn ddgmm_batch(
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
    pub fn ddgmm_batch_strided(
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
    pub fn cdgmm_batch(
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
    pub fn cdgmm_batch_strided(
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
    pub fn zdgmm_batch(
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
    pub fn zdgmm_batch_strided(
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
