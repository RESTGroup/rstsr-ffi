//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn LAPACKE_xerbla(name: *const c_char, info: lapack_int);
    pub fn LAPACKE_lsame(ca: c_char, cb: c_char) -> lapack_int;
    pub fn LAPACKE_cgb_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_cge_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_cgg_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_chb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_che_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_chp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_chs_trans(
        matrix_layout: c_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_cpb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_cpf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_cpo_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_cpp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_csp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_csy_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ctb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ctf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_ctp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        out: *mut __BindgenComplex<f32>,
    );
    pub fn LAPACKE_ctr_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ctz_trans(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f32>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f32>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dgb_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dge_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dgg_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dhs_trans(
        matrix_layout: c_int,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dpb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dpf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        n: lapack_int,
        in_: *const f64,
        out: *mut f64,
    );
    pub fn LAPACKE_dpo_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dpp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f64,
        out: *mut f64,
    );
    pub fn LAPACKE_dsb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dsp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f64,
        out: *mut f64,
    );
    pub fn LAPACKE_dsy_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dtb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dtf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f64,
        out: *mut f64,
    );
    pub fn LAPACKE_dtp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f64,
        out: *mut f64,
    );
    pub fn LAPACKE_dtr_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_dtz_trans(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        in_: *const f64,
        ldin: lapack_int,
        out: *mut f64,
        ldout: lapack_int,
    );
    pub fn LAPACKE_sgb_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_sge_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_sgg_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_shs_trans(
        matrix_layout: c_int,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_spb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_spf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        n: lapack_int,
        in_: *const f32,
        out: *mut f32,
    );
    pub fn LAPACKE_spo_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_spp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f32,
        out: *mut f32,
    );
    pub fn LAPACKE_ssb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ssp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f32,
        out: *mut f32,
    );
    pub fn LAPACKE_ssy_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_stb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_stf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f32,
        out: *mut f32,
    );
    pub fn LAPACKE_stp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f32,
        out: *mut f32,
    );
    pub fn LAPACKE_str_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_stz_trans(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        in_: *const f32,
        ldin: lapack_int,
        out: *mut f32,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zgb_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zge_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zgg_trans(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zhb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zhe_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zhp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_zhs_trans(
        matrix_layout: c_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zpb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zpf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_zpo_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_zpp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_zsp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_zsy_trans(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ztb_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ztf_trans(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_ztp_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        out: *mut __BindgenComplex<f64>,
    );
    pub fn LAPACKE_ztr_trans(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_ztz_trans(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        in_: *const __BindgenComplex<f64>,
        ldin: lapack_int,
        out: *mut __BindgenComplex<f64>,
        ldout: lapack_int,
    );
    pub fn LAPACKE_c_nancheck(
        n: lapack_int,
        x: *const __BindgenComplex<f32>,
        incx: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_d_nancheck(n: lapack_int, x: *const f64, incx: lapack_int) -> lapack_int;
    pub fn LAPACKE_s_nancheck(n: lapack_int, x: *const f32, incx: lapack_int) -> lapack_int;
    pub fn LAPACKE_z_nancheck(
        n: lapack_int,
        x: *const __BindgenComplex<f64>,
        incx: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cgb_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        ab: *const __BindgenComplex<f32>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cge_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cgg_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cgt_nancheck(
        n: lapack_int,
        dl: *const __BindgenComplex<f32>,
        d: *const __BindgenComplex<f32>,
        du: *const __BindgenComplex<f32>,
    ) -> lapack_int;
    pub fn LAPACKE_chb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f32>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_che_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_chp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int;
    pub fn LAPACKE_chs_nancheck(
        matrix_layout: c_int,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cpb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f32>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cpf_nancheck(n: lapack_int, a: *const __BindgenComplex<f32>) -> lapack_int;
    pub fn LAPACKE_cpo_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_cpp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int;
    pub fn LAPACKE_cpt_nancheck(
        n: lapack_int,
        d: *const f32,
        e: *const __BindgenComplex<f32>,
    ) -> lapack_int;
    pub fn LAPACKE_csp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int;
    pub fn LAPACKE_cst_nancheck(
        n: lapack_int,
        d: *const __BindgenComplex<f32>,
        e: *const __BindgenComplex<f32>,
    ) -> lapack_int;
    pub fn LAPACKE_csy_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ctb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f32>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ctf_nancheck(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
    ) -> lapack_int;
    pub fn LAPACKE_ctp_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        ap: *const __BindgenComplex<f32>,
    ) -> lapack_int;
    pub fn LAPACKE_ctr_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ctz_nancheck(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f32>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dgb_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        ab: *const f64,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dge_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dgg_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dgt_nancheck(
        n: lapack_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
    ) -> lapack_int;
    pub fn LAPACKE_dhs_nancheck(
        matrix_layout: c_int,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dpb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f64,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dpf_nancheck(n: lapack_int, a: *const f64) -> lapack_int;
    pub fn LAPACKE_dpo_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dpp_nancheck(n: lapack_int, ap: *const f64) -> lapack_int;
    pub fn LAPACKE_dpt_nancheck(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int;
    pub fn LAPACKE_dsb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f64,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dsp_nancheck(n: lapack_int, ap: *const f64) -> lapack_int;
    pub fn LAPACKE_dst_nancheck(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int;
    pub fn LAPACKE_dsy_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dtb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f64,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dtf_nancheck(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const f64,
    ) -> lapack_int;
    pub fn LAPACKE_dtp_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        ap: *const f64,
    ) -> lapack_int;
    pub fn LAPACKE_dtr_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_dtz_nancheck(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        a: *const f64,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_sgb_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        ab: *const f32,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_sge_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_sgg_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_sgt_nancheck(
        n: lapack_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
    ) -> lapack_int;
    pub fn LAPACKE_shs_nancheck(
        matrix_layout: c_int,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_spb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f32,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_spf_nancheck(n: lapack_int, a: *const f32) -> lapack_int;
    pub fn LAPACKE_spo_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_spp_nancheck(n: lapack_int, ap: *const f32) -> lapack_int;
    pub fn LAPACKE_spt_nancheck(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int;
    pub fn LAPACKE_ssb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f32,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ssp_nancheck(n: lapack_int, ap: *const f32) -> lapack_int;
    pub fn LAPACKE_sst_nancheck(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int;
    pub fn LAPACKE_ssy_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_stb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const f32,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_stf_nancheck(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const f32,
    ) -> lapack_int;
    pub fn LAPACKE_stp_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        ap: *const f32,
    ) -> lapack_int;
    pub fn LAPACKE_str_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_stz_nancheck(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        a: *const f32,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zgb_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        kl: lapack_int,
        ku: lapack_int,
        ab: *const __BindgenComplex<f64>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zge_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zgg_nancheck(
        matrix_layout: c_int,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zgt_nancheck(
        n: lapack_int,
        dl: *const __BindgenComplex<f64>,
        d: *const __BindgenComplex<f64>,
        du: *const __BindgenComplex<f64>,
    ) -> lapack_int;
    pub fn LAPACKE_zhb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f64>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zhe_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zhp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int;
    pub fn LAPACKE_zhs_nancheck(
        matrix_layout: c_int,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zpb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f64>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zpf_nancheck(n: lapack_int, a: *const __BindgenComplex<f64>) -> lapack_int;
    pub fn LAPACKE_zpo_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_zpp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int;
    pub fn LAPACKE_zpt_nancheck(
        n: lapack_int,
        d: *const f64,
        e: *const __BindgenComplex<f64>,
    ) -> lapack_int;
    pub fn LAPACKE_zsp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int;
    pub fn LAPACKE_zst_nancheck(
        n: lapack_int,
        d: *const __BindgenComplex<f64>,
        e: *const __BindgenComplex<f64>,
    ) -> lapack_int;
    pub fn LAPACKE_zsy_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ztb_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        kd: lapack_int,
        ab: *const __BindgenComplex<f64>,
        ldab: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ztf_nancheck(
        matrix_layout: c_int,
        transr: c_char,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
    ) -> lapack_int;
    pub fn LAPACKE_ztp_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        ap: *const __BindgenComplex<f64>,
    ) -> lapack_int;
    pub fn LAPACKE_ztr_nancheck(
        matrix_layout: c_int,
        uplo: c_char,
        diag: c_char,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
    pub fn LAPACKE_ztz_nancheck(
        matrix_layout: c_int,
        direct: c_char,
        uplo: c_char,
        diag: c_char,
        m: lapack_int,
        n: lapack_int,
        a: *const __BindgenComplex<f64>,
        lda: lapack_int,
    ) -> lapack_int;
}
