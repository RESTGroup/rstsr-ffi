//! Library struct definition for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

pub struct DyLoadLib {
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub LAPACKE_xerbla: Option<unsafe extern "C" fn(name: *const c_char, info: lapack_int)>,
    pub LAPACKE_lsame: Option<unsafe extern "C" fn(ca: c_char, cb: c_char) -> lapack_int>,
    pub LAPACKE_cgb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_cge_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_cgg_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_chb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_che_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_chp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_chs_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_cpb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_cpf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_cpo_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_cpp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_csp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_csy_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ctb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ctf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_ctp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            out: *mut __BindgenComplex<f32>,
        ),
    >,
    pub LAPACKE_ctr_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f32>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f32>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ctz_trans: Option<
        unsafe extern "C" fn(
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
        ),
    >,
    pub LAPACKE_dgb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dge_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dgg_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dhs_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dpb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dpf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            n: lapack_int,
            in_: *const f64,
            out: *mut f64,
        ),
    >,
    pub LAPACKE_dpo_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dpp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f64,
            out: *mut f64,
        ),
    >,
    pub LAPACKE_dsb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dsp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f64,
            out: *mut f64,
        ),
    >,
    pub LAPACKE_dsy_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dtb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dtf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f64,
            out: *mut f64,
        ),
    >,
    pub LAPACKE_dtp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f64,
            out: *mut f64,
        ),
    >,
    pub LAPACKE_dtr_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f64,
            ldin: lapack_int,
            out: *mut f64,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_dtz_trans: Option<
        unsafe extern "C" fn(
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
        ),
    >,
    pub LAPACKE_sgb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_sge_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_sgg_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_shs_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_spb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_spf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            n: lapack_int,
            in_: *const f32,
            out: *mut f32,
        ),
    >,
    pub LAPACKE_spo_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_spp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f32,
            out: *mut f32,
        ),
    >,
    pub LAPACKE_ssb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ssp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f32,
            out: *mut f32,
        ),
    >,
    pub LAPACKE_ssy_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_stb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_stf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f32,
            out: *mut f32,
        ),
    >,
    pub LAPACKE_stp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f32,
            out: *mut f32,
        ),
    >,
    pub LAPACKE_str_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const f32,
            ldin: lapack_int,
            out: *mut f32,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_stz_trans: Option<
        unsafe extern "C" fn(
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
        ),
    >,
    pub LAPACKE_zgb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zge_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zgg_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zhb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zhe_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zhp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_zhs_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zpb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zpf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_zpo_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_zpp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_zsp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_zsy_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ztb_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ztf_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_ztp_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            out: *mut __BindgenComplex<f64>,
        ),
    >,
    pub LAPACKE_ztr_trans: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            in_: *const __BindgenComplex<f64>,
            ldin: lapack_int,
            out: *mut __BindgenComplex<f64>,
            ldout: lapack_int,
        ),
    >,
    pub LAPACKE_ztz_trans: Option<
        unsafe extern "C" fn(
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
        ),
    >,
    pub LAPACKE_c_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            x: *const __BindgenComplex<f32>,
            incx: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_d_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, x: *const f64, incx: lapack_int) -> lapack_int>,
    pub LAPACKE_s_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, x: *const f32, incx: lapack_int) -> lapack_int>,
    pub LAPACKE_z_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            x: *const __BindgenComplex<f64>,
            incx: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cgb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            ab: *const __BindgenComplex<f32>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cge_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cgg_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cgt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            dl: *const __BindgenComplex<f32>,
            d: *const __BindgenComplex<f32>,
            du: *const __BindgenComplex<f32>,
        ) -> lapack_int,
    >,
    pub LAPACKE_chb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f32>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_che_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_chp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int>,
    pub LAPACKE_chs_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cpb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f32>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cpf_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, a: *const __BindgenComplex<f32>) -> lapack_int>,
    pub LAPACKE_cpo_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_cpp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int>,
    pub LAPACKE_cpt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            d: *const f32,
            e: *const __BindgenComplex<f32>,
        ) -> lapack_int,
    >,
    pub LAPACKE_csp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int>,
    pub LAPACKE_cst_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            d: *const __BindgenComplex<f32>,
            e: *const __BindgenComplex<f32>,
        ) -> lapack_int,
    >,
    pub LAPACKE_csy_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ctb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f32>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ctf_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
        ) -> lapack_int,
    >,
    pub LAPACKE_ctp_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            ap: *const __BindgenComplex<f32>,
        ) -> lapack_int,
    >,
    pub LAPACKE_ctr_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ctz_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            direct: c_char,
            uplo: c_char,
            diag: c_char,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f32>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dgb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            ab: *const f64,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dge_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dgg_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dgt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            dl: *const f64,
            d: *const f64,
            du: *const f64,
        ) -> lapack_int,
    >,
    pub LAPACKE_dhs_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dpb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f64,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dpf_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, a: *const f64) -> lapack_int>,
    pub LAPACKE_dpo_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dpp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const f64) -> lapack_int>,
    pub LAPACKE_dpt_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int>,
    pub LAPACKE_dsb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f64,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dsp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const f64) -> lapack_int>,
    pub LAPACKE_dst_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int>,
    pub LAPACKE_dsy_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dtb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f64,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dtf_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const f64,
        ) -> lapack_int,
    >,
    pub LAPACKE_dtp_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            ap: *const f64,
        ) -> lapack_int,
    >,
    pub LAPACKE_dtr_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_dtz_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            direct: c_char,
            uplo: c_char,
            diag: c_char,
            m: lapack_int,
            n: lapack_int,
            a: *const f64,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_sgb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            ab: *const f32,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_sge_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_sgg_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_sgt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            dl: *const f32,
            d: *const f32,
            du: *const f32,
        ) -> lapack_int,
    >,
    pub LAPACKE_shs_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_spb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f32,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_spf_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, a: *const f32) -> lapack_int>,
    pub LAPACKE_spo_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_spp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const f32) -> lapack_int>,
    pub LAPACKE_spt_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int>,
    pub LAPACKE_ssb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f32,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ssp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const f32) -> lapack_int>,
    pub LAPACKE_sst_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int>,
    pub LAPACKE_ssy_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_stb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const f32,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_stf_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const f32,
        ) -> lapack_int,
    >,
    pub LAPACKE_stp_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            ap: *const f32,
        ) -> lapack_int,
    >,
    pub LAPACKE_str_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_stz_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            direct: c_char,
            uplo: c_char,
            diag: c_char,
            m: lapack_int,
            n: lapack_int,
            a: *const f32,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zgb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            kl: lapack_int,
            ku: lapack_int,
            ab: *const __BindgenComplex<f64>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zge_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zgg_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zgt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            dl: *const __BindgenComplex<f64>,
            d: *const __BindgenComplex<f64>,
            du: *const __BindgenComplex<f64>,
        ) -> lapack_int,
    >,
    pub LAPACKE_zhb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f64>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zhe_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zhp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int>,
    pub LAPACKE_zhs_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zpb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f64>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zpf_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, a: *const __BindgenComplex<f64>) -> lapack_int>,
    pub LAPACKE_zpo_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_zpp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int>,
    pub LAPACKE_zpt_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            d: *const f64,
            e: *const __BindgenComplex<f64>,
        ) -> lapack_int,
    >,
    pub LAPACKE_zsp_nancheck:
        Option<unsafe extern "C" fn(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int>,
    pub LAPACKE_zst_nancheck: Option<
        unsafe extern "C" fn(
            n: lapack_int,
            d: *const __BindgenComplex<f64>,
            e: *const __BindgenComplex<f64>,
        ) -> lapack_int,
    >,
    pub LAPACKE_zsy_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ztb_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            kd: lapack_int,
            ab: *const __BindgenComplex<f64>,
            ldab: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ztf_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            transr: c_char,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
        ) -> lapack_int,
    >,
    pub LAPACKE_ztp_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            ap: *const __BindgenComplex<f64>,
        ) -> lapack_int,
    >,
    pub LAPACKE_ztr_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            uplo: c_char,
            diag: c_char,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
    pub LAPACKE_ztz_nancheck: Option<
        unsafe extern "C" fn(
            matrix_layout: c_int,
            direct: c_char,
            uplo: c_char,
            diag: c_char,
            m: lapack_int,
            n: lapack_int,
            a: *const __BindgenComplex<f64>,
            lda: lapack_int,
        ) -> lapack_int,
    >,
}
