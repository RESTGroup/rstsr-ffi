//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn openblas_set_num_threads(num_threads: c_int);
    pub fn goto_set_num_threads(num_threads: c_int);
    pub fn openblas_set_num_threads_local(num_threads: c_int) -> c_int;
    pub fn openblas_get_num_threads() -> c_int;
    pub fn openblas_get_num_procs() -> c_int;
    pub fn openblas_get_config() -> *mut c_char;
    pub fn openblas_get_corename() -> *mut c_char;
    pub fn openblas_set_threads_callback_function(callback: openblas_threads_callback);
    pub fn openblas_get_parallel() -> c_int;
    pub fn cblas_sdsdot(
        n: blas_int,
        alpha: f32,
        x: *const f32,
        incx: blas_int,
        y: *const f32,
        incy: blas_int,
    ) -> f32;
    pub fn cblas_dsdot(
        n: blas_int,
        x: *const f32,
        incx: blas_int,
        y: *const f32,
        incy: blas_int,
    ) -> f64;
    pub fn cblas_sdot(
        n: blas_int,
        x: *const f32,
        incx: blas_int,
        y: *const f32,
        incy: blas_int,
    ) -> f32;
    pub fn cblas_ddot(
        n: blas_int,
        x: *const f64,
        incx: blas_int,
        y: *const f64,
        incy: blas_int,
    ) -> f64;
    pub fn cblas_cdotu(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
    ) -> openblas_complex_float;
    pub fn cblas_cdotc(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
    ) -> openblas_complex_float;
    pub fn cblas_zdotu(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
    ) -> openblas_complex_double;
    pub fn cblas_zdotc(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
    ) -> openblas_complex_double;
    pub fn cblas_cdotu_sub(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
        ret: *mut c_void,
    );
    pub fn cblas_cdotc_sub(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
        ret: *mut c_void,
    );
    pub fn cblas_zdotu_sub(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
        ret: *mut c_void,
    );
    pub fn cblas_zdotc_sub(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *const c_void,
        incy: blas_int,
        ret: *mut c_void,
    );
    pub fn cblas_sasum(n: blas_int, x: *const f32, incx: blas_int) -> f32;
    pub fn cblas_dasum(n: blas_int, x: *const f64, incx: blas_int) -> f64;
    pub fn cblas_scasum(n: blas_int, x: *const c_void, incx: blas_int) -> f32;
    pub fn cblas_dzasum(n: blas_int, x: *const c_void, incx: blas_int) -> f64;
    pub fn cblas_ssum(n: blas_int, x: *const f32, incx: blas_int) -> f32;
    pub fn cblas_dsum(n: blas_int, x: *const f64, incx: blas_int) -> f64;
    pub fn cblas_scsum(n: blas_int, x: *const c_void, incx: blas_int) -> f32;
    pub fn cblas_dzsum(n: blas_int, x: *const c_void, incx: blas_int) -> f64;
    pub fn cblas_snrm2(N: blas_int, X: *const f32, incX: blas_int) -> f32;
    pub fn cblas_dnrm2(N: blas_int, X: *const f64, incX: blas_int) -> f64;
    pub fn cblas_scnrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f32;
    pub fn cblas_dznrm2(N: blas_int, X: *const c_void, incX: blas_int) -> f64;
    pub fn cblas_isamax(n: blas_int, x: *const f32, incx: blas_int) -> usize;
    pub fn cblas_idamax(n: blas_int, x: *const f64, incx: blas_int) -> usize;
    pub fn cblas_icamax(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_izamax(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_isamin(n: blas_int, x: *const f32, incx: blas_int) -> usize;
    pub fn cblas_idamin(n: blas_int, x: *const f64, incx: blas_int) -> usize;
    pub fn cblas_icamin(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_izamin(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_samax(n: blas_int, x: *const f32, incx: blas_int) -> f32;
    pub fn cblas_damax(n: blas_int, x: *const f64, incx: blas_int) -> f64;
    pub fn cblas_scamax(n: blas_int, x: *const c_void, incx: blas_int) -> f32;
    pub fn cblas_dzamax(n: blas_int, x: *const c_void, incx: blas_int) -> f64;
    pub fn cblas_samin(n: blas_int, x: *const f32, incx: blas_int) -> f32;
    pub fn cblas_damin(n: blas_int, x: *const f64, incx: blas_int) -> f64;
    pub fn cblas_scamin(n: blas_int, x: *const c_void, incx: blas_int) -> f32;
    pub fn cblas_dzamin(n: blas_int, x: *const c_void, incx: blas_int) -> f64;
    pub fn cblas_ismax(n: blas_int, x: *const f32, incx: blas_int) -> usize;
    pub fn cblas_idmax(n: blas_int, x: *const f64, incx: blas_int) -> usize;
    pub fn cblas_icmax(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_izmax(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_ismin(n: blas_int, x: *const f32, incx: blas_int) -> usize;
    pub fn cblas_idmin(n: blas_int, x: *const f64, incx: blas_int) -> usize;
    pub fn cblas_icmin(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_izmin(n: blas_int, x: *const c_void, incx: blas_int) -> usize;
    pub fn cblas_saxpy(
        n: blas_int,
        alpha: f32,
        x: *const f32,
        incx: blas_int,
        y: *mut f32,
        incy: blas_int,
    );
    pub fn cblas_daxpy(
        n: blas_int,
        alpha: f64,
        x: *const f64,
        incx: blas_int,
        y: *mut f64,
        incy: blas_int,
    );
    pub fn cblas_caxpy(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_zaxpy(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_caxpyc(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_zaxpyc(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_scopy(n: blas_int, x: *const f32, incx: blas_int, y: *mut f32, incy: blas_int);
    pub fn cblas_dcopy(n: blas_int, x: *const f64, incx: blas_int, y: *mut f64, incy: blas_int);
    pub fn cblas_ccopy(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_zcopy(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_sswap(n: blas_int, x: *mut f32, incx: blas_int, y: *mut f32, incy: blas_int);
    pub fn cblas_dswap(n: blas_int, x: *mut f64, incx: blas_int, y: *mut f64, incy: blas_int);
    pub fn cblas_cswap(n: blas_int, x: *mut c_void, incx: blas_int, y: *mut c_void, incy: blas_int);
    pub fn cblas_zswap(n: blas_int, x: *mut c_void, incx: blas_int, y: *mut c_void, incy: blas_int);
    pub fn cblas_srot(
        N: blas_int,
        X: *mut f32,
        incX: blas_int,
        Y: *mut f32,
        incY: blas_int,
        c: f32,
        s: f32,
    );
    pub fn cblas_drot(
        N: blas_int,
        X: *mut f64,
        incX: blas_int,
        Y: *mut f64,
        incY: blas_int,
        c: f64,
        s: f64,
    );
    pub fn cblas_csrot(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incY: blas_int,
        c: f32,
        s: f32,
    );
    pub fn cblas_zdrot(
        n: blas_int,
        x: *const c_void,
        incx: blas_int,
        y: *mut c_void,
        incY: blas_int,
        c: f64,
        s: f64,
    );
    pub fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32);
    pub fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64);
    pub fn cblas_crotg(a: *mut c_void, b: *mut c_void, c: *mut f32, s: *mut c_void);
    pub fn cblas_zrotg(a: *mut c_void, b: *mut c_void, c: *mut f64, s: *mut c_void);
    pub fn cblas_srotm(
        N: blas_int,
        X: *mut f32,
        incX: blas_int,
        Y: *mut f32,
        incY: blas_int,
        P: *const f32,
    );
    pub fn cblas_drotm(
        N: blas_int,
        X: *mut f64,
        incX: blas_int,
        Y: *mut f64,
        incY: blas_int,
        P: *const f64,
    );
    pub fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32);
    pub fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64);
    pub fn cblas_sscal(N: blas_int, alpha: f32, X: *mut f32, incX: blas_int);
    pub fn cblas_dscal(N: blas_int, alpha: f64, X: *mut f64, incX: blas_int);
    pub fn cblas_cscal(N: blas_int, alpha: *const c_void, X: *mut c_void, incX: blas_int);
    pub fn cblas_zscal(N: blas_int, alpha: *const c_void, X: *mut c_void, incX: blas_int);
    pub fn cblas_csscal(N: blas_int, alpha: f32, X: *mut c_void, incX: blas_int);
    pub fn cblas_zdscal(N: blas_int, alpha: f64, X: *mut c_void, incX: blas_int);
    pub fn cblas_sgemv(
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
    );
    pub fn cblas_dgemv(
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
    );
    pub fn cblas_cgemv(
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
    );
    pub fn cblas_zgemv(
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
    );
    pub fn cblas_sger(
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
    );
    pub fn cblas_dger(
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
    );
    pub fn cblas_cgeru(
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
    );
    pub fn cblas_cgerc(
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
    );
    pub fn cblas_zgeru(
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
    );
    pub fn cblas_zgerc(
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
    );
    pub fn cblas_strsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const f32,
        lda: blas_int,
        X: *mut f32,
        incX: blas_int,
    );
    pub fn cblas_dtrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const f64,
        lda: blas_int,
        X: *mut f64,
        incX: blas_int,
    );
    pub fn cblas_ctrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const c_void,
        lda: blas_int,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ztrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const c_void,
        lda: blas_int,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_strmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const f32,
        lda: blas_int,
        X: *mut f32,
        incX: blas_int,
    );
    pub fn cblas_dtrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const f64,
        lda: blas_int,
        X: *mut f64,
        incX: blas_int,
    );
    pub fn cblas_ctrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const c_void,
        lda: blas_int,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ztrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        A: *const c_void,
        lda: blas_int,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ssyr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f32,
        X: *const f32,
        incX: blas_int,
        A: *mut f32,
        lda: blas_int,
    );
    pub fn cblas_dsyr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f64,
        X: *const f64,
        incX: blas_int,
        A: *mut f64,
        lda: blas_int,
    );
    pub fn cblas_cher(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f32,
        X: *const c_void,
        incX: blas_int,
        A: *mut c_void,
        lda: blas_int,
    );
    pub fn cblas_zher(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f64,
        X: *const c_void,
        incX: blas_int,
        A: *mut c_void,
        lda: blas_int,
    );
    pub fn cblas_ssyr2(
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
    );
    pub fn cblas_dsyr2(
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
    );
    pub fn cblas_cher2(
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
    );
    pub fn cblas_zher2(
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
    );
    pub fn cblas_sgbmv(
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
    );
    pub fn cblas_dgbmv(
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
    );
    pub fn cblas_cgbmv(
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
    );
    pub fn cblas_zgbmv(
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
    );
    pub fn cblas_ssbmv(
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
    );
    pub fn cblas_dsbmv(
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
    );
    pub fn cblas_stbmv(
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
    );
    pub fn cblas_dtbmv(
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
    );
    pub fn cblas_ctbmv(
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
    );
    pub fn cblas_ztbmv(
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
    );
    pub fn cblas_stbsv(
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
    );
    pub fn cblas_dtbsv(
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
    );
    pub fn cblas_ctbsv(
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
    );
    pub fn cblas_ztbsv(
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
    );
    pub fn cblas_stpmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const f32,
        X: *mut f32,
        incX: blas_int,
    );
    pub fn cblas_dtpmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const f64,
        X: *mut f64,
        incX: blas_int,
    );
    pub fn cblas_ctpmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const c_void,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ztpmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const c_void,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_stpsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const f32,
        X: *mut f32,
        incX: blas_int,
    );
    pub fn cblas_dtpsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const f64,
        X: *mut f64,
        incX: blas_int,
    );
    pub fn cblas_ctpsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const c_void,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ztpsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: blas_int,
        Ap: *const c_void,
        X: *mut c_void,
        incX: blas_int,
    );
    pub fn cblas_ssymv(
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
    );
    pub fn cblas_dsymv(
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
    );
    pub fn cblas_chemv(
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
    );
    pub fn cblas_zhemv(
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
    );
    pub fn cblas_sspmv(
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
    );
    pub fn cblas_dspmv(
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
    );
    pub fn cblas_sspr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f32,
        X: *const f32,
        incX: blas_int,
        Ap: *mut f32,
    );
    pub fn cblas_dspr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f64,
        X: *const f64,
        incX: blas_int,
        Ap: *mut f64,
    );
    pub fn cblas_chpr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f32,
        X: *const c_void,
        incX: blas_int,
        A: *mut c_void,
    );
    pub fn cblas_zhpr(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f64,
        X: *const c_void,
        incX: blas_int,
        A: *mut c_void,
    );
    pub fn cblas_sspr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f32,
        X: *const f32,
        incX: blas_int,
        Y: *const f32,
        incY: blas_int,
        A: *mut f32,
    );
    pub fn cblas_dspr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: f64,
        X: *const f64,
        incX: blas_int,
        Y: *const f64,
        incY: blas_int,
        A: *mut f64,
    );
    pub fn cblas_chpr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: *const c_void,
        X: *const c_void,
        incX: blas_int,
        Y: *const c_void,
        incY: blas_int,
        Ap: *mut c_void,
    );
    pub fn cblas_zhpr2(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        N: blas_int,
        alpha: *const c_void,
        X: *const c_void,
        incX: blas_int,
        Y: *const c_void,
        incY: blas_int,
        Ap: *mut c_void,
    );
    pub fn cblas_chbmv(
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
    );
    pub fn cblas_zhbmv(
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
    );
    pub fn cblas_chpmv(
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
    );
    pub fn cblas_zhpmv(
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
    );
    pub fn cblas_sgemm(
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
    );
    pub fn cblas_dgemm(
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
    );
    pub fn cblas_cgemm(
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
    );
    pub fn cblas_cgemm3m(
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
    );
    pub fn cblas_zgemm(
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
    );
    pub fn cblas_zgemm3m(
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
    );
    pub fn cblas_sgemmt(
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
    );
    pub fn cblas_dgemmt(
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
    );
    pub fn cblas_cgemmt(
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
    );
    pub fn cblas_zgemmt(
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
    );
    pub fn cblas_ssymm(
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
    );
    pub fn cblas_dsymm(
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
    );
    pub fn cblas_csymm(
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
    );
    pub fn cblas_zsymm(
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
    );
    pub fn cblas_ssyrk(
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
    );
    pub fn cblas_dsyrk(
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
    );
    pub fn cblas_csyrk(
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
    );
    pub fn cblas_zsyrk(
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
    );
    pub fn cblas_ssyr2k(
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
    );
    pub fn cblas_dsyr2k(
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
    );
    pub fn cblas_csyr2k(
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
    );
    pub fn cblas_zsyr2k(
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
    );
    pub fn cblas_strmm(
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
    );
    pub fn cblas_dtrmm(
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
    );
    pub fn cblas_ctrmm(
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
    );
    pub fn cblas_ztrmm(
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
    );
    pub fn cblas_strsm(
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
    );
    pub fn cblas_dtrsm(
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
    );
    pub fn cblas_ctrsm(
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
    );
    pub fn cblas_ztrsm(
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
    );
    pub fn cblas_chemm(
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
    );
    pub fn cblas_zhemm(
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
    );
    pub fn cblas_cherk(
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
    );
    pub fn cblas_zherk(
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
    );
    pub fn cblas_cher2k(
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
    );
    pub fn cblas_zher2k(
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
    );
    pub fn cblas_xerbla(p: blas_int, rout: *const c_char, form: *const c_char, ...);
    #[doc = " BLAS extensions"]
    pub fn cblas_saxpby(
        n: blas_int,
        alpha: f32,
        x: *const f32,
        incx: blas_int,
        beta: f32,
        y: *mut f32,
        incy: blas_int,
    );
    pub fn cblas_daxpby(
        n: blas_int,
        alpha: f64,
        x: *const f64,
        incx: blas_int,
        beta: f64,
        y: *mut f64,
        incy: blas_int,
    );
    pub fn cblas_caxpby(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        beta: *const c_void,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_zaxpby(
        n: blas_int,
        alpha: *const c_void,
        x: *const c_void,
        incx: blas_int,
        beta: *const c_void,
        y: *mut c_void,
        incy: blas_int,
    );
    pub fn cblas_somatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: f32,
        a: *const f32,
        clda: blas_int,
        b: *mut f32,
        cldb: blas_int,
    );
    pub fn cblas_domatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: f64,
        a: *const f64,
        clda: blas_int,
        b: *mut f64,
        cldb: blas_int,
    );
    pub fn cblas_comatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f32,
        a: *const f32,
        clda: blas_int,
        b: *mut f32,
        cldb: blas_int,
    );
    pub fn cblas_zomatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f64,
        a: *const f64,
        clda: blas_int,
        b: *mut f64,
        cldb: blas_int,
    );
    pub fn cblas_simatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: f32,
        a: *mut f32,
        clda: blas_int,
        cldb: blas_int,
    );
    pub fn cblas_dimatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: f64,
        a: *mut f64,
        clda: blas_int,
        cldb: blas_int,
    );
    pub fn cblas_cimatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f32,
        a: *mut f32,
        clda: blas_int,
        cldb: blas_int,
    );
    pub fn cblas_zimatcopy(
        CORDER: CBLAS_ORDER,
        CTRANS: CBLAS_TRANSPOSE,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f64,
        a: *mut f64,
        clda: blas_int,
        cldb: blas_int,
    );
    pub fn cblas_sgeadd(
        CORDER: CBLAS_ORDER,
        crows: blas_int,
        ccols: blas_int,
        calpha: f32,
        a: *const f32,
        clda: blas_int,
        cbeta: f32,
        c: *mut f32,
        cldc: blas_int,
    );
    pub fn cblas_dgeadd(
        CORDER: CBLAS_ORDER,
        crows: blas_int,
        ccols: blas_int,
        calpha: f64,
        a: *const f64,
        clda: blas_int,
        cbeta: f64,
        c: *mut f64,
        cldc: blas_int,
    );
    pub fn cblas_cgeadd(
        CORDER: CBLAS_ORDER,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f32,
        a: *const f32,
        clda: blas_int,
        cbeta: *const f32,
        c: *mut f32,
        cldc: blas_int,
    );
    pub fn cblas_zgeadd(
        CORDER: CBLAS_ORDER,
        crows: blas_int,
        ccols: blas_int,
        calpha: *const f64,
        a: *const f64,
        clda: blas_int,
        cbeta: *const f64,
        c: *mut f64,
        cldc: blas_int,
    );
    pub fn cblas_sgemm_batch(
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
    );
    pub fn cblas_dgemm_batch(
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
    );
    pub fn cblas_cgemm_batch(
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
    );
    pub fn cblas_zgemm_batch(
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
    );
    #[doc = " BFLOAT16 and INT8 extensions"]
    pub fn cblas_sbstobf16(
        n: blas_int,
        in_: *const f32,
        incin: blas_int,
        out: *mut bfloat16,
        incout: blas_int,
    );
    pub fn cblas_sbdtobf16(
        n: blas_int,
        in_: *const f64,
        incin: blas_int,
        out: *mut bfloat16,
        incout: blas_int,
    );
    pub fn cblas_sbf16tos(
        n: blas_int,
        in_: *const bfloat16,
        incin: blas_int,
        out: *mut f32,
        incout: blas_int,
    );
    pub fn cblas_dbf16tod(
        n: blas_int,
        in_: *const bfloat16,
        incin: blas_int,
        out: *mut f64,
        incout: blas_int,
    );
    pub fn cblas_sbdot(
        n: blas_int,
        x: *const bfloat16,
        incx: blas_int,
        y: *const bfloat16,
        incy: blas_int,
    ) -> f32;
    pub fn cblas_sbgemv(
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
    );
    pub fn cblas_sbgemm(
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
    );
    pub fn cblas_sbgemm_batch(
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
    );
}
unsafe extern "C" {
    pub fn omp_set_num_threads(arg1: c_int);
    pub fn omp_get_max_threads() -> c_int;
}
