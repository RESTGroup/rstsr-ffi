//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn LAPACKE_xerbla(name: *const c_char, info: lapack_int) {
    dyload_lib().LAPACKE_xerbla.unwrap()(name, info)
}

pub unsafe fn LAPACKE_lsame(ca: c_char, cb: c_char) -> lapack_int {
    dyload_lib().LAPACKE_lsame.unwrap()(ca, cb)
}

pub unsafe fn LAPACKE_cgb_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_cgb_trans.unwrap()(matrix_layout, m, n, kl, ku, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_cge_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_cge_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_cgg_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_cgg_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_chb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_chb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_che_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_che_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_chp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_chp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_chs_trans(
    matrix_layout: c_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_chs_trans.unwrap()(matrix_layout, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_cpb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_cpb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_cpf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_cpf_trans.unwrap()(matrix_layout, transr, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_cpo_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_cpo_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_cpp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_cpp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_csp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_csp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_csy_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_csy_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ctb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ctb_trans.unwrap()(matrix_layout, uplo, diag, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ctf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_ctf_trans.unwrap()(matrix_layout, transr, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_ctp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    out: *mut __BindgenComplex<f32>,
) {
    dyload_lib().LAPACKE_ctp_trans.unwrap()(matrix_layout, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_ctr_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f32>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f32>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ctr_trans.unwrap()(matrix_layout, uplo, diag, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ctz_trans(
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
) {
    dyload_lib().LAPACKE_ctz_trans.unwrap()(
        matrix_layout,
        direct,
        uplo,
        diag,
        m,
        n,
        in_,
        ldin,
        out,
        ldout,
    )
}

pub unsafe fn LAPACKE_dgb_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dgb_trans.unwrap()(matrix_layout, m, n, kl, ku, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dge_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dge_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dgg_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dgg_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dhs_trans(
    matrix_layout: c_int,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dhs_trans.unwrap()(matrix_layout, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dpb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dpb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dpf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    n: lapack_int,
    in_: *const f64,
    out: *mut f64,
) {
    dyload_lib().LAPACKE_dpf_trans.unwrap()(matrix_layout, transr, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_dpo_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dpo_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dpp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f64,
    out: *mut f64,
) {
    dyload_lib().LAPACKE_dpp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_dsb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dsb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dsp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f64,
    out: *mut f64,
) {
    dyload_lib().LAPACKE_dsp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_dsy_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dsy_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dtb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dtb_trans.unwrap()(matrix_layout, uplo, diag, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dtf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f64,
    out: *mut f64,
) {
    dyload_lib().LAPACKE_dtf_trans.unwrap()(matrix_layout, transr, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_dtp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f64,
    out: *mut f64,
) {
    dyload_lib().LAPACKE_dtp_trans.unwrap()(matrix_layout, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_dtr_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f64,
    ldin: lapack_int,
    out: *mut f64,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_dtr_trans.unwrap()(matrix_layout, uplo, diag, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_dtz_trans(
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
) {
    dyload_lib().LAPACKE_dtz_trans.unwrap()(
        matrix_layout,
        direct,
        uplo,
        diag,
        m,
        n,
        in_,
        ldin,
        out,
        ldout,
    )
}

pub unsafe fn LAPACKE_sgb_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_sgb_trans.unwrap()(matrix_layout, m, n, kl, ku, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_sge_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_sge_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_sgg_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_sgg_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_shs_trans(
    matrix_layout: c_int,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_shs_trans.unwrap()(matrix_layout, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_spb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_spb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_spf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    n: lapack_int,
    in_: *const f32,
    out: *mut f32,
) {
    dyload_lib().LAPACKE_spf_trans.unwrap()(matrix_layout, transr, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_spo_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_spo_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_spp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f32,
    out: *mut f32,
) {
    dyload_lib().LAPACKE_spp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_ssb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ssb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ssp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f32,
    out: *mut f32,
) {
    dyload_lib().LAPACKE_ssp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_ssy_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ssy_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_stb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_stb_trans.unwrap()(matrix_layout, uplo, diag, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_stf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f32,
    out: *mut f32,
) {
    dyload_lib().LAPACKE_stf_trans.unwrap()(matrix_layout, transr, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_stp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f32,
    out: *mut f32,
) {
    dyload_lib().LAPACKE_stp_trans.unwrap()(matrix_layout, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_str_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const f32,
    ldin: lapack_int,
    out: *mut f32,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_str_trans.unwrap()(matrix_layout, uplo, diag, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_stz_trans(
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
) {
    dyload_lib().LAPACKE_stz_trans.unwrap()(
        matrix_layout,
        direct,
        uplo,
        diag,
        m,
        n,
        in_,
        ldin,
        out,
        ldout,
    )
}

pub unsafe fn LAPACKE_zgb_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zgb_trans.unwrap()(matrix_layout, m, n, kl, ku, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zge_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zge_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zgg_trans(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zgg_trans.unwrap()(matrix_layout, m, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zhb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zhb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zhe_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zhe_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zhp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_zhp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_zhs_trans(
    matrix_layout: c_int,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zhs_trans.unwrap()(matrix_layout, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zpb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zpb_trans.unwrap()(matrix_layout, uplo, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zpf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_zpf_trans.unwrap()(matrix_layout, transr, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_zpo_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zpo_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_zpp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_zpp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_zsp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_zsp_trans.unwrap()(matrix_layout, uplo, n, in_, out)
}

pub unsafe fn LAPACKE_zsy_trans(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_zsy_trans.unwrap()(matrix_layout, uplo, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ztb_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ztb_trans.unwrap()(matrix_layout, uplo, diag, n, kd, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ztf_trans(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_ztf_trans.unwrap()(matrix_layout, transr, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_ztp_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    out: *mut __BindgenComplex<f64>,
) {
    dyload_lib().LAPACKE_ztp_trans.unwrap()(matrix_layout, uplo, diag, n, in_, out)
}

pub unsafe fn LAPACKE_ztr_trans(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    in_: *const __BindgenComplex<f64>,
    ldin: lapack_int,
    out: *mut __BindgenComplex<f64>,
    ldout: lapack_int,
) {
    dyload_lib().LAPACKE_ztr_trans.unwrap()(matrix_layout, uplo, diag, n, in_, ldin, out, ldout)
}

pub unsafe fn LAPACKE_ztz_trans(
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
) {
    dyload_lib().LAPACKE_ztz_trans.unwrap()(
        matrix_layout,
        direct,
        uplo,
        diag,
        m,
        n,
        in_,
        ldin,
        out,
        ldout,
    )
}

pub unsafe fn LAPACKE_c_nancheck(
    n: lapack_int,
    x: *const __BindgenComplex<f32>,
    incx: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_c_nancheck.unwrap()(n, x, incx)
}

pub unsafe fn LAPACKE_d_nancheck(n: lapack_int, x: *const f64, incx: lapack_int) -> lapack_int {
    dyload_lib().LAPACKE_d_nancheck.unwrap()(n, x, incx)
}

pub unsafe fn LAPACKE_s_nancheck(n: lapack_int, x: *const f32, incx: lapack_int) -> lapack_int {
    dyload_lib().LAPACKE_s_nancheck.unwrap()(n, x, incx)
}

pub unsafe fn LAPACKE_z_nancheck(
    n: lapack_int,
    x: *const __BindgenComplex<f64>,
    incx: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_z_nancheck.unwrap()(n, x, incx)
}

pub unsafe fn LAPACKE_cgb_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    ab: *const __BindgenComplex<f32>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_cgb_nancheck.unwrap()(matrix_layout, m, n, kl, ku, ab, ldab)
}

pub unsafe fn LAPACKE_cge_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_cge_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_cgg_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_cgg_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_cgt_nancheck(
    n: lapack_int,
    dl: *const __BindgenComplex<f32>,
    d: *const __BindgenComplex<f32>,
    du: *const __BindgenComplex<f32>,
) -> lapack_int {
    dyload_lib().LAPACKE_cgt_nancheck.unwrap()(n, dl, d, du)
}

pub unsafe fn LAPACKE_chb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f32>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_chb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_che_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_che_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_chp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int {
    dyload_lib().LAPACKE_chp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_chs_nancheck(
    matrix_layout: c_int,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_chs_nancheck.unwrap()(matrix_layout, n, a, lda)
}

pub unsafe fn LAPACKE_cpb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f32>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_cpb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_cpf_nancheck(n: lapack_int, a: *const __BindgenComplex<f32>) -> lapack_int {
    dyload_lib().LAPACKE_cpf_nancheck.unwrap()(n, a)
}

pub unsafe fn LAPACKE_cpo_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_cpo_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_cpp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int {
    dyload_lib().LAPACKE_cpp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_cpt_nancheck(
    n: lapack_int,
    d: *const f32,
    e: *const __BindgenComplex<f32>,
) -> lapack_int {
    dyload_lib().LAPACKE_cpt_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_csp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f32>) -> lapack_int {
    dyload_lib().LAPACKE_csp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_cst_nancheck(
    n: lapack_int,
    d: *const __BindgenComplex<f32>,
    e: *const __BindgenComplex<f32>,
) -> lapack_int {
    dyload_lib().LAPACKE_cst_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_csy_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_csy_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_ctb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f32>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ctb_nancheck.unwrap()(matrix_layout, uplo, diag, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_ctf_nancheck(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
) -> lapack_int {
    dyload_lib().LAPACKE_ctf_nancheck.unwrap()(matrix_layout, transr, uplo, diag, n, a)
}

pub unsafe fn LAPACKE_ctp_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    ap: *const __BindgenComplex<f32>,
) -> lapack_int {
    dyload_lib().LAPACKE_ctp_nancheck.unwrap()(matrix_layout, uplo, diag, n, ap)
}

pub unsafe fn LAPACKE_ctr_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ctr_nancheck.unwrap()(matrix_layout, uplo, diag, n, a, lda)
}

pub unsafe fn LAPACKE_ctz_nancheck(
    matrix_layout: c_int,
    direct: c_char,
    uplo: c_char,
    diag: c_char,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f32>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ctz_nancheck.unwrap()(matrix_layout, direct, uplo, diag, m, n, a, lda)
}

pub unsafe fn LAPACKE_dgb_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    ab: *const f64,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dgb_nancheck.unwrap()(matrix_layout, m, n, kl, ku, ab, ldab)
}

pub unsafe fn LAPACKE_dge_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dge_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_dgg_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dgg_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_dgt_nancheck(
    n: lapack_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
) -> lapack_int {
    dyload_lib().LAPACKE_dgt_nancheck.unwrap()(n, dl, d, du)
}

pub unsafe fn LAPACKE_dhs_nancheck(
    matrix_layout: c_int,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dhs_nancheck.unwrap()(matrix_layout, n, a, lda)
}

pub unsafe fn LAPACKE_dpb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f64,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dpb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_dpf_nancheck(n: lapack_int, a: *const f64) -> lapack_int {
    dyload_lib().LAPACKE_dpf_nancheck.unwrap()(n, a)
}

pub unsafe fn LAPACKE_dpo_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dpo_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_dpp_nancheck(n: lapack_int, ap: *const f64) -> lapack_int {
    dyload_lib().LAPACKE_dpp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_dpt_nancheck(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int {
    dyload_lib().LAPACKE_dpt_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_dsb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f64,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dsb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_dsp_nancheck(n: lapack_int, ap: *const f64) -> lapack_int {
    dyload_lib().LAPACKE_dsp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_dst_nancheck(n: lapack_int, d: *const f64, e: *const f64) -> lapack_int {
    dyload_lib().LAPACKE_dst_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_dsy_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dsy_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_dtb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f64,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dtb_nancheck.unwrap()(matrix_layout, uplo, diag, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_dtf_nancheck(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const f64,
) -> lapack_int {
    dyload_lib().LAPACKE_dtf_nancheck.unwrap()(matrix_layout, transr, uplo, diag, n, a)
}

pub unsafe fn LAPACKE_dtp_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    ap: *const f64,
) -> lapack_int {
    dyload_lib().LAPACKE_dtp_nancheck.unwrap()(matrix_layout, uplo, diag, n, ap)
}

pub unsafe fn LAPACKE_dtr_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dtr_nancheck.unwrap()(matrix_layout, uplo, diag, n, a, lda)
}

pub unsafe fn LAPACKE_dtz_nancheck(
    matrix_layout: c_int,
    direct: c_char,
    uplo: c_char,
    diag: c_char,
    m: lapack_int,
    n: lapack_int,
    a: *const f64,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_dtz_nancheck.unwrap()(matrix_layout, direct, uplo, diag, m, n, a, lda)
}

pub unsafe fn LAPACKE_sgb_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    ab: *const f32,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_sgb_nancheck.unwrap()(matrix_layout, m, n, kl, ku, ab, ldab)
}

pub unsafe fn LAPACKE_sge_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_sge_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_sgg_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_sgg_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_sgt_nancheck(
    n: lapack_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
) -> lapack_int {
    dyload_lib().LAPACKE_sgt_nancheck.unwrap()(n, dl, d, du)
}

pub unsafe fn LAPACKE_shs_nancheck(
    matrix_layout: c_int,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_shs_nancheck.unwrap()(matrix_layout, n, a, lda)
}

pub unsafe fn LAPACKE_spb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f32,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_spb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_spf_nancheck(n: lapack_int, a: *const f32) -> lapack_int {
    dyload_lib().LAPACKE_spf_nancheck.unwrap()(n, a)
}

pub unsafe fn LAPACKE_spo_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_spo_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_spp_nancheck(n: lapack_int, ap: *const f32) -> lapack_int {
    dyload_lib().LAPACKE_spp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_spt_nancheck(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int {
    dyload_lib().LAPACKE_spt_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_ssb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f32,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ssb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_ssp_nancheck(n: lapack_int, ap: *const f32) -> lapack_int {
    dyload_lib().LAPACKE_ssp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_sst_nancheck(n: lapack_int, d: *const f32, e: *const f32) -> lapack_int {
    dyload_lib().LAPACKE_sst_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_ssy_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ssy_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_stb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const f32,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_stb_nancheck.unwrap()(matrix_layout, uplo, diag, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_stf_nancheck(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const f32,
) -> lapack_int {
    dyload_lib().LAPACKE_stf_nancheck.unwrap()(matrix_layout, transr, uplo, diag, n, a)
}

pub unsafe fn LAPACKE_stp_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    ap: *const f32,
) -> lapack_int {
    dyload_lib().LAPACKE_stp_nancheck.unwrap()(matrix_layout, uplo, diag, n, ap)
}

pub unsafe fn LAPACKE_str_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_str_nancheck.unwrap()(matrix_layout, uplo, diag, n, a, lda)
}

pub unsafe fn LAPACKE_stz_nancheck(
    matrix_layout: c_int,
    direct: c_char,
    uplo: c_char,
    diag: c_char,
    m: lapack_int,
    n: lapack_int,
    a: *const f32,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_stz_nancheck.unwrap()(matrix_layout, direct, uplo, diag, m, n, a, lda)
}

pub unsafe fn LAPACKE_zgb_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    kl: lapack_int,
    ku: lapack_int,
    ab: *const __BindgenComplex<f64>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zgb_nancheck.unwrap()(matrix_layout, m, n, kl, ku, ab, ldab)
}

pub unsafe fn LAPACKE_zge_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zge_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_zgg_nancheck(
    matrix_layout: c_int,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zgg_nancheck.unwrap()(matrix_layout, m, n, a, lda)
}

pub unsafe fn LAPACKE_zgt_nancheck(
    n: lapack_int,
    dl: *const __BindgenComplex<f64>,
    d: *const __BindgenComplex<f64>,
    du: *const __BindgenComplex<f64>,
) -> lapack_int {
    dyload_lib().LAPACKE_zgt_nancheck.unwrap()(n, dl, d, du)
}

pub unsafe fn LAPACKE_zhb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f64>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zhb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_zhe_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zhe_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_zhp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int {
    dyload_lib().LAPACKE_zhp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_zhs_nancheck(
    matrix_layout: c_int,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zhs_nancheck.unwrap()(matrix_layout, n, a, lda)
}

pub unsafe fn LAPACKE_zpb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f64>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zpb_nancheck.unwrap()(matrix_layout, uplo, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_zpf_nancheck(n: lapack_int, a: *const __BindgenComplex<f64>) -> lapack_int {
    dyload_lib().LAPACKE_zpf_nancheck.unwrap()(n, a)
}

pub unsafe fn LAPACKE_zpo_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zpo_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_zpp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int {
    dyload_lib().LAPACKE_zpp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_zpt_nancheck(
    n: lapack_int,
    d: *const f64,
    e: *const __BindgenComplex<f64>,
) -> lapack_int {
    dyload_lib().LAPACKE_zpt_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_zsp_nancheck(n: lapack_int, ap: *const __BindgenComplex<f64>) -> lapack_int {
    dyload_lib().LAPACKE_zsp_nancheck.unwrap()(n, ap)
}

pub unsafe fn LAPACKE_zst_nancheck(
    n: lapack_int,
    d: *const __BindgenComplex<f64>,
    e: *const __BindgenComplex<f64>,
) -> lapack_int {
    dyload_lib().LAPACKE_zst_nancheck.unwrap()(n, d, e)
}

pub unsafe fn LAPACKE_zsy_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_zsy_nancheck.unwrap()(matrix_layout, uplo, n, a, lda)
}

pub unsafe fn LAPACKE_ztb_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    kd: lapack_int,
    ab: *const __BindgenComplex<f64>,
    ldab: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ztb_nancheck.unwrap()(matrix_layout, uplo, diag, n, kd, ab, ldab)
}

pub unsafe fn LAPACKE_ztf_nancheck(
    matrix_layout: c_int,
    transr: c_char,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
) -> lapack_int {
    dyload_lib().LAPACKE_ztf_nancheck.unwrap()(matrix_layout, transr, uplo, diag, n, a)
}

pub unsafe fn LAPACKE_ztp_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    ap: *const __BindgenComplex<f64>,
) -> lapack_int {
    dyload_lib().LAPACKE_ztp_nancheck.unwrap()(matrix_layout, uplo, diag, n, ap)
}

pub unsafe fn LAPACKE_ztr_nancheck(
    matrix_layout: c_int,
    uplo: c_char,
    diag: c_char,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ztr_nancheck.unwrap()(matrix_layout, uplo, diag, n, a, lda)
}

pub unsafe fn LAPACKE_ztz_nancheck(
    matrix_layout: c_int,
    direct: c_char,
    uplo: c_char,
    diag: c_char,
    m: lapack_int,
    n: lapack_int,
    a: *const __BindgenComplex<f64>,
    lda: lapack_int,
) -> lapack_int {
    dyload_lib().LAPACKE_ztz_nancheck.unwrap()(matrix_layout, direct, uplo, diag, m, n, a, lda)
}
