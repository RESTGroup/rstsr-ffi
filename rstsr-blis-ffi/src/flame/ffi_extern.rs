//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn bl1_s2() -> f32;
    pub fn bl1_d2() -> f64;
    pub fn bl1_c2() -> scomplex;
    pub fn bl1_z2() -> dcomplex;
    pub fn bl1_s1() -> f32;
    pub fn bl1_d1() -> f64;
    pub fn bl1_c1() -> scomplex;
    pub fn bl1_z1() -> dcomplex;
    pub fn bl1_s1h() -> f32;
    pub fn bl1_d1h() -> f64;
    pub fn bl1_c1h() -> scomplex;
    pub fn bl1_z1h() -> dcomplex;
    pub fn bl1_s0() -> f32;
    pub fn bl1_d0() -> f64;
    pub fn bl1_c0() -> scomplex;
    pub fn bl1_z0() -> dcomplex;
    pub fn bl1_sm1h() -> f32;
    pub fn bl1_dm1h() -> f64;
    pub fn bl1_cm1h() -> scomplex;
    pub fn bl1_zm1h() -> dcomplex;
    pub fn bl1_sm1() -> f32;
    pub fn bl1_dm1() -> f64;
    pub fn bl1_cm1() -> scomplex;
    pub fn bl1_zm1() -> dcomplex;
    pub fn bl1_sm2() -> f32;
    pub fn bl1_dm2() -> f64;
    pub fn bl1_cm2() -> scomplex;
    pub fn bl1_zm2() -> dcomplex;
    pub fn bl1_vallocv(n_elem: c_uint, elem_size: c_uint) -> *mut c_void;
    pub fn bl1_iallocv(n_elem: c_uint) -> *mut c_int;
    pub fn bl1_sallocv(n_elem: c_uint) -> *mut f32;
    pub fn bl1_dallocv(n_elem: c_uint) -> *mut f64;
    pub fn bl1_callocv(n_elem: c_uint) -> *mut scomplex;
    pub fn bl1_zallocv(n_elem: c_uint) -> *mut dcomplex;
    pub fn bl1_vallocm(m: c_uint, n: c_uint, elem_size: c_uint) -> *mut c_void;
    pub fn bl1_iallocm(m: c_uint, n: c_uint) -> *mut c_int;
    pub fn bl1_sallocm(m: c_uint, n: c_uint) -> *mut f32;
    pub fn bl1_dallocm(m: c_uint, n: c_uint) -> *mut f64;
    pub fn bl1_callocm(m: c_uint, n: c_uint) -> *mut scomplex;
    pub fn bl1_zallocm(m: c_uint, n: c_uint) -> *mut dcomplex;
    pub fn bl1_sapdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dapdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csapdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_capdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut scomplex,
        incx: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdapdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zapdiagmv(
        side: side1_t,
        conj: conj1_t,
        m: c_int,
        n: c_int,
        x: *mut dcomplex,
        incx: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_screate_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dcreate_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_ccreate_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zcreate_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_screate_contigmt(
        trans_dims: trans1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dcreate_contigmt(
        trans_dims: trans1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_ccreate_contigmt(
        trans_dims: trans1_t,
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zcreate_contigmt(
        trans_dims: trans1_t,
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_screate_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dcreate_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_ccreate_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zcreate_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_screate_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dcreate_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_ccreate_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zcreate_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_sfree_contigm(
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dfree_contigm(
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_cfree_contigm(
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zfree_contigm(
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_sfree_saved_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dfree_saved_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_cfree_saved_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zfree_saved_contigm(
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_sfree_saved_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dfree_saved_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_cfree_saved_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zfree_saved_contigmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_sfree_saved_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f32,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f32,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_dfree_saved_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut f64,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut f64,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_cfree_saved_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut scomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut scomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_zfree_saved_contigmsr(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a_save: *mut dcomplex,
        a_rs_save: c_int,
        a_cs_save: c_int,
        a: *mut *mut dcomplex,
        a_rs: *mut c_int,
        a_cs: *mut c_int,
    );
    pub fn bl1_sewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_csewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_cewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zdewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_zewinvscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_csewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zdewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zewinvscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_csewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_cewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zdewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_zewscalv(
        conj: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_csewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zdewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zewscalmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_vfree(p: *mut c_void);
    pub fn bl1_ifree(p: *mut c_int);
    pub fn bl1_sfree(p: *mut f32);
    pub fn bl1_dfree(p: *mut f64);
    pub fn bl1_cfree(p: *mut scomplex);
    pub fn bl1_zfree(p: *mut dcomplex);
    pub fn bl1_sinverts(conj: conj1_t, alpha: *mut f32);
    pub fn bl1_dinverts(conj: conj1_t, alpha: *mut f64);
    pub fn bl1_cinverts(conj: conj1_t, alpha: *mut scomplex);
    pub fn bl1_zinverts(conj: conj1_t, alpha: *mut dcomplex);
    pub fn bl1_sinvert2s(conj: conj1_t, alpha: *mut f32, beta: *mut f32);
    pub fn bl1_dinvert2s(conj: conj1_t, alpha: *mut f64, beta: *mut f64);
    pub fn bl1_cinvert2s(conj: conj1_t, alpha: *mut scomplex, beta: *mut scomplex);
    pub fn bl1_zinvert2s(conj: conj1_t, alpha: *mut dcomplex, beta: *mut dcomplex);
    pub fn bl1_sinvertv(conj: conj1_t, n: c_int, x: *mut f32, incx: c_int);
    pub fn bl1_dinvertv(conj: conj1_t, n: c_int, x: *mut f64, incx: c_int);
    pub fn bl1_cinvertv(conj: conj1_t, n: c_int, x: *mut scomplex, incx: c_int);
    pub fn bl1_zinvertv(conj: conj1_t, n: c_int, x: *mut dcomplex, incx: c_int);
    pub fn bl1_sident(m: c_int, a: *mut f32, a_rs: c_int, a_cs: c_int);
    pub fn bl1_dident(m: c_int, a: *mut f64, a_rs: c_int, a_cs: c_int);
    pub fn bl1_cident(m: c_int, a: *mut scomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_zident(m: c_int, a: *mut dcomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_smaxabsv(n: c_int, x: *mut f32, incx: c_int, maxabs: *mut f32);
    pub fn bl1_dmaxabsv(n: c_int, x: *mut f64, incx: c_int, maxabs: *mut f64);
    pub fn bl1_cmaxabsv(n: c_int, x: *mut scomplex, incx: c_int, maxabs: *mut f32);
    pub fn bl1_zmaxabsv(n: c_int, x: *mut dcomplex, incx: c_int, maxabs: *mut f64);
    pub fn bl1_smaxabsm(
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f32,
    );
    pub fn bl1_dmaxabsm(
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f64,
    );
    pub fn bl1_cmaxabsm(
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f32,
    );
    pub fn bl1_zmaxabsm(
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f64,
    );
    pub fn bl1_smaxabsmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f32,
    );
    pub fn bl1_dmaxabsmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f64,
    );
    pub fn bl1_cmaxabsmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f32,
    );
    pub fn bl1_zmaxabsmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        maxabs: *mut f64,
    );
    pub fn bl1_srands(alpha: *mut f32);
    pub fn bl1_drands(alpha: *mut f64);
    pub fn bl1_crands(alpha: *mut scomplex);
    pub fn bl1_zrands(alpha: *mut dcomplex);
    pub fn bl1_srandv(n: c_int, x: *mut f32, incx: c_int);
    pub fn bl1_drandv(n: c_int, x: *mut f64, incx: c_int);
    pub fn bl1_crandv(n: c_int, x: *mut scomplex, incx: c_int);
    pub fn bl1_zrandv(n: c_int, x: *mut dcomplex, incx: c_int);
    pub fn bl1_srandm(m: c_int, n: c_int, a: *mut f32, a_rs: c_int, a_cs: c_int);
    pub fn bl1_drandm(m: c_int, n: c_int, a: *mut f64, a_rs: c_int, a_cs: c_int);
    pub fn bl1_crandm(m: c_int, n: c_int, a: *mut scomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_zrandm(m: c_int, n: c_int, a: *mut dcomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_srandmr(
        uplo: uplo1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_drandmr(
        uplo: uplo1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_crandmr(
        uplo: uplo1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zrandmr(
        uplo: uplo1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_set_contig_strides(m: c_int, n: c_int, rs: *mut c_int, cs: *mut c_int);
    pub fn bl1_set_dim_with_side(side: side1_t, m: c_int, n: c_int, dim_new: *mut c_int);
    pub fn bl1_set_dims_with_trans(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        m_new: *mut c_int,
        n_new: *mut c_int,
    );
    pub fn bl1_isetv(m: c_int, sigma: *mut c_int, x: *mut c_int, incx: c_int);
    pub fn bl1_ssetv(m: c_int, sigma: *mut f32, x: *mut f32, incx: c_int);
    pub fn bl1_dsetv(m: c_int, sigma: *mut f64, x: *mut f64, incx: c_int);
    pub fn bl1_csetv(m: c_int, sigma: *mut scomplex, x: *mut scomplex, incx: c_int);
    pub fn bl1_zsetv(m: c_int, sigma: *mut dcomplex, x: *mut dcomplex, incx: c_int);
    pub fn bl1_isetm(
        m: c_int,
        n: c_int,
        sigma: *mut c_int,
        a: *mut c_int,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssetm(m: c_int, n: c_int, sigma: *mut f32, a: *mut f32, a_rs: c_int, a_cs: c_int);
    pub fn bl1_dsetm(m: c_int, n: c_int, sigma: *mut f64, a: *mut f64, a_rs: c_int, a_cs: c_int);
    pub fn bl1_csetm(
        m: c_int,
        n: c_int,
        sigma: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsetm(
        m: c_int,
        n: c_int,
        sigma: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssetmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dsetmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csetmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        sigma: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsetmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        sigma: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_isetdiag(
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut c_int,
        a: *mut c_int,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssetdiag(
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dsetdiag(
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csetdiag(
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsetdiag(
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_sscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdscalediag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_sshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdshiftdiag(
        conj: conj1_t,
        offset: c_int,
        m: c_int,
        n: c_int,
        sigma: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssymmize(
        conj: conj1_t,
        uplo: uplo1_t,
        m: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dsymmize(
        conj: conj1_t,
        uplo: uplo1_t,
        m: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csymmize(
        conj: conj1_t,
        uplo: uplo1_t,
        m: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsymmize(
        conj: conj1_t,
        uplo: uplo1_t,
        m: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_does_trans(trans: trans1_t) -> c_int;
    pub fn bl1_does_notrans(trans: trans1_t) -> c_int;
    pub fn bl1_does_conj(trans: trans1_t) -> c_int;
    pub fn bl1_is_notrans(trans: trans1_t) -> c_int;
    pub fn bl1_is_trans(trans: trans1_t) -> c_int;
    pub fn bl1_is_conjnotrans(trans: trans1_t) -> c_int;
    pub fn bl1_is_conjtrans(trans: trans1_t) -> c_int;
    pub fn bl1_is_noconj(conj: conj1_t) -> c_int;
    pub fn bl1_is_conj(conj: conj1_t) -> c_int;
    pub fn bl1_is_lower(uplo: uplo1_t) -> c_int;
    pub fn bl1_is_upper(uplo: uplo1_t) -> c_int;
    pub fn bl1_is_left(side: side1_t) -> c_int;
    pub fn bl1_is_right(side: side1_t) -> c_int;
    pub fn bl1_is_nonunit_diag(diag: diag1_t) -> c_int;
    pub fn bl1_is_unit_diag(diag: diag1_t) -> c_int;
    pub fn bl1_is_zero_diag(diag: diag1_t) -> c_int;
    pub fn bl1_proj_trans1_to_conj(trans: trans1_t) -> conj1_t;
    pub fn bl1_check_storage_3m(
        a_rs: c_int,
        a_cs: c_int,
        b_rs: c_int,
        b_cs: c_int,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_check_storage_2m(a_rs: c_int, a_cs: c_int, b_rs: c_int, b_cs: c_int);
    pub fn bl1_is_row_or_col_storage(rs: c_int, cs: c_int) -> c_int;
    pub fn bl1_is_row_storage(rs: c_int, cs: c_int) -> c_int;
    pub fn bl1_is_col_storage(rs: c_int, cs: c_int) -> c_int;
    pub fn bl1_is_gen_storage(rs: c_int, cs: c_int) -> c_int;
    pub fn bl1_is_vector(m: c_int, n: c_int) -> c_int;
    pub fn bl1_vector_dim(m: c_int, n: c_int) -> c_int;
    pub fn bl1_vector_inc(trans: trans1_t, m: c_int, n: c_int, rs: c_int, cs: c_int) -> c_int;
    pub fn bl1_zero_dim1(m: c_int) -> c_int;
    pub fn bl1_zero_dim2(m: c_int, n: c_int) -> c_int;
    pub fn bl1_zero_dim3(m: c_int, k: c_int, n: c_int) -> c_int;
    pub fn bl1_abort();
    pub fn bl1_abort_msg(message: *mut c_char);
    pub fn bl1_param_map_to_netlib_trans(blis_trans: trans1_t, blas_trans: *mut c_void);
    pub fn bl1_param_map_to_netlib_uplo(blis_uplo: uplo1_t, blas_uplo: *mut c_void);
    pub fn bl1_param_map_to_netlib_side(blis_side: side1_t, blas_side: *mut c_void);
    pub fn bl1_param_map_to_netlib_diag(blis_diag: diag1_t, blas_diag: *mut c_void);
    pub fn bl1_samax(n: c_int, x: *mut f32, incx: c_int, index: *mut c_int);
    pub fn bl1_damax(n: c_int, x: *mut f64, incx: c_int, index: *mut c_int);
    pub fn bl1_camax(n: c_int, x: *mut scomplex, incx: c_int, index: *mut c_int);
    pub fn bl1_zamax(n: c_int, x: *mut dcomplex, incx: c_int, index: *mut c_int);
    pub fn bl1_sasum(n: c_int, x: *mut f32, incx: c_int, norm: *mut f32);
    pub fn bl1_dasum(n: c_int, x: *mut f64, incx: c_int, norm: *mut f64);
    pub fn bl1_casum(n: c_int, x: *mut scomplex, incx: c_int, norm: *mut f32);
    pub fn bl1_zasum(n: c_int, x: *mut dcomplex, incx: c_int, norm: *mut f64);
    pub fn bl1_saxpy(n: c_int, alpha: *mut f32, x: *mut f32, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_daxpy(n: c_int, alpha: *mut f64, x: *mut f64, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_caxpy(
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zaxpy(
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_saxpyv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_daxpyv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_caxpyv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zaxpyv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_saxpymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_daxpymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_caxpymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zaxpymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_saxpymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_daxpymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_caxpymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zaxpymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_saxpysv(
        n: c_int,
        alpha0: *mut f32,
        alpha1: *mut f32,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_daxpysv(
        n: c_int,
        alpha0: *mut f64,
        alpha1: *mut f64,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_caxpysv(
        n: c_int,
        alpha0: *mut scomplex,
        alpha1: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zaxpysv(
        n: c_int,
        alpha0: *mut dcomplex,
        alpha1: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_saxpysmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha0: *mut f32,
        alpha1: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f32,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_daxpysmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha0: *mut f64,
        alpha1: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f64,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_caxpysmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha0: *mut scomplex,
        alpha1: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut scomplex,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zaxpysmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        alpha0: *mut dcomplex,
        alpha1: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut dcomplex,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sconjv(m: c_int, x: *mut f32, incx: c_int);
    pub fn bl1_dconjv(m: c_int, x: *mut f64, incx: c_int);
    pub fn bl1_cconjv(m: c_int, x: *mut scomplex, incx: c_int);
    pub fn bl1_zconjv(m: c_int, x: *mut dcomplex, incx: c_int);
    pub fn bl1_sconjm(m: c_int, n: c_int, a: *mut f32, a_rs: c_int, a_cs: c_int);
    pub fn bl1_dconjm(m: c_int, n: c_int, a: *mut f64, a_rs: c_int, a_cs: c_int);
    pub fn bl1_cconjm(m: c_int, n: c_int, a: *mut scomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_zconjm(m: c_int, n: c_int, a: *mut dcomplex, a_rs: c_int, a_cs: c_int);
    pub fn bl1_sconjmr(uplo: uplo1_t, m: c_int, n: c_int, a: *mut f32, a_rs: c_int, a_cs: c_int);
    pub fn bl1_dconjmr(uplo: uplo1_t, m: c_int, n: c_int, a: *mut f64, a_rs: c_int, a_cs: c_int);
    pub fn bl1_cconjmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zconjmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_scopy(m: c_int, x: *mut f32, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_dcopy(m: c_int, x: *mut f64, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_ccopy(m: c_int, x: *mut scomplex, incx: c_int, y: *mut scomplex, incy: c_int);
    pub fn bl1_zcopy(m: c_int, x: *mut dcomplex, incx: c_int, y: *mut dcomplex, incy: c_int);
    pub fn bl1_icopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut c_int,
        incx: c_int,
        y: *mut c_int,
        incy: c_int,
    );
    pub fn bl1_scopyv(conj: conj1_t, m: c_int, x: *mut f32, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_dcopyv(conj: conj1_t, m: c_int, x: *mut f64, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_ccopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sdcopyv(conj: conj1_t, m: c_int, x: *mut f32, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_dscopyv(conj: conj1_t, m: c_int, x: *mut f64, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_sccopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_cscopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_szcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_zscopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dccopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_cdcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_dzcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_zdcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_czcopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_zccopyv(
        conj: conj1_t,
        m: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_scopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ccopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sscopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sdcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dscopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sccopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cscopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_szcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zscopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ddcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dccopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cdcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dzcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zdcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cccopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_czcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zccopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zzcopymr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_scopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ccopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sscopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sdcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sccopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_szcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dscopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ddcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dccopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dzcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cscopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cdcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cccopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_czcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zscopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zdcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zccopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zzcopymrt(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_icopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut c_int,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut c_int,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_scopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ccopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sscopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sdcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dscopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sccopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cscopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_szcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zscopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ddcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dccopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cdcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dzcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zdcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cccopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_czcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zccopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zzcopymt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cdot_in(
        conj: conj1_t,
        n: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        rho: *mut scomplex,
    );
    pub fn bl1_zdot_in(
        conj: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        rho: *mut dcomplex,
    );
    pub fn bl1_sdot(
        conj: conj1_t,
        n: c_int,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        rho: *mut f32,
    );
    pub fn bl1_ddot(
        conj: conj1_t,
        n: c_int,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        rho: *mut f64,
    );
    pub fn bl1_cdot(
        conj: conj1_t,
        n: c_int,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        rho: *mut scomplex,
    );
    pub fn bl1_zdot(
        conj: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        rho: *mut dcomplex,
    );
    pub fn bl1_sdots(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        beta: *mut f32,
        rho: *mut f32,
    );
    pub fn bl1_ddots(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        beta: *mut f64,
        rho: *mut f64,
    );
    pub fn bl1_cdots(
        conj: conj1_t,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        beta: *mut scomplex,
        rho: *mut scomplex,
    );
    pub fn bl1_zdots(
        conj: conj1_t,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        beta: *mut dcomplex,
        rho: *mut dcomplex,
    );
    pub fn bl1_sdot2s(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        beta: *mut f32,
        rho: *mut f32,
    );
    pub fn bl1_ddot2s(
        conj: conj1_t,
        n: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        beta: *mut f64,
        rho: *mut f64,
    );
    pub fn bl1_cdot2s(
        conj: conj1_t,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        beta: *mut scomplex,
        rho: *mut scomplex,
    );
    pub fn bl1_zdot2s(
        conj: conj1_t,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        beta: *mut dcomplex,
        rho: *mut dcomplex,
    );
    pub fn bl1_sfnorm(m: c_int, n: c_int, a: *mut f32, a_rs: c_int, a_cs: c_int, norm: *mut f32);
    pub fn bl1_dfnorm(m: c_int, n: c_int, a: *mut f64, a_rs: c_int, a_cs: c_int, norm: *mut f64);
    pub fn bl1_cfnorm(
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        norm: *mut f32,
    );
    pub fn bl1_zfnorm(
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        norm: *mut f64,
    );
    pub fn bl1_sinvscalv(conj: conj1_t, n: c_int, alpha: *mut f32, x: *mut f32, incx: c_int);
    pub fn bl1_dinvscalv(conj: conj1_t, n: c_int, alpha: *mut f64, x: *mut f64, incx: c_int);
    pub fn bl1_csinvscalv(conj: conj1_t, n: c_int, alpha: *mut f32, x: *mut scomplex, incx: c_int);
    pub fn bl1_cinvscalv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
    );
    pub fn bl1_zdinvscalv(conj: conj1_t, n: c_int, alpha: *mut f64, x: *mut dcomplex, incx: c_int);
    pub fn bl1_zinvscalv(
        conj: conj1_t,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
    );
    pub fn bl1_sinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zinvscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_snrm2(n: c_int, x: *mut f32, incx: c_int, norm: *mut f32);
    pub fn bl1_dnrm2(n: c_int, x: *mut f64, incx: c_int, norm: *mut f64);
    pub fn bl1_cnrm2(n: c_int, x: *mut scomplex, incx: c_int, norm: *mut f32);
    pub fn bl1_znrm2(n: c_int, x: *mut dcomplex, incx: c_int, norm: *mut f64);
    pub fn bl1_sscal(n: c_int, alpha: *mut f32, x: *mut f32, incx: c_int);
    pub fn bl1_dscal(n: c_int, alpha: *mut f64, x: *mut f64, incx: c_int);
    pub fn bl1_csscal(n: c_int, alpha: *mut f32, x: *mut scomplex, incx: c_int);
    pub fn bl1_cscal(n: c_int, alpha: *mut scomplex, x: *mut scomplex, incx: c_int);
    pub fn bl1_zdscal(n: c_int, alpha: *mut f64, x: *mut dcomplex, incx: c_int);
    pub fn bl1_zscal(n: c_int, alpha: *mut dcomplex, x: *mut dcomplex, incx: c_int);
    pub fn bl1_sscalv(conj: conj1_t, n: c_int, alpha: *mut f32, x: *mut f32, incx: c_int);
    pub fn bl1_dscalv(conj: conj1_t, n: c_int, alpha: *mut f64, x: *mut f64, incx: c_int);
    pub fn bl1_csscalv(conj: conj1_t, n: c_int, alpha: *mut f32, x: *mut scomplex, incx: c_int);
    pub fn bl1_cscalv(conj: conj1_t, n: c_int, alpha: *mut scomplex, x: *mut scomplex, incx: c_int);
    pub fn bl1_zdscalv(conj: conj1_t, n: c_int, alpha: *mut f64, x: *mut dcomplex, incx: c_int);
    pub fn bl1_zscalv(conj: conj1_t, n: c_int, alpha: *mut dcomplex, x: *mut dcomplex, incx: c_int);
    pub fn bl1_sscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zscalm(
        conj: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_sscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zdscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zscalmr(
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_sswap(n: c_int, x: *mut f32, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_dswap(n: c_int, x: *mut f64, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_cswap(n: c_int, x: *mut scomplex, incx: c_int, y: *mut scomplex, incy: c_int);
    pub fn bl1_zswap(n: c_int, x: *mut dcomplex, incx: c_int, y: *mut dcomplex, incy: c_int);
    pub fn bl1_sswapv(n: c_int, x: *mut f32, incx: c_int, y: *mut f32, incy: c_int);
    pub fn bl1_dswapv(n: c_int, x: *mut f64, incx: c_int, y: *mut f64, incy: c_int);
    pub fn bl1_cswapv(n: c_int, x: *mut scomplex, incx: c_int, y: *mut scomplex, incy: c_int);
    pub fn bl1_zswapv(n: c_int, x: *mut dcomplex, incx: c_int, y: *mut dcomplex, incy: c_int);
    pub fn bl1_sswapmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dswapmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_cswapmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_zswapmt(
        trans: trans1_t,
        m: c_int,
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_sgemv(
        transa: trans1_t,
        conjx: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dgemv(
        transa: trans1_t,
        conjx: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_cgemv(
        transa: trans1_t,
        conjx: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zgemv(
        transa: trans1_t,
        conjx: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sgemv_blas(
        transa: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dgemv_blas(
        transa: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_cgemv_blas(
        transa: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zgemv_blas(
        transa: trans1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sger(
        conjx: conj1_t,
        conjy: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dger(
        conjx: conj1_t,
        conjy: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cger(
        conjx: conj1_t,
        conjy: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zger(
        conjx: conj1_t,
        conjy: conj1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_sger_blas(
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        a: *mut f32,
        lda: c_int,
    );
    pub fn bl1_dger_blas(
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        a: *mut f64,
        lda: c_int,
    );
    pub fn bl1_cgerc_blas(
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_cgeru_blas(
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_zgerc_blas(
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_zgeru_blas(
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_shemv(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dhemv(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_chemv(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zhemv(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_chemv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zhemv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sher(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dher(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cher(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut scomplex,
        incx: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zher(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut dcomplex,
        incx: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cher_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut scomplex,
        incx: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_zher_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut dcomplex,
        incx: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_sher2(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dher2(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cher2(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zher2(
        uplo: uplo1_t,
        conj: conj1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_cher2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_zher2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_ssymv(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dsymv(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_csymv(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zsymv(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_ssymv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dsymv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_csymv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_zsymv_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_ssyr(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dsyr(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csyr(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsyr(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssyr_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        a: *mut f32,
        lda: c_int,
    );
    pub fn bl1_dsyr_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        a: *mut f64,
        lda: c_int,
    );
    pub fn bl1_csyr_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_zsyr_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_ssyr2(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_dsyr2(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_csyr2(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_zsyr2(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
    );
    pub fn bl1_ssyr2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: c_int,
        y: *mut f32,
        incy: c_int,
        a: *mut f32,
        lda: c_int,
    );
    pub fn bl1_dsyr2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: c_int,
        y: *mut f64,
        incy: c_int,
        a: *mut f64,
        lda: c_int,
    );
    pub fn bl1_csyr2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: c_int,
        y: *mut scomplex,
        incy: c_int,
        a: *mut scomplex,
        lda: c_int,
    );
    pub fn bl1_zsyr2_blas(
        uplo: uplo1_t,
        m: c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: c_int,
        y: *mut dcomplex,
        incy: c_int,
        a: *mut dcomplex,
        lda: c_int,
    );
    pub fn bl1_strmv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
    );
    pub fn bl1_dtrmv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
    );
    pub fn bl1_ctrmv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
    );
    pub fn bl1_ztrmv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
    );
    pub fn bl1_strmv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f32,
        lda: c_int,
        x: *mut f32,
        incx: c_int,
    );
    pub fn bl1_dtrmv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f64,
        lda: c_int,
        x: *mut f64,
        incx: c_int,
    );
    pub fn bl1_ctrmv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut scomplex,
        lda: c_int,
        x: *mut scomplex,
        incx: c_int,
    );
    pub fn bl1_ztrmv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut dcomplex,
        lda: c_int,
        x: *mut dcomplex,
        incx: c_int,
    );
    pub fn bl1_strsv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
    );
    pub fn bl1_dtrsv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
    );
    pub fn bl1_ctrsv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
    );
    pub fn bl1_ztrsv(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
    );
    pub fn bl1_strsv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f32,
        lda: c_int,
        x: *mut f32,
        incx: c_int,
    );
    pub fn bl1_dtrsv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut f64,
        lda: c_int,
        x: *mut f64,
        incx: c_int,
    );
    pub fn bl1_ctrsv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut scomplex,
        lda: c_int,
        x: *mut scomplex,
        incx: c_int,
    );
    pub fn bl1_ztrsv_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        a: *mut dcomplex,
        lda: c_int,
        x: *mut dcomplex,
        incx: c_int,
    );
    pub fn bl1_strmvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dtrmvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_ctrmvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_ztrmvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_strsvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f32,
        incx: c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: c_int,
    );
    pub fn bl1_dtrsvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut f64,
        incx: c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: c_int,
    );
    pub fn bl1_ctrsvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut scomplex,
        incx: c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: c_int,
    );
    pub fn bl1_ztrsvsx(
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        x: *mut dcomplex,
        incx: c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: c_int,
    );
    pub fn bl1_sgemm(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        k: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dgemm(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        k: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_cgemm(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        k: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zgemm(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        k: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_sgemm_blas(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        b: *mut f32,
        ldb: c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: c_int,
    );
    pub fn bl1_dgemm_blas(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        b: *mut f64,
        ldb: c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: c_int,
    );
    pub fn bl1_cgemm_blas(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zgemm_blas(
        transa: trans1_t,
        transb: trans1_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_shemm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dhemm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_chemm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zhemm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_chemm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zhemm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_sherk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dherk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_cherk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f32,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zherk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_cherk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        lda: c_int,
        beta: *mut f32,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zherk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        lda: c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_sher2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dher2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_cher2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zher2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_cher2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
        beta: *mut f32,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zher2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_ssymm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dsymm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_csymm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zsymm(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ssymm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        b: *mut f32,
        ldb: c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: c_int,
    );
    pub fn bl1_dsymm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        b: *mut f64,
        ldb: c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: c_int,
    );
    pub fn bl1_csymm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zsymm_blas(
        side: side1_t,
        uplo: uplo1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_ssyrk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dsyrk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_csyrk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zsyrk(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ssyrk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: c_int,
    );
    pub fn bl1_dsyrk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: c_int,
    );
    pub fn bl1_csyrk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zsyrk_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_ssyr2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dsyr2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_csyr2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_zsyr2k(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ssyr2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        b: *mut f32,
        ldb: c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: c_int,
    );
    pub fn bl1_dsyr2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        b: *mut f64,
        ldb: c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: c_int,
    );
    pub fn bl1_csyr2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: c_int,
    );
    pub fn bl1_zsyr2k_blas(
        uplo: uplo1_t,
        trans: trans1_t,
        m: c_int,
        k: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: c_int,
    );
    pub fn bl1_strmm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dtrmm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ctrmm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ztrmm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_strmm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        b: *mut f32,
        ldb: c_int,
    );
    pub fn bl1_dtrmm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        b: *mut f64,
        ldb: c_int,
    );
    pub fn bl1_ctrmm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
    );
    pub fn bl1_ztrmm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
    );
    pub fn bl1_strsm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_dtrsm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ctrsm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_ztrsm(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
    );
    pub fn bl1_strsm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: c_int,
        b: *mut f32,
        ldb: c_int,
    );
    pub fn bl1_dtrsm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: c_int,
        b: *mut f64,
        ldb: c_int,
    );
    pub fn bl1_ctrsm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: c_int,
        b: *mut scomplex,
        ldb: c_int,
    );
    pub fn bl1_ztrsm_blas(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: c_int,
        b: *mut dcomplex,
        ldb: c_int,
    );
    pub fn bl1_strmmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dtrmmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ctrmmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ztrmmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_strsmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f32,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f32,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f32,
        c: *mut f32,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_dtrsmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut f64,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut f64,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut f64,
        c: *mut f64,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ctrsmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut scomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_ztrsmsx(
        side: side1_t,
        uplo: uplo1_t,
        trans: trans1_t,
        diag: diag1_t,
        m: c_int,
        n: c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        b: *mut dcomplex,
        b_rs: c_int,
        b_cs: c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        c_rs: c_int,
        c_cs: c_int,
    );
    pub fn bl1_saxmyv2(
        conjx: conj1_t,
        n: c_int,
        alpha: *mut f32,
        beta: *mut f32,
        x: *mut f32,
        inc_x: c_int,
        y: *mut f32,
        inc_y: c_int,
        z: *mut f32,
        inc_z: c_int,
    );
    pub fn bl1_daxmyv2(
        conjx: conj1_t,
        n: c_int,
        alpha: *mut f64,
        beta: *mut f64,
        x: *mut f64,
        inc_x: c_int,
        y: *mut f64,
        inc_y: c_int,
        z: *mut f64,
        inc_z: c_int,
    );
    pub fn bl1_caxmyv2(
        conjx: conj1_t,
        n: c_int,
        alpha: *mut scomplex,
        beta: *mut scomplex,
        x: *mut scomplex,
        inc_x: c_int,
        y: *mut scomplex,
        inc_y: c_int,
        z: *mut scomplex,
        inc_z: c_int,
    );
    pub fn bl1_zaxmyv2(
        conjx: conj1_t,
        n: c_int,
        alpha: *mut dcomplex,
        beta: *mut dcomplex,
        x: *mut dcomplex,
        inc_x: c_int,
        y: *mut dcomplex,
        inc_y: c_int,
        z: *mut dcomplex,
        inc_z: c_int,
    );
    pub fn bl1_saxpyv2b(
        n: c_int,
        beta1: *mut f32,
        beta2: *mut f32,
        a1: *mut f32,
        inc_a1: c_int,
        a2: *mut f32,
        inc_a2: c_int,
        w: *mut f32,
        inc_w: c_int,
    );
    pub fn bl1_daxpyv2b(
        n: c_int,
        beta1: *mut f64,
        beta2: *mut f64,
        a1: *mut f64,
        inc_a1: c_int,
        a2: *mut f64,
        inc_a2: c_int,
        w: *mut f64,
        inc_w: c_int,
    );
    pub fn bl1_caxpyv2b(
        n: c_int,
        beta1: *mut scomplex,
        beta2: *mut scomplex,
        a1: *mut scomplex,
        inc_a1: c_int,
        a2: *mut scomplex,
        inc_a2: c_int,
        w: *mut scomplex,
        inc_w: c_int,
    );
    pub fn bl1_zaxpyv2b(
        n: c_int,
        beta1: *mut dcomplex,
        beta2: *mut dcomplex,
        a1: *mut dcomplex,
        inc_a1: c_int,
        a2: *mut dcomplex,
        inc_a2: c_int,
        w: *mut dcomplex,
        inc_w: c_int,
    );
    pub fn bl1_saxpyv3b(
        n: c_int,
        beta1: *mut f32,
        beta2: *mut f32,
        beta3: *mut f32,
        a1: *mut f32,
        inc_a1: c_int,
        a2: *mut f32,
        inc_a2: c_int,
        a3: *mut f32,
        inc_a3: c_int,
        w: *mut f32,
        inc_w: c_int,
    );
    pub fn bl1_daxpyv3b(
        n: c_int,
        beta1: *mut f64,
        beta2: *mut f64,
        beta3: *mut f64,
        a1: *mut f64,
        inc_a1: c_int,
        a2: *mut f64,
        inc_a2: c_int,
        a3: *mut f64,
        inc_a3: c_int,
        w: *mut f64,
        inc_w: c_int,
    );
    pub fn bl1_caxpyv3b(
        n: c_int,
        beta1: *mut scomplex,
        beta2: *mut scomplex,
        beta3: *mut scomplex,
        a1: *mut scomplex,
        inc_a1: c_int,
        a2: *mut scomplex,
        inc_a2: c_int,
        a3: *mut scomplex,
        inc_a3: c_int,
        w: *mut scomplex,
        inc_w: c_int,
    );
    pub fn bl1_zaxpyv3b(
        n: c_int,
        beta1: *mut dcomplex,
        beta2: *mut dcomplex,
        beta3: *mut dcomplex,
        a1: *mut dcomplex,
        inc_a1: c_int,
        a2: *mut dcomplex,
        inc_a2: c_int,
        a3: *mut dcomplex,
        inc_a3: c_int,
        w: *mut dcomplex,
        inc_w: c_int,
    );
    pub fn bl1_saxpyv2bdotaxpy(
        n: c_int,
        beta: *mut f32,
        u: *mut f32,
        inc_u: c_int,
        gamma: *mut f32,
        z: *mut f32,
        inc_z: c_int,
        a: *mut f32,
        inc_a: c_int,
        x: *mut f32,
        inc_x: c_int,
        kappa: *mut f32,
        rho: *mut f32,
        w: *mut f32,
        inc_w: c_int,
    );
    pub fn bl1_daxpyv2bdotaxpy(
        n: c_int,
        beta: *mut f64,
        u: *mut f64,
        inc_u: c_int,
        gamma: *mut f64,
        z: *mut f64,
        inc_z: c_int,
        a: *mut f64,
        inc_a: c_int,
        x: *mut f64,
        inc_x: c_int,
        kappa: *mut f64,
        rho: *mut f64,
        w: *mut f64,
        inc_w: c_int,
    );
    pub fn bl1_caxpyv2bdotaxpy(
        n: c_int,
        beta: *mut scomplex,
        u: *mut scomplex,
        inc_u: c_int,
        gamma: *mut scomplex,
        z: *mut scomplex,
        inc_z: c_int,
        a: *mut scomplex,
        inc_a: c_int,
        x: *mut scomplex,
        inc_x: c_int,
        kappa: *mut scomplex,
        rho: *mut scomplex,
        w: *mut scomplex,
        inc_w: c_int,
    );
    pub fn bl1_zaxpyv2bdotaxpy(
        n: c_int,
        beta: *mut dcomplex,
        u: *mut dcomplex,
        inc_u: c_int,
        gamma: *mut dcomplex,
        z: *mut dcomplex,
        inc_z: c_int,
        a: *mut dcomplex,
        inc_a: c_int,
        x: *mut dcomplex,
        inc_x: c_int,
        kappa: *mut dcomplex,
        rho: *mut dcomplex,
        w: *mut dcomplex,
        inc_w: c_int,
    );
    pub fn bl1_sdotsv2(
        conjxy: conj1_t,
        n: c_int,
        x: *mut f32,
        inc_x: c_int,
        y: *mut f32,
        inc_y: c_int,
        z: *mut f32,
        inc_z: c_int,
        beta: *mut f32,
        rho_xz: *mut f32,
        rho_yz: *mut f32,
    );
    pub fn bl1_ddotsv2(
        conjxy: conj1_t,
        n: c_int,
        x: *mut f64,
        inc_x: c_int,
        y: *mut f64,
        inc_y: c_int,
        z: *mut f64,
        inc_z: c_int,
        beta: *mut f64,
        rho_xz: *mut f64,
        rho_yz: *mut f64,
    );
    pub fn bl1_cdotsv2(
        conjxy: conj1_t,
        n: c_int,
        x: *mut scomplex,
        inc_x: c_int,
        y: *mut scomplex,
        inc_y: c_int,
        z: *mut scomplex,
        inc_z: c_int,
        beta: *mut scomplex,
        rho_xz: *mut scomplex,
        rho_yz: *mut scomplex,
    );
    pub fn bl1_zdotsv2(
        conjxy: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        inc_x: c_int,
        y: *mut dcomplex,
        inc_y: c_int,
        z: *mut dcomplex,
        inc_z: c_int,
        beta: *mut dcomplex,
        rho_xz: *mut dcomplex,
        rho_yz: *mut dcomplex,
    );
    pub fn bl1_sdotsv3(
        conjxyw: conj1_t,
        n: c_int,
        x: *mut f32,
        inc_x: c_int,
        y: *mut f32,
        inc_y: c_int,
        w: *mut f32,
        inc_w: c_int,
        z: *mut f32,
        inc_z: c_int,
        beta: *mut f32,
        rho_xz: *mut f32,
        rho_yz: *mut f32,
        rho_wz: *mut f32,
    );
    pub fn bl1_ddotsv3(
        conjxyw: conj1_t,
        n: c_int,
        x: *mut f64,
        inc_x: c_int,
        y: *mut f64,
        inc_y: c_int,
        w: *mut f64,
        inc_w: c_int,
        z: *mut f64,
        inc_z: c_int,
        beta: *mut f64,
        rho_xz: *mut f64,
        rho_yz: *mut f64,
        rho_wz: *mut f64,
    );
    pub fn bl1_cdotsv3(
        conjxyw: conj1_t,
        n: c_int,
        x: *mut scomplex,
        inc_x: c_int,
        y: *mut scomplex,
        inc_y: c_int,
        w: *mut scomplex,
        inc_w: c_int,
        z: *mut scomplex,
        inc_z: c_int,
        beta: *mut scomplex,
        rho_xz: *mut scomplex,
        rho_yz: *mut scomplex,
        rho_wz: *mut scomplex,
    );
    pub fn bl1_zdotsv3(
        conjxyw: conj1_t,
        n: c_int,
        x: *mut dcomplex,
        inc_x: c_int,
        y: *mut dcomplex,
        inc_y: c_int,
        w: *mut dcomplex,
        inc_w: c_int,
        z: *mut dcomplex,
        inc_z: c_int,
        beta: *mut dcomplex,
        rho_xz: *mut dcomplex,
        rho_yz: *mut dcomplex,
        rho_wz: *mut dcomplex,
    );
    pub fn bl1_sdotaxpy(
        n: c_int,
        a: *mut f32,
        inc_a: c_int,
        x: *mut f32,
        inc_x: c_int,
        kappa: *mut f32,
        rho: *mut f32,
        w: *mut f32,
        inc_w: c_int,
    );
    pub fn bl1_ddotaxpy(
        n: c_int,
        a: *mut f64,
        inc_a: c_int,
        x: *mut f64,
        inc_x: c_int,
        kappa: *mut f64,
        rho: *mut f64,
        w: *mut f64,
        inc_w: c_int,
    );
    pub fn bl1_cdotaxpy(
        n: c_int,
        a: *mut scomplex,
        inc_a: c_int,
        x: *mut scomplex,
        inc_x: c_int,
        kappa: *mut scomplex,
        rho: *mut scomplex,
        w: *mut scomplex,
        inc_w: c_int,
    );
    pub fn bl1_zdotaxpy(
        n: c_int,
        a: *mut dcomplex,
        inc_a: c_int,
        x: *mut dcomplex,
        inc_x: c_int,
        kappa: *mut dcomplex,
        rho: *mut dcomplex,
        w: *mut dcomplex,
        inc_w: c_int,
    );
    pub fn bl1_sdotaxmyv2(
        n: c_int,
        alpha: *mut f32,
        beta: *mut f32,
        x: *mut f32,
        inc_x: c_int,
        u: *mut f32,
        inc_u: c_int,
        rho: *mut f32,
        y: *mut f32,
        inc_y: c_int,
        z: *mut f32,
        inc_z: c_int,
    );
    pub fn bl1_ddotaxmyv2(
        n: c_int,
        alpha: *mut f64,
        beta: *mut f64,
        x: *mut f64,
        inc_x: c_int,
        u: *mut f64,
        inc_u: c_int,
        rho: *mut f64,
        y: *mut f64,
        inc_y: c_int,
        z: *mut f64,
        inc_z: c_int,
    );
    pub fn bl1_cdotaxmyv2(
        n: c_int,
        alpha: *mut scomplex,
        beta: *mut scomplex,
        x: *mut scomplex,
        inc_x: c_int,
        u: *mut scomplex,
        inc_u: c_int,
        rho: *mut scomplex,
        y: *mut scomplex,
        inc_y: c_int,
        z: *mut scomplex,
        inc_z: c_int,
    );
    pub fn bl1_zdotaxmyv2(
        n: c_int,
        alpha: *mut dcomplex,
        beta: *mut dcomplex,
        x: *mut dcomplex,
        inc_x: c_int,
        u: *mut dcomplex,
        inc_u: c_int,
        rho: *mut dcomplex,
        y: *mut dcomplex,
        inc_y: c_int,
        z: *mut dcomplex,
        inc_z: c_int,
    );
    pub fn bl1_sdotv2axpyv2b(
        n: c_int,
        a1: *mut f32,
        inc_a1: c_int,
        a2: *mut f32,
        inc_a2: c_int,
        x: *mut f32,
        inc_x: c_int,
        kappa1: *mut f32,
        kappa2: *mut f32,
        rho1: *mut f32,
        rho2: *mut f32,
        w: *mut f32,
        inc_w: c_int,
    );
    pub fn bl1_ddotv2axpyv2b(
        n: c_int,
        a1: *mut f64,
        inc_a1: c_int,
        a2: *mut f64,
        inc_a2: c_int,
        x: *mut f64,
        inc_x: c_int,
        kappa1: *mut f64,
        kappa2: *mut f64,
        rho1: *mut f64,
        rho2: *mut f64,
        w: *mut f64,
        inc_w: c_int,
    );
    pub fn bl1_cdotv2axpyv2b(
        n: c_int,
        a1: *mut scomplex,
        inc_a1: c_int,
        a2: *mut scomplex,
        inc_a2: c_int,
        x: *mut scomplex,
        inc_x: c_int,
        kappa1: *mut scomplex,
        kappa2: *mut scomplex,
        rho1: *mut scomplex,
        rho2: *mut scomplex,
        w: *mut scomplex,
        inc_w: c_int,
    );
    pub fn bl1_zdotv2axpyv2b(
        n: c_int,
        a1: *mut dcomplex,
        inc_a1: c_int,
        a2: *mut dcomplex,
        inc_a2: c_int,
        x: *mut dcomplex,
        inc_x: c_int,
        kappa1: *mut dcomplex,
        kappa2: *mut dcomplex,
        rho1: *mut dcomplex,
        rho2: *mut dcomplex,
        w: *mut dcomplex,
        inc_w: c_int,
    );
    pub fn bl1_zaxpyv2bdots(
        n: c_int,
        alpha1: *mut dcomplex,
        alpha2: *mut dcomplex,
        x1: *mut dcomplex,
        inc_x1: c_int,
        x2: *mut dcomplex,
        inc_x2: c_int,
        y: *mut dcomplex,
        inc_y: c_int,
        u: *mut dcomplex,
        inc_u: c_int,
        beta: *mut dcomplex,
        rho: *mut dcomplex,
    );
    pub fn isamax_(n: *mut c_int, x: *mut f32, incx: *mut c_int) -> c_int;
    pub fn idamax_(n: *mut c_int, x: *mut f64, incx: *mut c_int) -> c_int;
    pub fn icamax_(n: *mut c_int, x: *mut scomplex, incx: *mut c_int) -> c_int;
    pub fn izamax_(n: *mut c_int, x: *mut dcomplex, incx: *mut c_int) -> c_int;
    pub fn sasum_(n: *mut c_int, x: *mut f32, incx: *mut c_int) -> f32;
    pub fn dasum_(n: *mut c_int, x: *mut f64, incx: *mut c_int) -> f64;
    pub fn scasum_(n: *mut c_int, x: *mut scomplex, incx: *mut c_int) -> f32;
    pub fn dzasum_(n: *mut c_int, x: *mut dcomplex, incx: *mut c_int) -> f64;
    pub fn saxpy_(
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
    );
    pub fn daxpy_(
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
    );
    pub fn caxpy_(
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn zaxpy_(
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn scopy_(n: *mut c_int, x: *mut f32, incx: *mut c_int, y: *mut f32, incy: *mut c_int);
    pub fn dcopy_(n: *mut c_int, x: *mut f64, incx: *mut c_int, y: *mut f64, incy: *mut c_int);
    pub fn ccopy_(
        n: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn zcopy_(
        n: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn sdot_(
        n: *mut c_int,
        x: *mut f32,
        incx: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
    ) -> f32;
    pub fn ddot_(
        n: *mut c_int,
        x: *mut f64,
        incx: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
    ) -> f64;
    pub fn cdotu_(
        n: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    ) -> scomplex;
    pub fn cdotc_(
        n: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    ) -> scomplex;
    pub fn zdotu_(
        n: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    ) -> dcomplex;
    pub fn zdotc_(
        n: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    ) -> dcomplex;
    pub fn snrm2_(n: *mut c_int, x: *mut f32, incx: *mut c_int) -> f32;
    pub fn dnrm2_(n: *mut c_int, x: *mut f64, incx: *mut c_int) -> f64;
    pub fn scnrm2_(n: *mut c_int, x: *mut scomplex, incx: *mut c_int) -> f32;
    pub fn dznrm2_(n: *mut c_int, x: *mut dcomplex, incx: *mut c_int) -> f64;
    pub fn sscal_(n: *mut c_int, alpha: *mut f32, y: *mut f32, incy: *mut c_int);
    pub fn dscal_(n: *mut c_int, alpha: *mut f64, y: *mut f64, incy: *mut c_int);
    pub fn cscal_(n: *mut c_int, alpha: *mut scomplex, y: *mut scomplex, incy: *mut c_int);
    pub fn csscal_(n: *mut c_int, alpha: *mut f32, y: *mut scomplex, incy: *mut c_int);
    pub fn zscal_(n: *mut c_int, alpha: *mut dcomplex, y: *mut dcomplex, incy: *mut c_int);
    pub fn zdscal_(n: *mut c_int, alpha: *mut f64, y: *mut dcomplex, incy: *mut c_int);
    pub fn sswap_(n: *mut c_int, x: *mut f32, incx: *mut c_int, y: *mut f32, incy: *mut c_int);
    pub fn dswap_(n: *mut c_int, x: *mut f64, incx: *mut c_int, y: *mut f64, incy: *mut c_int);
    pub fn cswap_(
        n: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn zswap_(
        n: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn sgemv_(
        transa: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        x: *mut f32,
        incx: *mut c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut c_int,
    );
    pub fn dgemv_(
        transa: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        x: *mut f64,
        incx: *mut c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut c_int,
    );
    pub fn cgemv_(
        transa: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn zgemv_(
        transa: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn sger_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
    );
    pub fn dger_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
    );
    pub fn cgerc_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
    );
    pub fn cgeru_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
    );
    pub fn zgerc_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
    );
    pub fn zgeru_(
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
    );
    pub fn chemv_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        x: *mut scomplex,
        incx: *mut c_int,
        beta: *mut scomplex,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn zhemv_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        x: *mut dcomplex,
        incx: *mut c_int,
        beta: *mut dcomplex,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn cher_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut scomplex,
        incx: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
    );
    pub fn zher_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut dcomplex,
        incx: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
    );
    pub fn cher2_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
    );
    pub fn zher2_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
    );
    pub fn ssymv_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        x: *mut f32,
        incx: *mut c_int,
        beta: *mut f32,
        y: *mut f32,
        incy: *mut c_int,
    );
    pub fn dsymv_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        x: *mut f64,
        incx: *mut c_int,
        beta: *mut f64,
        y: *mut f64,
        incy: *mut c_int,
    );
    pub fn ssyr_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
    );
    pub fn dsyr_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
    );
    pub fn ssyr2_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
    );
    pub fn dsyr2_(
        uplo: *mut c_char,
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
    );
    pub fn strmv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
    );
    pub fn dtrmv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
    );
    pub fn ctrmv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn ztrmv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn strsv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        y: *mut f32,
        incy: *mut c_int,
    );
    pub fn dtrsv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        y: *mut f64,
        incy: *mut c_int,
    );
    pub fn ctrsv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        y: *mut scomplex,
        incy: *mut c_int,
    );
    pub fn ztrsv_(
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        y: *mut dcomplex,
        incy: *mut c_int,
    );
    pub fn sgemm_(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
    );
    pub fn dgemm_(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
    );
    pub fn cgemm_(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zgemm_(
        transa: *mut c_char,
        transb: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn chemm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zhemm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn cherk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f32,
        a: *mut scomplex,
        lda: *mut c_int,
        beta: *mut f32,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zherk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f64,
        a: *mut dcomplex,
        lda: *mut c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn cher2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        beta: *mut f32,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zher2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        beta: *mut f64,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn ssymm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
    );
    pub fn dsymm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
    );
    pub fn csymm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zsymm_(
        side: *mut c_char,
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn ssyrk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
    );
    pub fn dsyrk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
    );
    pub fn csyrk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zsyrk_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn ssyr2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        beta: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
    );
    pub fn dsyr2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        beta: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
    );
    pub fn csyr2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        beta: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
    );
    pub fn zsyr2k_(
        uplo: *mut c_char,
        transa: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        beta: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
    );
    pub fn strmm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
    );
    pub fn dtrmm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
    );
    pub fn ctrmm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
    );
    pub fn ztrmm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
    );
    pub fn strsm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
    );
    pub fn dtrsm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
    );
    pub fn ctrsm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
    );
    pub fn ztrsm_(
        side: *mut c_char,
        uplo: *mut c_char,
        transa: *mut c_char,
        diag: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
    );
    pub static mut FLA_THREE: FLA_Obj;
    pub static mut FLA_TWO: FLA_Obj;
    pub static mut FLA_ONE: FLA_Obj;
    pub static mut FLA_ONE_HALF: FLA_Obj;
    pub static mut FLA_ZERO: FLA_Obj;
    pub static mut FLA_MINUS_ONE_HALF: FLA_Obj;
    pub static mut FLA_MINUS_ONE: FLA_Obj;
    pub static mut FLA_MINUS_TWO: FLA_Obj;
    pub static mut FLA_MINUS_THREE: FLA_Obj;
    pub static mut FLA_EPSILON: FLA_Obj;
    pub static mut FLA_SAFE_MIN: FLA_Obj;
    pub static mut FLA_SAFE_MIN_SQUARE: FLA_Obj;
    pub static mut FLA_SAFE_INV_MIN: FLA_Obj;
    pub static mut FLA_SAFE_INV_MIN_SQUARE: FLA_Obj;
    pub static mut FLA_UNDERFLOW_THRES: FLA_Obj;
    pub static mut FLA_OVERFLOW_THRES: FLA_Obj;
    pub static mut FLA_UNDERFLOW_SQUARE_THRES: FLA_Obj;
    pub static mut FLA_OVERFLOW_SQUARE_THRES: FLA_Obj;
    pub static fzero: f32;
    pub static dzero: f64;
    pub static czero: scomplex;
    pub static zzero: dcomplex;
    pub fn FLA_Cntl_obj_free(cntl: *mut c_void);
    pub fn FLA_Cntl_axpy_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_axpy: *mut fla_axpy_t,
    ) -> *mut fla_axpy_t;
    pub fn FLA_Cntl_axpyt_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_axpyt: *mut fla_axpyt_t,
    ) -> *mut fla_axpyt_t;
    pub fn FLA_Cntl_copy_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_copy: *mut fla_copy_t,
    ) -> *mut fla_copy_t;
    pub fn FLA_Cntl_copyt_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_copyt: *mut fla_copyt_t,
    ) -> *mut fla_copyt_t;
    pub fn FLA_Cntl_copyr_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_copyr: *mut fla_copyr_t,
        sub_copy: *mut fla_copy_t,
    ) -> *mut fla_copyr_t;
    pub fn FLA_Cntl_scal_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
    ) -> *mut fla_scal_t;
    pub fn FLA_Cntl_scalr_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scalr: *mut fla_scalr_t,
        sub_scal: *mut fla_scal_t,
    ) -> *mut fla_scalr_t;
    pub fn FLA_Cntl_swap_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_swap: *mut fla_swap_t,
    ) -> *mut fla_swap_t;
    pub fn FLA_Cntl_tpose_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_trans: *mut fla_tpose_t,
        sub_swap: *mut fla_swap_t,
    ) -> *mut fla_tpose_t;
    pub fn FLA_Cntl_gemv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_gemv: *mut fla_gemv_t,
    ) -> *mut fla_gemv_t;
    pub fn FLA_Cntl_trsv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_trsv: *mut fla_trsv_t,
        sub_gemv: *mut fla_gemv_t,
    ) -> *mut fla_trsv_t;
    pub fn FLA_Cntl_gemm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_gemm_t;
    pub fn FLA_Cntl_hemm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_hemm: *mut fla_hemm_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
    ) -> *mut fla_hemm_t;
    pub fn FLA_Cntl_herk_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scalr: *mut fla_scalr_t,
        sub_herk: *mut fla_herk_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_herk_t;
    pub fn FLA_Cntl_her2k_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scalr: *mut fla_scalr_t,
        sub_her2k: *mut fla_her2k_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
    ) -> *mut fla_her2k_t;
    pub fn FLA_Cntl_symm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_symm: *mut fla_symm_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
    ) -> *mut fla_symm_t;
    pub fn FLA_Cntl_syrk_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scalr: *mut fla_scalr_t,
        sub_syrk: *mut fla_syrk_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_syrk_t;
    pub fn FLA_Cntl_syr2k_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scalr: *mut fla_scalr_t,
        sub_syr2k: *mut fla_syr2k_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
    ) -> *mut fla_syr2k_t;
    pub fn FLA_Cntl_trmm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_trmm: *mut fla_trmm_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_trmm_t;
    pub fn FLA_Cntl_trsm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_trsm: *mut fla_trsm_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_trsm_t;
    pub fn FLA_Cntl_chol_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_chol: *mut fla_chol_t,
        sub_herk: *mut fla_herk_t,
        sub_trsm: *mut fla_trsm_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_chol_t;
    pub fn FLA_Cntl_lu_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_lu: *mut fla_lu_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_gemm3: *mut fla_gemm_t,
        sub_trsm1: *mut fla_trsm_t,
        sub_trsm2: *mut fla_trsm_t,
        sub_appiv1: *mut fla_appiv_t,
        sub_appiv2: *mut fla_appiv_t,
    ) -> *mut fla_lu_t;
    pub fn FLA_Cntl_appiv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_appiv: *mut fla_appiv_t,
    ) -> *mut fla_appiv_t;
    pub fn FLA_Cntl_qrut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_qrut: *mut fla_qrut_t,
        sub_apqut: *mut fla_apqut_t,
    ) -> *mut fla_qrut_t;
    pub fn FLA_Cntl_qr2ut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_qr2ut: *mut fla_qr2ut_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_copy: *mut fla_copy_t,
        sub_axpy: *mut fla_axpy_t,
    ) -> *mut fla_qr2ut_t;
    pub fn FLA_Cntl_lqut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_lqut: *mut fla_lqut_t,
        sub_apqut: *mut fla_apqut_t,
    ) -> *mut fla_lqut_t;
    pub fn FLA_Cntl_caqr2ut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_caqr2ut: *mut fla_caqr2ut_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_trmm1: *mut fla_trmm_t,
        sub_trmm2: *mut fla_trmm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_axpy1: *mut fla_axpy_t,
        sub_axpy2: *mut fla_axpy_t,
        sub_axpy3: *mut fla_axpy_t,
        sub_copy: *mut fla_copy_t,
    ) -> *mut fla_caqr2ut_t;
    pub fn FLA_Cntl_hessut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
    ) -> *mut fla_hessut_t;
    pub fn FLA_Cntl_tridiagut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
    ) -> *mut fla_tridiagut_t;
    pub fn FLA_Cntl_bidiagut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
    ) -> *mut fla_bidiagut_t;
    pub fn FLA_Cntl_trinv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_trinv: *mut fla_trinv_t,
        sub_trmm: *mut fla_trmm_t,
        sub_trsm1: *mut fla_trsm_t,
        sub_trsm2: *mut fla_trsm_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_trinv_t;
    pub fn FLA_Cntl_ttmm_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_ttmm: *mut fla_ttmm_t,
        sub_herk: *mut fla_herk_t,
        sub_trmm: *mut fla_trmm_t,
        sub_gemm: *mut fla_gemm_t,
    ) -> *mut fla_ttmm_t;
    pub fn FLA_Cntl_sylv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_sylv1: *mut fla_sylv_t,
        sub_sylv2: *mut fla_sylv_t,
        sub_sylv3: *mut fla_sylv_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_gemm3: *mut fla_gemm_t,
        sub_gemm4: *mut fla_gemm_t,
        sub_gemm5: *mut fla_gemm_t,
        sub_gemm6: *mut fla_gemm_t,
        sub_gemm7: *mut fla_gemm_t,
        sub_gemm8: *mut fla_gemm_t,
    ) -> *mut fla_sylv_t;
    pub fn FLA_Cntl_lyap_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_scal: *mut fla_scal_t,
        sub_lyap: *mut fla_lyap_t,
        sub_sylv: *mut fla_sylv_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_hemm: *mut fla_hemm_t,
        sub_her2k: *mut fla_her2k_t,
    ) -> *mut fla_lyap_t;
    pub fn FLA_Cntl_spdinv_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_chol: *mut fla_chol_t,
        sub_trinv: *mut fla_trinv_t,
        sub_ttmm: *mut fla_ttmm_t,
    ) -> *mut fla_spdinv_t;
    pub fn FLA_Cntl_apqut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apqut: *mut fla_apqut_t,
        sub_trmm1: *mut fla_trmm_t,
        sub_trmm2: *mut fla_trmm_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_copyt: *mut fla_copyt_t,
        sub_axpyt: *mut fla_axpyt_t,
    ) -> *mut fla_apqut_t;
    pub fn FLA_Cntl_apq2ut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apq2ut: *mut fla_apq2ut_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_copyt: *mut fla_copyt_t,
        sub_axpyt: *mut fla_axpyt_t,
    ) -> *mut fla_apq2ut_t;
    pub fn FLA_Cntl_apcaq2ut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apcaq2ut: *mut fla_apcaq2ut_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_trmm1: *mut fla_trmm_t,
        sub_trmm2: *mut fla_trmm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_axpy1: *mut fla_axpy_t,
        sub_axpy2: *mut fla_axpy_t,
        sub_axpy3: *mut fla_axpy_t,
        sub_copy: *mut fla_copy_t,
    ) -> *mut fla_apcaq2ut_t;
    pub fn FLA_Cntl_qrutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_qrut: *mut fla_qrut_t,
        sub_apqut: *mut fla_apqut_t,
        sub_qr2ut: *mut fla_qr2ut_t,
        sub_apq2ut: *mut fla_apq2ut_t,
    ) -> *mut fla_qrutinc_t;
    pub fn FLA_Cntl_apqutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apqut: *mut fla_apqut_t,
        sub_apq2ut: *mut fla_apq2ut_t,
    ) -> *mut fla_apqutinc_t;
    pub fn FLA_Cntl_caqrutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_caqr2ut: *mut fla_caqr2ut_t,
        sub_apcaq2ut: *mut fla_apcaq2ut_t,
    ) -> *mut fla_caqrutinc_t;
    pub fn FLA_Cntl_apcaqutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apcaq2ut: *mut fla_apcaq2ut_t,
    ) -> *mut fla_apcaqutinc_t;
    pub fn FLA_Cntl_uddateut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_uddateut: *mut fla_uddateut_t,
        sub_apqudut: *mut fla_apqudut_t,
    ) -> *mut fla_uddateut_t;
    pub fn FLA_Cntl_apqudut_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apq2ut: *mut fla_apqudut_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_gemm3: *mut fla_gemm_t,
        sub_gemm4: *mut fla_gemm_t,
        sub_trsm: *mut fla_trsm_t,
        sub_copyt: *mut fla_copyt_t,
        sub_axpyt: *mut fla_axpyt_t,
    ) -> *mut fla_apqudut_t;
    pub fn FLA_Cntl_uddateutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_uddateut: *mut fla_uddateut_t,
        sub_apqudut: *mut fla_apqudut_t,
    ) -> *mut fla_uddateutinc_t;
    pub fn FLA_Cntl_apqudutinc_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_apqudut: *mut fla_apqudut_t,
    ) -> *mut fla_apqudutinc_t;
    pub fn FLA_Cntl_eig_gest_obj_create(
        matrix_type: FLA_Matrix_type,
        variant: c_int,
        blocksize: *mut fla_blocksize_t,
        sub_eig_gest: *mut fla_eig_gest_t,
        sub_axpy1: *mut fla_axpy_t,
        sub_axpy2: *mut fla_axpy_t,
        sub_gemm1: *mut fla_gemm_t,
        sub_gemm2: *mut fla_gemm_t,
        sub_gemm3: *mut fla_gemm_t,
        sub_hemm: *mut fla_hemm_t,
        sub_her2k: *mut fla_her2k_t,
        sub_trmm1: *mut fla_trmm_t,
        sub_trmm2: *mut fla_trmm_t,
        sub_trsm1: *mut fla_trsm_t,
        sub_trsm2: *mut fla_trsm_t,
    ) -> *mut fla_eig_gest_t;
    pub fn FLA_Cntl_init_flamec();
    pub fn FLA_Cntl_finalize_flamec();
    pub fn FLA_Transpose_cntl_init();
    pub fn FLA_Transpose_cntl_finalize();
    pub fn FLA_Axpy_cntl_init();
    pub fn FLA_Axpyt_cntl_init();
    pub fn FLA_Copy_cntl_init();
    pub fn FLA_Copyt_cntl_init();
    pub fn FLA_Copyr_cntl_init();
    pub fn FLA_Scal_cntl_init();
    pub fn FLA_Scalr_cntl_init();
    pub fn FLA_Axpy_cntl_finalize();
    pub fn FLA_Axpyt_cntl_finalize();
    pub fn FLA_Copy_cntl_finalize();
    pub fn FLA_Copyt_cntl_finalize();
    pub fn FLA_Copyr_cntl_finalize();
    pub fn FLA_Scal_cntl_finalize();
    pub fn FLA_Scalr_cntl_finalize();
    pub fn FLA_Gemv_cntl_init();
    pub fn FLA_Trsv_cntl_init();
    pub fn FLA_Gemv_cntl_finalize();
    pub fn FLA_Trsv_cntl_finalize();
    pub fn FLA_Gemm_cntl_init();
    pub fn FLA_Hemm_cntl_init();
    pub fn FLA_Herk_cntl_init();
    pub fn FLA_Her2k_cntl_init();
    pub fn FLA_Symm_cntl_init();
    pub fn FLA_Syrk_cntl_init();
    pub fn FLA_Syr2k_cntl_init();
    pub fn FLA_Trmm_cntl_init();
    pub fn FLA_Trsm_cntl_init();
    pub fn FLA_Gemm_cntl_finalize();
    pub fn FLA_Hemm_cntl_finalize();
    pub fn FLA_Herk_cntl_finalize();
    pub fn FLA_Her2k_cntl_finalize();
    pub fn FLA_Symm_cntl_finalize();
    pub fn FLA_Syrk_cntl_finalize();
    pub fn FLA_Syr2k_cntl_finalize();
    pub fn FLA_Trmm_cntl_finalize();
    pub fn FLA_Trsm_cntl_finalize();
    pub fn FLA_Apply_pivots_cntl_init();
    pub fn FLA_Chol_cntl_init();
    pub fn FLA_LU_piv_cntl_init();
    pub fn FLA_LU_nopiv_cntl_init();
    pub fn FLA_QR_UT_cntl_init();
    pub fn FLA_QR2_UT_cntl_init();
    pub fn FLA_LQ_UT_cntl_init();
    pub fn FLA_CAQR2_UT_cntl_init();
    pub fn FLA_UDdate_UT_cntl_init();
    pub fn FLA_Hess_UT_cntl_init();
    pub fn FLA_Tridiag_UT_cntl_init();
    pub fn FLA_Bidiag_UT_cntl_init();
    pub fn FLA_Trinv_cntl_init();
    pub fn FLA_Ttmm_cntl_init();
    pub fn FLA_Sylv_cntl_init();
    pub fn FLA_Lyap_cntl_init();
    pub fn FLA_SPDinv_cntl_init();
    pub fn FLA_Apply_Q_UT_cntl_init();
    pub fn FLA_Apply_Q2_UT_cntl_init();
    pub fn FLA_Apply_CAQ2_UT_cntl_init();
    pub fn FLA_Apply_QUD_UT_cntl_init();
    pub fn FLA_Eig_gest_cntl_init();
    pub fn FLA_Apply_pivots_cntl_finalize();
    pub fn FLA_Chol_cntl_finalize();
    pub fn FLA_LU_piv_cntl_finalize();
    pub fn FLA_LU_nopiv_cntl_finalize();
    pub fn FLA_QR_UT_cntl_finalize();
    pub fn FLA_QR2_UT_cntl_finalize();
    pub fn FLA_LQ_UT_cntl_finalize();
    pub fn FLA_CAQR2_UT_cntl_finalize();
    pub fn FLA_UDdate_UT_cntl_finalize();
    pub fn FLA_Hess_UT_cntl_finalize();
    pub fn FLA_Tridiag_UT_cntl_finalize();
    pub fn FLA_Bidiag_UT_cntl_finalize();
    pub fn FLA_Trinv_cntl_finalize();
    pub fn FLA_Ttmm_cntl_finalize();
    pub fn FLA_Sylv_cntl_finalize();
    pub fn FLA_Lyap_cntl_finalize();
    pub fn FLA_SPDinv_cntl_finalize();
    pub fn FLA_Apply_Q_UT_cntl_finalize();
    pub fn FLA_Apply_Q2_UT_cntl_finalize();
    pub fn FLA_Apply_CAQ2_UT_cntl_finalize();
    pub fn FLA_Apply_QUD_UT_cntl_finalize();
    pub fn FLA_Eig_gest_cntl_finalize();
    pub fn FLA_Cntl_init_flash();
    pub fn FLA_Cntl_finalize_flash();
    pub fn FLASH_Transpose_cntl_init();
    pub fn FLASH_Transpose_cntl_finalize();
    pub fn FLASH_Axpy_cntl_init();
    pub fn FLASH_Axpyt_cntl_init();
    pub fn FLASH_Copy_cntl_init();
    pub fn FLASH_Copyt_cntl_init();
    pub fn FLASH_Copyr_cntl_init();
    pub fn FLASH_Scal_cntl_init();
    pub fn FLASH_Scalr_cntl_init();
    pub fn FLASH_Axpy_cntl_finalize();
    pub fn FLASH_Axpyt_cntl_finalize();
    pub fn FLASH_Copy_cntl_finalize();
    pub fn FLASH_Copyt_cntl_finalize();
    pub fn FLASH_Copyr_cntl_finalize();
    pub fn FLASH_Scal_cntl_finalize();
    pub fn FLASH_Scalr_cntl_finalize();
    pub fn FLASH_Gemv_cntl_init();
    pub fn FLASH_Trsv_cntl_init();
    pub fn FLASH_Gemv_cntl_finalize();
    pub fn FLASH_Trsv_cntl_finalize();
    pub fn FLASH_Gemm_cntl_init();
    pub fn FLASH_Hemm_cntl_init();
    pub fn FLASH_Herk_cntl_init();
    pub fn FLASH_Her2k_cntl_init();
    pub fn FLASH_Symm_cntl_init();
    pub fn FLASH_Syrk_cntl_init();
    pub fn FLASH_Syr2k_cntl_init();
    pub fn FLASH_Trmm_cntl_init();
    pub fn FLASH_Trsm_cntl_init();
    pub fn FLASH_Gemm_cntl_finalize();
    pub fn FLASH_Hemm_cntl_finalize();
    pub fn FLASH_Herk_cntl_finalize();
    pub fn FLASH_Her2k_cntl_finalize();
    pub fn FLASH_Symm_cntl_finalize();
    pub fn FLASH_Syrk_cntl_finalize();
    pub fn FLASH_Syr2k_cntl_finalize();
    pub fn FLASH_Trmm_cntl_finalize();
    pub fn FLASH_Trsm_cntl_finalize();
    pub fn FLASH_Apply_pivots_cntl_init();
    pub fn FLASH_Chol_cntl_init();
    pub fn FLASH_LU_nopiv_cntl_init();
    pub fn FLASH_LU_piv_cntl_init();
    pub fn FLASH_LU_incpiv_cntl_init();
    pub fn FLASH_Trinv_cntl_init();
    pub fn FLASH_Ttmm_cntl_init();
    pub fn FLASH_SPDinv_cntl_init();
    pub fn FLASH_Sylv_cntl_init();
    pub fn FLASH_Lyap_cntl_init();
    pub fn FLASH_QR_UT_cntl_init();
    pub fn FLASH_QR2_UT_cntl_init();
    pub fn FLASH_LQ_UT_cntl_init();
    pub fn FLASH_CAQR2_UT_cntl_init();
    pub fn FLASH_UDdate_UT_cntl_init();
    pub fn FLASH_QR_UT_inc_cntl_init();
    pub fn FLASH_CAQR_UT_inc_cntl_init();
    pub fn FLASH_UDdate_UT_inc_cntl_init();
    pub fn FLASH_Apply_Q_UT_cntl_init();
    pub fn FLASH_Apply_Q2_UT_cntl_init();
    pub fn FLASH_Apply_CAQ2_UT_cntl_init();
    pub fn FLASH_Apply_QUD_UT_cntl_init();
    pub fn FLASH_Apply_Q_UT_inc_cntl_init();
    pub fn FLASH_Apply_CAQ_UT_inc_cntl_init();
    pub fn FLASH_Apply_QUD_UT_inc_cntl_init();
    pub fn FLASH_Eig_gest_cntl_init();
    pub fn FLASH_Apply_pivots_cntl_finalize();
    pub fn FLASH_Chol_cntl_finalize();
    pub fn FLASH_LU_nopiv_cntl_finalize();
    pub fn FLASH_LU_piv_cntl_finalize();
    pub fn FLASH_LU_incpiv_cntl_finalize();
    pub fn FLASH_Trinv_cntl_finalize();
    pub fn FLASH_Ttmm_cntl_finalize();
    pub fn FLASH_SPDinv_cntl_finalize();
    pub fn FLASH_Sylv_cntl_finalize();
    pub fn FLASH_Lyap_cntl_finalize();
    pub fn FLASH_QR_UT_cntl_finalize();
    pub fn FLASH_QR2_UT_cntl_finalize();
    pub fn FLASH_LQ_UT_cntl_finalize();
    pub fn FLASH_CAQR2_UT_cntl_finalize();
    pub fn FLASH_UDdate_UT_cntl_finalize();
    pub fn FLASH_QR_UT_inc_cntl_finalize();
    pub fn FLASH_CAQR_UT_inc_cntl_finalize();
    pub fn FLASH_UDdate_UT_inc_cntl_finalize();
    pub fn FLASH_Apply_Q_UT_cntl_finalize();
    pub fn FLASH_Apply_Q2_UT_cntl_finalize();
    pub fn FLASH_Apply_CAQ2_UT_cntl_finalize();
    pub fn FLASH_Apply_QUD_UT_cntl_finalize();
    pub fn FLASH_Apply_Q_UT_inc_cntl_finalize();
    pub fn FLASH_Apply_CAQ_UT_inc_cntl_finalize();
    pub fn FLASH_Apply_QUD_UT_inc_cntl_finalize();
    pub fn FLASH_Eig_gest_cntl_finalize();
    pub fn FLA_Cntl_init();
    pub fn FLA_Cntl_finalize();
    pub fn FLA_Blocksize_create(
        b_s: dim_t,
        b_d: dim_t,
        b_c: dim_t,
        b_z: dim_t,
    ) -> *mut fla_blocksize_t;
    pub fn FLA_Blocksize_create_copy(bp: *mut fla_blocksize_t) -> *mut fla_blocksize_t;
    pub fn FLA_Blocksize_set(
        bp: *mut fla_blocksize_t,
        b_s: dim_t,
        b_d: dim_t,
        b_c: dim_t,
        b_z: dim_t,
    );
    pub fn FLA_Blocksize_scale(bp: *mut fla_blocksize_t, factor: f64);
    pub fn FLA_Blocksize_free(bp: *mut fla_blocksize_t);
    pub fn FLA_Blocksize_extract(dt: FLA_Datatype, bp: *mut fla_blocksize_t) -> dim_t;
    pub fn FLA_Query_blocksizes(dim: FLA_Dimension) -> *mut fla_blocksize_t;
    pub fn FLA_Query_blocksize(dt: FLA_Datatype, dim: FLA_Dimension) -> dim_t;
    pub fn FLA_Determine_blocksize(
        A_unproc: FLA_Obj,
        to_dir: FLA_Quadrant,
        cntl_blocksizes: *mut fla_blocksize_t,
    ) -> dim_t;
    pub fn FLA_determine_matrix_size(A_unproc: FLA_Obj, to_dir: FLA_Quadrant) -> dim_t;
    pub fn FLA_Check_error_level() -> c_uint;
    pub fn FLA_Check_error_level_set(level: c_uint) -> c_uint;
    pub fn FLA_Check_error_code_helper(code: c_int, file: *mut c_char, line: c_int) -> FLA_Error;
    pub fn FLA_Check_valid_side(side: FLA_Side) -> FLA_Error;
    pub fn FLA_Check_valid_uplo(uplo: FLA_Uplo) -> FLA_Error;
    pub fn FLA_Check_valid_trans(trans: FLA_Trans) -> FLA_Error;
    pub fn FLA_Check_valid_diag(diag: FLA_Diag) -> FLA_Error;
    pub fn FLA_Check_valid_conj(conj: FLA_Conj) -> FLA_Error;
    pub fn FLA_Check_valid_direct(direct: FLA_Conj) -> FLA_Error;
    pub fn FLA_Check_valid_storev(storev: FLA_Conj) -> FLA_Error;
    pub fn FLA_Check_valid_inverse(inv: FLA_Inv) -> FLA_Error;
    pub fn FLA_Check_valid_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_valid_object_datatype(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_valid_evd_type(evd_type: FLA_Evd_type) -> FLA_Error;
    pub fn FLA_Check_valid_svd_type(svd_type: FLA_Svd_type) -> FLA_Error;
    pub fn FLA_Check_valid_svd_type_combination(
        svd_type_u: FLA_Svd_type,
        svd_type_v: FLA_Svd_type,
    ) -> FLA_Error;
    pub fn FLA_Check_valid_svd_type_and_trans_combination(
        svd_type_u: FLA_Svd_type,
        transu: FLA_Trans,
        svd_type_v: FLA_Svd_type,
        transv: FLA_Trans,
    ) -> FLA_Error;
    pub fn FLA_Check_floating_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_int_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_real_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_complex_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_floating_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_int_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_real_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_comparable_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_complex_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_consistent_datatype(datatype: FLA_Datatype, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_consistent_object_datatype(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_identical_object_precision(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_square(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_if_scalar(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_if_vector(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_conformal_dims(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_matrix_matrix_dims(
        transa: FLA_Trans,
        transb: FLA_Trans,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Check_matrix_vector_dims(
        trans: FLA_Trans,
        A: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Check_equal_vector_dims(x: FLA_Obj, y: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_conj1_trans_and_datatype(trans: FLA_Trans, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_hess_indices(A: FLA_Obj, ilo: c_int, ihi: c_int) -> FLA_Error;
    pub fn FLA_Check_null_pointer(ptr: *mut c_void) -> FLA_Error;
    pub fn FLA_Check_object_dims(trans: FLA_Trans, m: dim_t, n: dim_t, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_valid_pivot_type(ptype: FLA_Pivot_type) -> FLA_Error;
    pub fn FLA_Check_malloc_pointer(ptr: *mut c_void) -> FLA_Error;
    pub fn FLA_Check_base_buffer_mismatch(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_adjacent_objects_2x2(
        A11: FLA_Obj,
        A12: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Check_adjacent_objects_2x1(AT: FLA_Obj, AB: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_adjacent_objects_1x2(AL: FLA_Obj, AR: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_blocksize_value(b: dim_t) -> FLA_Error;
    pub fn FLA_Check_blocksize_object(
        datatype: FLA_Datatype,
        bp: *mut fla_blocksize_t,
    ) -> FLA_Error;
    pub fn FLA_Check_file_descriptor(fd: c_int) -> FLA_Error;
    pub fn FLA_Check_lseek_result(requested_offset: c_int, lseek_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_close_result(close_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_unlink_result(unlink_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_read_result(requested_size: c_int, read_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_write_result(requested_size: c_int, write_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_valid_quadrant(quad: FLA_Quadrant) -> FLA_Error;
    pub fn FLA_Check_vector_dim_min(x: FLA_Obj, min_dim: dim_t) -> FLA_Error;
    pub fn FLA_Check_pthread_create_result(pthread_create_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_pthread_join_result(pthread_join_r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_valid_isgn_value(isgn: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_sylv_matrix_dims(A: FLA_Obj, B: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_chol_failure(r_val: FLA_Error) -> FLA_Error;
    pub fn FLA_Check_valid_elemtype(elemtype: FLA_Elemtype) -> FLA_Error;
    pub fn FLA_Check_posix_memalign_failure(r_val: c_int) -> FLA_Error;
    pub fn FLA_Check_submatrix_dims_and_offset(
        m: dim_t,
        n: dim_t,
        i: dim_t,
        j: dim_t,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Check_object_scalar_elemtype(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_object_matrix_elemtype(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_num_threads(n_threads: c_uint) -> FLA_Error;
    pub fn FLA_Check_conj_and_datatype(conj: FLA_Conj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_valid_complex_trans(trans: FLA_Trans) -> FLA_Error;
    pub fn FLA_Check_valid_real_trans(trans: FLA_Trans) -> FLA_Error;
    pub fn FLA_Check_valid_blas_trans(trans: FLA_Trans) -> FLA_Error;
    pub fn FLA_Check_nonconstant_datatype(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Check_nonconstant_object(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_identical_object_datatype(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_divide_by_zero(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_identical_object_elemtype(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_pivot_index_range(p: FLA_Obj, k1: dim_t, k2: dim_t) -> FLA_Error;
    pub fn FLA_Check_householder_panel_dims(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_object_length_equals(A: FLA_Obj, m: dim_t) -> FLA_Error;
    pub fn FLA_Check_object_width_equals(A: FLA_Obj, n: dim_t) -> FLA_Error;
    pub fn FLA_Check_object_length_min(A: FLA_Obj, m: dim_t) -> FLA_Error;
    pub fn FLA_Check_object_width_min(A: FLA_Obj, n: dim_t) -> FLA_Error;
    pub fn FLA_Check_valid_error_level(level: c_uint) -> FLA_Error;
    pub fn FLA_Check_attempted_repart_2x2(A_quad: FLA_Obj, b_m: dim_t, b_n: dim_t) -> FLA_Error;
    pub fn FLA_Check_attempted_repart_2x1(A_side: FLA_Obj, b_m: dim_t) -> FLA_Error;
    pub fn FLA_Check_attempted_repart_1x2(A_side: FLA_Obj, b_n: dim_t) -> FLA_Error;
    pub fn FLA_Check_valid_leftright_side(side: FLA_Side) -> FLA_Error;
    pub fn FLA_Check_valid_topbottom_side(side: FLA_Side) -> FLA_Error;
    pub fn FLA_Check_matrix_strides(m: dim_t, n: dim_t, rs: dim_t, cs: dim_t) -> FLA_Error;
    pub fn FLA_Check_vector_dim(x: FLA_Obj, expected_length: dim_t) -> FLA_Error;
    pub fn FLA_Check_row_vector(x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_col_vector(x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_valid_machval(val: FLA_Machval) -> FLA_Error;
    pub fn FLA_Check_valid_diag_offset(A: FLA_Obj, offset: FLA_Diag_off) -> FLA_Error;
    pub fn FLA_Check_col_storage(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Check_row_storage(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Error_string_for_code(code: c_int) -> *mut c_char;
    pub fn FLA_Error_messages_init();
    pub fn FLA_Print_message(str_: *mut c_char, file: *mut c_char, line: c_int);
    pub fn FLA_Abort();
    pub fn FLA_Init();
    pub fn FLA_Finalize();
    pub fn FLA_Initialized() -> FLA_Bool;
    pub fn FLA_Init_safe(init_result: *mut FLA_Error);
    pub fn FLA_Finalize_safe(init_result: FLA_Error);
    pub fn FLA_Init_constants();
    pub fn FLA_Finalize_constants();
    pub fn FLA_Init_numerical_constants();
    pub fn FLA_Finalize_numerical_constants();
    pub fn FLA_Lock_init(fla_lock_ptr: *mut FLA_Lock);
    pub fn FLA_Lock_destroy(fla_lock_ptr: *mut FLA_Lock);
    pub fn FLA_Lock_acquire(fla_lock_ptr: *mut FLA_Lock);
    pub fn FLA_Lock_release(fla_lock_ptr: *mut FLA_Lock);
    pub fn FLA_RWLock_init(fla_lock_ptr: *mut FLA_RWLock);
    pub fn FLA_RWLock_destroy(fla_lock_ptr: *mut FLA_RWLock);
    pub fn FLA_RWLock_write_acquire(fla_lock_ptr: *mut FLA_RWLock);
    pub fn FLA_RWLock_read_acquire(fla_lock_ptr: *mut FLA_RWLock);
    pub fn FLA_RWLock_release(fla_lock_ptr: *mut FLA_RWLock);
    pub fn FLA_Memory_leak_counter_init();
    pub fn FLA_Memory_leak_counter_finalize();
    pub fn FLA_Memory_leak_counter_status() -> FLA_Bool;
    pub fn FLA_Memory_leak_counter_set(new_status: FLA_Bool) -> FLA_Bool;
    pub fn FLA_malloc(size: usize) -> *mut c_void;
    pub fn FLA_realloc(old_ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn FLA_buff_malloc(size: usize) -> *mut c_void;
    pub fn FLA_free(ptr: *mut c_void);
    pub fn FLA_buff_free(ptr: *mut c_void);
    pub fn FLA_Obj_copy_view(A: FLA_Obj, B: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_extract_real_scalar(alpha: FLA_Obj, alpha_value: *mut f64);
    pub fn FLA_Obj_extract_complex_scalar(alpha: FLA_Obj, alpha_value: *mut dcomplex);
    pub fn FLA_Obj_extract_real_part(alpha: FLA_Obj, beta: FLA_Obj);
    pub fn FLA_Obj_extract_imag_part(alpha: FLA_Obj, beta: FLA_Obj);
    pub fn FLA_Obj_set_real_part(alpha: FLA_Obj, beta: FLA_Obj);
    pub fn FLA_Obj_set_imag_part(alpha: FLA_Obj, beta: FLA_Obj);
    pub fn FLA_Obj_show(
        s1: *mut c_char,
        A: FLA_Obj,
        format: *mut c_char,
        s2: *mut c_char,
    ) -> FLA_Error;
    pub fn FLA_Obj_fshow(
        file: *mut FILE,
        s1: *mut c_char,
        A: FLA_Obj,
        format: *mut c_char,
        s2: *mut c_char,
    ) -> FLA_Error;
    pub fn FLA_Obj_copy_view_check(A: FLA_Obj, B: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_extract_real_scalar_check(alpha: FLA_Obj, alpha_value: *mut f64) -> FLA_Error;
    pub fn FLA_Obj_extract_complex_scalar_check(
        alpha: FLA_Obj,
        alpha_value: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Obj_extract_real_part_check(alpha: FLA_Obj, beta: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_extract_imag_part_check(alpha: FLA_Obj, beta: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_set_real_part_check(alpha: FLA_Obj, beta: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_set_imag_part_check(alpha: FLA_Obj, beta: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_show_check(
        s1: *mut c_char,
        obj: FLA_Obj,
        format: *mut c_char,
        s2: *mut c_char,
    ) -> FLA_Error;
    pub fn FLA_Obj_fshow_check(
        file: *mut FILE,
        s1: *mut c_char,
        obj: FLA_Obj,
        format: *mut c_char,
        s2: *mut c_char,
    ) -> FLA_Error;
    pub fn FLA_Copy_buffer_to_object(
        trans: FLA_Trans,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        obj: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copy_object_to_buffer(
        trans: FLA_Trans,
        i: dim_t,
        j: dim_t,
        obj: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Copy_buffer_to_object_check(
        trans: FLA_Trans,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        obj: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copy_object_to_buffer_check(
        trans: FLA_Trans,
        i: dim_t,
        j: dim_t,
        obj: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_buffer_to_object(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpy_object_to_buffer(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        i: dim_t,
        j: dim_t,
        C: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_buffer_to_object_check(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpy_object_to_buffer_check(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        i: dim_t,
        j: dim_t,
        C: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Obj_nullify(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        rs: dim_t,
        cs: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_ext(
        datatype: FLA_Datatype,
        elemtype: FLA_Elemtype,
        m: dim_t,
        n: dim_t,
        m_inner: dim_t,
        n_inner: dim_t,
        rs: dim_t,
        cs: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_conf_to(trans: FLA_Trans, old: FLA_Obj, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_copy_of(trans: FLA_Trans, old: FLA_Obj, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_without_buffer(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_constant(const_real: f64, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_constant_ext(const_s: f32, const_d: f64, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_complex_constant(
        const_real: f64,
        const_imag: f64,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_attach_buffer(
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_buffer(rs: dim_t, cs: dim_t, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free_without_buffer(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free_buffer(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_align_ldim(ldim: dim_t, elem_size: dim_t) -> dim_t;
    pub fn FLA_compute_num_elem(
        elem_size: dim_t,
        m: dim_t,
        n: dim_t,
        rs: *mut dim_t,
        cs: *mut dim_t,
    ) -> dim_t;
    pub fn FLA_adjust_strides(m: dim_t, n: dim_t, rs: *mut dim_t, cs: *mut dim_t);
    pub fn FLA_Obj_flip_base(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_flip_view(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_ext_check(
        datatype: FLA_Datatype,
        elemtype: FLA_Elemtype,
        m: dim_t,
        n: dim_t,
        m_inner: dim_t,
        n_inner: dim_t,
        rs: dim_t,
        cs: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_conf_to_check(
        trans: FLA_Trans,
        obj_old: FLA_Obj,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_without_buffer_check(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_constant_check(const_real: f64, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_constant_ext_check(
        const_s: f32,
        const_d: f64,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_complex_constant_check(
        const_real: f64,
        const_imag: f64,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_attach_buffer_check(
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        obj: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Obj_create_buffer_check(rs: dim_t, cs: dim_t, obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free_check(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free_without_buffer_check(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_free_buffer_check(obj: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_create_buffer_task(
        rs: dim_t,
        cs: dim_t,
        obj: FLA_Obj,
        cntl: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Obj_free_buffer_task(obj: FLA_Obj, cntl: *mut c_void) -> FLA_Error;
    pub fn FLA_Obj_datatype(obj: FLA_Obj) -> FLA_Datatype;
    pub fn FLA_Obj_datatype_proj_to_real(A: FLA_Obj) -> FLA_Datatype;
    pub fn FLA_Obj_datatype_proj_to_complex(A: FLA_Obj) -> FLA_Datatype;
    pub fn FLA_Obj_elemtype(obj: FLA_Obj) -> FLA_Elemtype;
    pub fn FLA_Obj_datatype_size(datatype: FLA_Datatype) -> dim_t;
    pub fn FLA_Obj_elem_size(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_length(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_width(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_structure(obj: FLA_Obj) -> FLA_Uplo;
    pub fn FLA_Obj_vector_dim(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_vector_inc(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_min_dim(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_max_dim(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_row_stride(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_col_stride(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_row_offset(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_col_offset(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_base_length(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_base_width(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_num_elem_alloc(obj: FLA_Obj) -> dim_t;
    pub fn FLA_Obj_base_buffer(obj: FLA_Obj) -> *mut c_void;
    pub fn FLA_Obj_buffer_at_view(obj: FLA_Obj) -> *mut c_void;
    pub fn FLA_Obj_buffer_is_null(obj: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_int(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_floating_point(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_constant(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_real(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_complex(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_single_precision(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_double_precision(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_scalar(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_vector(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_has_zero_dim(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_row_major(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_col_major(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_conformal_to(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_identical(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_is_overlapped(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_equals(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_gt(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_ge(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_lt(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_le(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Submatrix_at(
        datatype: FLA_Datatype,
        buffer: *mut c_void,
        i: dim_t,
        j: dim_t,
        rs: dim_t,
        cs: dim_t,
    ) -> *mut c_void;
    pub fn FLA_Obj_has_nan(A: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_datatype_check(obj: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_datatype_proj_to_real_check(obj: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_elemtype_check(obj: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_datatype_size_check(datatype: FLA_Datatype) -> FLA_Error;
    pub fn FLA_Obj_elem_size_check(obj: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_buffer_at_view_check(obj: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_equals_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Obj_gt_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_ge_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_lt_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Obj_le_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Bool;
    pub fn FLA_Submatrix_at_check(
        datatype: FLA_Datatype,
        buffer: *mut c_void,
        i: dim_t,
        j: dim_t,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Obj_has_nan_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Param_map_flame_to_netlib_trans(trans: FLA_Trans, blas_trans: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_uplo(uplo: FLA_Uplo, blas_uplo: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_side(side: FLA_Uplo, blas_side: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_diag(diag: FLA_Diag, blas_diag: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_direct(direct: FLA_Direct, lapack_direct: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_storev(storev: FLA_Store, lapack_storev: *mut c_void);
    pub fn FLA_Param_map_flame_to_netlib_evd_type(
        evd_type: FLA_Evd_type,
        lapack_evd_type: *mut c_void,
    );
    pub fn FLA_Param_map_flame_to_netlib_svd_type(
        svd_type: FLA_Svd_type,
        lapack_svd_type: *mut c_void,
    );
    pub fn FLA_Param_map_flame_to_netlib_machval(machval: FLA_Machval, blas_machval: *mut c_void);
    pub fn FLA_Param_map_flame_to_blis_trans(trans: FLA_Trans, blis_trans: *mut trans1_t);
    pub fn FLA_Param_map_flame_to_blis_conj(conj: FLA_Conj, blis_conj: *mut conj1_t);
    pub fn FLA_Param_map_flame_to_blis_uplo(uplo: FLA_Uplo, blis_uplo: *mut uplo1_t);
    pub fn FLA_Param_map_flame_to_blis_side(side: FLA_Uplo, blis_side: *mut side1_t);
    pub fn FLA_Param_map_flame_to_blis_diag(diag: FLA_Diag, blis_diag: *mut diag1_t);
    pub fn FLA_Param_map_blis_to_flame_trans(trans: trans1_t, flame_trans: *mut FLA_Trans);
    pub fn FLA_Param_map_blis_to_flame_uplo(uplo: uplo1_t, flame_uplo: *mut FLA_Uplo);
    pub fn FLA_Param_map_blis_to_flame_side(side: side1_t, flame_side: *mut FLA_Side);
    pub fn FLA_Param_map_blis_to_flame_diag(diag: diag1_t, flame_diag: *mut FLA_Diag);
    pub fn FLA_Param_map_char_to_flame_trans(trans: *mut c_char, flame_trans: *mut FLA_Trans);
    pub fn FLA_Param_map_char_to_flame_uplo(uplo: *mut c_char, flame_uplo: *mut FLA_Uplo);
    pub fn FLA_Param_map_char_to_flame_side(side: *mut c_char, flame_side: *mut FLA_Side);
    pub fn FLA_Param_map_char_to_flame_diag(diag: *mut c_char, flame_diag: *mut FLA_Diag);
    pub fn FLA_Param_map_char_to_flame_storev(storev: *mut c_char, flame_storev: *mut FLA_Direct);
    pub fn FLA_Param_map_char_to_flame_direct(direct: *mut c_char, flame_direct: *mut FLA_Direct);
    pub fn FLA_Param_map_char_to_flame_inv(inv: *mut c_char, flame_inv: *mut FLA_Inv);
    pub fn FLA_Param_map_netlib_to_flame_trans(trans: *mut c_char, flame_trans: *mut FLA_Trans);
    pub fn FLA_Param_map_netlib_to_flame_uplo(uplo: *mut c_char, flame_uplo: *mut FLA_Uplo);
    pub fn FLA_Param_map_netlib_to_flame_side(side: *mut c_char, flame_side: *mut FLA_Side);
    pub fn FLA_Param_map_netlib_to_flame_diag(diag: *mut c_char, flame_diag: *mut FLA_Diag);
    pub fn FLA_Param_map_netlib_to_flame_inv(itype: *mut c_int, flame_inv: *mut FLA_Inv);
    pub fn FLA_Param_map_netlib_to_flame_svd_type(svd: *mut c_char, flame_svd: *mut FLA_Svd_type);
    pub fn FLA_Part_2x2(
        A: FLA_Obj,
        A11: *mut FLA_Obj,
        A12: *mut FLA_Obj,
        A21: *mut FLA_Obj,
        A22: *mut FLA_Obj,
        mb: dim_t,
        nb: dim_t,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Part_2x1(
        A: FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        mb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Part_1x2(
        A: FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        nb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Merge_2x2(
        A11: FLA_Obj,
        A12: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
        A: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Merge_2x1(AT: FLA_Obj, AB: FLA_Obj, A: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Merge_1x2(AL: FLA_Obj, AR: FLA_Obj, A: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Repart_2x2_to_3x3(
        ATL: FLA_Obj,
        ATR: FLA_Obj,
        A00: *mut FLA_Obj,
        A01: *mut FLA_Obj,
        A02: *mut FLA_Obj,
        A10: *mut FLA_Obj,
        A11: *mut FLA_Obj,
        A12: *mut FLA_Obj,
        ABL: FLA_Obj,
        ABR: FLA_Obj,
        A20: *mut FLA_Obj,
        A21: *mut FLA_Obj,
        A22: *mut FLA_Obj,
        mb: dim_t,
        nb: dim_t,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Repart_2x1_to_3x1(
        AT: FLA_Obj,
        A0: *mut FLA_Obj,
        A1: *mut FLA_Obj,
        AB: FLA_Obj,
        A2: *mut FLA_Obj,
        mb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Repart_1x2_to_1x3(
        AL: FLA_Obj,
        AR: FLA_Obj,
        A0: *mut FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        nb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_3x3_to_2x2(
        ATL: *mut FLA_Obj,
        ATR: *mut FLA_Obj,
        A00: FLA_Obj,
        A01: FLA_Obj,
        A02: FLA_Obj,
        A10: FLA_Obj,
        A11: FLA_Obj,
        A12: FLA_Obj,
        ABL: *mut FLA_Obj,
        ABR: *mut FLA_Obj,
        A20: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_3x1_to_2x1(
        AT: *mut FLA_Obj,
        A0: FLA_Obj,
        A1: FLA_Obj,
        AB: *mut FLA_Obj,
        A2: FLA_Obj,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_1x3_to_1x2(
        AL: *mut FLA_Obj,
        AR: *mut FLA_Obj,
        A0: FLA_Obj,
        A1: FLA_Obj,
        A2: FLA_Obj,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Repart_3x3_to_5x5(
        ATL: FLA_Obj,
        ATM: FLA_Obj,
        ATR: FLA_Obj,
        AML: FLA_Obj,
        AMM: FLA_Obj,
        AMR: FLA_Obj,
        ABL: FLA_Obj,
        ABM: FLA_Obj,
        ABR: FLA_Obj,
        A00: *mut FLA_Obj,
        A01: *mut FLA_Obj,
        A02: *mut FLA_Obj,
        A03: *mut FLA_Obj,
        A04: *mut FLA_Obj,
        A10: *mut FLA_Obj,
        A11: *mut FLA_Obj,
        A12: *mut FLA_Obj,
        A13: *mut FLA_Obj,
        A14: *mut FLA_Obj,
        A20: *mut FLA_Obj,
        A21: *mut FLA_Obj,
        A22: *mut FLA_Obj,
        A23: *mut FLA_Obj,
        A24: *mut FLA_Obj,
        A30: *mut FLA_Obj,
        A31: *mut FLA_Obj,
        A32: *mut FLA_Obj,
        A33: *mut FLA_Obj,
        A34: *mut FLA_Obj,
        A40: *mut FLA_Obj,
        A41: *mut FLA_Obj,
        A42: *mut FLA_Obj,
        A43: *mut FLA_Obj,
        A44: *mut FLA_Obj,
        b: dim_t,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_5x5_to_3x3(
        ATL: *mut FLA_Obj,
        ATM: *mut FLA_Obj,
        ATR: *mut FLA_Obj,
        AML: *mut FLA_Obj,
        AMM: *mut FLA_Obj,
        AMR: *mut FLA_Obj,
        ABL: *mut FLA_Obj,
        ABM: *mut FLA_Obj,
        ABR: *mut FLA_Obj,
        A00: FLA_Obj,
        A01: FLA_Obj,
        A02: FLA_Obj,
        A03: FLA_Obj,
        A04: FLA_Obj,
        A10: FLA_Obj,
        A11: FLA_Obj,
        A12: FLA_Obj,
        A13: FLA_Obj,
        A14: FLA_Obj,
        A20: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
        A23: FLA_Obj,
        A24: FLA_Obj,
        A30: FLA_Obj,
        A31: FLA_Obj,
        A32: FLA_Obj,
        A33: FLA_Obj,
        A34: FLA_Obj,
        A40: FLA_Obj,
        A41: FLA_Obj,
        A42: FLA_Obj,
        A43: FLA_Obj,
        A44: FLA_Obj,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Part_2x2_check(
        A: FLA_Obj,
        A11: *mut FLA_Obj,
        A12: *mut FLA_Obj,
        A21: *mut FLA_Obj,
        A22: *mut FLA_Obj,
        mb: dim_t,
        nb: dim_t,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Part_2x1_check(
        A: FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        mb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Part_1x2_check(
        A: FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        nb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Merge_2x2_check(
        A11: FLA_Obj,
        A12: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
        A: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Merge_2x1_check(AT: FLA_Obj, AB: FLA_Obj, A: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Merge_1x2_check(AL: FLA_Obj, AR: FLA_Obj, A: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Repart_2x2_to_3x3_check(
        ATL: FLA_Obj,
        ATR: FLA_Obj,
        A00: *mut FLA_Obj,
        A01: *mut FLA_Obj,
        A02: *mut FLA_Obj,
        A10: *mut FLA_Obj,
        A11: *mut FLA_Obj,
        A12: *mut FLA_Obj,
        ABL: FLA_Obj,
        ABR: FLA_Obj,
        A20: *mut FLA_Obj,
        A21: *mut FLA_Obj,
        A22: *mut FLA_Obj,
        mb: dim_t,
        nb: dim_t,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Repart_2x1_to_3x1_check(
        AT: FLA_Obj,
        A0: *mut FLA_Obj,
        A1: *mut FLA_Obj,
        AB: FLA_Obj,
        A2: *mut FLA_Obj,
        mb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Repart_1x2_to_1x3_check(
        AL: FLA_Obj,
        AR: FLA_Obj,
        A0: *mut FLA_Obj,
        A1: *mut FLA_Obj,
        A2: *mut FLA_Obj,
        nb: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_3x3_to_2x2_check(
        ATL: *mut FLA_Obj,
        ATR: *mut FLA_Obj,
        A00: FLA_Obj,
        A01: FLA_Obj,
        A02: FLA_Obj,
        A10: FLA_Obj,
        A11: FLA_Obj,
        A12: FLA_Obj,
        ABL: *mut FLA_Obj,
        ABR: *mut FLA_Obj,
        A20: FLA_Obj,
        A21: FLA_Obj,
        A22: FLA_Obj,
        quadrant: FLA_Quadrant,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_3x1_to_2x1_check(
        AT: *mut FLA_Obj,
        A0: FLA_Obj,
        A1: FLA_Obj,
        AB: *mut FLA_Obj,
        A2: FLA_Obj,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_Cont_with_1x3_to_1x2_check(
        AL: *mut FLA_Obj,
        AR: *mut FLA_Obj,
        A0: FLA_Obj,
        A1: FLA_Obj,
        A2: FLA_Obj,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLA_random_float() -> f32;
    pub fn FLA_random_double() -> f64;
    pub fn FLA_random_scomplex() -> scomplex;
    pub fn FLA_random_dcomplex() -> dcomplex;
    pub fn FLA_Absolute_square(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Absolute_value(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Clock() -> f64;
    pub fn FLA_Conjugate(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Conjugate_r(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_linear_dist(shift: FLA_Obj, delta: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_inverse_dist(alpha: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_geometric_dist(alpha: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_random_dist(shift: FLA_Obj, max: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_logarithmic_dist(max: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_cluster_dist(
        n_clusters: FLA_Obj,
        cluster_width: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hermitianize(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Invert(conj: FLA_Conj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scal_elemwise(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_abs_value(A: FLA_Obj, amax: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_abs_value_herm(uplo: FLA_Uplo, A: FLA_Obj, maxabs: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_elemwise_diff(A: FLA_Obj, B: FLA_Obj) -> f64;
    pub fn FLA_Mult_add(alpha: FLA_Obj, beta: FLA_Obj, gamma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Negate(x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm1(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm_inf(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm_frob(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Pow(base: FLA_Obj, exp: FLA_Obj, btoe: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_matrix(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_herm_matrix(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_symm_matrix(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_spd_matrix(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_tri_matrix(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_unitary_matrix(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scal_elemwise(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Setr(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Shift_pivots_to_check(ptype: FLA_Pivot_type, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sqrt(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Symmetrize(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Triangularize(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Transpose(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_diag(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_offdiag(offset: c_int, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_to_identity(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Add_to_diag(diag_value: *mut c_void, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Shift_diag(conj: FLA_Conj, sigma: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scale_diag(conj: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_diagonal_vector(A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_diagonal_matrix(d: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Absolute_square_check(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Absolute_value_check(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Conjugate_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Conjugate_r_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_linear_dist_check(shift: FLA_Obj, delta: FLA_Obj, x: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Fill_with_inverse_dist_check(alpha: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_geometric_dist_check(alpha: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_random_dist_check(shift: FLA_Obj, max: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_logarithmic_dist_check(alpha: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Fill_with_cluster_dist_check(
        n_clusters: FLA_Obj,
        cluster_width: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hermitianize_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Invert_check(conj: FLA_Conj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scal_elemwise_check(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_abs_value_check(A: FLA_Obj, amax: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_abs_value_herm_check(uplo: FLA_Uplo, A: FLA_Obj, maxabs: FLA_Obj) -> FLA_Error;
    pub fn FLA_Max_elemwise_diff_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Mult_add_check(alpha: FLA_Obj, beta: FLA_Obj, gamma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Negate_check(x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm1_check(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm_inf_check(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm_frob_check(A: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Pow_check(base: FLA_Obj, exp: FLA_Obj, btoe: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_matrix_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_herm_matrix_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_symm_matrix_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_spd_matrix_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_tri_matrix_check(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Random_unitary_matrix_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scal_elemwise_check(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Setr_check(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_check(direct: FLA_Direct, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sqrt_check(alpha: FLA_Obj) -> FLA_Error;
    pub fn FLA_Symmetrize_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Triangularize_check(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Transpose_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_check(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_diag_check(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Set_to_identity_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Add_to_diag_check(diag_value: *mut c_void, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Shift_diag_check(conj: FLA_Conj, sigma: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scale_diag_check(conj: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Transpose_blk_var1(A: FLA_Obj, cntl: *mut fla_tpose_t) -> FLA_Error;
    pub fn FLA_Transpose_blk_var2(A: FLA_Obj, cntl: *mut fla_tpose_t) -> FLA_Error;
    pub fn FLA_Transpose_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Transpose_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swap_t_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_swap_t) -> FLA_Error;
    pub fn FLA_Swap_t_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_swap_t) -> FLA_Error;
    pub fn FLA_Sort(direct: FLA_Direct, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_f_ops(m_x: c_int, x: *mut f32, inc_x: c_int) -> FLA_Error;
    pub fn FLA_Sort_b_ops(m_x: c_int, x: *mut f32, inc_x: c_int) -> FLA_Error;
    pub fn FLA_Sort_f_opd(m_x: c_int, x: *mut f64, inc_x: c_int) -> FLA_Error;
    pub fn FLA_Sort_b_opd(m_x: c_int, x: *mut f64, inc_x: c_int) -> FLA_Error;
    pub fn FLA_Househ2_UT(side: FLA_Side, chi_1: FLA_Obj, x2: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_Househ2_UT_l_ops(
        m_x2: c_int,
        chi_1: *mut f32,
        x2: *mut f32,
        inc_x2: c_int,
        tau: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_l_opd(
        m_x2: c_int,
        chi_1: *mut f64,
        x2: *mut f64,
        inc_x2: c_int,
        tau: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_l_opc(
        m_x2: c_int,
        chi_1: *mut scomplex,
        x2: *mut scomplex,
        inc_x2: c_int,
        tau: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_l_opz(
        m_x2: c_int,
        chi_1: *mut dcomplex,
        x2: *mut dcomplex,
        inc_x2: c_int,
        tau: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_r_ops(
        m_x2: c_int,
        chi_1: *mut f32,
        x2: *mut f32,
        inc_x2: c_int,
        tau: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_r_opd(
        m_x2: c_int,
        chi_1: *mut f64,
        x2: *mut f64,
        inc_x2: c_int,
        tau: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_r_opc(
        m_x2: c_int,
        chi_1: *mut scomplex,
        x2: *mut scomplex,
        inc_x2: c_int,
        tau: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2_UT_r_opz(
        m_x2: c_int,
        chi_1: *mut dcomplex,
        x2: *mut dcomplex,
        inc_x2: c_int,
        tau: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ3UD_UT(chi_1: FLA_Obj, x2: FLA_Obj, y2: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_Househ3UD_UT_ops(
        m_x2: c_int,
        m_y2: c_int,
        chi_1: *mut f32,
        x2: *mut f32,
        inc_x2: c_int,
        y2: *mut f32,
        inc_y2: c_int,
        tau: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Househ3UD_UT_opd(
        m_x2: c_int,
        m_y2: c_int,
        chi_1: *mut f64,
        x2: *mut f64,
        inc_x2: c_int,
        y2: *mut f64,
        inc_y2: c_int,
        tau: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Househ3UD_UT_opc(
        m_x2: c_int,
        m_y2: c_int,
        chi_1: *mut scomplex,
        x2: *mut scomplex,
        inc_x2: c_int,
        y2: *mut scomplex,
        inc_y2: c_int,
        tau: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ3UD_UT_opz(
        m_x2: c_int,
        m_y2: c_int,
        chi_1: *mut dcomplex,
        x2: *mut dcomplex,
        inc_x2: c_int,
        y2: *mut dcomplex,
        inc_y2: c_int,
        tau: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT(
        side: FLA_Side,
        chi_1: FLA_Obj,
        x2: FLA_Obj,
        alpha: FLA_Obj,
        chi_1_minus_alpha: FLA_Obj,
        tau: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_l_ops(
        m_x2: c_int,
        chi_1: *mut f32,
        x2: *mut f32,
        inc_x2: c_int,
        alpha: *mut f32,
        chi_1_minus_alpha: *mut f32,
        tau: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_l_opd(
        m_x2: c_int,
        chi_1: *mut f64,
        x2: *mut f64,
        inc_x2: c_int,
        alpha: *mut f64,
        chi_1_minus_alpha: *mut f64,
        tau: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_l_opc(
        m_x2: c_int,
        chi_1: *mut scomplex,
        x2: *mut scomplex,
        inc_x2: c_int,
        alpha: *mut scomplex,
        chi_1_minus_alpha: *mut scomplex,
        tau: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_l_opz(
        m_x2: c_int,
        chi_1: *mut dcomplex,
        x2: *mut dcomplex,
        inc_x2: c_int,
        alpha: *mut dcomplex,
        chi_1_minus_alpha: *mut dcomplex,
        tau: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_r_ops(
        m_x2: c_int,
        chi_1: *mut f32,
        x2: *mut f32,
        inc_x2: c_int,
        alpha: *mut f32,
        chi_1_minus_alpha: *mut f32,
        tau: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_r_opd(
        m_x2: c_int,
        chi_1: *mut f64,
        x2: *mut f64,
        inc_x2: c_int,
        alpha: *mut f64,
        chi_1_minus_alpha: *mut f64,
        tau: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_r_opc(
        m_x2: c_int,
        chi_1: *mut scomplex,
        x2: *mut scomplex,
        inc_x2: c_int,
        alpha: *mut scomplex,
        chi_1_minus_alpha: *mut scomplex,
        tau: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_r_opz(
        m_x2: c_int,
        chi_1: *mut dcomplex,
        x2: *mut dcomplex,
        inc_x2: c_int,
        alpha: *mut dcomplex,
        chi_1_minus_alpha: *mut dcomplex,
        tau: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Hev_2x2(
        alpha11: FLA_Obj,
        alpha21: FLA_Obj,
        alpha22: FLA_Obj,
        lambda1: FLA_Obj,
        lambda2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hev_2x2_ops(
        buff_alpha11: *mut f32,
        buff_alpha21: *mut f32,
        buff_alpha22: *mut f32,
        buff_lambda1: *mut f32,
        buff_lambda2: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Hev_2x2_opd(
        buff_alpha11: *mut f64,
        buff_alpha21: *mut f64,
        buff_alpha22: *mut f64,
        buff_lambda1: *mut f64,
        buff_lambda2: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Hevv_2x2(
        alpha11: FLA_Obj,
        alpha21: FLA_Obj,
        alpha22: FLA_Obj,
        lambda1: FLA_Obj,
        lambda2: FLA_Obj,
        gamma1: FLA_Obj,
        sigma1: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hevv_2x2_ops(
        alpha11: *mut f32,
        alpha21: *mut f32,
        alpha22: *mut f32,
        lambda1: *mut f32,
        lambda2: *mut f32,
        gamma1: *mut f32,
        sigma1: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Hevv_2x2_opd(
        alpha11: *mut f64,
        alpha21: *mut f64,
        alpha22: *mut f64,
        lambda1: *mut f64,
        lambda2: *mut f64,
        gamma1: *mut f64,
        sigma1: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Hevv_2x2_opc(
        alpha11: *mut scomplex,
        alpha21: *mut scomplex,
        alpha22: *mut scomplex,
        lambda1: *mut f32,
        lambda2: *mut f32,
        gamma1: *mut f32,
        sigma1: *mut scomplex,
    ) -> FLA_Error;
    pub fn FLA_Hevv_2x2_opz(
        alpha11: *mut dcomplex,
        alpha21: *mut dcomplex,
        alpha22: *mut dcomplex,
        lambda1: *mut f64,
        lambda2: *mut f64,
        gamma1: *mut f64,
        sigma1: *mut dcomplex,
    ) -> FLA_Error;
    pub fn FLA_Wilkshift_tridiag(
        delta1: FLA_Obj,
        epsilon: FLA_Obj,
        delta2: FLA_Obj,
        kappa: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Wilkshift_tridiag_ops(
        delta1: f32,
        epsilon: f32,
        delta2: f32,
        kappa: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Wilkshift_tridiag_opd(
        delta1: f64,
        epsilon: f64,
        delta2: f64,
        kappa: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Pythag2(chi: FLA_Obj, psi: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Pythag2_ops(chi: *mut f32, psi: *mut f32, rho: *mut f32) -> FLA_Error;
    pub fn FLA_Pythag2_opd(chi: *mut f64, psi: *mut f64, rho: *mut f64) -> FLA_Error;
    pub fn FLA_Pythag3(chi: FLA_Obj, psi: FLA_Obj, zeta: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Pythag3_ops(
        chi: *mut f32,
        psi: *mut f32,
        zeta: *mut f32,
        rho: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Pythag3_opd(
        chi: *mut f64,
        psi: *mut f64,
        zeta: *mut f64,
        rho: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd(direct: FLA_Direct, l: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_evd_f_ops(
        m_A: c_int,
        l: *mut f32,
        inc_l: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_b_ops(
        m_A: c_int,
        l: *mut f32,
        inc_l: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_f_opd(
        m_A: c_int,
        l: *mut f64,
        inc_l: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_b_opd(
        m_A: c_int,
        l: *mut f64,
        inc_l: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_f_opc(
        m_A: c_int,
        l: *mut f32,
        inc_l: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_b_opc(
        m_A: c_int,
        l: *mut f32,
        inc_l: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_f_opz(
        m_A: c_int,
        l: *mut f64,
        inc_l: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_evd_b_opz(
        m_A: c_int,
        l: *mut f64,
        inc_l: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext(
        direct: FLA_Direct,
        s: FLA_Obj,
        apply_U: FLA_Bool,
        U: FLA_Obj,
        apply_V: FLA_Bool,
        V: FLA_Obj,
        apply_C: FLA_Bool,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_f_ops(
        m_s: c_int,
        s: *mut f32,
        inc_s: c_int,
        m_U: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_b_ops(
        m_s: c_int,
        s: *mut f32,
        inc_s: c_int,
        m_U: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_f_opd(
        m_s: c_int,
        s: *mut f64,
        inc_s: c_int,
        m_U: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_b_opd(
        m_s: c_int,
        s: *mut f64,
        inc_s: c_int,
        m_U: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_f_opc(
        m_s: c_int,
        s: *mut f32,
        inc_s: c_int,
        m_U: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_b_opc(
        m_s: c_int,
        s: *mut f32,
        inc_s: c_int,
        m_U: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_f_opz(
        m_s: c_int,
        s: *mut f64,
        inc_s: c_int,
        m_U: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_bsvd_ext_b_opz(
        m_s: c_int,
        s: *mut f64,
        inc_s: c_int,
        m_U: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        m_V: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        n_C: c_int,
        C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd(direct: FLA_Direct, s: FLA_Obj, U: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_svd_f_ops(
        m_U: c_int,
        n_V: c_int,
        s: *mut f32,
        inc_s: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_b_ops(
        m_U: c_int,
        n_V: c_int,
        s: *mut f32,
        inc_s: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_f_opd(
        m_U: c_int,
        n_V: c_int,
        s: *mut f64,
        inc_s: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_b_opd(
        m_U: c_int,
        n_V: c_int,
        s: *mut f64,
        inc_s: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_f_opc(
        m_U: c_int,
        n_V: c_int,
        s: *mut f32,
        inc_s: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_b_opc(
        m_U: c_int,
        n_V: c_int,
        s: *mut f32,
        inc_s: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_f_opz(
        m_U: c_int,
        n_V: c_int,
        s: *mut f64,
        inc_s: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sort_svd_b_opz(
        m_U: c_int,
        n_V: c_int,
        s: *mut f64,
        inc_s: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
    ) -> FLA_Error;
    pub fn FLA_Sv_2x2(
        alpha11: FLA_Obj,
        alpha12: FLA_Obj,
        alpha22: FLA_Obj,
        sigma1: FLA_Obj,
        sigma2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sv_2x2_ops(
        alpha11: *mut f32,
        alpha12: *mut f32,
        alpha22: *mut f32,
        sigma1: *mut f32,
        sigma2: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Sv_2x2_opd(
        alpha11: *mut f64,
        alpha12: *mut f64,
        alpha22: *mut f64,
        sigma1: *mut f64,
        sigma2: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Svv_2x2(
        alpha11: FLA_Obj,
        alpha12: FLA_Obj,
        alpha22: FLA_Obj,
        sigma1: FLA_Obj,
        sigma2: FLA_Obj,
        gammaL: FLA_Obj,
        sigmaL: FLA_Obj,
        gammaR: FLA_Obj,
        sigmaR: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svv_2x2_ops(
        alpha11: *mut f32,
        alpha12: *mut f32,
        alpha22: *mut f32,
        sigma1: *mut f32,
        sigma2: *mut f32,
        gammaL: *mut f32,
        sigmaL: *mut f32,
        gammaR: *mut f32,
        sigmaR: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Svv_2x2_opd(
        alpha11: *mut f64,
        alpha12: *mut f64,
        alpha22: *mut f64,
        sigma1: *mut f64,
        sigma2: *mut f64,
        gammaL: *mut f64,
        sigmaL: *mut f64,
        gammaR: *mut f64,
        sigmaR: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Mach_params(machval: FLA_Machval, val: FLA_Obj) -> FLA_Error;
    pub fn FLA_Mach_params_ops(machval: FLA_Machval) -> f32;
    pub fn FLA_Mach_params_opd(machval: FLA_Machval) -> f64;
    pub fn FLA_Apply_diag_matrix(
        side: FLA_Side,
        conj: FLA_Conj,
        x: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Shift_pivots_to(ptype: FLA_Pivot_type, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_Form_perm_matrix(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_find_zero_on_diagonal(A: FLA_Obj) -> FLA_Error;
    pub fn fla_dlamch(cmach: *mut c_char, cmach_len: ftnlen) -> doublereal;
    pub fn fla_slamch(cmach: *mut c_char, cmach_len: ftnlen) -> real;
    pub fn fla_lsame(ca: *mut c_char, cb: *mut c_char, ca_len: ftnlen, cb_len: ftnlen) -> logical;
    pub fn fla_pow_di(a: *mut doublereal, n: *mut integer) -> f64;
    pub fn fla_pow_ri(a: *mut real, n: *mut integer) -> real;
    pub fn FLA_Househ2_UT_check(
        side: FLA_Side,
        chi_1: FLA_Obj,
        x2: FLA_Obj,
        tau: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Househ3UD_UT_check(
        chi_1: FLA_Obj,
        x2: FLA_Obj,
        y2: FLA_Obj,
        tau: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Househ2s_UT_check(
        side: FLA_Side,
        chi_1: FLA_Obj,
        x2: FLA_Obj,
        alpha: FLA_Obj,
        chi_1_minus_alpha: FLA_Obj,
        tau: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Givens2_check(
        chi_1: FLA_Obj,
        chi_2: FLA_Obj,
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        chi_1_new: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_GTG_check(
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        delta1: FLA_Obj,
        epsilon1: FLA_Obj,
        delta2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_1x2_check(
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        beta: FLA_Obj,
        epsilon: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_mx2_check(
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        a1: FLA_Obj,
        a2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_check(
        side: FLA_Side,
        direct: FLA_Direct,
        G: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Wilkshift_tridiag_check(
        delta1: FLA_Obj,
        epsilon: FLA_Obj,
        delta2: FLA_Obj,
        kappa: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Wilkshift_bidiag_check(
        epsilon1: FLA_Obj,
        delta1: FLA_Obj,
        epsilon2: FLA_Obj,
        delta2: FLA_Obj,
        kappa: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Introduce_bulge_check(
        shift: FLA_Obj,
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        delta1: FLA_Obj,
        epsilon1: FLA_Obj,
        delta2: FLA_Obj,
        beta: FLA_Obj,
        epsilon2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Mach_params_check(machval: FLA_Machval, val: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_evd_check(direct: FLA_Direct, l: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sort_svd_check(direct: FLA_Direct, s: FLA_Obj, U: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_diag_matrix_check(
        side: FLA_Side,
        conj: FLA_Conj,
        x: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Form_perm_matrix_check(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_find_zero_on_diagonal_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Asum(x: FLA_Obj, asum_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpys(
        alpha0: FLA_Obj,
        alpha1: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpyt(trans: FLA_Trans, alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpyrt(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copy(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyr(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyrt(uplo: FLA_Uplo, trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyt(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dot(x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dot2cs(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dot2s(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dotc(conj: FLA_Conj, x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dotcs(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dots(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Amax(x: FLA_Obj, index: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scal(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scalc(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Nrm2(x: FLA_Obj, norm_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scal(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalc(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalr(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swap(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swapt(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_task(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copy_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copyt_task(
        trans: FLA_Trans,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copyr_task(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyr_t,
    ) -> FLA_Error;
    pub fn FLA_Scal_task(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scalr_task(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_scalr_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_t_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_c_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_h_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copyt_n_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyr_l_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u_task(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Scalr_l_task(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u_task(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Asum_external(x: FLA_Obj, asum_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy_external(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpys_external(
        alpha0: FLA_Obj,
        alpha1: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_external(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpyrt_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copy_external(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyr_external(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyrt_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copyt_external(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dot_external(x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dotc_external(conj: FLA_Conj, x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dots_external(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dotcs_external(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dot2s_external(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dot2cs_external(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Amax_external(x: FLA_Obj, index: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scal_external(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scalc_external(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Nrm2_external(x: FLA_Obj, nrm_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scal_external(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalc_external(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalr_external(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swap_external(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swapt_external(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy_external_gpu(
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Copy_external_gpu(
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Scal_external_gpu(alpha: FLA_Obj, A: FLA_Obj, A_gpu: *mut c_void) -> FLA_Error;
    pub fn FLA_Scalr_external_gpu(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Asum_check(x: FLA_Obj, asum_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy_check(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpys_check(
        alpha0: FLA_Obj,
        alpha1: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_check(trans: FLA_Trans, alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpyrt_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Copy_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyr_check(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyrt_check(uplo: FLA_Uplo, trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyt_check(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dot_check(x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dotc_check(conj: FLA_Conj, x: FLA_Obj, y: FLA_Obj, rho: FLA_Obj) -> FLA_Error;
    pub fn FLA_Dots_check(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dotcs_check(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dot2s_check(
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Dot2cs_check(
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        beta: FLA_Obj,
        rho: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Amax_check(x: FLA_Obj, index: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scal_check(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Inv_scalc_check(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Nrm2_check(x: FLA_Obj, nrm_x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scal_check(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalc_check(conjalpha: FLA_Conj, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Scalr_check(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swap_check(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Swapt_check(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Axpy_internal_check(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_internal_check(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copy_internal_check(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copyt_internal_check(
        trans: FLA_Trans,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copyr_internal_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyr_t,
    ) -> FLA_Error;
    pub fn FLA_Scal_internal_check(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scalr_internal_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_scalr_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemvc(
        transa: FLA_Trans,
        conjx: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Ger(alpha: FLA_Obj, x: FLA_Obj, y: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Gerc(
        conjx: FLA_Conj,
        conjy: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemv(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemvc(
        uplo: FLA_Uplo,
        conja: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Herc(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2c(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symv(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Syr2(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmv(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmvsx(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsv(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsvsx(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemv_task(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_task(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_h_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lc_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ln_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lt_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_uc_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_un_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ut_task(
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_external(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemvc_external(
        transa: FLA_Trans,
        conjx: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Ger_external(alpha: FLA_Obj, x: FLA_Obj, y: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Gerc_external(
        conjx: FLA_Conj,
        conjy: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemv_external(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemvc_external(
        uplo: FLA_Uplo,
        conja: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her_external(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Herc_external(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2_external(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2c_external(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symv_external(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr_external(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Syr2_external(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmv_external(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmvsx_external(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsv_external(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsvsx_external(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemv_external_gpu(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        x: FLA_Obj,
        x_gpu: *mut c_void,
        beta: FLA_Obj,
        y: FLA_Obj,
        y_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Trsv_external_gpu(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        x: FLA_Obj,
        x_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Gemv_check(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemvc_check(
        transa: FLA_Trans,
        conjx: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Ger_check(alpha: FLA_Obj, x: FLA_Obj, y: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Gerc_check(
        conjx: FLA_Conj,
        conjy: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemv_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemvc_check(
        uplo: FLA_Uplo,
        conja: FLA_Conj,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her_check(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Herc_check(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2c_check(
        uplo: FLA_Uplo,
        conj: FLA_Conj,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symv_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr_check(uplo: FLA_Uplo, alpha: FLA_Obj, x: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Syr2_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        x: FLA_Obj,
        y: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmv_check(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmvsx_check(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsv_check(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsvsx_check(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemv_internal_check(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_internal_check(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Herk(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syrk(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmmsx(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsmsx(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemp(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gepm(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gepp(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_task(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_task(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_task(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_task(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_task(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_task(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_task(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_task(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_task(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_task(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_task(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_external(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Herk_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syrk_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_external(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmmsx_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsmsx_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_external_gpu(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Hemm_external_gpu(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Herk_external_gpu(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Her2k_external_gpu(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Symm_external_gpu(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Syrk_external_gpu(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_external_gpu(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
        beta: FLA_Obj,
        C: FLA_Obj,
        C_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Trmm_external_gpu(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Trsm_external_gpu(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        A_gpu: *mut c_void,
        B: FLA_Obj,
        B_gpu: *mut c_void,
    ) -> FLA_Error;
    pub fn FLA_Gemm_check(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Herk_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syrk_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmmsx_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsmsx_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_internal_check(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_internal_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_internal_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_internal_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_internal_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_internal_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_internal_check(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_internal_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_internal_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Chol(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_piv(A: FLA_Obj, T: FLA_Obj, w: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT(A: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sylv(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_SPDinv(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest(inv: FLA_Inv, uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT(
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        tau: FLA_Obj,
        T: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT(
        side: FLA_Side,
        tau: FLA_Obj,
        u2: FLA_Obj,
        a1: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT(
        side: FLA_Side,
        tau: FLA_Obj,
        w12t: FLA_Obj,
        u2: FLA_Obj,
        v2: FLA_Obj,
        r12t: FLA_Obj,
        C2: FLA_Obj,
        D2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots(side: FLA_Side, trans: FLA_Trans, p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_task(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_l_task(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_u_task(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_LU_piv_macro_task(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_Apply_pivots_task(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_appiv_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_task(p: FLA_Obj, A: FLA_Obj, cntl: *mut fla_appiv_t) -> FLA_Error;
    pub fn FLA_Apply_pivots_macro_task(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_appiv_t,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_task(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_task(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_copy_task(
        A: FLA_Obj,
        p: FLA_Obj,
        U: FLA_Obj,
        cntl: *mut fla_lu_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_piv_task(
        A: FLA_Obj,
        B: FLA_Obj,
        p: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_SA_LU_task(
        U: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_lu_t,
    ) -> FLA_Error;
    pub fn FLA_SA_FS_task(
        L: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Trinv_task(
        uplo: FLA_Uplo,
        diag: FLA_Diag,
        A: FLA_Obj,
        cntl: *mut fla_trinv_t,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_task(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu_task(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un_task(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu_task(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Ttmm_task(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_l_task(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_u_task(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Sylv_task(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_task(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_task(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_task(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfc_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfr_task(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_task(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lhfc_task(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_task(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_lhfc_task(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_task(B: FLA_Obj, D: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qr2ut_t)
        -> FLA_Error;
    pub fn FLA_CAQR2_UT_task(
        B: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_caqr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_macro_task(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_task(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_copy_task(
        A: FLA_Obj,
        T: FLA_Obj,
        U: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_macro_task(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_LQ_UT_task(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_UDdate_UT_task(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_uddateut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_task(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_lhfc_task(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_task(
        inv: FLA_Inv,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_task(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_task(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_task(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_task(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_blk_external(
        side: FLA_Side,
        trans: FLA_Trans,
        storev: FLA_Store,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_unb_external(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_unb_ext(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_pivots_macro_external(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Chol_blk_external(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_unb_external(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_blk_external(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_blk_ext(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_unb_external(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_unb_ext(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_blk_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_unb_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_blk_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_unb_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_blk_external(A: FLA_Obj, t: FLA_Obj, ilo: c_int, ihi: c_int) -> FLA_Error;
    pub fn FLA_Hess_unb_external(A: FLA_Obj, t: FLA_Obj, ilo: c_int, ihi: c_int) -> FLA_Error;
    pub fn FLA_Tridiag_blk_external(uplo: FLA_Uplo, A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_unb_external(uplo: FLA_Uplo, A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_blk_external(A: FLA_Obj, tu: FLA_Obj, tv: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_unb_external(A: FLA_Obj, tu: FLA_Obj, tv: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_form_Q_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_form_Q_external(uplo: FLA_Uplo, A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_apply_Q_external(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_form_U_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_form_V_external(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_apply_U_external(
        side: FLA_Side,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_apply_V_external(
        side: FLA_Side,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trinv_blk_external(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_unb_external(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_blk_external(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_blk_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_unb_external(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_unb_ext(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sylv_blk_external(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_unb_external(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_unb_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_unb_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_unb_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_unb_ext(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_SPDinv_blk_external(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_blk_external(
        inv: FLA_Inv,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_unb_external(
        inv: FLA_Inv,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_ext(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tevd_external(jobz: FLA_Evd_type, d: FLA_Obj, e: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tevdd_external(jobz: FLA_Evd_type, d: FLA_Obj, e: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tevdr_external(
        jobz: FLA_Evd_type,
        d: FLA_Obj,
        e: FLA_Obj,
        l: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hevd_external(
        jobz: FLA_Evd_type,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        l: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hevdd_external(
        jobz: FLA_Evd_type,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        l: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hevdr_external(
        jobz: FLA_Evd_type,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        l: FLA_Obj,
        Z: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_external(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvdd_external(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svd_external(
        jobu: FLA_Svd_type,
        jobv: FLA_Svd_type,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svdd_external(
        jobz: FLA_Svd_type,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Chol_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_solve_check(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_solve_check(A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_check(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_solve_check(A: FLA_Obj, p: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_incpiv_check(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_incpiv_solve_check(
        A: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        B: FLA_Obj,
        X: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_FS_incpiv_check(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj, b: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_check(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_check(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_solve_check(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_recover_tau_check(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_check(A: FLA_Obj, T: FLA_Obj, Q: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_check(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_check(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_solve_check(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_recover_tau_check(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_form_Q_check(A: FLA_Obj, T: FLA_Obj, Q: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_check(A: FLA_Obj, t: FLA_Obj, ilo: c_int, ihi: c_int) -> FLA_Error;
    pub fn FLA_Hess_UT_check(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_recover_tau_check(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_check(uplo: FLA_Uplo, A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_check(uplo: FLA_Uplo, A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_recover_tau_check(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_scale_diagonals_check(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_extract_diagonals_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_extract_real_diagonals_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_realify_check(uplo: FLA_Uplo, A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_realify_subdiagonal_check(b: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        T: FLA_Obj,
        Q: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trinv_check(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_check(A: FLA_Obj, tu: FLA_Obj, tv: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_check(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_recover_tau_check(
        TU: FLA_Obj,
        TV: FLA_Obj,
        tu: FLA_Obj,
        tv: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_extract_diagonals_check(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_extract_real_diagonals_check(
        A: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_scale_diagonals_check(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_realify_check(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_realify_diagonals_check(
        uplo: FLA_Uplo,
        a: FLA_Obj,
        b: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_U_check(A: FLA_Obj, T: FLA_Obj, U: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_V_check(A: FLA_Obj, S: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Sylv_check(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Lyap_check(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_SPDinv_check(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_check(inv: FLA_Inv, uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_Q_check(
        side: FLA_Side,
        trans: FLA_Trans,
        storev: FLA_Store,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_QR_form_Q_check(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_form_Q_check(uplo: FLA_Uplo, A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_apply_Q_check(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_form_U_check(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_form_V_check(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_apply_U_check(
        side: FLA_Side,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_apply_V_check(
        side: FLA_Side,
        trans: FLA_Trans,
        A: FLA_Obj,
        t: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_check(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_check(B: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR2_UT_check(B: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_inc_check(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_inc_solve_check(A: FLA_Obj, TW: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR_UT_inc_solve_check(
        p: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
        B: FLA_Obj,
        X: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_check(R: FLA_Obj, C: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_update_rhs_check(
        T: FLA_Obj,
        bR: FLA_Obj,
        C: FLA_Obj,
        bC: FLA_Obj,
        D: FLA_Obj,
        bD: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_solve_check(R: FLA_Obj, bR: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_inc_check(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_inc_update_rhs_check(
        T: FLA_Obj,
        bR: FLA_Obj,
        C: FLA_Obj,
        bC: FLA_Obj,
        D: FLA_Obj,
        bD: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_inc_solve_check(R: FLA_Obj, bR: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR_UT_inc_check(
        p: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_inc_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_check(
        side: FLA_Side,
        tau: FLA_Obj,
        u2: FLA_Obj,
        a1t: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_check(
        side: FLA_Side,
        tau: FLA_Obj,
        w12t: FLA_Obj,
        u2: FLA_Obj,
        v2: FLA_Obj,
        r12t: FLA_Obj,
        C2: FLA_Obj,
        D2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_check(
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        tau: FLA_Obj,
        T: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tevd_compute_scaling_check(d: FLA_Obj, e: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hevd_compute_scaling_check(uplo: FLA_Uplo, A: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hevd_check(jobz: FLA_Evd_type, uplo: FLA_Uplo, A: FLA_Obj, l: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hevdd_check(jobz: FLA_Evd_type, uplo: FLA_Uplo, A: FLA_Obj, l: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Hevdr_check(
        jobz: FLA_Evd_type,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        l: FLA_Obj,
        Z: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_check(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        jobu: FLA_Svd_type,
        U: FLA_Obj,
        jobv: FLA_Svd_type,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_check(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        jobu: FLA_Svd_type,
        U: FLA_Obj,
        jobv: FLA_Svd_type,
        V: FLA_Obj,
        apply_Uh2C: FLA_Bool,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_scaling_check(d: FLA_Obj, e: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Svd_compute_scaling_check(A: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Svd_check(
        jobu: FLA_Svd_type,
        jobv: FLA_Svd_type,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svd_ext_check(
        jobu: FLA_Svd_type,
        transu: FLA_Trans,
        jobv: FLA_Svd_type,
        transv: FLA_Trans,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svdd_check(
        jobz: FLA_Svd_type,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Chol_internal_check(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_internal_check(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_Trinv_internal_check(
        uplo: FLA_Uplo,
        diag: FLA_Diag,
        A: FLA_Obj,
        cntl: *mut fla_trinv_t,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_internal_check(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_SPDinv_internal_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        cntl: *mut fla_spdinv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_internal_check(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_internal_check(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_internal_check(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_copy_internal_check(
        A: FLA_Obj,
        T: FLA_Obj,
        U: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_internal_check(
        B: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_qr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_internal_check(
        B: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_caqr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_internal_check(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_Hess_UT_internal_check(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_hessut_t)
        -> FLA_Error;
    pub fn FLA_Tridiag_UT_internal_check(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_tridiagut_t,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_internal_check(
        A: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
        cntl: *mut fla_bidiagut_t,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_internal_check(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_uddateut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        R: FLA_Obj,
        TW: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apcaqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_inc_internal_check(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_internal_check(
        inv: FLA_Inv,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpy_internal(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpy_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_t_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_t_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_t_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_t_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_c_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_c_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_c_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_c_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_h_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_h_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_h_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_h_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_internal(
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_axpyt_t,
    ) -> FLA_Error;
    pub fn FLA_Axpyt_n(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_axpyt_t)
        -> FLA_Error;
    pub fn FLA_Axpyt_t(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_axpyt_t)
        -> FLA_Error;
    pub fn FLA_Axpyt_c(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_axpyt_t)
        -> FLA_Error;
    pub fn FLA_Axpyt_h(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_axpyt_t)
        -> FLA_Error;
    pub fn FLA_Copy_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copy_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copy_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copy_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copy_internal(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copy_t) -> FLA_Error;
    pub fn FLA_Copyt_n_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_n_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_n_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_n_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_internal(
        trans: FLA_Trans,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyt_t,
    ) -> FLA_Error;
    pub fn FLA_Copyt_n(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_t(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_c(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyt_h(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyt_t) -> FLA_Error;
    pub fn FLA_Copyr_l_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_l_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_l_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_l_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u_blk_var1(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u_blk_var2(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u_blk_var3(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u_blk_var4(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLASH_Copyr(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Copyr_internal(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_copyr_t,
    ) -> FLA_Error;
    pub fn FLA_Copyr_l(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Copyr_u(A: FLA_Obj, B: FLA_Obj, cntl: *mut fla_copyr_t) -> FLA_Error;
    pub fn FLA_Scal_blk_var1(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scal_blk_var2(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scal_blk_var3(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scal_blk_var4(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scal_internal(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scal_t) -> FLA_Error;
    pub fn FLA_Scalr_l_blk_var1(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_l_blk_var2(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_l_blk_var3(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_l_blk_var4(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u_blk_var1(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u_blk_var2(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u_blk_var3(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u_blk_var4(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_internal(
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_scalr_t,
    ) -> FLA_Error;
    pub fn FLA_Scalr_l(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Scalr_u(alpha: FLA_Obj, A: FLA_Obj, cntl: *mut fla_scalr_t) -> FLA_Error;
    pub fn FLA_Gemv_h_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_h_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_h_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_h_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_internal(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_h(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_n(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Gemv_t(
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        cntl: *mut fla_gemv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lc_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lc_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ln_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ln_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lt_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lt_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_uc_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_uc_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_un_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_un_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ut_blk_var1(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_ut_blk_var2(
        diagA: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_internal(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
        cntl: *mut fla_trsv_t,
    ) -> FLA_Error;
    pub fn FLA_Trsv_lc(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Trsv_ln(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Trsv_lt(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Trsv_uc(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Trsv_un(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Trsv_ut(diag: FLA_Diag, A: FLA_Obj, x: FLA_Obj, cntl: *mut fla_trsv_t) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Gemm_internal(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cc(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ch(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_cn(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ct(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hc(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_hn(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_ht(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nc(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nn(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_nt(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tc(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_th(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tn(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Gemm_tt(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hemm_internal(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ll(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_lu(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_rl(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Hemm_ru(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_hemm_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_lh_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_ln_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_uh_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_un_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Herk_internal(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_lh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_ln(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_uh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Herk_un(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_herk_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Her2k_internal(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_lh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_ln(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_uh(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Her2k_un(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_her2k_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Symm_internal(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ll(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_lu(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_rl(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Symm_ru(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_symm_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ln_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_lt_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_un_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var1(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var2(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var3(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var4(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var5(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_ut_unb_var6(alpha: FLA_Obj, A: FLA_Obj, beta: FLA_Obj, C: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Syrk_internal(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ln(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_lt(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_un(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syrk_ut(
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syrk_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_blk_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var1(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var2(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var3(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var4(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var5(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var6(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var7(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var8(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var9(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut_unb_var10(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_internal(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ln(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_lt(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_un(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Syr2k_ut(
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
        cntl: *mut fla_syr2k_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trmm_internal(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lln(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_llt(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_luh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lun(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_lut(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rln(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rlt(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_ruh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_run(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trmm_rut(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trmm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_blk_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_blk_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_blk_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_blk_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_unb_var1(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_unb_var2(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_unb_var3(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut_unb_var4(
        diagA: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trsm_internal(
        side: FLA_Side,
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lln(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_llt(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_luh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lun(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_lut(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rln(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rlt(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruc(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_ruh(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_run(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Trsm_rut(
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_trsm_t,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_blk_var1(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_l_blk_var2(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_l_blk_var3(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_l_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_l_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_l_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_l_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_blk_var1(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_u_blk_var2(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_u_blk_var3(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_u_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Chol_u_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Chol_u_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_u_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Chol_internal(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_l(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_u(A: FLA_Obj, cntl: *mut fla_chol_t) -> FLA_Error;
    pub fn FLA_Chol_solve(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Chol_solve(uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_blk_var1(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_blk_var2(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_blk_var3(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_blk_var4(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_blk_var5(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_unb_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_unb_var5(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_ops_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opd_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opc_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opz_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_ops_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opd_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opc_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opz_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_ops_var3(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opd_var3(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opc_var3(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opz_var3(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opt_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_ops_var4(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opd_var4(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opc_var4(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opz_var4(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opt_var5(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_nopiv_ops_var5(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opd_var5(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opc_var5(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_opz_var5(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_nopiv_internal(A: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_nopiv_solve(A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_nopiv_solve(A: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_blk_var3(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_blk_var4(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_blk_var5(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_unb_var3(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_unb_var3b(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_unb_var4(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_unb_var5(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_opt_var3(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_ops_var3(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opd_var3(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opc_var3(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opz_var3(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opt_var4(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_ops_var4(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opd_var4(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opc_var4(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opz_var4(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opt_var5(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_LU_piv_ops_var5(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opd_var5(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opc_var5(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_opz_var5(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_p: *mut c_int,
        inc_p: c_int,
    ) -> FLA_Error;
    pub fn FLA_LU_piv_internal(A: FLA_Obj, p: FLA_Obj, cntl: *mut fla_lu_t) -> FLA_Error;
    pub fn FLA_LU_piv_solve(A: FLA_Obj, p: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_piv_solve(A: FLA_Obj, p: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_SA_Apply_pivots(C: FLA_Obj, E: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_SA_LU_blk(
        U: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        nb_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_SA_LU_unb(U: FLA_Obj, D: FLA_Obj, p: FLA_Obj, L: FLA_Obj) -> FLA_Error;
    pub fn FLA_SA_FS_blk(
        L: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        nb_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLASH_LU_incpiv_var1(
        A: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_lu_t,
    ) -> FLA_Error;
    pub fn FLASH_LU_incpiv_var2(
        A: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        U: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_lu_t,
    ) -> FLA_Error;
    pub fn FLASH_Trsm_piv(A: FLA_Obj, B: FLA_Obj, p: FLA_Obj, cntl: *mut fla_trsm_t) -> FLA_Error;
    pub fn FLASH_SA_LU(
        B: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        E: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_lu_t,
    ) -> FLA_Error;
    pub fn FLASH_SA_FS(
        L: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        nb_alg: dim_t,
        cntl: *mut fla_gemm_t,
    ) -> FLA_Error;
    pub fn FLASH_FS_incpiv_aux1(
        A: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        b: FLA_Obj,
        nb_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLASH_FS_incpiv_aux2(
        L: FLA_Obj,
        D: FLA_Obj,
        p: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        nb_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLASH_LU_incpiv_create_hier_matrices(
        A_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        b_alg: dim_t,
        A: *mut FLA_Obj,
        p: *mut FLA_Obj,
        L: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_LU_incpiv_determine_alg_blocksize(A: FLA_Obj) -> dim_t;
    pub fn FLASH_LU_incpiv_noopt(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_incpiv_opt1(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_incpiv_solve(
        A: FLA_Obj,
        p: FLA_Obj,
        L: FLA_Obj,
        B: FLA_Obj,
        X: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_unb_var1(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_blk_var1(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_opt_var1(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_ops_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut f32,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opd_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut f64,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opc_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut scomplex,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opz_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut dcomplex,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_blk_var2(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_ops_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opd_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opc_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_opz_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_blk_var3(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_internal(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_qrut_t) -> FLA_Error;
    pub fn FLA_QR_UT_copy_internal(
        A: FLA_Obj,
        T: FLA_Obj,
        U: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_create_T(A: FLA_Obj, T: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_recover_tau(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_solve(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLASH_QR_UT(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLASH_QR_UT_create_hier_matrices(
        A_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        A: *mut FLA_Obj,
        TW: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_QR_UT_solve(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q(A: FLA_Obj, T: FLA_Obj, Q: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_blk_var1(A: FLA_Obj, T: FLA_Obj, W: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_ops_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_opd_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_opc_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_form_Q_opz_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_piv_unb_var1(A: FLA_Obj, T: FLA_Obj, w: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_piv_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        w: FLA_Obj,
        p: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_piv_unb_var2(A: FLA_Obj, T: FLA_Obj, w: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_piv_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        w: FLA_Obj,
        p: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_piv_row(
        tau: FLA_Obj,
        a1t: FLA_Obj,
        u1t: FLA_Obj,
        W: FLA_Obj,
        u2: FLA_Obj,
        A2: FLA_Obj,
        U2: FLA_Obj,
        w1t: FLA_Obj,
        vt: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_piv_internal(
        A: FLA_Obj,
        T: FLA_Obj,
        w: FLA_Obj,
        p: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_piv_colnorm(alpha: FLA_Obj, A: FLA_Obj, b: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_piv_check(A: FLA_Obj, T: FLA_Obj, w: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_piv_internal_check(
        A: FLA_Obj,
        T: FLA_Obj,
        w: FLA_Obj,
        p: FLA_Obj,
        cntl: *mut fla_qrut_t,
    ) -> FLA_Error;
    pub fn FLA_QR_UT_piv_colnorm_check(alpha: FLA_Obj, A: FLA_Obj, b: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR2_UT_blk_var1(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_qr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_blk_var2(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_qr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_unb_var1(U: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR2_UT_opt_var1(U: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR2_UT_ops_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut f32,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_opd_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut f64,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_opc_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut scomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_QR2_UT_opz_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut dcomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLASH_QR2_UT(U: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR2_UT_internal(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_qr2ut_t,
    ) -> FLA_Error;
    pub fn FLASH_QR_UT_inc(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLASH_QR_UT_inc_noopt(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLASH_QR_UT_inc_opt1(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLA_QR_UT_inc_blk_var1(A: FLA_Obj, TW: FLA_Obj, cntl: *mut fla_qrutinc_t) -> FLA_Error;
    pub fn FLA_QR_UT_inc_blk_var2(
        A: FLA_Obj,
        TW: FLA_Obj,
        U: FLA_Obj,
        cntl: *mut fla_qrutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_QR_UT_inc_create_hier_matrices(
        A_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        b_alg: dim_t,
        A: *mut FLA_Obj,
        TW: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_QR_UT_inc_determine_alg_blocksize(A: FLA_Obj) -> dim_t;
    pub fn FLASH_QR_UT_inc_solve(A: FLA_Obj, TW: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_unb_var1(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_blk_var1(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_LQ_UT_opt_var1(A: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_ops_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut f32,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opd_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut f64,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opc_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut scomplex,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opz_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        t: *mut dcomplex,
        inc_t: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_blk_var2(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_LQ_UT_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_ops_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opd_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opc_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_opz_var2(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_LQ_UT_blk_var3(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_LQ_UT_internal(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_lqut_t) -> FLA_Error;
    pub fn FLA_LQ_UT_create_T(A: FLA_Obj, T: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_recover_tau(T: FLA_Obj, tau: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_solve(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LQ_UT(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LQ_UT_create_hier_matrices(
        A_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        A: *mut FLA_Obj,
        TW: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_LQ_UT_solve(A: FLA_Obj, T: FLA_Obj, B: FLA_Obj, X: FLA_Obj) -> FLA_Error;
    pub fn FLA_LQ_UT_form_Q(A: FLA_Obj, T: FLA_Obj, Q: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR2_UT_blk_var1(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_caqr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_blk_var2(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_caqr2ut_t,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_unb_var1(U: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR2_UT_opt_var1(U: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR2_UT_ops_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut f32,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_opd_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut f64,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_opc_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut scomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_opz_var1(
        m_UT: c_int,
        m_D: c_int,
        U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        D: *mut dcomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_CAQR2_UT_internal(
        U: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_caqr2ut_t,
    ) -> FLA_Error;
    pub fn FLASH_CAQR_UT_inc(
        p: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_CAQR_UT_inc_noopt(
        p: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_CAQR_UT_inc_create_hier_matrices(
        p: dim_t,
        A_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        b_alg: dim_t,
        A: *mut FLA_Obj,
        ATW: *mut FLA_Obj,
        R: *mut FLA_Obj,
        RTW: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_CAQR_UT_inc_determine_alg_blocksize(A: FLA_Obj) -> dim_t;
    pub fn FLASH_CAQR_UT_inc_adjust_views(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR_UT_inc_init_structure(p: dim_t, nb_part: dim_t, R: FLA_Obj);
    pub fn FLA_CAQR_UT_inc_compute_blocks_per_part(p: dim_t, A: FLA_Obj) -> dim_t;
    pub fn FLA_CAQR_UT_inc_factorize_panels(nb_part: dim_t, A: FLA_Obj, ATW: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR_UT_inc_copy_triangles(nb_part: dim_t, A: FLA_Obj, R: FLA_Obj) -> FLA_Error;
    pub fn FLA_CAQR_UT_inc_blk_var1(
        R: FLA_Obj,
        TW: FLA_Obj,
        cntl: *mut fla_caqrutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_CAQR_UT_inc_solve(
        p: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
        B: FLA_Obj,
        X: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Hevd_ln_unb_var1(A: FLA_Obj, l: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hevd_lv_unb_var1(
        n_iter_max: dim_t,
        A: FLA_Obj,
        l: FLA_Obj,
        k_accum: dim_t,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Hevd_lv_unb_var2(
        n_iter_max: dim_t,
        A: FLA_Obj,
        l: FLA_Obj,
        k_accum: dim_t,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Hevd_compute_scaling(uplo: FLA_Uplo, A: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hevd(jobz: FLA_Evd_type, uplo: FLA_Uplo, A: FLA_Obj, l: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_n_ops_var1(
        m_A: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_n_opd_var1(
        m_A: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_n_opt_var1(
        G: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        n_iter: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_n_ops_var1(
        m_A: c_int,
        n_G: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_n_opd_var1(
        m_A: c_int,
        n_G: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_francis_n_opt_var1(shift: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tevd_francis_n_ops_var1(
        m_A: c_int,
        buff_shift: *mut f32,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_francis_n_opd_var1(
        m_A: c_int,
        buff_shift: *mut f64,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_find_submatrix_ops(
        m_A: c_int,
        ij_begin: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        ijTL: *mut c_int,
        ijBR: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_find_submatrix_opd(
        m_A: c_int,
        ij_begin: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        ijTL: *mut c_int,
        ijBR: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Norm1_tridiag(d: FLA_Obj, e: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLA_Norm1_tridiag_ops(
        m_A: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        norm: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Norm1_tridiag_opd(
        m_A: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        norm: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Tevd_n_opt_var1(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        U: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tevd_n_ops_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_n_opd_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_n_opc_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_n_opz_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_v_ops_var1(
        m_A: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_v_opd_var1(
        m_A: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_v_ops_var3(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_l: *mut f32,
        inc_l: c_int,
        buff_ls: *mut c_int,
        inc_ls: c_int,
        buff_pu: *mut f32,
        inc_pu: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_iteracc_v_opd_var3(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        ijTL: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_l: *mut f64,
        inc_l: c_int,
        buff_ls: *mut c_int,
        inc_ls: c_int,
        buff_pu: *mut f64,
        inc_pu: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_v_opt_var1(
        G: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        n_iter: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_v_ops_var1(
        m_A: c_int,
        n_G: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_v_opd_var1(
        m_A: c_int,
        n_G: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_v_ops_var3(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_l: *mut f32,
        inc_l: c_int,
        buff_ls: *mut c_int,
        inc_ls: c_int,
        buff_pu: *mut f32,
        inc_pu: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_eigval_v_opd_var3(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_l: *mut f64,
        inc_l: c_int,
        buff_ls: *mut c_int,
        inc_ls: c_int,
        buff_pu: *mut f64,
        inc_pu: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_francis_v_opt_var1(
        shift: FLA_Obj,
        g: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tevd_francis_v_ops_var1(
        m_A: c_int,
        buff_shift: *mut f32,
        buff_g: *mut scomplex,
        inc_g: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_francis_v_opd_var1(
        m_A: c_int,
        buff_shift: *mut f64,
        buff_g: *mut dcomplex,
        inc_g: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_compute_scaling_ops(
        m_A: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        sigma: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Tevd_compute_scaling_opd(
        m_A: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        sigma: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Tevd_find_perfshift_ops(
        m_d: c_int,
        m_l: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_l: *mut f32,
        inc_l: c_int,
        buff_lstat: *mut c_int,
        inc_lstat: c_int,
        buff_pu: *mut f32,
        inc_pu: c_int,
        ij_shift: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_find_perfshift_opd(
        m_d: c_int,
        m_l: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_l: *mut f64,
        inc_l: c_int,
        buff_lstat: *mut c_int,
        inc_lstat: c_int,
        buff_pu: *mut f64,
        inc_pu: c_int,
        ij_shift: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opt_var1(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        U: FLA_Obj,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_ops_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opd_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opc_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opz_var1(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opt_var2(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        R: FLA_Obj,
        W: FLA_Obj,
        U: FLA_Obj,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_ops_var2(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_G_extra: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_R: *mut f32,
        rs_R: c_int,
        cs_R: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opd_var2(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_G_extra: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_R: *mut f64,
        rs_R: c_int,
        cs_R: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opc_var2(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_G_extra: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_R: *mut f32,
        rs_R: c_int,
        cs_R: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd_v_opz_var2(
        m_A: c_int,
        m_U: c_int,
        n_G: c_int,
        n_G_extra: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_R: *mut f64,
        rs_R: c_int,
        cs_R: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tevd(
        jobz: FLA_Evd_type,
        U: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        l: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svd_ext_u_unb_var1(
        jobu: FLA_Svd_type,
        jobv: FLA_Svd_type,
        n_iter_max: dim_t,
        A: FLA_Obj,
        s: FLA_Obj,
        V: FLA_Obj,
        U: FLA_Obj,
        k_accum: dim_t,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Svd_uv_unb_var1(
        n_iter_max: dim_t,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
        k_accum: dim_t,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Svd_uv_unb_var2(
        n_iter_max: dim_t,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
        k_accum: dim_t,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Svd_compute_scaling(A: FLA_Obj, sigma: FLA_Obj) -> FLA_Error;
    pub fn FLA_Svd(
        jobu: FLA_Svd_type,
        jobv: FLA_Svd_type,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Svd_ext(
        jobu: FLA_Svd_type,
        transu: FLA_Trans,
        jobv: FLA_Svd_type,
        transv: FLA_Trans,
        A: FLA_Obj,
        s: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_iteracc_v_ops_var1(
        m_A: c_int,
        n_GH: c_int,
        ijTL: c_int,
        tol: f32,
        thresh: f32,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_iteracc_v_opd_var1(
        m_A: c_int,
        n_GH: c_int,
        ijTL: c_int,
        tol: f64,
        thresh: f64,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        n_iter_perf: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_sinval_v_opt_var1(
        tol: FLA_Obj,
        thresh: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        n_iter: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_sinval_v_ops_var1(
        m_A: c_int,
        n_GH: c_int,
        n_iter_allowed: c_int,
        tol: f32,
        thresh: f32,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_sinval_v_opd_var1(
        m_A: c_int,
        n_GH: c_int,
        n_iter_allowed: c_int,
        tol: f64,
        thresh: f64,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        n_iter: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_francis_v_opt_var1(
        shift: FLA_Obj,
        g: FLA_Obj,
        h: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_francis_v_ops_var1(
        m_A: c_int,
        shift: f32,
        buff_g: *mut scomplex,
        inc_g: c_int,
        buff_h: *mut scomplex,
        inc_h: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_francis_v_opd_var1(
        m_A: c_int,
        shift: f64,
        buff_g: *mut dcomplex,
        inc_g: c_int,
        buff_h: *mut dcomplex,
        inc_h: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_shift(
        tol: FLA_Obj,
        sminl: FLA_Obj,
        smax: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        shift: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_shift_ops(
        m_A: c_int,
        tol: f32,
        sminl: f32,
        smax: f32,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        shift: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_shift_opd(
        m_A: c_int,
        tol: f64,
        sminl: f64,
        smax: f64,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        shift: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_tol_thresh(
        tolmul: FLA_Obj,
        maxit: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        tol: FLA_Obj,
        thresh: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_tol_thresh_ops(
        m_A: c_int,
        tolmul: f32,
        maxit: f32,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        tol: *mut f32,
        thresh: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_compute_tol_thresh_opd(
        m_A: c_int,
        tolmul: f64,
        maxit: f64,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        tol: *mut f64,
        thresh: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_converged(
        tol: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
        sminl: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_converged_ops(
        m_A: c_int,
        tol: f32,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        sminl: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_converged_opd(
        m_A: c_int,
        tol: f64,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        sminl: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_max_min(d: FLA_Obj, e: FLA_Obj, smax: FLA_Obj, smin: FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Bsvd_find_max_min_ops(
        m_A: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        smax: *mut f32,
        smin: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_max_min_opd(
        m_A: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        smax: *mut f64,
        smin: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_submatrix_ops(
        mn_A: c_int,
        ij_begin: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        ijTL: *mut c_int,
        ijBR: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_find_submatrix_opd(
        mn_A: c_int,
        ij_begin: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        ijTL: *mut c_int,
        ijBR: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opt_var1(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_ops_var1(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opd_var1(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opc_var1(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opz_var1(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opt_var2(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        RG: FLA_Obj,
        RH: FLA_Obj,
        W: FLA_Obj,
        U: FLA_Obj,
        V: FLA_Obj,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_ops_var2(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_RG: *mut f32,
        rs_RG: c_int,
        cs_RG: c_int,
        buff_RH: *mut f32,
        rs_RH: c_int,
        cs_RH: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opd_var2(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_RG: *mut f64,
        rs_RG: c_int,
        cs_RG: c_int,
        buff_RH: *mut f64,
        rs_RH: c_int,
        cs_RH: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opc_var2(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_RG: *mut f32,
        rs_RG: c_int,
        cs_RG: c_int,
        buff_RH: *mut f32,
        rs_RH: c_int,
        cs_RH: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_v_opz_var2(
        min_m_n: c_int,
        m_U: c_int,
        m_V: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_RG: *mut f64,
        rs_RG: c_int,
        cs_RG: c_int,
        buff_RH: *mut f64,
        rs_RH: c_int,
        cs_RH: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_opt_var1(
        n_iter_max: dim_t,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        jobu: FLA_Svd_type,
        U: FLA_Obj,
        jobv: FLA_Svd_type,
        V: FLA_Obj,
        apply_Uh2C: FLA_Bool,
        C: FLA_Obj,
        b_alg: dim_t,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_ops_var1(
        m_d: c_int,
        m_U: c_int,
        m_V: c_int,
        m_C: c_int,
        n_C: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_opd_var1(
        m_d: c_int,
        m_U: c_int,
        m_V: c_int,
        m_C: c_int,
        n_C: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_opc_var1(
        m_d: c_int,
        m_U: c_int,
        m_V: c_int,
        m_C: c_int,
        n_C: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f32,
        inc_d: c_int,
        buff_e: *mut f32,
        inc_e: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut scomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext_opz_var1(
        m_d: c_int,
        m_U: c_int,
        m_V: c_int,
        m_C: c_int,
        n_C: c_int,
        n_GH: c_int,
        n_iter_max: c_int,
        buff_d: *mut f64,
        inc_d: c_int,
        buff_e: *mut f64,
        inc_e: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_H: *mut dcomplex,
        rs_H: c_int,
        cs_H: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_create_workspace(d: FLA_Obj, G: *mut FLA_Obj, H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Bsvd(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        jobu: FLA_Svd_type,
        U: FLA_Obj,
        jobv: FLA_Svd_type,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bsvd_ext(
        uplo: FLA_Uplo,
        d: FLA_Obj,
        e: FLA_Obj,
        G: FLA_Obj,
        H: FLA_Obj,
        jobu: FLA_Svd_type,
        U: FLA_Obj,
        jobv: FLA_Svd_type,
        V: FLA_Obj,
        apply_Uh2C: FLA_Bool,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_blk_var1(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_ln_blk_var2(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_ln_blk_var3(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_ln_blk_var4(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_ln_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_unb_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opt_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_ln_ops_var4(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opd_var4(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_ln_opc_var4(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln_opz_var4(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_blk_var1(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu_blk_var2(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu_blk_var3(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu_blk_var4(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_unb_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opt_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_lu_ops_var4(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opd_var4(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_lu_opc_var4(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_lu_opz_var4(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_blk_var1(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un_blk_var2(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un_blk_var3(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un_blk_var4(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_unb_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opt_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_un_ops_var4(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opd_var4(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_un_opc_var4(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_un_opz_var4(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_blk_var1(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu_blk_var2(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu_blk_var3(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu_blk_var4(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_unb_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opt_var4(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Trinv_uu_ops_var4(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opd_var4(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Trinv_uu_opc_var4(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_uu_opz_var4(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Trinv_internal(
        uplo: FLA_Uplo,
        diag: FLA_Diag,
        A: FLA_Obj,
        cntl: *mut fla_trinv_t,
    ) -> FLA_Error;
    pub fn FLA_Trinv_ln(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_lu(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_un(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_Trinv_uu(A: FLA_Obj, cntl: *mut fla_trinv_t) -> FLA_Error;
    pub fn FLA_SPDinv_internal(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_spdinv_t) -> FLA_Error;
    pub fn FLA_Hess_UT_blk_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_unb_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_unb_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blk_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blf_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blk_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blf_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_unb_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_unb_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blk_var4(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blf_var4(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_unb_var4(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_unb_var4(A: FLA_Obj, Y: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_blk_var5(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_unb_var5(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_unb_var5(A: FLA_Obj, U: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ops_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opd_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opc_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opz_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ops_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opd_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opc_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opz_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_opt_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opt_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ops_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opd_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opc_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opz_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_opt_var4(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opt_var4(A: FLA_Obj, Y: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ops_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opd_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opc_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opz_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_opt_var5(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opt_var5(A: FLA_Obj, U: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ops_var5(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opd_var5(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opc_var5(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_opz_var5(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_ofu_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofu_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofs_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofd_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofc_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofz_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_ofu_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofu_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofs_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofd_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofc_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofz_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_ofu_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofu_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofs_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofd_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofc_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofz_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_ofu_var4(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofu_var4(A: FLA_Obj, Y: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofs_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofd_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofc_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_step_ofz_var4(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Ax_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f32,
        inc_x: c_int,
        buff_v: *mut f32,
        inc_v: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Ax_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f64,
        inc_x: c_int,
        buff_v: *mut f64,
        inc_v: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Ax_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut scomplex,
        inc_x: c_int,
        buff_v: *mut scomplex,
        inc_v: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Ax_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut dcomplex,
        inc_x: c_int,
        buff_v: *mut dcomplex,
        inc_v: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Ax_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut f32,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_z: *mut f32,
        inc_z: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f32,
        inc_x: c_int,
        buff_v: *mut f32,
        inc_v: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Ax_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut f64,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_z: *mut f64,
        inc_z: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f64,
        inc_x: c_int,
        buff_v: *mut f64,
        inc_v: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Ax_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut scomplex,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_z: *mut scomplex,
        inc_z: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut scomplex,
        inc_x: c_int,
        buff_v: *mut scomplex,
        inc_v: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Ax_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut dcomplex,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_z: *mut dcomplex,
        inc_z: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut dcomplex,
        inc_x: c_int,
        buff_v: *mut dcomplex,
        inc_v: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Uhu_Yhu_Zhu_ops_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut f32,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut f32,
        inc_t: c_int,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_z: *mut f32,
        inc_z: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Uhu_Yhu_Zhu_opd_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut f64,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut f64,
        inc_t: c_int,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_z: *mut f64,
        inc_z: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Uhu_Yhu_Zhu_opc_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut scomplex,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut scomplex,
        inc_t: c_int,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_z: *mut scomplex,
        inc_z: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Uhu_Yhu_Zhu_opz_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut dcomplex,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut dcomplex,
        inc_t: c_int,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_z: *mut dcomplex,
        inc_z: c_int,
    ) -> FLA_Error;
    pub fn FLA_Hess_UT_internal(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_hessut_t) -> FLA_Error;
    pub fn FLA_Hess_UT_create_T(A: FLA_Obj, T: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Hess_UT_recover_tau(T: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_blk_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_unb_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_unb_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_blk_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_blf_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_unb_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_blk_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_blf_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_unb_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_unb_var3(A: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ops_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opd_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opc_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opz_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opt_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ops_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opd_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opc_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opz_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_opt_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opt_var3(A: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ops_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opd_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opc_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_opz_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_ofu_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofu_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofs_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofd_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofc_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofz_var1(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_ofu_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofu_var2(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofs_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofd_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofc_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofz_var2(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_ofu_var3(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofu_var3(A: FLA_Obj, Z: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofs_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofd_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofc_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_step_ofz_var3(
        m_A: c_int,
        m_T: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Her2_Ax_l_opt_var1(
        alpha: FLA_Obj,
        u: FLA_Obj,
        z: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        w: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_Her2_Ax_l_ops_var1(
        m_A: c_int,
        buff_alpha: *mut f32,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_z: *mut f32,
        inc_z: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f32,
        inc_x: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Her2_Ax_l_opd_var1(
        m_A: c_int,
        buff_alpha: *mut f64,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_z: *mut f64,
        inc_z: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut f64,
        inc_x: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Her2_Ax_l_opc_var1(
        m_A: c_int,
        buff_alpha: *mut scomplex,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_z: *mut scomplex,
        inc_z: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut scomplex,
        inc_x: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Her2_Ax_l_opz_var1(
        m_A: c_int,
        buff_alpha: *mut dcomplex,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_z: *mut dcomplex,
        inc_z: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_x: *mut dcomplex,
        inc_x: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UZhu_ZUhu_opt_var1(
        delta: FLA_Obj,
        U: FLA_Obj,
        Z: FLA_Obj,
        t: FLA_Obj,
        u: FLA_Obj,
        w: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_UZhu_ZUhu_ops_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut f32,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut f32,
        inc_t: c_int,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UZhu_ZUhu_opd_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut f64,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut f64,
        inc_t: c_int,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UZhu_ZUhu_opc_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut scomplex,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut scomplex,
        inc_t: c_int,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UZhu_ZUhu_opz_var1(
        m_U: c_int,
        n_U: c_int,
        buff_delta: *mut dcomplex,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_t: *mut dcomplex,
        inc_t: c_int,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT(uplo: FLA_Uplo, A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_internal(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_tridiagut_t,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_tridiagut_t) -> FLA_Error;
    pub fn FLA_Tridiag_UT_u(A: FLA_Obj, T: FLA_Obj, cntl: *mut fla_tridiagut_t) -> FLA_Error;
    pub fn FLA_Tridiag_UT_create_T(A: FLA_Obj, T: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_recover_tau(T: FLA_Obj, t: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_scale_diagonals(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_extract_diagonals(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_extract_real_diagonals(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    #[doc = " FLA_Error FLA_Tridiag_UT_l_extract_diagonals( FLA_Obj A, FLA_Obj d, FLA_Obj e );\n FLA_Error FLA_Tridiag_UT_u_extract_diagonals( FLA_Obj A, FLA_Obj d, FLA_Obj e );"]
    pub fn FLA_Tridiag_UT_realify(uplo: FLA_Uplo, A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_realify_unb(A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_l_realify_opt(A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_u_realify_unb(A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_u_realify_opt(A: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_realify_subdiagonal(b: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_realify_subdiagonal_opt(b: FLA_Obj, d: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_l_ops(
        m_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_u_ops(
        m_A: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_l_opd(
        m_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_u_opd(
        m_A: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_l_opc(
        m_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_u_opc(
        m_A: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_l_opz(
        m_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_shift_U_u_opz(
        m_A: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q(uplo: FLA_Uplo, A: FLA_Obj, T: FLA_Obj, Q: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_blk_var1(A: FLA_Obj, T: FLA_Obj, W: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_u_blk_var1(A: FLA_Obj, T: FLA_Obj, W: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_opt_var1(A: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_ops_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_opd_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_opc_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Tridiag_UT_form_Q_l_opz_var1(
        m_A: c_int,
        n_AT: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_unb_var1(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blk_var1(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_unb_var1(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_unb_var2(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blk_var2(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blf_var2(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_unb_var2(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_unb_var3(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blk_var3(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blf_var3(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_unb_var3(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_unb_var4(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blk_var4(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blf_var4(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_unb_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_unb_var5(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_blk_var5(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_unb_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_opt_var1(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opt_var1(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ops_var1(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opd_var1(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opc_var1(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opz_var1(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_opt_var2(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opt_var2(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ops_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opd_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opc_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opz_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_opt_var3(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opt_var3(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ops_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opd_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opc_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opz_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_opt_var4(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opt_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        T: FLA_Obj,
        S: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ops_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opd_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opc_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opz_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_opt_var5(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opt_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        T: FLA_Obj,
        S: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ops_var5(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opd_var5(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opc_var5(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_opz_var5(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_ofu_var2(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofu_var2(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofs_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofd_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofc_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofz_var2(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_ofu_var3(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofu_var3(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofs_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofd_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofc_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofz_var3(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_ofu_var4(A: FLA_Obj, T: FLA_Obj, S: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofu_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        T: FLA_Obj,
        S: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofs_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f32,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofd_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut f64,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofc_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut scomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_step_ofz_var4(
        m_A: c_int,
        n_A: c_int,
        m_TS: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
        buff_S: *mut dcomplex,
        rs_S: c_int,
        cs_S: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_opt_var1(
        alpha: FLA_Obj,
        u: FLA_Obj,
        y: FLA_Obj,
        z: FLA_Obj,
        v: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut f32,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_z: *mut f32,
        inc_z: c_int,
        buff_v: *mut f32,
        inc_v: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut f64,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_z: *mut f64,
        inc_z: c_int,
        buff_v: *mut f64,
        inc_v: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut scomplex,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_z: *mut scomplex,
        inc_z: c_int,
        buff_v: *mut scomplex,
        inc_v: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_alpha: *mut dcomplex,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_z: *mut dcomplex,
        inc_z: c_int,
        buff_v: *mut dcomplex,
        inc_v: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Axpy_Ax_opt_var1(
        A: FLA_Obj,
        u: FLA_Obj,
        tau: FLA_Obj,
        a: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
        w: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Axpy_Ax_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut f32,
        buff_beta: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_a: *mut f32,
        inc_a: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Axpy_Ax_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut f64,
        buff_beta: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_a: *mut f64,
        inc_a: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Axpy_Ax_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut scomplex,
        buff_beta: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_a: *mut scomplex,
        inc_a: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Ahx_Axpy_Ax_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut dcomplex,
        buff_beta: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_a: *mut dcomplex,
        inc_a: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Axpy_Ax_opt_var1(
        alpha: FLA_Obj,
        tau: FLA_Obj,
        u: FLA_Obj,
        y: FLA_Obj,
        z: FLA_Obj,
        v: FLA_Obj,
        A: FLA_Obj,
        up: FLA_Obj,
        a: FLA_Obj,
        w: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Axpy_Ax_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut f32,
        buff_alpha: *mut f32,
        buff_u: *mut f32,
        inc_u: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_z: *mut f32,
        inc_z: c_int,
        buff_v: *mut f32,
        inc_v: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_up: *mut f32,
        inc_up: c_int,
        buff_a: *mut f32,
        inc_a: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Axpy_Ax_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut f64,
        buff_alpha: *mut f64,
        buff_u: *mut f64,
        inc_u: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_z: *mut f64,
        inc_z: c_int,
        buff_v: *mut f64,
        inc_v: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_up: *mut f64,
        inc_up: c_int,
        buff_a: *mut f64,
        inc_a: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Axpy_Ax_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut scomplex,
        buff_alpha: *mut scomplex,
        buff_u: *mut scomplex,
        inc_u: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_z: *mut scomplex,
        inc_z: c_int,
        buff_v: *mut scomplex,
        inc_v: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_up: *mut scomplex,
        inc_up: c_int,
        buff_a: *mut scomplex,
        inc_a: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_Gerc2_Ahx_Axpy_Ax_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_tau: *mut dcomplex,
        buff_alpha: *mut dcomplex,
        buff_u: *mut dcomplex,
        inc_u: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_z: *mut dcomplex,
        inc_z: c_int,
        buff_v: *mut dcomplex,
        inc_v: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_up: *mut dcomplex,
        inc_up: c_int,
        buff_a: *mut dcomplex,
        inc_a: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UYx_ZVx_opt_var1(
        delta: FLA_Obj,
        a: FLA_Obj,
        U: FLA_Obj,
        Y: FLA_Obj,
        Z: FLA_Obj,
        V: FLA_Obj,
        A: FLA_Obj,
        temp: FLA_Obj,
        t: FLA_Obj,
        w: FLA_Obj,
        al: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Fused_UYx_ZVx_ops_var1(
        m_U: c_int,
        n_U: c_int,
        m_V: c_int,
        n_V: c_int,
        buff_delta: *mut f32,
        buff_U: *mut f32,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f32,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_V: *mut f32,
        rs_V: c_int,
        cs_V: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_temp: *mut f32,
        inc_temp: c_int,
        buff_t: *mut f32,
        inc_t: c_int,
        buff_a: *mut f32,
        inc_a: c_int,
        buff_w: *mut f32,
        inc_w: c_int,
        buff_al: *mut f32,
        inc_al: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UYx_ZVx_opd_var1(
        m_U: c_int,
        n_U: c_int,
        m_V: c_int,
        n_V: c_int,
        buff_delta: *mut f64,
        buff_U: *mut f64,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut f64,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_V: *mut f64,
        rs_V: c_int,
        cs_V: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_temp: *mut f64,
        inc_temp: c_int,
        buff_t: *mut f64,
        inc_t: c_int,
        buff_a: *mut f64,
        inc_a: c_int,
        buff_w: *mut f64,
        inc_w: c_int,
        buff_al: *mut f64,
        inc_al: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UYx_ZVx_opc_var1(
        m_U: c_int,
        n_U: c_int,
        m_V: c_int,
        n_V: c_int,
        buff_delta: *mut scomplex,
        buff_U: *mut scomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut scomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_V: *mut scomplex,
        rs_V: c_int,
        cs_V: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_temp: *mut scomplex,
        inc_temp: c_int,
        buff_t: *mut scomplex,
        inc_t: c_int,
        buff_a: *mut scomplex,
        inc_a: c_int,
        buff_w: *mut scomplex,
        inc_w: c_int,
        buff_al: *mut scomplex,
        inc_al: c_int,
    ) -> FLA_Error;
    pub fn FLA_Fused_UYx_ZVx_opz_var1(
        m_U: c_int,
        n_U: c_int,
        m_V: c_int,
        n_V: c_int,
        buff_delta: *mut dcomplex,
        buff_U: *mut dcomplex,
        rs_U: c_int,
        cs_U: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_Z: *mut dcomplex,
        rs_Z: c_int,
        cs_Z: c_int,
        buff_V: *mut dcomplex,
        rs_V: c_int,
        cs_V: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_temp: *mut dcomplex,
        inc_temp: c_int,
        buff_t: *mut dcomplex,
        inc_t: c_int,
        buff_a: *mut dcomplex,
        inc_a: c_int,
        buff_w: *mut dcomplex,
        inc_w: c_int,
        buff_al: *mut dcomplex,
        inc_al: c_int,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT(A: FLA_Obj, TU: FLA_Obj, TV: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_internal(
        A: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
        cntl: *mut fla_bidiagut_t,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l(
        A: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
        cntl: *mut fla_bidiagut_t,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u(
        A: FLA_Obj,
        TU: FLA_Obj,
        TV: FLA_Obj,
        cntl: *mut fla_bidiagut_t,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_create_T(A: FLA_Obj, TU: *mut FLA_Obj, TV: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_recover_tau(
        TU: FLA_Obj,
        TV: FLA_Obj,
        tu: FLA_Obj,
        tv: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_extract_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_extract_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l_extract_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_extract_real_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_extract_real_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l_extract_real_diagonals(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_scale_diagonals(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_scale_diagonals(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l_scale_diagonals(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_realify(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l_realify_unb(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_l_realify_opt(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_realify_unb(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_u_realify_opt(A: FLA_Obj, d: FLA_Obj, e: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_realify_diagonals(
        uplo: FLA_Uplo,
        a: FLA_Obj,
        b: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_realify_diagonals_opt(
        a: FLA_Obj,
        b: FLA_Obj,
        d: FLA_Obj,
        e: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_U(A: FLA_Obj, T: FLA_Obj, U: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_V(A: FLA_Obj, S: FLA_Obj, V: FLA_Obj) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_U_ext(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        T: FLA_Obj,
        transu: FLA_Trans,
        U: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Bidiag_UT_form_V_ext(
        uplo: FLA_Uplo,
        A: FLA_Obj,
        S: FLA_Obj,
        transv: FLA_Trans,
        V: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_unb_var1(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_unb_var2(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_unb_var3(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_unb_var4(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opt_var1(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_ops_var1(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opd_var1(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opc_var1(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opz_var1(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opt_var2(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_ops_var2(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opd_var2(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opc_var2(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opz_var2(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opt_var3(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_ops_var3(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opd_var3(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opc_var3(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opz_var3(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opt_var4(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_n_ops_var4(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opd_var4(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opc_var4(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n_opz_var4(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_unb_var1(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_unb_var2(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_unb_var3(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_unb_var4(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opt_var1(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_ops_var1(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opd_var1(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opc_var1(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opz_var1(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opt_var2(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_ops_var2(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opd_var2(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opc_var2(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opz_var2(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opt_var3(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_ops_var3(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opd_var3(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opc_var3(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opz_var3(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opt_var4(isgn: FLA_Obj, A: FLA_Obj, C: FLA_Obj) -> FLA_Error;
    pub fn FLA_Lyap_h_ops_var4(
        m_AC: c_int,
        buff_sgn: *mut f32,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f32,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opd_var4(
        m_AC: c_int,
        buff_sgn: *mut f64,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut f64,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opc_var4(
        m_AC: c_int,
        buff_sgn: *mut scomplex,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut scomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h_opz_var4(
        m_AC: c_int,
        buff_sgn: *mut dcomplex,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_W: *mut dcomplex,
        rs_W: c_int,
        cs_W: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
    ) -> FLA_Error;
    pub fn FLASH_Lyap(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Lyap(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Lyap_internal(
        trans: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_n(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Lyap_h(
        isgn: FLA_Obj,
        A: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_lyap_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_blk_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opt_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_ops_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f32,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opd_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f64,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opc_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut scomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn_opz_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut dcomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_blk_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opt_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_ops_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f32,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opd_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f64,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opc_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut scomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh_opz_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut dcomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_blk_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opt_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_ops_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f32,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opd_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f64,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opc_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut scomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn_opz_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut dcomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_blk_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var1(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var2(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var3(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var4(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var5(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var6(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var7(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var8(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var9(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var10(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var11(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var12(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var13(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var14(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var15(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var16(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var17(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opt_var18(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_ops_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f32,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opd_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut f64,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opc_var1(
        sgn: f32,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut scomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh_opz_var1(
        sgn: f64,
        m_C: c_int,
        n_C: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
        buff_C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        buff_scale: *mut dcomplex,
        info: *mut c_int,
    ) -> FLA_Error;
    pub fn FLA_Sylv_internal(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nn(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_nh(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hn(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Sylv_hh(
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
        cntl: *mut fla_sylv_t,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_blk_var1(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_l_blk_var2(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_l_blk_var3(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_l_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_l_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_l_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_l_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_blk_var1(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_u_blk_var2(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_u_blk_var3(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_u_unb_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_unb_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_unb_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_opt_var1(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_ops_var1(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opd_var1(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opc_var1(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_opz_var1(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_opt_var2(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_ops_var2(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opd_var2(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opc_var2(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_opz_var2(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_opt_var3(A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Ttmm_u_ops_var3(mn_A: c_int, A: *mut f32, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opd_var3(mn_A: c_int, A: *mut f64, rs_A: c_int, cs_A: c_int) -> FLA_Error;
    pub fn FLA_Ttmm_u_opc_var3(
        mn_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_u_opz_var3(
        mn_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Ttmm_internal(uplo: FLA_Uplo, A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_l(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_Ttmm_u(A: FLA_Obj, cntl: *mut fla_ttmm_t) -> FLA_Error;
    pub fn FLA_UDdate_UT_blk_var1(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_uddateut_t,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_blk_var2(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_uddateut_t,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_unb_var1(R: FLA_Obj, C: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_opt_var1(R: FLA_Obj, C: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_ops_var1(
        mn_RT: c_int,
        m_C: c_int,
        m_D: c_int,
        R: *mut f32,
        rs_R: c_int,
        cs_R: c_int,
        C: *mut f32,
        rs_C: c_int,
        cs_C: c_int,
        D: *mut f32,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_opd_var1(
        mn_RT: c_int,
        m_C: c_int,
        m_D: c_int,
        R: *mut f64,
        rs_R: c_int,
        cs_R: c_int,
        C: *mut f64,
        rs_C: c_int,
        cs_C: c_int,
        D: *mut f64,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_opc_var1(
        mn_RT: c_int,
        m_C: c_int,
        m_D: c_int,
        R: *mut scomplex,
        rs_R: c_int,
        cs_R: c_int,
        C: *mut scomplex,
        rs_C: c_int,
        cs_C: c_int,
        D: *mut scomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_opz_var1(
        mn_RT: c_int,
        m_C: c_int,
        m_D: c_int,
        R: *mut dcomplex,
        rs_R: c_int,
        cs_R: c_int,
        C: *mut dcomplex,
        rs_C: c_int,
        cs_C: c_int,
        D: *mut dcomplex,
        rs_D: c_int,
        cs_D: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT(R: FLA_Obj, C: FLA_Obj, D: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_internal(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        cntl: *mut fla_uddateut_t,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_create_T(R: FLA_Obj, T: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_UDdate_UT_update_rhs(
        T: FLA_Obj,
        bR: FLA_Obj,
        C: FLA_Obj,
        bC: FLA_Obj,
        D: FLA_Obj,
        bD: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_solve(R: FLA_Obj, bR: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLASH_UDdate_UT_inc(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_UDdate_UT_inc_blk_var1(
        R: FLA_Obj,
        C: FLA_Obj,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        cntl: *mut fla_uddateutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_UDdate_UT_inc_create_hier_matrices(
        R_flat: FLA_Obj,
        C_flat: FLA_Obj,
        D_flat: FLA_Obj,
        depth: dim_t,
        b_flash: *mut dim_t,
        b_alg: dim_t,
        R: *mut FLA_Obj,
        C: *mut FLA_Obj,
        D: *mut FLA_Obj,
        T: *mut FLA_Obj,
        W: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_UDdate_UT_inc_determine_alg_blocksize(R: FLA_Obj) -> dim_t;
    pub fn FLASH_UDdate_UT_inc_update_rhs(
        T: FLA_Obj,
        bR: FLA_Obj,
        C: FLA_Obj,
        bC: FLA_Obj,
        D: FLA_Obj,
        bD: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_UDdate_UT_inc_solve(R: FLA_Obj, bR: FLA_Obj, x: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_unb_var1(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_blk_var2(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_opt_var1(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_ops_var1(
        m_A: c_int,
        n_AT: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut f32,
        inc_t: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_opd_var1(
        m_A: c_int,
        n_AT: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut f64,
        inc_t: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_opc_var1(
        m_A: c_int,
        n_AT: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut scomplex,
        inc_t: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fc_opz_var1(
        m_A: c_int,
        n_AT: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut dcomplex,
        inc_t: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_unb_var1(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_blk_var2(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_opt_var1(A: FLA_Obj, t: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_ops_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut f32,
        inc_t: c_int,
        T: *mut f32,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_opd_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut f64,
        inc_t: c_int,
        T: *mut f64,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_opc_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut scomplex,
        inc_t: c_int,
        T: *mut scomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_fr_opz_var1(
        m_A: c_int,
        n_A: c_int,
        A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        m_t: c_int,
        t: *mut dcomplex,
        inc_t: c_int,
        T: *mut dcomplex,
        rs_T: c_int,
        cs_T: c_int,
    ) -> FLA_Error;
    pub fn FLA_Accum_T_UT_internal(
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        tau: FLA_Obj,
        T: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_lf_opt_var1(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_lf_blk_var3(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_lb_opt_var1(c: FLA_Obj, s: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_lb_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f32,
        inc_c: c_int,
        buff_s: *mut f32,
        inc_s: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_lb_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f64,
        inc_c: c_int,
        buff_s: *mut f64,
        inc_s: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_lb_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f32,
        inc_c: c_int,
        buff_s: *mut f32,
        inc_s: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_lb_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f64,
        inc_c: c_int,
        buff_s: *mut f64,
        inc_s: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var1(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var1(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var1(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var1(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var2(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var2(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var2(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var2(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var3(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var3(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var3(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var4(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var4(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var4(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var4(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var5(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var5(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var5(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var5(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var6(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var6(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var6(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var6(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var7(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var7(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var7(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var7(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var8(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var8(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var8(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var8(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opt_var9(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ops_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opd_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opc_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_opz_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var9(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var9(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var9(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var3b(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var3b(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var3b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var5b(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var5b(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var5b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var6b(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var6b(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var6b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var8b(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var8b(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var8b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bhs_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bhd_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bhc_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bhz_var3(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut FLA_Obj,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asm_var9b(G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rf_ass_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asd_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asc_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_asz_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        iTL: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blk_var9b(G: FLA_Obj, A: FLA_Obj, b_alg: dim_t) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bls_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_bld_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blc_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut scomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rf_blz_var9b(
        k_G: c_int,
        m_A: c_int,
        n_A: c_int,
        i_k: c_int,
        buff_G: *mut dcomplex,
        rs_G: c_int,
        cs_G: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        b_alg: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rb_opt_var1(c: FLA_Obj, s: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_rb_ops_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f32,
        inc_c: c_int,
        buff_s: *mut f32,
        inc_s: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rb_opd_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f64,
        inc_c: c_int,
        buff_s: *mut f64,
        inc_s: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rb_opc_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f32,
        inc_c: c_int,
        buff_s: *mut f32,
        inc_s: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G_rb_opz_var1(
        m_A: c_int,
        n_A: c_int,
        buff_c: *mut f64,
        inc_c: c_int,
        buff_s: *mut f64,
        inc_s: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_G(side: FLA_Side, direct: FLA_Direct, G: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_G_internal(
        side: FLA_Side,
        direct: FLA_Direct,
        G: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Givens2(
        chi_1: FLA_Obj,
        chi_2: FLA_Obj,
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        chi_1_new: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Givens2_ops(
        chi_1: *mut f32,
        chi_2: *mut f32,
        gamma: *mut f32,
        sigma: *mut f32,
        chi_1_new: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Givens2_opd(
        chi_1: *mut f64,
        chi_2: *mut f64,
        gamma: *mut f64,
        sigma: *mut f64,
        chi_1_new: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Apply_GTG(
        gamma: FLA_Obj,
        sigma: FLA_Obj,
        delta1: FLA_Obj,
        epsilon1: FLA_Obj,
        delta2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_GTG_ops(
        gamma: *mut f32,
        sigma: *mut f32,
        delta1: *mut f32,
        epsilon1: *mut f32,
        delta2: *mut f32,
    ) -> FLA_Error;
    pub fn FLA_Apply_GTG_opd(
        gamma: *mut f64,
        sigma: *mut f64,
        delta1: *mut f64,
        epsilon1: *mut f64,
        delta2: *mut f64,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_unb_var1(
        tau: FLA_Obj,
        u2: FLA_Obj,
        a1t: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_opt_var1(
        tau: FLA_Obj,
        u2: FLA_Obj,
        a1t: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_ops_var1(
        m_u2_A2: c_int,
        n_a1t: c_int,
        tau: *mut f32,
        u2: *mut f32,
        inc_u2: c_int,
        a1t: *mut f32,
        inc_a1t: c_int,
        A2: *mut f32,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_opd_var1(
        m_u2_A2: c_int,
        n_a1t: c_int,
        tau: *mut f64,
        u2: *mut f64,
        inc_u2: c_int,
        a1t: *mut f64,
        inc_a1t: c_int,
        A2: *mut f64,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_opc_var1(
        m_u2_A2: c_int,
        n_a1t: c_int,
        tau: *mut scomplex,
        u2: *mut scomplex,
        inc_u2: c_int,
        a1t: *mut scomplex,
        inc_a1t: c_int,
        A2: *mut scomplex,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_l_opz_var1(
        m_u2_A2: c_int,
        n_a1t: c_int,
        tau: *mut dcomplex,
        u2: *mut dcomplex,
        inc_u2: c_int,
        a1t: *mut dcomplex,
        inc_a1t: c_int,
        A2: *mut dcomplex,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_unb_var1(
        tau: FLA_Obj,
        u2h: FLA_Obj,
        a1: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_opt_var1(
        tau: FLA_Obj,
        u2h: FLA_Obj,
        a1: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_ops_var1(
        n_u2h_A2: c_int,
        m_a1: c_int,
        tau: *mut f32,
        u2h: *mut f32,
        inc_u2h: c_int,
        a1: *mut f32,
        inc_a1: c_int,
        A2: *mut f32,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_opd_var1(
        n_u2h_A2: c_int,
        m_a1: c_int,
        tau: *mut f64,
        u2h: *mut f64,
        inc_u2h: c_int,
        a1: *mut f64,
        inc_a1: c_int,
        A2: *mut f64,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_opc_var1(
        n_u2h_A2: c_int,
        m_a1: c_int,
        tau: *mut scomplex,
        u2h: *mut scomplex,
        inc_u2h: c_int,
        a1: *mut scomplex,
        inc_a1: c_int,
        A2: *mut scomplex,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_r_opz_var1(
        n_u2h_A2: c_int,
        m_a1: c_int,
        tau: *mut dcomplex,
        u2h: *mut dcomplex,
        inc_u2h: c_int,
        a1: *mut dcomplex,
        inc_a1: c_int,
        A2: *mut dcomplex,
        rs_A2: c_int,
        cs_A2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_H2_UT_internal(
        side: FLA_Side,
        tau: FLA_Obj,
        u2: FLA_Obj,
        a1: FLA_Obj,
        A2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_unb_var1(
        tau: FLA_Obj,
        w12t: FLA_Obj,
        r12t: FLA_Obj,
        u1: FLA_Obj,
        C2: FLA_Obj,
        v1: FLA_Obj,
        D2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_opt_var1(
        tau: FLA_Obj,
        w12t: FLA_Obj,
        r12t: FLA_Obj,
        u1: FLA_Obj,
        C2: FLA_Obj,
        v1: FLA_Obj,
        D2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_ops_var1(
        m_u1_C2: c_int,
        m_v1_D2: c_int,
        n_r12t: c_int,
        tau: *mut f32,
        w12t: *mut f32,
        inc_w12t: c_int,
        r12t: *mut f32,
        inc_r12t: c_int,
        u1: *mut f32,
        inc_u1: c_int,
        C2: *mut f32,
        rs_C2: c_int,
        cs_C2: c_int,
        v1: *mut f32,
        inc_v1: c_int,
        D2: *mut f32,
        rs_D2: c_int,
        cs_D2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_opd_var1(
        m_u1_C2: c_int,
        m_v1_D2: c_int,
        n_r12t: c_int,
        tau: *mut f64,
        w12t: *mut f64,
        inc_w12t: c_int,
        r12t: *mut f64,
        inc_r12t: c_int,
        u1: *mut f64,
        inc_u1: c_int,
        C2: *mut f64,
        rs_C2: c_int,
        cs_C2: c_int,
        v1: *mut f64,
        inc_v1: c_int,
        D2: *mut f64,
        rs_D2: c_int,
        cs_D2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_opc_var1(
        m_u1_C2: c_int,
        m_v1_D2: c_int,
        n_r12t: c_int,
        tau: *mut scomplex,
        w12t: *mut scomplex,
        inc_w12t: c_int,
        r12t: *mut scomplex,
        inc_r12t: c_int,
        u1: *mut scomplex,
        inc_u1: c_int,
        C2: *mut scomplex,
        rs_C2: c_int,
        cs_C2: c_int,
        v1: *mut scomplex,
        inc_v1: c_int,
        D2: *mut scomplex,
        rs_D2: c_int,
        cs_D2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_l_opz_var1(
        m_u1_C2: c_int,
        m_v1_D2: c_int,
        n_r12t: c_int,
        tau: *mut dcomplex,
        w12t: *mut dcomplex,
        inc_w12t: c_int,
        r12t: *mut dcomplex,
        inc_r12t: c_int,
        u1: *mut dcomplex,
        inc_u1: c_int,
        C2: *mut dcomplex,
        rs_C2: c_int,
        cs_C2: c_int,
        v1: *mut dcomplex,
        inc_v1: c_int,
        D2: *mut dcomplex,
        rs_D2: c_int,
        cs_D2: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_HUD_UT_internal(
        side: FLA_Side,
        tau: FLA_Obj,
        w12t: FLA_Obj,
        r12t: FLA_Obj,
        u1: FLA_Obj,
        C2: FLA_Obj,
        v1: FLA_Obj,
        D2: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfc_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfc_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfc_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfr_blk_var1(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfr_blk_var2(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfr_blk_var3(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnfr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lnbr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhfr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_lhbr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhbr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rhfr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnbr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfc(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_rnfr(
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_create_workspace(T: FLA_Obj, B: FLA_Obj, W: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_create_workspace_side(
        side: FLA_Side,
        T: FLA_Obj,
        B: FLA_Obj,
        W: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Apply_Q_UT(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Apply_Q_UT_create_workspace(TW: FLA_Obj, B: FLA_Obj, W: *mut FLA_Obj)
        -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lhfc_blk_var1(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lhfc_blk_var2(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lhfc_blk_var3(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lnfc_blk_var1(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lnfc_blk_var2(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lnfc_blk_var3(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLASH_Apply_Q2_UT(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lhfc(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q2_UT_lnfc(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_lhfc_blk_var1(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_lhfc_blk_var2(
        D: FLA_Obj,
        T: FLA_Obj,
        W1: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_lhfc_blk_var3(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ2_UT_lhfc(
        D: FLA_Obj,
        T: FLA_Obj,
        W: FLA_Obj,
        C: FLA_Obj,
        E: FLA_Obj,
        cntl: *mut fla_apcaq2ut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_lhfc_blk_var1(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_lhfc_blk_var2(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_lhfc_blk_var3(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_lhfc(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudut_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_create_workspace(T: FLA_Obj, R: FLA_Obj, W: *mut FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_lhfc_blk_var1(
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_lnfc_blk_var1(
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_Apply_Q_UT_inc(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Apply_Q_UT_inc_create_workspace(
        TW: FLA_Obj,
        B: FLA_Obj,
        W: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_lhfc(
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_Q_UT_inc_lnfc(
        A: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_lhfc_blk_var1(
        R: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apcaqutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_Apply_CAQ_UT_inc(
        p: dim_t,
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        A: FLA_Obj,
        ATW: FLA_Obj,
        R: FLA_Obj,
        RTW: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_apply_panels(
        nb_part: dim_t,
        A: FLA_Obj,
        ATW: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Apply_CAQ_UT_inc_create_workspace(
        p: dim_t,
        TW: FLA_Obj,
        B: FLA_Obj,
        W: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        R: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apcaqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_CAQ_UT_inc_lhfc(
        R: FLA_Obj,
        TW: FLA_Obj,
        W1: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_apcaqutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_inc_lhfc_blk_var1(
        T: FLA_Obj,
        W: FLA_Obj,
        B: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_Apply_QUD_UT_inc(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_inc_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        direct: FLA_Direct,
        storev: FLA_Store,
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudutinc_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_QUD_UT_inc_lhfc(
        T: FLA_Obj,
        W: FLA_Obj,
        R: FLA_Obj,
        U: FLA_Obj,
        C: FLA_Obj,
        V: FLA_Obj,
        D: FLA_Obj,
        cntl: *mut fla_apqudutinc_t,
    ) -> FLA_Error;
    pub fn FLASH_Apply_QUD_UT_inc_create_workspace(
        T: FLA_Obj,
        R: FLA_Obj,
        W: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_blk_var1(
        p: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_appiv_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_blk_var2(
        p: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_appiv_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_opt_var1(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_opi_var1(
        n: c_int,
        a: *mut c_int,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_ops_var1(
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_opd_var1(
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_opc_var1(
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln_opz_var1(
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_lt_opt_var1(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn_opt_var1(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn_ops_var1(
        n: c_int,
        a: *mut f32,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn_opd_var1(
        n: c_int,
        a: *mut f64,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn_opc_var1(
        n: c_int,
        a: *mut scomplex,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn_opz_var1(
        n: c_int,
        a: *mut dcomplex,
        a_rs: c_int,
        a_cs: c_int,
        k1: c_int,
        k2: c_int,
        p: *mut c_int,
        incp: c_int,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_rt_opt_var1(p: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLA_Apply_pivots_internal(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
        cntl: *mut fla_appiv_t,
    ) -> FLA_Error;
    pub fn FLA_Apply_pivots_ln(p: FLA_Obj, A: FLA_Obj, cntl: *mut fla_appiv_t) -> FLA_Error;
    pub fn FLA_Apply_pivots_lt(p: FLA_Obj, A: FLA_Obj, cntl: *mut fla_appiv_t) -> FLA_Error;
    pub fn FLA_Apply_pivots_rn(p: FLA_Obj, A: FLA_Obj, cntl: *mut fla_appiv_t) -> FLA_Error;
    pub fn FLA_Apply_pivots_rt(p: FLA_Obj, A: FLA_Obj, cntl: *mut fla_appiv_t) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_var1(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_var2(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_var3(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_blk_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_unb_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opt_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_ops_var1(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opd_var1(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opc_var1(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opz_var1(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opt_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_ops_var2(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opd_var2(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opc_var2(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opz_var2(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opt_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_ops_var3(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opd_var3(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opc_var3(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opz_var3(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opt_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_ops_var4(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opd_var4(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opc_var4(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opz_var4(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opt_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_il_ops_var5(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opd_var5(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opc_var5(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il_opz_var5(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_var1(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_var2(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_var3(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_blk_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_unb_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opt_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_ops_var1(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opd_var1(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opc_var1(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opz_var1(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opt_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_ops_var2(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opd_var2(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opc_var2(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opz_var2(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opt_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_ops_var3(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f32,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opd_var3(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut f64,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opc_var3(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut scomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opz_var3(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_Y: *mut dcomplex,
        rs_Y: c_int,
        cs_Y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opt_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_ops_var4(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opd_var4(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opc_var4(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opz_var4(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opt_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_ops_var5(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opd_var5(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opc_var5(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu_opz_var5(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_var1(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_var2(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_var3(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_blk_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_unb_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opt_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_ops_var1(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opd_var1(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opc_var1(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opz_var1(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opt_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_ops_var2(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opd_var2(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opc_var2(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opz_var2(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opt_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_ops_var3(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opd_var3(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opc_var3(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opz_var3(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opt_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_ops_var4(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opd_var4(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opc_var4(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opz_var4(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opt_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_ops_var5(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opd_var5(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opc_var5(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl_opz_var5(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_var1(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_var2(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_var3(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_var4(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_blk_var5(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_unb_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opt_var1(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_ops_var1(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opd_var1(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opc_var1(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opz_var1(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opt_var2(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_ops_var2(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opd_var2(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opc_var2(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opz_var2(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opt_var3(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_ops_var3(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opd_var3(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opc_var3(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opz_var3(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opt_var4(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_ops_var4(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opd_var4(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opc_var4(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opz_var4(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opt_var5(A: FLA_Obj, Y: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_ops_var5(
        m_AB: c_int,
        buff_A: *mut f32,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f32,
        inc_y: c_int,
        buff_B: *mut f32,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opd_var5(
        m_AB: c_int,
        buff_A: *mut f64,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut f64,
        inc_y: c_int,
        buff_B: *mut f64,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opc_var5(
        m_AB: c_int,
        buff_A: *mut scomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut scomplex,
        inc_y: c_int,
        buff_B: *mut scomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu_opz_var5(
        m_AB: c_int,
        buff_A: *mut dcomplex,
        rs_A: c_int,
        cs_A: c_int,
        buff_y: *mut dcomplex,
        inc_y: c_int,
        buff_B: *mut dcomplex,
        rs_B: c_int,
        cs_B: c_int,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_internal(
        inv: FLA_Inv,
        uplo: FLA_Uplo,
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_il(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_iu(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nl(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLA_Eig_gest_nu(
        A: FLA_Obj,
        Y: FLA_Obj,
        B: FLA_Obj,
        cntl: *mut fla_eig_gest_t,
    ) -> FLA_Error;
    pub fn FLASH_Obj_blocksizes_check(H: FLA_Obj, b_m: *mut dim_t, b_n: *mut dim_t) -> FLA_Error;
    pub fn FLASH_Obj_create_helper_check(
        without_buffer: FLA_Bool,
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hierarchy_check(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        elem_sizes_m: *mut dim_t,
        elem_sizes_n: *mut dim_t,
        flat_matrix: FLA_Obj,
        H: *mut FLA_Obj,
        id: c_ulong,
        depth_overall: dim_t,
        depth_sizes_m: *mut dim_t,
        depth_sizes_n: *mut dim_t,
        m_offsets: *mut dim_t,
        n_offsets: *mut dim_t,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_conf_to_check(
        trans: FLA_Trans,
        H_cur: FLA_Obj,
        H_new: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_conf_to_flat_check(
        trans: FLA_Trans,
        F: FLA_Obj,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_conf_to_flat_ext_check(
        trans: FLA_Trans,
        F: FLA_Obj,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_flat_conf_to_hier_check(
        trans: FLA_Trans,
        H: FLA_Obj,
        F: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_copy_of_flat_check(
        F: FLA_Obj,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_copy_of_flat_ext_check(
        F: FLA_Obj,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_flat_copy_of_hier_check(H: FLA_Obj, F: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_free_check(H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_free_without_buffer_check(H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_free_hierarchy_check(H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_attach_buffer_check(
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_attach_buffer_hierarchy_check(F: FLA_Obj, H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Part_create_2x1(
        A: FLA_Obj,
        AT: *mut FLA_Obj,
        AB: *mut FLA_Obj,
        n_rows: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLASH_Part_create_1x2(
        A: FLA_Obj,
        AL: *mut FLA_Obj,
        AR: *mut FLA_Obj,
        n_cols: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLASH_Part_create_2x2(
        A: FLA_Obj,
        ATL: *mut FLA_Obj,
        ATR: *mut FLA_Obj,
        ABL: *mut FLA_Obj,
        ABR: *mut FLA_Obj,
        n_rows: dim_t,
        n_cols: dim_t,
        side: FLA_Side,
    ) -> FLA_Error;
    pub fn FLASH_Part_free_2x1(AT: *mut FLA_Obj, AB: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Part_free_1x2(AL: *mut FLA_Obj, AR: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Part_free_2x2(
        ATL: *mut FLA_Obj,
        ATR: *mut FLA_Obj,
        ABL: *mut FLA_Obj,
        ABR: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_adjust_views(
        attach_buffer: FLA_Bool,
        offm: dim_t,
        offn: dim_t,
        m: dim_t,
        n: dim_t,
        A: FLA_Obj,
        S: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_adjust_views_hierarchy(
        attach_buffer: FLA_Bool,
        offm: dim_t,
        offn: dim_t,
        m: dim_t,
        n: dim_t,
        A: FLA_Obj,
        S: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_scalar_length(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_width(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_min_dim(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_max_dim(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_vector_dim(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_row_offset(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_col_offset(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_length_tl(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_scalar_width_tl(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_base_scalar_length(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_base_scalar_width(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_show(
        header: *mut c_char,
        H: FLA_Obj,
        elem_format: *mut c_char,
        footer: *mut c_char,
    ) -> FLA_Error;
    pub fn FLASH_Obj_show_hierarchy(H: FLA_Obj, i: dim_t, elem_format: *mut c_char) -> FLA_Error;
    pub fn FLASH_Axpy_buffer_to_hier(
        alpha: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Axpy_hier_to_buffer(
        alpha: FLA_Obj,
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLASH_Axpy_flat_to_hier(
        alpha: FLA_Obj,
        F: FLA_Obj,
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Axpy_hier_to_flat(
        alpha: FLA_Obj,
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
        F: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Axpy_hierarchy(
        direction: c_int,
        alpha: FLA_Obj,
        F: FLA_Obj,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Copy_buffer_to_hier(
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Copy_hier_to_buffer(
        i: dim_t,
        j: dim_t,
        H: FLA_Obj,
        m: dim_t,
        n: dim_t,
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
    ) -> FLA_Error;
    pub fn FLASH_Copy_flat_to_hier(F: FLA_Obj, i: dim_t, j: dim_t, H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Copy_hier_to_flat(i: dim_t, j: dim_t, H: FLA_Obj, F: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Copy_hierarchy(direction: c_int, F: FLA_Obj, H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_datatype(H: FLA_Obj) -> FLA_Datatype;
    pub fn FLASH_Obj_depth(H: FLA_Obj) -> dim_t;
    pub fn FLASH_Obj_blocksizes(H: FLA_Obj, b_m: *mut dim_t, b_n: *mut dim_t) -> dim_t;
    pub fn FLASH_Obj_create(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_ext(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_without_buffer(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_without_buffer_ext(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_helper(
        without_buffer: FLA_Bool,
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hierarchy(
        datatype: FLA_Datatype,
        m: dim_t,
        n: dim_t,
        depth: dim_t,
        elem_sizes_m: *mut dim_t,
        elem_sizes_n: *mut dim_t,
        flat_matrix: FLA_Obj,
        H: *mut FLA_Obj,
        id: c_ulong,
        depth_overall: dim_t,
        depth_sizes_m: *mut dim_t,
        depth_sizes_n: *mut dim_t,
        m_offsets: *mut dim_t,
        n_offsets: *mut dim_t,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_conf_to(
        trans: FLA_Trans,
        H_cur: FLA_Obj,
        H_new: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_conf_to_flat(
        trans: FLA_Trans,
        F: FLA_Obj,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_conf_to_flat_ext(
        trans: FLA_Trans,
        F: FLA_Obj,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_flat_conf_to_hier(
        trans: FLA_Trans,
        H: FLA_Obj,
        F: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_copy_of(
        trans: FLA_Trans,
        H_cur: FLA_Obj,
        H_new: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_copy_of_flat(
        F: FLA_Obj,
        depth: dim_t,
        b_mn: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_hier_copy_of_flat_ext(
        F: FLA_Obj,
        depth: dim_t,
        b_m: *mut dim_t,
        b_n: *mut dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_create_flat_copy_of_hier(H: FLA_Obj, F: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_free(H: *mut FLA_Obj);
    pub fn FLASH_Obj_free_hierarchy(H: *mut FLA_Obj);
    pub fn FLASH_Obj_free_without_buffer(H: *mut FLA_Obj);
    pub fn FLASH_Obj_attach_buffer(
        buffer: *mut c_void,
        rs: dim_t,
        cs: dim_t,
        H: *mut FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Obj_attach_buffer_hierarchy(F: FLA_Obj, H: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_flatten(H: FLA_Obj, F: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_hierarchify(F: FLA_Obj, H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_extract_buffer(H: FLA_Obj) -> *mut c_void;
    pub fn FLASH_print_struct(H: FLA_Obj);
    pub fn FLASH_print_struct_helper(H: FLA_Obj, indent: c_int);
    pub fn FLASH_Max_elemwise_diff(A: FLA_Obj, B: FLA_Obj) -> f64;
    pub fn FLASH_Random_matrix(H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Random_spd_matrix(uplo: FLA_Uplo, H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Norm1(H: FLA_Obj, norm: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_shift_diagonal(conj: FLA_Conj, sigma: FLA_Obj, H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Set(alpha: FLA_Obj, H: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Obj_create_diag_panel(A: FLA_Obj, U: *mut FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_find_zero_on_diagonal(A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Triangularize(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Hermitianize(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_find_zero_on_diagonal_check(A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Axpy(alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Axpyt(trans: FLA_Trans, alpha: FLA_Obj, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Copy(A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Copyt(trans: FLA_Trans, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Scal(alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Scalr(uplo: FLA_Uplo, alpha: FLA_Obj, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Gemv(
        transa: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        x: FLA_Obj,
        beta: FLA_Obj,
        y: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Trsv(
        uplo: FLA_Uplo,
        transa: FLA_Trans,
        diag: FLA_Diag,
        A: FLA_Obj,
        x: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Gemm(
        transa: FLA_Trans,
        transb: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Hemm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Herk(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Her2k(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Symm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Syrk(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Syr2k(
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        beta: FLA_Obj,
        C: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Trmm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Trsm(
        side: FLA_Side,
        uplo: FLA_Uplo,
        trans: FLA_Trans,
        diag: FLA_Diag,
        alpha: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Chol(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_nopiv(A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_piv(A: FLA_Obj, p: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LU_incpiv(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj) -> FLA_Error;
    pub fn FLASH_FS_incpiv(A: FLA_Obj, p: FLA_Obj, L: FLA_Obj, b: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Trinv(uplo: FLA_Uplo, diag: FLA_Diag, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Ttmm(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_SPDinv(uplo: FLA_Uplo, A: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Sylv(
        transa: FLA_Trans,
        transb: FLA_Trans,
        isgn: FLA_Obj,
        A: FLA_Obj,
        B: FLA_Obj,
        C: FLA_Obj,
        scale: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Apply_pivots(
        side: FLA_Side,
        trans: FLA_Trans,
        p: FLA_Obj,
        A: FLA_Obj,
    ) -> FLA_Error;
    pub fn FLASH_Eig_gest(inv: FLA_Inv, uplo: FLA_Uplo, A: FLA_Obj, B: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LQ_UT_inv(A: FLA_Obj, TW: FLA_Obj) -> FLA_Error;
    pub fn FLASH_LQ2_UT(B: FLA_Obj, C: FLA_Obj, T: FLA_Obj) -> FLA_Error;
    pub fn FLASH_Queue_begin();
    pub fn FLASH_Queue_end();
    pub fn FLASH_Queue_stack_depth() -> c_uint;
    pub fn FLASH_Queue_enable() -> FLA_Error;
    pub fn FLASH_Queue_disable() -> FLA_Error;
    pub fn FLASH_Queue_get_enabled() -> FLA_Bool;
    pub fn FLASH_Queue_set_num_threads(n_threads: c_uint);
    pub fn FLASH_Queue_get_num_threads() -> c_uint;
    pub fn spotrf_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dpotrf_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cpotrf_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zpotrf_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn spotf2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dpotf2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cpotf2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zpotf2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgetrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgetrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgetrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgetrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgetf2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgetf2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgetf2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgetf2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        ipiv: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgeqrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgeqrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgeqrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgeqrf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgeqr2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgeqr2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgeqr2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgeqr2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgeqpf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgeqpf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgeqpf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgeqpf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgeqp3_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgeqp3_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgeqp3_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgeqp3_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        jpvt: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgelqf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgelqf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgelqf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgelqf_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgelq2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgelq2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgelq2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgelq2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgelsd_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        s: *mut f32,
        rcond: *mut f32,
        rank: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgelsd_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        s: *mut f64,
        rcond: *mut f64,
        rank: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgelsd_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        s: *mut f32,
        rcond: *mut f32,
        rank: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgelsd_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        s: *mut f64,
        rcond: *mut f64,
        rank: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgelss_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        s: *mut f32,
        rcond: *mut f32,
        rank: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgelss_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        s: *mut f64,
        rcond: *mut f64,
        rank: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgelss_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        s: *mut f32,
        rcond: *mut f32,
        rank: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgelss_(
        m: *mut c_int,
        n: *mut c_int,
        nrhs: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        s: *mut f64,
        rcond: *mut f64,
        rank: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn slauum_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dlauum_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn clauum_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zlauum_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn slauu2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dlauu2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn clauu2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zlauu2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn spotri_(
        uplo: *mut c_char,
        n: *mut c_int,
        buff_A: *mut f32,
        ldim_A: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dpotri_(
        uplo: *mut c_char,
        n: *mut c_int,
        buff_A: *mut f64,
        ldim_A: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cpotri_(
        uplo: *mut c_char,
        n: *mut c_int,
        buff_A: *mut scomplex,
        ldim_A: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zpotri_(
        uplo: *mut c_char,
        n: *mut c_int,
        buff_A: *mut dcomplex,
        ldim_A: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn strtri_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dtrtri_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ctrtri_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ztrtri_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn strti2_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dtrti2_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ctrti2_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ztrti2_(
        uplo: *mut c_char,
        diag: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn strsyl_(
        transa: *mut c_char,
        transb: *mut c_char,
        isgn: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        c: *mut f32,
        ldc: *mut c_int,
        scale: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dtrsyl_(
        transa: *mut c_char,
        transb: *mut c_char,
        isgn: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        c: *mut f64,
        ldc: *mut c_int,
        scale: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn ctrsyl_(
        transa: *mut c_char,
        transb: *mut c_char,
        isgn: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        c: *mut scomplex,
        ldc: *mut c_int,
        scale: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn ztrsyl_(
        transa: *mut c_char,
        transb: *mut c_char,
        isgn: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        c: *mut dcomplex,
        ldc: *mut c_int,
        scale: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgehrd_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgehrd_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgehrd_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgehrd_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgehd2_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgehd2_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgehd2_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgehd2_(
        n: *mut c_int,
        ilo: *mut c_int,
        ihi: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssytrd_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsytrd_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn chetrd_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zhetrd_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssytd2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsytd2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn chetd2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tau: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zhetd2_(
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tau: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgebrd_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut f32,
        taup: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgebrd_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut f64,
        taup: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgebrd_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut scomplex,
        taup: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgebrd_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut dcomplex,
        taup: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgebd2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut f32,
        taup: *mut f32,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgebd2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut f64,
        taup: *mut f64,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgebd2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut scomplex,
        taup: *mut scomplex,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgebd2_(
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut dcomplex,
        taup: *mut dcomplex,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssygst_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsygst_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn chegst_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zhegst_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssygs2_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        b: *mut f32,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsygs2_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        b: *mut f64,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn chegs2_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        b: *mut scomplex,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zhegs2_(
        itype: *mut c_int,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        b: *mut dcomplex,
        ldb: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn slarft_(
        direct: *mut c_char,
        storev: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        v: *mut f32,
        ldv: *mut c_int,
        tau: *mut f32,
        t: *mut f32,
        ldt: *mut c_int,
    ) -> c_int;
    pub fn dlarft_(
        direct: *mut c_char,
        storev: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        v: *mut f64,
        ldv: *mut c_int,
        tau: *mut f64,
        t: *mut f64,
        ldt: *mut c_int,
    ) -> c_int;
    pub fn clarft_(
        direct: *mut c_char,
        storev: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        v: *mut scomplex,
        ldv: *mut c_int,
        tau: *mut scomplex,
        t: *mut scomplex,
        ldt: *mut c_int,
    ) -> c_int;
    pub fn zlarft_(
        direct: *mut c_char,
        storev: *mut c_char,
        n: *mut c_int,
        k: *mut c_int,
        v: *mut dcomplex,
        ldv: *mut c_int,
        tau: *mut dcomplex,
        t: *mut dcomplex,
        ldt: *mut c_int,
    ) -> c_int;
    pub fn slarfg_(
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        tau: *mut f32,
    ) -> c_int;
    pub fn dlarfg_(
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        tau: *mut f64,
    ) -> c_int;
    pub fn clarfg_(
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        tau: *mut scomplex,
    ) -> c_int;
    pub fn zlarfg_(
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        tau: *mut dcomplex,
    ) -> c_int;
    pub fn slarfgp_(
        n: *mut c_int,
        alpha: *mut f32,
        x: *mut f32,
        incx: *mut c_int,
        tau: *mut f32,
    ) -> c_int;
    pub fn dlarfgp_(
        n: *mut c_int,
        alpha: *mut f64,
        x: *mut f64,
        incx: *mut c_int,
        tau: *mut f64,
    ) -> c_int;
    pub fn clarfgp_(
        n: *mut c_int,
        alpha: *mut scomplex,
        x: *mut scomplex,
        incx: *mut c_int,
        tau: *mut scomplex,
    ) -> c_int;
    pub fn zlarfgp_(
        n: *mut c_int,
        alpha: *mut dcomplex,
        x: *mut dcomplex,
        incx: *mut c_int,
        tau: *mut dcomplex,
    ) -> c_int;
    pub fn sorgqr_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorgqr_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cungqr_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zungqr_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sormqr_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dormqr_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunmqr_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunmqr_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sorm2r_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorm2r_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunm2r_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunm2r_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn sorglq_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorglq_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunglq_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunglq_(
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sormlq_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dormlq_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunmlq_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunmlq_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sorml2_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorml2_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunml2_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunml2_(
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        info: *mut c_int,
    ) -> c_int;
    pub fn sorgtr_(
        uplo: *mut c_char,
        m: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorgtr_(
        uplo: *mut c_char,
        m: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cungtr_(
        uplo: *mut c_char,
        m: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zungtr_(
        uplo: *mut c_char,
        m: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sormtr_(
        side: *mut c_char,
        uplo: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dormtr_(
        side: *mut c_char,
        uplo: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunmtr_(
        side: *mut c_char,
        uplo: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunmtr_(
        side: *mut c_char,
        uplo: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sorgbr_(
        vect: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dorgbr_(
        vect: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cungbr_(
        vect: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zungbr_(
        vect: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sormbr_(
        vect: *mut c_char,
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        tau: *mut f32,
        c: *mut f32,
        ldc: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dormbr_(
        vect: *mut c_char,
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        tau: *mut f64,
        c: *mut f64,
        ldc: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cunmbr_(
        vect: *mut c_char,
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        tau: *mut scomplex,
        c: *mut scomplex,
        ldc: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zunmbr_(
        vect: *mut c_char,
        side: *mut c_char,
        trans: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        k: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        tau: *mut dcomplex,
        c: *mut dcomplex,
        ldc: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssteqr_(
        jobz: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *mut c_int,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsteqr_(
        jobz: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *mut c_int,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn csteqr_(
        jobz: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        z: *mut scomplex,
        ldz: *mut c_int,
        work: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zsteqr_(
        jobz: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        z: *mut dcomplex,
        ldz: *mut c_int,
        work: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sstedc_(
        compz: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dstedc_(
        compz: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cstedc_(
        compz: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        z: *mut scomplex,
        ldz: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zstedc_(
        compz: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        z: *mut dcomplex,
        ldz: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sstemr_(
        jobz: *mut c_char,
        range: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        vl: *mut c_int,
        vu: *mut c_int,
        il: *mut c_int,
        iu: *mut c_int,
        m: *mut c_int,
        w: *mut f32,
        z: *mut f32,
        ldz: *mut c_int,
        nzc: *mut c_int,
        isuppz: *mut c_int,
        tryrac: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dstemr_(
        jobz: *mut c_char,
        range: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        vl: *mut c_int,
        vu: *mut c_int,
        il: *mut c_int,
        iu: *mut c_int,
        m: *mut c_int,
        w: *mut f64,
        z: *mut f64,
        ldz: *mut c_int,
        nzc: *mut c_int,
        isuppz: *mut c_int,
        tryrac: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cstemr_(
        jobz: *mut c_char,
        range: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        vl: *mut c_int,
        vu: *mut c_int,
        il: *mut c_int,
        iu: *mut c_int,
        m: *mut c_int,
        w: *mut f32,
        z: *mut scomplex,
        ldz: *mut c_int,
        nzc: *mut c_int,
        isuppz: *mut c_int,
        tryrac: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zstemr_(
        jobz: *mut c_char,
        range: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        vl: *mut c_int,
        vu: *mut c_int,
        il: *mut c_int,
        iu: *mut c_int,
        m: *mut c_int,
        w: *mut f64,
        z: *mut dcomplex,
        ldz: *mut c_int,
        nzc: *mut c_int,
        isuppz: *mut c_int,
        tryrac: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssyev_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        w: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsyev_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        w: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cheev_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        w: *mut f32,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zheev_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        w: *mut f64,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssyevd_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        w: *mut f32,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsyevd_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        w: *mut f64,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cheevd_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        w: *mut f32,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zheevd_(
        jobz: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        w: *mut f64,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn ssyevr_(
        jobz: *mut c_char,
        range: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        vl: *mut f32,
        vu: *mut f32,
        il: *mut c_int,
        iu: *mut c_int,
        abstol: *mut f32,
        m: *mut c_int,
        w: *mut f32,
        z: *mut f32,
        ldz: *mut c_int,
        isuppz: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dsyevr_(
        jobz: *mut c_char,
        range: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        vl: *mut f64,
        vu: *mut f64,
        il: *mut c_int,
        iu: *mut c_int,
        abstol: *mut f64,
        m: *mut c_int,
        w: *mut f64,
        z: *mut f64,
        ldz: *mut c_int,
        isuppz: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cheevr_(
        jobz: *mut c_char,
        range: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        vl: *mut f32,
        vu: *mut f32,
        il: *mut c_int,
        iu: *mut c_int,
        abstol: *mut f32,
        m: *mut c_int,
        w: *mut f32,
        z: *mut scomplex,
        ldz: *mut c_int,
        isuppz: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zheevr_(
        jobz: *mut c_char,
        range: *mut c_char,
        uplo: *mut c_char,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        vl: *mut f64,
        vu: *mut f64,
        il: *mut c_int,
        iu: *mut c_int,
        abstol: *mut f64,
        m: *mut c_int,
        w: *mut f64,
        z: *mut dcomplex,
        ldz: *mut c_int,
        isuppz: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        lrwork: *mut c_int,
        iwork: *mut c_int,
        liwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sbdsqr_(
        uplo: *mut c_char,
        n: *mut c_int,
        ncvt: *mut c_int,
        nru: *mut c_int,
        ncc: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        vt: *mut f32,
        ldvt: *mut c_int,
        u: *mut f32,
        ldu: *mut c_int,
        c: *mut f32,
        ldc: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn dbdsqr_(
        uplo: *mut c_char,
        n: *mut c_int,
        ncvt: *mut c_int,
        nru: *mut c_int,
        ncc: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        vt: *mut f64,
        ldvt: *mut c_int,
        u: *mut f64,
        ldu: *mut c_int,
        c: *mut f64,
        ldc: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn cbdsqr_(
        uplo: *mut c_char,
        n: *mut c_int,
        ncvt: *mut c_int,
        nru: *mut c_int,
        ncc: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        vt: *mut scomplex,
        ldvt: *mut c_int,
        u: *mut scomplex,
        ldu: *mut c_int,
        c: *mut scomplex,
        ldc: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zbdsqr_(
        uplo: *mut c_char,
        n: *mut c_int,
        ncvt: *mut c_int,
        nru: *mut c_int,
        ncc: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        vt: *mut dcomplex,
        ldvt: *mut c_int,
        u: *mut dcomplex,
        ldu: *mut c_int,
        c: *mut dcomplex,
        ldc: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sbdsdc_(
        uplo: *mut c_char,
        compq: *mut c_char,
        n: *mut c_int,
        d: *mut f32,
        e: *mut f32,
        u: *mut f32,
        ldu: *mut c_int,
        vt: *mut f32,
        ldvt: *mut c_int,
        q: *mut f32,
        iq: *mut f32,
        work: *mut f32,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dbdsdc_(
        uplo: *mut c_char,
        compq: *mut c_char,
        n: *mut c_int,
        d: *mut f64,
        e: *mut f64,
        u: *mut f64,
        ldu: *mut c_int,
        vt: *mut f64,
        ldvt: *mut c_int,
        q: *mut f64,
        iq: *mut f64,
        work: *mut f64,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgesvd_(
        jobu: *mut c_char,
        jobv: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        s: *mut f32,
        u: *mut f32,
        ldu: *mut c_int,
        vt: *mut f32,
        ldvt: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgesvd_(
        jobu: *mut c_char,
        jobv: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        s: *mut f64,
        u: *mut f64,
        ldu: *mut c_int,
        vt: *mut f64,
        ldvt: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgesvd_(
        jobu: *mut c_char,
        jobv: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        s: *mut f32,
        u: *mut scomplex,
        ldu: *mut c_int,
        vt: *mut scomplex,
        ldvt: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgesvd_(
        jobu: *mut c_char,
        jobv: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        s: *mut f64,
        u: *mut dcomplex,
        ldu: *mut c_int,
        vt: *mut dcomplex,
        ldvt: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        info: *mut c_int,
    ) -> c_int;
    pub fn sgesdd_(
        jobz: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        s: *mut f32,
        u: *mut f32,
        ldu: *mut c_int,
        vt: *mut f32,
        ldvt: *mut c_int,
        work: *mut f32,
        lwork: *mut c_int,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn dgesdd_(
        jobz: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        s: *mut f64,
        u: *mut f64,
        ldu: *mut c_int,
        vt: *mut f64,
        ldvt: *mut c_int,
        work: *mut f64,
        lwork: *mut c_int,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn cgesdd_(
        jobz: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        s: *mut f32,
        u: *mut scomplex,
        ldu: *mut c_int,
        vt: *mut scomplex,
        ldvt: *mut c_int,
        work: *mut scomplex,
        lwork: *mut c_int,
        rwork: *mut f32,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn zgesdd_(
        jobz: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        s: *mut f64,
        u: *mut dcomplex,
        ldu: *mut c_int,
        vt: *mut dcomplex,
        ldvt: *mut c_int,
        work: *mut dcomplex,
        lwork: *mut c_int,
        rwork: *mut f64,
        iwork: *mut c_int,
        info: *mut c_int,
    ) -> c_int;
    pub fn slaswp_(
        n: *mut c_int,
        a: *mut f32,
        lda: *mut c_int,
        k1: *mut c_int,
        k2: *mut c_int,
        ipiv: *mut c_int,
        incx: *mut c_int,
    ) -> c_int;
    pub fn dlaswp_(
        n: *mut c_int,
        a: *mut f64,
        lda: *mut c_int,
        k1: *mut c_int,
        k2: *mut c_int,
        ipiv: *mut c_int,
        incx: *mut c_int,
    ) -> c_int;
    pub fn claswp_(
        n: *mut c_int,
        a: *mut scomplex,
        lda: *mut c_int,
        k1: *mut c_int,
        k2: *mut c_int,
        ipiv: *mut c_int,
        incx: *mut c_int,
    ) -> c_int;
    pub fn zlaswp_(
        n: *mut c_int,
        a: *mut dcomplex,
        lda: *mut c_int,
        k1: *mut c_int,
        k2: *mut c_int,
        ipiv: *mut c_int,
        incx: *mut c_int,
    ) -> c_int;
    pub fn slaset_(
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f32,
        beta: *mut f32,
        a: *mut f32,
        lda: *mut c_int,
    ) -> c_int;
    pub fn dlaset_(
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut f64,
        beta: *mut f64,
        a: *mut f64,
        lda: *mut c_int,
    ) -> c_int;
    pub fn claset_(
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut scomplex,
        beta: *mut scomplex,
        a: *mut scomplex,
        lda: *mut c_int,
    ) -> c_int;
    pub fn zlaset_(
        uplo: *mut c_char,
        m: *mut c_int,
        n: *mut c_int,
        alpha: *mut dcomplex,
        beta: *mut dcomplex,
        a: *mut dcomplex,
        lda: *mut c_int,
    ) -> c_int;
}
