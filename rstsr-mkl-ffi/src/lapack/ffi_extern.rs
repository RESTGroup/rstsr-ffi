//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe extern "C" {
    pub fn cgesvdq_(
        joba: *const c_char,
        jobp: *const c_char,
        jobr: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        numrank: *mut MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        cwork: *mut MKL_Complex8,
        lcwork: *mut MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvdq_(
        joba: *const c_char,
        jobp: *const c_char,
        jobr: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        numrank: *mut MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        work: *mut f64,
        lwork: *mut MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvdq_(
        joba: *const c_char,
        jobp: *const c_char,
        jobr: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        numrank: *mut MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        work: *mut f32,
        lwork: *mut MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvdq_(
        joba: *const c_char,
        jobp: *const c_char,
        jobr: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        numrank: *mut MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        cwork: *mut MKL_Complex16,
        lcwork: *mut MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claunhr_col_getrfnp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn claunhr_col_getrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dlaorhr_col_getrfnp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaorhr_col_getrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slaorhr_col_getrfnp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaorhr_col_getrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zlaunhr_col_getrfnp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zlaunhr_col_getrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cungtsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunhr_col_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        d: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dorgtsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorhr_col_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        d: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn sorgtsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorhr_col_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        d: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zungtsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunhr_col_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        d: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn sgetsqrhrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb1: *const MKL_INT,
        nb1: *const MKL_INT,
        nb2: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetsqrhrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb1: *const MKL_INT,
        nb1: *const MKL_INT,
        nb2: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetsqrhrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb1: *const MKL_INT,
        nb1: *const MKL_INT,
        nb2: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetsqrhrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb1: *const MKL_INT,
        nb1: *const MKL_INT,
        nb2: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarfb_gett_(
        ident: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        ldwork: *const MKL_INT,
    );
    pub fn dlarfb_gett_(
        ident: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        ldwork: *const MKL_INT,
    );
    pub fn clarfb_gett_(
        ident: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        ldwork: *const MKL_INT,
    );
    pub fn zlarfb_gett_(
        ident: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        ldwork: *const MKL_INT,
    );
    pub fn sorgtsqr_row_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgtsqr_row_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungtsqr_row_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungtsqr_row_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        tsize: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgelqt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgemlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        t: *const MKL_Complex8,
        tsize: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgemlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgeqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        tsize: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetsls_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clamswlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clamtsqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claswlq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clatsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctplqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctplqt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctpmlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dgelq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        tsize: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgelqt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgemlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        t: *const f64,
        tsize: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgemlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        tsize: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlamswlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlamtsqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaswlq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtplqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtplqt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtpmlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn sgelq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        tsize: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetsls_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgelqt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgemlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        tsize: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetsls_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slamtsqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaswlq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stplqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stpmlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zgelq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        tsize: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgemlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        t: *const f32,
        tsize: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slamswlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slatsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stplqt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgelqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgelqt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgemlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        t: *const MKL_Complex16,
        tsize: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgemlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgeqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        tsize: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetsls_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlamswlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlamtsqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaswlq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlatsqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        mb: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztplqt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztplqt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztpmlqt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        mb: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn chesv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clahef_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dlasyf_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsysv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slasyf_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssysv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhesv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlahef_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cgemqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        t: *const MKL_Complex8,
        tsize: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgemqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        t: *const f64,
        tsize: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgemqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        t: *const f32,
        tsize: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgemqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        t: *const MKL_Complex16,
        tsize: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn checon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chesv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clahef_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clarfy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clasyf_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn clasyf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csycon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csyconvf_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csyconvf_rook_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csysv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csysv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        e: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarfy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlasyf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *mut MKL_INT,
        w: *mut f64,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsycon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        e: *const f64,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyconvf_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyconvf_rook_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        ipiv: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *const f64,
        ipiv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *const f64,
        ipiv: *const MKL_INT,
        work: *mut f64,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        e: *const f64,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarfy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slasyf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *mut MKL_INT,
        w: *mut f32,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssycon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        e: *const f32,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyconvf_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyconvf_rook_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        ipiv: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *const f32,
        ipiv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *const f32,
        ipiv: *const MKL_INT,
        work: *mut f32,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        e: *const f32,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhecon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhesv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlahef_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlarfy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlasyf_aa_(
        uplo: *const c_char,
        j1: *const MKL_INT,
        m: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zlasyf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsycon_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsyconvf_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsyconvf_rook_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsysv_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsysv_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytf2_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrf_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrf_rk_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri_3x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrs_3_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        e: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrs_aa_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chb2st_kernels_(
        uplo: *const c_char,
        wantz: *const MKL_INT,
        ttype: *const MKL_INT,
        st: *const MKL_INT,
        ed: *const MKL_INT,
        sweep: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        ib: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        v: *mut MKL_Complex8,
        tau: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn chbev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chbevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chbevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cheevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheevr_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chegv_2stage_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chetrd_2stage_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        hous2: *mut MKL_Complex8,
        lhous2: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrd_hb2st_(
        stage1: *const c_char,
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        hous: *mut MKL_Complex8,
        lhous: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrd_he2hb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsb2st_kernels_(
        uplo: *const c_char,
        wantz: *const MKL_INT,
        ttype: *const MKL_INT,
        st: *const MKL_INT,
        ed: *const MKL_INT,
        sweep: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        ib: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        v: *mut f64,
        tau: *mut f64,
        ldvt: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dsbev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyevr_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygv_2stage_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrd_2stage_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        hous2: *mut f64,
        lhous2: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrd_sb2st_(
        stage1: *const c_char,
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        hous: *mut f64,
        lhous: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrd_sy2sb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn iparam2stage_(
        ispec: *const MKL_INT,
        name: *const c_char,
        opts: *const c_char,
        ni: *const MKL_INT,
        nbi: *const MKL_INT,
        ibi: *const MKL_INT,
        nxi: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ssb2st_kernels_(
        uplo: *const c_char,
        wantz: *const MKL_INT,
        ttype: *const MKL_INT,
        st: *const MKL_INT,
        ed: *const MKL_INT,
        sweep: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        ib: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        v: *mut f32,
        tau: *mut f32,
        ldvt: *const MKL_INT,
        work: *mut f32,
    );
    pub fn ssbev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyevr_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygv_2stage_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrd_2stage_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        hous2: *mut f32,
        lhous2: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrd_sb2st_(
        stage1: *const c_char,
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        hous: *mut f32,
        lhous: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrd_sy2sb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhb2st_kernels_(
        uplo: *const c_char,
        wantz: *const MKL_INT,
        ttype: *const MKL_INT,
        st: *const MKL_INT,
        ed: *const MKL_INT,
        sweep: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        ib: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        v: *mut MKL_Complex16,
        tau: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zhbev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhbevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhbevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheev_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zheevd_2stage_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheevr_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheevx_2stage_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhegv_2stage_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhetrd_2stage_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        hous2: *mut MKL_Complex16,
        lhous2: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrd_hb2st_(
        stage1: *const c_char,
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        hous: *mut MKL_Complex16,
        lhous: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrd_he2hb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_cgetrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_dgetrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_sgetrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_zgetrfnp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_cgetrinp_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_dgetrinp_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_sgetrinp_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_zgetrinp_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tb: *mut f64,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chesv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csysv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex8,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhesv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsysv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tb: *mut MKL_Complex16,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tb: *mut f64,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tb: *mut f64,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysv_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tb: *mut f32,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrs_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tb: *mut f32,
        ltb: *const MKL_INT,
        ipiv: *const MKL_INT,
        ipiv2: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrf_aa_2stage_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tb: *mut f32,
        ltb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv2: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrevc3_(
        side: *const c_char,
        howmny: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strevc3_(
        side: *const c_char,
        howmny: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrevc3_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrevc3_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrf_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        stride_ipiv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrf_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        stride_ipiv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrf_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        stride_ipiv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrf_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        stride_ipiv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrs_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrs_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrs_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrs_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrfnp_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrfnp_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrfnp_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrfnp_batch_strided_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrsnp_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrsnp_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrsnp_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrsnp_batch_strided_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrf_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut *mut MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrf_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut *mut MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrf_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut *mut MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrf_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut *mut MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetri_oop_batch_strided_(
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        stridea: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        ainv: *mut f32,
        ldainv: *const MKL_INT,
        stride_ainv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetri_oop_batch_strided_(
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        stridea: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        ainv: *mut f64,
        ldainv: *const MKL_INT,
        stride_ainv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetri_oop_batch_strided_(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        stridea: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        ainv: *mut MKL_Complex8,
        ldainv: *const MKL_INT,
        stride_ainv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetri_oop_batch_strided_(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        stridea: *const MKL_INT,
        ipiv: *const MKL_INT,
        stride_ipiv: *const MKL_INT,
        ainv: *mut MKL_Complex16,
        ldainv: *const MKL_INT,
        stride_ainv: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetri_oop_batch_(
        n: *const MKL_INT,
        a: *mut *const f32,
        lda: *const MKL_INT,
        ipiv: *mut *const MKL_INT,
        ainv: *mut *mut f32,
        ldainv: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetri_oop_batch_(
        n: *const MKL_INT,
        a: *mut *const f64,
        lda: *const MKL_INT,
        ipiv: *mut *const MKL_INT,
        ainv: *mut *mut f64,
        ldainv: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetri_oop_batch_(
        n: *const MKL_INT,
        a: *mut *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut *const MKL_INT,
        ainv: *mut *mut MKL_Complex8,
        ldainv: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetri_oop_batch_(
        n: *const MKL_INT,
        a: *mut *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut *const MKL_INT,
        ainv: *mut *mut MKL_Complex16,
        ldainv: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrfnp_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut f32,
        lda: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrfnp_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut f64,
        lda: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrfnp_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut MKL_Complex8,
        lda: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrfnp_batch_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut *mut MKL_Complex16,
        lda: *const MKL_INT,
        group_count: *const MKL_INT,
        group_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgels_batch_strided_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgels_batch_strided_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgels_batch_strided_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgels_batch_strided_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        stride_b: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvda_batch_strided_(
        iparm: *const MKL_INT,
        irank: *mut MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        s: *mut f32,
        stride_s: *const MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        stride_u: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        stride_vt: *const MKL_INT,
        tolerance: *const f32,
        residual: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvda_batch_strided_(
        iparm: *const MKL_INT,
        irank: *mut MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        s: *mut f64,
        stride_s: *const MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        stride_u: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        stride_vt: *const MKL_INT,
        tolerance: *const f64,
        residual: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesvda_batch_strided_(
        iparm: *const MKL_INT,
        irank: *mut MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        s: *mut f32,
        stride_s: *const MKL_INT,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        stride_u: *const MKL_INT,
        vt: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        stride_vt: *const MKL_INT,
        tolerance: *const f32,
        residual: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvda_batch_strided_(
        iparm: *const MKL_INT,
        irank: *mut MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        s: *mut f64,
        stride_s: *const MKL_INT,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        stride_u: *const MKL_INT,
        vt: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        stride_vt: *const MKL_INT,
        tolerance: *const f64,
        residual: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetri_batch_strided_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv_stride: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetri_batch_strided_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv_stride: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetri_batch_strided_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv_stride: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetri_batch_strided_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        stride_a: *const MKL_INT,
        ipiv: *mut MKL_INT,
        ipiv_stride: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        batch_size: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgejsv_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        jobr: *const c_char,
        jobt: *const c_char,
        jobp: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        sva: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        cwork: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgejsv_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        jobr: *const c_char,
        jobt: *const c_char,
        jobp: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        sva: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        cwork: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvdx_(
        jobu: *const c_char,
        jobvt: *const c_char,
        range: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvdx_(
        jobu: *const c_char,
        jobvt: *const c_char,
        range: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesvdx_(
        jobu: *const c_char,
        jobvt: *const c_char,
        range: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvdx_(
        jobu: *const c_char,
        jobvt: *const c_char,
        range: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesvj_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        cwork: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvj_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        cwork: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggsvd3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggsvd3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggsvd3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggsvd3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggsvp3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggsvp3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggsvp3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        rwork: *mut f32,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggsvp3_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        rwork: *mut f64,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgsvj0_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut MKL_Complex8,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        eps: *const f32,
        sfmin: *const f32,
        tol: *const f32,
        nsweep: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgsvj0_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut MKL_Complex16,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        eps: *const f64,
        sfmin: *const f64,
        tol: *const f64,
        nsweep: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgsvj1_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut MKL_Complex8,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        eps: *const f32,
        sfmin: *const f32,
        tol: *const f32,
        nsweep: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgsvj1_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut MKL_Complex16,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        eps: *const f64,
        sfmin: *const f64,
        tol: *const f64,
        nsweep: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpotrf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpotrf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spotrf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpotrf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dbdsvdx_(
        uplo: *const c_char,
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sbdsvdx_(
        uplo: *const c_char,
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        ns: *mut MKL_INT,
        s: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggev3_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zggev3_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dggev3_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggev3_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunm22_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        q: *const MKL_Complex8,
        ldq: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunm22_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        q: *const MKL_Complex16,
        ldq: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgghd3_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgghd3_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgghd3_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgghd3_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgges3_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_C_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vsl: *mut MKL_Complex8,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex8,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgges3_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_Z_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vsl: *mut MKL_Complex16,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex16,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgges3_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_D_SELECT_FUNCTION_3,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vsl: *mut f64,
        ldvsl: *const MKL_INT,
        vsr: *mut f64,
        ldvsr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgges3_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_S_SELECT_FUNCTION_3,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vsl: *mut f32,
        ldvsl: *const MKL_INT,
        vsr: *mut f32,
        ldvsr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorm22_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        q: *const f64,
        ldq: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorm22_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        q: *const f32,
        ldq: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn checon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chesv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chetrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clahef_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clasyf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csycon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csysv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csytrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlasyf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut f64,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsycon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsytrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slasyf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut f32,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssycon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssytrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhecon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhesv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhetrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlahef_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlasyf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsycon_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsysv_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytf2_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrf_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsytrs_rook_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb1_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut MKL_Complex8,
        taup2: *mut MKL_Complex8,
        tauq1: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb2_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut MKL_Complex8,
        taup2: *mut MKL_Complex8,
        tauq1: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb3_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut MKL_Complex8,
        taup2: *mut MKL_Complex8,
        tauq1: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb4_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut MKL_Complex8,
        taup2: *mut MKL_Complex8,
        tauq1: *mut MKL_Complex8,
        phantom: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb5_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut MKL_Complex8,
        incx1: *const MKL_INT,
        x2: *mut MKL_Complex8,
        incx2: *const MKL_INT,
        q1: *mut MKL_Complex8,
        ldq1: *const MKL_INT,
        q2: *mut MKL_Complex8,
        ldq2: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb6_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut MKL_Complex8,
        incx1: *const MKL_INT,
        x2: *mut MKL_Complex8,
        incx2: *const MKL_INT,
        q1: *mut MKL_Complex8,
        ldq1: *const MKL_INT,
        q2: *mut MKL_Complex8,
        ldq2: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cuncsd2by1_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        u1: *mut MKL_Complex8,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex8,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex8,
        ldv1t: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb1_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut f64,
        taup2: *mut f64,
        tauq1: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb2_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut f64,
        taup2: *mut f64,
        tauq1: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb3_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut f64,
        taup2: *mut f64,
        tauq1: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb4_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut f64,
        taup2: *mut f64,
        tauq1: *mut f64,
        phantom: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb5_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut f64,
        incx1: *const MKL_INT,
        x2: *mut f64,
        incx2: *const MKL_INT,
        q1: *mut f64,
        ldq1: *const MKL_INT,
        q2: *mut f64,
        ldq2: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb6_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut f64,
        incx1: *const MKL_INT,
        x2: *mut f64,
        incx2: *const MKL_INT,
        q1: *mut f64,
        ldq1: *const MKL_INT,
        q2: *mut f64,
        ldq2: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorcsd2by1_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        u1: *mut f64,
        ldu1: *const MKL_INT,
        u2: *mut f64,
        ldu2: *const MKL_INT,
        v1t: *mut f64,
        ldv1t: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb1_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut f32,
        taup2: *mut f32,
        tauq1: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb2_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut f32,
        taup2: *mut f32,
        tauq1: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb3_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut f32,
        taup2: *mut f32,
        tauq1: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb4_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut f32,
        taup2: *mut f32,
        tauq1: *mut f32,
        phantom: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb5_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut f32,
        incx1: *const MKL_INT,
        x2: *mut f32,
        incx2: *const MKL_INT,
        q1: *mut f32,
        ldq1: *const MKL_INT,
        q2: *mut f32,
        ldq2: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb6_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut f32,
        incx1: *const MKL_INT,
        x2: *mut f32,
        incx2: *const MKL_INT,
        q1: *mut f32,
        ldq1: *const MKL_INT,
        q2: *mut f32,
        ldq2: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorcsd2by1_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        theta: *mut f32,
        u1: *mut f32,
        ldu1: *const MKL_INT,
        u2: *mut f32,
        ldu2: *const MKL_INT,
        v1t: *mut f32,
        ldv1t: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb1_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut MKL_Complex16,
        taup2: *mut MKL_Complex16,
        tauq1: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb2_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut MKL_Complex16,
        taup2: *mut MKL_Complex16,
        tauq1: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb3_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut MKL_Complex16,
        taup2: *mut MKL_Complex16,
        tauq1: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb4_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut MKL_Complex16,
        taup2: *mut MKL_Complex16,
        tauq1: *mut MKL_Complex16,
        phantom: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb5_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut MKL_Complex16,
        incx1: *const MKL_INT,
        x2: *mut MKL_Complex16,
        incx2: *const MKL_INT,
        q1: *mut MKL_Complex16,
        ldq1: *const MKL_INT,
        q2: *mut MKL_Complex16,
        ldq2: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb6_(
        m1: *const MKL_INT,
        m2: *const MKL_INT,
        n: *const MKL_INT,
        x1: *mut MKL_Complex16,
        incx1: *const MKL_INT,
        x2: *mut MKL_Complex16,
        incx2: *const MKL_INT,
        q1: *mut MKL_Complex16,
        ldq1: *const MKL_INT,
        q2: *mut MKL_Complex16,
        ldq2: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zuncsd2by1_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        theta: *mut f64,
        u1: *mut MKL_Complex16,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex16,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex16,
        ldv1t: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgemqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dgemqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn sgemqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zgemqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cgeqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dgeqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn sgeqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zgeqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cgeqrt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeqrt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeqrt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeqrt3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctpmqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dtpmqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn stpmqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ztpmqrt_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ctpqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dtpqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn stpqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ztpqrt_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ctpqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtpqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stpqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztpqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cbbcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        u1: *mut MKL_Complex8,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex8,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex8,
        ldv1t: *const MKL_INT,
        v2t: *mut MKL_Complex8,
        ldv2t: *const MKL_INT,
        b11d: *mut f32,
        b11e: *mut f32,
        b12d: *mut f32,
        b12e: *mut f32,
        b21d: *mut f32,
        b21e: *mut f32,
        b22d: *mut f32,
        b22e: *mut f32,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zbbcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        u1: *mut MKL_Complex16,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex16,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex16,
        ldv1t: *const MKL_INT,
        v2t: *mut MKL_Complex16,
        ldv2t: *const MKL_INT,
        b11d: *mut f64,
        b11e: *mut f64,
        b12d: *mut f64,
        b12e: *mut f64,
        b21d: *mut f64,
        b21e: *mut f64,
        b22d: *mut f64,
        b22e: *mut f64,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sbbcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        u1: *mut f32,
        ldu1: *const MKL_INT,
        u2: *mut f32,
        ldu2: *const MKL_INT,
        v1t: *mut f32,
        ldv1t: *const MKL_INT,
        v2t: *mut f32,
        ldv2t: *const MKL_INT,
        b11d: *mut f32,
        b11e: *mut f32,
        b12d: *mut f32,
        b12e: *mut f32,
        b21d: *mut f32,
        b21e: *mut f32,
        b22d: *mut f32,
        b22e: *mut f32,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dbbcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        u1: *mut f64,
        ldu1: *const MKL_INT,
        u2: *mut f64,
        ldu2: *const MKL_INT,
        v1t: *mut f64,
        ldv1t: *const MKL_INT,
        v2t: *mut f64,
        ldv2t: *const MKL_INT,
        b11d: *mut f64,
        b11e: *mut f64,
        b12d: *mut f64,
        b12e: *mut f64,
        b21d: *mut f64,
        b21e: *mut f64,
        b22d: *mut f64,
        b22e: *mut f64,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunbdb_(
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x12: *mut MKL_Complex8,
        ldx12: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        x22: *mut MKL_Complex8,
        ldx22: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut MKL_Complex8,
        taup2: *mut MKL_Complex8,
        tauq1: *mut MKL_Complex8,
        tauq2: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunbdb_(
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x12: *mut MKL_Complex16,
        ldx12: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        x22: *mut MKL_Complex16,
        ldx22: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut MKL_Complex16,
        taup2: *mut MKL_Complex16,
        tauq1: *mut MKL_Complex16,
        tauq2: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorbdb_(
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x12: *mut f32,
        ldx12: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        x22: *mut f32,
        ldx22: *const MKL_INT,
        theta: *mut f32,
        phi: *mut f32,
        taup1: *mut f32,
        taup2: *mut f32,
        tauq1: *mut f32,
        tauq2: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorbdb_(
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x12: *mut f64,
        ldx12: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        x22: *mut f64,
        ldx22: *const MKL_INT,
        theta: *mut f64,
        phi: *mut f64,
        taup1: *mut f64,
        taup2: *mut f64,
        tauq1: *mut f64,
        tauq2: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cuncsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut MKL_Complex8,
        ldx11: *const MKL_INT,
        x12: *mut MKL_Complex8,
        ldx12: *const MKL_INT,
        x21: *mut MKL_Complex8,
        ldx21: *const MKL_INT,
        x22: *mut MKL_Complex8,
        ldx22: *const MKL_INT,
        theta: *mut f32,
        u1: *mut MKL_Complex8,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex8,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex8,
        ldv1t: *const MKL_INT,
        v2t: *mut MKL_Complex8,
        ldv2t: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zuncsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut MKL_Complex16,
        ldx11: *const MKL_INT,
        x12: *mut MKL_Complex16,
        ldx12: *const MKL_INT,
        x21: *mut MKL_Complex16,
        ldx21: *const MKL_INT,
        x22: *mut MKL_Complex16,
        ldx22: *const MKL_INT,
        theta: *mut f64,
        u1: *mut MKL_Complex16,
        ldu1: *const MKL_INT,
        u2: *mut MKL_Complex16,
        ldu2: *const MKL_INT,
        v1t: *mut MKL_Complex16,
        ldv1t: *const MKL_INT,
        v2t: *mut MKL_Complex16,
        ldv2t: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut f32,
        ldx11: *const MKL_INT,
        x12: *mut f32,
        ldx12: *const MKL_INT,
        x21: *mut f32,
        ldx21: *const MKL_INT,
        x22: *mut f32,
        ldx22: *const MKL_INT,
        theta: *mut f32,
        u1: *mut f32,
        ldu1: *const MKL_INT,
        u2: *mut f32,
        ldu2: *const MKL_INT,
        v1t: *mut f32,
        ldv1t: *const MKL_INT,
        v2t: *mut f32,
        ldv2t: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorcsd_(
        jobu1: *const c_char,
        jobu2: *const c_char,
        jobv1t: *const c_char,
        jobv2t: *const c_char,
        trans: *const c_char,
        signs: *const c_char,
        M: *const MKL_INT,
        P: *const MKL_INT,
        Q: *const MKL_INT,
        x11: *mut f64,
        ldx11: *const MKL_INT,
        x12: *mut f64,
        ldx12: *const MKL_INT,
        x21: *mut f64,
        ldx21: *const MKL_INT,
        x22: *mut f64,
        ldx22: *const MKL_INT,
        theta: *mut f64,
        u1: *mut f64,
        ldu1: *const MKL_INT,
        u2: *mut f64,
        ldu2: *const MKL_INT,
        v1t: *mut f64,
        ldv1t: *const MKL_INT,
        v2t: *mut f64,
        ldv2t: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clapmr_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn dlapmr_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn zlapmr_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn slapmr_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn csyconv_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        e: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zsyconv_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        e: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ssyconv_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        e: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn dsyconv_(
        uplo: *const c_char,
        way: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        e: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn csyswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn zsyswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn ssyswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn dsyswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn csytri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slartgp_(f: *const f32, g: *const f32, cs: *mut f32, sn: *mut f32, r: *mut f32);
    pub fn dlartgp_(f: *const f64, g: *const f64, cs: *mut f64, sn: *mut f64, r: *mut f64);
    pub fn slartgs_(x: *const f32, y: *const f32, sigma: *const f32, cs: *mut f32, sn: *mut f32);
    pub fn dlartgs_(x: *const f64, y: *const f64, sigma: *const f64, cs: *mut f64, sn: *mut f64);
    pub fn csytrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zsytrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ssytrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn dsytrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn chetrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zhetrs2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cbdsqr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vt: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbbrd_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ncc: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        pt: *mut MKL_Complex8,
        ldpt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbrfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbsv_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgbsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbsvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgbtf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgbtrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgbtrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgebak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        scale: *const f32,
        m: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgebal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgebd2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut MKL_Complex8,
        taup: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgebrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut MKL_Complex8,
        taup: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgecon_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgedmd_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f32,
        k: *mut MKL_INT,
        eigs: *mut MKL_Complex8,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        res: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        s: *mut MKL_Complex8,
        lds: *const MKL_INT,
        zwork: *mut MKL_Complex8,
        lzwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgedmdq_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobq: *const c_char,
        jobt: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        f: *mut MKL_Complex8,
        ldf: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f32,
        k: *mut MKL_INT,
        eigs: *mut MKL_Complex8,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        res: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        s: *mut MKL_Complex8,
        lds: *const MKL_INT,
        zwork: *mut MKL_Complex8,
        lzwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgeequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgees_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_C_SELECT_FUNCTION_1,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        w: *mut MKL_Complex8,
        vs: *mut MKL_Complex8,
        ldvs: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeesx_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_C_SELECT_FUNCTION_1,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        w: *mut MKL_Complex8,
        vs: *mut MKL_Complex8,
        ldvs: *const MKL_INT,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgeevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f32,
        abnrm: *mut f32,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgegs_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vsl: *mut MKL_Complex8,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex8,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgegv_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgehd2_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgehrd_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgelqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelsd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        s: *mut f32,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgels_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelst_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgelss_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        s: *mut f32,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgelsx_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgelsy_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgeql2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgeqlf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeqp3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgeqp3rk_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        kmax: *const MKL_INT,
        abstol: *const f32,
        reltol: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        k: *mut MKL_INT,
        maxc2nrmk: *mut f32,
        relmaxc2nrmk: *mut f32,
        jpiv: *mut MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeqpf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgeqr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgeqr2p_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgeqrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgeqrfp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgerfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgerfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgerq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgerqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesc2_(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        rhs: *mut MKL_Complex8,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
        scale: *mut f32,
    );
    pub fn cgesdd_(
        jobz: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesvd_(
        jobu: *const c_char,
        jobvt: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex8,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgesvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgesvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgetc2_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        jpiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_cgetrfnpi_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nfact: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetri_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgetrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggbak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        lscale: *const f32,
        rscale: *const f32,
        m: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggbal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f32,
        rscale: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgges_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_C_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vsl: *mut MKL_Complex8,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex8,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggesx_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_C_SELECT_FUNCTION_2,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vsl: *mut MKL_Complex8,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex8,
        ldvsr: *const MKL_INT,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cggevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f32,
        rscale: *mut f32,
        abnrm: *mut f32,
        bbnrm: *mut f32,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggglm_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        d: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        y: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgghrd_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgglse_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggqrf_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        taua: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        taub: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggrqf_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        taua: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        taub: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggsvd_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cggsvp_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        rwork: *mut f32,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgtcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        du2: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cgtrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        dlf: *const MKL_Complex8,
        df: *const MKL_Complex8,
        duf: *const MKL_Complex8,
        du2: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgtsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
        du: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgtsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        dlf: *mut MKL_Complex8,
        df: *mut MKL_Complex8,
        duf: *mut MKL_Complex8,
        du2: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cgttrf_(
        n: *const MKL_INT,
        dl: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
        du: *mut MKL_Complex8,
        du2: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgttrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        du2: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cgtts2_(
        itrans: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        du2: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn chbevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chbev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chbevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chbgst_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        bb: *const MKL_Complex8,
        ldbb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chbgvd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex8,
        ldbb: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chbgv_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex8,
        ldbb: *const MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chbgvx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex8,
        ldbb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chbtrd_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn checon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cheequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cheevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cheevr_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chegs2_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chegst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chegvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chegv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chegvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cherfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cherfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chesv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chesvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chesvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chetd2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chetf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chetrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chfrk_(
        transr: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f32,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        beta: *const f32,
        c: *mut MKL_Complex8,
    );
    pub fn chgeqz_(
        job: *const c_char,
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chla_transtype_(ret_val: *mut c_char, ret_val_len: c_int, trans: *const MKL_INT);
    pub fn chpcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chpevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chpev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chpevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chpgst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        bp: *const MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chpgvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        bp: *mut MKL_Complex8,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chpgv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        bp: *mut MKL_Complex8,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chpgvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        bp: *mut MKL_Complex8,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        afp: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chpsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chpsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        afp: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn chptrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        d: *mut f32,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn chptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chsein_(
        side: *const c_char,
        eigsrc: *const c_char,
        initv: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        h: *const MKL_Complex8,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex8,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        ifaill: *mut MKL_INT,
        ifailr: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chseqr_(
        job: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex8,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clabrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut MKL_Complex8,
        taup: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
    );
    pub fn clacgv_(n: *const MKL_INT, x: *mut MKL_Complex8, incx: *const MKL_INT);
    pub fn clacn2_(
        n: *const MKL_INT,
        v: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        est: *mut f32,
        kase: *mut MKL_INT,
        isave: *mut MKL_INT,
    );
    pub fn clacon_(
        n: *const MKL_INT,
        v: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        est: *mut f32,
        kase: *mut MKL_INT,
    );
    pub fn clacp2_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn clacpy_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn clacrm_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        rwork: *mut f32,
    );
    pub fn clacrt_(
        n: *const MKL_INT,
        cx: *mut MKL_Complex8,
        incx: *const MKL_INT,
        cy: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const MKL_Complex8,
        s: *const MKL_Complex8,
    );
    pub fn cladiv_(ret_value: *mut MKL_Complex8, x: *const MKL_Complex8, y: *const MKL_Complex8);
    pub fn claed0_(
        qsiz: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        qstore: *mut MKL_Complex8,
        ldqs: *const MKL_INT,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claed7_(
        n: *const MKL_INT,
        cutpnt: *const MKL_INT,
        qsiz: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        d: *mut f32,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        rho: *const f32,
        indxq: *mut MKL_INT,
        qstore: *mut f32,
        qptr: *mut MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claed8_(
        k: *mut MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        d: *mut f32,
        rho: *mut f32,
        cutpnt: *const MKL_INT,
        z: *const f32,
        dlamda: *mut f32,
        q2: *mut MKL_Complex8,
        ldq2: *const MKL_INT,
        w: *mut f32,
        indxp: *mut MKL_INT,
        indx: *mut MKL_INT,
        indxq: *const MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        givnum: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn claein_(
        rightv: *const MKL_INT,
        noinit: *const MKL_INT,
        n: *const MKL_INT,
        h: *const MKL_Complex8,
        ldh: *const MKL_INT,
        w: *const MKL_Complex8,
        v: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        rwork: *mut f32,
        eps3: *const f32,
        smlnum: *const f32,
        info: *mut MKL_INT,
    );
    pub fn claesy_(
        a: *const MKL_Complex8,
        b: *const MKL_Complex8,
        c: *const MKL_Complex8,
        rt1: *mut MKL_Complex8,
        rt2: *mut MKL_Complex8,
        evscal: *mut MKL_Complex8,
        cs1: *mut MKL_Complex8,
        sn1: *mut MKL_Complex8,
    );
    pub fn claev2_(
        a: *const MKL_Complex8,
        b: *const MKL_Complex8,
        c: *const MKL_Complex8,
        rt1: *mut f32,
        rt2: *mut f32,
        cs1: *mut f32,
        sn1: *mut MKL_Complex8,
    );
    pub fn clag2z_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        sa: *const MKL_Complex8,
        ldsa: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clags2_(
        upper: *const MKL_INT,
        a1: *const f32,
        a2: *const MKL_Complex8,
        a3: *const f32,
        b1: *const f32,
        b2: *const MKL_Complex8,
        b3: *const f32,
        csu: *mut f32,
        snu: *mut MKL_Complex8,
        csv: *mut f32,
        snv: *mut MKL_Complex8,
        csq: *mut f32,
        snq: *mut MKL_Complex8,
    );
    pub fn clagtm_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        alpha: *const f32,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        x: *const MKL_Complex8,
        ldx: *const MKL_INT,
        beta: *const f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn clahef_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clahqr_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex8,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clahr2_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
    );
    pub fn clahrd_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
    );
    pub fn claic1_(
        job: *const MKL_INT,
        j: *const MKL_INT,
        x: *const MKL_Complex8,
        sest: *const f32,
        w: *const MKL_Complex8,
        gamma: *const MKL_Complex8,
        sestpr: *mut f32,
        s: *mut MKL_Complex8,
        c: *mut MKL_Complex8,
    );
    pub fn clals0_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        bx: *mut MKL_Complex8,
        ldbx: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *const f32,
        ldgnum: *const MKL_INT,
        poles: *const f32,
        difl: *const f32,
        difr: *const f32,
        z: *const f32,
        k: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clalsa_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        bx: *mut MKL_Complex8,
        ldbx: *const MKL_INT,
        u: *const f32,
        ldu: *const MKL_INT,
        vt: *const f32,
        k: *const MKL_INT,
        difl: *const f32,
        difr: *const f32,
        z: *const f32,
        poles: *const f32,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *const MKL_INT,
        givnum: *const f32,
        c: *const f32,
        s: *const f32,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clalsd_(
        uplo: *const c_char,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clangb_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clange_(
        norm: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clangt_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
    ) -> f32;
    pub fn clanhb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clanhe_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clanhf_(
        norm: *const c_char,
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        work: *mut f32,
    ) -> f32;
    pub fn clanhp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        work: *mut f32,
    ) -> f32;
    pub fn clanhs_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clanht_(
        norm: *const c_char,
        n: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
    ) -> f32;
    pub fn clansb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clansp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        work: *mut f32,
    ) -> f32;
    pub fn clansy_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clantb_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clantp_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        work: *mut f32,
    ) -> f32;
    pub fn clantr_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn clapll_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        ssmin: *mut f32,
    );
    pub fn clapmt_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn claqgb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        rowcnd: *const f32,
        colcnd: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        rowcnd: *const f32,
        colcnd: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqhb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        s: *mut f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqhe_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqhp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex8,
        vn1: *mut f32,
        vn2: *mut f32,
        work: *mut MKL_Complex8,
    );
    pub fn claqps_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex8,
        vn1: *mut f32,
        vn2: *mut f32,
        auxv: *mut MKL_Complex8,
        f: *mut MKL_Complex8,
        ldf: *const MKL_INT,
    );
    pub fn claqr0_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex8,
        iloz: *mut MKL_INT,
        ihiz: *mut MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claqr1_(
        n: *const MKL_INT,
        h: *const MKL_Complex8,
        ldh: *const MKL_INT,
        s1: *const MKL_Complex8,
        s2: *mut MKL_Complex8,
        v: *mut MKL_Complex8,
    );
    pub fn claqr2_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sh: *mut MKL_Complex8,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex8,
        ldwv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
    );
    pub fn claqr3_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sh: *mut MKL_Complex8,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex8,
        ldwv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
    );
    pub fn claqr4_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex8,
        iloz: *mut MKL_INT,
        ihiz: *mut MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claqr5_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        kacc22: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nshfts: *const MKL_INT,
        s: *mut MKL_Complex8,
        h: *mut MKL_Complex8,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex8,
        ldwv: *const MKL_INT,
        nh: *const MKL_INT,
        wh: *mut MKL_Complex8,
        ldwh: *const MKL_INT,
    );
    pub fn claqsb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqsp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqsy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn claqz0_(
        wants: *const c_char,
        wantq: *const c_char,
        wantz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claqz1_(
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        k: *const MKL_INT,
        istartm: *const MKL_INT,
        istopm: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        nq: *const MKL_INT,
        qstart: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        nz: *const MKL_INT,
        zstart: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
    );
    pub fn claqz2_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nw: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        qc: *mut MKL_Complex8,
        ldqc: *const MKL_INT,
        zc: *mut MKL_Complex8,
        ldzc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn claqz3_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nshifts: *const MKL_INT,
        nb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        qc: *mut MKL_Complex8,
        ldqc: *const MKL_INT,
        zc: *mut MKL_Complex8,
        ldzc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clar1v_(
        n: *const MKL_INT,
        b1: *const MKL_INT,
        bn: *const MKL_INT,
        lambda: *const f32,
        d: *const f32,
        l: *const f32,
        ld: *const f32,
        lld: *const f32,
        pivmin: *const f32,
        gaptol: *const f32,
        z: *mut MKL_Complex8,
        wantnc: *const MKL_INT,
        negcnt: *mut MKL_INT,
        ztz: *mut f32,
        mingma: *mut f32,
        r: *mut MKL_INT,
        isuppz: *mut MKL_INT,
        nrminv: *mut f32,
        resid: *mut f32,
        rqcorr: *mut f32,
        work: *mut f32,
    );
    pub fn clar2v_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        y: *mut MKL_Complex8,
        z: *mut MKL_Complex8,
        incx: *const MKL_INT,
        c: *const f32,
        s: *const MKL_Complex8,
        incc: *const MKL_INT,
    );
    pub fn clarcm_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        rwork: *mut f32,
    );
    pub fn clarfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        ldwork: *const MKL_INT,
    );
    pub fn clarf_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clarf1f_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clarf1l_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clarfg_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex8,
    );
    pub fn clarfgp_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex8,
    );
    pub fn clarfp_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex8,
    );
    pub fn clarft_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        tau: *const MKL_Complex8,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
    );
    pub fn clarfx_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clargv_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *mut f32,
        incc: *const MKL_INT,
    );
    pub fn clarnv_(
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
    );
    pub fn clarrv_(
        n: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        d: *mut f32,
        l: *mut f32,
        pivmin: *mut f32,
        isplit: *const MKL_INT,
        m: *const MKL_INT,
        dol: *const MKL_INT,
        dou: *const MKL_INT,
        minrgp: *const f32,
        rtol1: *const f32,
        rtol2: *const f32,
        w: *mut f32,
        werr: *mut f32,
        wgap: *mut f32,
        iblock: *const MKL_INT,
        indexw: *const MKL_INT,
        gers: *const f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clarscl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f32,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
    );
    pub fn clartg_(
        f: *const MKL_Complex8,
        g: *const MKL_Complex8,
        cs: *mut f32,
        sn: *mut MKL_Complex8,
        r: *mut MKL_Complex8,
    );
    pub fn clartv_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        incx: *const MKL_INT,
        y: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const MKL_Complex8,
        incc: *const MKL_INT,
    );
    pub fn clarzb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        ldwork: *const MKL_INT,
    );
    pub fn clarz_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clarzt_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        tau: *const MKL_Complex8,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
    );
    pub fn clascl_(
        type_: *const c_char,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        cfrom: *const f32,
        cto: *const f32,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clascl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f32,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
    );
    pub fn claset_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn clasr_(
        side: *const c_char,
        pivot: *const c_char,
        direct: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn classq_(
        n: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        scale: *mut f32,
        sumsq: *mut f32,
    );
    pub fn claswp_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        k1: *const MKL_INT,
        k2: *const MKL_INT,
        ipiv: *const MKL_INT,
        incx: *const MKL_INT,
    );
    pub fn clasyf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clatbs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        x: *mut MKL_Complex8,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clatdf_(
        ijob: *const MKL_INT,
        n: *const MKL_INT,
        z: *const MKL_Complex8,
        ldz: *const MKL_INT,
        rhs: *mut MKL_Complex8,
        rdsum: *mut f32,
        rdscal: *mut f32,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
    );
    pub fn clatps_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clatrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        w: *mut MKL_Complex8,
        ldw: *const MKL_INT,
    );
    pub fn clatrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clatrs3_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        scale: *mut f32,
        cnorm: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clatrz_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
    );
    pub fn clatzm_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex8,
        incv: *const MKL_INT,
        tau: *const MKL_Complex8,
        c1: *mut MKL_Complex8,
        c2: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
    );
    pub fn clauu2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clauum_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpbcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpbequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpbrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpbstf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpbsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpbsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex8,
        ldafb: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpbtf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpbtrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex8,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpbtrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpftrf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cpftri_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cpftrs_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpocon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpoequb_(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpoequ_(
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cporfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cporfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        s: *mut f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cposvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cposvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpotf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpotrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpotri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpotrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cppcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cppequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        afp: *const MKL_Complex8,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cppsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cppsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex8,
        afp: *mut MKL_Complex8,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cpptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cpptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cpstf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpstrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cptcon_(
        n: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
        anorm: *const f32,
        rcond: *mut f32,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cptrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
        df: *const f32,
        ef: *const MKL_Complex8,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cptsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f32,
        e: *mut MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cptsvx_(
        fact: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
        df: *mut f32,
        ef: *mut MKL_Complex8,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cpttrf_(n: *const MKL_INT, d: *mut f32, e: *mut MKL_Complex8, info: *mut MKL_INT);
    pub fn cpttrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cptts2_(
        iuplo: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn crot_(
        n: *const MKL_INT,
        cx: *mut MKL_Complex8,
        incx: *const MKL_INT,
        cy: *mut MKL_Complex8,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const MKL_Complex8,
    );
    pub fn cspcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cspmv_(
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
    pub fn cspr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex8,
    );
    pub fn csprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        afp: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn cspsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cspsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        afp: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csrscl_(n: *const MKL_INT, sa: *const f32, sx: *mut MKL_Complex8, incx: *const MKL_INT);
    pub fn cstedc_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cstegr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cstein_(
        n: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        m: *const MKL_INT,
        w: *const f32,
        iblock: *const MKL_INT,
        isplit: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cstemr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        nzc: *const MKL_INT,
        isuppz: *mut MKL_INT,
        tryrac: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csycon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csyequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csymv_(
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
    pub fn csyr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
    );
    pub fn csyrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csyrfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csysv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csysvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csysvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        af: *mut MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn csytf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn csytri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn csytrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctbcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctbrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *const MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctbtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctfsm_(
        transr: *const c_char,
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex8,
        a: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
    );
    pub fn ctftri_(
        transr: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctfttp_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const MKL_Complex8,
        ap: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctfttr_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const MKL_Complex8,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        s: *const MKL_Complex8,
        lds: *const MKL_INT,
        p: *const MKL_Complex8,
        ldp: *const MKL_INT,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctgex2_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        j1: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgexc_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        ifst: *const MKL_INT,
        ilst: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgsen_(
        ijob: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex8,
        beta: *mut MKL_Complex8,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        m: *mut MKL_INT,
        pl: *mut f32,
        pr: *mut f32,
        dif: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgsja_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut MKL_Complex8,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex8,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex8,
        ncycle: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        vl: *const MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *const MKL_Complex8,
        ldvr: *const MKL_INT,
        s: *mut f32,
        dif: *mut f32,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctgsy2_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        d: *const MKL_Complex8,
        ldd: *const MKL_INT,
        e: *const MKL_Complex8,
        lde: *const MKL_INT,
        f: *mut MKL_Complex8,
        ldf: *const MKL_INT,
        scale: *mut f32,
        rdsum: *mut f32,
        rdscal: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctgsyl_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        d: *const MKL_Complex8,
        ldd: *const MKL_INT,
        e: *const MKL_Complex8,
        lde: *const MKL_INT,
        f: *mut MKL_Complex8,
        ldf: *const MKL_INT,
        scale: *mut f32,
        dif: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctpcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctprfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *const MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctptri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctptrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctpttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        arf: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctpttr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        rcond: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctrevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        vl: *mut MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex8,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctrexc_(
        compq: *const c_char,
        n: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        ifst: *const MKL_INT,
        ilst: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        x: *const MKL_Complex8,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctrsen_(
        job: *const c_char,
        compq: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        w: *mut MKL_Complex8,
        m: *mut MKL_INT,
        s: *mut f32,
        sep: *mut f32,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        vl: *const MKL_Complex8,
        ldvl: *const MKL_INT,
        vr: *const MKL_Complex8,
        ldvr: *const MKL_INT,
        s: *mut f32,
        sep: *mut f32,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex8,
        ldwork: *const MKL_INT,
        rwork: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctrsyl_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        scale: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ctrsyl3_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        scale: *mut f32,
        swork: *mut f32,
        ldswork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrti2_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrtri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ctrttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        arf: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctrttp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        ap: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctzrqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn ctzrzf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cung2l_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cung2r_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cungbr_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunghr_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunglq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungql_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cungrq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cungtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunm2l_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunm2r_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunmbr_(
        vect: *const c_char,
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmhr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunml2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunmlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmql_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmr2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunmr3_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cunmrq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmrz_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cunmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cupgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        tau: *const MKL_Complex8,
        q: *mut MKL_Complex8,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cupmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        tau: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dbdsdc_(
        uplo: *const c_char,
        compq: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        q: *mut f64,
        iq: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dbdsqr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ddisna_(
        job: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f64,
        sep: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgbbrd_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ncc: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        pt: *mut f64,
        ldpt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgbcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgbequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgbrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbrfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbsv_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        afb: *mut f64,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbsvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        afb: *mut f64,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbtf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbtrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgbtrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgebak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        scale: *const f64,
        m: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgebal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgebd2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut f64,
        taup: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgebrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut f64,
        taup: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgecon_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgedmd_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f64,
        k: *mut MKL_INT,
        reig: *mut f64,
        imeig: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        res: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        w: *mut f64,
        ldw: *const MKL_INT,
        s: *mut f64,
        lds: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgedmdq_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobq: *const c_char,
        jobt: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        f: *mut f64,
        ldf: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f64,
        k: *mut MKL_INT,
        reigs: *mut f64,
        imeig: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        res: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        s: *mut f64,
        lds: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgees_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_D_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        vs: *mut f64,
        ldvs: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeesx_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_D_SELECT_FUNCTION_2,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        vs: *mut f64,
        ldvs: *const MKL_INT,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f64,
        abnrm: *mut f64,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgegs_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vsl: *mut f64,
        ldvsl: *const MKL_INT,
        vsr: *mut f64,
        ldvsr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgegv_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgehd2_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgehrd_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgejsv_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        jobr: *const c_char,
        jobt: *const c_char,
        jobp: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        sva: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgelqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelsd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        s: *mut f64,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgels_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelst_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelss_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        s: *mut f64,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgelsx_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgelsy_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeql2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeqlf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeqp3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeqp3rk_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        kmax: *const MKL_INT,
        abstol: *const f64,
        reltol: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        k: *mut MKL_INT,
        maxc2nrmk: *mut f64,
        relmaxc2nrmk: *mut f64,
        jpiv: *mut MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeqpf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeqr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeqr2p_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgeqrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgeqrfp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgerfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgerfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgerq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgerqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesc2_(
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        rhs: *mut f64,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
        scale: *mut f64,
    );
    pub fn dgesdd_(
        jobz: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvd_(
        jobu: *const c_char,
        jobvt: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvj_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgesvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetc2_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        jpiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_dgetrfnpi_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nfact: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetri_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgetrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggbak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        lscale: *const f64,
        rscale: *const f64,
        m: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggbal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f64,
        rscale: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgges_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_D_SELECT_FUNCTION_3,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vsl: *mut f64,
        ldvsl: *const MKL_INT,
        vsr: *mut f64,
        ldvsr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggesx_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_D_SELECT_FUNCTION_3,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vsl: *mut f64,
        ldvsl: *const MKL_INT,
        vsr: *mut f64,
        ldvsr: *const MKL_INT,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f64,
        rscale: *mut f64,
        abnrm: *mut f64,
        bbnrm: *mut f64,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggglm_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        d: *mut f64,
        x: *mut f64,
        y: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgghrd_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgglse_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        d: *mut f64,
        x: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggqrf_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        taua: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        taub: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggrqf_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        taua: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        taub: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggsvd_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dggsvp_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dgsvj0_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        eps: *const f64,
        sfmin: *const f64,
        tol: *const f64,
        nsweep: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgsvj1_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        sva: *mut f64,
        mv: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        eps: *const f64,
        sfmin: *const f64,
        tol: *const f64,
        nsweep: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgtcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        du2: *const f64,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgtrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        dlf: *const f64,
        df: *const f64,
        duf: *const f64,
        du2: *const f64,
        ipiv: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgtsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgtsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        dlf: *mut f64,
        df: *mut f64,
        duf: *mut f64,
        du2: *mut f64,
        ipiv: *mut MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgttrf_(
        n: *const MKL_INT,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        du2: *mut f64,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgttrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        du2: *const f64,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dgtts2_(
        itrans: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        du2: *const f64,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
    );
    pub fn dhgeqz_(
        job: *const c_char,
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dhsein_(
        side: *const c_char,
        eigsrc: *const c_char,
        initv: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        h: *const f64,
        ldh: *const MKL_INT,
        wr: *mut f64,
        wi: *const f64,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        ifaill: *mut MKL_INT,
        ifailr: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dhseqr_(
        job: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn disnan_(din: *const f64) -> MKL_INT;
    pub fn dlabad_(smallx: *mut f64, large: *mut f64);
    pub fn dlabrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut f64,
        taup: *mut f64,
        x: *mut f64,
        ldx: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
    );
    pub fn dlacn2_(
        n: *const MKL_INT,
        v: *mut f64,
        x: *mut f64,
        isgn: *mut MKL_INT,
        est: *mut f64,
        kase: *mut MKL_INT,
        isave: *mut MKL_INT,
    );
    pub fn dlacon_(
        n: *const MKL_INT,
        v: *mut f64,
        x: *mut f64,
        isgn: *mut MKL_INT,
        est: *mut f64,
        kase: *mut MKL_INT,
    );
    pub fn dlacpy_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
    );
    pub fn dladiv_(
        a: *const f64,
        b: *const f64,
        c: *const f64,
        d: *const f64,
        p: *mut f64,
        q: *mut f64,
    );
    pub fn dlae2_(a: *const f64, b: *const f64, c: *const f64, rt1: *mut f64, rt2: *mut f64);
    pub fn dlaebz_(
        ijob: *const MKL_INT,
        nitmax: *const MKL_INT,
        n: *const MKL_INT,
        mmax: *const MKL_INT,
        minp: *const MKL_INT,
        nbmin: *const MKL_INT,
        abstol: *const f64,
        reltol: *const f64,
        pivmin: *const f64,
        d: *const f64,
        e: *const f64,
        e2: *const f64,
        nval: *mut MKL_INT,
        ab: *mut f64,
        c: *mut f64,
        mout: *mut MKL_INT,
        nab: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed0_(
        icompq: *const MKL_INT,
        qsiz: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f64,
        e: *const f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        qstore: *mut f64,
        ldqs: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed1_(
        n: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *const f64,
        cutpnt: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed2_(
        k: *mut MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *mut f64,
        z: *const f64,
        dlamda: *mut f64,
        w: *mut f64,
        q2: *mut f64,
        indx: *mut MKL_INT,
        indxc: *mut MKL_INT,
        indxp: *mut MKL_INT,
        coltyp: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed3_(
        k: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        rho: *const f64,
        dlamda: *mut f64,
        q2: *const f64,
        indx: *const MKL_INT,
        ctot: *const MKL_INT,
        w: *mut f64,
        s: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaed4_(
        n: *const MKL_INT,
        i: *const MKL_INT,
        d: *const f64,
        z: *const f64,
        delta: *mut f64,
        rho: *const f64,
        dlam: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaed5_(
        i: *const MKL_INT,
        d: *const f64,
        z: *const f64,
        delta: *mut f64,
        rho: *const f64,
        dlam: *mut f64,
    );
    pub fn dlaed6_(
        kniter: *const MKL_INT,
        orgati: *const MKL_INT,
        rho: *const f64,
        d: *const f64,
        z: *const f64,
        finit: *const f64,
        tau: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaed7_(
        icompq: *const MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *const f64,
        cutpnt: *const MKL_INT,
        qstore: *mut f64,
        qptr: *mut MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed8_(
        icompq: *const MKL_INT,
        k: *mut MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        indxq: *const MKL_INT,
        rho: *mut f64,
        cutpnt: *const MKL_INT,
        z: *const f64,
        dlamda: *mut f64,
        q2: *mut f64,
        ldq2: *const MKL_INT,
        w: *mut f64,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        givnum: *mut f64,
        indxp: *mut MKL_INT,
        indx: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaed9_(
        k: *const MKL_INT,
        kstart: *const MKL_INT,
        kstop: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        rho: *const f64,
        dlamda: *const f64,
        w: *const f64,
        s: *mut f64,
        lds: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaeda_(
        n: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f64,
        q: *const f64,
        qptr: *const MKL_INT,
        z: *mut f64,
        ztemp: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaein_(
        rightv: *const MKL_INT,
        noinit: *const MKL_INT,
        n: *const MKL_INT,
        h: *const f64,
        ldh: *const MKL_INT,
        wr: *const f64,
        wi: *const f64,
        vr: *mut f64,
        vi: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        eps3: *const f64,
        smlnum: *const f64,
        bignum: *const f64,
        info: *mut MKL_INT,
    );
    pub fn dlaev2_(
        a: *const f64,
        b: *const f64,
        c: *const f64,
        rt1: *mut f64,
        rt2: *mut f64,
        cs1: *mut f64,
        sn1: *mut f64,
    );
    pub fn dlaexc_(
        wantq: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        j1: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlag2_(
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        safmin: *const f64,
        scale1: *mut f64,
        scale2: *mut f64,
        wr1: *mut f64,
        wr2: *mut f64,
        wi: *mut f64,
    );
    pub fn dlag2s_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        sa: *mut f32,
        ldsa: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlags2_(
        upper: *const MKL_INT,
        a1: *const f64,
        a2: *const f64,
        a3: *const f64,
        b1: *const f64,
        b2: *const f64,
        b3: *const f64,
        csu: *mut f64,
        snu: *mut f64,
        csv: *mut f64,
        snv: *mut f64,
        csq: *mut f64,
        snq: *mut f64,
    );
    pub fn dlagtf_(
        n: *const MKL_INT,
        a: *mut f64,
        lambda: *const f64,
        b: *mut f64,
        c: *mut f64,
        tol: *const f64,
        d: *mut f64,
        in_: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlagtm_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        alpha: *const f64,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *const f64,
        ldx: *const MKL_INT,
        beta: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
    );
    pub fn dlagts_(
        job: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        b: *const f64,
        c: *const f64,
        d: *const f64,
        in_: *const MKL_INT,
        y: *mut f64,
        tol: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlagv2_(
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        csl: *mut f64,
        snl: *mut f64,
        csr: *mut f64,
        snr: *mut f64,
    );
    pub fn dlahqr_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlahr2_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        t: *mut f64,
        ldt: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
    );
    pub fn dlahrd_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        t: *mut f64,
        ldt: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
    );
    pub fn dlaic1_(
        job: *const MKL_INT,
        j: *const MKL_INT,
        x: *const f64,
        sest: *const f64,
        w: *const f64,
        gamma: *const f64,
        sestpr: *mut f64,
        s: *mut f64,
        c: *mut f64,
    );
    pub fn dlaisnan_(din1: *const f64, din2: *const f64) -> MKL_INT;
    pub fn dlaln2_(
        ltrans: *const MKL_INT,
        na: *const MKL_INT,
        nw: *const MKL_INT,
        smin: *const f64,
        ca: *const f64,
        a: *const f64,
        lda: *const MKL_INT,
        d1: *const f64,
        d2: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        wr: *const f64,
        wi: *const f64,
        x: *mut f64,
        ldx: *const MKL_INT,
        scale: *mut f64,
        xnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlals0_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        bx: *mut f64,
        ldbx: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *const f64,
        ldgnum: *const MKL_INT,
        poles: *const f64,
        difl: *const f64,
        difr: *const f64,
        z: *const f64,
        k: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlalsa_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        bx: *mut f64,
        ldbx: *const MKL_INT,
        u: *const f64,
        ldu: *const MKL_INT,
        vt: *const f64,
        k: *const MKL_INT,
        difl: *const f64,
        difr: *const f64,
        z: *const f64,
        poles: *const f64,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *const MKL_INT,
        givnum: *const f64,
        c: *const f64,
        s: *const f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlalsd_(
        uplo: *const c_char,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlamch_(cmach: *const c_char) -> f64;
    pub fn dlamc1_(beta: *mut MKL_INT, t: *mut MKL_INT, rnd: *mut MKL_INT, ieee1: *mut MKL_INT);
    pub fn dlamc2_(
        beta: *mut MKL_INT,
        t: *mut MKL_INT,
        rnd: *mut MKL_INT,
        eps: *mut f64,
        emin: *mut MKL_INT,
        rmin: *mut f64,
        emax: *mut MKL_INT,
        rmax: *mut f64,
    );
    pub fn dlamc3_(a: *const f64, b: *const f64) -> f64;
    pub fn dlamc4_(emin: *mut MKL_INT, start: *const f64, base: *const MKL_INT);
    pub fn dlamc5_(
        beta: *const MKL_INT,
        p: *const MKL_INT,
        emin: *const MKL_INT,
        ieee: *const MKL_INT,
        emax: *mut MKL_INT,
        rmax: *mut f64,
    );
    pub fn dlamrg_(
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        a: *const f64,
        dtrd1: *const MKL_INT,
        dtrd2: *const MKL_INT,
        index: *mut MKL_INT,
    );
    pub fn dlaneg_(
        n: *const MKL_INT,
        d: *const f64,
        lld: *const f64,
        sigma: *const f64,
        pivmin: *const f64,
        r: *const MKL_INT,
    ) -> MKL_INT;
    pub fn dlangb_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlange_(
        norm: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlangt_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
    ) -> f64;
    pub fn dlanhs_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlansb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlansf_(
        norm: *const c_char,
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        work: *mut f64,
    ) -> f64;
    pub fn dlansp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        work: *mut f64,
    ) -> f64;
    pub fn dlanst_(norm: *const c_char, n: *const MKL_INT, d: *const f64, e: *const f64) -> f64;
    pub fn dlansy_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlantb_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlantp_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        work: *mut f64,
    ) -> f64;
    pub fn dlantr_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dlanv2_(
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        d: *mut f64,
        rt1r: *mut f64,
        rt1i: *mut f64,
        rt2r: *mut f64,
        rt2i: *mut f64,
        cs: *mut f64,
        sn: *mut f64,
    );
    pub fn dlapll_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        ssmin: *mut f64,
    );
    pub fn dlapmt_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn dlapy2_(x: *const f64, y: *const f64) -> f64;
    pub fn dlapy3_(x: *const f64, y: *const f64, z: *const f64) -> f64;
    pub fn dlaqgb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        rowcnd: *const f64,
        colcnd: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn dlaqge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        rowcnd: *const f64,
        colcnd: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn dlaqp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f64,
        vn1: *mut f64,
        vn2: *mut f64,
        work: *mut f64,
    );
    pub fn dlaqps_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f64,
        vn1: *mut f64,
        vn2: *mut f64,
        auxv: *mut f64,
        f: *mut f64,
        ldf: *const MKL_INT,
    );
    pub fn dlaqr0_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaqr1_(
        n: *const MKL_INT,
        h: *const f64,
        ldh: *const MKL_INT,
        sr1: *const f64,
        si1: *mut f64,
        sr2: *mut f64,
        si2: *mut f64,
        v: *mut f64,
    );
    pub fn dlaqr2_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sr: *mut f64,
        si: *mut f64,
        v: *mut f64,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f64,
        ldwv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
    );
    pub fn dlaqr3_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sr: *mut f64,
        si: *mut f64,
        v: *mut f64,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f64,
        ldwv: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
    );
    pub fn dlaqr4_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f64,
        ldh: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaqr5_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        kacc22: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nshfts: *const MKL_INT,
        sr: *mut f64,
        si: *mut f64,
        h: *mut f64,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f64,
        ldwv: *const MKL_INT,
        nh: *const MKL_INT,
        wh: *mut f64,
        ldwh: *const MKL_INT,
    );
    pub fn dlaqsb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn dlaqsp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn dlaqsy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn dlaqtr_(
        ltran: *const MKL_INT,
        lreal: *const MKL_INT,
        n: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        b: *const f64,
        w: *const f64,
        scale: *mut f64,
        x: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlaqz0_(
        wants: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaqz1_(
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        sr1: *const f64,
        sr2: *const f64,
        si: *const f64,
        beta1: *const f64,
        beta2: *const f64,
        v: *mut f64,
    );
    pub fn dlaqz2_(
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        k: *const MKL_INT,
        istartm: *const MKL_INT,
        istopm: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        nq: *const MKL_INT,
        qstart: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        nz: *const MKL_INT,
        zstart: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
    );
    pub fn dlaqz3_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nw: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        qc: *mut f64,
        ldqc: *const MKL_INT,
        zc: *mut f64,
        ldzc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlaqz4_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nshifts: *const MKL_INT,
        nb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        qc: *mut f64,
        ldqc: *const MKL_INT,
        zc: *mut f64,
        ldzc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlar1v_(
        n: *const MKL_INT,
        b1: *const MKL_INT,
        bn: *const MKL_INT,
        lambda: *const f64,
        d: *const f64,
        l: *const f64,
        ld: *const f64,
        lld: *const f64,
        pivmin: *const f64,
        gaptol: *const f64,
        z: *mut f64,
        wantnc: *const MKL_INT,
        negcnt: *mut MKL_INT,
        ztz: *mut f64,
        mingma: *mut f64,
        r: *mut MKL_INT,
        isuppz: *mut MKL_INT,
        nrminv: *mut f64,
        resid: *mut f64,
        rqcorr: *mut f64,
        work: *mut f64,
    );
    pub fn dlar2v_(
        n: *const MKL_INT,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
        incx: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        incc: *const MKL_INT,
    );
    pub fn dlarfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        ldwork: *const MKL_INT,
    );
    pub fn dlarf_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlarf1f_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlarf1l_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlarfg_(
        n: *const MKL_INT,
        alpha: *mut f64,
        x: *mut f64,
        incx: *const MKL_INT,
        tau: *mut f64,
    );
    pub fn dlarfgp_(
        n: *const MKL_INT,
        alpha: *mut f64,
        x: *mut f64,
        incx: *const MKL_INT,
        tau: *mut f64,
    );
    pub fn dlarfp_(
        n: *const MKL_INT,
        alpha: *mut f64,
        x: *mut f64,
        incx: *const MKL_INT,
        tau: *mut f64,
    );
    pub fn dlarft_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        tau: *const f64,
        t: *mut f64,
        ldt: *const MKL_INT,
    );
    pub fn dlarfx_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f64,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlargv_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        c: *mut f64,
        incc: *const MKL_INT,
    );
    pub fn dlarnv_(idist: *const MKL_INT, iseed: *mut MKL_INT, n: *const MKL_INT, x: *mut f64);
    pub fn dlarra_(
        n: *const MKL_INT,
        d: *const f64,
        e: *mut f64,
        e2: *mut f64,
        spltol: *const f64,
        tnrm: *const f64,
        nsplit: *mut MKL_INT,
        isplit: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarrb_(
        n: *const MKL_INT,
        d: *const f64,
        lld: *const f64,
        ifirst: *const MKL_INT,
        ilast: *const MKL_INT,
        rtol1: *const f64,
        rtol2: *const f64,
        offset: *const MKL_INT,
        w: *mut f64,
        wgap: *mut f64,
        werr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        pivmin: *const f64,
        spdiam: *const f64,
        twist: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarrc_(
        jobt: *const c_char,
        n: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        d: *const f64,
        e: *const f64,
        pivmin: *const f64,
        eigcnt: *mut MKL_INT,
        lcnt: *mut MKL_INT,
        rcnt: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarrd_(
        range: *const c_char,
        order: *const c_char,
        n: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        gers: *const f64,
        reltol: *const f64,
        d: *const f64,
        e: *const f64,
        e2: *const f64,
        pivmin: *const f64,
        nsplit: *const MKL_INT,
        isplit: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f64,
        werr: *mut f64,
        wl: *mut f64,
        wu: *mut f64,
        iblock: *mut MKL_INT,
        indexw: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarre_(
        range: *const c_char,
        n: *const MKL_INT,
        vl: *mut f64,
        vu: *mut f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        e2: *mut f64,
        rtol1: *const f64,
        rtol2: *const f64,
        spltol: *const f64,
        nsplit: *mut MKL_INT,
        isplit: *mut MKL_INT,
        m: *mut MKL_INT,
        w: *mut f64,
        werr: *mut f64,
        wgap: *mut f64,
        iblock: *mut MKL_INT,
        indexw: *mut MKL_INT,
        gers: *mut f64,
        pivmin: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarrf_(
        n: *const MKL_INT,
        d: *const f64,
        l: *const f64,
        ld: *const f64,
        clstrt: *const MKL_INT,
        clend: *const MKL_INT,
        w: *const f64,
        wgap: *mut f64,
        werr: *const f64,
        spdiam: *const f64,
        clgapl: *const f64,
        clgapr: *mut f64,
        pivmin: *const f64,
        sigma: *mut f64,
        dplus: *mut f64,
        lplus: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlarrj_(
        n: *const MKL_INT,
        d: *const f64,
        e2: *const f64,
        ifirst: *const MKL_INT,
        ilast: *const MKL_INT,
        rtol: *const f64,
        offset: *const MKL_INT,
        w: *mut f64,
        werr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        pivmin: *const f64,
        spdiam: *const f64,
        info: *mut MKL_INT,
    );
    pub fn dlarrk_(
        n: *const MKL_INT,
        iw: *const MKL_INT,
        gl: *const f64,
        gu: *const f64,
        d: *const f64,
        e2: *const f64,
        pivmin: *const f64,
        reltol: *const f64,
        w: *mut f64,
        werr: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlarrr_(n: *const MKL_INT, d: *const f64, e: *mut f64, info: *mut MKL_INT);
    pub fn dlarrv_(
        n: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        d: *mut f64,
        l: *mut f64,
        pivmin: *mut f64,
        isplit: *const MKL_INT,
        m: *const MKL_INT,
        dol: *const MKL_INT,
        dou: *const MKL_INT,
        minrgp: *const f64,
        rtol1: *const f64,
        rtol2: *const f64,
        w: *mut f64,
        werr: *mut f64,
        wgap: *mut f64,
        iblock: *const MKL_INT,
        indexw: *const MKL_INT,
        gers: *const f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlarscl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f64,
        x: *mut f64,
        ldx: *const MKL_INT,
    );
    pub fn dlartg_(f: *const f64, g: *const f64, cs: *mut f64, sn: *mut f64, r: *mut f64);
    pub fn dlartv_(
        n: *const MKL_INT,
        x: *mut f64,
        incx: *const MKL_INT,
        y: *mut f64,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        incc: *const MKL_INT,
    );
    pub fn dlaruv_(iseed: *mut MKL_INT, n: *const MKL_INT, x: *mut f64);
    pub fn dlarzb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        ldwork: *const MKL_INT,
    );
    pub fn dlarz_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlarzt_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        tau: *const f64,
        t: *mut f64,
        ldt: *const MKL_INT,
    );
    pub fn dlas2_(f: *const f64, g: *const f64, h: *const f64, ssmin: *mut f64, ssmax: *mut f64);
    pub fn dlascl_(
        type_: *const c_char,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        cfrom: *const f64,
        cto: *const f64,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlascl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f64,
        x: *mut f64,
        ldx: *const MKL_INT,
    );
    pub fn dlasd0_(
        n: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f64,
        e: *const f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        smlsiz: *const MKL_INT,
        iwork: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasd1_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f64,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        idxq: *mut MKL_INT,
        iwork: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasd2_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *mut MKL_INT,
        d: *mut f64,
        z: *mut f64,
        alpha: *const f64,
        beta: *const f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        dsigma: *mut f64,
        u2: *mut f64,
        ldu2: *const MKL_INT,
        vt2: *mut f64,
        ldvt2: *const MKL_INT,
        idxp: *mut MKL_INT,
        idx: *mut MKL_INT,
        idxc: *mut MKL_INT,
        idxq: *mut MKL_INT,
        coltyp: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlasd3_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *const MKL_INT,
        d: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        dsigma: *const f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        u2: *mut f64,
        ldu2: *const MKL_INT,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        vt2: *mut f64,
        ldvt2: *const MKL_INT,
        idxc: *const MKL_INT,
        ctot: *const MKL_INT,
        z: *const f64,
        info: *mut MKL_INT,
    );
    pub fn dlasd4_(
        n: *const MKL_INT,
        i: *const MKL_INT,
        d: *const f64,
        z: *const f64,
        delta: *mut f64,
        rho: *const f64,
        sigma: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasd5_(
        i: *const MKL_INT,
        d: *const f64,
        z: *const f64,
        delta: *mut f64,
        rho: *const f64,
        dsigma: *mut f64,
        work: *mut f64,
    );
    pub fn dlasd6_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f64,
        vf: *mut f64,
        vl: *mut f64,
        alpha: *mut f64,
        beta: *mut f64,
        idxq: *mut MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *mut f64,
        ldgnum: *const MKL_INT,
        poles: *mut f64,
        difl: *mut f64,
        difr: *mut f64,
        z: *mut f64,
        k: *mut MKL_INT,
        c: *mut f64,
        s: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlasd7_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *mut MKL_INT,
        d: *mut f64,
        z: *mut f64,
        zw: *mut f64,
        vf: *mut f64,
        vfw: *mut f64,
        vl: *mut f64,
        vlw: *mut f64,
        alpha: *const f64,
        beta: *const f64,
        dsigma: *mut f64,
        idx: *mut MKL_INT,
        idxp: *mut MKL_INT,
        idxq: *const MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *mut f64,
        ldgnum: *const MKL_INT,
        c: *mut f64,
        s: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasd8_(
        icompq: *const MKL_INT,
        k: *const MKL_INT,
        d: *mut f64,
        z: *mut f64,
        vf: *mut f64,
        vl: *mut f64,
        difl: *mut f64,
        difr: *mut f64,
        lddifr: *const MKL_INT,
        dsigma: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasda_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f64,
        e: *const f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        vt: *mut f64,
        k: *mut MKL_INT,
        difl: *mut f64,
        difr: *mut f64,
        z: *mut f64,
        poles: *mut f64,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *mut MKL_INT,
        givnum: *mut f64,
        c: *mut f64,
        s: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlasdq_(
        uplo: *const c_char,
        sqre: *const MKL_INT,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vt: *mut f64,
        ldvt: *const MKL_INT,
        u: *mut f64,
        ldu: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasdt_(
        n: *const MKL_INT,
        lvl: *mut MKL_INT,
        nd: *mut MKL_INT,
        inode: *mut MKL_INT,
        ndiml: *mut MKL_INT,
        ndimr: *mut MKL_INT,
        msub: *const MKL_INT,
    );
    pub fn dlaset_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        beta: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dlasq1_(n: *const MKL_INT, d: *mut f64, e: *mut f64, work: *mut f64, info: *mut MKL_INT);
    pub fn dlasq2_(n: *const MKL_INT, z: *mut f64, info: *mut MKL_INT);
    pub fn dlasq3_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f64,
        pp: *mut MKL_INT,
        dmin: *mut f64,
        sigma: *mut f64,
        desig: *mut f64,
        qmax: *const f64,
        nfail: *mut MKL_INT,
        iter: *mut MKL_INT,
        ndiv: *mut MKL_INT,
        ieee: *const MKL_INT,
        ttype: *mut MKL_INT,
        dmin1: *mut f64,
        dmin2: *mut f64,
        dn: *mut f64,
        dn1: *mut f64,
        dn2: *mut f64,
        g: *mut f64,
        tau: *mut f64,
    );
    pub fn dlasq4_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f64,
        pp: *const MKL_INT,
        n0in: *mut MKL_INT,
        dmin: *const f64,
        dmin1: *const f64,
        dmin2: *const f64,
        dn: *const f64,
        dn1: *const f64,
        dn2: *const f64,
        tau: *mut f64,
        ttype: *mut MKL_INT,
        g: *mut f64,
    );
    pub fn dlasq5_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f64,
        pp: *const MKL_INT,
        tau: *const f64,
        sigma: *const f64,
        dmin: *mut f64,
        dmin1: *mut f64,
        dmin2: *mut f64,
        dn: *mut f64,
        dnm1: *mut f64,
        dnm2: *mut f64,
        ieee: *const MKL_INT,
        eps: *const f64,
    );
    pub fn dlasq6_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f64,
        pp: *const MKL_INT,
        dmin: *mut f64,
        dmin1: *mut f64,
        dmin2: *mut f64,
        dn: *mut f64,
        dnm1: *mut f64,
        dnm2: *mut f64,
    );
    pub fn dlasr_(
        side: *const c_char,
        pivot: *const c_char,
        direct: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
    );
    pub fn dlasrt_(id: *const c_char, n: *const MKL_INT, d: *mut f64, info: *mut MKL_INT);
    pub fn dlassq_(
        n: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        scale: *mut f64,
        sumsq: *mut f64,
    );
    pub fn dlasv2_(
        f: *const f64,
        g: *const f64,
        h: *const f64,
        ssmin: *mut f64,
        ssmax: *mut f64,
        snr: *mut f64,
        csr: *mut f64,
        snl: *mut f64,
        csl: *mut f64,
    );
    pub fn dlaswp_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        k1: *const MKL_INT,
        k2: *const MKL_INT,
        ipiv: *const MKL_INT,
        incx: *const MKL_INT,
    );
    pub fn dlasy2_(
        ltranl: *const MKL_INT,
        ltranr: *const MKL_INT,
        isgn: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        tl: *const f64,
        ldtl: *const MKL_INT,
        tr: *const f64,
        ldtr: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        scale: *mut f64,
        x: *mut f64,
        ldx: *const MKL_INT,
        xnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlasyf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut f64,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlat2s_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        sa: *mut f32,
        ldsa: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatbs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        x: *mut f64,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlatdf_(
        ijob: *const MKL_INT,
        n: *const MKL_INT,
        z: *const f64,
        ldz: *const MKL_INT,
        rhs: *mut f64,
        rdsum: *mut f64,
        rdscal: *mut f64,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
    );
    pub fn dlatps_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        x: *mut f64,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlatrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        e: *mut f64,
        tau: *mut f64,
        w: *mut f64,
        ldw: *const MKL_INT,
    );
    pub fn dlatrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dlatrs3_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        scale: *mut f64,
        cnorm: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatrz_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
    );
    pub fn dlatzm_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f64,
        incv: *const MKL_INT,
        tau: *const f64,
        c1: *mut f64,
        c2: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
    );
    pub fn dlauu2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlauum_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dopgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        tau: *const f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dopmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ap: *const f64,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorg2l_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorg2r_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorgbr_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorghr_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorglq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgql_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorgrq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *const f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorm2l_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dorm2r_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dormbr_(
        vect: *const c_char,
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormhr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dorml2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dormlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormql_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormr2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dormr3_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dormrq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormrz_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dormtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        tau: *const f64,
        c: *mut f64,
        ldc: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpbrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbstf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        afb: *mut f64,
        ldafb: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbtf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbtrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpbtrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpftrf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpftri_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpftrs_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpocon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpoequb_(
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpoequ_(
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dporfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dporfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        s: *mut f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dposvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dposvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpotf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpotrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpotri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpotrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dppcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dppequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        afp: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dppsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dppsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f64,
        afp: *mut f64,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpptrf_(uplo: *const c_char, n: *const MKL_INT, ap: *mut f64, info: *mut MKL_INT);
    pub fn dpptri_(uplo: *const c_char, n: *const MKL_INT, ap: *mut f64, info: *mut MKL_INT);
    pub fn dpptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dpstf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpstrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dptcon_(
        n: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dptrfs_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        df: *const f64,
        ef: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dptsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dptsvx_(
        fact: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        df: *mut f64,
        ef: *mut f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dpttrf_(n: *const MKL_INT, d: *mut f64, e: *mut f64, info: *mut MKL_INT);
    pub fn dpttrs_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dptts2_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
    );
    pub fn drscl_(n: *const MKL_INT, sa: *const f64, sx: *mut f64, incx: *const MKL_INT);
    pub fn dsbevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsbevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbgst_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        bb: *const f64,
        ldbb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsbgvd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        bb: *mut f64,
        ldbb: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbgv_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        bb: *mut f64,
        ldbb: *const MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsbgvx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        bb: *mut f64,
        ldbb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsbtrd_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f64,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsecnd_() -> f64;
    pub fn dsfrk_(
        transr: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f64,
        a: *const f64,
        lda: *const MKL_INT,
        beta: *const f64,
        c: *mut f64,
    );
    pub fn dsgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        work: *mut f64,
        swork: *mut f32,
        iter: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dspevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspgst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        bp: *const f64,
        info: *mut MKL_INT,
    );
    pub fn dspgvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        bp: *mut f64,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspgv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        bp: *mut f64,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dspgvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        bp: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        work: *mut f64,
        swork: *mut f32,
        iter: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        afp: *const f64,
        ipiv: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f64,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dspsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        afp: *mut f64,
        ipiv: *mut MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsptrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        ipiv: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstebz_(
        range: *const c_char,
        order: *const c_char,
        n: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        d: *const f64,
        e: *const f64,
        m: *mut MKL_INT,
        nsplit: *mut MKL_INT,
        w: *mut f64,
        iblock: *mut MKL_INT,
        isplit: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstedc_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstegr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstein_(
        n: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        m: *const MKL_INT,
        w: *const f64,
        iblock: *const MKL_INT,
        isplit: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstemr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        nzc: *const MKL_INT,
        isuppz: *mut MKL_INT,
        tryrac: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsterf_(n: *const MKL_INT, d: *mut f64, e: *mut f64, info: *mut MKL_INT);
    pub fn dstevd_(
        jobz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstev_(
        jobz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dstevr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dstevx_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsycon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsyevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyevr_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygs2_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsygvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyrfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsysvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        af: *mut f64,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        x: *mut f64,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytd2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsytf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsytri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dsytrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtbcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtbrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *const f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtbtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtfsm_(
        transr: *const c_char,
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        a: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
    );
    pub fn dtftri_(
        transr: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtfttp_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const f64,
        ap: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtfttr_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        s: *const f64,
        lds: *const MKL_INT,
        p: *const f64,
        ldp: *const MKL_INT,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtgex2_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        j1: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgexc_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        ifst: *mut MKL_INT,
        ilst: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgsen_(
        ijob: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        alphar: *mut f64,
        alphai: *mut f64,
        beta: *mut f64,
        q: *mut f64,
        ldq: *const MKL_INT,
        z: *mut f64,
        ldz: *const MKL_INT,
        m: *mut MKL_INT,
        pl: *mut f64,
        pr: *mut f64,
        dif: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgsja_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut f64,
        ldu: *const MKL_INT,
        v: *mut f64,
        ldv: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        work: *mut f64,
        ncycle: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        vl: *const f64,
        ldvl: *const MKL_INT,
        vr: *const f64,
        ldvr: *const MKL_INT,
        s: *mut f64,
        dif: *mut f64,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgsy2_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        d: *const f64,
        ldd: *const MKL_INT,
        e: *const f64,
        lde: *const MKL_INT,
        f: *mut f64,
        ldf: *const MKL_INT,
        scale: *mut f64,
        rdsum: *mut f64,
        rdscal: *mut f64,
        iwork: *mut MKL_INT,
        pq: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtgsyl_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        d: *const f64,
        ldd: *const MKL_INT,
        e: *const f64,
        lde: *const MKL_INT,
        f: *mut f64,
        ldf: *const MKL_INT,
        scale: *mut f64,
        dif: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtpcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtprfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *const f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtptri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtptrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtpttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        arf: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtpttr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        rcond: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        vl: *mut f64,
        ldvl: *const MKL_INT,
        vr: *mut f64,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtrexc_(
        compq: *const c_char,
        n: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        ifst: *mut MKL_INT,
        ilst: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtrrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        x: *const f64,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrsen_(
        job: *const c_char,
        compq: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        q: *mut f64,
        ldq: *const MKL_INT,
        wr: *mut f64,
        wi: *mut f64,
        m: *mut MKL_INT,
        s: *mut f64,
        sep: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        vl: *const f64,
        ldvl: *const MKL_INT,
        vr: *const f64,
        ldvr: *const MKL_INT,
        s: *mut f64,
        sep: *mut f64,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f64,
        ldwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrsyl_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        scale: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtrsyl3_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        scale: *mut f64,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        swork: *mut f64,
        ldswork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrti2_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrtri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dtrttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        arf: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtrttp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        ap: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtzrqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn dtzrzf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        tau: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dzsum1_(n: *const MKL_INT, cx: *const MKL_Complex16, incx: *const MKL_INT) -> f64;
    pub fn icmax1_(n: *const MKL_INT, cx: *const MKL_Complex8, incx: *const MKL_INT) -> MKL_INT;
    pub fn ieeeck_(ispec: *const MKL_INT, zero: *const f32, one: *const f32) -> MKL_INT;
    pub fn ilaclc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilaclr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn iladiag_(diag: *const c_char) -> MKL_INT;
    pub fn iladlc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn iladlr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilaenv_(
        ispec: *const MKL_INT,
        name: *const c_char,
        opts: *const c_char,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        n3: *const MKL_INT,
        n4: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilaenv2stage_(
        ispec: *const MKL_INT,
        name: *const c_char,
        opts: *const c_char,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        n3: *const MKL_INT,
        n4: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilaprec_(prec: *const c_char) -> MKL_INT;
    pub fn ilaslc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilaslr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilatrans_(trans: *const c_char) -> MKL_INT;
    pub fn ilauplo_(uplo: *const c_char) -> MKL_INT;
    pub fn ilaver_(vers_major: *mut MKL_INT, vers_minor: *mut MKL_INT, vers_patch: *mut MKL_INT);
    pub fn ilazlc_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn ilazlr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
    ) -> MKL_INT;
    pub fn iparmq_(
        ispec: *const MKL_INT,
        name: *const c_char,
        opts: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        lwork: *const MKL_INT,
    ) -> MKL_INT;
    pub fn izmax1_(n: *const MKL_INT, cx: *const MKL_Complex16, incx: *const MKL_INT) -> MKL_INT;
    pub fn lsamen_(n: *const MKL_INT, ca: *const c_char, cb: *const c_char) -> MKL_INT;
    pub fn sbdsdc_(
        uplo: *const c_char,
        compq: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        q: *mut f32,
        iq: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sbdsqr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn scsum1_(n: *const MKL_INT, cx: *const MKL_Complex8, incx: *const MKL_INT) -> f32;
    pub fn sdisna_(
        job: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f32,
        sep: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn second_() -> f32;
    pub fn sgbbrd_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ncc: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        pt: *mut f32,
        ldpt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgbcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgbequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgbrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbrfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbsv_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        afb: *mut f32,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbsvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        afb: *mut f32,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbtf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbtrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgbtrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgebak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        scale: *const f32,
        m: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgebal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgebd2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut f32,
        taup: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgebrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut f32,
        taup: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgecon_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgedmd_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f32,
        k: *mut MKL_INT,
        reig: *mut f32,
        imeig: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        res: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        w: *mut f32,
        ldw: *const MKL_INT,
        s: *mut f32,
        lds: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgedmdq_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobq: *const c_char,
        jobt: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        f: *mut f32,
        ldf: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f32,
        k: *mut MKL_INT,
        reigs: *mut f32,
        imeig: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        res: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        s: *mut f32,
        lds: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        r: *mut f32,
        c: *mut f32,
        rowcnd: *mut f32,
        colcnd: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgees_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_S_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        vs: *mut f32,
        ldvs: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeesx_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_S_SELECT_FUNCTION_2,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        vs: *mut f32,
        ldvs: *const MKL_INT,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f32,
        abnrm: *mut f32,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgegs_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vsl: *mut f32,
        ldvsl: *const MKL_INT,
        vsr: *mut f32,
        ldvsr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgegv_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgehd2_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgehrd_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgejsv_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        jobr: *const c_char,
        jobt: *const c_char,
        jobp: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        sva: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgelqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelsd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        s: *mut f32,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgels_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelst_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelss_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        s: *mut f32,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgelsx_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgelsy_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeql2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeqlf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeqp3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeqp3rk_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        kmax: *const MKL_INT,
        abstol: *const f32,
        reltol: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        k: *mut MKL_INT,
        maxc2nrmk: *mut f32,
        relmaxc2nrmk: *mut f32,
        jpiv: *mut MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeqpf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeqr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeqr2p_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgeqrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgeqrfp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgerfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgerfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgerq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgerqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesc2_(
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        rhs: *mut f32,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
        scale: *mut f32,
    );
    pub fn sgesdd_(
        jobz: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvd_(
        jobu: *const c_char,
        jobvt: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        s: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvj_(
        joba: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgesvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f32,
        c: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetc2_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        jpiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_sgetrfnpi_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nfact: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetri_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgetrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggbak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        lscale: *const f32,
        rscale: *const f32,
        m: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggbal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f32,
        rscale: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgges_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_S_SELECT_FUNCTION_3,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vsl: *mut f32,
        ldvsl: *const MKL_INT,
        vsr: *mut f32,
        ldvsr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggesx_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_S_SELECT_FUNCTION_3,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vsl: *mut f32,
        ldvsl: *const MKL_INT,
        vsr: *mut f32,
        ldvsr: *const MKL_INT,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f32,
        rscale: *mut f32,
        abnrm: *mut f32,
        bbnrm: *mut f32,
        rconde: *mut f32,
        rcondv: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggglm_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        d: *mut f32,
        x: *mut f32,
        y: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgghrd_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgglse_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        d: *mut f32,
        x: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggqrf_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        taua: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        taub: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggrqf_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        taua: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        taub: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggsvd_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sggsvp_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sgsvj0_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        eps: *const f32,
        sfmin: *const f32,
        tol: *const f32,
        nsweep: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgsvj1_(
        jobv: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        sva: *mut f32,
        mv: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        eps: *const f32,
        sfmin: *const f32,
        tol: *const f32,
        nsweep: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgtcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        du2: *const f32,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgtrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        dlf: *const f32,
        df: *const f32,
        duf: *const f32,
        du2: *const f32,
        ipiv: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgtsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgtsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        dlf: *mut f32,
        df: *mut f32,
        duf: *mut f32,
        du2: *mut f32,
        ipiv: *mut MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgttrf_(
        n: *const MKL_INT,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        du2: *mut f32,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgttrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        du2: *const f32,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sgtts2_(
        itrans: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        du2: *const f32,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
    );
    pub fn shgeqz_(
        job: *const c_char,
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn shsein_(
        side: *const c_char,
        eigsrc: *const c_char,
        initv: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        h: *const f32,
        ldh: *const MKL_INT,
        wr: *mut f32,
        wi: *const f32,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        ifaill: *mut MKL_INT,
        ifailr: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn shseqr_(
        job: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sisnan_(sin: *const f32) -> MKL_INT;
    pub fn slabad_(smallx: *mut f32, large: *mut f32);
    pub fn slabrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tauq: *mut f32,
        taup: *mut f32,
        x: *mut f32,
        ldx: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
    );
    pub fn slacn2_(
        n: *const MKL_INT,
        v: *mut f32,
        x: *mut f32,
        isgn: *mut MKL_INT,
        est: *mut f32,
        kase: *mut MKL_INT,
        isave: *mut MKL_INT,
    );
    pub fn slacon_(
        n: *const MKL_INT,
        v: *mut f32,
        x: *mut f32,
        isgn: *mut MKL_INT,
        est: *mut f32,
        kase: *mut MKL_INT,
    );
    pub fn slacpy_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
    );
    pub fn sladiv_(
        a: *const f32,
        b: *const f32,
        c: *const f32,
        d: *const f32,
        p: *mut f32,
        q: *mut f32,
    );
    pub fn slae2_(a: *const f32, b: *const f32, c: *const f32, rt1: *mut f32, rt2: *mut f32);
    pub fn slaebz_(
        ijob: *const MKL_INT,
        nitmax: *const MKL_INT,
        n: *const MKL_INT,
        mmax: *const MKL_INT,
        minp: *const MKL_INT,
        nbmin: *const MKL_INT,
        abstol: *const f32,
        reltol: *const f32,
        pivmin: *const f32,
        d: *const f32,
        e: *const f32,
        e2: *const f32,
        nval: *mut MKL_INT,
        ab: *mut f32,
        c: *mut f32,
        mout: *mut MKL_INT,
        nab: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed0_(
        icompq: *const MKL_INT,
        qsiz: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f32,
        e: *const f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        qstore: *mut f32,
        ldqs: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed1_(
        n: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *const f32,
        cutpnt: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed2_(
        k: *mut MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *mut f32,
        z: *const f32,
        dlamda: *mut f32,
        w: *mut f32,
        q2: *mut f32,
        indx: *mut MKL_INT,
        indxc: *mut MKL_INT,
        indxp: *mut MKL_INT,
        coltyp: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed3_(
        k: *const MKL_INT,
        n: *const MKL_INT,
        n1: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        rho: *const f32,
        dlamda: *mut f32,
        q2: *const f32,
        indx: *const MKL_INT,
        ctot: *const MKL_INT,
        w: *mut f32,
        s: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaed4_(
        n: *const MKL_INT,
        i: *const MKL_INT,
        d: *const f32,
        z: *const f32,
        delta: *mut f32,
        rho: *const f32,
        dlam: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaed5_(
        i: *const MKL_INT,
        d: *const f32,
        z: *const f32,
        delta: *mut f32,
        rho: *const f32,
        dlam: *mut f32,
    );
    pub fn slaed6_(
        kniter: *const MKL_INT,
        orgati: *const MKL_INT,
        rho: *const f32,
        d: *const f32,
        z: *const f32,
        finit: *const f32,
        tau: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaed7_(
        icompq: *const MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        indxq: *mut MKL_INT,
        rho: *const f32,
        cutpnt: *const MKL_INT,
        qstore: *mut f32,
        qptr: *mut MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed8_(
        icompq: *const MKL_INT,
        k: *mut MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        indxq: *const MKL_INT,
        rho: *mut f32,
        cutpnt: *const MKL_INT,
        z: *const f32,
        dlamda: *mut f32,
        q2: *mut f32,
        ldq2: *const MKL_INT,
        w: *mut f32,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        givnum: *mut f32,
        indxp: *mut MKL_INT,
        indx: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaed9_(
        k: *const MKL_INT,
        kstart: *const MKL_INT,
        kstop: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        rho: *const f32,
        dlamda: *const f32,
        w: *const f32,
        s: *mut f32,
        lds: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaeda_(
        n: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f32,
        q: *const f32,
        qptr: *const MKL_INT,
        z: *mut f32,
        ztemp: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaein_(
        rightv: *const MKL_INT,
        noinit: *const MKL_INT,
        n: *const MKL_INT,
        h: *const f32,
        ldh: *const MKL_INT,
        wr: *const f32,
        wi: *const f32,
        vr: *mut f32,
        vi: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        eps3: *const f32,
        smlnum: *const f32,
        bignum: *const f32,
        info: *mut MKL_INT,
    );
    pub fn slaev2_(
        a: *const f32,
        b: *const f32,
        c: *const f32,
        rt1: *mut f32,
        rt2: *mut f32,
        cs1: *mut f32,
        sn1: *mut f32,
    );
    pub fn slaexc_(
        wantq: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        j1: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slag2d_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        sa: *const f32,
        ldsa: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slag2_(
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        safmin: *const f32,
        scale1: *mut f32,
        scale2: *mut f32,
        wr1: *mut f32,
        wr2: *mut f32,
        wi: *mut f32,
    );
    pub fn slags2_(
        upper: *const MKL_INT,
        a1: *const f32,
        a2: *const f32,
        a3: *const f32,
        b1: *const f32,
        b2: *const f32,
        b3: *const f32,
        csu: *mut f32,
        snu: *mut f32,
        csv: *mut f32,
        snv: *mut f32,
        csq: *mut f32,
        snq: *mut f32,
    );
    pub fn slagtf_(
        n: *const MKL_INT,
        a: *mut f32,
        lambda: *const f32,
        b: *mut f32,
        c: *mut f32,
        tol: *const f32,
        d: *mut f32,
        in_: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slagtm_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        alpha: *const f32,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *const f32,
        ldx: *const MKL_INT,
        beta: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
    );
    pub fn slagts_(
        job: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        b: *const f32,
        c: *const f32,
        d: *const f32,
        in_: *const MKL_INT,
        y: *mut f32,
        tol: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slagv2_(
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        csl: *mut f32,
        snl: *mut f32,
        csr: *mut f32,
        snr: *mut f32,
    );
    pub fn slahqr_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slahr2_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        t: *mut f32,
        ldt: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
    );
    pub fn slahrd_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        t: *mut f32,
        ldt: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
    );
    pub fn slaic1_(
        job: *const MKL_INT,
        j: *const MKL_INT,
        x: *const f32,
        sest: *const f32,
        w: *const f32,
        gamma: *const f32,
        sestpr: *mut f32,
        s: *mut f32,
        c: *mut f32,
    );
    pub fn slaisnan_(sin1: *const f32, sin2: *const f32) -> MKL_INT;
    pub fn slaln2_(
        ltrans: *const MKL_INT,
        na: *const MKL_INT,
        nw: *const MKL_INT,
        smin: *const f32,
        ca: *const f32,
        a: *const f32,
        lda: *const MKL_INT,
        d1: *const f32,
        d2: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        wr: *const f32,
        wi: *const f32,
        x: *mut f32,
        ldx: *const MKL_INT,
        scale: *mut f32,
        xnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slals0_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        bx: *mut f32,
        ldbx: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *const f32,
        ldgnum: *const MKL_INT,
        poles: *const f32,
        difl: *const f32,
        difr: *const f32,
        z: *const f32,
        k: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slalsa_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        bx: *mut f32,
        ldbx: *const MKL_INT,
        u: *const f32,
        ldu: *const MKL_INT,
        vt: *const f32,
        k: *const MKL_INT,
        difl: *const f32,
        difr: *const f32,
        z: *const f32,
        poles: *const f32,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *const MKL_INT,
        givnum: *const f32,
        c: *const f32,
        s: *const f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slalsd_(
        uplo: *const c_char,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        rcond: *const f32,
        rank: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slamch_(cmach: *const c_char) -> f32;
    pub fn slamc1_(beta: *mut MKL_INT, t: *mut MKL_INT, rnd: *mut MKL_INT, ieee1: *mut MKL_INT);
    pub fn slamc2_(
        beta: *mut MKL_INT,
        t: *mut MKL_INT,
        rnd: *mut MKL_INT,
        eps: *mut f32,
        emin: *mut MKL_INT,
        rmin: *mut f32,
        emax: *mut MKL_INT,
        rmax: *mut f32,
    );
    pub fn slamc3_(a: *const f32, b: *const f32) -> f32;
    pub fn slamc4_(emin: *mut MKL_INT, start: *const f32, base: *const MKL_INT);
    pub fn slamc5_(
        beta: *const MKL_INT,
        p: *const MKL_INT,
        emin: *const MKL_INT,
        ieee: *const MKL_INT,
        emax: *mut MKL_INT,
        rmax: *mut f32,
    );
    pub fn slamrg_(
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        a: *const f32,
        strd1: *const MKL_INT,
        strd2: *const MKL_INT,
        index: *mut MKL_INT,
    );
    pub fn slaneg_(
        n: *const MKL_INT,
        d: *const f32,
        lld: *const f32,
        sigma: *const f32,
        pivmin: *const f32,
        r: *const MKL_INT,
    ) -> MKL_INT;
    pub fn slangb_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slange_(
        norm: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slangt_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
    ) -> f32;
    pub fn slanhs_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slansb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slansf_(
        norm: *const c_char,
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        work: *mut f32,
    ) -> f32;
    pub fn slansp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        work: *mut f32,
    ) -> f32;
    pub fn slanst_(norm: *const c_char, n: *const MKL_INT, d: *const f32, e: *const f32) -> f32;
    pub fn slansy_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slantb_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slantp_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        work: *mut f32,
    ) -> f32;
    pub fn slantr_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn slanv2_(
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        d: *mut f32,
        rt1r: *mut f32,
        rt1i: *mut f32,
        rt2r: *mut f32,
        rt2i: *mut f32,
        cs: *mut f32,
        sn: *mut f32,
    );
    pub fn slapll_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        ssmin: *mut f32,
    );
    pub fn slapmt_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn slapy2_(x: *const f32, y: *const f32) -> f32;
    pub fn slapy3_(x: *const f32, y: *const f32, z: *const f32) -> f32;
    pub fn slaqgb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        rowcnd: *const f32,
        colcnd: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn slaqge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        r: *const f32,
        c: *const f32,
        rowcnd: *const f32,
        colcnd: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn slaqp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f32,
        vn1: *mut f32,
        vn2: *mut f32,
        work: *mut f32,
    );
    pub fn slaqps_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut f32,
        vn1: *mut f32,
        vn2: *mut f32,
        auxv: *mut f32,
        f: *mut f32,
        ldf: *const MKL_INT,
    );
    pub fn slaqr0_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaqr1_(
        n: *const MKL_INT,
        h: *const f32,
        ldh: *const MKL_INT,
        sr1: *const f32,
        si1: *mut f32,
        sr2: *mut f32,
        si2: *mut f32,
        v: *mut f32,
    );
    pub fn slaqr2_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sr: *mut f32,
        si: *mut f32,
        v: *mut f32,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f32,
        ldwv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
    );
    pub fn slaqr3_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sr: *mut f32,
        si: *mut f32,
        v: *mut f32,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f32,
        ldwv: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
    );
    pub fn slaqr4_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut f32,
        ldh: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaqr5_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        kacc22: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nshfts: *const MKL_INT,
        sr: *mut f32,
        si: *mut f32,
        h: *mut f32,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut f32,
        ldwv: *const MKL_INT,
        nh: *const MKL_INT,
        wh: *mut f32,
        ldwh: *const MKL_INT,
    );
    pub fn slaqsb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn slaqsp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn slaqsy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        s: *const f32,
        scond: *const f32,
        amax: *const f32,
        equed: *mut c_char,
    );
    pub fn slaqtr_(
        ltran: *const MKL_INT,
        lreal: *const MKL_INT,
        n: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        b: *const f32,
        w: *const f32,
        scale: *mut f32,
        x: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slaqz0_(
        wants: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaqz1_(
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        sr1: *const f32,
        sr2: *const f32,
        si: *const f32,
        beta1: *const f32,
        beta2: *const f32,
        v: *mut f32,
    );
    pub fn slaqz2_(
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        k: *const MKL_INT,
        istartm: *const MKL_INT,
        istopm: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        nq: *const MKL_INT,
        qstart: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        nz: *const MKL_INT,
        zstart: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
    );
    pub fn slaqz3_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nw: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        qc: *mut f32,
        ldqc: *const MKL_INT,
        zc: *mut f32,
        ldzc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slaqz4_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nshifts: *const MKL_INT,
        nb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        qc: *mut f32,
        ldqc: *const MKL_INT,
        zc: *mut f32,
        ldzc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slar1v_(
        n: *const MKL_INT,
        b1: *const MKL_INT,
        bn: *const MKL_INT,
        lambda: *const f32,
        d: *const f32,
        l: *const f32,
        ld: *const f32,
        lld: *const f32,
        pivmin: *const f32,
        gaptol: *const f32,
        z: *mut f32,
        wantnc: *const MKL_INT,
        negcnt: *mut MKL_INT,
        ztz: *mut f32,
        mingma: *mut f32,
        r: *mut MKL_INT,
        isuppz: *mut MKL_INT,
        nrminv: *mut f32,
        resid: *mut f32,
        rqcorr: *mut f32,
        work: *mut f32,
    );
    pub fn slar2v_(
        n: *const MKL_INT,
        x: *mut f32,
        y: *mut f32,
        z: *mut f32,
        incx: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        incc: *const MKL_INT,
    );
    pub fn slarfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        ldwork: *const MKL_INT,
    );
    pub fn slarf_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slarf1f_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slarf1l_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slarfg_(
        n: *const MKL_INT,
        alpha: *mut f32,
        x: *mut f32,
        incx: *const MKL_INT,
        tau: *mut f32,
    );
    pub fn slarfgp_(
        n: *const MKL_INT,
        alpha: *mut f32,
        x: *mut f32,
        incx: *const MKL_INT,
        tau: *mut f32,
    );
    pub fn slarfp_(
        n: *const MKL_INT,
        alpha: *mut f32,
        x: *mut f32,
        incx: *const MKL_INT,
        tau: *mut f32,
    );
    pub fn slarft_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        tau: *const f32,
        t: *mut f32,
        ldt: *const MKL_INT,
    );
    pub fn slarfx_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f32,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slargv_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        c: *mut f32,
        incc: *const MKL_INT,
    );
    pub fn slarnv_(idist: *const MKL_INT, iseed: *mut MKL_INT, n: *const MKL_INT, x: *mut f32);
    pub fn slarra_(
        n: *const MKL_INT,
        d: *const f32,
        e: *mut f32,
        e2: *mut f32,
        spltol: *const f32,
        tnrm: *const f32,
        nsplit: *mut MKL_INT,
        isplit: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarrb_(
        n: *const MKL_INT,
        d: *const f32,
        lld: *const f32,
        ifirst: *const MKL_INT,
        ilast: *const MKL_INT,
        rtol1: *const f32,
        rtol2: *const f32,
        offset: *const MKL_INT,
        w: *mut f32,
        wgap: *mut f32,
        werr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        pivmin: *const f32,
        spdiam: *const f32,
        twist: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarrc_(
        jobt: *const c_char,
        n: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        d: *const f32,
        e: *const f32,
        pivmin: *const f32,
        eigcnt: *mut MKL_INT,
        lcnt: *mut MKL_INT,
        rcnt: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarrd_(
        range: *const c_char,
        order: *const c_char,
        n: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        gers: *const f32,
        reltol: *const f32,
        d: *const f32,
        e: *const f32,
        e2: *const f32,
        pivmin: *const f32,
        nsplit: *const MKL_INT,
        isplit: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f32,
        werr: *mut f32,
        wl: *mut f32,
        wu: *mut f32,
        iblock: *mut MKL_INT,
        indexw: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarre_(
        range: *const c_char,
        n: *const MKL_INT,
        vl: *mut f32,
        vu: *mut f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        e2: *mut f32,
        rtol1: *const f32,
        rtol2: *const f32,
        spltol: *const f32,
        nsplit: *mut MKL_INT,
        isplit: *mut MKL_INT,
        m: *mut MKL_INT,
        w: *mut f32,
        werr: *mut f32,
        wgap: *mut f32,
        iblock: *mut MKL_INT,
        indexw: *mut MKL_INT,
        gers: *mut f32,
        pivmin: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarrf_(
        n: *const MKL_INT,
        d: *const f32,
        l: *const f32,
        ld: *const f32,
        clstrt: *const MKL_INT,
        clend: *const MKL_INT,
        w: *const f32,
        wgap: *mut f32,
        werr: *const f32,
        spdiam: *const f32,
        clgapl: *const f32,
        clgapr: *mut f32,
        pivmin: *const f32,
        sigma: *mut f32,
        dplus: *mut f32,
        lplus: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slarrj_(
        n: *const MKL_INT,
        d: *const f32,
        e2: *const f32,
        ifirst: *const MKL_INT,
        ilast: *const MKL_INT,
        rtol: *const f32,
        offset: *const MKL_INT,
        w: *mut f32,
        werr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        pivmin: *const f32,
        spdiam: *const f32,
        info: *mut MKL_INT,
    );
    pub fn slarrk_(
        n: *const MKL_INT,
        iw: *const MKL_INT,
        gl: *const f32,
        gu: *const f32,
        d: *const f32,
        e2: *const f32,
        pivmin: *const f32,
        reltol: *const f32,
        w: *mut f32,
        werr: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slarrr_(n: *const MKL_INT, d: *const f32, e: *mut f32, info: *mut MKL_INT);
    pub fn slarrv_(
        n: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        d: *mut f32,
        l: *mut f32,
        pivmin: *mut f32,
        isplit: *const MKL_INT,
        m: *const MKL_INT,
        dol: *const MKL_INT,
        dou: *const MKL_INT,
        minrgp: *const f32,
        rtol1: *const f32,
        rtol2: *const f32,
        w: *mut f32,
        werr: *mut f32,
        wgap: *mut f32,
        iblock: *const MKL_INT,
        indexw: *const MKL_INT,
        gers: *const f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slarscl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f32,
        x: *mut f32,
        ldx: *const MKL_INT,
    );
    pub fn slartg_(f: *const f32, g: *const f32, cs: *mut f32, sn: *mut f32, r: *mut f32);
    pub fn slartv_(
        n: *const MKL_INT,
        x: *mut f32,
        incx: *const MKL_INT,
        y: *mut f32,
        incy: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        incc: *const MKL_INT,
    );
    pub fn slaruv_(iseed: *mut MKL_INT, n: *const MKL_INT, x: *mut f32);
    pub fn slarzb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        ldwork: *const MKL_INT,
    );
    pub fn slarz_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slarzt_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        tau: *const f32,
        t: *mut f32,
        ldt: *const MKL_INT,
    );
    pub fn slas2_(f: *const f32, g: *const f32, h: *const f32, ssmin: *mut f32, ssmax: *mut f32);
    pub fn slascl_(
        type_: *const c_char,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        cfrom: *const f32,
        cto: *const f32,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slascl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f32,
        x: *mut f32,
        ldx: *const MKL_INT,
    );
    pub fn slasd0_(
        n: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f32,
        e: *const f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        smlsiz: *const MKL_INT,
        iwork: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasd1_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f32,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        idxq: *mut MKL_INT,
        iwork: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasd2_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *mut MKL_INT,
        d: *mut f32,
        z: *mut f32,
        alpha: *const f32,
        beta: *const f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        dsigma: *mut f32,
        u2: *mut f32,
        ldu2: *const MKL_INT,
        vt2: *mut f32,
        ldvt2: *const MKL_INT,
        idxp: *mut MKL_INT,
        idx: *mut MKL_INT,
        idxc: *mut MKL_INT,
        idxq: *mut MKL_INT,
        coltyp: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slasd3_(
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *const MKL_INT,
        d: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        dsigma: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        u2: *const f32,
        ldu2: *const MKL_INT,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        vt2: *mut f32,
        ldvt2: *const MKL_INT,
        idxc: *const MKL_INT,
        ctot: *const MKL_INT,
        z: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasd4_(
        n: *const MKL_INT,
        i: *const MKL_INT,
        d: *const f32,
        z: *const f32,
        delta: *mut f32,
        rho: *const f32,
        sigma: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasd5_(
        i: *const MKL_INT,
        d: *const f32,
        z: *const f32,
        delta: *mut f32,
        rho: *const f32,
        dsigma: *mut f32,
        work: *mut f32,
    );
    pub fn slasd6_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f32,
        vf: *mut f32,
        vl: *mut f32,
        alpha: *mut f32,
        beta: *mut f32,
        idxq: *mut MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *mut f32,
        ldgnum: *const MKL_INT,
        poles: *mut f32,
        difl: *mut f32,
        difr: *mut f32,
        z: *mut f32,
        k: *mut MKL_INT,
        c: *mut f32,
        s: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slasd7_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        k: *mut MKL_INT,
        d: *mut f32,
        z: *mut f32,
        zw: *mut f32,
        vf: *mut f32,
        vfw: *mut f32,
        vl: *mut f32,
        vlw: *mut f32,
        alpha: *const f32,
        beta: *const f32,
        dsigma: *mut f32,
        idx: *mut MKL_INT,
        idxp: *mut MKL_INT,
        idxq: *const MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *mut f32,
        ldgnum: *const MKL_INT,
        c: *mut f32,
        s: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasd8_(
        icompq: *const MKL_INT,
        k: *const MKL_INT,
        d: *mut f32,
        z: *mut f32,
        vf: *mut f32,
        vl: *mut f32,
        difl: *mut f32,
        difr: *mut f32,
        lddifr: *const MKL_INT,
        dsigma: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasda_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        sqre: *const MKL_INT,
        d: *mut f32,
        e: *const f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        vt: *mut f32,
        k: *mut MKL_INT,
        difl: *mut f32,
        difr: *mut f32,
        z: *mut f32,
        poles: *mut f32,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *mut MKL_INT,
        givnum: *mut f32,
        c: *mut f32,
        s: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slasdq_(
        uplo: *const c_char,
        sqre: *const MKL_INT,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vt: *mut f32,
        ldvt: *const MKL_INT,
        u: *mut f32,
        ldu: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasdt_(
        n: *const MKL_INT,
        lvl: *mut MKL_INT,
        nd: *mut MKL_INT,
        inode: *mut MKL_INT,
        ndiml: *mut MKL_INT,
        ndimr: *mut MKL_INT,
        msub: *const MKL_INT,
    );
    pub fn slaset_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        beta: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn slasq1_(n: *const MKL_INT, d: *mut f32, e: *mut f32, work: *mut f32, info: *mut MKL_INT);
    pub fn slasq2_(n: *const MKL_INT, z: *mut f32, info: *mut MKL_INT);
    pub fn slasq3_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f32,
        pp: *mut MKL_INT,
        dmin: *mut f32,
        sigma: *mut f32,
        desig: *mut f32,
        qmax: *const f32,
        nfail: *mut MKL_INT,
        iter: *mut MKL_INT,
        ndiv: *mut MKL_INT,
        ieee: *const MKL_INT,
        ttype: *mut MKL_INT,
        dmin1: *mut f32,
        dmin2: *mut f32,
        dn: *mut f32,
        dn1: *mut f32,
        dn2: *mut f32,
        g: *mut f32,
        tau: *mut f32,
    );
    pub fn slasq4_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f32,
        pp: *const MKL_INT,
        n0in: *mut MKL_INT,
        dmin: *const f32,
        dmin1: *const f32,
        dmin2: *const f32,
        dn: *const f32,
        dn1: *const f32,
        dn2: *const f32,
        tau: *mut f32,
        ttype: *mut MKL_INT,
        g: *mut f32,
    );
    pub fn slasq5_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f32,
        pp: *const MKL_INT,
        tau: *const f32,
        sigma: *const f32,
        dmin: *mut f32,
        dmin1: *mut f32,
        dmin2: *mut f32,
        dn: *mut f32,
        dnm1: *mut f32,
        dnm2: *mut f32,
        ieee: *const MKL_INT,
        eps: *const f32,
    );
    pub fn slasq6_(
        i0: *const MKL_INT,
        n0: *const MKL_INT,
        z: *const f32,
        pp: *const MKL_INT,
        dmin: *mut f32,
        dmin1: *mut f32,
        dmin2: *mut f32,
        dn: *mut f32,
        dnm1: *mut f32,
        dnm2: *mut f32,
    );
    pub fn slasr_(
        side: *const c_char,
        pivot: *const c_char,
        direct: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
    );
    pub fn slasrt_(id: *const c_char, n: *const MKL_INT, d: *mut f32, info: *mut MKL_INT);
    pub fn slassq_(
        n: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        scale: *mut f32,
        sumsq: *mut f32,
    );
    pub fn slasv2_(
        f: *const f32,
        g: *const f32,
        h: *const f32,
        ssmin: *mut f32,
        ssmax: *mut f32,
        snr: *mut f32,
        csr: *mut f32,
        snl: *mut f32,
        csl: *mut f32,
    );
    pub fn slaswp_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        k1: *const MKL_INT,
        k2: *const MKL_INT,
        ipiv: *const MKL_INT,
        incx: *const MKL_INT,
    );
    pub fn slasy2_(
        ltranl: *const MKL_INT,
        ltranr: *const MKL_INT,
        isgn: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        tl: *const f32,
        ldtl: *const MKL_INT,
        tr: *const f32,
        ldtr: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        scale: *mut f32,
        x: *mut f32,
        ldx: *const MKL_INT,
        xnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slasyf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut f32,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slatbs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        x: *mut f32,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slatdf_(
        ijob: *const MKL_INT,
        n: *const MKL_INT,
        z: *const f32,
        ldz: *const MKL_INT,
        rhs: *mut f32,
        rdsum: *mut f32,
        rdscal: *mut f32,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
    );
    pub fn slatps_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        x: *mut f32,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slatrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        e: *mut f32,
        tau: *mut f32,
        w: *mut f32,
        ldw: *const MKL_INT,
    );
    pub fn slatrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        scale: *mut f32,
        cnorm: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn slatrs3_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        scale: *mut f32,
        cnorm: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slatrz_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
    );
    pub fn slatzm_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const f32,
        incv: *const MKL_INT,
        tau: *const f32,
        c1: *mut f32,
        c2: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
    );
    pub fn slauu2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slauum_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sopgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        tau: *const f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sopmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ap: *const f32,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorg2l_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorg2r_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorgbr_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorghr_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorgl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorglq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorgql_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorgqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorgr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorgrq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *const f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorm2l_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sorm2r_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sormbr_(
        vect: *const c_char,
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormhr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sorml2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sormlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormql_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormr2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sormr3_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sormrq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormrz_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sormtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        tau: *const f32,
        c: *mut f32,
        ldc: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spbrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbstf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        afb: *mut f32,
        ldafb: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbtf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbtrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spbtrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spftrf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spftri_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spftrs_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spocon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spoequb_(
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spoequ_(
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sporfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sporfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        s: *mut f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sposvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sposvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spotf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spotrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spotri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spotrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sppcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sppequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        afp: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sppsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sppsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f32,
        afp: *mut f32,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spptrf_(uplo: *const c_char, n: *const MKL_INT, ap: *mut f32, info: *mut MKL_INT);
    pub fn spptri_(uplo: *const c_char, n: *const MKL_INT, ap: *mut f32, info: *mut MKL_INT);
    pub fn spptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn spstf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spstrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sptcon_(
        n: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sptrfs_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        df: *const f32,
        ef: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sptsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sptsvx_(
        fact: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        df: *mut f32,
        ef: *mut f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn spttrf_(n: *const MKL_INT, d: *mut f32, e: *mut f32, info: *mut MKL_INT);
    pub fn spttrs_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sptts2_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
    );
    pub fn srscl_(n: *const MKL_INT, sa: *const f32, sx: *mut f32, incx: *const MKL_INT);
    pub fn ssbevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssbevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbgst_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        bb: *const f32,
        ldbb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssbgvd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        bb: *mut f32,
        ldbb: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbgv_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        bb: *mut f32,
        ldbb: *const MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssbgvx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        bb: *mut f32,
        ldbb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssbtrd_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut f32,
        ldab: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssfrk_(
        transr: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f32,
        a: *const f32,
        lda: *const MKL_INT,
        beta: *const f32,
        c: *mut f32,
    );
    pub fn sspcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sspevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspgst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        bp: *const f32,
        info: *mut MKL_INT,
    );
    pub fn sspgvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        bp: *mut f32,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspgv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        bp: *mut f32,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sspgvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        bp: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        afp: *const f32,
        ipiv: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut f32,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sspsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        afp: *mut f32,
        ipiv: *mut MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssptrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        ipiv: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstebz_(
        range: *const c_char,
        order: *const c_char,
        n: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        d: *const f32,
        e: *const f32,
        m: *mut MKL_INT,
        nsplit: *mut MKL_INT,
        w: *mut f32,
        iblock: *mut MKL_INT,
        isplit: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstedc_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstegr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstein_(
        n: *const MKL_INT,
        d: *const f32,
        e: *const f32,
        m: *const MKL_INT,
        w: *const f32,
        iblock: *const MKL_INT,
        isplit: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstemr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        nzc: *const MKL_INT,
        isuppz: *mut MKL_INT,
        tryrac: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssterf_(n: *const MKL_INT, d: *mut f32, e: *mut f32, info: *mut MKL_INT);
    pub fn sstevd_(
        jobz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstev_(
        jobz: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn sstevr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sstevx_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssycon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        s: *mut f32,
        scond: *mut f32,
        amax: *mut f32,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssyevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyevr_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygs2_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        w: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssygvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        vl: *const f32,
        vu: *const f32,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f32,
        m: *mut MKL_INT,
        w: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyrfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssysvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        af: *mut f32,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        x: *mut f32,
        ldx: *const MKL_INT,
        rcond: *mut f32,
        rpvgrw: *mut f32,
        berr: *mut f32,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        nparams: *const MKL_INT,
        params: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytd2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssytf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssytri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn ssytrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stbcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stbrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *const f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stbtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stfsm_(
        transr: *const c_char,
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        a: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
    );
    pub fn stftri_(
        transr: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stfttp_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const f32,
        ap: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stfttr_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        s: *const f32,
        lds: *const MKL_INT,
        p: *const f32,
        ldp: *const MKL_INT,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stgex2_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        j1: *const MKL_INT,
        n1: *const MKL_INT,
        n2: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgexc_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        ifst: *mut MKL_INT,
        ilst: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgsen_(
        ijob: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        alphar: *mut f32,
        alphai: *mut f32,
        beta: *mut f32,
        q: *mut f32,
        ldq: *const MKL_INT,
        z: *mut f32,
        ldz: *const MKL_INT,
        m: *mut MKL_INT,
        pl: *mut f32,
        pr: *mut f32,
        dif: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgsja_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        tola: *const f32,
        tolb: *const f32,
        alpha: *mut f32,
        beta: *mut f32,
        u: *mut f32,
        ldu: *const MKL_INT,
        v: *mut f32,
        ldv: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        work: *mut f32,
        ncycle: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        vl: *const f32,
        ldvl: *const MKL_INT,
        vr: *const f32,
        ldvr: *const MKL_INT,
        s: *mut f32,
        dif: *mut f32,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgsy2_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        d: *const f32,
        ldd: *const MKL_INT,
        e: *const f32,
        lde: *const MKL_INT,
        f: *mut f32,
        ldf: *const MKL_INT,
        scale: *mut f32,
        rdsum: *mut f32,
        rdscal: *mut f32,
        iwork: *mut MKL_INT,
        pq: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stgsyl_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        d: *const f32,
        ldd: *const MKL_INT,
        e: *const f32,
        lde: *const MKL_INT,
        f: *mut f32,
        ldf: *const MKL_INT,
        scale: *mut f32,
        dif: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stpcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stprfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *const f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stptri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stptrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn stpttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        arf: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stpttr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        rcond: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *mut MKL_INT,
        n: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        vl: *mut f32,
        ldvl: *const MKL_INT,
        vr: *mut f32,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn strexc_(
        compq: *const c_char,
        n: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        ifst: *mut MKL_INT,
        ilst: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn strrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        x: *const f32,
        ldx: *const MKL_INT,
        ferr: *mut f32,
        berr: *mut f32,
        work: *mut f32,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strsen_(
        job: *const c_char,
        compq: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        q: *mut f32,
        ldq: *const MKL_INT,
        wr: *mut f32,
        wi: *mut f32,
        m: *mut MKL_INT,
        s: *mut f32,
        sep: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        vl: *const f32,
        ldvl: *const MKL_INT,
        vr: *const f32,
        ldvr: *const MKL_INT,
        s: *mut f32,
        sep: *mut f32,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut f32,
        ldwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strsyl_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        scale: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn strsyl3_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        swork: *mut f32,
        ldswork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strti2_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strtri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn strttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        arf: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn strttp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        ap: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stzrqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn stzrzf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        tau: *mut f32,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn xerbla_array_(
        srname_array: *const c_char,
        srname_len: *const MKL_INT,
        info: *const MKL_INT,
    );
    pub fn zbdsqr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ncvt: *const MKL_INT,
        nru: *const MKL_INT,
        ncc: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vt: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zcgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        work: *mut MKL_Complex16,
        swork: *mut MKL_Complex8,
        rwork: *mut f64,
        iter: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zcposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        work: *mut MKL_Complex16,
        swork: *mut MKL_Complex8,
        rwork: *mut f64,
        iter: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zdrscl_(n: *const MKL_INT, sa: *const f64, sx: *mut MKL_Complex16, incx: *const MKL_INT);
    pub fn zgbbrd_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ncc: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        pt: *mut MKL_Complex16,
        ldpt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbrfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbsv_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgbsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbsvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgbtf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgbtrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgbtrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgebak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        scale: *const f64,
        m: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgebal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgebd2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut MKL_Complex16,
        taup: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgebrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut MKL_Complex16,
        taup: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgecon_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgedmd_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f64,
        k: *mut MKL_INT,
        eigs: *mut MKL_Complex16,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        res: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        s: *mut MKL_Complex16,
        lds: *const MKL_INT,
        zwork: *mut MKL_Complex16,
        lzwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgedmdq_(
        jobs: *const c_char,
        jobz: *const c_char,
        jobr: *const c_char,
        jobq: *const c_char,
        jobt: *const c_char,
        jobf: *const c_char,
        whtsvd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        f: *mut MKL_Complex16,
        ldf: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        nrnk: *const MKL_INT,
        tol: *const f64,
        k: *mut MKL_INT,
        eigs: *mut MKL_Complex16,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        res: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        s: *mut MKL_Complex16,
        lds: *const MKL_INT,
        zwork: *mut MKL_Complex16,
        lzwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeequb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgeequ_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        r: *mut f64,
        c: *mut f64,
        rowcnd: *mut f64,
        colcnd: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgees_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_Z_SELECT_FUNCTION_1,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        w: *mut MKL_Complex16,
        vs: *mut MKL_Complex16,
        ldvs: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeesx_(
        jobvs: *const c_char,
        sort: *const c_char,
        select: MKL_Z_SELECT_FUNCTION_1,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        sdim: *mut MKL_INT,
        w: *mut MKL_Complex16,
        vs: *mut MKL_Complex16,
        ldvs: *const MKL_INT,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgeevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        scale: *mut f64,
        abnrm: *mut f64,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgegs_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vsl: *mut MKL_Complex16,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex16,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgegv_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgehd2_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgehrd_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgelq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgelqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgelsd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        s: *mut f64,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgels_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgelst_(
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgelss_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        s: *mut f64,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgelsx_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgelsy_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        jpvt: *mut MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgeql2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgeqlf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeqp3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgeqp3rk_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        kmax: *const MKL_INT,
        abstol: *const f64,
        reltol: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        k: *mut MKL_INT,
        maxc2nrmk: *mut f64,
        relmaxc2nrmk: *mut f64,
        jpiv: *mut MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeqpf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgeqr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgeqr2p_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgeqrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgeqrfp_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgerfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgerfsx_(
        trans: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgerq2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgerqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesc2_(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        rhs: *mut MKL_Complex16,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
        scale: *mut f64,
    );
    pub fn zgesdd_(
        jobz: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvd_(
        jobu: *const c_char,
        jobvt: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        vt: *mut MKL_Complex16,
        ldvt: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgesv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgesvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgesvxx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        r: *mut f64,
        c: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgetc2_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        jpiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_zgetrfnpi_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nfact: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetri_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgetrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggbak_(
        job: *const c_char,
        side: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        lscale: *const f64,
        rscale: *const f64,
        m: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggbal_(
        job: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f64,
        rscale: *mut f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgges_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_Z_SELECT_FUNCTION_2,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vsl: *mut MKL_Complex16,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex16,
        ldvsr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggesx_(
        jobvsl: *const c_char,
        jobvsr: *const c_char,
        sort: *const c_char,
        selctg: MKL_Z_SELECT_FUNCTION_2,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        sdim: *mut MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vsl: *mut MKL_Complex16,
        ldvsl: *const MKL_INT,
        vsr: *mut MKL_Complex16,
        ldvsr: *const MKL_INT,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggev_(
        jobvl: *const c_char,
        jobvr: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zggevx_(
        balanc: *const c_char,
        jobvl: *const c_char,
        jobvr: *const c_char,
        sense: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        ilo: *mut MKL_INT,
        ihi: *mut MKL_INT,
        lscale: *mut f64,
        rscale: *mut f64,
        abnrm: *mut f64,
        bbnrm: *mut f64,
        rconde: *mut f64,
        rcondv: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        bwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggglm_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        d: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        y: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgghrd_(
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgglse_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggqrf_(
        n: *const MKL_INT,
        m: *const MKL_INT,
        p: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        taua: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        taub: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggrqf_(
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        taua: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        taub: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggsvd_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        p: *const MKL_INT,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zggsvp_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        k: *mut MKL_INT,
        l: *mut MKL_INT,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        iwork: *mut MKL_INT,
        rwork: *mut f64,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgtcon_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        du2: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zgtrfs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        dlf: *const MKL_Complex16,
        df: *const MKL_Complex16,
        duf: *const MKL_Complex16,
        du2: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgtsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
        du: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgtsvx_(
        fact: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        dlf: *mut MKL_Complex16,
        df: *mut MKL_Complex16,
        duf: *mut MKL_Complex16,
        du2: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zgttrf_(
        n: *const MKL_INT,
        dl: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
        du: *mut MKL_Complex16,
        du2: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgttrs_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        du2: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zgtts2_(
        itrans: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        du2: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn zhbevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhbev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhbevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhbgst_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        bb: *const MKL_Complex16,
        ldbb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhbgvd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex16,
        ldbb: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhbgv_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex16,
        ldbb: *const MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhbgvx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ka: *const MKL_INT,
        kb: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        bb: *mut MKL_Complex16,
        ldbb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhbtrd_(
        vect: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhecon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zheequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zheevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zheevr_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhegs2_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhegst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhegvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhegv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        w: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhegvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zherfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zherfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhesv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhesvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhesvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhetd2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhetf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhetrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhfrk_(
        transr: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        alpha: *const f64,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        beta: *const f64,
        c: *mut MKL_Complex16,
    );
    pub fn zhgeqz_(
        job: *const c_char,
        compq: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhpcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhpevd_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhpev_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhpevx_(
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhpgst_(
        itype: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        bp: *const MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhpgvd_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        bp: *mut MKL_Complex16,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhpgv_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        bp: *mut MKL_Complex16,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhpgvx_(
        itype: *const MKL_INT,
        jobz: *const c_char,
        range: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        bp: *mut MKL_Complex16,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        afp: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhpsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhpsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        afp: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zhptrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        d: *mut f64,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zhptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhsein_(
        side: *const c_char,
        eigsrc: *const c_char,
        initv: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        h: *const MKL_Complex16,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex16,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        ifaill: *mut MKL_INT,
        ifailr: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhseqr_(
        job: *const c_char,
        compz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex16,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlabrd_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tauq: *mut MKL_Complex16,
        taup: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
    );
    pub fn zlacgv_(n: *const MKL_INT, x: *mut MKL_Complex16, incx: *const MKL_INT);
    pub fn zlacn2_(
        n: *const MKL_INT,
        v: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        est: *mut f64,
        kase: *mut MKL_INT,
        isave: *mut MKL_INT,
    );
    pub fn zlacon_(
        n: *const MKL_INT,
        v: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        est: *mut f64,
        kase: *mut MKL_INT,
    );
    pub fn zlacp2_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn zlacpy_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn zlacrm_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const f64,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        rwork: *mut f64,
    );
    pub fn zlacrt_(
        n: *const MKL_INT,
        cx: *mut MKL_Complex16,
        incx: *const MKL_INT,
        cy: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const MKL_Complex16,
        s: *const MKL_Complex16,
    );
    pub fn zladiv_(ret_value: *mut MKL_Complex16, x: *const MKL_Complex16, y: *const MKL_Complex16);
    pub fn zlaed0_(
        qsiz: *const MKL_INT,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        qstore: *mut MKL_Complex16,
        ldqs: *const MKL_INT,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaed7_(
        n: *const MKL_INT,
        cutpnt: *const MKL_INT,
        qsiz: *const MKL_INT,
        tlvls: *const MKL_INT,
        curlvl: *const MKL_INT,
        curpbm: *const MKL_INT,
        d: *mut f64,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        rho: *const f64,
        indxq: *mut MKL_INT,
        qstore: *mut f64,
        qptr: *mut MKL_INT,
        prmptr: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        givnum: *const f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaed8_(
        k: *mut MKL_INT,
        n: *const MKL_INT,
        qsiz: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        d: *mut f64,
        rho: *mut f64,
        cutpnt: *const MKL_INT,
        z: *const f64,
        dlamda: *mut f64,
        q2: *mut MKL_Complex16,
        ldq2: *const MKL_INT,
        w: *mut f64,
        indxp: *mut MKL_INT,
        indx: *mut MKL_INT,
        indxq: *const MKL_INT,
        perm: *mut MKL_INT,
        givptr: *mut MKL_INT,
        givcol: *mut MKL_INT,
        givnum: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zlaein_(
        rightv: *const MKL_INT,
        noinit: *const MKL_INT,
        n: *const MKL_INT,
        h: *const MKL_Complex16,
        ldh: *const MKL_INT,
        w: *const MKL_Complex16,
        v: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        rwork: *mut f64,
        eps3: *const f64,
        smlnum: *const f64,
        info: *mut MKL_INT,
    );
    pub fn zlaesy_(
        a: *const MKL_Complex16,
        b: *const MKL_Complex16,
        c: *const MKL_Complex16,
        rt1: *mut MKL_Complex16,
        rt2: *mut MKL_Complex16,
        evscal: *mut MKL_Complex16,
        cs1: *mut MKL_Complex16,
        sn1: *mut MKL_Complex16,
    );
    pub fn zlaev2_(
        a: *const MKL_Complex16,
        b: *const MKL_Complex16,
        c: *const MKL_Complex16,
        rt1: *mut f64,
        rt2: *mut f64,
        cs1: *mut f64,
        sn1: *mut MKL_Complex16,
    );
    pub fn zlag2c_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        sa: *mut MKL_Complex8,
        ldsa: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlags2_(
        upper: *const MKL_INT,
        a1: *const f64,
        a2: *const MKL_Complex16,
        a3: *const f64,
        b1: *const f64,
        b2: *const MKL_Complex16,
        b3: *const f64,
        csu: *mut f64,
        snu: *mut MKL_Complex16,
        csv: *mut f64,
        snv: *mut MKL_Complex16,
        csq: *mut f64,
        snq: *mut MKL_Complex16,
    );
    pub fn zlagtm_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        alpha: *const f64,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        x: *const MKL_Complex16,
        ldx: *const MKL_INT,
        beta: *const f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn zlahef_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlahqr_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex16,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlahr2_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
    );
    pub fn zlahrd_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
    );
    pub fn zlaic1_(
        job: *const MKL_INT,
        j: *const MKL_INT,
        x: *const MKL_Complex16,
        sest: *const f64,
        w: *const MKL_Complex16,
        gamma: *const MKL_Complex16,
        sestpr: *mut f64,
        s: *mut MKL_Complex16,
        c: *mut MKL_Complex16,
    );
    pub fn zlals0_(
        icompq: *const MKL_INT,
        nl: *const MKL_INT,
        nr: *const MKL_INT,
        sqre: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        bx: *mut MKL_Complex16,
        ldbx: *const MKL_INT,
        perm: *const MKL_INT,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        givnum: *const f64,
        ldgnum: *const MKL_INT,
        poles: *const f64,
        difl: *const f64,
        difr: *const f64,
        z: *const f64,
        k: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zlalsa_(
        icompq: *const MKL_INT,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        bx: *mut MKL_Complex16,
        ldbx: *const MKL_INT,
        u: *const f64,
        ldu: *const MKL_INT,
        vt: *const f64,
        k: *const MKL_INT,
        difl: *const f64,
        difr: *const f64,
        z: *const f64,
        poles: *const f64,
        givptr: *const MKL_INT,
        givcol: *const MKL_INT,
        ldgcol: *const MKL_INT,
        perm: *const MKL_INT,
        givnum: *const f64,
        c: *const f64,
        s: *const f64,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlalsd_(
        uplo: *const c_char,
        smlsiz: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        rcond: *const f64,
        rank: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlangb_(
        norm: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlange_(
        norm: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlangt_(
        norm: *const c_char,
        n: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
    ) -> f64;
    pub fn zlanhb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlanhe_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlanhf_(
        norm: *const c_char,
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        work: *mut f64,
    ) -> f64;
    pub fn zlanhp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        work: *mut f64,
    ) -> f64;
    pub fn zlanhs_(
        norm: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlanht_(
        norm: *const c_char,
        n: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
    ) -> f64;
    pub fn zlansb_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlansp_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        work: *mut f64,
    ) -> f64;
    pub fn zlansy_(
        norm: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlantb_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlantp_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        work: *mut f64,
    ) -> f64;
    pub fn zlantr_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zlapll_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        ssmin: *mut f64,
    );
    pub fn zlapmt_(
        forwrd: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        k: *mut MKL_INT,
    );
    pub fn zlaqgb_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        rowcnd: *const f64,
        colcnd: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        r: *const f64,
        c: *const f64,
        rowcnd: *const f64,
        colcnd: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqhb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        s: *mut f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqhe_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqhp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqp2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex16,
        vn1: *mut f64,
        vn2: *mut f64,
        work: *mut MKL_Complex16,
    );
    pub fn zlaqps_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        offset: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        jpvt: *mut MKL_INT,
        tau: *mut MKL_Complex16,
        vn1: *mut f64,
        vn2: *mut f64,
        auxv: *mut MKL_Complex16,
        f: *mut MKL_Complex16,
        ldf: *const MKL_INT,
    );
    pub fn zlaqr0_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex16,
        iloz: *mut MKL_INT,
        ihiz: *mut MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaqr1_(
        n: *const MKL_INT,
        h: *const MKL_Complex16,
        ldh: *const MKL_INT,
        s1: *const MKL_Complex16,
        s2: *mut MKL_Complex16,
        v: *mut MKL_Complex16,
    );
    pub fn zlaqr2_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sh: *mut MKL_Complex16,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex16,
        ldwv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
    );
    pub fn zlaqr3_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nw: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        sh: *mut MKL_Complex16,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        nh: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex16,
        ldwv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
    );
    pub fn zlaqr4_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        w: *mut MKL_Complex16,
        iloz: *mut MKL_INT,
        ihiz: *mut MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaqr5_(
        wantt: *const MKL_INT,
        wantz: *const MKL_INT,
        kacc22: *const MKL_INT,
        n: *const MKL_INT,
        ktop: *const MKL_INT,
        kbot: *const MKL_INT,
        nshfts: *const MKL_INT,
        s: *mut MKL_Complex16,
        h: *mut MKL_Complex16,
        ldh: *const MKL_INT,
        iloz: *const MKL_INT,
        ihiz: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        nv: *const MKL_INT,
        wv: *mut MKL_Complex16,
        ldwv: *const MKL_INT,
        nh: *const MKL_INT,
        wh: *mut MKL_Complex16,
        ldwh: *const MKL_INT,
    );
    pub fn zlaqsb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqsp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqsy_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        s: *const f64,
        scond: *const f64,
        amax: *const f64,
        equed: *mut c_char,
    );
    pub fn zlaqz0_(
        wants: *const c_char,
        wantq: *const c_char,
        wantz: *const c_char,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f32,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaqz1_(
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        k: *const MKL_INT,
        istartm: *const MKL_INT,
        istopm: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        nq: *const MKL_INT,
        qstart: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        nz: *const MKL_INT,
        zstart: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
    );
    pub fn zlaqz2_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nw: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        ns: *mut MKL_INT,
        nd: *mut MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        qc: *mut MKL_Complex16,
        ldqc: *const MKL_INT,
        zc: *mut MKL_Complex16,
        ldzc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        rec: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlaqz3_(
        ilschur: *const MKL_INT,
        ilq: *const MKL_INT,
        ilz: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        nshifts: *const MKL_INT,
        nb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        qc: *mut MKL_Complex16,
        ldqc: *const MKL_INT,
        zc: *mut MKL_Complex16,
        ldzc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlar1v_(
        n: *const MKL_INT,
        b1: *const MKL_INT,
        bn: *const MKL_INT,
        lambda: *const f64,
        d: *const f64,
        l: *const f64,
        ld: *const f64,
        lld: *const f64,
        pivmin: *const f64,
        gaptol: *const f64,
        z: *mut MKL_Complex16,
        wantnc: *const MKL_INT,
        negcnt: *mut MKL_INT,
        ztz: *mut f64,
        mingma: *mut f64,
        r: *mut MKL_INT,
        isuppz: *mut MKL_INT,
        nrminv: *mut f64,
        resid: *mut f64,
        rqcorr: *mut f64,
        work: *mut f64,
    );
    pub fn zlar2v_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        y: *mut MKL_Complex16,
        z: *mut MKL_Complex16,
        incx: *const MKL_INT,
        c: *const f64,
        s: *const MKL_Complex16,
        incc: *const MKL_INT,
    );
    pub fn zlarcm_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        rwork: *mut f64,
    );
    pub fn zlarfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        ldwork: *const MKL_INT,
    );
    pub fn zlarf_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlarf1f_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlarf1l_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlarfg_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex16,
    );
    pub fn zlarfgp_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex16,
    );
    pub fn zlarfp_(
        n: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        tau: *mut MKL_Complex16,
    );
    pub fn zlarft_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        tau: *const MKL_Complex16,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
    );
    pub fn zlarfx_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlargv_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *mut f64,
        incc: *const MKL_INT,
    );
    pub fn zlarnv_(
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
    );
    pub fn zlarrv_(
        n: *const MKL_INT,
        vl: *const f64,
        vu: *const f64,
        d: *mut f64,
        l: *mut f64,
        pivmin: *mut f64,
        isplit: *const MKL_INT,
        m: *const MKL_INT,
        dol: *const MKL_INT,
        dou: *const MKL_INT,
        minrgp: *const f64,
        rtol1: *const f64,
        rtol2: *const f64,
        w: *mut f64,
        werr: *mut f64,
        wgap: *mut f64,
        iblock: *const MKL_INT,
        indexw: *const MKL_INT,
        gers: *const f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlarscl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f64,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
    );
    pub fn zlartg_(
        f: *const MKL_Complex16,
        g: *const MKL_Complex16,
        cs: *mut f64,
        sn: *mut MKL_Complex16,
        r: *mut MKL_Complex16,
    );
    pub fn zlartv_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        incx: *const MKL_INT,
        y: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const MKL_Complex16,
        incc: *const MKL_INT,
    );
    pub fn zlarzb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        ldwork: *const MKL_INT,
    );
    pub fn zlarz_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlarzt_(
        direct: *const c_char,
        storev: *const c_char,
        n: *const MKL_INT,
        k: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        tau: *const MKL_Complex16,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
    );
    pub fn zlascl_(
        type_: *const c_char,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        cfrom: *const f64,
        cto: *const f64,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlascl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        d: *const f64,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
    );
    pub fn zlaset_(
        uplo: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zlasr_(
        side: *const c_char,
        pivot: *const c_char,
        direct: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zlassq_(
        n: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        scale: *mut f64,
        sumsq: *mut f64,
    );
    pub fn zlaswp_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        k1: *const MKL_INT,
        k2: *const MKL_INT,
        ipiv: *const MKL_INT,
        incx: *const MKL_INT,
    );
    pub fn zlasyf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        kb: *mut MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlat2c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        sa: *mut MKL_Complex8,
        ldsa: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlatbs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        x: *mut MKL_Complex16,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zlatdf_(
        ijob: *const MKL_INT,
        n: *const MKL_INT,
        z: *const MKL_Complex16,
        ldz: *const MKL_INT,
        rhs: *mut MKL_Complex16,
        rdsum: *mut f64,
        rdscal: *mut f64,
        ipiv: *const MKL_INT,
        jpiv: *const MKL_INT,
    );
    pub fn zlatps_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zlatrd_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nb: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        w: *mut MKL_Complex16,
        ldw: *const MKL_INT,
    );
    pub fn zlatrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        scale: *mut f64,
        cnorm: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zlatrs3_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        normin: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        scale: *mut f64,
        cnorm: *mut f64,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlatrz_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
    );
    pub fn zlatzm_(
        side: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        v: *const MKL_Complex16,
        incv: *const MKL_INT,
        tau: *const MKL_Complex16,
        c1: *mut MKL_Complex16,
        c2: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
    );
    pub fn zlauu2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlauum_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpbcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpbequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpbrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpbstf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpbsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpbsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *mut MKL_Complex16,
        ldafb: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpbtf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpbtrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *mut MKL_Complex16,
        ldab: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpbtrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpftrf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zpftri_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zpftrs_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpocon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpoequb_(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpoequ_(
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zporfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zporfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        s: *mut f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zposv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zposvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zposvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpotf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpotrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpotri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpotrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zppcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zppequ_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        afp: *const MKL_Complex16,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zppsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zppsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex16,
        afp: *mut MKL_Complex16,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zpptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zpptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zpstf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpstrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        piv: *mut MKL_INT,
        rank: *mut MKL_INT,
        tol: *const f64,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zptcon_(
        n: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
        anorm: *const f64,
        rcond: *mut f64,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zptrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
        df: *const f64,
        ef: *const MKL_Complex16,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zptsv_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *mut f64,
        e: *mut MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zptsvx_(
        fact: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
        df: *mut f64,
        ef: *mut MKL_Complex16,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zpttrf_(n: *const MKL_INT, d: *mut f64, e: *mut MKL_Complex16, info: *mut MKL_INT);
    pub fn zpttrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zptts2_(
        iuplo: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        d: *const f64,
        e: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn zrot_(
        n: *const MKL_INT,
        cx: *mut MKL_Complex16,
        incx: *const MKL_INT,
        cy: *mut MKL_Complex16,
        incy: *const MKL_INT,
        c: *const f64,
        s: *const MKL_Complex16,
    );
    pub fn zspcon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zspmv_(
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
    pub fn zspr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        ap: *mut MKL_Complex16,
    );
    pub fn zsprfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        afp: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zspsv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zspsvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        afp: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsptrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsptri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsptrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zstedc_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        lrwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zstegr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        abstol: *const f64,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        isuppz: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zstein_(
        n: *const MKL_INT,
        d: *const f64,
        e: *const f64,
        m: *const MKL_INT,
        w: *const f64,
        iblock: *const MKL_INT,
        isplit: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
        ifail: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zstemr_(
        jobz: *const c_char,
        range: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        vl: *const f64,
        vu: *const f64,
        il: *const MKL_INT,
        iu: *const MKL_INT,
        m: *mut MKL_INT,
        w: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        nzc: *const MKL_INT,
        isuppz: *mut MKL_INT,
        tryrac: *mut MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsteqr_(
        compz: *const c_char,
        n: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsycon_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        anorm: *const f64,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsyequb_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        s: *mut f64,
        scond: *mut f64,
        amax: *mut f64,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsymv_(
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
    pub fn zsyr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
    );
    pub fn zsyrfs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsyrfsx_(
        uplo: *const c_char,
        equed: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        s: *mut f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsysv_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsysvx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsysvxx_(
        fact: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        af: *mut MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *mut MKL_INT,
        equed: *mut c_char,
        s: *mut f64,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        rcond: *mut f64,
        rpvgrw: *mut f64,
        berr: *mut f64,
        n_err_bnds: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        nparams: *const MKL_INT,
        params: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn zsytf2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytrf_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zsytri_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zsytrs_(
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztbcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztbrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *const MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztbtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztfsm_(
        transr: *const c_char,
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const MKL_Complex16,
        a: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
    );
    pub fn ztftri_(
        transr: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztfttp_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const MKL_Complex16,
        ap: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztfttr_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        arf: *const MKL_Complex16,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        s: *const MKL_Complex16,
        lds: *const MKL_INT,
        p: *const MKL_Complex16,
        ldp: *const MKL_INT,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztgex2_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        j1: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgexc_(
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        ifst: *const MKL_INT,
        ilst: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgsen_(
        ijob: *const MKL_INT,
        wantq: *const MKL_INT,
        wantz: *const MKL_INT,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        alpha: *mut MKL_Complex16,
        beta: *mut MKL_Complex16,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        m: *mut MKL_INT,
        pl: *mut f64,
        pr: *mut f64,
        dif: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        liwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgsja_(
        jobu: *const c_char,
        jobv: *const c_char,
        jobq: *const c_char,
        m: *const MKL_INT,
        p: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        tola: *const f64,
        tolb: *const f64,
        alpha: *mut f64,
        beta: *mut f64,
        u: *mut MKL_Complex16,
        ldu: *const MKL_INT,
        v: *mut MKL_Complex16,
        ldv: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex16,
        ncycle: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        vl: *const MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *const MKL_Complex16,
        ldvr: *const MKL_INT,
        s: *mut f64,
        dif: *mut f64,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztgsy2_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        d: *const MKL_Complex16,
        ldd: *const MKL_INT,
        e: *const MKL_Complex16,
        lde: *const MKL_INT,
        f: *mut MKL_Complex16,
        ldf: *const MKL_INT,
        scale: *mut f64,
        rdsum: *mut f64,
        rdscal: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztgsyl_(
        trans: *const c_char,
        ijob: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        d: *const MKL_Complex16,
        ldd: *const MKL_INT,
        e: *const MKL_Complex16,
        lde: *const MKL_INT,
        f: *mut MKL_Complex16,
        ldf: *const MKL_INT,
        scale: *mut f64,
        dif: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztpcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztprfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *const MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztptri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztptrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        ap: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztpttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        arf: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztpttr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrcon_(
        norm: *const c_char,
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        rcond: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztrevc_(
        side: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        vl: *mut MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *mut MKL_Complex16,
        ldvr: *const MKL_INT,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztrexc_(
        compq: *const c_char,
        n: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        ifst: *const MKL_INT,
        ilst: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrrfs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        x: *const MKL_Complex16,
        ldx: *const MKL_INT,
        ferr: *mut f64,
        berr: *mut f64,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztrsen_(
        job: *const c_char,
        compq: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        w: *mut MKL_Complex16,
        m: *mut MKL_INT,
        s: *mut f64,
        sep: *mut f64,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrsna_(
        job: *const c_char,
        howmny: *const c_char,
        select: *const MKL_INT,
        n: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        vl: *const MKL_Complex16,
        ldvl: *const MKL_INT,
        vr: *const MKL_Complex16,
        ldvr: *const MKL_INT,
        s: *mut f64,
        sep: *mut f64,
        mm: *const MKL_INT,
        m: *mut MKL_INT,
        work: *mut MKL_Complex16,
        ldwork: *const MKL_INT,
        rwork: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztrsyl_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        scale: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn ztrsyl3_(
        trana: *const c_char,
        tranb: *const c_char,
        isgn: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        scale: *mut f64,
        swork: *mut f64,
        ldswork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrti2_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrtri_(
        uplo: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrtrs_(
        uplo: *const c_char,
        trans: *const c_char,
        diag: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ztrttf_(
        transr: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        arf: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztrttp_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        ap: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztzrqf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn ztzrzf_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *mut MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zung2l_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zung2r_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zungbr_(
        vect: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunghr_(
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungl2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunglq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungql_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungqr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungr2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zungrq_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zungtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunm2l_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunm2r_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunmbr_(
        vect: *const c_char,
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmhr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ilo: *const MKL_INT,
        ihi: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunml2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunmlq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmql_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmqr_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmr2_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunmr3_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zunmrq_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmrz_(
        side: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zunmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zupgtr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        tau: *const MKL_Complex16,
        q: *mut MKL_Complex16,
        ldq: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zupmtr_(
        side: *const c_char,
        uplo: *const c_char,
        trans: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        tau: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn cgeqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        t: *mut MKL_Complex8,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cheswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn chetri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn chetri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex8,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_gbamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        alpha: *const f32,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn cla_gbrcond_c_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f32,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_gbrcond_x_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex8,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_gbrfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut MKL_Complex8,
        ayb: *mut f32,
        dy: *mut MKL_Complex8,
        y_tail: *mut MKL_Complex8,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_gbrpvgrw_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ncols: *const MKL_INT,
        ab: *const MKL_Complex8,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex8,
        ldafb: *const MKL_INT,
    ) -> f32;
    pub fn cla_geamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn cla_gercond_c_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f32,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_gercond_x_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex8,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_gerfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        errs_n: *mut f32,
        errs_c: *mut f32,
        res: *mut MKL_Complex8,
        ayb: *mut f32,
        dy: *mut MKL_Complex8,
        y_tail: *mut MKL_Complex8,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_gerpvgrw_(
        n: *const MKL_INT,
        ncols: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
    ) -> f32;
    pub fn cla_heamv_(
        uplo: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn cla_hercond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f32,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_hercond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex8,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_herfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut MKL_Complex8,
        ayb: *mut f32,
        dy: *mut MKL_Complex8,
        y_tail: *mut MKL_Complex8,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_herpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn cla_lin_berr_(
        n: *const MKL_INT,
        nz: *const MKL_INT,
        nrhs: *const MKL_INT,
        res: *const MKL_Complex8,
        ayb: *const f32,
        berr: *mut f32,
    );
    pub fn cla_porcond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        c: *const f32,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_porcond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        x: *const MKL_Complex8,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_porfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut MKL_Complex8,
        ayb: *mut f32,
        dy: *mut MKL_Complex8,
        y_tail: *mut MKL_Complex8,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_porpvgrw_(
        uplo: *const c_char,
        ncols: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn cla_syamv_(
        uplo: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f32,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        x: *const MKL_Complex8,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn cla_syrcond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f32,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_syrcond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex8,
        info: *mut MKL_INT,
        work: *mut MKL_Complex8,
        rwork: *mut f32,
    ) -> f32;
    pub fn cla_syrfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const MKL_Complex8,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut MKL_Complex8,
        ayb: *mut f32,
        dy: *mut MKL_Complex8,
        y_tail: *mut MKL_Complex8,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cla_syrpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        af: *const MKL_Complex8,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn cla_wwaddw_(
        n: *const MKL_INT,
        x: *mut MKL_Complex8,
        y: *mut MKL_Complex8,
        w: *const MKL_Complex8,
    );
    pub fn ctprfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex8,
        ldv: *const MKL_INT,
        t: *const MKL_Complex8,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex8,
        ldwork: *const MKL_INT,
    );
    pub fn dgeqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        t: *mut f64,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dla_gbamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        alpha: *const f64,
        ab: *const f64,
        ldab: *const MKL_INT,
        x: *const f64,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn dla_gbrcond_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f64,
        info: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
    ) -> f64;
    pub fn dla_gbrfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut f64,
        ayb: *mut f64,
        dy: *mut f64,
        y_tail: *mut f64,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dla_gbrpvgrw_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ncols: *const MKL_INT,
        ab: *const f64,
        ldab: *const MKL_INT,
        afb: *const f64,
        ldafb: *const MKL_INT,
    ) -> f64;
    pub fn dla_geamv_(
        trans: *const MKL_INT,
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
    pub fn dla_gercond_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f64,
        info: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
    ) -> f64;
    pub fn dla_gerfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        errs_n: *mut f64,
        errs_c: *mut f64,
        res: *mut f64,
        ayb: *mut f64,
        dy: *mut f64,
        y_tail: *mut f64,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dla_gerpvgrw_(
        n: *const MKL_INT,
        ncols: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
    ) -> f64;
    pub fn dla_lin_berr_(
        n: *const MKL_INT,
        nz: *const MKL_INT,
        nrhs: *const MKL_INT,
        res: *const f64,
        ayb: *const f64,
        berr: *mut f64,
    );
    pub fn dla_porcond_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f64,
        info: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
    ) -> f64;
    pub fn dla_porfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut f64,
        ayb: *mut f64,
        dy: *mut f64,
        y_tail: *mut f64,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dla_porpvgrw_(
        uplo: *const c_char,
        ncols: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dla_syamv_(
        uplo: *const MKL_INT,
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
    pub fn dla_syrcond_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f64,
        info: *mut MKL_INT,
        work: *mut f64,
        iwork: *mut MKL_INT,
    ) -> f64;
    pub fn dla_syrfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const f64,
        ldb: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut f64,
        ayb: *mut f64,
        dy: *mut f64,
        y_tail: *mut f64,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dla_syrpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        af: *const f64,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn dla_wwaddw_(n: *const MKL_INT, x: *mut f64, y: *mut f64, w: *const f64);
    pub fn dtprfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f64,
        ldv: *const MKL_INT,
        t: *const f64,
        ldt: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        work: *mut f64,
        ldwork: *const MKL_INT,
    );
    pub fn sgeqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        t: *mut f32,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sla_gbamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        alpha: *const f32,
        ab: *const f32,
        ldab: *const MKL_INT,
        x: *const f32,
        incx: *const MKL_INT,
        beta: *const f32,
        y: *mut f32,
        incy: *const MKL_INT,
    );
    pub fn sla_gbrcond_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f32,
        info: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
    ) -> f32;
    pub fn sla_gbrfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut f32,
        ayb: *mut f32,
        dy: *mut f32,
        y_tail: *mut f32,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sla_gbrpvgrw_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ncols: *const MKL_INT,
        ab: *const f32,
        ldab: *const MKL_INT,
        afb: *const f32,
        ldafb: *const MKL_INT,
    ) -> f32;
    pub fn sla_geamv_(
        trans: *const MKL_INT,
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
    pub fn sla_gercond_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f32,
        info: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
    ) -> f32;
    pub fn sla_gerfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        errs_n: *mut f32,
        errs_c: *mut f32,
        res: *mut f32,
        ayb: *mut f32,
        dy: *mut f32,
        y_tail: *mut f32,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sla_gerpvgrw_(
        n: *const MKL_INT,
        ncols: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
    ) -> f32;
    pub fn sla_lin_berr_(
        n: *const MKL_INT,
        nz: *const MKL_INT,
        nrhs: *const MKL_INT,
        res: *const f32,
        ayb: *const f32,
        berr: *mut f32,
    );
    pub fn sla_porcond_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f32,
        info: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
    ) -> f32;
    pub fn sla_porfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut f32,
        ayb: *mut f32,
        dy: *mut f32,
        y_tail: *mut f32,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sla_porpvgrw_(
        uplo: *const c_char,
        ncols: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn sla_syamv_(
        uplo: *const MKL_INT,
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
    pub fn sla_syrcond_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        cmode: *const MKL_INT,
        c: *const f32,
        info: *mut MKL_INT,
        work: *mut f32,
        iwork: *mut MKL_INT,
    ) -> f32;
    pub fn sla_syrfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f32,
        b: *const f32,
        ldb: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        berr_out: *mut f32,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f32,
        err_bnds_comp: *mut f32,
        res: *mut f32,
        ayb: *mut f32,
        dy: *mut f32,
        y_tail: *mut f32,
        rcond: *const f32,
        ithresh: *const MKL_INT,
        rthresh: *const f32,
        dz_ub: *const f32,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sla_syrpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        af: *const f32,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f32,
    ) -> f32;
    pub fn sla_wwaddw_(n: *const MKL_INT, x: *mut f32, y: *mut f32, w: *const f32);
    pub fn stprfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const f32,
        ldv: *const MKL_INT,
        t: *const f32,
        ldt: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        work: *mut f32,
        ldwork: *const MKL_INT,
    );
    pub fn zgeqrt2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        t: *mut MKL_Complex16,
        ldt: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zheswapr_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        i1: *const MKL_INT,
        i2: *const MKL_INT,
    );
    pub fn zhetri2_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zhetri2x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut MKL_Complex16,
        nb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_gbamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        alpha: *const f64,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn zla_gbrcond_c_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f64,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_gbrcond_x_(
        trans: *const c_char,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex16,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_gbrfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        nrhs: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut MKL_Complex16,
        ayb: *mut f64,
        dy: *mut MKL_Complex16,
        y_tail: *mut MKL_Complex16,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_gbrpvgrw_(
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        ncols: *const MKL_INT,
        ab: *const MKL_Complex16,
        ldab: *const MKL_INT,
        afb: *const MKL_Complex16,
        ldafb: *const MKL_INT,
    ) -> f64;
    pub fn zla_geamv_(
        trans: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn zla_gercond_c_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f64,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_gercond_x_(
        trans: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex16,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_gerfsx_extended_(
        prec_type: *const MKL_INT,
        trans_type: *const MKL_INT,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        errs_n: *mut f64,
        errs_c: *mut f64,
        res: *mut MKL_Complex16,
        ayb: *mut f64,
        dy: *mut MKL_Complex16,
        y_tail: *mut MKL_Complex16,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_gerpvgrw_(
        n: *const MKL_INT,
        ncols: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
    ) -> f64;
    pub fn zla_heamv_(
        uplo: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn zla_hercond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f64,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_hercond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex16,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_herfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut MKL_Complex16,
        ayb: *mut f64,
        dy: *mut MKL_Complex16,
        y_tail: *mut MKL_Complex16,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_herpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zla_lin_berr_(
        n: *const MKL_INT,
        nz: *const MKL_INT,
        nrhs: *const MKL_INT,
        res: *const MKL_Complex16,
        ayb: *const f64,
        berr: *mut f64,
    );
    pub fn zla_porcond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        c: *const f64,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_porcond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        x: *const MKL_Complex16,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_porfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut MKL_Complex16,
        ayb: *mut f64,
        dy: *mut MKL_Complex16,
        y_tail: *mut MKL_Complex16,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_porpvgrw_(
        uplo: *const c_char,
        ncols: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zla_syamv_(
        uplo: *const MKL_INT,
        n: *const MKL_INT,
        alpha: *const f64,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        x: *const MKL_Complex16,
        incx: *const MKL_INT,
        beta: *const f64,
        y: *mut f64,
        incy: *const MKL_INT,
    );
    pub fn zla_syrcond_c_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        c: *const f64,
        capply: *const MKL_INT,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_syrcond_x_(
        uplo: *const c_char,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        x: *const MKL_Complex16,
        info: *mut MKL_INT,
        work: *mut MKL_Complex16,
        rwork: *mut f64,
    ) -> f64;
    pub fn zla_syrfsx_extended_(
        prec_type: *const MKL_INT,
        uplo: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        colequ: *const MKL_INT,
        c: *const f64,
        b: *const MKL_Complex16,
        ldb: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        berr_out: *mut f64,
        n_norms: *const MKL_INT,
        err_bnds_norm: *mut f64,
        err_bnds_comp: *mut f64,
        res: *mut MKL_Complex16,
        ayb: *mut f64,
        dy: *mut MKL_Complex16,
        y_tail: *mut MKL_Complex16,
        rcond: *const f64,
        ithresh: *const MKL_INT,
        rthresh: *const f64,
        dz_ub: *const f64,
        ignore_cwise: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zla_syrpvgrw_(
        uplo: *const c_char,
        n: *const MKL_INT,
        info: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        af: *const MKL_Complex16,
        ldaf: *const MKL_INT,
        ipiv: *const MKL_INT,
        work: *mut f64,
    ) -> f64;
    pub fn zla_wwaddw_(
        n: *const MKL_INT,
        x: *mut MKL_Complex16,
        y: *mut MKL_Complex16,
        w: *const MKL_Complex16,
    );
    pub fn ztprfb_(
        side: *const c_char,
        trans: *const c_char,
        direct: *const c_char,
        storev: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        k: *const MKL_INT,
        l: *const MKL_INT,
        v: *const MKL_Complex16,
        ldv: *const MKL_INT,
        t: *const MKL_Complex16,
        ldt: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        work: *mut MKL_Complex16,
        ldwork: *const MKL_INT,
    );
    pub fn cherdb_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut MKL_Complex8,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex8,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dsyrdb_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut f64,
        z: *mut f64,
        ldz: *const MKL_INT,
        work: *mut f64,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ssyrdb_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        d: *mut f32,
        e: *mut f32,
        tau: *mut f32,
        z: *mut f32,
        ldz: *const MKL_INT,
        work: *mut f32,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zherdb_(
        jobz: *const c_char,
        uplo: *const c_char,
        n: *const MKL_INT,
        kd: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        d: *mut f64,
        e: *mut f64,
        tau: *mut MKL_Complex16,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
        work: *mut MKL_Complex16,
        lwork: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cdtsvb_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
        du: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn cdttrfb_(
        n: *const MKL_INT,
        dl: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
        du: *const MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn cdttrsb_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex8,
        d: *const MKL_Complex8,
        du: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ddtsvb_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut f64,
        d: *mut f64,
        du: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn ddttrfb_(
        n: *const MKL_INT,
        dl: *mut f64,
        d: *mut f64,
        du: *const f64,
        info: *mut MKL_INT,
    );
    pub fn ddttrsb_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        b: *mut f64,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sdtsvb_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut f32,
        d: *mut f32,
        du: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn sdttrfb_(
        n: *const MKL_INT,
        dl: *mut f32,
        d: *mut f32,
        du: *const f32,
        info: *mut MKL_INT,
    );
    pub fn sdttrsb_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        b: *mut f32,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zdtsvb_(
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
        du: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zdttrfb_(
        n: *const MKL_INT,
        dl: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
        du: *const MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn zdttrsb_(
        trans: *const c_char,
        n: *const MKL_INT,
        nrhs: *const MKL_INT,
        dl: *const MKL_Complex16,
        d: *const MKL_Complex16,
        du: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_ctppack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex8,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_dtppack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *mut f64,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_stppack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *mut f32,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_ztppack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *mut MKL_Complex16,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_ctpunpack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex8,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_dtpunpack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *const f64,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_stpunpack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *const f32,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn mkl_ztpunpack_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const MKL_INT,
        ap: *const MKL_Complex16,
        i: *const MKL_INT,
        j: *const MKL_INT,
        rows: *const MKL_INT,
        cols: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatm1_(
        mode: *const MKL_INT,
        cond: *const f64,
        irsign: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *mut f64,
        n: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slatm1_(
        mode: *const MKL_INT,
        cond: *const f32,
        irsign: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *mut f32,
        n: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clatm1_(
        mode: *const MKL_INT,
        cond: *const f32,
        irsign: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *mut MKL_Complex8,
        n: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlatm1_(
        mode: *const MKL_INT,
        cond: *const f64,
        irsign: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *mut MKL_Complex16,
        n: *const MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatm2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const f64,
        igrade: *const MKL_INT,
        dl: *const f64,
        dr: *const f64,
        ipvtng: *const MKL_INT,
        iwork: *mut MKL_INT,
        sparse: *const f64,
    ) -> f64;
    pub fn slatm2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const f32,
        igrade: *const MKL_INT,
        dl: *const f32,
        dr: *const f32,
        ipvtng: *const MKL_INT,
        iwork: *mut MKL_INT,
        sparse: *const f32,
    ) -> f32;
    pub fn clatm2_(
        retval: *mut MKL_Complex8,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const MKL_Complex8,
        igrade: *const MKL_INT,
        dl: *const MKL_Complex8,
        dr: *const MKL_Complex8,
        ipvtng: *const MKL_INT,
        iwork: *mut MKL_INT,
        sparse: *const f32,
    );
    pub fn zlatm2_(
        retval: *mut MKL_Complex16,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const MKL_Complex16,
        igrade: *const MKL_INT,
        dl: *const MKL_Complex16,
        dr: *const MKL_Complex16,
        ipvtng: *const MKL_INT,
        iwork: *mut MKL_INT,
        sparse: *const f64,
    );
    pub fn dlatm3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        isub: *mut MKL_INT,
        jsub: *mut MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const f64,
        igrade: *const MKL_INT,
        dl: *const f64,
        dr: *const f64,
        ipvtng: *const MKL_INT,
        iwork: *const MKL_INT,
        sparse: *const f64,
    ) -> f64;
    pub fn slatm3_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        isub: *mut MKL_INT,
        jsub: *mut MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const f32,
        igrade: *const MKL_INT,
        dl: *const f32,
        dr: *const f32,
        ipvtng: *const MKL_INT,
        iwork: *const MKL_INT,
        sparse: *const f32,
    ) -> f32;
    pub fn clatm3_(
        retval: *mut MKL_Complex8,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        isub: *mut MKL_INT,
        jsub: *mut MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const MKL_Complex8,
        igrade: *const MKL_INT,
        dl: *const MKL_Complex8,
        dr: *const MKL_Complex8,
        ipvtng: *const MKL_INT,
        iwork: *const MKL_INT,
        sparse: *const f32,
    );
    pub fn zlatm3_(
        retval: *mut MKL_Complex16,
        m: *const MKL_INT,
        n: *const MKL_INT,
        ii: *mut MKL_INT,
        j: *const MKL_INT,
        isub: *mut MKL_INT,
        jsub: *mut MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        idist: *const MKL_INT,
        iseed: *mut MKL_INT,
        d: *const MKL_Complex16,
        igrade: *const MKL_INT,
        dl: *const MKL_Complex16,
        dr: *const MKL_Complex16,
        ipvtng: *const MKL_INT,
        iwork: *const MKL_INT,
        sparse: *const f64,
    );
    pub fn dlatm5_(
        prtype: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        ldb: *const MKL_INT,
        c: *mut f64,
        ldc: *const MKL_INT,
        d: *mut f64,
        ldd: *const MKL_INT,
        e: *mut f64,
        lde: *const MKL_INT,
        f: *mut f64,
        ldf: *const MKL_INT,
        r: *mut f64,
        ldr: *const MKL_INT,
        l: *mut f64,
        ldl: *const MKL_INT,
        alpha: *const f64,
        qblcka: *const MKL_INT,
        qblckb: *const MKL_INT,
    );
    pub fn slatm5_(
        prtype: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        ldb: *const MKL_INT,
        c: *mut f32,
        ldc: *const MKL_INT,
        d: *mut f32,
        ldd: *const MKL_INT,
        e: *mut f32,
        lde: *const MKL_INT,
        f: *mut f32,
        ldf: *const MKL_INT,
        r: *mut f32,
        ldr: *const MKL_INT,
        l: *mut f32,
        ldl: *const MKL_INT,
        alpha: *const f32,
        qblcka: *const MKL_INT,
        qblckb: *const MKL_INT,
    );
    pub fn clatm5_(
        prtype: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex8,
        ldc: *const MKL_INT,
        d: *mut MKL_Complex8,
        ldd: *const MKL_INT,
        e: *mut MKL_Complex8,
        lde: *const MKL_INT,
        f: *mut MKL_Complex8,
        ldf: *const MKL_INT,
        r: *mut MKL_Complex8,
        ldr: *const MKL_INT,
        l: *mut MKL_Complex8,
        ldl: *const MKL_INT,
        alpha: *const f32,
        qblcka: *const MKL_INT,
        qblckb: *const MKL_INT,
    );
    pub fn zlatm5_(
        prtype: *const MKL_INT,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        ldb: *const MKL_INT,
        c: *mut MKL_Complex16,
        ldc: *const MKL_INT,
        d: *mut MKL_Complex16,
        ldd: *const MKL_INT,
        e: *mut MKL_Complex16,
        lde: *const MKL_INT,
        f: *mut MKL_Complex16,
        ldf: *const MKL_INT,
        r: *mut MKL_Complex16,
        ldr: *const MKL_INT,
        l: *mut MKL_Complex16,
        ldl: *const MKL_INT,
        alpha: *const f64,
        qblcka: *const MKL_INT,
        qblckb: *const MKL_INT,
    );
    pub fn dlatm6_(
        type_: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        b: *mut f64,
        x: *mut f64,
        ldx: *const MKL_INT,
        y: *mut f64,
        ldy: *const MKL_INT,
        alpha: *const f64,
        beta: *const f64,
        wx: *const f64,
        wy: *const f64,
        s: *mut f64,
        dif: *mut f64,
    );
    pub fn slatm6_(
        type_: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        b: *mut f32,
        x: *mut f32,
        ldx: *const MKL_INT,
        y: *mut f32,
        ldy: *const MKL_INT,
        alpha: *const f32,
        beta: *const f32,
        wx: *const f32,
        wy: *const f32,
        s: *mut f32,
        dif: *mut f32,
    );
    pub fn clatm6_(
        type_: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        b: *mut MKL_Complex8,
        x: *mut MKL_Complex8,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex8,
        ldy: *const MKL_INT,
        alpha: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        wx: *const MKL_Complex8,
        wy: *const MKL_Complex8,
        s: *mut f32,
        dif: *mut f32,
    );
    pub fn zlatm6_(
        type_: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        b: *mut MKL_Complex16,
        x: *mut MKL_Complex16,
        ldx: *const MKL_INT,
        y: *mut MKL_Complex16,
        ldy: *const MKL_INT,
        alpha: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        wx: *const MKL_Complex16,
        wy: *const MKL_Complex16,
        s: *mut f64,
        dif: *mut f64,
    );
    pub fn dlatme_(
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        d: *mut f64,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const f64,
        ei: *const c_char,
        rsign: *const c_char,
        upper: *const c_char,
        sim: *const c_char,
        ds: *mut f64,
        modes: *const MKL_INT,
        conds: *const f64,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        anorm: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slatme_(
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        d: *mut f32,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const f32,
        ei: *const c_char,
        rsign: *const c_char,
        upper: *const c_char,
        sim: *const c_char,
        ds: *mut f32,
        modes: *const MKL_INT,
        conds: *const f32,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        anorm: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clatme_(
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        d: *mut MKL_Complex8,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const MKL_Complex8,
        rsign: *const c_char,
        upper: *const c_char,
        sim: *const c_char,
        ds: *mut f32,
        modes: *const MKL_INT,
        conds: *const f32,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        anorm: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlatme_(
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        d: *mut MKL_Complex16,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const MKL_Complex16,
        rsign: *const c_char,
        upper: *const c_char,
        sim: *const c_char,
        ds: *mut f64,
        modes: *const MKL_INT,
        conds: *const f64,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        anorm: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn dlatmr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut f64,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const f64,
        rsign: *const c_char,
        grade: *const c_char,
        dl: *mut f64,
        model: *const MKL_INT,
        condl: *const f64,
        dr: *mut f64,
        moder: *const MKL_INT,
        condr: *const f64,
        pivtng: *const c_char,
        ipivot: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        sparse: *const f64,
        anorm: *const f64,
        pack: *const c_char,
        a: *mut f64,
        lda: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn slatmr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *const f32,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const f32,
        rsign: *const c_char,
        grade: *const c_char,
        dl: *mut f32,
        model: *const MKL_INT,
        condl: *const f32,
        dr: *mut f32,
        moder: *const MKL_INT,
        condr: *const f32,
        pivtng: *const c_char,
        ipivot: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        sparse: *const f32,
        anorm: *const f32,
        pack: *const c_char,
        a: *mut f32,
        lda: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn clatmr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut MKL_Complex8,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const MKL_Complex8,
        rsign: *const c_char,
        grade: *const c_char,
        dl: *mut MKL_Complex8,
        model: *const MKL_INT,
        condl: *const f32,
        dr: *mut MKL_Complex8,
        moder: *const MKL_INT,
        condr: *const f32,
        pivtng: *const c_char,
        ipivot: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        sparse: *const f32,
        anorm: *const f32,
        pack: *const c_char,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn zlatmr_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut MKL_Complex16,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const MKL_Complex16,
        rsign: *const c_char,
        grade: *const c_char,
        dl: *mut MKL_Complex16,
        model: *const MKL_INT,
        condl: *const f64,
        dr: *mut MKL_Complex16,
        moder: *const MKL_INT,
        condr: *const f64,
        pivtng: *const c_char,
        ipivot: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        sparse: *const f64,
        anorm: *const f64,
        pack: *const c_char,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iwork: *mut MKL_INT,
        info: *mut MKL_INT,
    );
    pub fn dlatms_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut f64,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const f64,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        pack: *const c_char,
        a: *mut f64,
        lda: *const MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slatms_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut f32,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const f32,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        pack: *const c_char,
        a: *mut f32,
        lda: *const MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clatms_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut f32,
        mode: *const MKL_INT,
        cond: *const f32,
        dmax: *const f32,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        pack: *const c_char,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlatms_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        dist: *const c_char,
        iseed: *mut MKL_INT,
        sym: *const c_char,
        d: *mut f64,
        mode: *const MKL_INT,
        cond: *const f64,
        dmax: *const f64,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        pack: *const c_char,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn dlakf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f64,
        lda: *const MKL_INT,
        b: *const f64,
        d: *const f64,
        e: *const f64,
        z: *mut f64,
        ldz: *const MKL_INT,
    );
    pub fn slakf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const f32,
        lda: *const MKL_INT,
        b: *const f32,
        d: *const f32,
        e: *const f32,
        z: *mut f32,
        ldz: *const MKL_INT,
    );
    pub fn clakf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex8,
        lda: *const MKL_INT,
        b: *const MKL_Complex8,
        d: *const MKL_Complex8,
        e: *const MKL_Complex8,
        z: *mut MKL_Complex8,
        ldz: *const MKL_INT,
    );
    pub fn zlakf2_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *const MKL_Complex16,
        lda: *const MKL_INT,
        b: *const MKL_Complex16,
        d: *const MKL_Complex16,
        e: *const MKL_Complex16,
        z: *mut MKL_Complex16,
        ldz: *const MKL_INT,
    );
    pub fn dlarge_(
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slarge_(
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clarge_(
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlarge_(
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn dlarnd_(idist: *const MKL_INT, iseed: *mut MKL_INT) -> f64;
    pub fn slarnd_(idist: *const MKL_INT, iseed: *mut MKL_INT) -> f32;
    pub fn clarnd_(retval: *mut MKL_Complex8, idist: *const MKL_INT, iseed: *mut MKL_INT);
    pub fn zlarnd_(retval: *mut MKL_Complex16, idist: *const MKL_INT, iseed: *mut MKL_INT);
    pub fn dlaror_(
        side: *const c_char,
        init: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f64,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        x: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slaror_(
        side: *const c_char,
        init: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut f32,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        x: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn claror_(
        side: *const c_char,
        init: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        x: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlaror_(
        side: *const c_char,
        init: *const c_char,
        m: *const MKL_INT,
        n: *const MKL_INT,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        x: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn dlarot_(
        lrows: *const MKL_INT,
        lleft: *const MKL_INT,
        lright: *const MKL_INT,
        nl: *const MKL_INT,
        c: *const f64,
        s: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        xleft: *mut f64,
        xright: *mut f64,
    );
    pub fn slarot_(
        lrows: *const MKL_INT,
        lleft: *const MKL_INT,
        lright: *const MKL_INT,
        nl: *const MKL_INT,
        c: *const f32,
        s: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        xleft: *mut f32,
        xright: *mut f32,
    );
    pub fn clarot_(
        lrows: *const MKL_INT,
        lleft: *const MKL_INT,
        lright: *mut MKL_INT,
        nl: *const MKL_INT,
        c: *const MKL_Complex8,
        s: *const MKL_Complex8,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        xleft: *mut MKL_Complex8,
        xright: *mut MKL_Complex8,
    );
    pub fn zlarot_(
        lrows: *const MKL_INT,
        lleft: *const MKL_INT,
        lright: *const MKL_INT,
        nl: *const MKL_INT,
        c: *const MKL_Complex16,
        s: *const MKL_Complex16,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        xleft: *mut MKL_Complex16,
        xright: *mut MKL_Complex16,
    );
    pub fn dlaran_(iseed: *mut MKL_INT) -> f64;
    pub fn slaran_(iseed: *mut MKL_INT) -> f32;
    pub fn dlagge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        d: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slagge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        d: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn clagge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        d: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlagge_(
        m: *const MKL_INT,
        n: *const MKL_INT,
        kl: *const MKL_INT,
        ku: *const MKL_INT,
        d: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn clagsy_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn dlagsy_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f64,
        a: *mut f64,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f64,
        info: *mut MKL_INT,
    );
    pub fn slagsy_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f32,
        a: *mut f32,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut f32,
        info: *mut MKL_INT,
    );
    pub fn zlagsy_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn claghe_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f32,
        a: *mut MKL_Complex8,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex8,
        info: *mut MKL_INT,
    );
    pub fn zlaghe_(
        n: *const MKL_INT,
        k: *const MKL_INT,
        d: *const f64,
        a: *mut MKL_Complex16,
        lda: *const MKL_INT,
        iseed: *mut MKL_INT,
        work: *mut MKL_Complex16,
        info: *mut MKL_INT,
    );
    pub fn mkl_cspffrt2_(
        ap: *mut MKL_Complex8,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut MKL_Complex8,
        work2: *mut MKL_Complex8,
    );
    pub fn mkl_cspffrtx_(
        ap: *mut MKL_Complex8,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut MKL_Complex8,
        work2: *mut MKL_Complex8,
    );
    pub fn mkl_dspffrt2_(
        ap: *mut f64,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut f64,
        work2: *mut f64,
    );
    pub fn mkl_dspffrtx_(
        ap: *mut f64,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut f64,
        work2: *mut f64,
    );
    pub fn mkl_sspffrt2_(
        ap: *mut f32,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut f32,
        work2: *mut f32,
    );
    pub fn mkl_sspffrtx_(
        ap: *mut f32,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut f32,
        work2: *mut f32,
    );
    pub fn mkl_zspffrt2_(
        ap: *mut MKL_Complex16,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut MKL_Complex16,
        work2: *mut MKL_Complex16,
    );
    pub fn mkl_zspffrtx_(
        ap: *mut MKL_Complex16,
        n: *const MKL_INT,
        ncolm: *const MKL_INT,
        work: *mut MKL_Complex16,
        work2: *mut MKL_Complex16,
    );
    pub fn mkl_progress_(
        thread: *mut c_int,
        step: *mut c_int,
        stage: *mut c_char,
        lstage: c_int,
    ) -> c_int;
}

/* #region upper case alias */

pub use cbbcsd_ as CBBCSD;
pub use cbdsqr_ as CBDSQR;
pub use cdtsvb_ as CDTSVB;
pub use cdttrfb_ as CDTTRFB;
pub use cdttrsb_ as CDTTRSB;
pub use cgbbrd_ as CGBBRD;
pub use cgbcon_ as CGBCON;
pub use cgbequ_ as CGBEQU;
pub use cgbequb_ as CGBEQUB;
pub use cgbrfs_ as CGBRFS;
pub use cgbrfsx_ as CGBRFSX;
pub use cgbsv_ as CGBSV;
pub use cgbsvx_ as CGBSVX;
pub use cgbsvxx_ as CGBSVXX;
pub use cgbtf2_ as CGBTF2;
pub use cgbtrf_ as CGBTRF;
pub use cgbtrs_ as CGBTRS;
pub use cgebak_ as CGEBAK;
pub use cgebal_ as CGEBAL;
pub use cgebd2_ as CGEBD2;
pub use cgebrd_ as CGEBRD;
pub use cgecon_ as CGECON;
pub use cgedmd_ as CGEDMD;
pub use cgedmdq_ as CGEDMDQ;
pub use cgeequ_ as CGEEQU;
pub use cgeequb_ as CGEEQUB;
pub use cgees_ as CGEES;
pub use cgeesx_ as CGEESX;
pub use cgeev_ as CGEEV;
pub use cgeevx_ as CGEEVX;
pub use cgegs_ as CGEGS;
pub use cgegv_ as CGEGV;
pub use cgehd2_ as CGEHD2;
pub use cgehrd_ as CGEHRD;
pub use cgejsv_ as CGEJSV;
pub use cgelq2_ as CGELQ2;
pub use cgelq_ as CGELQ;
pub use cgelqf_ as CGELQF;
pub use cgelqt3_ as CGELQT3;
pub use cgelqt_ as CGELQT;
pub use cgels_ as CGELS;
pub use cgels_batch_strided_ as CGELS_BATCH_STRIDED;
pub use cgelsd_ as CGELSD;
pub use cgelss_ as CGELSS;
pub use cgelst_ as CGELST;
pub use cgelsx_ as CGELSX;
pub use cgelsy_ as CGELSY;
pub use cgemlq_ as CGEMLQ;
pub use cgemlqt_ as CGEMLQT;
pub use cgemqr_ as CGEMQR;
pub use cgemqrt_ as CGEMQRT;
pub use cgeql2_ as CGEQL2;
pub use cgeqlf_ as CGEQLF;
pub use cgeqp3_ as CGEQP3;
pub use cgeqp3rk_ as CGEQP3RK;
pub use cgeqpf_ as CGEQPF;
pub use cgeqr2_ as CGEQR2;
pub use cgeqr2p_ as CGEQR2P;
pub use cgeqr_ as CGEQR;
pub use cgeqrf_ as CGEQRF;
pub use cgeqrfp_ as CGEQRFP;
pub use cgeqrt2_ as CGEQRT2;
pub use cgeqrt3_ as CGEQRT3;
pub use cgeqrt_ as CGEQRT;
pub use cgerfs_ as CGERFS;
pub use cgerfsx_ as CGERFSX;
pub use cgerq2_ as CGERQ2;
pub use cgerqf_ as CGERQF;
pub use cgesc2_ as CGESC2;
pub use cgesdd_ as CGESDD;
pub use cgesv_ as CGESV;
pub use cgesvd_ as CGESVD;
pub use cgesvda_batch_strided_ as CGESVDA_BATCH_STRIDED;
pub use cgesvdq_ as CGESVDQ;
pub use cgesvdx_ as CGESVDX;
pub use cgesvj_ as CGESVJ;
pub use cgesvx_ as CGESVX;
pub use cgesvxx_ as CGESVXX;
pub use cgetc2_ as CGETC2;
pub use cgetf2_ as CGETF2;
pub use cgetrf2_ as CGETRF2;
pub use cgetrf_ as CGETRF;
pub use cgetrf_batch_ as CGETRF_BATCH;
pub use cgetrf_batch_strided_ as CGETRF_BATCH_STRIDED;
pub use cgetrfnp_batch_ as CGETRFNP_BATCH;
pub use cgetrfnp_batch_strided_ as CGETRFNP_BATCH_STRIDED;
pub use cgetri_ as CGETRI;
pub use cgetri_batch_strided_ as CGETRI_BATCH_STRIDED;
pub use cgetri_oop_batch_ as CGETRI_OOP_BATCH;
pub use cgetri_oop_batch_strided_ as CGETRI_OOP_BATCH_STRIDED;
pub use cgetrs_ as CGETRS;
pub use cgetrs_batch_strided_ as CGETRS_BATCH_STRIDED;
pub use cgetrsnp_batch_strided_ as CGETRSNP_BATCH_STRIDED;
pub use cgetsls_ as CGETSLS;
pub use cgetsqrhrt_ as CGETSQRHRT;
pub use cggbak_ as CGGBAK;
pub use cggbal_ as CGGBAL;
pub use cgges3_ as CGGES3;
pub use cgges_ as CGGES;
pub use cggesx_ as CGGESX;
pub use cggev3_ as CGGEV3;
pub use cggev_ as CGGEV;
pub use cggevx_ as CGGEVX;
pub use cggglm_ as CGGGLM;
pub use cgghd3_ as CGGHD3;
pub use cgghrd_ as CGGHRD;
pub use cgglse_ as CGGLSE;
pub use cggqrf_ as CGGQRF;
pub use cggrqf_ as CGGRQF;
pub use cggsvd3_ as CGGSVD3;
pub use cggsvd_ as CGGSVD;
pub use cggsvp3_ as CGGSVP3;
pub use cggsvp_ as CGGSVP;
pub use cgsvj0_ as CGSVJ0;
pub use cgsvj1_ as CGSVJ1;
pub use cgtcon_ as CGTCON;
pub use cgtrfs_ as CGTRFS;
pub use cgtsv_ as CGTSV;
pub use cgtsvx_ as CGTSVX;
pub use cgttrf_ as CGTTRF;
pub use cgttrs_ as CGTTRS;
pub use cgtts2_ as CGTTS2;
pub use chb2st_kernels_ as CHB2ST_KERNELS;
pub use chbev_ as CHBEV;
pub use chbev_2stage_ as CHBEV_2STAGE;
pub use chbevd_ as CHBEVD;
pub use chbevd_2stage_ as CHBEVD_2STAGE;
pub use chbevx_ as CHBEVX;
pub use chbevx_2stage_ as CHBEVX_2STAGE;
pub use chbgst_ as CHBGST;
pub use chbgv_ as CHBGV;
pub use chbgvd_ as CHBGVD;
pub use chbgvx_ as CHBGVX;
pub use chbtrd_ as CHBTRD;
pub use checon_ as CHECON;
pub use checon_3_ as CHECON_3;
pub use checon_rook_ as CHECON_ROOK;
pub use cheequb_ as CHEEQUB;
pub use cheev_ as CHEEV;
pub use cheev_2stage_ as CHEEV_2STAGE;
pub use cheevd_ as CHEEVD;
pub use cheevd_2stage_ as CHEEVD_2STAGE;
pub use cheevr_ as CHEEVR;
pub use cheevr_2stage_ as CHEEVR_2STAGE;
pub use cheevx_ as CHEEVX;
pub use cheevx_2stage_ as CHEEVX_2STAGE;
pub use chegs2_ as CHEGS2;
pub use chegst_ as CHEGST;
pub use chegv_ as CHEGV;
pub use chegv_2stage_ as CHEGV_2STAGE;
pub use chegvd_ as CHEGVD;
pub use chegvx_ as CHEGVX;
pub use cherdb_ as CHERDB;
pub use cherfs_ as CHERFS;
pub use cherfsx_ as CHERFSX;
pub use chesv_ as CHESV;
pub use chesv_aa_ as CHESV_AA;
pub use chesv_aa_2stage_ as CHESV_AA_2STAGE;
pub use chesv_rk_ as CHESV_RK;
pub use chesv_rook_ as CHESV_ROOK;
pub use chesvx_ as CHESVX;
pub use chesvxx_ as CHESVXX;
pub use cheswapr_ as CHESWAPR;
pub use chetd2_ as CHETD2;
pub use chetf2_ as CHETF2;
pub use chetf2_rk_ as CHETF2_RK;
pub use chetf2_rook_ as CHETF2_ROOK;
pub use chetrd_ as CHETRD;
pub use chetrd_2stage_ as CHETRD_2STAGE;
pub use chetrd_hb2st_ as CHETRD_HB2ST;
pub use chetrd_he2hb_ as CHETRD_HE2HB;
pub use chetrf_ as CHETRF;
pub use chetrf_aa_ as CHETRF_AA;
pub use chetrf_aa_2stage_ as CHETRF_AA_2STAGE;
pub use chetrf_rk_ as CHETRF_RK;
pub use chetrf_rook_ as CHETRF_ROOK;
pub use chetri2_ as CHETRI2;
pub use chetri2x_ as CHETRI2X;
pub use chetri_ as CHETRI;
pub use chetri_3_ as CHETRI_3;
pub use chetri_3x_ as CHETRI_3X;
pub use chetri_rook_ as CHETRI_ROOK;
pub use chetrs2_ as CHETRS2;
pub use chetrs_ as CHETRS;
pub use chetrs_3_ as CHETRS_3;
pub use chetrs_aa_ as CHETRS_AA;
pub use chetrs_aa_2stage_ as CHETRS_AA_2STAGE;
pub use chetrs_rook_ as CHETRS_ROOK;
pub use chfrk_ as CHFRK;
pub use chgeqz_ as CHGEQZ;
pub use chla_transtype_ as CHLA_TRANSTYPE;
pub use chpcon_ as CHPCON;
pub use chpev_ as CHPEV;
pub use chpevd_ as CHPEVD;
pub use chpevx_ as CHPEVX;
pub use chpgst_ as CHPGST;
pub use chpgv_ as CHPGV;
pub use chpgvd_ as CHPGVD;
pub use chpgvx_ as CHPGVX;
pub use chprfs_ as CHPRFS;
pub use chpsv_ as CHPSV;
pub use chpsvx_ as CHPSVX;
pub use chptrd_ as CHPTRD;
pub use chptrf_ as CHPTRF;
pub use chptri_ as CHPTRI;
pub use chptrs_ as CHPTRS;
pub use chsein_ as CHSEIN;
pub use chseqr_ as CHSEQR;
pub use cla_gbamv_ as CLA_GBAMV;
pub use cla_gbrcond_c_ as CLA_GBRCOND_C;
pub use cla_gbrcond_x_ as CLA_GBRCOND_X;
pub use cla_gbrfsx_extended_ as CLA_GBRFSX_EXTENDED;
pub use cla_gbrpvgrw_ as CLA_GBRPVGRW;
pub use cla_geamv_ as CLA_GEAMV;
pub use cla_gercond_c_ as CLA_GERCOND_C;
pub use cla_gercond_x_ as CLA_GERCOND_X;
pub use cla_gerfsx_extended_ as CLA_GERFSX_EXTENDED;
pub use cla_gerpvgrw_ as CLA_GERPVGRW;
pub use cla_heamv_ as CLA_HEAMV;
pub use cla_hercond_c_ as CLA_HERCOND_C;
pub use cla_hercond_x_ as CLA_HERCOND_X;
pub use cla_herfsx_extended_ as CLA_HERFSX_EXTENDED;
pub use cla_herpvgrw_ as CLA_HERPVGRW;
pub use cla_lin_berr_ as CLA_LIN_BERR;
pub use cla_porcond_c_ as CLA_PORCOND_C;
pub use cla_porcond_x_ as CLA_PORCOND_X;
pub use cla_porfsx_extended_ as CLA_PORFSX_EXTENDED;
pub use cla_porpvgrw_ as CLA_PORPVGRW;
pub use cla_syamv_ as CLA_SYAMV;
pub use cla_syrcond_c_ as CLA_SYRCOND_C;
pub use cla_syrcond_x_ as CLA_SYRCOND_X;
pub use cla_syrfsx_extended_ as CLA_SYRFSX_EXTENDED;
pub use cla_syrpvgrw_ as CLA_SYRPVGRW;
pub use cla_wwaddw_ as CLA_WWADDW;
pub use clabrd_ as CLABRD;
pub use clacgv_ as CLACGV;
pub use clacn2_ as CLACN2;
pub use clacon_ as CLACON;
pub use clacp2_ as CLACP2;
pub use clacpy_ as CLACPY;
pub use clacrm_ as CLACRM;
pub use clacrt_ as CLACRT;
pub use cladiv_ as CLADIV;
pub use claed0_ as CLAED0;
pub use claed7_ as CLAED7;
pub use claed8_ as CLAED8;
pub use claein_ as CLAEIN;
pub use claesy_ as CLAESY;
pub use claev2_ as CLAEV2;
pub use clag2z_ as CLAG2Z;
pub use clagge_ as CLAGGE;
pub use claghe_ as CLAGHE;
pub use clags2_ as CLAGS2;
pub use clagsy_ as CLAGSY;
pub use clagtm_ as CLAGTM;
pub use clahef_ as CLAHEF;
pub use clahef_aa_ as CLAHEF_AA;
pub use clahef_rk_ as CLAHEF_RK;
pub use clahef_rook_ as CLAHEF_ROOK;
pub use clahqr_ as CLAHQR;
pub use clahr2_ as CLAHR2;
pub use clahrd_ as CLAHRD;
pub use claic1_ as CLAIC1;
pub use clakf2_ as CLAKF2;
pub use clals0_ as CLALS0;
pub use clalsa_ as CLALSA;
pub use clalsd_ as CLALSD;
pub use clamswlq_ as CLAMSWLQ;
pub use clamtsqr_ as CLAMTSQR;
pub use clangb_ as CLANGB;
pub use clange_ as CLANGE;
pub use clangt_ as CLANGT;
pub use clanhb_ as CLANHB;
pub use clanhe_ as CLANHE;
pub use clanhf_ as CLANHF;
pub use clanhp_ as CLANHP;
pub use clanhs_ as CLANHS;
pub use clanht_ as CLANHT;
pub use clansb_ as CLANSB;
pub use clansp_ as CLANSP;
pub use clansy_ as CLANSY;
pub use clantb_ as CLANTB;
pub use clantp_ as CLANTP;
pub use clantr_ as CLANTR;
pub use clapll_ as CLAPLL;
pub use clapmr_ as CLAPMR;
pub use clapmt_ as CLAPMT;
pub use claqgb_ as CLAQGB;
pub use claqge_ as CLAQGE;
pub use claqhb_ as CLAQHB;
pub use claqhe_ as CLAQHE;
pub use claqhp_ as CLAQHP;
pub use claqp2_ as CLAQP2;
pub use claqps_ as CLAQPS;
pub use claqr0_ as CLAQR0;
pub use claqr1_ as CLAQR1;
pub use claqr2_ as CLAQR2;
pub use claqr3_ as CLAQR3;
pub use claqr4_ as CLAQR4;
pub use claqr5_ as CLAQR5;
pub use claqsb_ as CLAQSB;
pub use claqsp_ as CLAQSP;
pub use claqsy_ as CLAQSY;
pub use claqz0_ as CLAQZ0;
pub use claqz1_ as CLAQZ1;
pub use claqz2_ as CLAQZ2;
pub use claqz3_ as CLAQZ3;
pub use clar1v_ as CLAR1V;
pub use clar2v_ as CLAR2V;
pub use clarcm_ as CLARCM;
pub use clarf1f_ as CLARF1F;
pub use clarf1l_ as CLARF1L;
pub use clarf_ as CLARF;
pub use clarfb_ as CLARFB;
pub use clarfb_gett_ as CLARFB_GETT;
pub use clarfg_ as CLARFG;
pub use clarfgp_ as CLARFGP;
pub use clarfp_ as CLARFP;
pub use clarft_ as CLARFT;
pub use clarfx_ as CLARFX;
pub use clarfy_ as CLARFY;
pub use clarge_ as CLARGE;
pub use clargv_ as CLARGV;
pub use clarnd_ as CLARND;
pub use clarnv_ as CLARNV;
pub use claror_ as CLAROR;
pub use clarot_ as CLAROT;
pub use clarrv_ as CLARRV;
pub use clarscl2_ as CLARSCL2;
pub use clartg_ as CLARTG;
pub use clartv_ as CLARTV;
pub use clarz_ as CLARZ;
pub use clarzb_ as CLARZB;
pub use clarzt_ as CLARZT;
pub use clascl2_ as CLASCL2;
pub use clascl_ as CLASCL;
pub use claset_ as CLASET;
pub use clasr_ as CLASR;
pub use classq_ as CLASSQ;
pub use claswlq_ as CLASWLQ;
pub use claswp_ as CLASWP;
pub use clasyf_ as CLASYF;
pub use clasyf_aa_ as CLASYF_AA;
pub use clasyf_rk_ as CLASYF_RK;
pub use clasyf_rook_ as CLASYF_ROOK;
pub use clatbs_ as CLATBS;
pub use clatdf_ as CLATDF;
pub use clatm1_ as CLATM1;
pub use clatm2_ as CLATM2;
pub use clatm3_ as CLATM3;
pub use clatm5_ as CLATM5;
pub use clatm6_ as CLATM6;
pub use clatme_ as CLATME;
pub use clatmr_ as CLATMR;
pub use clatms_ as CLATMS;
pub use clatps_ as CLATPS;
pub use clatrd_ as CLATRD;
pub use clatrs3_ as CLATRS3;
pub use clatrs_ as CLATRS;
pub use clatrz_ as CLATRZ;
pub use clatsqr_ as CLATSQR;
pub use clatzm_ as CLATZM;
pub use claunhr_col_getrfnp2_ as CLAUNHR_COL_GETRFNP2;
pub use claunhr_col_getrfnp_ as CLAUNHR_COL_GETRFNP;
pub use clauu2_ as CLAUU2;
pub use clauum_ as CLAUUM;
pub use cpbcon_ as CPBCON;
pub use cpbequ_ as CPBEQU;
pub use cpbrfs_ as CPBRFS;
pub use cpbstf_ as CPBSTF;
pub use cpbsv_ as CPBSV;
pub use cpbsvx_ as CPBSVX;
pub use cpbtf2_ as CPBTF2;
pub use cpbtrf_ as CPBTRF;
pub use cpbtrs_ as CPBTRS;
pub use cpftrf_ as CPFTRF;
pub use cpftri_ as CPFTRI;
pub use cpftrs_ as CPFTRS;
pub use cpocon_ as CPOCON;
pub use cpoequ_ as CPOEQU;
pub use cpoequb_ as CPOEQUB;
pub use cporfs_ as CPORFS;
pub use cporfsx_ as CPORFSX;
pub use cposv_ as CPOSV;
pub use cposvx_ as CPOSVX;
pub use cposvxx_ as CPOSVXX;
pub use cpotf2_ as CPOTF2;
pub use cpotrf2_ as CPOTRF2;
pub use cpotrf_ as CPOTRF;
pub use cpotri_ as CPOTRI;
pub use cpotrs_ as CPOTRS;
pub use cppcon_ as CPPCON;
pub use cppequ_ as CPPEQU;
pub use cpprfs_ as CPPRFS;
pub use cppsv_ as CPPSV;
pub use cppsvx_ as CPPSVX;
pub use cpptrf_ as CPPTRF;
pub use cpptri_ as CPPTRI;
pub use cpptrs_ as CPPTRS;
pub use cpstf2_ as CPSTF2;
pub use cpstrf_ as CPSTRF;
pub use cptcon_ as CPTCON;
pub use cpteqr_ as CPTEQR;
pub use cptrfs_ as CPTRFS;
pub use cptsv_ as CPTSV;
pub use cptsvx_ as CPTSVX;
pub use cpttrf_ as CPTTRF;
pub use cpttrs_ as CPTTRS;
pub use cptts2_ as CPTTS2;
pub use crot_ as CROT;
pub use cspcon_ as CSPCON;
pub use cspmv_ as CSPMV;
pub use cspr_ as CSPR;
pub use csprfs_ as CSPRFS;
pub use cspsv_ as CSPSV;
pub use cspsvx_ as CSPSVX;
pub use csptrf_ as CSPTRF;
pub use csptri_ as CSPTRI;
pub use csptrs_ as CSPTRS;
pub use csrscl_ as CSRSCL;
pub use cstedc_ as CSTEDC;
pub use cstegr_ as CSTEGR;
pub use cstein_ as CSTEIN;
pub use cstemr_ as CSTEMR;
pub use csteqr_ as CSTEQR;
pub use csycon_ as CSYCON;
pub use csycon_3_ as CSYCON_3;
pub use csycon_rook_ as CSYCON_ROOK;
pub use csyconv_ as CSYCONV;
pub use csyconvf_ as CSYCONVF;
pub use csyconvf_rook_ as CSYCONVF_ROOK;
pub use csyequb_ as CSYEQUB;
pub use csymv_ as CSYMV;
pub use csyr_ as CSYR;
pub use csyrfs_ as CSYRFS;
pub use csyrfsx_ as CSYRFSX;
pub use csysv_ as CSYSV;
pub use csysv_aa_ as CSYSV_AA;
pub use csysv_aa_2stage_ as CSYSV_AA_2STAGE;
pub use csysv_rk_ as CSYSV_RK;
pub use csysv_rook_ as CSYSV_ROOK;
pub use csysvx_ as CSYSVX;
pub use csysvxx_ as CSYSVXX;
pub use csyswapr_ as CSYSWAPR;
pub use csytf2_ as CSYTF2;
pub use csytf2_rk_ as CSYTF2_RK;
pub use csytf2_rook_ as CSYTF2_ROOK;
pub use csytrf_ as CSYTRF;
pub use csytrf_aa_ as CSYTRF_AA;
pub use csytrf_aa_2stage_ as CSYTRF_AA_2STAGE;
pub use csytrf_rk_ as CSYTRF_RK;
pub use csytrf_rook_ as CSYTRF_ROOK;
pub use csytri2_ as CSYTRI2;
pub use csytri2x_ as CSYTRI2X;
pub use csytri_ as CSYTRI;
pub use csytri_3_ as CSYTRI_3;
pub use csytri_3x_ as CSYTRI_3X;
pub use csytri_rook_ as CSYTRI_ROOK;
pub use csytrs2_ as CSYTRS2;
pub use csytrs_ as CSYTRS;
pub use csytrs_3_ as CSYTRS_3;
pub use csytrs_aa_ as CSYTRS_AA;
pub use csytrs_aa_2stage_ as CSYTRS_AA_2STAGE;
pub use csytrs_rook_ as CSYTRS_ROOK;
pub use ctbcon_ as CTBCON;
pub use ctbrfs_ as CTBRFS;
pub use ctbtrs_ as CTBTRS;
pub use ctfsm_ as CTFSM;
pub use ctftri_ as CTFTRI;
pub use ctfttp_ as CTFTTP;
pub use ctfttr_ as CTFTTR;
pub use ctgevc_ as CTGEVC;
pub use ctgex2_ as CTGEX2;
pub use ctgexc_ as CTGEXC;
pub use ctgsen_ as CTGSEN;
pub use ctgsja_ as CTGSJA;
pub use ctgsna_ as CTGSNA;
pub use ctgsy2_ as CTGSY2;
pub use ctgsyl_ as CTGSYL;
pub use ctpcon_ as CTPCON;
pub use ctplqt2_ as CTPLQT2;
pub use ctplqt_ as CTPLQT;
pub use ctpmlqt_ as CTPMLQT;
pub use ctpmqrt_ as CTPMQRT;
pub use ctpqrt2_ as CTPQRT2;
pub use ctpqrt_ as CTPQRT;
pub use ctprfb_ as CTPRFB;
pub use ctprfs_ as CTPRFS;
pub use ctptri_ as CTPTRI;
pub use ctptrs_ as CTPTRS;
pub use ctpttf_ as CTPTTF;
pub use ctpttr_ as CTPTTR;
pub use ctrcon_ as CTRCON;
pub use ctrevc3_ as CTREVC3;
pub use ctrevc_ as CTREVC;
pub use ctrexc_ as CTREXC;
pub use ctrrfs_ as CTRRFS;
pub use ctrsen_ as CTRSEN;
pub use ctrsna_ as CTRSNA;
pub use ctrsyl3_ as CTRSYL3;
pub use ctrsyl_ as CTRSYL;
pub use ctrti2_ as CTRTI2;
pub use ctrtri_ as CTRTRI;
pub use ctrtrs_ as CTRTRS;
pub use ctrttf_ as CTRTTF;
pub use ctrttp_ as CTRTTP;
pub use ctzrqf_ as CTZRQF;
pub use ctzrzf_ as CTZRZF;
pub use cunbdb1_ as CUNBDB1;
pub use cunbdb2_ as CUNBDB2;
pub use cunbdb3_ as CUNBDB3;
pub use cunbdb4_ as CUNBDB4;
pub use cunbdb5_ as CUNBDB5;
pub use cunbdb6_ as CUNBDB6;
pub use cunbdb_ as CUNBDB;
pub use cuncsd2by1_ as CUNCSD2BY1;
pub use cuncsd_ as CUNCSD;
pub use cung2l_ as CUNG2L;
pub use cung2r_ as CUNG2R;
pub use cungbr_ as CUNGBR;
pub use cunghr_ as CUNGHR;
pub use cungl2_ as CUNGL2;
pub use cunglq_ as CUNGLQ;
pub use cungql_ as CUNGQL;
pub use cungqr_ as CUNGQR;
pub use cungr2_ as CUNGR2;
pub use cungrq_ as CUNGRQ;
pub use cungtr_ as CUNGTR;
pub use cungtsqr_ as CUNGTSQR;
pub use cungtsqr_row_ as CUNGTSQR_ROW;
pub use cunhr_col_ as CUNHR_COL;
pub use cunm22_ as CUNM22;
pub use cunm2l_ as CUNM2L;
pub use cunm2r_ as CUNM2R;
pub use cunmbr_ as CUNMBR;
pub use cunmhr_ as CUNMHR;
pub use cunml2_ as CUNML2;
pub use cunmlq_ as CUNMLQ;
pub use cunmql_ as CUNMQL;
pub use cunmqr_ as CUNMQR;
pub use cunmr2_ as CUNMR2;
pub use cunmr3_ as CUNMR3;
pub use cunmrq_ as CUNMRQ;
pub use cunmrz_ as CUNMRZ;
pub use cunmtr_ as CUNMTR;
pub use cupgtr_ as CUPGTR;
pub use cupmtr_ as CUPMTR;
pub use dbbcsd_ as DBBCSD;
pub use dbdsdc_ as DBDSDC;
pub use dbdsqr_ as DBDSQR;
pub use dbdsvdx_ as DBDSVDX;
pub use ddisna_ as DDISNA;
pub use ddtsvb_ as DDTSVB;
pub use ddttrfb_ as DDTTRFB;
pub use ddttrsb_ as DDTTRSB;
pub use dgbbrd_ as DGBBRD;
pub use dgbcon_ as DGBCON;
pub use dgbequ_ as DGBEQU;
pub use dgbequb_ as DGBEQUB;
pub use dgbrfs_ as DGBRFS;
pub use dgbrfsx_ as DGBRFSX;
pub use dgbsv_ as DGBSV;
pub use dgbsvx_ as DGBSVX;
pub use dgbsvxx_ as DGBSVXX;
pub use dgbtf2_ as DGBTF2;
pub use dgbtrf_ as DGBTRF;
pub use dgbtrs_ as DGBTRS;
pub use dgebak_ as DGEBAK;
pub use dgebal_ as DGEBAL;
pub use dgebd2_ as DGEBD2;
pub use dgebrd_ as DGEBRD;
pub use dgecon_ as DGECON;
pub use dgedmd_ as DGEDMD;
pub use dgedmdq_ as DGEDMDQ;
pub use dgeequ_ as DGEEQU;
pub use dgeequb_ as DGEEQUB;
pub use dgees_ as DGEES;
pub use dgeesx_ as DGEESX;
pub use dgeev_ as DGEEV;
pub use dgeevx_ as DGEEVX;
pub use dgegs_ as DGEGS;
pub use dgegv_ as DGEGV;
pub use dgehd2_ as DGEHD2;
pub use dgehrd_ as DGEHRD;
pub use dgejsv_ as DGEJSV;
pub use dgelq2_ as DGELQ2;
pub use dgelq_ as DGELQ;
pub use dgelqf_ as DGELQF;
pub use dgelqt3_ as DGELQT3;
pub use dgelqt_ as DGELQT;
pub use dgels_ as DGELS;
pub use dgels_batch_strided_ as DGELS_BATCH_STRIDED;
pub use dgelsd_ as DGELSD;
pub use dgelss_ as DGELSS;
pub use dgelst_ as DGELST;
pub use dgelsx_ as DGELSX;
pub use dgelsy_ as DGELSY;
pub use dgemlq_ as DGEMLQ;
pub use dgemlqt_ as DGEMLQT;
pub use dgemqr_ as DGEMQR;
pub use dgemqrt_ as DGEMQRT;
pub use dgeql2_ as DGEQL2;
pub use dgeqlf_ as DGEQLF;
pub use dgeqp3_ as DGEQP3;
pub use dgeqp3rk_ as DGEQP3RK;
pub use dgeqpf_ as DGEQPF;
pub use dgeqr2_ as DGEQR2;
pub use dgeqr2p_ as DGEQR2P;
pub use dgeqr_ as DGEQR;
pub use dgeqrf_ as DGEQRF;
pub use dgeqrfp_ as DGEQRFP;
pub use dgeqrt2_ as DGEQRT2;
pub use dgeqrt3_ as DGEQRT3;
pub use dgeqrt_ as DGEQRT;
pub use dgerfs_ as DGERFS;
pub use dgerfsx_ as DGERFSX;
pub use dgerq2_ as DGERQ2;
pub use dgerqf_ as DGERQF;
pub use dgesc2_ as DGESC2;
pub use dgesdd_ as DGESDD;
pub use dgesv_ as DGESV;
pub use dgesvd_ as DGESVD;
pub use dgesvda_batch_strided_ as DGESVDA_BATCH_STRIDED;
pub use dgesvdq_ as DGESVDQ;
pub use dgesvdx_ as DGESVDX;
pub use dgesvj_ as DGESVJ;
pub use dgesvx_ as DGESVX;
pub use dgesvxx_ as DGESVXX;
pub use dgetc2_ as DGETC2;
pub use dgetf2_ as DGETF2;
pub use dgetrf2_ as DGETRF2;
pub use dgetrf_ as DGETRF;
pub use dgetrf_batch_ as DGETRF_BATCH;
pub use dgetrf_batch_strided_ as DGETRF_BATCH_STRIDED;
pub use dgetrfnp_batch_ as DGETRFNP_BATCH;
pub use dgetrfnp_batch_strided_ as DGETRFNP_BATCH_STRIDED;
pub use dgetri_ as DGETRI;
pub use dgetri_batch_strided_ as DGETRI_BATCH_STRIDED;
pub use dgetri_oop_batch_ as DGETRI_OOP_BATCH;
pub use dgetri_oop_batch_strided_ as DGETRI_OOP_BATCH_STRIDED;
pub use dgetrs_ as DGETRS;
pub use dgetrs_batch_strided_ as DGETRS_BATCH_STRIDED;
pub use dgetrsnp_batch_strided_ as DGETRSNP_BATCH_STRIDED;
pub use dgetsls_ as DGETSLS;
pub use dgetsqrhrt_ as DGETSQRHRT;
pub use dggbak_ as DGGBAK;
pub use dggbal_ as DGGBAL;
pub use dgges3_ as DGGES3;
pub use dgges_ as DGGES;
pub use dggesx_ as DGGESX;
pub use dggev3_ as DGGEV3;
pub use dggev_ as DGGEV;
pub use dggevx_ as DGGEVX;
pub use dggglm_ as DGGGLM;
pub use dgghd3_ as DGGHD3;
pub use dgghrd_ as DGGHRD;
pub use dgglse_ as DGGLSE;
pub use dggqrf_ as DGGQRF;
pub use dggrqf_ as DGGRQF;
pub use dggsvd3_ as DGGSVD3;
pub use dggsvd_ as DGGSVD;
pub use dggsvp3_ as DGGSVP3;
pub use dggsvp_ as DGGSVP;
pub use dgsvj0_ as DGSVJ0;
pub use dgsvj1_ as DGSVJ1;
pub use dgtcon_ as DGTCON;
pub use dgtrfs_ as DGTRFS;
pub use dgtsv_ as DGTSV;
pub use dgtsvx_ as DGTSVX;
pub use dgttrf_ as DGTTRF;
pub use dgttrs_ as DGTTRS;
pub use dgtts2_ as DGTTS2;
pub use dhgeqz_ as DHGEQZ;
pub use dhsein_ as DHSEIN;
pub use dhseqr_ as DHSEQR;
pub use disnan_ as DISNAN;
pub use dla_gbamv_ as DLA_GBAMV;
pub use dla_gbrcond_ as DLA_GBRCOND;
pub use dla_gbrfsx_extended_ as DLA_GBRFSX_EXTENDED;
pub use dla_gbrpvgrw_ as DLA_GBRPVGRW;
pub use dla_geamv_ as DLA_GEAMV;
pub use dla_gercond_ as DLA_GERCOND;
pub use dla_gerfsx_extended_ as DLA_GERFSX_EXTENDED;
pub use dla_gerpvgrw_ as DLA_GERPVGRW;
pub use dla_lin_berr_ as DLA_LIN_BERR;
pub use dla_porcond_ as DLA_PORCOND;
pub use dla_porfsx_extended_ as DLA_PORFSX_EXTENDED;
pub use dla_porpvgrw_ as DLA_PORPVGRW;
pub use dla_syamv_ as DLA_SYAMV;
pub use dla_syrcond_ as DLA_SYRCOND;
pub use dla_syrfsx_extended_ as DLA_SYRFSX_EXTENDED;
pub use dla_syrpvgrw_ as DLA_SYRPVGRW;
pub use dla_wwaddw_ as DLA_WWADDW;
pub use dlabad_ as DLABAD;
pub use dlabrd_ as DLABRD;
pub use dlacn2_ as DLACN2;
pub use dlacon_ as DLACON;
pub use dlacpy_ as DLACPY;
pub use dladiv_ as DLADIV;
pub use dlae2_ as DLAE2;
pub use dlaebz_ as DLAEBZ;
pub use dlaed0_ as DLAED0;
pub use dlaed1_ as DLAED1;
pub use dlaed2_ as DLAED2;
pub use dlaed3_ as DLAED3;
pub use dlaed4_ as DLAED4;
pub use dlaed5_ as DLAED5;
pub use dlaed6_ as DLAED6;
pub use dlaed7_ as DLAED7;
pub use dlaed8_ as DLAED8;
pub use dlaed9_ as DLAED9;
pub use dlaeda_ as DLAEDA;
pub use dlaein_ as DLAEIN;
pub use dlaev2_ as DLAEV2;
pub use dlaexc_ as DLAEXC;
pub use dlag2_ as DLAG2;
pub use dlag2s_ as DLAG2S;
pub use dlagge_ as DLAGGE;
pub use dlags2_ as DLAGS2;
pub use dlagsy_ as DLAGSY;
pub use dlagtf_ as DLAGTF;
pub use dlagtm_ as DLAGTM;
pub use dlagts_ as DLAGTS;
pub use dlagv2_ as DLAGV2;
pub use dlahqr_ as DLAHQR;
pub use dlahr2_ as DLAHR2;
pub use dlahrd_ as DLAHRD;
pub use dlaic1_ as DLAIC1;
pub use dlaisnan_ as DLAISNAN;
pub use dlakf2_ as DLAKF2;
pub use dlaln2_ as DLALN2;
pub use dlals0_ as DLALS0;
pub use dlalsa_ as DLALSA;
pub use dlalsd_ as DLALSD;
pub use dlamc1_ as DLAMC1;
pub use dlamc2_ as DLAMC2;
pub use dlamc3_ as DLAMC3;
pub use dlamc4_ as DLAMC4;
pub use dlamc5_ as DLAMC5;
pub use dlamch_ as DLAMCH;
pub use dlamrg_ as DLAMRG;
pub use dlamswlq_ as DLAMSWLQ;
pub use dlamtsqr_ as DLAMTSQR;
pub use dlaneg_ as DLANEG;
pub use dlangb_ as DLANGB;
pub use dlange_ as DLANGE;
pub use dlangt_ as DLANGT;
pub use dlanhs_ as DLANHS;
pub use dlansb_ as DLANSB;
pub use dlansf_ as DLANSF;
pub use dlansp_ as DLANSP;
pub use dlanst_ as DLANST;
pub use dlansy_ as DLANSY;
pub use dlantb_ as DLANTB;
pub use dlantp_ as DLANTP;
pub use dlantr_ as DLANTR;
pub use dlanv2_ as DLANV2;
pub use dlaorhr_col_getrfnp2_ as DLAORHR_COL_GETRFNP2;
pub use dlaorhr_col_getrfnp_ as DLAORHR_COL_GETRFNP;
pub use dlapll_ as DLAPLL;
pub use dlapmr_ as DLAPMR;
pub use dlapmt_ as DLAPMT;
pub use dlapy2_ as DLAPY2;
pub use dlapy3_ as DLAPY3;
pub use dlaqgb_ as DLAQGB;
pub use dlaqge_ as DLAQGE;
pub use dlaqp2_ as DLAQP2;
pub use dlaqps_ as DLAQPS;
pub use dlaqr0_ as DLAQR0;
pub use dlaqr1_ as DLAQR1;
pub use dlaqr2_ as DLAQR2;
pub use dlaqr3_ as DLAQR3;
pub use dlaqr4_ as DLAQR4;
pub use dlaqr5_ as DLAQR5;
pub use dlaqsb_ as DLAQSB;
pub use dlaqsp_ as DLAQSP;
pub use dlaqsy_ as DLAQSY;
pub use dlaqtr_ as DLAQTR;
pub use dlaqz0_ as DLAQZ0;
pub use dlaqz1_ as DLAQZ1;
pub use dlaqz2_ as DLAQZ2;
pub use dlaqz3_ as DLAQZ3;
pub use dlaqz4_ as DLAQZ4;
pub use dlar1v_ as DLAR1V;
pub use dlar2v_ as DLAR2V;
pub use dlaran_ as DLARAN;
pub use dlarf1f_ as DLARF1F;
pub use dlarf1l_ as DLARF1L;
pub use dlarf_ as DLARF;
pub use dlarfb_ as DLARFB;
pub use dlarfb_gett_ as DLARFB_GETT;
pub use dlarfg_ as DLARFG;
pub use dlarfgp_ as DLARFGP;
pub use dlarfp_ as DLARFP;
pub use dlarft_ as DLARFT;
pub use dlarfx_ as DLARFX;
pub use dlarfy_ as DLARFY;
pub use dlarge_ as DLARGE;
pub use dlargv_ as DLARGV;
pub use dlarnd_ as DLARND;
pub use dlarnv_ as DLARNV;
pub use dlaror_ as DLAROR;
pub use dlarot_ as DLAROT;
pub use dlarra_ as DLARRA;
pub use dlarrb_ as DLARRB;
pub use dlarrc_ as DLARRC;
pub use dlarrd_ as DLARRD;
pub use dlarre_ as DLARRE;
pub use dlarrf_ as DLARRF;
pub use dlarrj_ as DLARRJ;
pub use dlarrk_ as DLARRK;
pub use dlarrr_ as DLARRR;
pub use dlarrv_ as DLARRV;
pub use dlarscl2_ as DLARSCL2;
pub use dlartg_ as DLARTG;
pub use dlartgp_ as DLARTGP;
pub use dlartgs_ as DLARTGS;
pub use dlartv_ as DLARTV;
pub use dlaruv_ as DLARUV;
pub use dlarz_ as DLARZ;
pub use dlarzb_ as DLARZB;
pub use dlarzt_ as DLARZT;
pub use dlas2_ as DLAS2;
pub use dlascl2_ as DLASCL2;
pub use dlascl_ as DLASCL;
pub use dlasd0_ as DLASD0;
pub use dlasd1_ as DLASD1;
pub use dlasd2_ as DLASD2;
pub use dlasd3_ as DLASD3;
pub use dlasd4_ as DLASD4;
pub use dlasd5_ as DLASD5;
pub use dlasd6_ as DLASD6;
pub use dlasd7_ as DLASD7;
pub use dlasd8_ as DLASD8;
pub use dlasda_ as DLASDA;
pub use dlasdq_ as DLASDQ;
pub use dlasdt_ as DLASDT;
pub use dlaset_ as DLASET;
pub use dlasq1_ as DLASQ1;
pub use dlasq2_ as DLASQ2;
pub use dlasq3_ as DLASQ3;
pub use dlasq4_ as DLASQ4;
pub use dlasq5_ as DLASQ5;
pub use dlasq6_ as DLASQ6;
pub use dlasr_ as DLASR;
pub use dlasrt_ as DLASRT;
pub use dlassq_ as DLASSQ;
pub use dlasv2_ as DLASV2;
pub use dlaswlq_ as DLASWLQ;
pub use dlaswp_ as DLASWP;
pub use dlasy2_ as DLASY2;
pub use dlasyf_ as DLASYF;
pub use dlasyf_aa_ as DLASYF_AA;
pub use dlasyf_rk_ as DLASYF_RK;
pub use dlasyf_rook_ as DLASYF_ROOK;
pub use dlat2s_ as DLAT2S;
pub use dlatbs_ as DLATBS;
pub use dlatdf_ as DLATDF;
pub use dlatm1_ as DLATM1;
pub use dlatm2_ as DLATM2;
pub use dlatm3_ as DLATM3;
pub use dlatm5_ as DLATM5;
pub use dlatm6_ as DLATM6;
pub use dlatme_ as DLATME;
pub use dlatmr_ as DLATMR;
pub use dlatms_ as DLATMS;
pub use dlatps_ as DLATPS;
pub use dlatrd_ as DLATRD;
pub use dlatrs3_ as DLATRS3;
pub use dlatrs_ as DLATRS;
pub use dlatrz_ as DLATRZ;
pub use dlatsqr_ as DLATSQR;
pub use dlatzm_ as DLATZM;
pub use dlauu2_ as DLAUU2;
pub use dlauum_ as DLAUUM;
pub use dopgtr_ as DOPGTR;
pub use dopmtr_ as DOPMTR;
pub use dorbdb1_ as DORBDB1;
pub use dorbdb2_ as DORBDB2;
pub use dorbdb3_ as DORBDB3;
pub use dorbdb4_ as DORBDB4;
pub use dorbdb5_ as DORBDB5;
pub use dorbdb6_ as DORBDB6;
pub use dorbdb_ as DORBDB;
pub use dorcsd2by1_ as DORCSD2BY1;
pub use dorcsd_ as DORCSD;
pub use dorg2l_ as DORG2L;
pub use dorg2r_ as DORG2R;
pub use dorgbr_ as DORGBR;
pub use dorghr_ as DORGHR;
pub use dorgl2_ as DORGL2;
pub use dorglq_ as DORGLQ;
pub use dorgql_ as DORGQL;
pub use dorgqr_ as DORGQR;
pub use dorgr2_ as DORGR2;
pub use dorgrq_ as DORGRQ;
pub use dorgtr_ as DORGTR;
pub use dorgtsqr_ as DORGTSQR;
pub use dorgtsqr_row_ as DORGTSQR_ROW;
pub use dorhr_col_ as DORHR_COL;
pub use dorm22_ as DORM22;
pub use dorm2l_ as DORM2L;
pub use dorm2r_ as DORM2R;
pub use dormbr_ as DORMBR;
pub use dormhr_ as DORMHR;
pub use dorml2_ as DORML2;
pub use dormlq_ as DORMLQ;
pub use dormql_ as DORMQL;
pub use dormqr_ as DORMQR;
pub use dormr2_ as DORMR2;
pub use dormr3_ as DORMR3;
pub use dormrq_ as DORMRQ;
pub use dormrz_ as DORMRZ;
pub use dormtr_ as DORMTR;
pub use dpbcon_ as DPBCON;
pub use dpbequ_ as DPBEQU;
pub use dpbrfs_ as DPBRFS;
pub use dpbstf_ as DPBSTF;
pub use dpbsv_ as DPBSV;
pub use dpbsvx_ as DPBSVX;
pub use dpbtf2_ as DPBTF2;
pub use dpbtrf_ as DPBTRF;
pub use dpbtrs_ as DPBTRS;
pub use dpftrf_ as DPFTRF;
pub use dpftri_ as DPFTRI;
pub use dpftrs_ as DPFTRS;
pub use dpocon_ as DPOCON;
pub use dpoequ_ as DPOEQU;
pub use dpoequb_ as DPOEQUB;
pub use dporfs_ as DPORFS;
pub use dporfsx_ as DPORFSX;
pub use dposv_ as DPOSV;
pub use dposvx_ as DPOSVX;
pub use dposvxx_ as DPOSVXX;
pub use dpotf2_ as DPOTF2;
pub use dpotrf2_ as DPOTRF2;
pub use dpotrf_ as DPOTRF;
pub use dpotri_ as DPOTRI;
pub use dpotrs_ as DPOTRS;
pub use dppcon_ as DPPCON;
pub use dppequ_ as DPPEQU;
pub use dpprfs_ as DPPRFS;
pub use dppsv_ as DPPSV;
pub use dppsvx_ as DPPSVX;
pub use dpptrf_ as DPPTRF;
pub use dpptri_ as DPPTRI;
pub use dpptrs_ as DPPTRS;
pub use dpstf2_ as DPSTF2;
pub use dpstrf_ as DPSTRF;
pub use dptcon_ as DPTCON;
pub use dpteqr_ as DPTEQR;
pub use dptrfs_ as DPTRFS;
pub use dptsv_ as DPTSV;
pub use dptsvx_ as DPTSVX;
pub use dpttrf_ as DPTTRF;
pub use dpttrs_ as DPTTRS;
pub use dptts2_ as DPTTS2;
pub use drscl_ as DRSCL;
pub use dsb2st_kernels_ as DSB2ST_KERNELS;
pub use dsbev_ as DSBEV;
pub use dsbev_2stage_ as DSBEV_2STAGE;
pub use dsbevd_ as DSBEVD;
pub use dsbevd_2stage_ as DSBEVD_2STAGE;
pub use dsbevx_ as DSBEVX;
pub use dsbevx_2stage_ as DSBEVX_2STAGE;
pub use dsbgst_ as DSBGST;
pub use dsbgv_ as DSBGV;
pub use dsbgvd_ as DSBGVD;
pub use dsbgvx_ as DSBGVX;
pub use dsbtrd_ as DSBTRD;
pub use dsecnd_ as DSECND;
pub use dsfrk_ as DSFRK;
pub use dsgesv_ as DSGESV;
pub use dspcon_ as DSPCON;
pub use dspev_ as DSPEV;
pub use dspevd_ as DSPEVD;
pub use dspevx_ as DSPEVX;
pub use dspgst_ as DSPGST;
pub use dspgv_ as DSPGV;
pub use dspgvd_ as DSPGVD;
pub use dspgvx_ as DSPGVX;
pub use dsposv_ as DSPOSV;
pub use dsprfs_ as DSPRFS;
pub use dspsv_ as DSPSV;
pub use dspsvx_ as DSPSVX;
pub use dsptrd_ as DSPTRD;
pub use dsptrf_ as DSPTRF;
pub use dsptri_ as DSPTRI;
pub use dsptrs_ as DSPTRS;
pub use dstebz_ as DSTEBZ;
pub use dstedc_ as DSTEDC;
pub use dstegr_ as DSTEGR;
pub use dstein_ as DSTEIN;
pub use dstemr_ as DSTEMR;
pub use dsteqr_ as DSTEQR;
pub use dsterf_ as DSTERF;
pub use dstev_ as DSTEV;
pub use dstevd_ as DSTEVD;
pub use dstevr_ as DSTEVR;
pub use dstevx_ as DSTEVX;
pub use dsycon_ as DSYCON;
pub use dsycon_3_ as DSYCON_3;
pub use dsycon_rook_ as DSYCON_ROOK;
pub use dsyconv_ as DSYCONV;
pub use dsyconvf_ as DSYCONVF;
pub use dsyconvf_rook_ as DSYCONVF_ROOK;
pub use dsyequb_ as DSYEQUB;
pub use dsyev_ as DSYEV;
pub use dsyev_2stage_ as DSYEV_2STAGE;
pub use dsyevd_ as DSYEVD;
pub use dsyevd_2stage_ as DSYEVD_2STAGE;
pub use dsyevr_ as DSYEVR;
pub use dsyevr_2stage_ as DSYEVR_2STAGE;
pub use dsyevx_ as DSYEVX;
pub use dsyevx_2stage_ as DSYEVX_2STAGE;
pub use dsygs2_ as DSYGS2;
pub use dsygst_ as DSYGST;
pub use dsygv_ as DSYGV;
pub use dsygv_2stage_ as DSYGV_2STAGE;
pub use dsygvd_ as DSYGVD;
pub use dsygvx_ as DSYGVX;
pub use dsyrdb_ as DSYRDB;
pub use dsyrfs_ as DSYRFS;
pub use dsyrfsx_ as DSYRFSX;
pub use dsysv_ as DSYSV;
pub use dsysv_aa_ as DSYSV_AA;
pub use dsysv_aa_2stage_ as DSYSV_AA_2STAGE;
pub use dsysv_rk_ as DSYSV_RK;
pub use dsysv_rook_ as DSYSV_ROOK;
pub use dsysvx_ as DSYSVX;
pub use dsysvxx_ as DSYSVXX;
pub use dsyswapr_ as DSYSWAPR;
pub use dsytd2_ as DSYTD2;
pub use dsytf2_ as DSYTF2;
pub use dsytf2_rk_ as DSYTF2_RK;
pub use dsytf2_rook_ as DSYTF2_ROOK;
pub use dsytrd_ as DSYTRD;
pub use dsytrd_2stage_ as DSYTRD_2STAGE;
pub use dsytrd_sb2st_ as DSYTRD_SB2ST;
pub use dsytrd_sy2sb_ as DSYTRD_SY2SB;
pub use dsytrf_ as DSYTRF;
pub use dsytrf_aa_ as DSYTRF_AA;
pub use dsytrf_aa_2stage_ as DSYTRF_AA_2STAGE;
pub use dsytrf_rk_ as DSYTRF_RK;
pub use dsytrf_rook_ as DSYTRF_ROOK;
pub use dsytri2_ as DSYTRI2;
pub use dsytri2x_ as DSYTRI2X;
pub use dsytri_ as DSYTRI;
pub use dsytri_3_ as DSYTRI_3;
pub use dsytri_3x_ as DSYTRI_3X;
pub use dsytri_rook_ as DSYTRI_ROOK;
pub use dsytrs2_ as DSYTRS2;
pub use dsytrs_ as DSYTRS;
pub use dsytrs_3_ as DSYTRS_3;
pub use dsytrs_aa_ as DSYTRS_AA;
pub use dsytrs_aa_2stage_ as DSYTRS_AA_2STAGE;
pub use dsytrs_rook_ as DSYTRS_ROOK;
pub use dtbcon_ as DTBCON;
pub use dtbrfs_ as DTBRFS;
pub use dtbtrs_ as DTBTRS;
pub use dtfsm_ as DTFSM;
pub use dtftri_ as DTFTRI;
pub use dtfttp_ as DTFTTP;
pub use dtfttr_ as DTFTTR;
pub use dtgevc_ as DTGEVC;
pub use dtgex2_ as DTGEX2;
pub use dtgexc_ as DTGEXC;
pub use dtgsen_ as DTGSEN;
pub use dtgsja_ as DTGSJA;
pub use dtgsna_ as DTGSNA;
pub use dtgsy2_ as DTGSY2;
pub use dtgsyl_ as DTGSYL;
pub use dtpcon_ as DTPCON;
pub use dtplqt2_ as DTPLQT2;
pub use dtplqt_ as DTPLQT;
pub use dtpmlqt_ as DTPMLQT;
pub use dtpmqrt_ as DTPMQRT;
pub use dtpqrt2_ as DTPQRT2;
pub use dtpqrt_ as DTPQRT;
pub use dtprfb_ as DTPRFB;
pub use dtprfs_ as DTPRFS;
pub use dtptri_ as DTPTRI;
pub use dtptrs_ as DTPTRS;
pub use dtpttf_ as DTPTTF;
pub use dtpttr_ as DTPTTR;
pub use dtrcon_ as DTRCON;
pub use dtrevc3_ as DTREVC3;
pub use dtrevc_ as DTREVC;
pub use dtrexc_ as DTREXC;
pub use dtrrfs_ as DTRRFS;
pub use dtrsen_ as DTRSEN;
pub use dtrsna_ as DTRSNA;
pub use dtrsyl3_ as DTRSYL3;
pub use dtrsyl_ as DTRSYL;
pub use dtrti2_ as DTRTI2;
pub use dtrtri_ as DTRTRI;
pub use dtrtrs_ as DTRTRS;
pub use dtrttf_ as DTRTTF;
pub use dtrttp_ as DTRTTP;
pub use dtzrqf_ as DTZRQF;
pub use dtzrzf_ as DTZRZF;
pub use dzsum1_ as DZSUM1;
pub use icmax1_ as ICMAX1;
pub use ieeeck_ as IEEECK;
pub use ilaclc_ as ILACLC;
pub use ilaclr_ as ILACLR;
pub use iladiag_ as ILADIAG;
pub use iladlc_ as ILADLC;
pub use iladlr_ as ILADLR;
pub use ilaenv2stage_ as ILAENV2STAGE;
pub use ilaenv_ as ILAENV;
pub use ilaprec_ as ILAPREC;
pub use ilaslc_ as ILASLC;
pub use ilaslr_ as ILASLR;
pub use ilatrans_ as ILATRANS;
pub use ilauplo_ as ILAUPLO;
pub use ilaver_ as ILAVER;
pub use ilazlc_ as ILAZLC;
pub use ilazlr_ as ILAZLR;
pub use iparam2stage_ as IPARAM2STAGE;
pub use iparmq_ as IPARMQ;
pub use izmax1_ as IZMAX1;
pub use lsamen_ as LSAMEN;
pub use mkl_cgetrfnp_ as MKL_CGETRFNP;
pub use mkl_cgetrfnpi_ as MKL_CGETRFNPI;
pub use mkl_cgetrinp_ as MKL_CGETRINP;
pub use mkl_cspffrt2_ as MKL_CSPFFRT2;
pub use mkl_cspffrtx_ as MKL_CSPFFRTX;
pub use mkl_ctppack_ as MKL_CTPPACK;
pub use mkl_ctpunpack_ as MKL_CTPUNPACK;
pub use mkl_dgetrfnp_ as MKL_DGETRFNP;
pub use mkl_dgetrfnpi_ as MKL_DGETRFNPI;
pub use mkl_dgetrinp_ as MKL_DGETRINP;
pub use mkl_dspffrt2_ as MKL_DSPFFRT2;
pub use mkl_dspffrtx_ as MKL_DSPFFRTX;
pub use mkl_dtppack_ as MKL_DTPPACK;
pub use mkl_dtpunpack_ as MKL_DTPUNPACK;
pub use mkl_progress_ as MKL_PROGRESS;
pub use mkl_sgetrfnp_ as MKL_SGETRFNP;
pub use mkl_sgetrfnpi_ as MKL_SGETRFNPI;
pub use mkl_sgetrinp_ as MKL_SGETRINP;
pub use mkl_sspffrt2_ as MKL_SSPFFRT2;
pub use mkl_sspffrtx_ as MKL_SSPFFRTX;
pub use mkl_stppack_ as MKL_STPPACK;
pub use mkl_stpunpack_ as MKL_STPUNPACK;
pub use mkl_zgetrfnp_ as MKL_ZGETRFNP;
pub use mkl_zgetrfnpi_ as MKL_ZGETRFNPI;
pub use mkl_zgetrinp_ as MKL_ZGETRINP;
pub use mkl_zspffrt2_ as MKL_ZSPFFRT2;
pub use mkl_zspffrtx_ as MKL_ZSPFFRTX;
pub use mkl_ztppack_ as MKL_ZTPPACK;
pub use mkl_ztpunpack_ as MKL_ZTPUNPACK;
pub use sbbcsd_ as SBBCSD;
pub use sbdsdc_ as SBDSDC;
pub use sbdsqr_ as SBDSQR;
pub use sbdsvdx_ as SBDSVDX;
pub use scsum1_ as SCSUM1;
pub use sdisna_ as SDISNA;
pub use sdtsvb_ as SDTSVB;
pub use sdttrfb_ as SDTTRFB;
pub use sdttrsb_ as SDTTRSB;
pub use second_ as SECOND;
pub use sgbbrd_ as SGBBRD;
pub use sgbcon_ as SGBCON;
pub use sgbequ_ as SGBEQU;
pub use sgbequb_ as SGBEQUB;
pub use sgbrfs_ as SGBRFS;
pub use sgbrfsx_ as SGBRFSX;
pub use sgbsv_ as SGBSV;
pub use sgbsvx_ as SGBSVX;
pub use sgbsvxx_ as SGBSVXX;
pub use sgbtf2_ as SGBTF2;
pub use sgbtrf_ as SGBTRF;
pub use sgbtrs_ as SGBTRS;
pub use sgebak_ as SGEBAK;
pub use sgebal_ as SGEBAL;
pub use sgebd2_ as SGEBD2;
pub use sgebrd_ as SGEBRD;
pub use sgecon_ as SGECON;
pub use sgedmd_ as SGEDMD;
pub use sgedmdq_ as SGEDMDQ;
pub use sgeequ_ as SGEEQU;
pub use sgeequb_ as SGEEQUB;
pub use sgees_ as SGEES;
pub use sgeesx_ as SGEESX;
pub use sgeev_ as SGEEV;
pub use sgeevx_ as SGEEVX;
pub use sgegs_ as SGEGS;
pub use sgegv_ as SGEGV;
pub use sgehd2_ as SGEHD2;
pub use sgehrd_ as SGEHRD;
pub use sgejsv_ as SGEJSV;
pub use sgelq2_ as SGELQ2;
pub use sgelq_ as SGELQ;
pub use sgelqf_ as SGELQF;
pub use sgelqt3_ as SGELQT3;
pub use sgelqt_ as SGELQT;
pub use sgels_ as SGELS;
pub use sgels_batch_strided_ as SGELS_BATCH_STRIDED;
pub use sgelsd_ as SGELSD;
pub use sgelss_ as SGELSS;
pub use sgelst_ as SGELST;
pub use sgelsx_ as SGELSX;
pub use sgelsy_ as SGELSY;
pub use sgemlq_ as SGEMLQ;
pub use sgemlqt_ as SGEMLQT;
pub use sgemqr_ as SGEMQR;
pub use sgemqrt_ as SGEMQRT;
pub use sgeql2_ as SGEQL2;
pub use sgeqlf_ as SGEQLF;
pub use sgeqp3_ as SGEQP3;
pub use sgeqp3rk_ as SGEQP3RK;
pub use sgeqpf_ as SGEQPF;
pub use sgeqr2_ as SGEQR2;
pub use sgeqr2p_ as SGEQR2P;
pub use sgeqr_ as SGEQR;
pub use sgeqrf_ as SGEQRF;
pub use sgeqrfp_ as SGEQRFP;
pub use sgeqrt2_ as SGEQRT2;
pub use sgeqrt3_ as SGEQRT3;
pub use sgeqrt_ as SGEQRT;
pub use sgerfs_ as SGERFS;
pub use sgerfsx_ as SGERFSX;
pub use sgerq2_ as SGERQ2;
pub use sgerqf_ as SGERQF;
pub use sgesc2_ as SGESC2;
pub use sgesdd_ as SGESDD;
pub use sgesv_ as SGESV;
pub use sgesvd_ as SGESVD;
pub use sgesvda_batch_strided_ as SGESVDA_BATCH_STRIDED;
pub use sgesvdq_ as SGESVDQ;
pub use sgesvdx_ as SGESVDX;
pub use sgesvj_ as SGESVJ;
pub use sgesvx_ as SGESVX;
pub use sgesvxx_ as SGESVXX;
pub use sgetc2_ as SGETC2;
pub use sgetf2_ as SGETF2;
pub use sgetrf2_ as SGETRF2;
pub use sgetrf_ as SGETRF;
pub use sgetrf_batch_ as SGETRF_BATCH;
pub use sgetrf_batch_strided_ as SGETRF_BATCH_STRIDED;
pub use sgetrfnp_batch_ as SGETRFNP_BATCH;
pub use sgetrfnp_batch_strided_ as SGETRFNP_BATCH_STRIDED;
pub use sgetri_ as SGETRI;
pub use sgetri_batch_strided_ as SGETRI_BATCH_STRIDED;
pub use sgetri_oop_batch_ as SGETRI_OOP_BATCH;
pub use sgetri_oop_batch_strided_ as SGETRI_OOP_BATCH_STRIDED;
pub use sgetrs_ as SGETRS;
pub use sgetrs_batch_strided_ as SGETRS_BATCH_STRIDED;
pub use sgetrsnp_batch_strided_ as SGETRSNP_BATCH_STRIDED;
pub use sgetsls_ as SGETSLS;
pub use sgetsqrhrt_ as SGETSQRHRT;
pub use sggbak_ as SGGBAK;
pub use sggbal_ as SGGBAL;
pub use sgges3_ as SGGES3;
pub use sgges_ as SGGES;
pub use sggesx_ as SGGESX;
pub use sggev3_ as SGGEV3;
pub use sggev_ as SGGEV;
pub use sggevx_ as SGGEVX;
pub use sggglm_ as SGGGLM;
pub use sgghd3_ as SGGHD3;
pub use sgghrd_ as SGGHRD;
pub use sgglse_ as SGGLSE;
pub use sggqrf_ as SGGQRF;
pub use sggrqf_ as SGGRQF;
pub use sggsvd3_ as SGGSVD3;
pub use sggsvd_ as SGGSVD;
pub use sggsvp3_ as SGGSVP3;
pub use sggsvp_ as SGGSVP;
pub use sgsvj0_ as SGSVJ0;
pub use sgsvj1_ as SGSVJ1;
pub use sgtcon_ as SGTCON;
pub use sgtrfs_ as SGTRFS;
pub use sgtsv_ as SGTSV;
pub use sgtsvx_ as SGTSVX;
pub use sgttrf_ as SGTTRF;
pub use sgttrs_ as SGTTRS;
pub use sgtts2_ as SGTTS2;
pub use shgeqz_ as SHGEQZ;
pub use shsein_ as SHSEIN;
pub use shseqr_ as SHSEQR;
pub use sisnan_ as SISNAN;
pub use sla_gbamv_ as SLA_GBAMV;
pub use sla_gbrcond_ as SLA_GBRCOND;
pub use sla_gbrfsx_extended_ as SLA_GBRFSX_EXTENDED;
pub use sla_gbrpvgrw_ as SLA_GBRPVGRW;
pub use sla_geamv_ as SLA_GEAMV;
pub use sla_gercond_ as SLA_GERCOND;
pub use sla_gerfsx_extended_ as SLA_GERFSX_EXTENDED;
pub use sla_gerpvgrw_ as SLA_GERPVGRW;
pub use sla_lin_berr_ as SLA_LIN_BERR;
pub use sla_porcond_ as SLA_PORCOND;
pub use sla_porfsx_extended_ as SLA_PORFSX_EXTENDED;
pub use sla_porpvgrw_ as SLA_PORPVGRW;
pub use sla_syamv_ as SLA_SYAMV;
pub use sla_syrcond_ as SLA_SYRCOND;
pub use sla_syrfsx_extended_ as SLA_SYRFSX_EXTENDED;
pub use sla_syrpvgrw_ as SLA_SYRPVGRW;
pub use sla_wwaddw_ as SLA_WWADDW;
pub use slabad_ as SLABAD;
pub use slabrd_ as SLABRD;
pub use slacn2_ as SLACN2;
pub use slacon_ as SLACON;
pub use slacpy_ as SLACPY;
pub use sladiv_ as SLADIV;
pub use slae2_ as SLAE2;
pub use slaebz_ as SLAEBZ;
pub use slaed0_ as SLAED0;
pub use slaed1_ as SLAED1;
pub use slaed2_ as SLAED2;
pub use slaed3_ as SLAED3;
pub use slaed4_ as SLAED4;
pub use slaed5_ as SLAED5;
pub use slaed6_ as SLAED6;
pub use slaed7_ as SLAED7;
pub use slaed8_ as SLAED8;
pub use slaed9_ as SLAED9;
pub use slaeda_ as SLAEDA;
pub use slaein_ as SLAEIN;
pub use slaev2_ as SLAEV2;
pub use slaexc_ as SLAEXC;
pub use slag2_ as SLAG2;
pub use slag2d_ as SLAG2D;
pub use slagge_ as SLAGGE;
pub use slags2_ as SLAGS2;
pub use slagsy_ as SLAGSY;
pub use slagtf_ as SLAGTF;
pub use slagtm_ as SLAGTM;
pub use slagts_ as SLAGTS;
pub use slagv2_ as SLAGV2;
pub use slahqr_ as SLAHQR;
pub use slahr2_ as SLAHR2;
pub use slahrd_ as SLAHRD;
pub use slaic1_ as SLAIC1;
pub use slaisnan_ as SLAISNAN;
pub use slakf2_ as SLAKF2;
pub use slaln2_ as SLALN2;
pub use slals0_ as SLALS0;
pub use slalsa_ as SLALSA;
pub use slalsd_ as SLALSD;
pub use slamc1_ as SLAMC1;
pub use slamc2_ as SLAMC2;
pub use slamc3_ as SLAMC3;
pub use slamc4_ as SLAMC4;
pub use slamc5_ as SLAMC5;
pub use slamch_ as SLAMCH;
pub use slamrg_ as SLAMRG;
pub use slamswlq_ as SLAMSWLQ;
pub use slamtsqr_ as SLAMTSQR;
pub use slaneg_ as SLANEG;
pub use slangb_ as SLANGB;
pub use slange_ as SLANGE;
pub use slangt_ as SLANGT;
pub use slanhs_ as SLANHS;
pub use slansb_ as SLANSB;
pub use slansf_ as SLANSF;
pub use slansp_ as SLANSP;
pub use slanst_ as SLANST;
pub use slansy_ as SLANSY;
pub use slantb_ as SLANTB;
pub use slantp_ as SLANTP;
pub use slantr_ as SLANTR;
pub use slanv2_ as SLANV2;
pub use slaorhr_col_getrfnp2_ as SLAORHR_COL_GETRFNP2;
pub use slaorhr_col_getrfnp_ as SLAORHR_COL_GETRFNP;
pub use slapll_ as SLAPLL;
pub use slapmr_ as SLAPMR;
pub use slapmt_ as SLAPMT;
pub use slapy2_ as SLAPY2;
pub use slapy3_ as SLAPY3;
pub use slaqgb_ as SLAQGB;
pub use slaqge_ as SLAQGE;
pub use slaqp2_ as SLAQP2;
pub use slaqps_ as SLAQPS;
pub use slaqr0_ as SLAQR0;
pub use slaqr1_ as SLAQR1;
pub use slaqr2_ as SLAQR2;
pub use slaqr3_ as SLAQR3;
pub use slaqr4_ as SLAQR4;
pub use slaqr5_ as SLAQR5;
pub use slaqsb_ as SLAQSB;
pub use slaqsp_ as SLAQSP;
pub use slaqsy_ as SLAQSY;
pub use slaqtr_ as SLAQTR;
pub use slaqz0_ as SLAQZ0;
pub use slaqz1_ as SLAQZ1;
pub use slaqz2_ as SLAQZ2;
pub use slaqz3_ as SLAQZ3;
pub use slaqz4_ as SLAQZ4;
pub use slar1v_ as SLAR1V;
pub use slar2v_ as SLAR2V;
pub use slaran_ as SLARAN;
pub use slarf1f_ as SLARF1F;
pub use slarf1l_ as SLARF1L;
pub use slarf_ as SLARF;
pub use slarfb_ as SLARFB;
pub use slarfb_gett_ as SLARFB_GETT;
pub use slarfg_ as SLARFG;
pub use slarfgp_ as SLARFGP;
pub use slarfp_ as SLARFP;
pub use slarft_ as SLARFT;
pub use slarfx_ as SLARFX;
pub use slarfy_ as SLARFY;
pub use slarge_ as SLARGE;
pub use slargv_ as SLARGV;
pub use slarnd_ as SLARND;
pub use slarnv_ as SLARNV;
pub use slaror_ as SLAROR;
pub use slarot_ as SLAROT;
pub use slarra_ as SLARRA;
pub use slarrb_ as SLARRB;
pub use slarrc_ as SLARRC;
pub use slarrd_ as SLARRD;
pub use slarre_ as SLARRE;
pub use slarrf_ as SLARRF;
pub use slarrj_ as SLARRJ;
pub use slarrk_ as SLARRK;
pub use slarrr_ as SLARRR;
pub use slarrv_ as SLARRV;
pub use slarscl2_ as SLARSCL2;
pub use slartg_ as SLARTG;
pub use slartgp_ as SLARTGP;
pub use slartgs_ as SLARTGS;
pub use slartv_ as SLARTV;
pub use slaruv_ as SLARUV;
pub use slarz_ as SLARZ;
pub use slarzb_ as SLARZB;
pub use slarzt_ as SLARZT;
pub use slas2_ as SLAS2;
pub use slascl2_ as SLASCL2;
pub use slascl_ as SLASCL;
pub use slasd0_ as SLASD0;
pub use slasd1_ as SLASD1;
pub use slasd2_ as SLASD2;
pub use slasd3_ as SLASD3;
pub use slasd4_ as SLASD4;
pub use slasd5_ as SLASD5;
pub use slasd6_ as SLASD6;
pub use slasd7_ as SLASD7;
pub use slasd8_ as SLASD8;
pub use slasda_ as SLASDA;
pub use slasdq_ as SLASDQ;
pub use slasdt_ as SLASDT;
pub use slaset_ as SLASET;
pub use slasq1_ as SLASQ1;
pub use slasq2_ as SLASQ2;
pub use slasq3_ as SLASQ3;
pub use slasq4_ as SLASQ4;
pub use slasq5_ as SLASQ5;
pub use slasq6_ as SLASQ6;
pub use slasr_ as SLASR;
pub use slasrt_ as SLASRT;
pub use slassq_ as SLASSQ;
pub use slasv2_ as SLASV2;
pub use slaswlq_ as SLASWLQ;
pub use slaswp_ as SLASWP;
pub use slasy2_ as SLASY2;
pub use slasyf_ as SLASYF;
pub use slasyf_aa_ as SLASYF_AA;
pub use slasyf_rk_ as SLASYF_RK;
pub use slasyf_rook_ as SLASYF_ROOK;
pub use slatbs_ as SLATBS;
pub use slatdf_ as SLATDF;
pub use slatm1_ as SLATM1;
pub use slatm2_ as SLATM2;
pub use slatm3_ as SLATM3;
pub use slatm5_ as SLATM5;
pub use slatm6_ as SLATM6;
pub use slatme_ as SLATME;
pub use slatmr_ as SLATMR;
pub use slatms_ as SLATMS;
pub use slatps_ as SLATPS;
pub use slatrd_ as SLATRD;
pub use slatrs3_ as SLATRS3;
pub use slatrs_ as SLATRS;
pub use slatrz_ as SLATRZ;
pub use slatsqr_ as SLATSQR;
pub use slatzm_ as SLATZM;
pub use slauu2_ as SLAUU2;
pub use slauum_ as SLAUUM;
pub use sopgtr_ as SOPGTR;
pub use sopmtr_ as SOPMTR;
pub use sorbdb1_ as SORBDB1;
pub use sorbdb2_ as SORBDB2;
pub use sorbdb3_ as SORBDB3;
pub use sorbdb4_ as SORBDB4;
pub use sorbdb5_ as SORBDB5;
pub use sorbdb6_ as SORBDB6;
pub use sorbdb_ as SORBDB;
pub use sorcsd2by1_ as SORCSD2BY1;
pub use sorcsd_ as SORCSD;
pub use sorg2l_ as SORG2L;
pub use sorg2r_ as SORG2R;
pub use sorgbr_ as SORGBR;
pub use sorghr_ as SORGHR;
pub use sorgl2_ as SORGL2;
pub use sorglq_ as SORGLQ;
pub use sorgql_ as SORGQL;
pub use sorgqr_ as SORGQR;
pub use sorgr2_ as SORGR2;
pub use sorgrq_ as SORGRQ;
pub use sorgtr_ as SORGTR;
pub use sorgtsqr_ as SORGTSQR;
pub use sorgtsqr_row_ as SORGTSQR_ROW;
pub use sorhr_col_ as SORHR_COL;
pub use sorm22_ as SORM22;
pub use sorm2l_ as SORM2L;
pub use sorm2r_ as SORM2R;
pub use sormbr_ as SORMBR;
pub use sormhr_ as SORMHR;
pub use sorml2_ as SORML2;
pub use sormlq_ as SORMLQ;
pub use sormql_ as SORMQL;
pub use sormqr_ as SORMQR;
pub use sormr2_ as SORMR2;
pub use sormr3_ as SORMR3;
pub use sormrq_ as SORMRQ;
pub use sormrz_ as SORMRZ;
pub use sormtr_ as SORMTR;
pub use spbcon_ as SPBCON;
pub use spbequ_ as SPBEQU;
pub use spbrfs_ as SPBRFS;
pub use spbstf_ as SPBSTF;
pub use spbsv_ as SPBSV;
pub use spbsvx_ as SPBSVX;
pub use spbtf2_ as SPBTF2;
pub use spbtrf_ as SPBTRF;
pub use spbtrs_ as SPBTRS;
pub use spftrf_ as SPFTRF;
pub use spftri_ as SPFTRI;
pub use spftrs_ as SPFTRS;
pub use spocon_ as SPOCON;
pub use spoequ_ as SPOEQU;
pub use spoequb_ as SPOEQUB;
pub use sporfs_ as SPORFS;
pub use sporfsx_ as SPORFSX;
pub use sposv_ as SPOSV;
pub use sposvx_ as SPOSVX;
pub use sposvxx_ as SPOSVXX;
pub use spotf2_ as SPOTF2;
pub use spotrf2_ as SPOTRF2;
pub use spotrf_ as SPOTRF;
pub use spotri_ as SPOTRI;
pub use spotrs_ as SPOTRS;
pub use sppcon_ as SPPCON;
pub use sppequ_ as SPPEQU;
pub use spprfs_ as SPPRFS;
pub use sppsv_ as SPPSV;
pub use sppsvx_ as SPPSVX;
pub use spptrf_ as SPPTRF;
pub use spptri_ as SPPTRI;
pub use spptrs_ as SPPTRS;
pub use spstf2_ as SPSTF2;
pub use spstrf_ as SPSTRF;
pub use sptcon_ as SPTCON;
pub use spteqr_ as SPTEQR;
pub use sptrfs_ as SPTRFS;
pub use sptsv_ as SPTSV;
pub use sptsvx_ as SPTSVX;
pub use spttrf_ as SPTTRF;
pub use spttrs_ as SPTTRS;
pub use sptts2_ as SPTTS2;
pub use srscl_ as SRSCL;
pub use ssb2st_kernels_ as SSB2ST_KERNELS;
pub use ssbev_ as SSBEV;
pub use ssbev_2stage_ as SSBEV_2STAGE;
pub use ssbevd_ as SSBEVD;
pub use ssbevd_2stage_ as SSBEVD_2STAGE;
pub use ssbevx_ as SSBEVX;
pub use ssbevx_2stage_ as SSBEVX_2STAGE;
pub use ssbgst_ as SSBGST;
pub use ssbgv_ as SSBGV;
pub use ssbgvd_ as SSBGVD;
pub use ssbgvx_ as SSBGVX;
pub use ssbtrd_ as SSBTRD;
pub use ssfrk_ as SSFRK;
pub use sspcon_ as SSPCON;
pub use sspev_ as SSPEV;
pub use sspevd_ as SSPEVD;
pub use sspevx_ as SSPEVX;
pub use sspgst_ as SSPGST;
pub use sspgv_ as SSPGV;
pub use sspgvd_ as SSPGVD;
pub use sspgvx_ as SSPGVX;
pub use ssprfs_ as SSPRFS;
pub use sspsv_ as SSPSV;
pub use sspsvx_ as SSPSVX;
pub use ssptrd_ as SSPTRD;
pub use ssptrf_ as SSPTRF;
pub use ssptri_ as SSPTRI;
pub use ssptrs_ as SSPTRS;
pub use sstebz_ as SSTEBZ;
pub use sstedc_ as SSTEDC;
pub use sstegr_ as SSTEGR;
pub use sstein_ as SSTEIN;
pub use sstemr_ as SSTEMR;
pub use ssteqr_ as SSTEQR;
pub use ssterf_ as SSTERF;
pub use sstev_ as SSTEV;
pub use sstevd_ as SSTEVD;
pub use sstevr_ as SSTEVR;
pub use sstevx_ as SSTEVX;
pub use ssycon_ as SSYCON;
pub use ssycon_3_ as SSYCON_3;
pub use ssycon_rook_ as SSYCON_ROOK;
pub use ssyconv_ as SSYCONV;
pub use ssyconvf_ as SSYCONVF;
pub use ssyconvf_rook_ as SSYCONVF_ROOK;
pub use ssyequb_ as SSYEQUB;
pub use ssyev_ as SSYEV;
pub use ssyev_2stage_ as SSYEV_2STAGE;
pub use ssyevd_ as SSYEVD;
pub use ssyevd_2stage_ as SSYEVD_2STAGE;
pub use ssyevr_ as SSYEVR;
pub use ssyevr_2stage_ as SSYEVR_2STAGE;
pub use ssyevx_ as SSYEVX;
pub use ssyevx_2stage_ as SSYEVX_2STAGE;
pub use ssygs2_ as SSYGS2;
pub use ssygst_ as SSYGST;
pub use ssygv_ as SSYGV;
pub use ssygv_2stage_ as SSYGV_2STAGE;
pub use ssygvd_ as SSYGVD;
pub use ssygvx_ as SSYGVX;
pub use ssyrdb_ as SSYRDB;
pub use ssyrfs_ as SSYRFS;
pub use ssyrfsx_ as SSYRFSX;
pub use ssysv_ as SSYSV;
pub use ssysv_aa_ as SSYSV_AA;
pub use ssysv_aa_2stage_ as SSYSV_AA_2STAGE;
pub use ssysv_rk_ as SSYSV_RK;
pub use ssysv_rook_ as SSYSV_ROOK;
pub use ssysvx_ as SSYSVX;
pub use ssysvxx_ as SSYSVXX;
pub use ssyswapr_ as SSYSWAPR;
pub use ssytd2_ as SSYTD2;
pub use ssytf2_ as SSYTF2;
pub use ssytf2_rk_ as SSYTF2_RK;
pub use ssytf2_rook_ as SSYTF2_ROOK;
pub use ssytrd_ as SSYTRD;
pub use ssytrd_2stage_ as SSYTRD_2STAGE;
pub use ssytrd_sb2st_ as SSYTRD_SB2ST;
pub use ssytrd_sy2sb_ as SSYTRD_SY2SB;
pub use ssytrf_ as SSYTRF;
pub use ssytrf_aa_ as SSYTRF_AA;
pub use ssytrf_aa_2stage_ as SSYTRF_AA_2STAGE;
pub use ssytrf_rk_ as SSYTRF_RK;
pub use ssytrf_rook_ as SSYTRF_ROOK;
pub use ssytri2_ as SSYTRI2;
pub use ssytri2x_ as SSYTRI2X;
pub use ssytri_ as SSYTRI;
pub use ssytri_3_ as SSYTRI_3;
pub use ssytri_3x_ as SSYTRI_3X;
pub use ssytri_rook_ as SSYTRI_ROOK;
pub use ssytrs2_ as SSYTRS2;
pub use ssytrs_ as SSYTRS;
pub use ssytrs_3_ as SSYTRS_3;
pub use ssytrs_aa_ as SSYTRS_AA;
pub use ssytrs_aa_2stage_ as SSYTRS_AA_2STAGE;
pub use ssytrs_rook_ as SSYTRS_ROOK;
pub use stbcon_ as STBCON;
pub use stbrfs_ as STBRFS;
pub use stbtrs_ as STBTRS;
pub use stfsm_ as STFSM;
pub use stftri_ as STFTRI;
pub use stfttp_ as STFTTP;
pub use stfttr_ as STFTTR;
pub use stgevc_ as STGEVC;
pub use stgex2_ as STGEX2;
pub use stgexc_ as STGEXC;
pub use stgsen_ as STGSEN;
pub use stgsja_ as STGSJA;
pub use stgsna_ as STGSNA;
pub use stgsy2_ as STGSY2;
pub use stgsyl_ as STGSYL;
pub use stpcon_ as STPCON;
pub use stplqt2_ as STPLQT2;
pub use stplqt_ as STPLQT;
pub use stpmlqt_ as STPMLQT;
pub use stpmqrt_ as STPMQRT;
pub use stpqrt2_ as STPQRT2;
pub use stpqrt_ as STPQRT;
pub use stprfb_ as STPRFB;
pub use stprfs_ as STPRFS;
pub use stptri_ as STPTRI;
pub use stptrs_ as STPTRS;
pub use stpttf_ as STPTTF;
pub use stpttr_ as STPTTR;
pub use strcon_ as STRCON;
pub use strevc3_ as STREVC3;
pub use strevc_ as STREVC;
pub use strexc_ as STREXC;
pub use strrfs_ as STRRFS;
pub use strsen_ as STRSEN;
pub use strsna_ as STRSNA;
pub use strsyl3_ as STRSYL3;
pub use strsyl_ as STRSYL;
pub use strti2_ as STRTI2;
pub use strtri_ as STRTRI;
pub use strtrs_ as STRTRS;
pub use strttf_ as STRTTF;
pub use strttp_ as STRTTP;
pub use stzrqf_ as STZRQF;
pub use stzrzf_ as STZRZF;
pub use xerbla_array_ as XERBLA_ARRAY;
pub use zbbcsd_ as ZBBCSD;
pub use zbdsqr_ as ZBDSQR;
pub use zcgesv_ as ZCGESV;
pub use zcposv_ as ZCPOSV;
pub use zdrscl_ as ZDRSCL;
pub use zdtsvb_ as ZDTSVB;
pub use zdttrfb_ as ZDTTRFB;
pub use zdttrsb_ as ZDTTRSB;
pub use zgbbrd_ as ZGBBRD;
pub use zgbcon_ as ZGBCON;
pub use zgbequ_ as ZGBEQU;
pub use zgbequb_ as ZGBEQUB;
pub use zgbrfs_ as ZGBRFS;
pub use zgbrfsx_ as ZGBRFSX;
pub use zgbsv_ as ZGBSV;
pub use zgbsvx_ as ZGBSVX;
pub use zgbsvxx_ as ZGBSVXX;
pub use zgbtf2_ as ZGBTF2;
pub use zgbtrf_ as ZGBTRF;
pub use zgbtrs_ as ZGBTRS;
pub use zgebak_ as ZGEBAK;
pub use zgebal_ as ZGEBAL;
pub use zgebd2_ as ZGEBD2;
pub use zgebrd_ as ZGEBRD;
pub use zgecon_ as ZGECON;
pub use zgedmd_ as ZGEDMD;
pub use zgedmdq_ as ZGEDMDQ;
pub use zgeequ_ as ZGEEQU;
pub use zgeequb_ as ZGEEQUB;
pub use zgees_ as ZGEES;
pub use zgeesx_ as ZGEESX;
pub use zgeev_ as ZGEEV;
pub use zgeevx_ as ZGEEVX;
pub use zgegs_ as ZGEGS;
pub use zgegv_ as ZGEGV;
pub use zgehd2_ as ZGEHD2;
pub use zgehrd_ as ZGEHRD;
pub use zgejsv_ as ZGEJSV;
pub use zgelq2_ as ZGELQ2;
pub use zgelq_ as ZGELQ;
pub use zgelqf_ as ZGELQF;
pub use zgelqt3_ as ZGELQT3;
pub use zgelqt_ as ZGELQT;
pub use zgels_ as ZGELS;
pub use zgels_batch_strided_ as ZGELS_BATCH_STRIDED;
pub use zgelsd_ as ZGELSD;
pub use zgelss_ as ZGELSS;
pub use zgelst_ as ZGELST;
pub use zgelsx_ as ZGELSX;
pub use zgelsy_ as ZGELSY;
pub use zgemlq_ as ZGEMLQ;
pub use zgemlqt_ as ZGEMLQT;
pub use zgemqr_ as ZGEMQR;
pub use zgemqrt_ as ZGEMQRT;
pub use zgeql2_ as ZGEQL2;
pub use zgeqlf_ as ZGEQLF;
pub use zgeqp3_ as ZGEQP3;
pub use zgeqp3rk_ as ZGEQP3RK;
pub use zgeqpf_ as ZGEQPF;
pub use zgeqr2_ as ZGEQR2;
pub use zgeqr2p_ as ZGEQR2P;
pub use zgeqr_ as ZGEQR;
pub use zgeqrf_ as ZGEQRF;
pub use zgeqrfp_ as ZGEQRFP;
pub use zgeqrt2_ as ZGEQRT2;
pub use zgeqrt3_ as ZGEQRT3;
pub use zgeqrt_ as ZGEQRT;
pub use zgerfs_ as ZGERFS;
pub use zgerfsx_ as ZGERFSX;
pub use zgerq2_ as ZGERQ2;
pub use zgerqf_ as ZGERQF;
pub use zgesc2_ as ZGESC2;
pub use zgesdd_ as ZGESDD;
pub use zgesv_ as ZGESV;
pub use zgesvd_ as ZGESVD;
pub use zgesvda_batch_strided_ as ZGESVDA_BATCH_STRIDED;
pub use zgesvdq_ as ZGESVDQ;
pub use zgesvdx_ as ZGESVDX;
pub use zgesvj_ as ZGESVJ;
pub use zgesvx_ as ZGESVX;
pub use zgesvxx_ as ZGESVXX;
pub use zgetc2_ as ZGETC2;
pub use zgetf2_ as ZGETF2;
pub use zgetrf2_ as ZGETRF2;
pub use zgetrf_ as ZGETRF;
pub use zgetrf_batch_ as ZGETRF_BATCH;
pub use zgetrf_batch_strided_ as ZGETRF_BATCH_STRIDED;
pub use zgetrfnp_batch_ as ZGETRFNP_BATCH;
pub use zgetrfnp_batch_strided_ as ZGETRFNP_BATCH_STRIDED;
pub use zgetri_ as ZGETRI;
pub use zgetri_batch_strided_ as ZGETRI_BATCH_STRIDED;
pub use zgetri_oop_batch_ as ZGETRI_OOP_BATCH;
pub use zgetri_oop_batch_strided_ as ZGETRI_OOP_BATCH_STRIDED;
pub use zgetrs_ as ZGETRS;
pub use zgetrs_batch_strided_ as ZGETRS_BATCH_STRIDED;
pub use zgetrsnp_batch_strided_ as ZGETRSNP_BATCH_STRIDED;
pub use zgetsls_ as ZGETSLS;
pub use zgetsqrhrt_ as ZGETSQRHRT;
pub use zggbak_ as ZGGBAK;
pub use zggbal_ as ZGGBAL;
pub use zgges3_ as ZGGES3;
pub use zgges_ as ZGGES;
pub use zggesx_ as ZGGESX;
pub use zggev3_ as ZGGEV3;
pub use zggev_ as ZGGEV;
pub use zggevx_ as ZGGEVX;
pub use zggglm_ as ZGGGLM;
pub use zgghd3_ as ZGGHD3;
pub use zgghrd_ as ZGGHRD;
pub use zgglse_ as ZGGLSE;
pub use zggqrf_ as ZGGQRF;
pub use zggrqf_ as ZGGRQF;
pub use zggsvd3_ as ZGGSVD3;
pub use zggsvd_ as ZGGSVD;
pub use zggsvp3_ as ZGGSVP3;
pub use zggsvp_ as ZGGSVP;
pub use zgsvj0_ as ZGSVJ0;
pub use zgsvj1_ as ZGSVJ1;
pub use zgtcon_ as ZGTCON;
pub use zgtrfs_ as ZGTRFS;
pub use zgtsv_ as ZGTSV;
pub use zgtsvx_ as ZGTSVX;
pub use zgttrf_ as ZGTTRF;
pub use zgttrs_ as ZGTTRS;
pub use zgtts2_ as ZGTTS2;
pub use zhb2st_kernels_ as ZHB2ST_KERNELS;
pub use zhbev_ as ZHBEV;
pub use zhbev_2stage_ as ZHBEV_2STAGE;
pub use zhbevd_ as ZHBEVD;
pub use zhbevd_2stage_ as ZHBEVD_2STAGE;
pub use zhbevx_ as ZHBEVX;
pub use zhbevx_2stage_ as ZHBEVX_2STAGE;
pub use zhbgst_ as ZHBGST;
pub use zhbgv_ as ZHBGV;
pub use zhbgvd_ as ZHBGVD;
pub use zhbgvx_ as ZHBGVX;
pub use zhbtrd_ as ZHBTRD;
pub use zhecon_ as ZHECON;
pub use zhecon_3_ as ZHECON_3;
pub use zhecon_rook_ as ZHECON_ROOK;
pub use zheequb_ as ZHEEQUB;
pub use zheev_ as ZHEEV;
pub use zheev_2stage_ as ZHEEV_2STAGE;
pub use zheevd_ as ZHEEVD;
pub use zheevd_2stage_ as ZHEEVD_2STAGE;
pub use zheevr_ as ZHEEVR;
pub use zheevr_2stage_ as ZHEEVR_2STAGE;
pub use zheevx_ as ZHEEVX;
pub use zheevx_2stage_ as ZHEEVX_2STAGE;
pub use zhegs2_ as ZHEGS2;
pub use zhegst_ as ZHEGST;
pub use zhegv_ as ZHEGV;
pub use zhegv_2stage_ as ZHEGV_2STAGE;
pub use zhegvd_ as ZHEGVD;
pub use zhegvx_ as ZHEGVX;
pub use zherdb_ as ZHERDB;
pub use zherfs_ as ZHERFS;
pub use zherfsx_ as ZHERFSX;
pub use zhesv_ as ZHESV;
pub use zhesv_aa_ as ZHESV_AA;
pub use zhesv_aa_2stage_ as ZHESV_AA_2STAGE;
pub use zhesv_rk_ as ZHESV_RK;
pub use zhesv_rook_ as ZHESV_ROOK;
pub use zhesvx_ as ZHESVX;
pub use zhesvxx_ as ZHESVXX;
pub use zheswapr_ as ZHESWAPR;
pub use zhetd2_ as ZHETD2;
pub use zhetf2_ as ZHETF2;
pub use zhetf2_rk_ as ZHETF2_RK;
pub use zhetf2_rook_ as ZHETF2_ROOK;
pub use zhetrd_ as ZHETRD;
pub use zhetrd_2stage_ as ZHETRD_2STAGE;
pub use zhetrd_hb2st_ as ZHETRD_HB2ST;
pub use zhetrd_he2hb_ as ZHETRD_HE2HB;
pub use zhetrf_ as ZHETRF;
pub use zhetrf_aa_ as ZHETRF_AA;
pub use zhetrf_aa_2stage_ as ZHETRF_AA_2STAGE;
pub use zhetrf_rk_ as ZHETRF_RK;
pub use zhetrf_rook_ as ZHETRF_ROOK;
pub use zhetri2_ as ZHETRI2;
pub use zhetri2x_ as ZHETRI2X;
pub use zhetri_ as ZHETRI;
pub use zhetri_3_ as ZHETRI_3;
pub use zhetri_3x_ as ZHETRI_3X;
pub use zhetri_rook_ as ZHETRI_ROOK;
pub use zhetrs2_ as ZHETRS2;
pub use zhetrs_ as ZHETRS;
pub use zhetrs_3_ as ZHETRS_3;
pub use zhetrs_aa_ as ZHETRS_AA;
pub use zhetrs_aa_2stage_ as ZHETRS_AA_2STAGE;
pub use zhetrs_rook_ as ZHETRS_ROOK;
pub use zhfrk_ as ZHFRK;
pub use zhgeqz_ as ZHGEQZ;
pub use zhpcon_ as ZHPCON;
pub use zhpev_ as ZHPEV;
pub use zhpevd_ as ZHPEVD;
pub use zhpevx_ as ZHPEVX;
pub use zhpgst_ as ZHPGST;
pub use zhpgv_ as ZHPGV;
pub use zhpgvd_ as ZHPGVD;
pub use zhpgvx_ as ZHPGVX;
pub use zhprfs_ as ZHPRFS;
pub use zhpsv_ as ZHPSV;
pub use zhpsvx_ as ZHPSVX;
pub use zhptrd_ as ZHPTRD;
pub use zhptrf_ as ZHPTRF;
pub use zhptri_ as ZHPTRI;
pub use zhptrs_ as ZHPTRS;
pub use zhsein_ as ZHSEIN;
pub use zhseqr_ as ZHSEQR;
pub use zla_gbamv_ as ZLA_GBAMV;
pub use zla_gbrcond_c_ as ZLA_GBRCOND_C;
pub use zla_gbrcond_x_ as ZLA_GBRCOND_X;
pub use zla_gbrfsx_extended_ as ZLA_GBRFSX_EXTENDED;
pub use zla_gbrpvgrw_ as ZLA_GBRPVGRW;
pub use zla_geamv_ as ZLA_GEAMV;
pub use zla_gercond_c_ as ZLA_GERCOND_C;
pub use zla_gercond_x_ as ZLA_GERCOND_X;
pub use zla_gerfsx_extended_ as ZLA_GERFSX_EXTENDED;
pub use zla_gerpvgrw_ as ZLA_GERPVGRW;
pub use zla_heamv_ as ZLA_HEAMV;
pub use zla_hercond_c_ as ZLA_HERCOND_C;
pub use zla_hercond_x_ as ZLA_HERCOND_X;
pub use zla_herfsx_extended_ as ZLA_HERFSX_EXTENDED;
pub use zla_herpvgrw_ as ZLA_HERPVGRW;
pub use zla_lin_berr_ as ZLA_LIN_BERR;
pub use zla_porcond_c_ as ZLA_PORCOND_C;
pub use zla_porcond_x_ as ZLA_PORCOND_X;
pub use zla_porfsx_extended_ as ZLA_PORFSX_EXTENDED;
pub use zla_porpvgrw_ as ZLA_PORPVGRW;
pub use zla_syamv_ as ZLA_SYAMV;
pub use zla_syrcond_c_ as ZLA_SYRCOND_C;
pub use zla_syrcond_x_ as ZLA_SYRCOND_X;
pub use zla_syrfsx_extended_ as ZLA_SYRFSX_EXTENDED;
pub use zla_syrpvgrw_ as ZLA_SYRPVGRW;
pub use zla_wwaddw_ as ZLA_WWADDW;
pub use zlabrd_ as ZLABRD;
pub use zlacgv_ as ZLACGV;
pub use zlacn2_ as ZLACN2;
pub use zlacon_ as ZLACON;
pub use zlacp2_ as ZLACP2;
pub use zlacpy_ as ZLACPY;
pub use zlacrm_ as ZLACRM;
pub use zlacrt_ as ZLACRT;
pub use zladiv_ as ZLADIV;
pub use zlaed0_ as ZLAED0;
pub use zlaed7_ as ZLAED7;
pub use zlaed8_ as ZLAED8;
pub use zlaein_ as ZLAEIN;
pub use zlaesy_ as ZLAESY;
pub use zlaev2_ as ZLAEV2;
pub use zlag2c_ as ZLAG2C;
pub use zlagge_ as ZLAGGE;
pub use zlaghe_ as ZLAGHE;
pub use zlags2_ as ZLAGS2;
pub use zlagsy_ as ZLAGSY;
pub use zlagtm_ as ZLAGTM;
pub use zlahef_ as ZLAHEF;
pub use zlahef_aa_ as ZLAHEF_AA;
pub use zlahef_rk_ as ZLAHEF_RK;
pub use zlahef_rook_ as ZLAHEF_ROOK;
pub use zlahqr_ as ZLAHQR;
pub use zlahr2_ as ZLAHR2;
pub use zlahrd_ as ZLAHRD;
pub use zlaic1_ as ZLAIC1;
pub use zlakf2_ as ZLAKF2;
pub use zlals0_ as ZLALS0;
pub use zlalsa_ as ZLALSA;
pub use zlalsd_ as ZLALSD;
pub use zlamswlq_ as ZLAMSWLQ;
pub use zlamtsqr_ as ZLAMTSQR;
pub use zlangb_ as ZLANGB;
pub use zlange_ as ZLANGE;
pub use zlangt_ as ZLANGT;
pub use zlanhb_ as ZLANHB;
pub use zlanhe_ as ZLANHE;
pub use zlanhf_ as ZLANHF;
pub use zlanhp_ as ZLANHP;
pub use zlanhs_ as ZLANHS;
pub use zlanht_ as ZLANHT;
pub use zlansb_ as ZLANSB;
pub use zlansp_ as ZLANSP;
pub use zlansy_ as ZLANSY;
pub use zlantb_ as ZLANTB;
pub use zlantp_ as ZLANTP;
pub use zlantr_ as ZLANTR;
pub use zlapll_ as ZLAPLL;
pub use zlapmr_ as ZLAPMR;
pub use zlapmt_ as ZLAPMT;
pub use zlaqgb_ as ZLAQGB;
pub use zlaqge_ as ZLAQGE;
pub use zlaqhb_ as ZLAQHB;
pub use zlaqhe_ as ZLAQHE;
pub use zlaqhp_ as ZLAQHP;
pub use zlaqp2_ as ZLAQP2;
pub use zlaqps_ as ZLAQPS;
pub use zlaqr0_ as ZLAQR0;
pub use zlaqr1_ as ZLAQR1;
pub use zlaqr2_ as ZLAQR2;
pub use zlaqr3_ as ZLAQR3;
pub use zlaqr4_ as ZLAQR4;
pub use zlaqr5_ as ZLAQR5;
pub use zlaqsb_ as ZLAQSB;
pub use zlaqsp_ as ZLAQSP;
pub use zlaqsy_ as ZLAQSY;
pub use zlaqz0_ as ZLAQZ0;
pub use zlaqz1_ as ZLAQZ1;
pub use zlaqz2_ as ZLAQZ2;
pub use zlaqz3_ as ZLAQZ3;
pub use zlar1v_ as ZLAR1V;
pub use zlar2v_ as ZLAR2V;
pub use zlarcm_ as ZLARCM;
pub use zlarf1f_ as ZLARF1F;
pub use zlarf1l_ as ZLARF1L;
pub use zlarf_ as ZLARF;
pub use zlarfb_ as ZLARFB;
pub use zlarfb_gett_ as ZLARFB_GETT;
pub use zlarfg_ as ZLARFG;
pub use zlarfgp_ as ZLARFGP;
pub use zlarfp_ as ZLARFP;
pub use zlarft_ as ZLARFT;
pub use zlarfx_ as ZLARFX;
pub use zlarfy_ as ZLARFY;
pub use zlarge_ as ZLARGE;
pub use zlargv_ as ZLARGV;
pub use zlarnd_ as ZLARND;
pub use zlarnv_ as ZLARNV;
pub use zlaror_ as ZLAROR;
pub use zlarot_ as ZLAROT;
pub use zlarrv_ as ZLARRV;
pub use zlarscl2_ as ZLARSCL2;
pub use zlartg_ as ZLARTG;
pub use zlartv_ as ZLARTV;
pub use zlarz_ as ZLARZ;
pub use zlarzb_ as ZLARZB;
pub use zlarzt_ as ZLARZT;
pub use zlascl2_ as ZLASCL2;
pub use zlascl_ as ZLASCL;
pub use zlaset_ as ZLASET;
pub use zlasr_ as ZLASR;
pub use zlassq_ as ZLASSQ;
pub use zlaswlq_ as ZLASWLQ;
pub use zlaswp_ as ZLASWP;
pub use zlasyf_ as ZLASYF;
pub use zlasyf_aa_ as ZLASYF_AA;
pub use zlasyf_rk_ as ZLASYF_RK;
pub use zlasyf_rook_ as ZLASYF_ROOK;
pub use zlat2c_ as ZLAT2C;
pub use zlatbs_ as ZLATBS;
pub use zlatdf_ as ZLATDF;
pub use zlatm1_ as ZLATM1;
pub use zlatm2_ as ZLATM2;
pub use zlatm3_ as ZLATM3;
pub use zlatm5_ as ZLATM5;
pub use zlatm6_ as ZLATM6;
pub use zlatme_ as ZLATME;
pub use zlatmr_ as ZLATMR;
pub use zlatms_ as ZLATMS;
pub use zlatps_ as ZLATPS;
pub use zlatrd_ as ZLATRD;
pub use zlatrs3_ as ZLATRS3;
pub use zlatrs_ as ZLATRS;
pub use zlatrz_ as ZLATRZ;
pub use zlatsqr_ as ZLATSQR;
pub use zlatzm_ as ZLATZM;
pub use zlaunhr_col_getrfnp2_ as ZLAUNHR_COL_GETRFNP2;
pub use zlaunhr_col_getrfnp_ as ZLAUNHR_COL_GETRFNP;
pub use zlauu2_ as ZLAUU2;
pub use zlauum_ as ZLAUUM;
pub use zpbcon_ as ZPBCON;
pub use zpbequ_ as ZPBEQU;
pub use zpbrfs_ as ZPBRFS;
pub use zpbstf_ as ZPBSTF;
pub use zpbsv_ as ZPBSV;
pub use zpbsvx_ as ZPBSVX;
pub use zpbtf2_ as ZPBTF2;
pub use zpbtrf_ as ZPBTRF;
pub use zpbtrs_ as ZPBTRS;
pub use zpftrf_ as ZPFTRF;
pub use zpftri_ as ZPFTRI;
pub use zpftrs_ as ZPFTRS;
pub use zpocon_ as ZPOCON;
pub use zpoequ_ as ZPOEQU;
pub use zpoequb_ as ZPOEQUB;
pub use zporfs_ as ZPORFS;
pub use zporfsx_ as ZPORFSX;
pub use zposv_ as ZPOSV;
pub use zposvx_ as ZPOSVX;
pub use zposvxx_ as ZPOSVXX;
pub use zpotf2_ as ZPOTF2;
pub use zpotrf2_ as ZPOTRF2;
pub use zpotrf_ as ZPOTRF;
pub use zpotri_ as ZPOTRI;
pub use zpotrs_ as ZPOTRS;
pub use zppcon_ as ZPPCON;
pub use zppequ_ as ZPPEQU;
pub use zpprfs_ as ZPPRFS;
pub use zppsv_ as ZPPSV;
pub use zppsvx_ as ZPPSVX;
pub use zpptrf_ as ZPPTRF;
pub use zpptri_ as ZPPTRI;
pub use zpptrs_ as ZPPTRS;
pub use zpstf2_ as ZPSTF2;
pub use zpstrf_ as ZPSTRF;
pub use zptcon_ as ZPTCON;
pub use zpteqr_ as ZPTEQR;
pub use zptrfs_ as ZPTRFS;
pub use zptsv_ as ZPTSV;
pub use zptsvx_ as ZPTSVX;
pub use zpttrf_ as ZPTTRF;
pub use zpttrs_ as ZPTTRS;
pub use zptts2_ as ZPTTS2;
pub use zrot_ as ZROT;
pub use zspcon_ as ZSPCON;
pub use zspmv_ as ZSPMV;
pub use zspr_ as ZSPR;
pub use zsprfs_ as ZSPRFS;
pub use zspsv_ as ZSPSV;
pub use zspsvx_ as ZSPSVX;
pub use zsptrf_ as ZSPTRF;
pub use zsptri_ as ZSPTRI;
pub use zsptrs_ as ZSPTRS;
pub use zstedc_ as ZSTEDC;
pub use zstegr_ as ZSTEGR;
pub use zstein_ as ZSTEIN;
pub use zstemr_ as ZSTEMR;
pub use zsteqr_ as ZSTEQR;
pub use zsycon_ as ZSYCON;
pub use zsycon_3_ as ZSYCON_3;
pub use zsycon_rook_ as ZSYCON_ROOK;
pub use zsyconv_ as ZSYCONV;
pub use zsyconvf_ as ZSYCONVF;
pub use zsyconvf_rook_ as ZSYCONVF_ROOK;
pub use zsyequb_ as ZSYEQUB;
pub use zsymv_ as ZSYMV;
pub use zsyr_ as ZSYR;
pub use zsyrfs_ as ZSYRFS;
pub use zsyrfsx_ as ZSYRFSX;
pub use zsysv_ as ZSYSV;
pub use zsysv_aa_ as ZSYSV_AA;
pub use zsysv_aa_2stage_ as ZSYSV_AA_2STAGE;
pub use zsysv_rk_ as ZSYSV_RK;
pub use zsysv_rook_ as ZSYSV_ROOK;
pub use zsysvx_ as ZSYSVX;
pub use zsysvxx_ as ZSYSVXX;
pub use zsyswapr_ as ZSYSWAPR;
pub use zsytf2_ as ZSYTF2;
pub use zsytf2_rk_ as ZSYTF2_RK;
pub use zsytf2_rook_ as ZSYTF2_ROOK;
pub use zsytrf_ as ZSYTRF;
pub use zsytrf_aa_ as ZSYTRF_AA;
pub use zsytrf_aa_2stage_ as ZSYTRF_AA_2STAGE;
pub use zsytrf_rk_ as ZSYTRF_RK;
pub use zsytrf_rook_ as ZSYTRF_ROOK;
pub use zsytri2_ as ZSYTRI2;
pub use zsytri2x_ as ZSYTRI2X;
pub use zsytri_ as ZSYTRI;
pub use zsytri_3_ as ZSYTRI_3;
pub use zsytri_3x_ as ZSYTRI_3X;
pub use zsytri_rook_ as ZSYTRI_ROOK;
pub use zsytrs2_ as ZSYTRS2;
pub use zsytrs_ as ZSYTRS;
pub use zsytrs_3_ as ZSYTRS_3;
pub use zsytrs_aa_ as ZSYTRS_AA;
pub use zsytrs_aa_2stage_ as ZSYTRS_AA_2STAGE;
pub use zsytrs_rook_ as ZSYTRS_ROOK;
pub use ztbcon_ as ZTBCON;
pub use ztbrfs_ as ZTBRFS;
pub use ztbtrs_ as ZTBTRS;
pub use ztfsm_ as ZTFSM;
pub use ztftri_ as ZTFTRI;
pub use ztfttp_ as ZTFTTP;
pub use ztfttr_ as ZTFTTR;
pub use ztgevc_ as ZTGEVC;
pub use ztgex2_ as ZTGEX2;
pub use ztgexc_ as ZTGEXC;
pub use ztgsen_ as ZTGSEN;
pub use ztgsja_ as ZTGSJA;
pub use ztgsna_ as ZTGSNA;
pub use ztgsy2_ as ZTGSY2;
pub use ztgsyl_ as ZTGSYL;
pub use ztpcon_ as ZTPCON;
pub use ztplqt2_ as ZTPLQT2;
pub use ztplqt_ as ZTPLQT;
pub use ztpmlqt_ as ZTPMLQT;
pub use ztpmqrt_ as ZTPMQRT;
pub use ztpqrt2_ as ZTPQRT2;
pub use ztpqrt_ as ZTPQRT;
pub use ztprfb_ as ZTPRFB;
pub use ztprfs_ as ZTPRFS;
pub use ztptri_ as ZTPTRI;
pub use ztptrs_ as ZTPTRS;
pub use ztpttf_ as ZTPTTF;
pub use ztpttr_ as ZTPTTR;
pub use ztrcon_ as ZTRCON;
pub use ztrevc3_ as ZTREVC3;
pub use ztrevc_ as ZTREVC;
pub use ztrexc_ as ZTREXC;
pub use ztrrfs_ as ZTRRFS;
pub use ztrsen_ as ZTRSEN;
pub use ztrsna_ as ZTRSNA;
pub use ztrsyl3_ as ZTRSYL3;
pub use ztrsyl_ as ZTRSYL;
pub use ztrti2_ as ZTRTI2;
pub use ztrtri_ as ZTRTRI;
pub use ztrtrs_ as ZTRTRS;
pub use ztrttf_ as ZTRTTF;
pub use ztrttp_ as ZTRTTP;
pub use ztzrqf_ as ZTZRQF;
pub use ztzrzf_ as ZTZRZF;
pub use zunbdb1_ as ZUNBDB1;
pub use zunbdb2_ as ZUNBDB2;
pub use zunbdb3_ as ZUNBDB3;
pub use zunbdb4_ as ZUNBDB4;
pub use zunbdb5_ as ZUNBDB5;
pub use zunbdb6_ as ZUNBDB6;
pub use zunbdb_ as ZUNBDB;
pub use zuncsd2by1_ as ZUNCSD2BY1;
pub use zuncsd_ as ZUNCSD;
pub use zung2l_ as ZUNG2L;
pub use zung2r_ as ZUNG2R;
pub use zungbr_ as ZUNGBR;
pub use zunghr_ as ZUNGHR;
pub use zungl2_ as ZUNGL2;
pub use zunglq_ as ZUNGLQ;
pub use zungql_ as ZUNGQL;
pub use zungqr_ as ZUNGQR;
pub use zungr2_ as ZUNGR2;
pub use zungrq_ as ZUNGRQ;
pub use zungtr_ as ZUNGTR;
pub use zungtsqr_ as ZUNGTSQR;
pub use zungtsqr_row_ as ZUNGTSQR_ROW;
pub use zunhr_col_ as ZUNHR_COL;
pub use zunm22_ as ZUNM22;
pub use zunm2l_ as ZUNM2L;
pub use zunm2r_ as ZUNM2R;
pub use zunmbr_ as ZUNMBR;
pub use zunmhr_ as ZUNMHR;
pub use zunml2_ as ZUNML2;
pub use zunmlq_ as ZUNMLQ;
pub use zunmql_ as ZUNMQL;
pub use zunmqr_ as ZUNMQR;
pub use zunmr2_ as ZUNMR2;
pub use zunmr3_ as ZUNMR3;
pub use zunmrq_ as ZUNMRQ;
pub use zunmrz_ as ZUNMRZ;
pub use zunmtr_ as ZUNMTR;
pub use zupgtr_ as ZUPGTR;
pub use zupmtr_ as ZUPMTR;

/* #endregion */

/* #region lower case with underscore alias */

pub use cbbcsd_ as cbbcsd;
pub use cbdsqr_ as cbdsqr;
pub use cdtsvb_ as cdtsvb;
pub use cdttrfb_ as cdttrfb;
pub use cdttrsb_ as cdttrsb;
pub use cgbbrd_ as cgbbrd;
pub use cgbcon_ as cgbcon;
pub use cgbequ_ as cgbequ;
pub use cgbequb_ as cgbequb;
pub use cgbrfs_ as cgbrfs;
pub use cgbrfsx_ as cgbrfsx;
pub use cgbsv_ as cgbsv;
pub use cgbsvx_ as cgbsvx;
pub use cgbsvxx_ as cgbsvxx;
pub use cgbtf2_ as cgbtf2;
pub use cgbtrf_ as cgbtrf;
pub use cgbtrs_ as cgbtrs;
pub use cgebak_ as cgebak;
pub use cgebal_ as cgebal;
pub use cgebd2_ as cgebd2;
pub use cgebrd_ as cgebrd;
pub use cgecon_ as cgecon;
pub use cgedmd_ as cgedmd;
pub use cgedmdq_ as cgedmdq;
pub use cgeequ_ as cgeequ;
pub use cgeequb_ as cgeequb;
pub use cgees_ as cgees;
pub use cgeesx_ as cgeesx;
pub use cgeev_ as cgeev;
pub use cgeevx_ as cgeevx;
pub use cgegs_ as cgegs;
pub use cgegv_ as cgegv;
pub use cgehd2_ as cgehd2;
pub use cgehrd_ as cgehrd;
pub use cgejsv_ as cgejsv;
pub use cgelq2_ as cgelq2;
pub use cgelq_ as cgelq;
pub use cgelqf_ as cgelqf;
pub use cgelqt3_ as cgelqt3;
pub use cgelqt_ as cgelqt;
pub use cgels_ as cgels;
pub use cgels_batch_strided_ as cgels_batch_strided;
pub use cgelsd_ as cgelsd;
pub use cgelss_ as cgelss;
pub use cgelst_ as cgelst;
pub use cgelsx_ as cgelsx;
pub use cgelsy_ as cgelsy;
pub use cgemlq_ as cgemlq;
pub use cgemlqt_ as cgemlqt;
pub use cgemqr_ as cgemqr;
pub use cgemqrt_ as cgemqrt;
pub use cgeql2_ as cgeql2;
pub use cgeqlf_ as cgeqlf;
pub use cgeqp3_ as cgeqp3;
pub use cgeqp3rk_ as cgeqp3rk;
pub use cgeqpf_ as cgeqpf;
pub use cgeqr2_ as cgeqr2;
pub use cgeqr2p_ as cgeqr2p;
pub use cgeqr_ as cgeqr;
pub use cgeqrf_ as cgeqrf;
pub use cgeqrfp_ as cgeqrfp;
pub use cgeqrt2_ as cgeqrt2;
pub use cgeqrt3_ as cgeqrt3;
pub use cgeqrt_ as cgeqrt;
pub use cgerfs_ as cgerfs;
pub use cgerfsx_ as cgerfsx;
pub use cgerq2_ as cgerq2;
pub use cgerqf_ as cgerqf;
pub use cgesc2_ as cgesc2;
pub use cgesdd_ as cgesdd;
pub use cgesv_ as cgesv;
pub use cgesvd_ as cgesvd;
pub use cgesvda_batch_strided_ as cgesvda_batch_strided;
pub use cgesvdq_ as cgesvdq;
pub use cgesvdx_ as cgesvdx;
pub use cgesvj_ as cgesvj;
pub use cgesvx_ as cgesvx;
pub use cgesvxx_ as cgesvxx;
pub use cgetc2_ as cgetc2;
pub use cgetf2_ as cgetf2;
pub use cgetrf2_ as cgetrf2;
pub use cgetrf_ as cgetrf;
pub use cgetrf_batch_ as cgetrf_batch;
pub use cgetrf_batch_strided_ as cgetrf_batch_strided;
pub use cgetrfnp_batch_ as cgetrfnp_batch;
pub use cgetrfnp_batch_strided_ as cgetrfnp_batch_strided;
pub use cgetri_ as cgetri;
pub use cgetri_batch_strided_ as cgetri_batch_strided;
pub use cgetri_oop_batch_ as cgetri_oop_batch;
pub use cgetri_oop_batch_strided_ as cgetri_oop_batch_strided;
pub use cgetrs_ as cgetrs;
pub use cgetrs_batch_strided_ as cgetrs_batch_strided;
pub use cgetrsnp_batch_strided_ as cgetrsnp_batch_strided;
pub use cgetsls_ as cgetsls;
pub use cgetsqrhrt_ as cgetsqrhrt;
pub use cggbak_ as cggbak;
pub use cggbal_ as cggbal;
pub use cgges3_ as cgges3;
pub use cgges_ as cgges;
pub use cggesx_ as cggesx;
pub use cggev3_ as cggev3;
pub use cggev_ as cggev;
pub use cggevx_ as cggevx;
pub use cggglm_ as cggglm;
pub use cgghd3_ as cgghd3;
pub use cgghrd_ as cgghrd;
pub use cgglse_ as cgglse;
pub use cggqrf_ as cggqrf;
pub use cggrqf_ as cggrqf;
pub use cggsvd3_ as cggsvd3;
pub use cggsvd_ as cggsvd;
pub use cggsvp3_ as cggsvp3;
pub use cggsvp_ as cggsvp;
pub use cgsvj0_ as cgsvj0;
pub use cgsvj1_ as cgsvj1;
pub use cgtcon_ as cgtcon;
pub use cgtrfs_ as cgtrfs;
pub use cgtsv_ as cgtsv;
pub use cgtsvx_ as cgtsvx;
pub use cgttrf_ as cgttrf;
pub use cgttrs_ as cgttrs;
pub use cgtts2_ as cgtts2;
pub use chb2st_kernels_ as chb2st_kernels;
pub use chbev_ as chbev;
pub use chbev_2stage_ as chbev_2stage;
pub use chbevd_ as chbevd;
pub use chbevd_2stage_ as chbevd_2stage;
pub use chbevx_ as chbevx;
pub use chbevx_2stage_ as chbevx_2stage;
pub use chbgst_ as chbgst;
pub use chbgv_ as chbgv;
pub use chbgvd_ as chbgvd;
pub use chbgvx_ as chbgvx;
pub use chbtrd_ as chbtrd;
pub use checon_ as checon;
pub use checon_3_ as checon_3;
pub use checon_rook_ as checon_rook;
pub use cheequb_ as cheequb;
pub use cheev_ as cheev;
pub use cheev_2stage_ as cheev_2stage;
pub use cheevd_ as cheevd;
pub use cheevd_2stage_ as cheevd_2stage;
pub use cheevr_ as cheevr;
pub use cheevr_2stage_ as cheevr_2stage;
pub use cheevx_ as cheevx;
pub use cheevx_2stage_ as cheevx_2stage;
pub use chegs2_ as chegs2;
pub use chegst_ as chegst;
pub use chegv_ as chegv;
pub use chegv_2stage_ as chegv_2stage;
pub use chegvd_ as chegvd;
pub use chegvx_ as chegvx;
pub use cherdb_ as cherdb;
pub use cherfs_ as cherfs;
pub use cherfsx_ as cherfsx;
pub use chesv_ as chesv;
pub use chesv_aa_ as chesv_aa;
pub use chesv_aa_2stage_ as chesv_aa_2stage;
pub use chesv_rk_ as chesv_rk;
pub use chesv_rook_ as chesv_rook;
pub use chesvx_ as chesvx;
pub use chesvxx_ as chesvxx;
pub use cheswapr_ as cheswapr;
pub use chetd2_ as chetd2;
pub use chetf2_ as chetf2;
pub use chetf2_rk_ as chetf2_rk;
pub use chetf2_rook_ as chetf2_rook;
pub use chetrd_ as chetrd;
pub use chetrd_2stage_ as chetrd_2stage;
pub use chetrd_hb2st_ as chetrd_hb2st;
pub use chetrd_he2hb_ as chetrd_he2hb;
pub use chetrf_ as chetrf;
pub use chetrf_aa_ as chetrf_aa;
pub use chetrf_aa_2stage_ as chetrf_aa_2stage;
pub use chetrf_rk_ as chetrf_rk;
pub use chetrf_rook_ as chetrf_rook;
pub use chetri2_ as chetri2;
pub use chetri2x_ as chetri2x;
pub use chetri_ as chetri;
pub use chetri_3_ as chetri_3;
pub use chetri_3x_ as chetri_3x;
pub use chetri_rook_ as chetri_rook;
pub use chetrs2_ as chetrs2;
pub use chetrs_ as chetrs;
pub use chetrs_3_ as chetrs_3;
pub use chetrs_aa_ as chetrs_aa;
pub use chetrs_aa_2stage_ as chetrs_aa_2stage;
pub use chetrs_rook_ as chetrs_rook;
pub use chfrk_ as chfrk;
pub use chgeqz_ as chgeqz;
pub use chla_transtype_ as chla_transtype;
pub use chpcon_ as chpcon;
pub use chpev_ as chpev;
pub use chpevd_ as chpevd;
pub use chpevx_ as chpevx;
pub use chpgst_ as chpgst;
pub use chpgv_ as chpgv;
pub use chpgvd_ as chpgvd;
pub use chpgvx_ as chpgvx;
pub use chprfs_ as chprfs;
pub use chpsv_ as chpsv;
pub use chpsvx_ as chpsvx;
pub use chptrd_ as chptrd;
pub use chptrf_ as chptrf;
pub use chptri_ as chptri;
pub use chptrs_ as chptrs;
pub use chsein_ as chsein;
pub use chseqr_ as chseqr;
pub use cla_gbamv_ as cla_gbamv;
pub use cla_gbrcond_c_ as cla_gbrcond_c;
pub use cla_gbrcond_x_ as cla_gbrcond_x;
pub use cla_gbrfsx_extended_ as cla_gbrfsx_extended;
pub use cla_gbrpvgrw_ as cla_gbrpvgrw;
pub use cla_geamv_ as cla_geamv;
pub use cla_gercond_c_ as cla_gercond_c;
pub use cla_gercond_x_ as cla_gercond_x;
pub use cla_gerfsx_extended_ as cla_gerfsx_extended;
pub use cla_gerpvgrw_ as cla_gerpvgrw;
pub use cla_heamv_ as cla_heamv;
pub use cla_hercond_c_ as cla_hercond_c;
pub use cla_hercond_x_ as cla_hercond_x;
pub use cla_herfsx_extended_ as cla_herfsx_extended;
pub use cla_herpvgrw_ as cla_herpvgrw;
pub use cla_lin_berr_ as cla_lin_berr;
pub use cla_porcond_c_ as cla_porcond_c;
pub use cla_porcond_x_ as cla_porcond_x;
pub use cla_porfsx_extended_ as cla_porfsx_extended;
pub use cla_porpvgrw_ as cla_porpvgrw;
pub use cla_syamv_ as cla_syamv;
pub use cla_syrcond_c_ as cla_syrcond_c;
pub use cla_syrcond_x_ as cla_syrcond_x;
pub use cla_syrfsx_extended_ as cla_syrfsx_extended;
pub use cla_syrpvgrw_ as cla_syrpvgrw;
pub use cla_wwaddw_ as cla_wwaddw;
pub use clabrd_ as clabrd;
pub use clacgv_ as clacgv;
pub use clacn2_ as clacn2;
pub use clacon_ as clacon;
pub use clacp2_ as clacp2;
pub use clacpy_ as clacpy;
pub use clacrm_ as clacrm;
pub use clacrt_ as clacrt;
pub use cladiv_ as cladiv;
pub use claed0_ as claed0;
pub use claed7_ as claed7;
pub use claed8_ as claed8;
pub use claein_ as claein;
pub use claesy_ as claesy;
pub use claev2_ as claev2;
pub use clag2z_ as clag2z;
pub use clagge_ as clagge;
pub use claghe_ as claghe;
pub use clags2_ as clags2;
pub use clagsy_ as clagsy;
pub use clagtm_ as clagtm;
pub use clahef_ as clahef;
pub use clahef_aa_ as clahef_aa;
pub use clahef_rk_ as clahef_rk;
pub use clahef_rook_ as clahef_rook;
pub use clahqr_ as clahqr;
pub use clahr2_ as clahr2;
pub use clahrd_ as clahrd;
pub use claic1_ as claic1;
pub use clakf2_ as clakf2;
pub use clals0_ as clals0;
pub use clalsa_ as clalsa;
pub use clalsd_ as clalsd;
pub use clamswlq_ as clamswlq;
pub use clamtsqr_ as clamtsqr;
pub use clangb_ as clangb;
pub use clange_ as clange;
pub use clangt_ as clangt;
pub use clanhb_ as clanhb;
pub use clanhe_ as clanhe;
pub use clanhf_ as clanhf;
pub use clanhp_ as clanhp;
pub use clanhs_ as clanhs;
pub use clanht_ as clanht;
pub use clansb_ as clansb;
pub use clansp_ as clansp;
pub use clansy_ as clansy;
pub use clantb_ as clantb;
pub use clantp_ as clantp;
pub use clantr_ as clantr;
pub use clapll_ as clapll;
pub use clapmr_ as clapmr;
pub use clapmt_ as clapmt;
pub use claqgb_ as claqgb;
pub use claqge_ as claqge;
pub use claqhb_ as claqhb;
pub use claqhe_ as claqhe;
pub use claqhp_ as claqhp;
pub use claqp2_ as claqp2;
pub use claqps_ as claqps;
pub use claqr0_ as claqr0;
pub use claqr1_ as claqr1;
pub use claqr2_ as claqr2;
pub use claqr3_ as claqr3;
pub use claqr4_ as claqr4;
pub use claqr5_ as claqr5;
pub use claqsb_ as claqsb;
pub use claqsp_ as claqsp;
pub use claqsy_ as claqsy;
pub use claqz0_ as claqz0;
pub use claqz1_ as claqz1;
pub use claqz2_ as claqz2;
pub use claqz3_ as claqz3;
pub use clar1v_ as clar1v;
pub use clar2v_ as clar2v;
pub use clarcm_ as clarcm;
pub use clarf1f_ as clarf1f;
pub use clarf1l_ as clarf1l;
pub use clarf_ as clarf;
pub use clarfb_ as clarfb;
pub use clarfb_gett_ as clarfb_gett;
pub use clarfg_ as clarfg;
pub use clarfgp_ as clarfgp;
pub use clarfp_ as clarfp;
pub use clarft_ as clarft;
pub use clarfx_ as clarfx;
pub use clarfy_ as clarfy;
pub use clarge_ as clarge;
pub use clargv_ as clargv;
pub use clarnd_ as clarnd;
pub use clarnv_ as clarnv;
pub use claror_ as claror;
pub use clarot_ as clarot;
pub use clarrv_ as clarrv;
pub use clarscl2_ as clarscl2;
pub use clartg_ as clartg;
pub use clartv_ as clartv;
pub use clarz_ as clarz;
pub use clarzb_ as clarzb;
pub use clarzt_ as clarzt;
pub use clascl2_ as clascl2;
pub use clascl_ as clascl;
pub use claset_ as claset;
pub use clasr_ as clasr;
pub use classq_ as classq;
pub use claswlq_ as claswlq;
pub use claswp_ as claswp;
pub use clasyf_ as clasyf;
pub use clasyf_aa_ as clasyf_aa;
pub use clasyf_rk_ as clasyf_rk;
pub use clasyf_rook_ as clasyf_rook;
pub use clatbs_ as clatbs;
pub use clatdf_ as clatdf;
pub use clatm1_ as clatm1;
pub use clatm2_ as clatm2;
pub use clatm3_ as clatm3;
pub use clatm5_ as clatm5;
pub use clatm6_ as clatm6;
pub use clatme_ as clatme;
pub use clatmr_ as clatmr;
pub use clatms_ as clatms;
pub use clatps_ as clatps;
pub use clatrd_ as clatrd;
pub use clatrs3_ as clatrs3;
pub use clatrs_ as clatrs;
pub use clatrz_ as clatrz;
pub use clatsqr_ as clatsqr;
pub use clatzm_ as clatzm;
pub use claunhr_col_getrfnp2_ as claunhr_col_getrfnp2;
pub use claunhr_col_getrfnp_ as claunhr_col_getrfnp;
pub use clauu2_ as clauu2;
pub use clauum_ as clauum;
pub use cpbcon_ as cpbcon;
pub use cpbequ_ as cpbequ;
pub use cpbrfs_ as cpbrfs;
pub use cpbstf_ as cpbstf;
pub use cpbsv_ as cpbsv;
pub use cpbsvx_ as cpbsvx;
pub use cpbtf2_ as cpbtf2;
pub use cpbtrf_ as cpbtrf;
pub use cpbtrs_ as cpbtrs;
pub use cpftrf_ as cpftrf;
pub use cpftri_ as cpftri;
pub use cpftrs_ as cpftrs;
pub use cpocon_ as cpocon;
pub use cpoequ_ as cpoequ;
pub use cpoequb_ as cpoequb;
pub use cporfs_ as cporfs;
pub use cporfsx_ as cporfsx;
pub use cposv_ as cposv;
pub use cposvx_ as cposvx;
pub use cposvxx_ as cposvxx;
pub use cpotf2_ as cpotf2;
pub use cpotrf2_ as cpotrf2;
pub use cpotrf_ as cpotrf;
pub use cpotri_ as cpotri;
pub use cpotrs_ as cpotrs;
pub use cppcon_ as cppcon;
pub use cppequ_ as cppequ;
pub use cpprfs_ as cpprfs;
pub use cppsv_ as cppsv;
pub use cppsvx_ as cppsvx;
pub use cpptrf_ as cpptrf;
pub use cpptri_ as cpptri;
pub use cpptrs_ as cpptrs;
pub use cpstf2_ as cpstf2;
pub use cpstrf_ as cpstrf;
pub use cptcon_ as cptcon;
pub use cpteqr_ as cpteqr;
pub use cptrfs_ as cptrfs;
pub use cptsv_ as cptsv;
pub use cptsvx_ as cptsvx;
pub use cpttrf_ as cpttrf;
pub use cpttrs_ as cpttrs;
pub use cptts2_ as cptts2;
pub use crot_ as crot;
pub use cspcon_ as cspcon;
pub use cspmv_ as cspmv;
pub use cspr_ as cspr;
pub use csprfs_ as csprfs;
pub use cspsv_ as cspsv;
pub use cspsvx_ as cspsvx;
pub use csptrf_ as csptrf;
pub use csptri_ as csptri;
pub use csptrs_ as csptrs;
pub use csrscl_ as csrscl;
pub use cstedc_ as cstedc;
pub use cstegr_ as cstegr;
pub use cstein_ as cstein;
pub use cstemr_ as cstemr;
pub use csteqr_ as csteqr;
pub use csycon_ as csycon;
pub use csycon_3_ as csycon_3;
pub use csycon_rook_ as csycon_rook;
pub use csyconv_ as csyconv;
pub use csyconvf_ as csyconvf;
pub use csyconvf_rook_ as csyconvf_rook;
pub use csyequb_ as csyequb;
pub use csymv_ as csymv;
pub use csyr_ as csyr;
pub use csyrfs_ as csyrfs;
pub use csyrfsx_ as csyrfsx;
pub use csysv_ as csysv;
pub use csysv_aa_ as csysv_aa;
pub use csysv_aa_2stage_ as csysv_aa_2stage;
pub use csysv_rk_ as csysv_rk;
pub use csysv_rook_ as csysv_rook;
pub use csysvx_ as csysvx;
pub use csysvxx_ as csysvxx;
pub use csyswapr_ as csyswapr;
pub use csytf2_ as csytf2;
pub use csytf2_rk_ as csytf2_rk;
pub use csytf2_rook_ as csytf2_rook;
pub use csytrf_ as csytrf;
pub use csytrf_aa_ as csytrf_aa;
pub use csytrf_aa_2stage_ as csytrf_aa_2stage;
pub use csytrf_rk_ as csytrf_rk;
pub use csytrf_rook_ as csytrf_rook;
pub use csytri2_ as csytri2;
pub use csytri2x_ as csytri2x;
pub use csytri_ as csytri;
pub use csytri_3_ as csytri_3;
pub use csytri_3x_ as csytri_3x;
pub use csytri_rook_ as csytri_rook;
pub use csytrs2_ as csytrs2;
pub use csytrs_ as csytrs;
pub use csytrs_3_ as csytrs_3;
pub use csytrs_aa_ as csytrs_aa;
pub use csytrs_aa_2stage_ as csytrs_aa_2stage;
pub use csytrs_rook_ as csytrs_rook;
pub use ctbcon_ as ctbcon;
pub use ctbrfs_ as ctbrfs;
pub use ctbtrs_ as ctbtrs;
pub use ctfsm_ as ctfsm;
pub use ctftri_ as ctftri;
pub use ctfttp_ as ctfttp;
pub use ctfttr_ as ctfttr;
pub use ctgevc_ as ctgevc;
pub use ctgex2_ as ctgex2;
pub use ctgexc_ as ctgexc;
pub use ctgsen_ as ctgsen;
pub use ctgsja_ as ctgsja;
pub use ctgsna_ as ctgsna;
pub use ctgsy2_ as ctgsy2;
pub use ctgsyl_ as ctgsyl;
pub use ctpcon_ as ctpcon;
pub use ctplqt2_ as ctplqt2;
pub use ctplqt_ as ctplqt;
pub use ctpmlqt_ as ctpmlqt;
pub use ctpmqrt_ as ctpmqrt;
pub use ctpqrt2_ as ctpqrt2;
pub use ctpqrt_ as ctpqrt;
pub use ctprfb_ as ctprfb;
pub use ctprfs_ as ctprfs;
pub use ctptri_ as ctptri;
pub use ctptrs_ as ctptrs;
pub use ctpttf_ as ctpttf;
pub use ctpttr_ as ctpttr;
pub use ctrcon_ as ctrcon;
pub use ctrevc3_ as ctrevc3;
pub use ctrevc_ as ctrevc;
pub use ctrexc_ as ctrexc;
pub use ctrrfs_ as ctrrfs;
pub use ctrsen_ as ctrsen;
pub use ctrsna_ as ctrsna;
pub use ctrsyl3_ as ctrsyl3;
pub use ctrsyl_ as ctrsyl;
pub use ctrti2_ as ctrti2;
pub use ctrtri_ as ctrtri;
pub use ctrtrs_ as ctrtrs;
pub use ctrttf_ as ctrttf;
pub use ctrttp_ as ctrttp;
pub use ctzrqf_ as ctzrqf;
pub use ctzrzf_ as ctzrzf;
pub use cunbdb1_ as cunbdb1;
pub use cunbdb2_ as cunbdb2;
pub use cunbdb3_ as cunbdb3;
pub use cunbdb4_ as cunbdb4;
pub use cunbdb5_ as cunbdb5;
pub use cunbdb6_ as cunbdb6;
pub use cunbdb_ as cunbdb;
pub use cuncsd2by1_ as cuncsd2by1;
pub use cuncsd_ as cuncsd;
pub use cung2l_ as cung2l;
pub use cung2r_ as cung2r;
pub use cungbr_ as cungbr;
pub use cunghr_ as cunghr;
pub use cungl2_ as cungl2;
pub use cunglq_ as cunglq;
pub use cungql_ as cungql;
pub use cungqr_ as cungqr;
pub use cungr2_ as cungr2;
pub use cungrq_ as cungrq;
pub use cungtr_ as cungtr;
pub use cungtsqr_ as cungtsqr;
pub use cungtsqr_row_ as cungtsqr_row;
pub use cunhr_col_ as cunhr_col;
pub use cunm22_ as cunm22;
pub use cunm2l_ as cunm2l;
pub use cunm2r_ as cunm2r;
pub use cunmbr_ as cunmbr;
pub use cunmhr_ as cunmhr;
pub use cunml2_ as cunml2;
pub use cunmlq_ as cunmlq;
pub use cunmql_ as cunmql;
pub use cunmqr_ as cunmqr;
pub use cunmr2_ as cunmr2;
pub use cunmr3_ as cunmr3;
pub use cunmrq_ as cunmrq;
pub use cunmrz_ as cunmrz;
pub use cunmtr_ as cunmtr;
pub use cupgtr_ as cupgtr;
pub use cupmtr_ as cupmtr;
pub use dbbcsd_ as dbbcsd;
pub use dbdsdc_ as dbdsdc;
pub use dbdsqr_ as dbdsqr;
pub use dbdsvdx_ as dbdsvdx;
pub use ddisna_ as ddisna;
pub use ddtsvb_ as ddtsvb;
pub use ddttrfb_ as ddttrfb;
pub use ddttrsb_ as ddttrsb;
pub use dgbbrd_ as dgbbrd;
pub use dgbcon_ as dgbcon;
pub use dgbequ_ as dgbequ;
pub use dgbequb_ as dgbequb;
pub use dgbrfs_ as dgbrfs;
pub use dgbrfsx_ as dgbrfsx;
pub use dgbsv_ as dgbsv;
pub use dgbsvx_ as dgbsvx;
pub use dgbsvxx_ as dgbsvxx;
pub use dgbtf2_ as dgbtf2;
pub use dgbtrf_ as dgbtrf;
pub use dgbtrs_ as dgbtrs;
pub use dgebak_ as dgebak;
pub use dgebal_ as dgebal;
pub use dgebd2_ as dgebd2;
pub use dgebrd_ as dgebrd;
pub use dgecon_ as dgecon;
pub use dgedmd_ as dgedmd;
pub use dgedmdq_ as dgedmdq;
pub use dgeequ_ as dgeequ;
pub use dgeequb_ as dgeequb;
pub use dgees_ as dgees;
pub use dgeesx_ as dgeesx;
pub use dgeev_ as dgeev;
pub use dgeevx_ as dgeevx;
pub use dgegs_ as dgegs;
pub use dgegv_ as dgegv;
pub use dgehd2_ as dgehd2;
pub use dgehrd_ as dgehrd;
pub use dgejsv_ as dgejsv;
pub use dgelq2_ as dgelq2;
pub use dgelq_ as dgelq;
pub use dgelqf_ as dgelqf;
pub use dgelqt3_ as dgelqt3;
pub use dgelqt_ as dgelqt;
pub use dgels_ as dgels;
pub use dgels_batch_strided_ as dgels_batch_strided;
pub use dgelsd_ as dgelsd;
pub use dgelss_ as dgelss;
pub use dgelst_ as dgelst;
pub use dgelsx_ as dgelsx;
pub use dgelsy_ as dgelsy;
pub use dgemlq_ as dgemlq;
pub use dgemlqt_ as dgemlqt;
pub use dgemqr_ as dgemqr;
pub use dgemqrt_ as dgemqrt;
pub use dgeql2_ as dgeql2;
pub use dgeqlf_ as dgeqlf;
pub use dgeqp3_ as dgeqp3;
pub use dgeqp3rk_ as dgeqp3rk;
pub use dgeqpf_ as dgeqpf;
pub use dgeqr2_ as dgeqr2;
pub use dgeqr2p_ as dgeqr2p;
pub use dgeqr_ as dgeqr;
pub use dgeqrf_ as dgeqrf;
pub use dgeqrfp_ as dgeqrfp;
pub use dgeqrt2_ as dgeqrt2;
pub use dgeqrt3_ as dgeqrt3;
pub use dgeqrt_ as dgeqrt;
pub use dgerfs_ as dgerfs;
pub use dgerfsx_ as dgerfsx;
pub use dgerq2_ as dgerq2;
pub use dgerqf_ as dgerqf;
pub use dgesc2_ as dgesc2;
pub use dgesdd_ as dgesdd;
pub use dgesv_ as dgesv;
pub use dgesvd_ as dgesvd;
pub use dgesvda_batch_strided_ as dgesvda_batch_strided;
pub use dgesvdq_ as dgesvdq;
pub use dgesvdx_ as dgesvdx;
pub use dgesvj_ as dgesvj;
pub use dgesvx_ as dgesvx;
pub use dgesvxx_ as dgesvxx;
pub use dgetc2_ as dgetc2;
pub use dgetf2_ as dgetf2;
pub use dgetrf2_ as dgetrf2;
pub use dgetrf_ as dgetrf;
pub use dgetrf_batch_ as dgetrf_batch;
pub use dgetrf_batch_strided_ as dgetrf_batch_strided;
pub use dgetrfnp_batch_ as dgetrfnp_batch;
pub use dgetrfnp_batch_strided_ as dgetrfnp_batch_strided;
pub use dgetri_ as dgetri;
pub use dgetri_batch_strided_ as dgetri_batch_strided;
pub use dgetri_oop_batch_ as dgetri_oop_batch;
pub use dgetri_oop_batch_strided_ as dgetri_oop_batch_strided;
pub use dgetrs_ as dgetrs;
pub use dgetrs_batch_strided_ as dgetrs_batch_strided;
pub use dgetrsnp_batch_strided_ as dgetrsnp_batch_strided;
pub use dgetsls_ as dgetsls;
pub use dgetsqrhrt_ as dgetsqrhrt;
pub use dggbak_ as dggbak;
pub use dggbal_ as dggbal;
pub use dgges3_ as dgges3;
pub use dgges_ as dgges;
pub use dggesx_ as dggesx;
pub use dggev3_ as dggev3;
pub use dggev_ as dggev;
pub use dggevx_ as dggevx;
pub use dggglm_ as dggglm;
pub use dgghd3_ as dgghd3;
pub use dgghrd_ as dgghrd;
pub use dgglse_ as dgglse;
pub use dggqrf_ as dggqrf;
pub use dggrqf_ as dggrqf;
pub use dggsvd3_ as dggsvd3;
pub use dggsvd_ as dggsvd;
pub use dggsvp3_ as dggsvp3;
pub use dggsvp_ as dggsvp;
pub use dgsvj0_ as dgsvj0;
pub use dgsvj1_ as dgsvj1;
pub use dgtcon_ as dgtcon;
pub use dgtrfs_ as dgtrfs;
pub use dgtsv_ as dgtsv;
pub use dgtsvx_ as dgtsvx;
pub use dgttrf_ as dgttrf;
pub use dgttrs_ as dgttrs;
pub use dgtts2_ as dgtts2;
pub use dhgeqz_ as dhgeqz;
pub use dhsein_ as dhsein;
pub use dhseqr_ as dhseqr;
pub use disnan_ as disnan;
pub use dla_gbamv_ as dla_gbamv;
pub use dla_gbrcond_ as dla_gbrcond;
pub use dla_gbrfsx_extended_ as dla_gbrfsx_extended;
pub use dla_gbrpvgrw_ as dla_gbrpvgrw;
pub use dla_geamv_ as dla_geamv;
pub use dla_gercond_ as dla_gercond;
pub use dla_gerfsx_extended_ as dla_gerfsx_extended;
pub use dla_gerpvgrw_ as dla_gerpvgrw;
pub use dla_lin_berr_ as dla_lin_berr;
pub use dla_porcond_ as dla_porcond;
pub use dla_porfsx_extended_ as dla_porfsx_extended;
pub use dla_porpvgrw_ as dla_porpvgrw;
pub use dla_syamv_ as dla_syamv;
pub use dla_syrcond_ as dla_syrcond;
pub use dla_syrfsx_extended_ as dla_syrfsx_extended;
pub use dla_syrpvgrw_ as dla_syrpvgrw;
pub use dla_wwaddw_ as dla_wwaddw;
pub use dlabad_ as dlabad;
pub use dlabrd_ as dlabrd;
pub use dlacn2_ as dlacn2;
pub use dlacon_ as dlacon;
pub use dlacpy_ as dlacpy;
pub use dladiv_ as dladiv;
pub use dlae2_ as dlae2;
pub use dlaebz_ as dlaebz;
pub use dlaed0_ as dlaed0;
pub use dlaed1_ as dlaed1;
pub use dlaed2_ as dlaed2;
pub use dlaed3_ as dlaed3;
pub use dlaed4_ as dlaed4;
pub use dlaed5_ as dlaed5;
pub use dlaed6_ as dlaed6;
pub use dlaed7_ as dlaed7;
pub use dlaed8_ as dlaed8;
pub use dlaed9_ as dlaed9;
pub use dlaeda_ as dlaeda;
pub use dlaein_ as dlaein;
pub use dlaev2_ as dlaev2;
pub use dlaexc_ as dlaexc;
pub use dlag2_ as dlag2;
pub use dlag2s_ as dlag2s;
pub use dlagge_ as dlagge;
pub use dlags2_ as dlags2;
pub use dlagsy_ as dlagsy;
pub use dlagtf_ as dlagtf;
pub use dlagtm_ as dlagtm;
pub use dlagts_ as dlagts;
pub use dlagv2_ as dlagv2;
pub use dlahqr_ as dlahqr;
pub use dlahr2_ as dlahr2;
pub use dlahrd_ as dlahrd;
pub use dlaic1_ as dlaic1;
pub use dlaisnan_ as dlaisnan;
pub use dlakf2_ as dlakf2;
pub use dlaln2_ as dlaln2;
pub use dlals0_ as dlals0;
pub use dlalsa_ as dlalsa;
pub use dlalsd_ as dlalsd;
pub use dlamc1_ as dlamc1;
pub use dlamc2_ as dlamc2;
pub use dlamc3_ as dlamc3;
pub use dlamc4_ as dlamc4;
pub use dlamc5_ as dlamc5;
pub use dlamch_ as dlamch;
pub use dlamrg_ as dlamrg;
pub use dlamswlq_ as dlamswlq;
pub use dlamtsqr_ as dlamtsqr;
pub use dlaneg_ as dlaneg;
pub use dlangb_ as dlangb;
pub use dlange_ as dlange;
pub use dlangt_ as dlangt;
pub use dlanhs_ as dlanhs;
pub use dlansb_ as dlansb;
pub use dlansf_ as dlansf;
pub use dlansp_ as dlansp;
pub use dlanst_ as dlanst;
pub use dlansy_ as dlansy;
pub use dlantb_ as dlantb;
pub use dlantp_ as dlantp;
pub use dlantr_ as dlantr;
pub use dlanv2_ as dlanv2;
pub use dlaorhr_col_getrfnp2_ as dlaorhr_col_getrfnp2;
pub use dlaorhr_col_getrfnp_ as dlaorhr_col_getrfnp;
pub use dlapll_ as dlapll;
pub use dlapmr_ as dlapmr;
pub use dlapmt_ as dlapmt;
pub use dlapy2_ as dlapy2;
pub use dlapy3_ as dlapy3;
pub use dlaqgb_ as dlaqgb;
pub use dlaqge_ as dlaqge;
pub use dlaqp2_ as dlaqp2;
pub use dlaqps_ as dlaqps;
pub use dlaqr0_ as dlaqr0;
pub use dlaqr1_ as dlaqr1;
pub use dlaqr2_ as dlaqr2;
pub use dlaqr3_ as dlaqr3;
pub use dlaqr4_ as dlaqr4;
pub use dlaqr5_ as dlaqr5;
pub use dlaqsb_ as dlaqsb;
pub use dlaqsp_ as dlaqsp;
pub use dlaqsy_ as dlaqsy;
pub use dlaqtr_ as dlaqtr;
pub use dlaqz0_ as dlaqz0;
pub use dlaqz1_ as dlaqz1;
pub use dlaqz2_ as dlaqz2;
pub use dlaqz3_ as dlaqz3;
pub use dlaqz4_ as dlaqz4;
pub use dlar1v_ as dlar1v;
pub use dlar2v_ as dlar2v;
pub use dlaran_ as dlaran;
pub use dlarf1f_ as dlarf1f;
pub use dlarf1l_ as dlarf1l;
pub use dlarf_ as dlarf;
pub use dlarfb_ as dlarfb;
pub use dlarfb_gett_ as dlarfb_gett;
pub use dlarfg_ as dlarfg;
pub use dlarfgp_ as dlarfgp;
pub use dlarfp_ as dlarfp;
pub use dlarft_ as dlarft;
pub use dlarfx_ as dlarfx;
pub use dlarfy_ as dlarfy;
pub use dlarge_ as dlarge;
pub use dlargv_ as dlargv;
pub use dlarnd_ as dlarnd;
pub use dlarnv_ as dlarnv;
pub use dlaror_ as dlaror;
pub use dlarot_ as dlarot;
pub use dlarra_ as dlarra;
pub use dlarrb_ as dlarrb;
pub use dlarrc_ as dlarrc;
pub use dlarrd_ as dlarrd;
pub use dlarre_ as dlarre;
pub use dlarrf_ as dlarrf;
pub use dlarrj_ as dlarrj;
pub use dlarrk_ as dlarrk;
pub use dlarrr_ as dlarrr;
pub use dlarrv_ as dlarrv;
pub use dlarscl2_ as dlarscl2;
pub use dlartg_ as dlartg;
pub use dlartgp_ as dlartgp;
pub use dlartgs_ as dlartgs;
pub use dlartv_ as dlartv;
pub use dlaruv_ as dlaruv;
pub use dlarz_ as dlarz;
pub use dlarzb_ as dlarzb;
pub use dlarzt_ as dlarzt;
pub use dlas2_ as dlas2;
pub use dlascl2_ as dlascl2;
pub use dlascl_ as dlascl;
pub use dlasd0_ as dlasd0;
pub use dlasd1_ as dlasd1;
pub use dlasd2_ as dlasd2;
pub use dlasd3_ as dlasd3;
pub use dlasd4_ as dlasd4;
pub use dlasd5_ as dlasd5;
pub use dlasd6_ as dlasd6;
pub use dlasd7_ as dlasd7;
pub use dlasd8_ as dlasd8;
pub use dlasda_ as dlasda;
pub use dlasdq_ as dlasdq;
pub use dlasdt_ as dlasdt;
pub use dlaset_ as dlaset;
pub use dlasq1_ as dlasq1;
pub use dlasq2_ as dlasq2;
pub use dlasq3_ as dlasq3;
pub use dlasq4_ as dlasq4;
pub use dlasq5_ as dlasq5;
pub use dlasq6_ as dlasq6;
pub use dlasr_ as dlasr;
pub use dlasrt_ as dlasrt;
pub use dlassq_ as dlassq;
pub use dlasv2_ as dlasv2;
pub use dlaswlq_ as dlaswlq;
pub use dlaswp_ as dlaswp;
pub use dlasy2_ as dlasy2;
pub use dlasyf_ as dlasyf;
pub use dlasyf_aa_ as dlasyf_aa;
pub use dlasyf_rk_ as dlasyf_rk;
pub use dlasyf_rook_ as dlasyf_rook;
pub use dlat2s_ as dlat2s;
pub use dlatbs_ as dlatbs;
pub use dlatdf_ as dlatdf;
pub use dlatm1_ as dlatm1;
pub use dlatm2_ as dlatm2;
pub use dlatm3_ as dlatm3;
pub use dlatm5_ as dlatm5;
pub use dlatm6_ as dlatm6;
pub use dlatme_ as dlatme;
pub use dlatmr_ as dlatmr;
pub use dlatms_ as dlatms;
pub use dlatps_ as dlatps;
pub use dlatrd_ as dlatrd;
pub use dlatrs3_ as dlatrs3;
pub use dlatrs_ as dlatrs;
pub use dlatrz_ as dlatrz;
pub use dlatsqr_ as dlatsqr;
pub use dlatzm_ as dlatzm;
pub use dlauu2_ as dlauu2;
pub use dlauum_ as dlauum;
pub use dopgtr_ as dopgtr;
pub use dopmtr_ as dopmtr;
pub use dorbdb1_ as dorbdb1;
pub use dorbdb2_ as dorbdb2;
pub use dorbdb3_ as dorbdb3;
pub use dorbdb4_ as dorbdb4;
pub use dorbdb5_ as dorbdb5;
pub use dorbdb6_ as dorbdb6;
pub use dorbdb_ as dorbdb;
pub use dorcsd2by1_ as dorcsd2by1;
pub use dorcsd_ as dorcsd;
pub use dorg2l_ as dorg2l;
pub use dorg2r_ as dorg2r;
pub use dorgbr_ as dorgbr;
pub use dorghr_ as dorghr;
pub use dorgl2_ as dorgl2;
pub use dorglq_ as dorglq;
pub use dorgql_ as dorgql;
pub use dorgqr_ as dorgqr;
pub use dorgr2_ as dorgr2;
pub use dorgrq_ as dorgrq;
pub use dorgtr_ as dorgtr;
pub use dorgtsqr_ as dorgtsqr;
pub use dorgtsqr_row_ as dorgtsqr_row;
pub use dorhr_col_ as dorhr_col;
pub use dorm22_ as dorm22;
pub use dorm2l_ as dorm2l;
pub use dorm2r_ as dorm2r;
pub use dormbr_ as dormbr;
pub use dormhr_ as dormhr;
pub use dorml2_ as dorml2;
pub use dormlq_ as dormlq;
pub use dormql_ as dormql;
pub use dormqr_ as dormqr;
pub use dormr2_ as dormr2;
pub use dormr3_ as dormr3;
pub use dormrq_ as dormrq;
pub use dormrz_ as dormrz;
pub use dormtr_ as dormtr;
pub use dpbcon_ as dpbcon;
pub use dpbequ_ as dpbequ;
pub use dpbrfs_ as dpbrfs;
pub use dpbstf_ as dpbstf;
pub use dpbsv_ as dpbsv;
pub use dpbsvx_ as dpbsvx;
pub use dpbtf2_ as dpbtf2;
pub use dpbtrf_ as dpbtrf;
pub use dpbtrs_ as dpbtrs;
pub use dpftrf_ as dpftrf;
pub use dpftri_ as dpftri;
pub use dpftrs_ as dpftrs;
pub use dpocon_ as dpocon;
pub use dpoequ_ as dpoequ;
pub use dpoequb_ as dpoequb;
pub use dporfs_ as dporfs;
pub use dporfsx_ as dporfsx;
pub use dposv_ as dposv;
pub use dposvx_ as dposvx;
pub use dposvxx_ as dposvxx;
pub use dpotf2_ as dpotf2;
pub use dpotrf2_ as dpotrf2;
pub use dpotrf_ as dpotrf;
pub use dpotri_ as dpotri;
pub use dpotrs_ as dpotrs;
pub use dppcon_ as dppcon;
pub use dppequ_ as dppequ;
pub use dpprfs_ as dpprfs;
pub use dppsv_ as dppsv;
pub use dppsvx_ as dppsvx;
pub use dpptrf_ as dpptrf;
pub use dpptri_ as dpptri;
pub use dpptrs_ as dpptrs;
pub use dpstf2_ as dpstf2;
pub use dpstrf_ as dpstrf;
pub use dptcon_ as dptcon;
pub use dpteqr_ as dpteqr;
pub use dptrfs_ as dptrfs;
pub use dptsv_ as dptsv;
pub use dptsvx_ as dptsvx;
pub use dpttrf_ as dpttrf;
pub use dpttrs_ as dpttrs;
pub use dptts2_ as dptts2;
pub use drscl_ as drscl;
pub use dsb2st_kernels_ as dsb2st_kernels;
pub use dsbev_ as dsbev;
pub use dsbev_2stage_ as dsbev_2stage;
pub use dsbevd_ as dsbevd;
pub use dsbevd_2stage_ as dsbevd_2stage;
pub use dsbevx_ as dsbevx;
pub use dsbevx_2stage_ as dsbevx_2stage;
pub use dsbgst_ as dsbgst;
pub use dsbgv_ as dsbgv;
pub use dsbgvd_ as dsbgvd;
pub use dsbgvx_ as dsbgvx;
pub use dsbtrd_ as dsbtrd;
pub use dsecnd_ as dsecnd;
pub use dsfrk_ as dsfrk;
pub use dsgesv_ as dsgesv;
pub use dspcon_ as dspcon;
pub use dspev_ as dspev;
pub use dspevd_ as dspevd;
pub use dspevx_ as dspevx;
pub use dspgst_ as dspgst;
pub use dspgv_ as dspgv;
pub use dspgvd_ as dspgvd;
pub use dspgvx_ as dspgvx;
pub use dsposv_ as dsposv;
pub use dsprfs_ as dsprfs;
pub use dspsv_ as dspsv;
pub use dspsvx_ as dspsvx;
pub use dsptrd_ as dsptrd;
pub use dsptrf_ as dsptrf;
pub use dsptri_ as dsptri;
pub use dsptrs_ as dsptrs;
pub use dstebz_ as dstebz;
pub use dstedc_ as dstedc;
pub use dstegr_ as dstegr;
pub use dstein_ as dstein;
pub use dstemr_ as dstemr;
pub use dsteqr_ as dsteqr;
pub use dsterf_ as dsterf;
pub use dstev_ as dstev;
pub use dstevd_ as dstevd;
pub use dstevr_ as dstevr;
pub use dstevx_ as dstevx;
pub use dsycon_ as dsycon;
pub use dsycon_3_ as dsycon_3;
pub use dsycon_rook_ as dsycon_rook;
pub use dsyconv_ as dsyconv;
pub use dsyconvf_ as dsyconvf;
pub use dsyconvf_rook_ as dsyconvf_rook;
pub use dsyequb_ as dsyequb;
pub use dsyev_ as dsyev;
pub use dsyev_2stage_ as dsyev_2stage;
pub use dsyevd_ as dsyevd;
pub use dsyevd_2stage_ as dsyevd_2stage;
pub use dsyevr_ as dsyevr;
pub use dsyevr_2stage_ as dsyevr_2stage;
pub use dsyevx_ as dsyevx;
pub use dsyevx_2stage_ as dsyevx_2stage;
pub use dsygs2_ as dsygs2;
pub use dsygst_ as dsygst;
pub use dsygv_ as dsygv;
pub use dsygv_2stage_ as dsygv_2stage;
pub use dsygvd_ as dsygvd;
pub use dsygvx_ as dsygvx;
pub use dsyrdb_ as dsyrdb;
pub use dsyrfs_ as dsyrfs;
pub use dsyrfsx_ as dsyrfsx;
pub use dsysv_ as dsysv;
pub use dsysv_aa_ as dsysv_aa;
pub use dsysv_aa_2stage_ as dsysv_aa_2stage;
pub use dsysv_rk_ as dsysv_rk;
pub use dsysv_rook_ as dsysv_rook;
pub use dsysvx_ as dsysvx;
pub use dsysvxx_ as dsysvxx;
pub use dsyswapr_ as dsyswapr;
pub use dsytd2_ as dsytd2;
pub use dsytf2_ as dsytf2;
pub use dsytf2_rk_ as dsytf2_rk;
pub use dsytf2_rook_ as dsytf2_rook;
pub use dsytrd_ as dsytrd;
pub use dsytrd_2stage_ as dsytrd_2stage;
pub use dsytrd_sb2st_ as dsytrd_sb2st;
pub use dsytrd_sy2sb_ as dsytrd_sy2sb;
pub use dsytrf_ as dsytrf;
pub use dsytrf_aa_ as dsytrf_aa;
pub use dsytrf_aa_2stage_ as dsytrf_aa_2stage;
pub use dsytrf_rk_ as dsytrf_rk;
pub use dsytrf_rook_ as dsytrf_rook;
pub use dsytri2_ as dsytri2;
pub use dsytri2x_ as dsytri2x;
pub use dsytri_ as dsytri;
pub use dsytri_3_ as dsytri_3;
pub use dsytri_3x_ as dsytri_3x;
pub use dsytri_rook_ as dsytri_rook;
pub use dsytrs2_ as dsytrs2;
pub use dsytrs_ as dsytrs;
pub use dsytrs_3_ as dsytrs_3;
pub use dsytrs_aa_ as dsytrs_aa;
pub use dsytrs_aa_2stage_ as dsytrs_aa_2stage;
pub use dsytrs_rook_ as dsytrs_rook;
pub use dtbcon_ as dtbcon;
pub use dtbrfs_ as dtbrfs;
pub use dtbtrs_ as dtbtrs;
pub use dtfsm_ as dtfsm;
pub use dtftri_ as dtftri;
pub use dtfttp_ as dtfttp;
pub use dtfttr_ as dtfttr;
pub use dtgevc_ as dtgevc;
pub use dtgex2_ as dtgex2;
pub use dtgexc_ as dtgexc;
pub use dtgsen_ as dtgsen;
pub use dtgsja_ as dtgsja;
pub use dtgsna_ as dtgsna;
pub use dtgsy2_ as dtgsy2;
pub use dtgsyl_ as dtgsyl;
pub use dtpcon_ as dtpcon;
pub use dtplqt2_ as dtplqt2;
pub use dtplqt_ as dtplqt;
pub use dtpmlqt_ as dtpmlqt;
pub use dtpmqrt_ as dtpmqrt;
pub use dtpqrt2_ as dtpqrt2;
pub use dtpqrt_ as dtpqrt;
pub use dtprfb_ as dtprfb;
pub use dtprfs_ as dtprfs;
pub use dtptri_ as dtptri;
pub use dtptrs_ as dtptrs;
pub use dtpttf_ as dtpttf;
pub use dtpttr_ as dtpttr;
pub use dtrcon_ as dtrcon;
pub use dtrevc3_ as dtrevc3;
pub use dtrevc_ as dtrevc;
pub use dtrexc_ as dtrexc;
pub use dtrrfs_ as dtrrfs;
pub use dtrsen_ as dtrsen;
pub use dtrsna_ as dtrsna;
pub use dtrsyl3_ as dtrsyl3;
pub use dtrsyl_ as dtrsyl;
pub use dtrti2_ as dtrti2;
pub use dtrtri_ as dtrtri;
pub use dtrtrs_ as dtrtrs;
pub use dtrttf_ as dtrttf;
pub use dtrttp_ as dtrttp;
pub use dtzrqf_ as dtzrqf;
pub use dtzrzf_ as dtzrzf;
pub use dzsum1_ as dzsum1;
pub use icmax1_ as icmax1;
pub use ieeeck_ as ieeeck;
pub use ilaclc_ as ilaclc;
pub use ilaclr_ as ilaclr;
pub use iladiag_ as iladiag;
pub use iladlc_ as iladlc;
pub use iladlr_ as iladlr;
pub use ilaenv2stage_ as ilaenv2stage;
pub use ilaenv_ as ilaenv;
pub use ilaprec_ as ilaprec;
pub use ilaslc_ as ilaslc;
pub use ilaslr_ as ilaslr;
pub use ilatrans_ as ilatrans;
pub use ilauplo_ as ilauplo;
pub use ilaver_ as ilaver;
pub use ilazlc_ as ilazlc;
pub use ilazlr_ as ilazlr;
pub use iparam2stage_ as iparam2stage;
pub use iparmq_ as iparmq;
pub use izmax1_ as izmax1;
pub use lsamen_ as lsamen;
pub use mkl_cgetrfnp_ as mkl_cgetrfnp;
pub use mkl_cgetrfnpi_ as mkl_cgetrfnpi;
pub use mkl_cgetrinp_ as mkl_cgetrinp;
pub use mkl_cspffrt2_ as mkl_cspffrt2;
pub use mkl_cspffrtx_ as mkl_cspffrtx;
pub use mkl_ctppack_ as mkl_ctppack;
pub use mkl_ctpunpack_ as mkl_ctpunpack;
pub use mkl_dgetrfnp_ as mkl_dgetrfnp;
pub use mkl_dgetrfnpi_ as mkl_dgetrfnpi;
pub use mkl_dgetrinp_ as mkl_dgetrinp;
pub use mkl_dspffrt2_ as mkl_dspffrt2;
pub use mkl_dspffrtx_ as mkl_dspffrtx;
pub use mkl_dtppack_ as mkl_dtppack;
pub use mkl_dtpunpack_ as mkl_dtpunpack;
pub use mkl_progress_ as mkl_progress;
pub use mkl_sgetrfnp_ as mkl_sgetrfnp;
pub use mkl_sgetrfnpi_ as mkl_sgetrfnpi;
pub use mkl_sgetrinp_ as mkl_sgetrinp;
pub use mkl_sspffrt2_ as mkl_sspffrt2;
pub use mkl_sspffrtx_ as mkl_sspffrtx;
pub use mkl_stppack_ as mkl_stppack;
pub use mkl_stpunpack_ as mkl_stpunpack;
pub use mkl_zgetrfnp_ as mkl_zgetrfnp;
pub use mkl_zgetrfnpi_ as mkl_zgetrfnpi;
pub use mkl_zgetrinp_ as mkl_zgetrinp;
pub use mkl_zspffrt2_ as mkl_zspffrt2;
pub use mkl_zspffrtx_ as mkl_zspffrtx;
pub use mkl_ztppack_ as mkl_ztppack;
pub use mkl_ztpunpack_ as mkl_ztpunpack;
pub use sbbcsd_ as sbbcsd;
pub use sbdsdc_ as sbdsdc;
pub use sbdsqr_ as sbdsqr;
pub use sbdsvdx_ as sbdsvdx;
pub use scsum1_ as scsum1;
pub use sdisna_ as sdisna;
pub use sdtsvb_ as sdtsvb;
pub use sdttrfb_ as sdttrfb;
pub use sdttrsb_ as sdttrsb;
pub use second_ as second;
pub use sgbbrd_ as sgbbrd;
pub use sgbcon_ as sgbcon;
pub use sgbequ_ as sgbequ;
pub use sgbequb_ as sgbequb;
pub use sgbrfs_ as sgbrfs;
pub use sgbrfsx_ as sgbrfsx;
pub use sgbsv_ as sgbsv;
pub use sgbsvx_ as sgbsvx;
pub use sgbsvxx_ as sgbsvxx;
pub use sgbtf2_ as sgbtf2;
pub use sgbtrf_ as sgbtrf;
pub use sgbtrs_ as sgbtrs;
pub use sgebak_ as sgebak;
pub use sgebal_ as sgebal;
pub use sgebd2_ as sgebd2;
pub use sgebrd_ as sgebrd;
pub use sgecon_ as sgecon;
pub use sgedmd_ as sgedmd;
pub use sgedmdq_ as sgedmdq;
pub use sgeequ_ as sgeequ;
pub use sgeequb_ as sgeequb;
pub use sgees_ as sgees;
pub use sgeesx_ as sgeesx;
pub use sgeev_ as sgeev;
pub use sgeevx_ as sgeevx;
pub use sgegs_ as sgegs;
pub use sgegv_ as sgegv;
pub use sgehd2_ as sgehd2;
pub use sgehrd_ as sgehrd;
pub use sgejsv_ as sgejsv;
pub use sgelq2_ as sgelq2;
pub use sgelq_ as sgelq;
pub use sgelqf_ as sgelqf;
pub use sgelqt3_ as sgelqt3;
pub use sgelqt_ as sgelqt;
pub use sgels_ as sgels;
pub use sgels_batch_strided_ as sgels_batch_strided;
pub use sgelsd_ as sgelsd;
pub use sgelss_ as sgelss;
pub use sgelst_ as sgelst;
pub use sgelsx_ as sgelsx;
pub use sgelsy_ as sgelsy;
pub use sgemlq_ as sgemlq;
pub use sgemlqt_ as sgemlqt;
pub use sgemqr_ as sgemqr;
pub use sgemqrt_ as sgemqrt;
pub use sgeql2_ as sgeql2;
pub use sgeqlf_ as sgeqlf;
pub use sgeqp3_ as sgeqp3;
pub use sgeqp3rk_ as sgeqp3rk;
pub use sgeqpf_ as sgeqpf;
pub use sgeqr2_ as sgeqr2;
pub use sgeqr2p_ as sgeqr2p;
pub use sgeqr_ as sgeqr;
pub use sgeqrf_ as sgeqrf;
pub use sgeqrfp_ as sgeqrfp;
pub use sgeqrt2_ as sgeqrt2;
pub use sgeqrt3_ as sgeqrt3;
pub use sgeqrt_ as sgeqrt;
pub use sgerfs_ as sgerfs;
pub use sgerfsx_ as sgerfsx;
pub use sgerq2_ as sgerq2;
pub use sgerqf_ as sgerqf;
pub use sgesc2_ as sgesc2;
pub use sgesdd_ as sgesdd;
pub use sgesv_ as sgesv;
pub use sgesvd_ as sgesvd;
pub use sgesvda_batch_strided_ as sgesvda_batch_strided;
pub use sgesvdq_ as sgesvdq;
pub use sgesvdx_ as sgesvdx;
pub use sgesvj_ as sgesvj;
pub use sgesvx_ as sgesvx;
pub use sgesvxx_ as sgesvxx;
pub use sgetc2_ as sgetc2;
pub use sgetf2_ as sgetf2;
pub use sgetrf2_ as sgetrf2;
pub use sgetrf_ as sgetrf;
pub use sgetrf_batch_ as sgetrf_batch;
pub use sgetrf_batch_strided_ as sgetrf_batch_strided;
pub use sgetrfnp_batch_ as sgetrfnp_batch;
pub use sgetrfnp_batch_strided_ as sgetrfnp_batch_strided;
pub use sgetri_ as sgetri;
pub use sgetri_batch_strided_ as sgetri_batch_strided;
pub use sgetri_oop_batch_ as sgetri_oop_batch;
pub use sgetri_oop_batch_strided_ as sgetri_oop_batch_strided;
pub use sgetrs_ as sgetrs;
pub use sgetrs_batch_strided_ as sgetrs_batch_strided;
pub use sgetrsnp_batch_strided_ as sgetrsnp_batch_strided;
pub use sgetsls_ as sgetsls;
pub use sgetsqrhrt_ as sgetsqrhrt;
pub use sggbak_ as sggbak;
pub use sggbal_ as sggbal;
pub use sgges3_ as sgges3;
pub use sgges_ as sgges;
pub use sggesx_ as sggesx;
pub use sggev3_ as sggev3;
pub use sggev_ as sggev;
pub use sggevx_ as sggevx;
pub use sggglm_ as sggglm;
pub use sgghd3_ as sgghd3;
pub use sgghrd_ as sgghrd;
pub use sgglse_ as sgglse;
pub use sggqrf_ as sggqrf;
pub use sggrqf_ as sggrqf;
pub use sggsvd3_ as sggsvd3;
pub use sggsvd_ as sggsvd;
pub use sggsvp3_ as sggsvp3;
pub use sggsvp_ as sggsvp;
pub use sgsvj0_ as sgsvj0;
pub use sgsvj1_ as sgsvj1;
pub use sgtcon_ as sgtcon;
pub use sgtrfs_ as sgtrfs;
pub use sgtsv_ as sgtsv;
pub use sgtsvx_ as sgtsvx;
pub use sgttrf_ as sgttrf;
pub use sgttrs_ as sgttrs;
pub use sgtts2_ as sgtts2;
pub use shgeqz_ as shgeqz;
pub use shsein_ as shsein;
pub use shseqr_ as shseqr;
pub use sisnan_ as sisnan;
pub use sla_gbamv_ as sla_gbamv;
pub use sla_gbrcond_ as sla_gbrcond;
pub use sla_gbrfsx_extended_ as sla_gbrfsx_extended;
pub use sla_gbrpvgrw_ as sla_gbrpvgrw;
pub use sla_geamv_ as sla_geamv;
pub use sla_gercond_ as sla_gercond;
pub use sla_gerfsx_extended_ as sla_gerfsx_extended;
pub use sla_gerpvgrw_ as sla_gerpvgrw;
pub use sla_lin_berr_ as sla_lin_berr;
pub use sla_porcond_ as sla_porcond;
pub use sla_porfsx_extended_ as sla_porfsx_extended;
pub use sla_porpvgrw_ as sla_porpvgrw;
pub use sla_syamv_ as sla_syamv;
pub use sla_syrcond_ as sla_syrcond;
pub use sla_syrfsx_extended_ as sla_syrfsx_extended;
pub use sla_syrpvgrw_ as sla_syrpvgrw;
pub use sla_wwaddw_ as sla_wwaddw;
pub use slabad_ as slabad;
pub use slabrd_ as slabrd;
pub use slacn2_ as slacn2;
pub use slacon_ as slacon;
pub use slacpy_ as slacpy;
pub use sladiv_ as sladiv;
pub use slae2_ as slae2;
pub use slaebz_ as slaebz;
pub use slaed0_ as slaed0;
pub use slaed1_ as slaed1;
pub use slaed2_ as slaed2;
pub use slaed3_ as slaed3;
pub use slaed4_ as slaed4;
pub use slaed5_ as slaed5;
pub use slaed6_ as slaed6;
pub use slaed7_ as slaed7;
pub use slaed8_ as slaed8;
pub use slaed9_ as slaed9;
pub use slaeda_ as slaeda;
pub use slaein_ as slaein;
pub use slaev2_ as slaev2;
pub use slaexc_ as slaexc;
pub use slag2_ as slag2;
pub use slag2d_ as slag2d;
pub use slagge_ as slagge;
pub use slags2_ as slags2;
pub use slagsy_ as slagsy;
pub use slagtf_ as slagtf;
pub use slagtm_ as slagtm;
pub use slagts_ as slagts;
pub use slagv2_ as slagv2;
pub use slahqr_ as slahqr;
pub use slahr2_ as slahr2;
pub use slahrd_ as slahrd;
pub use slaic1_ as slaic1;
pub use slaisnan_ as slaisnan;
pub use slakf2_ as slakf2;
pub use slaln2_ as slaln2;
pub use slals0_ as slals0;
pub use slalsa_ as slalsa;
pub use slalsd_ as slalsd;
pub use slamc1_ as slamc1;
pub use slamc2_ as slamc2;
pub use slamc3_ as slamc3;
pub use slamc4_ as slamc4;
pub use slamc5_ as slamc5;
pub use slamch_ as slamch;
pub use slamrg_ as slamrg;
pub use slamswlq_ as slamswlq;
pub use slamtsqr_ as slamtsqr;
pub use slaneg_ as slaneg;
pub use slangb_ as slangb;
pub use slange_ as slange;
pub use slangt_ as slangt;
pub use slanhs_ as slanhs;
pub use slansb_ as slansb;
pub use slansf_ as slansf;
pub use slansp_ as slansp;
pub use slanst_ as slanst;
pub use slansy_ as slansy;
pub use slantb_ as slantb;
pub use slantp_ as slantp;
pub use slantr_ as slantr;
pub use slanv2_ as slanv2;
pub use slaorhr_col_getrfnp2_ as slaorhr_col_getrfnp2;
pub use slaorhr_col_getrfnp_ as slaorhr_col_getrfnp;
pub use slapll_ as slapll;
pub use slapmr_ as slapmr;
pub use slapmt_ as slapmt;
pub use slapy2_ as slapy2;
pub use slapy3_ as slapy3;
pub use slaqgb_ as slaqgb;
pub use slaqge_ as slaqge;
pub use slaqp2_ as slaqp2;
pub use slaqps_ as slaqps;
pub use slaqr0_ as slaqr0;
pub use slaqr1_ as slaqr1;
pub use slaqr2_ as slaqr2;
pub use slaqr3_ as slaqr3;
pub use slaqr4_ as slaqr4;
pub use slaqr5_ as slaqr5;
pub use slaqsb_ as slaqsb;
pub use slaqsp_ as slaqsp;
pub use slaqsy_ as slaqsy;
pub use slaqtr_ as slaqtr;
pub use slaqz0_ as slaqz0;
pub use slaqz1_ as slaqz1;
pub use slaqz2_ as slaqz2;
pub use slaqz3_ as slaqz3;
pub use slaqz4_ as slaqz4;
pub use slar1v_ as slar1v;
pub use slar2v_ as slar2v;
pub use slaran_ as slaran;
pub use slarf1f_ as slarf1f;
pub use slarf1l_ as slarf1l;
pub use slarf_ as slarf;
pub use slarfb_ as slarfb;
pub use slarfb_gett_ as slarfb_gett;
pub use slarfg_ as slarfg;
pub use slarfgp_ as slarfgp;
pub use slarfp_ as slarfp;
pub use slarft_ as slarft;
pub use slarfx_ as slarfx;
pub use slarfy_ as slarfy;
pub use slarge_ as slarge;
pub use slargv_ as slargv;
pub use slarnd_ as slarnd;
pub use slarnv_ as slarnv;
pub use slaror_ as slaror;
pub use slarot_ as slarot;
pub use slarra_ as slarra;
pub use slarrb_ as slarrb;
pub use slarrc_ as slarrc;
pub use slarrd_ as slarrd;
pub use slarre_ as slarre;
pub use slarrf_ as slarrf;
pub use slarrj_ as slarrj;
pub use slarrk_ as slarrk;
pub use slarrr_ as slarrr;
pub use slarrv_ as slarrv;
pub use slarscl2_ as slarscl2;
pub use slartg_ as slartg;
pub use slartgp_ as slartgp;
pub use slartgs_ as slartgs;
pub use slartv_ as slartv;
pub use slaruv_ as slaruv;
pub use slarz_ as slarz;
pub use slarzb_ as slarzb;
pub use slarzt_ as slarzt;
pub use slas2_ as slas2;
pub use slascl2_ as slascl2;
pub use slascl_ as slascl;
pub use slasd0_ as slasd0;
pub use slasd1_ as slasd1;
pub use slasd2_ as slasd2;
pub use slasd3_ as slasd3;
pub use slasd4_ as slasd4;
pub use slasd5_ as slasd5;
pub use slasd6_ as slasd6;
pub use slasd7_ as slasd7;
pub use slasd8_ as slasd8;
pub use slasda_ as slasda;
pub use slasdq_ as slasdq;
pub use slasdt_ as slasdt;
pub use slaset_ as slaset;
pub use slasq1_ as slasq1;
pub use slasq2_ as slasq2;
pub use slasq3_ as slasq3;
pub use slasq4_ as slasq4;
pub use slasq5_ as slasq5;
pub use slasq6_ as slasq6;
pub use slasr_ as slasr;
pub use slasrt_ as slasrt;
pub use slassq_ as slassq;
pub use slasv2_ as slasv2;
pub use slaswlq_ as slaswlq;
pub use slaswp_ as slaswp;
pub use slasy2_ as slasy2;
pub use slasyf_ as slasyf;
pub use slasyf_aa_ as slasyf_aa;
pub use slasyf_rk_ as slasyf_rk;
pub use slasyf_rook_ as slasyf_rook;
pub use slatbs_ as slatbs;
pub use slatdf_ as slatdf;
pub use slatm1_ as slatm1;
pub use slatm2_ as slatm2;
pub use slatm3_ as slatm3;
pub use slatm5_ as slatm5;
pub use slatm6_ as slatm6;
pub use slatme_ as slatme;
pub use slatmr_ as slatmr;
pub use slatms_ as slatms;
pub use slatps_ as slatps;
pub use slatrd_ as slatrd;
pub use slatrs3_ as slatrs3;
pub use slatrs_ as slatrs;
pub use slatrz_ as slatrz;
pub use slatsqr_ as slatsqr;
pub use slatzm_ as slatzm;
pub use slauu2_ as slauu2;
pub use slauum_ as slauum;
pub use sopgtr_ as sopgtr;
pub use sopmtr_ as sopmtr;
pub use sorbdb1_ as sorbdb1;
pub use sorbdb2_ as sorbdb2;
pub use sorbdb3_ as sorbdb3;
pub use sorbdb4_ as sorbdb4;
pub use sorbdb5_ as sorbdb5;
pub use sorbdb6_ as sorbdb6;
pub use sorbdb_ as sorbdb;
pub use sorcsd2by1_ as sorcsd2by1;
pub use sorcsd_ as sorcsd;
pub use sorg2l_ as sorg2l;
pub use sorg2r_ as sorg2r;
pub use sorgbr_ as sorgbr;
pub use sorghr_ as sorghr;
pub use sorgl2_ as sorgl2;
pub use sorglq_ as sorglq;
pub use sorgql_ as sorgql;
pub use sorgqr_ as sorgqr;
pub use sorgr2_ as sorgr2;
pub use sorgrq_ as sorgrq;
pub use sorgtr_ as sorgtr;
pub use sorgtsqr_ as sorgtsqr;
pub use sorgtsqr_row_ as sorgtsqr_row;
pub use sorhr_col_ as sorhr_col;
pub use sorm22_ as sorm22;
pub use sorm2l_ as sorm2l;
pub use sorm2r_ as sorm2r;
pub use sormbr_ as sormbr;
pub use sormhr_ as sormhr;
pub use sorml2_ as sorml2;
pub use sormlq_ as sormlq;
pub use sormql_ as sormql;
pub use sormqr_ as sormqr;
pub use sormr2_ as sormr2;
pub use sormr3_ as sormr3;
pub use sormrq_ as sormrq;
pub use sormrz_ as sormrz;
pub use sormtr_ as sormtr;
pub use spbcon_ as spbcon;
pub use spbequ_ as spbequ;
pub use spbrfs_ as spbrfs;
pub use spbstf_ as spbstf;
pub use spbsv_ as spbsv;
pub use spbsvx_ as spbsvx;
pub use spbtf2_ as spbtf2;
pub use spbtrf_ as spbtrf;
pub use spbtrs_ as spbtrs;
pub use spftrf_ as spftrf;
pub use spftri_ as spftri;
pub use spftrs_ as spftrs;
pub use spocon_ as spocon;
pub use spoequ_ as spoequ;
pub use spoequb_ as spoequb;
pub use sporfs_ as sporfs;
pub use sporfsx_ as sporfsx;
pub use sposv_ as sposv;
pub use sposvx_ as sposvx;
pub use sposvxx_ as sposvxx;
pub use spotf2_ as spotf2;
pub use spotrf2_ as spotrf2;
pub use spotrf_ as spotrf;
pub use spotri_ as spotri;
pub use spotrs_ as spotrs;
pub use sppcon_ as sppcon;
pub use sppequ_ as sppequ;
pub use spprfs_ as spprfs;
pub use sppsv_ as sppsv;
pub use sppsvx_ as sppsvx;
pub use spptrf_ as spptrf;
pub use spptri_ as spptri;
pub use spptrs_ as spptrs;
pub use spstf2_ as spstf2;
pub use spstrf_ as spstrf;
pub use sptcon_ as sptcon;
pub use spteqr_ as spteqr;
pub use sptrfs_ as sptrfs;
pub use sptsv_ as sptsv;
pub use sptsvx_ as sptsvx;
pub use spttrf_ as spttrf;
pub use spttrs_ as spttrs;
pub use sptts2_ as sptts2;
pub use srscl_ as srscl;
pub use ssb2st_kernels_ as ssb2st_kernels;
pub use ssbev_ as ssbev;
pub use ssbev_2stage_ as ssbev_2stage;
pub use ssbevd_ as ssbevd;
pub use ssbevd_2stage_ as ssbevd_2stage;
pub use ssbevx_ as ssbevx;
pub use ssbevx_2stage_ as ssbevx_2stage;
pub use ssbgst_ as ssbgst;
pub use ssbgv_ as ssbgv;
pub use ssbgvd_ as ssbgvd;
pub use ssbgvx_ as ssbgvx;
pub use ssbtrd_ as ssbtrd;
pub use ssfrk_ as ssfrk;
pub use sspcon_ as sspcon;
pub use sspev_ as sspev;
pub use sspevd_ as sspevd;
pub use sspevx_ as sspevx;
pub use sspgst_ as sspgst;
pub use sspgv_ as sspgv;
pub use sspgvd_ as sspgvd;
pub use sspgvx_ as sspgvx;
pub use ssprfs_ as ssprfs;
pub use sspsv_ as sspsv;
pub use sspsvx_ as sspsvx;
pub use ssptrd_ as ssptrd;
pub use ssptrf_ as ssptrf;
pub use ssptri_ as ssptri;
pub use ssptrs_ as ssptrs;
pub use sstebz_ as sstebz;
pub use sstedc_ as sstedc;
pub use sstegr_ as sstegr;
pub use sstein_ as sstein;
pub use sstemr_ as sstemr;
pub use ssteqr_ as ssteqr;
pub use ssterf_ as ssterf;
pub use sstev_ as sstev;
pub use sstevd_ as sstevd;
pub use sstevr_ as sstevr;
pub use sstevx_ as sstevx;
pub use ssycon_ as ssycon;
pub use ssycon_3_ as ssycon_3;
pub use ssycon_rook_ as ssycon_rook;
pub use ssyconv_ as ssyconv;
pub use ssyconvf_ as ssyconvf;
pub use ssyconvf_rook_ as ssyconvf_rook;
pub use ssyequb_ as ssyequb;
pub use ssyev_ as ssyev;
pub use ssyev_2stage_ as ssyev_2stage;
pub use ssyevd_ as ssyevd;
pub use ssyevd_2stage_ as ssyevd_2stage;
pub use ssyevr_ as ssyevr;
pub use ssyevr_2stage_ as ssyevr_2stage;
pub use ssyevx_ as ssyevx;
pub use ssyevx_2stage_ as ssyevx_2stage;
pub use ssygs2_ as ssygs2;
pub use ssygst_ as ssygst;
pub use ssygv_ as ssygv;
pub use ssygv_2stage_ as ssygv_2stage;
pub use ssygvd_ as ssygvd;
pub use ssygvx_ as ssygvx;
pub use ssyrdb_ as ssyrdb;
pub use ssyrfs_ as ssyrfs;
pub use ssyrfsx_ as ssyrfsx;
pub use ssysv_ as ssysv;
pub use ssysv_aa_ as ssysv_aa;
pub use ssysv_aa_2stage_ as ssysv_aa_2stage;
pub use ssysv_rk_ as ssysv_rk;
pub use ssysv_rook_ as ssysv_rook;
pub use ssysvx_ as ssysvx;
pub use ssysvxx_ as ssysvxx;
pub use ssyswapr_ as ssyswapr;
pub use ssytd2_ as ssytd2;
pub use ssytf2_ as ssytf2;
pub use ssytf2_rk_ as ssytf2_rk;
pub use ssytf2_rook_ as ssytf2_rook;
pub use ssytrd_ as ssytrd;
pub use ssytrd_2stage_ as ssytrd_2stage;
pub use ssytrd_sb2st_ as ssytrd_sb2st;
pub use ssytrd_sy2sb_ as ssytrd_sy2sb;
pub use ssytrf_ as ssytrf;
pub use ssytrf_aa_ as ssytrf_aa;
pub use ssytrf_aa_2stage_ as ssytrf_aa_2stage;
pub use ssytrf_rk_ as ssytrf_rk;
pub use ssytrf_rook_ as ssytrf_rook;
pub use ssytri2_ as ssytri2;
pub use ssytri2x_ as ssytri2x;
pub use ssytri_ as ssytri;
pub use ssytri_3_ as ssytri_3;
pub use ssytri_3x_ as ssytri_3x;
pub use ssytri_rook_ as ssytri_rook;
pub use ssytrs2_ as ssytrs2;
pub use ssytrs_ as ssytrs;
pub use ssytrs_3_ as ssytrs_3;
pub use ssytrs_aa_ as ssytrs_aa;
pub use ssytrs_aa_2stage_ as ssytrs_aa_2stage;
pub use ssytrs_rook_ as ssytrs_rook;
pub use stbcon_ as stbcon;
pub use stbrfs_ as stbrfs;
pub use stbtrs_ as stbtrs;
pub use stfsm_ as stfsm;
pub use stftri_ as stftri;
pub use stfttp_ as stfttp;
pub use stfttr_ as stfttr;
pub use stgevc_ as stgevc;
pub use stgex2_ as stgex2;
pub use stgexc_ as stgexc;
pub use stgsen_ as stgsen;
pub use stgsja_ as stgsja;
pub use stgsna_ as stgsna;
pub use stgsy2_ as stgsy2;
pub use stgsyl_ as stgsyl;
pub use stpcon_ as stpcon;
pub use stplqt2_ as stplqt2;
pub use stplqt_ as stplqt;
pub use stpmlqt_ as stpmlqt;
pub use stpmqrt_ as stpmqrt;
pub use stpqrt2_ as stpqrt2;
pub use stpqrt_ as stpqrt;
pub use stprfb_ as stprfb;
pub use stprfs_ as stprfs;
pub use stptri_ as stptri;
pub use stptrs_ as stptrs;
pub use stpttf_ as stpttf;
pub use stpttr_ as stpttr;
pub use strcon_ as strcon;
pub use strevc3_ as strevc3;
pub use strevc_ as strevc;
pub use strexc_ as strexc;
pub use strrfs_ as strrfs;
pub use strsen_ as strsen;
pub use strsna_ as strsna;
pub use strsyl3_ as strsyl3;
pub use strsyl_ as strsyl;
pub use strti2_ as strti2;
pub use strtri_ as strtri;
pub use strtrs_ as strtrs;
pub use strttf_ as strttf;
pub use strttp_ as strttp;
pub use stzrqf_ as stzrqf;
pub use stzrzf_ as stzrzf;
pub use xerbla_array_ as xerbla_array;
pub use zbbcsd_ as zbbcsd;
pub use zbdsqr_ as zbdsqr;
pub use zcgesv_ as zcgesv;
pub use zcposv_ as zcposv;
pub use zdrscl_ as zdrscl;
pub use zdtsvb_ as zdtsvb;
pub use zdttrfb_ as zdttrfb;
pub use zdttrsb_ as zdttrsb;
pub use zgbbrd_ as zgbbrd;
pub use zgbcon_ as zgbcon;
pub use zgbequ_ as zgbequ;
pub use zgbequb_ as zgbequb;
pub use zgbrfs_ as zgbrfs;
pub use zgbrfsx_ as zgbrfsx;
pub use zgbsv_ as zgbsv;
pub use zgbsvx_ as zgbsvx;
pub use zgbsvxx_ as zgbsvxx;
pub use zgbtf2_ as zgbtf2;
pub use zgbtrf_ as zgbtrf;
pub use zgbtrs_ as zgbtrs;
pub use zgebak_ as zgebak;
pub use zgebal_ as zgebal;
pub use zgebd2_ as zgebd2;
pub use zgebrd_ as zgebrd;
pub use zgecon_ as zgecon;
pub use zgedmd_ as zgedmd;
pub use zgedmdq_ as zgedmdq;
pub use zgeequ_ as zgeequ;
pub use zgeequb_ as zgeequb;
pub use zgees_ as zgees;
pub use zgeesx_ as zgeesx;
pub use zgeev_ as zgeev;
pub use zgeevx_ as zgeevx;
pub use zgegs_ as zgegs;
pub use zgegv_ as zgegv;
pub use zgehd2_ as zgehd2;
pub use zgehrd_ as zgehrd;
pub use zgejsv_ as zgejsv;
pub use zgelq2_ as zgelq2;
pub use zgelq_ as zgelq;
pub use zgelqf_ as zgelqf;
pub use zgelqt3_ as zgelqt3;
pub use zgelqt_ as zgelqt;
pub use zgels_ as zgels;
pub use zgels_batch_strided_ as zgels_batch_strided;
pub use zgelsd_ as zgelsd;
pub use zgelss_ as zgelss;
pub use zgelst_ as zgelst;
pub use zgelsx_ as zgelsx;
pub use zgelsy_ as zgelsy;
pub use zgemlq_ as zgemlq;
pub use zgemlqt_ as zgemlqt;
pub use zgemqr_ as zgemqr;
pub use zgemqrt_ as zgemqrt;
pub use zgeql2_ as zgeql2;
pub use zgeqlf_ as zgeqlf;
pub use zgeqp3_ as zgeqp3;
pub use zgeqp3rk_ as zgeqp3rk;
pub use zgeqpf_ as zgeqpf;
pub use zgeqr2_ as zgeqr2;
pub use zgeqr2p_ as zgeqr2p;
pub use zgeqr_ as zgeqr;
pub use zgeqrf_ as zgeqrf;
pub use zgeqrfp_ as zgeqrfp;
pub use zgeqrt2_ as zgeqrt2;
pub use zgeqrt3_ as zgeqrt3;
pub use zgeqrt_ as zgeqrt;
pub use zgerfs_ as zgerfs;
pub use zgerfsx_ as zgerfsx;
pub use zgerq2_ as zgerq2;
pub use zgerqf_ as zgerqf;
pub use zgesc2_ as zgesc2;
pub use zgesdd_ as zgesdd;
pub use zgesv_ as zgesv;
pub use zgesvd_ as zgesvd;
pub use zgesvda_batch_strided_ as zgesvda_batch_strided;
pub use zgesvdq_ as zgesvdq;
pub use zgesvdx_ as zgesvdx;
pub use zgesvj_ as zgesvj;
pub use zgesvx_ as zgesvx;
pub use zgesvxx_ as zgesvxx;
pub use zgetc2_ as zgetc2;
pub use zgetf2_ as zgetf2;
pub use zgetrf2_ as zgetrf2;
pub use zgetrf_ as zgetrf;
pub use zgetrf_batch_ as zgetrf_batch;
pub use zgetrf_batch_strided_ as zgetrf_batch_strided;
pub use zgetrfnp_batch_ as zgetrfnp_batch;
pub use zgetrfnp_batch_strided_ as zgetrfnp_batch_strided;
pub use zgetri_ as zgetri;
pub use zgetri_batch_strided_ as zgetri_batch_strided;
pub use zgetri_oop_batch_ as zgetri_oop_batch;
pub use zgetri_oop_batch_strided_ as zgetri_oop_batch_strided;
pub use zgetrs_ as zgetrs;
pub use zgetrs_batch_strided_ as zgetrs_batch_strided;
pub use zgetrsnp_batch_strided_ as zgetrsnp_batch_strided;
pub use zgetsls_ as zgetsls;
pub use zgetsqrhrt_ as zgetsqrhrt;
pub use zggbak_ as zggbak;
pub use zggbal_ as zggbal;
pub use zgges3_ as zgges3;
pub use zgges_ as zgges;
pub use zggesx_ as zggesx;
pub use zggev3_ as zggev3;
pub use zggev_ as zggev;
pub use zggevx_ as zggevx;
pub use zggglm_ as zggglm;
pub use zgghd3_ as zgghd3;
pub use zgghrd_ as zgghrd;
pub use zgglse_ as zgglse;
pub use zggqrf_ as zggqrf;
pub use zggrqf_ as zggrqf;
pub use zggsvd3_ as zggsvd3;
pub use zggsvd_ as zggsvd;
pub use zggsvp3_ as zggsvp3;
pub use zggsvp_ as zggsvp;
pub use zgsvj0_ as zgsvj0;
pub use zgsvj1_ as zgsvj1;
pub use zgtcon_ as zgtcon;
pub use zgtrfs_ as zgtrfs;
pub use zgtsv_ as zgtsv;
pub use zgtsvx_ as zgtsvx;
pub use zgttrf_ as zgttrf;
pub use zgttrs_ as zgttrs;
pub use zgtts2_ as zgtts2;
pub use zhb2st_kernels_ as zhb2st_kernels;
pub use zhbev_ as zhbev;
pub use zhbev_2stage_ as zhbev_2stage;
pub use zhbevd_ as zhbevd;
pub use zhbevd_2stage_ as zhbevd_2stage;
pub use zhbevx_ as zhbevx;
pub use zhbevx_2stage_ as zhbevx_2stage;
pub use zhbgst_ as zhbgst;
pub use zhbgv_ as zhbgv;
pub use zhbgvd_ as zhbgvd;
pub use zhbgvx_ as zhbgvx;
pub use zhbtrd_ as zhbtrd;
pub use zhecon_ as zhecon;
pub use zhecon_3_ as zhecon_3;
pub use zhecon_rook_ as zhecon_rook;
pub use zheequb_ as zheequb;
pub use zheev_ as zheev;
pub use zheev_2stage_ as zheev_2stage;
pub use zheevd_ as zheevd;
pub use zheevd_2stage_ as zheevd_2stage;
pub use zheevr_ as zheevr;
pub use zheevr_2stage_ as zheevr_2stage;
pub use zheevx_ as zheevx;
pub use zheevx_2stage_ as zheevx_2stage;
pub use zhegs2_ as zhegs2;
pub use zhegst_ as zhegst;
pub use zhegv_ as zhegv;
pub use zhegv_2stage_ as zhegv_2stage;
pub use zhegvd_ as zhegvd;
pub use zhegvx_ as zhegvx;
pub use zherdb_ as zherdb;
pub use zherfs_ as zherfs;
pub use zherfsx_ as zherfsx;
pub use zhesv_ as zhesv;
pub use zhesv_aa_ as zhesv_aa;
pub use zhesv_aa_2stage_ as zhesv_aa_2stage;
pub use zhesv_rk_ as zhesv_rk;
pub use zhesv_rook_ as zhesv_rook;
pub use zhesvx_ as zhesvx;
pub use zhesvxx_ as zhesvxx;
pub use zheswapr_ as zheswapr;
pub use zhetd2_ as zhetd2;
pub use zhetf2_ as zhetf2;
pub use zhetf2_rk_ as zhetf2_rk;
pub use zhetf2_rook_ as zhetf2_rook;
pub use zhetrd_ as zhetrd;
pub use zhetrd_2stage_ as zhetrd_2stage;
pub use zhetrd_hb2st_ as zhetrd_hb2st;
pub use zhetrd_he2hb_ as zhetrd_he2hb;
pub use zhetrf_ as zhetrf;
pub use zhetrf_aa_ as zhetrf_aa;
pub use zhetrf_aa_2stage_ as zhetrf_aa_2stage;
pub use zhetrf_rk_ as zhetrf_rk;
pub use zhetrf_rook_ as zhetrf_rook;
pub use zhetri2_ as zhetri2;
pub use zhetri2x_ as zhetri2x;
pub use zhetri_ as zhetri;
pub use zhetri_3_ as zhetri_3;
pub use zhetri_3x_ as zhetri_3x;
pub use zhetri_rook_ as zhetri_rook;
pub use zhetrs2_ as zhetrs2;
pub use zhetrs_ as zhetrs;
pub use zhetrs_3_ as zhetrs_3;
pub use zhetrs_aa_ as zhetrs_aa;
pub use zhetrs_aa_2stage_ as zhetrs_aa_2stage;
pub use zhetrs_rook_ as zhetrs_rook;
pub use zhfrk_ as zhfrk;
pub use zhgeqz_ as zhgeqz;
pub use zhpcon_ as zhpcon;
pub use zhpev_ as zhpev;
pub use zhpevd_ as zhpevd;
pub use zhpevx_ as zhpevx;
pub use zhpgst_ as zhpgst;
pub use zhpgv_ as zhpgv;
pub use zhpgvd_ as zhpgvd;
pub use zhpgvx_ as zhpgvx;
pub use zhprfs_ as zhprfs;
pub use zhpsv_ as zhpsv;
pub use zhpsvx_ as zhpsvx;
pub use zhptrd_ as zhptrd;
pub use zhptrf_ as zhptrf;
pub use zhptri_ as zhptri;
pub use zhptrs_ as zhptrs;
pub use zhsein_ as zhsein;
pub use zhseqr_ as zhseqr;
pub use zla_gbamv_ as zla_gbamv;
pub use zla_gbrcond_c_ as zla_gbrcond_c;
pub use zla_gbrcond_x_ as zla_gbrcond_x;
pub use zla_gbrfsx_extended_ as zla_gbrfsx_extended;
pub use zla_gbrpvgrw_ as zla_gbrpvgrw;
pub use zla_geamv_ as zla_geamv;
pub use zla_gercond_c_ as zla_gercond_c;
pub use zla_gercond_x_ as zla_gercond_x;
pub use zla_gerfsx_extended_ as zla_gerfsx_extended;
pub use zla_gerpvgrw_ as zla_gerpvgrw;
pub use zla_heamv_ as zla_heamv;
pub use zla_hercond_c_ as zla_hercond_c;
pub use zla_hercond_x_ as zla_hercond_x;
pub use zla_herfsx_extended_ as zla_herfsx_extended;
pub use zla_herpvgrw_ as zla_herpvgrw;
pub use zla_lin_berr_ as zla_lin_berr;
pub use zla_porcond_c_ as zla_porcond_c;
pub use zla_porcond_x_ as zla_porcond_x;
pub use zla_porfsx_extended_ as zla_porfsx_extended;
pub use zla_porpvgrw_ as zla_porpvgrw;
pub use zla_syamv_ as zla_syamv;
pub use zla_syrcond_c_ as zla_syrcond_c;
pub use zla_syrcond_x_ as zla_syrcond_x;
pub use zla_syrfsx_extended_ as zla_syrfsx_extended;
pub use zla_syrpvgrw_ as zla_syrpvgrw;
pub use zla_wwaddw_ as zla_wwaddw;
pub use zlabrd_ as zlabrd;
pub use zlacgv_ as zlacgv;
pub use zlacn2_ as zlacn2;
pub use zlacon_ as zlacon;
pub use zlacp2_ as zlacp2;
pub use zlacpy_ as zlacpy;
pub use zlacrm_ as zlacrm;
pub use zlacrt_ as zlacrt;
pub use zladiv_ as zladiv;
pub use zlaed0_ as zlaed0;
pub use zlaed7_ as zlaed7;
pub use zlaed8_ as zlaed8;
pub use zlaein_ as zlaein;
pub use zlaesy_ as zlaesy;
pub use zlaev2_ as zlaev2;
pub use zlag2c_ as zlag2c;
pub use zlagge_ as zlagge;
pub use zlaghe_ as zlaghe;
pub use zlags2_ as zlags2;
pub use zlagsy_ as zlagsy;
pub use zlagtm_ as zlagtm;
pub use zlahef_ as zlahef;
pub use zlahef_aa_ as zlahef_aa;
pub use zlahef_rk_ as zlahef_rk;
pub use zlahef_rook_ as zlahef_rook;
pub use zlahqr_ as zlahqr;
pub use zlahr2_ as zlahr2;
pub use zlahrd_ as zlahrd;
pub use zlaic1_ as zlaic1;
pub use zlakf2_ as zlakf2;
pub use zlals0_ as zlals0;
pub use zlalsa_ as zlalsa;
pub use zlalsd_ as zlalsd;
pub use zlamswlq_ as zlamswlq;
pub use zlamtsqr_ as zlamtsqr;
pub use zlangb_ as zlangb;
pub use zlange_ as zlange;
pub use zlangt_ as zlangt;
pub use zlanhb_ as zlanhb;
pub use zlanhe_ as zlanhe;
pub use zlanhf_ as zlanhf;
pub use zlanhp_ as zlanhp;
pub use zlanhs_ as zlanhs;
pub use zlanht_ as zlanht;
pub use zlansb_ as zlansb;
pub use zlansp_ as zlansp;
pub use zlansy_ as zlansy;
pub use zlantb_ as zlantb;
pub use zlantp_ as zlantp;
pub use zlantr_ as zlantr;
pub use zlapll_ as zlapll;
pub use zlapmr_ as zlapmr;
pub use zlapmt_ as zlapmt;
pub use zlaqgb_ as zlaqgb;
pub use zlaqge_ as zlaqge;
pub use zlaqhb_ as zlaqhb;
pub use zlaqhe_ as zlaqhe;
pub use zlaqhp_ as zlaqhp;
pub use zlaqp2_ as zlaqp2;
pub use zlaqps_ as zlaqps;
pub use zlaqr0_ as zlaqr0;
pub use zlaqr1_ as zlaqr1;
pub use zlaqr2_ as zlaqr2;
pub use zlaqr3_ as zlaqr3;
pub use zlaqr4_ as zlaqr4;
pub use zlaqr5_ as zlaqr5;
pub use zlaqsb_ as zlaqsb;
pub use zlaqsp_ as zlaqsp;
pub use zlaqsy_ as zlaqsy;
pub use zlaqz0_ as zlaqz0;
pub use zlaqz1_ as zlaqz1;
pub use zlaqz2_ as zlaqz2;
pub use zlaqz3_ as zlaqz3;
pub use zlar1v_ as zlar1v;
pub use zlar2v_ as zlar2v;
pub use zlarcm_ as zlarcm;
pub use zlarf1f_ as zlarf1f;
pub use zlarf1l_ as zlarf1l;
pub use zlarf_ as zlarf;
pub use zlarfb_ as zlarfb;
pub use zlarfb_gett_ as zlarfb_gett;
pub use zlarfg_ as zlarfg;
pub use zlarfgp_ as zlarfgp;
pub use zlarfp_ as zlarfp;
pub use zlarft_ as zlarft;
pub use zlarfx_ as zlarfx;
pub use zlarfy_ as zlarfy;
pub use zlarge_ as zlarge;
pub use zlargv_ as zlargv;
pub use zlarnd_ as zlarnd;
pub use zlarnv_ as zlarnv;
pub use zlaror_ as zlaror;
pub use zlarot_ as zlarot;
pub use zlarrv_ as zlarrv;
pub use zlarscl2_ as zlarscl2;
pub use zlartg_ as zlartg;
pub use zlartv_ as zlartv;
pub use zlarz_ as zlarz;
pub use zlarzb_ as zlarzb;
pub use zlarzt_ as zlarzt;
pub use zlascl2_ as zlascl2;
pub use zlascl_ as zlascl;
pub use zlaset_ as zlaset;
pub use zlasr_ as zlasr;
pub use zlassq_ as zlassq;
pub use zlaswlq_ as zlaswlq;
pub use zlaswp_ as zlaswp;
pub use zlasyf_ as zlasyf;
pub use zlasyf_aa_ as zlasyf_aa;
pub use zlasyf_rk_ as zlasyf_rk;
pub use zlasyf_rook_ as zlasyf_rook;
pub use zlat2c_ as zlat2c;
pub use zlatbs_ as zlatbs;
pub use zlatdf_ as zlatdf;
pub use zlatm1_ as zlatm1;
pub use zlatm2_ as zlatm2;
pub use zlatm3_ as zlatm3;
pub use zlatm5_ as zlatm5;
pub use zlatm6_ as zlatm6;
pub use zlatme_ as zlatme;
pub use zlatmr_ as zlatmr;
pub use zlatms_ as zlatms;
pub use zlatps_ as zlatps;
pub use zlatrd_ as zlatrd;
pub use zlatrs3_ as zlatrs3;
pub use zlatrs_ as zlatrs;
pub use zlatrz_ as zlatrz;
pub use zlatsqr_ as zlatsqr;
pub use zlatzm_ as zlatzm;
pub use zlaunhr_col_getrfnp2_ as zlaunhr_col_getrfnp2;
pub use zlaunhr_col_getrfnp_ as zlaunhr_col_getrfnp;
pub use zlauu2_ as zlauu2;
pub use zlauum_ as zlauum;
pub use zpbcon_ as zpbcon;
pub use zpbequ_ as zpbequ;
pub use zpbrfs_ as zpbrfs;
pub use zpbstf_ as zpbstf;
pub use zpbsv_ as zpbsv;
pub use zpbsvx_ as zpbsvx;
pub use zpbtf2_ as zpbtf2;
pub use zpbtrf_ as zpbtrf;
pub use zpbtrs_ as zpbtrs;
pub use zpftrf_ as zpftrf;
pub use zpftri_ as zpftri;
pub use zpftrs_ as zpftrs;
pub use zpocon_ as zpocon;
pub use zpoequ_ as zpoequ;
pub use zpoequb_ as zpoequb;
pub use zporfs_ as zporfs;
pub use zporfsx_ as zporfsx;
pub use zposv_ as zposv;
pub use zposvx_ as zposvx;
pub use zposvxx_ as zposvxx;
pub use zpotf2_ as zpotf2;
pub use zpotrf2_ as zpotrf2;
pub use zpotrf_ as zpotrf;
pub use zpotri_ as zpotri;
pub use zpotrs_ as zpotrs;
pub use zppcon_ as zppcon;
pub use zppequ_ as zppequ;
pub use zpprfs_ as zpprfs;
pub use zppsv_ as zppsv;
pub use zppsvx_ as zppsvx;
pub use zpptrf_ as zpptrf;
pub use zpptri_ as zpptri;
pub use zpptrs_ as zpptrs;
pub use zpstf2_ as zpstf2;
pub use zpstrf_ as zpstrf;
pub use zptcon_ as zptcon;
pub use zpteqr_ as zpteqr;
pub use zptrfs_ as zptrfs;
pub use zptsv_ as zptsv;
pub use zptsvx_ as zptsvx;
pub use zpttrf_ as zpttrf;
pub use zpttrs_ as zpttrs;
pub use zptts2_ as zptts2;
pub use zrot_ as zrot;
pub use zspcon_ as zspcon;
pub use zspmv_ as zspmv;
pub use zspr_ as zspr;
pub use zsprfs_ as zsprfs;
pub use zspsv_ as zspsv;
pub use zspsvx_ as zspsvx;
pub use zsptrf_ as zsptrf;
pub use zsptri_ as zsptri;
pub use zsptrs_ as zsptrs;
pub use zstedc_ as zstedc;
pub use zstegr_ as zstegr;
pub use zstein_ as zstein;
pub use zstemr_ as zstemr;
pub use zsteqr_ as zsteqr;
pub use zsycon_ as zsycon;
pub use zsycon_3_ as zsycon_3;
pub use zsycon_rook_ as zsycon_rook;
pub use zsyconv_ as zsyconv;
pub use zsyconvf_ as zsyconvf;
pub use zsyconvf_rook_ as zsyconvf_rook;
pub use zsyequb_ as zsyequb;
pub use zsymv_ as zsymv;
pub use zsyr_ as zsyr;
pub use zsyrfs_ as zsyrfs;
pub use zsyrfsx_ as zsyrfsx;
pub use zsysv_ as zsysv;
pub use zsysv_aa_ as zsysv_aa;
pub use zsysv_aa_2stage_ as zsysv_aa_2stage;
pub use zsysv_rk_ as zsysv_rk;
pub use zsysv_rook_ as zsysv_rook;
pub use zsysvx_ as zsysvx;
pub use zsysvxx_ as zsysvxx;
pub use zsyswapr_ as zsyswapr;
pub use zsytf2_ as zsytf2;
pub use zsytf2_rk_ as zsytf2_rk;
pub use zsytf2_rook_ as zsytf2_rook;
pub use zsytrf_ as zsytrf;
pub use zsytrf_aa_ as zsytrf_aa;
pub use zsytrf_aa_2stage_ as zsytrf_aa_2stage;
pub use zsytrf_rk_ as zsytrf_rk;
pub use zsytrf_rook_ as zsytrf_rook;
pub use zsytri2_ as zsytri2;
pub use zsytri2x_ as zsytri2x;
pub use zsytri_ as zsytri;
pub use zsytri_3_ as zsytri_3;
pub use zsytri_3x_ as zsytri_3x;
pub use zsytri_rook_ as zsytri_rook;
pub use zsytrs2_ as zsytrs2;
pub use zsytrs_ as zsytrs;
pub use zsytrs_3_ as zsytrs_3;
pub use zsytrs_aa_ as zsytrs_aa;
pub use zsytrs_aa_2stage_ as zsytrs_aa_2stage;
pub use zsytrs_rook_ as zsytrs_rook;
pub use ztbcon_ as ztbcon;
pub use ztbrfs_ as ztbrfs;
pub use ztbtrs_ as ztbtrs;
pub use ztfsm_ as ztfsm;
pub use ztftri_ as ztftri;
pub use ztfttp_ as ztfttp;
pub use ztfttr_ as ztfttr;
pub use ztgevc_ as ztgevc;
pub use ztgex2_ as ztgex2;
pub use ztgexc_ as ztgexc;
pub use ztgsen_ as ztgsen;
pub use ztgsja_ as ztgsja;
pub use ztgsna_ as ztgsna;
pub use ztgsy2_ as ztgsy2;
pub use ztgsyl_ as ztgsyl;
pub use ztpcon_ as ztpcon;
pub use ztplqt2_ as ztplqt2;
pub use ztplqt_ as ztplqt;
pub use ztpmlqt_ as ztpmlqt;
pub use ztpmqrt_ as ztpmqrt;
pub use ztpqrt2_ as ztpqrt2;
pub use ztpqrt_ as ztpqrt;
pub use ztprfb_ as ztprfb;
pub use ztprfs_ as ztprfs;
pub use ztptri_ as ztptri;
pub use ztptrs_ as ztptrs;
pub use ztpttf_ as ztpttf;
pub use ztpttr_ as ztpttr;
pub use ztrcon_ as ztrcon;
pub use ztrevc3_ as ztrevc3;
pub use ztrevc_ as ztrevc;
pub use ztrexc_ as ztrexc;
pub use ztrrfs_ as ztrrfs;
pub use ztrsen_ as ztrsen;
pub use ztrsna_ as ztrsna;
pub use ztrsyl3_ as ztrsyl3;
pub use ztrsyl_ as ztrsyl;
pub use ztrti2_ as ztrti2;
pub use ztrtri_ as ztrtri;
pub use ztrtrs_ as ztrtrs;
pub use ztrttf_ as ztrttf;
pub use ztrttp_ as ztrttp;
pub use ztzrqf_ as ztzrqf;
pub use ztzrzf_ as ztzrzf;
pub use zunbdb1_ as zunbdb1;
pub use zunbdb2_ as zunbdb2;
pub use zunbdb3_ as zunbdb3;
pub use zunbdb4_ as zunbdb4;
pub use zunbdb5_ as zunbdb5;
pub use zunbdb6_ as zunbdb6;
pub use zunbdb_ as zunbdb;
pub use zuncsd2by1_ as zuncsd2by1;
pub use zuncsd_ as zuncsd;
pub use zung2l_ as zung2l;
pub use zung2r_ as zung2r;
pub use zungbr_ as zungbr;
pub use zunghr_ as zunghr;
pub use zungl2_ as zungl2;
pub use zunglq_ as zunglq;
pub use zungql_ as zungql;
pub use zungqr_ as zungqr;
pub use zungr2_ as zungr2;
pub use zungrq_ as zungrq;
pub use zungtr_ as zungtr;
pub use zungtsqr_ as zungtsqr;
pub use zungtsqr_row_ as zungtsqr_row;
pub use zunhr_col_ as zunhr_col;
pub use zunm22_ as zunm22;
pub use zunm2l_ as zunm2l;
pub use zunm2r_ as zunm2r;
pub use zunmbr_ as zunmbr;
pub use zunmhr_ as zunmhr;
pub use zunml2_ as zunml2;
pub use zunmlq_ as zunmlq;
pub use zunmql_ as zunmql;
pub use zunmqr_ as zunmqr;
pub use zunmr2_ as zunmr2;
pub use zunmr3_ as zunmr3;
pub use zunmrq_ as zunmrq;
pub use zunmrz_ as zunmrz;
pub use zunmtr_ as zunmtr;
pub use zupgtr_ as zupgtr;
pub use zupmtr_ as zupmtr;

/* #endregion */
