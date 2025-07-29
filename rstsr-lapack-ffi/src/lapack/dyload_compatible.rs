//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from
//! current layer of module.
//!
//! This file is generated automatically.

use super::*;

pub unsafe fn lsame_(ca: *const c_char, cb: *const c_char) -> lapack_int {
    dyload_lib().lsame_.unwrap()(ca, cb)
}

pub unsafe fn cbbcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    theta: *mut f32,
    phi: *mut f32,
    U1: *mut __BindgenComplex<f32>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f32>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f32>,
    ldv1t: *const lapack_int,
    V2T: *mut __BindgenComplex<f32>,
    ldv2t: *const lapack_int,
    B11D: *mut f32,
    B11E: *mut f32,
    B12D: *mut f32,
    B12E: *mut f32,
    B21D: *mut f32,
    B21E: *mut f32,
    B22D: *mut f32,
    B22E: *mut f32,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cbbcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, m, p, q, theta, phi, U1, ldu1, U2, ldu2, V1T, ldv1t,
        V2T, ldv2t, B11D, B11E, B12D, B12E, B21D, B21E, B22D, B22E, rwork, lrwork, info,
    )
}

pub unsafe fn dbbcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    theta: *mut f64,
    phi: *mut f64,
    U1: *mut f64,
    ldu1: *const lapack_int,
    U2: *mut f64,
    ldu2: *const lapack_int,
    V1T: *mut f64,
    ldv1t: *const lapack_int,
    V2T: *mut f64,
    ldv2t: *const lapack_int,
    B11D: *mut f64,
    B11E: *mut f64,
    B12D: *mut f64,
    B12E: *mut f64,
    b21d: *mut f64,
    b21e: *mut f64,
    b22d: *mut f64,
    b22e: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dbbcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, m, p, q, theta, phi, U1, ldu1, U2, ldu2, V1T, ldv1t,
        V2T, ldv2t, B11D, B11E, B12D, B12E, b21d, b21e, b22d, b22e, work, lwork, info,
    )
}

pub unsafe fn sbbcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    theta: *mut f32,
    phi: *mut f32,
    U1: *mut f32,
    ldu1: *const lapack_int,
    U2: *mut f32,
    ldu2: *const lapack_int,
    V1T: *mut f32,
    ldv1t: *const lapack_int,
    V2T: *mut f32,
    ldv2t: *const lapack_int,
    B11D: *mut f32,
    B11E: *mut f32,
    B12D: *mut f32,
    B12E: *mut f32,
    B21D: *mut f32,
    B21E: *mut f32,
    B22D: *mut f32,
    B22E: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sbbcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, m, p, q, theta, phi, U1, ldu1, U2, ldu2, V1T, ldv1t,
        V2T, ldv2t, B11D, B11E, B12D, B12E, B21D, B21E, B22D, B22E, work, lwork, info,
    )
}

pub unsafe fn zbbcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    theta: *mut f64,
    phi: *mut f64,
    U1: *mut __BindgenComplex<f64>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f64>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f64>,
    ldv1t: *const lapack_int,
    V2T: *mut __BindgenComplex<f64>,
    ldv2t: *const lapack_int,
    B11D: *mut f64,
    B11E: *mut f64,
    B12D: *mut f64,
    B12E: *mut f64,
    B21D: *mut f64,
    B21E: *mut f64,
    B22D: *mut f64,
    B22E: *mut f64,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zbbcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, m, p, q, theta, phi, U1, ldu1, U2, ldu2, V1T, ldv1t,
        V2T, ldv2t, B11D, B11E, B12D, B12E, B21D, B21E, B22D, B22E, rwork, lrwork, info,
    )
}

pub unsafe fn dbdsdc_(
    uplo: *const c_char,
    compq: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    VT: *mut f64,
    ldvt: *const lapack_int,
    Q: *mut f64,
    IQ: *mut lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dbdsdc_.unwrap()(uplo, compq, n, D, E, U, ldu, VT, ldvt, Q, IQ, work, iwork, info)
}

pub unsafe fn sbdsdc_(
    uplo: *const c_char,
    compq: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    VT: *mut f32,
    ldvt: *const lapack_int,
    Q: *mut f32,
    IQ: *mut lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sbdsdc_.unwrap()(uplo, compq, n, D, E, U, ldu, VT, ldvt, Q, IQ, work, iwork, info)
}

pub unsafe fn cbdsqr_(
    uplo: *const c_char,
    n: *const lapack_int,
    ncvt: *const lapack_int,
    nru: *const lapack_int,
    ncc: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    VT: *mut __BindgenComplex<f32>,
    ldvt: *const lapack_int,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cbdsqr_.unwrap()(
        uplo, n, ncvt, nru, ncc, D, E, VT, ldvt, U, ldu, C, ldc, rwork, info,
    )
}

pub unsafe fn dbdsqr_(
    uplo: *const c_char,
    n: *const lapack_int,
    ncvt: *const lapack_int,
    nru: *const lapack_int,
    ncc: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    VT: *mut f64,
    ldvt: *const lapack_int,
    U: *mut f64,
    ldu: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dbdsqr_.unwrap()(
        uplo, n, ncvt, nru, ncc, D, E, VT, ldvt, U, ldu, C, ldc, work, info,
    )
}

pub unsafe fn sbdsqr_(
    uplo: *const c_char,
    n: *const lapack_int,
    ncvt: *const lapack_int,
    nru: *const lapack_int,
    ncc: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    VT: *mut f32,
    ldvt: *const lapack_int,
    U: *mut f32,
    ldu: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sbdsqr_.unwrap()(
        uplo, n, ncvt, nru, ncc, D, E, VT, ldvt, U, ldu, C, ldc, work, info,
    )
}

pub unsafe fn zbdsqr_(
    uplo: *const c_char,
    n: *const lapack_int,
    ncvt: *const lapack_int,
    nru: *const lapack_int,
    ncc: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    VT: *mut __BindgenComplex<f64>,
    ldvt: *const lapack_int,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zbdsqr_.unwrap()(
        uplo, n, ncvt, nru, ncc, D, E, VT, ldvt, U, ldu, C, ldc, rwork, info,
    )
}

pub unsafe fn dbdsvdx_(
    uplo: *const c_char,
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *const f64,
    E: *const f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dbdsvdx_.unwrap()(
        uplo, jobz, range, n, D, E, vl, vu, il, iu, ns, S, Z, ldz, work, iwork, info,
    )
}

pub unsafe fn sbdsvdx_(
    uplo: *const c_char,
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *const f32,
    E: *const f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sbdsvdx_.unwrap()(
        uplo, jobz, range, n, D, E, vl, vu, il, iu, ns, S, Z, ldz, work, iwork, info,
    )
}

pub unsafe fn ddisna_(
    job: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    D: *const f64,
    SEP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ddisna_.unwrap()(job, m, n, D, SEP, info)
}

pub unsafe fn sdisna_(
    job: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    D: *const f32,
    SEP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sdisna_.unwrap()(job, m, n, D, SEP, info)
}

pub unsafe fn cgbbrd_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ncc: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    PT: *mut __BindgenComplex<f32>,
    ldpt: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbbrd_.unwrap()(
        vect, m, n, ncc, kl, ku, AB, ldab, D, E, Q, ldq, PT, ldpt, C, ldc, work, rwork, info,
    )
}

pub unsafe fn dgbbrd_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ncc: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Q: *mut f64,
    ldq: *const lapack_int,
    PT: *mut f64,
    ldpt: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgbbrd_.unwrap()(
        vect, m, n, ncc, kl, ku, AB, ldab, D, E, Q, ldq, PT, ldpt, C, ldc, work, info,
    )
}

pub unsafe fn sgbbrd_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ncc: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Q: *mut f32,
    ldq: *const lapack_int,
    PT: *mut f32,
    ldpt: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgbbrd_.unwrap()(
        vect, m, n, ncc, kl, ku, AB, ldab, D, E, Q, ldq, PT, ldpt, C, ldc, work, info,
    )
}

pub unsafe fn zgbbrd_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ncc: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    PT: *mut __BindgenComplex<f64>,
    ldpt: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbbrd_.unwrap()(
        vect, m, n, ncc, kl, ku, AB, ldab, D, E, Q, ldq, PT, ldpt, C, ldc, work, rwork, info,
    )
}

pub unsafe fn cgbcon_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbcon_.unwrap()(norm, n, kl, ku, AB, ldab, ipiv, anorm, rcond, work, rwork, info)
}

pub unsafe fn dgbcon_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbcon_.unwrap()(norm, n, kl, ku, AB, ldab, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn sgbcon_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbcon_.unwrap()(norm, n, kl, ku, AB, ldab, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn zgbcon_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbcon_.unwrap()(norm, n, kl, ku, AB, ldab, ipiv, anorm, rcond, work, rwork, info)
}

pub unsafe fn cgbequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbequ_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn dgbequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgbequ_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn sgbequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgbequ_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn zgbequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbequ_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn cgbequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbequb_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn dgbequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgbequb_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn sgbequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgbequb_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn zgbequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbequb_.unwrap()(m, n, kl, ku, AB, ldab, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn cgbrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbrfs_.unwrap()(
        trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, B, ldb, X, ldx, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn dgbrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    AFB: *const f64,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbrfs_.unwrap()(
        trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, B, ldb, X, ldx, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn sgbrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    AFB: *const f32,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbrfs_.unwrap()(
        trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, B, ldb, X, ldx, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn zgbrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbrfs_.unwrap()(
        trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, B, ldb, X, ldx, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn cgbrfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f32,
    C: *const f32,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbrfsx_.unwrap()(
        trans,
        equed,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dgbrfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    AFB: *const f64,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f64,
    C: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbrfsx_.unwrap()(
        trans,
        equed,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sgbrfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    AFB: *const f32,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f32,
    C: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbrfsx_.unwrap()(
        trans,
        equed,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zgbrfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f64,
    C: *const f64,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbrfsx_.unwrap()(
        trans,
        equed,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cgbsv_(
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgbsv_.unwrap()(n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn dgbsv_(
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbsv_.unwrap()(n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn sgbsv_(
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbsv_.unwrap()(n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn zgbsv_(
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgbsv_.unwrap()(n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn cgbsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbsvx_.unwrap()(
        fact, trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, equed, R, C, B, ldb, X, ldx,
        rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dgbsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    AFB: *mut f64,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbsvx_.unwrap()(
        fact, trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, equed, R, C, B, ldb, X, ldx,
        rcond, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn sgbsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    AFB: *mut f32,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbsvx_.unwrap()(
        fact, trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, equed, R, C, B, ldb, X, ldx,
        rcond, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zgbsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbsvx_.unwrap()(
        fact, trans, n, kl, ku, nrhs, AB, ldab, AFB, ldafb, ipiv, equed, R, C, B, ldb, X, ldx,
        rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cgbsvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgbsvxx_.unwrap()(
        fact,
        trans,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dgbsvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    AFB: *mut f64,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbsvxx_.unwrap()(
        fact,
        trans,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sgbsvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    AFB: *mut f32,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbsvxx_.unwrap()(
        fact,
        trans,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zgbsvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgbsvxx_.unwrap()(
        fact,
        trans,
        n,
        kl,
        ku,
        nrhs,
        AB,
        ldab,
        AFB,
        ldafb,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cgbtrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgbtrf_.unwrap()(m, n, kl, ku, AB, ldab, ipiv, info)
}

pub unsafe fn dgbtrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbtrf_.unwrap()(m, n, kl, ku, AB, ldab, ipiv, info)
}

pub unsafe fn sgbtrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbtrf_.unwrap()(m, n, kl, ku, AB, ldab, ipiv, info)
}

pub unsafe fn zgbtrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgbtrf_.unwrap()(m, n, kl, ku, AB, ldab, ipiv, info)
}

pub unsafe fn cgbtrs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgbtrs_.unwrap()(trans, n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn dgbtrs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgbtrs_.unwrap()(trans, n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn sgbtrs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgbtrs_.unwrap()(trans, n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn zgbtrs_(
    trans: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgbtrs_.unwrap()(trans, n, kl, ku, nrhs, AB, ldab, ipiv, B, ldb, info)
}

pub unsafe fn cgebak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    scale: *const f32,
    m: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgebak_.unwrap()(job, side, n, ilo, ihi, scale, m, V, ldv, info)
}

pub unsafe fn dgebak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    scale: *const f64,
    m: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgebak_.unwrap()(job, side, n, ilo, ihi, scale, m, V, ldv, info)
}

pub unsafe fn sgebak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    scale: *const f32,
    m: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgebak_.unwrap()(job, side, n, ilo, ihi, scale, m, V, ldv, info)
}

pub unsafe fn zgebak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    scale: *const f64,
    m: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgebak_.unwrap()(job, side, n, ilo, ihi, scale, m, V, ldv, info)
}

pub unsafe fn cgebal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgebal_.unwrap()(job, n, A, lda, ilo, ihi, scale, info)
}

pub unsafe fn dgebal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgebal_.unwrap()(job, n, A, lda, ilo, ihi, scale, info)
}

pub unsafe fn sgebal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgebal_.unwrap()(job, n, A, lda, ilo, ihi, scale, info)
}

pub unsafe fn zgebal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgebal_.unwrap()(job, n, A, lda, ilo, ihi, scale, info)
}

pub unsafe fn cgebrd_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tauq: *mut __BindgenComplex<f32>,
    taup: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgebrd_.unwrap()(m, n, A, lda, D, E, tauq, taup, work, lwork, info)
}

pub unsafe fn dgebrd_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tauq: *mut f64,
    taup: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgebrd_.unwrap()(m, n, A, lda, D, E, tauq, taup, work, lwork, info)
}

pub unsafe fn sgebrd_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tauq: *mut f32,
    taup: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgebrd_.unwrap()(m, n, A, lda, D, E, tauq, taup, work, lwork, info)
}

pub unsafe fn zgebrd_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tauq: *mut __BindgenComplex<f64>,
    taup: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgebrd_.unwrap()(m, n, A, lda, D, E, tauq, taup, work, lwork, info)
}

pub unsafe fn cgecon_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgecon_.unwrap()(norm, n, A, lda, anorm, rcond, work, rwork, info)
}

pub unsafe fn dgecon_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgecon_.unwrap()(norm, n, A, lda, anorm, rcond, work, iwork, info)
}

pub unsafe fn sgecon_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgecon_.unwrap()(norm, n, A, lda, anorm, rcond, work, iwork, info)
}

pub unsafe fn zgecon_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgecon_.unwrap()(norm, n, A, lda, anorm, rcond, work, rwork, info)
}

pub unsafe fn cgeequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeequ_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn dgeequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeequ_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn sgeequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeequ_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn zgeequ_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeequ_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn cgeequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeequb_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn dgeequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeequb_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn sgeequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    R: *mut f32,
    C: *mut f32,
    rowcnd: *mut f32,
    colcnd: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeequb_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn zgeequb_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    R: *mut f64,
    C: *mut f64,
    rowcnd: *mut f64,
    colcnd: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeequb_.unwrap()(m, n, A, lda, R, C, rowcnd, colcnd, amax, info)
}

pub unsafe fn cgees_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_C_SELECT1,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    W: *mut __BindgenComplex<f32>,
    VS: *mut __BindgenComplex<f32>,
    ldvs: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgees_.unwrap()(
        jobvs, sort, select, n, A, lda, sdim, W, VS, ldvs, work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn dgees_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_D_SELECT2,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    VS: *mut f64,
    ldvs: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgees_.unwrap()(
        jobvs, sort, select, n, A, lda, sdim, WR, WI, VS, ldvs, work, lwork, BWORK, info,
    )
}

pub unsafe fn sgees_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_S_SELECT2,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    VS: *mut f32,
    ldvs: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgees_.unwrap()(
        jobvs, sort, select, n, A, lda, sdim, WR, WI, VS, ldvs, work, lwork, BWORK, info,
    )
}

pub unsafe fn zgees_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_Z_SELECT1,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    W: *mut __BindgenComplex<f64>,
    VS: *mut __BindgenComplex<f64>,
    ldvs: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgees_.unwrap()(
        jobvs, sort, select, n, A, lda, sdim, W, VS, ldvs, work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn cgeesx_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_C_SELECT1,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    W: *mut __BindgenComplex<f32>,
    VS: *mut __BindgenComplex<f32>,
    ldvs: *const lapack_int,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeesx_.unwrap()(
        jobvs, sort, select, sense, n, A, lda, sdim, W, VS, ldvs, rconde, rcondv, work, lwork,
        rwork, BWORK, info,
    )
}

pub unsafe fn dgeesx_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_D_SELECT2,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    VS: *mut f64,
    ldvs: *const lapack_int,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeesx_.unwrap()(
        jobvs, sort, select, sense, n, A, lda, sdim, WR, WI, VS, ldvs, rconde, rcondv, work, lwork,
        iwork, liwork, BWORK, info,
    )
}

pub unsafe fn sgeesx_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_S_SELECT2,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    VS: *mut f32,
    ldvs: *const lapack_int,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeesx_.unwrap()(
        jobvs, sort, select, sense, n, A, lda, sdim, WR, WI, VS, ldvs, rconde, rcondv, work, lwork,
        iwork, liwork, BWORK, info,
    )
}

pub unsafe fn zgeesx_(
    jobvs: *const c_char,
    sort: *const c_char,
    select: LAPACK_Z_SELECT1,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    sdim: *mut lapack_int,
    W: *mut __BindgenComplex<f64>,
    VS: *mut __BindgenComplex<f64>,
    ldvs: *const lapack_int,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeesx_.unwrap()(
        jobvs, sort, select, sense, n, A, lda, sdim, W, VS, ldvs, rconde, rcondv, work, lwork,
        rwork, BWORK, info,
    )
}

pub unsafe fn cgeev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeev_.unwrap()(
        jobvl, jobvr, n, A, lda, W, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn dgeev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeev_.unwrap()(
        jobvl, jobvr, n, A, lda, WR, WI, VL, ldvl, VR, ldvr, work, lwork, info,
    )
}

pub unsafe fn sgeev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeev_.unwrap()(
        jobvl, jobvr, n, A, lda, WR, WI, VL, ldvl, VR, ldvr, work, lwork, info,
    )
}

pub unsafe fn zgeev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeev_.unwrap()(
        jobvl, jobvr, n, A, lda, W, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn cgeevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f32,
    abnrm: *mut f32,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, W, VL, ldvl, VR, ldvr, ilo, ihi, scale, abnrm,
        rconde, rcondv, work, lwork, rwork, info,
    )
}

pub unsafe fn dgeevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f64,
    abnrm: *mut f64,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, WR, WI, VL, ldvl, VR, ldvr, ilo, ihi, scale, abnrm,
        rconde, rcondv, work, lwork, iwork, info,
    )
}

pub unsafe fn sgeevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f32,
    abnrm: *mut f32,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, WR, WI, VL, ldvl, VR, ldvr, ilo, ihi, scale, abnrm,
        rconde, rcondv, work, lwork, iwork, info,
    )
}

pub unsafe fn zgeevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    scale: *mut f64,
    abnrm: *mut f64,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, W, VL, ldvl, VR, ldvr, ilo, ihi, scale, abnrm,
        rconde, rcondv, work, lwork, rwork, info,
    )
}

pub unsafe fn cgehrd_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgehrd_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgehrd_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgehrd_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgehrd_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgehrd_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgehrd_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgehrd_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn cgejsv_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    jobr: *const c_char,
    jobt: *const c_char,
    jobp: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    SVA: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    cwork: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgejsv_.unwrap()(
        joba, jobu, jobv, jobr, jobt, jobp, m, n, A, lda, SVA, U, ldu, V, ldv, cwork, lwork, rwork,
        lrwork, iwork, info,
    )
}

pub unsafe fn dgejsv_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    jobr: *const c_char,
    jobt: *const c_char,
    jobp: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    SVA: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgejsv_.unwrap()(
        joba, jobu, jobv, jobr, jobt, jobp, m, n, A, lda, SVA, U, ldu, V, ldv, work, lwork, iwork,
        info,
    )
}

pub unsafe fn sgejsv_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    jobr: *const c_char,
    jobt: *const c_char,
    jobp: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    SVA: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgejsv_.unwrap()(
        joba, jobu, jobv, jobr, jobt, jobp, m, n, A, lda, SVA, U, ldu, V, ldv, work, lwork, iwork,
        info,
    )
}

pub unsafe fn zgejsv_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    jobr: *const c_char,
    jobt: *const c_char,
    jobp: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    SVA: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    cwork: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgejsv_.unwrap()(
        joba, jobu, jobv, jobr, jobt, jobp, m, n, A, lda, SVA, U, ldu, V, ldv, cwork, lwork, rwork,
        lrwork, iwork, info,
    )
}

pub unsafe fn cgelq_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    tsize: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgelq_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn dgelq_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    tsize: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgelq_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn sgelq_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    tsize: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgelq_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn zgelq_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    tsize: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgelq_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn cgelq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgelq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn dgelq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgelq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn sgelq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgelq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn zgelq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgelq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn cgelqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgelqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgelqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgelqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgelqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgelqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgelqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgelqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cgels_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgels_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn dgels_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgels_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn sgels_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgels_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn zgels_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgels_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn cgelsd_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    S: *mut f32,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgelsd_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, rwork, iwork, info,
    )
}

pub unsafe fn dgelsd_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    S: *mut f64,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgelsd_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, iwork, info,
    )
}

pub unsafe fn sgelsd_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    S: *mut f32,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgelsd_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, iwork, info,
    )
}

pub unsafe fn zgelsd_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    S: *mut f64,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgelsd_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, rwork, iwork, info,
    )
}

pub unsafe fn cgelss_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    S: *mut f32,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgelss_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, rwork, info,
    )
}

pub unsafe fn dgelss_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    S: *mut f64,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgelss_.unwrap()(m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, info)
}

pub unsafe fn sgelss_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    S: *mut f32,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgelss_.unwrap()(m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, info)
}

pub unsafe fn zgelss_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    S: *mut f64,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgelss_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, S, rcond, rank, work, lwork, rwork, info,
    )
}

pub unsafe fn cgelsy_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    JPVT: *mut lapack_int,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgelsy_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, JPVT, rcond, rank, work, lwork, rwork, info,
    )
}

pub unsafe fn dgelsy_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    JPVT: *mut lapack_int,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgelsy_.unwrap()(m, n, nrhs, A, lda, B, ldb, JPVT, rcond, rank, work, lwork, info)
}

pub unsafe fn sgelsy_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    JPVT: *mut lapack_int,
    rcond: *const f32,
    rank: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgelsy_.unwrap()(m, n, nrhs, A, lda, B, ldb, JPVT, rcond, rank, work, lwork, info)
}

pub unsafe fn zgelsy_(
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    JPVT: *mut lapack_int,
    rcond: *const f64,
    rank: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgelsy_.unwrap()(
        m, n, nrhs, A, lda, B, ldb, JPVT, rcond, rank, work, lwork, rwork, info,
    )
}

pub unsafe fn cgemlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    tsize: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgemlq_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn dgemlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    T: *const f64,
    tsize: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgemlq_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn sgemlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    T: *const f32,
    tsize: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgemlq_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn zgemlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    tsize: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgemlq_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn cgemqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    tsize: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgemqr_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn dgemqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    T: *const f64,
    tsize: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgemqr_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn sgemqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    T: *const f32,
    tsize: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgemqr_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn zgemqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    tsize: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgemqr_.unwrap()(side, trans, m, n, k, A, lda, T, tsize, C, ldc, work, lwork, info)
}

pub unsafe fn cgemqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    nb: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgemqrt_.unwrap()(side, trans, m, n, k, nb, V, ldv, T, ldt, C, ldc, work, info)
}

pub unsafe fn dgemqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    nb: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgemqrt_.unwrap()(side, trans, m, n, k, nb, V, ldv, T, ldt, C, ldc, work, info)
}

pub unsafe fn sgemqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    nb: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgemqrt_.unwrap()(side, trans, m, n, k, nb, V, ldv, T, ldt, C, ldc, work, info)
}

pub unsafe fn zgemqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    nb: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgemqrt_.unwrap()(side, trans, m, n, k, nb, V, ldv, T, ldt, C, ldc, work, info)
}

pub unsafe fn cgeql2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgeql2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn dgeql2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeql2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn sgeql2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeql2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn zgeql2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgeql2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn cgeqlf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqlf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgeqlf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqlf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgeqlf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqlf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgeqlf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqlf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgeqpf_(
    m: *mut lapack_int,
    n: *mut lapack_int,
    a: *mut f32,
    lda: *mut lapack_int,
    jpvt: *mut lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqpf_.unwrap()(m, n, a, lda, jpvt, tau, work, info)
}

pub unsafe fn dgeqpf_(
    m: *mut lapack_int,
    n: *mut lapack_int,
    a: *mut f64,
    lda: *mut lapack_int,
    jpvt: *mut lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqpf_.unwrap()(m, n, a, lda, jpvt, tau, work, info)
}

pub unsafe fn cgeqpf_(
    m: *mut lapack_int,
    n: *mut lapack_int,
    a: *mut __BindgenComplex<f32>,
    lda: *mut lapack_int,
    jpvt: *mut lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqpf_.unwrap()(m, n, a, lda, jpvt, tau, work, rwork, info)
}

pub unsafe fn zgeqpf_(
    m: *mut lapack_int,
    n: *mut lapack_int,
    a: *mut __BindgenComplex<f64>,
    lda: *mut lapack_int,
    jpvt: *mut lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqpf_.unwrap()(m, n, a, lda, jpvt, tau, work, rwork, info)
}

pub unsafe fn cgeqp3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    JPVT: *mut lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqp3_.unwrap()(m, n, A, lda, JPVT, tau, work, lwork, rwork, info)
}

pub unsafe fn dgeqp3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    JPVT: *mut lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqp3_.unwrap()(m, n, A, lda, JPVT, tau, work, lwork, info)
}

pub unsafe fn sgeqp3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    JPVT: *mut lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqp3_.unwrap()(m, n, A, lda, JPVT, tau, work, lwork, info)
}

pub unsafe fn zgeqp3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    JPVT: *mut lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqp3_.unwrap()(m, n, A, lda, JPVT, tau, work, lwork, rwork, info)
}

pub unsafe fn cgeqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    tsize: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqr_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn dgeqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    tsize: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqr_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn sgeqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    tsize: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqr_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn zgeqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    tsize: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqr_.unwrap()(m, n, A, lda, T, tsize, work, lwork, info)
}

pub unsafe fn cgeqr2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqr2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn dgeqr2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqr2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn sgeqr2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqr2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn zgeqr2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqr2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn cgeqrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqrf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgeqrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqrf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgeqrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqrf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgeqrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqrf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cgeqrfp_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqrfp_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgeqrfp_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqrfp_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgeqrfp_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqrfp_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgeqrfp_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqrfp_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cgeqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqrt_.unwrap()(m, n, nb, A, lda, T, ldt, work, info)
}

pub unsafe fn dgeqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqrt_.unwrap()(m, n, nb, A, lda, T, ldt, work, info)
}

pub unsafe fn sgeqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqrt_.unwrap()(m, n, nb, A, lda, T, ldt, work, info)
}

pub unsafe fn zgeqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqrt_.unwrap()(m, n, nb, A, lda, T, ldt, work, info)
}

pub unsafe fn cgeqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqrt2_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn dgeqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqrt2_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn sgeqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqrt2_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn zgeqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqrt2_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn cgeqrt3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgeqrt3_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn dgeqrt3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgeqrt3_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn sgeqrt3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgeqrt3_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn zgeqrt3_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgeqrt3_.unwrap()(m, n, A, lda, T, ldt, info)
}

pub unsafe fn cgerfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgerfs_.unwrap()(
        trans, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dgerfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgerfs_.unwrap()(
        trans, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn sgerfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgerfs_.unwrap()(
        trans, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zgerfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgerfs_.unwrap()(
        trans, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cgerfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f32,
    C: *const f32,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgerfsx_.unwrap()(
        trans,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dgerfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f64,
    C: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgerfsx_.unwrap()(
        trans,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sgerfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f32,
    C: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgerfsx_.unwrap()(
        trans,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zgerfsx_(
    trans: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    R: *const f64,
    C: *const f64,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgerfsx_.unwrap()(
        trans,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cgerq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgerq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn dgerq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dgerq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn sgerq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sgerq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn zgerq2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgerq2_.unwrap()(m, n, A, lda, tau, work, info)
}

pub unsafe fn cgerqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgerqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dgerqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgerqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sgerqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgerqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zgerqf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgerqf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cgesdd_(
    jobz: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f32>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgesdd_.unwrap()(
        jobz, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, iwork, info,
    )
}

pub unsafe fn dgesdd_(
    jobz: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    VT: *mut f64,
    ldvt: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesdd_.unwrap()(jobz, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, iwork, info)
}

pub unsafe fn sgesdd_(
    jobz: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    VT: *mut f32,
    ldvt: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesdd_.unwrap()(jobz, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, iwork, info)
}

pub unsafe fn zgesdd_(
    jobz: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f64>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgesdd_.unwrap()(
        jobz, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, iwork, info,
    )
}

pub unsafe fn cgedmd_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    x: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    y: *mut __BindgenComplex<f32>,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f32,
    k: *mut lapack_int,
    eigs: *mut __BindgenComplex<f32>,
    z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    res: *mut f32,
    b: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    w: *mut __BindgenComplex<f32>,
    ldw: *const lapack_int,
    s: *mut __BindgenComplex<f32>,
    lds: *const lapack_int,
    zwork: *mut __BindgenComplex<f32>,
    lzwork: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgedmd_.unwrap()(
        jobs, jobz, jobr, jobf, whtsvd, m, n, x, ldx, y, ldy, nrnk, tol, k, eigs, z, ldz, res, b,
        ldb, w, ldw, s, lds, zwork, lzwork, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dgedmd_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    x: *mut f64,
    ldx: *const lapack_int,
    y: *mut f64,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f64,
    k: *mut lapack_int,
    reig: *mut f64,
    imeig: *mut f64,
    z: *mut f64,
    ldz: *const lapack_int,
    res: *mut f64,
    b: *mut f64,
    ldb: *const lapack_int,
    w: *mut f64,
    ldw: *const lapack_int,
    s: *mut f64,
    lds: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgedmd_.unwrap()(
        jobs, jobz, jobr, jobf, whtsvd, m, n, x, ldx, y, ldy, nrnk, tol, k, reig, imeig, z, ldz,
        res, b, ldb, w, ldw, s, lds, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn sgedmd_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    x: *mut f32,
    ldx: *const lapack_int,
    y: *mut f32,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f32,
    k: *mut lapack_int,
    reig: *mut f32,
    imeig: *mut f32,
    z: *mut f32,
    ldz: *const lapack_int,
    res: *mut f32,
    b: *mut f32,
    ldb: *const lapack_int,
    w: *mut f32,
    ldw: *const lapack_int,
    s: *mut f32,
    lds: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgedmd_.unwrap()(
        jobs, jobz, jobr, jobf, whtsvd, m, n, x, ldx, y, ldy, nrnk, tol, k, reig, imeig, z, ldz,
        res, b, ldb, w, ldw, s, lds, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn zgedmd_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    x: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    y: *mut __BindgenComplex<f64>,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f64,
    k: *mut lapack_int,
    eigs: *mut __BindgenComplex<f64>,
    z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    res: *mut f64,
    b: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    w: *mut __BindgenComplex<f64>,
    ldw: *const lapack_int,
    s: *mut __BindgenComplex<f64>,
    lds: *const lapack_int,
    zwork: *mut __BindgenComplex<f64>,
    lzwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgedmd_.unwrap()(
        jobs, jobz, jobr, jobf, whtsvd, m, n, x, ldx, y, ldy, nrnk, tol, k, eigs, z, ldz, res, b,
        ldb, w, ldw, s, lds, zwork, lzwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cgedmdq_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobq: *const c_char,
    jobt: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    f: *mut __BindgenComplex<f32>,
    ldf: *const lapack_int,
    x: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    y: *mut __BindgenComplex<f32>,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f32,
    k: *const lapack_int,
    eigs: *mut __BindgenComplex<f32>,
    z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    res: *mut f32,
    b: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    v: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    s: *mut __BindgenComplex<f32>,
    lds: *const lapack_int,
    zwork: *mut __BindgenComplex<f32>,
    lzwork: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgedmdq_.unwrap()(
        jobs, jobz, jobr, jobq, jobt, jobf, whtsvd, m, n, f, ldf, x, ldx, y, ldy, nrnk, tol, k,
        eigs, z, ldz, res, b, ldb, v, ldv, s, lds, zwork, lzwork, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dgedmdq_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobq: *const c_char,
    jobt: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    f: *mut f64,
    ldf: *const lapack_int,
    x: *mut f64,
    ldx: *const lapack_int,
    y: *mut f64,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f64,
    k: *mut lapack_int,
    reig: *mut f64,
    imeig: *mut f64,
    z: *mut f64,
    ldz: *const lapack_int,
    res: *mut f64,
    b: *mut f64,
    ldb: *const lapack_int,
    v: *mut f64,
    ldv: *const lapack_int,
    s: *mut f64,
    lds: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgedmdq_.unwrap()(
        jobs, jobz, jobr, jobq, jobt, jobf, whtsvd, m, n, f, ldf, x, ldx, y, ldy, nrnk, tol, k,
        reig, imeig, z, ldz, res, b, ldb, v, ldv, s, lds, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn sgedmdq_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobq: *const c_char,
    jobt: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    f: *mut f32,
    ldf: *const lapack_int,
    x: *mut f32,
    ldx: *const lapack_int,
    y: *mut f32,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f32,
    k: *const lapack_int,
    reig: *mut f32,
    imeig: *mut f32,
    z: *mut f32,
    ldz: *const lapack_int,
    res: *mut f32,
    b: *mut f32,
    ldb: *const lapack_int,
    v: *mut f32,
    ldv: *const lapack_int,
    s: *mut f32,
    lds: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgedmdq_.unwrap()(
        jobs, jobz, jobr, jobq, jobt, jobf, whtsvd, m, n, f, ldf, x, ldx, y, ldy, nrnk, tol, k,
        reig, imeig, z, ldz, res, b, ldb, v, ldv, s, lds, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn zgedmdq_(
    jobs: *const c_char,
    jobz: *const c_char,
    jobr: *const c_char,
    jobq: *const c_char,
    jobt: *const c_char,
    jobf: *const c_char,
    whtsvd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    f: *mut __BindgenComplex<f64>,
    ldf: *const lapack_int,
    x: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    y: *mut __BindgenComplex<f64>,
    ldy: *const lapack_int,
    nrnk: *const lapack_int,
    tol: *const f64,
    k: *const lapack_int,
    eigs: *mut __BindgenComplex<f64>,
    z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    res: *mut f64,
    b: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    v: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    s: *mut __BindgenComplex<f64>,
    lds: *const lapack_int,
    zwork: *mut __BindgenComplex<f64>,
    lzwork: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgedmdq_.unwrap()(
        jobs, jobz, jobr, jobq, jobt, jobf, whtsvd, m, n, f, ldf, x, ldx, y, ldy, nrnk, tol, k,
        eigs, z, ldz, res, b, ldb, v, ldv, s, lds, zwork, lzwork, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn cgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgesv_.unwrap()(n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn dgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesv_.unwrap()(n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn sgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesv_.unwrap()(n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgesv_.unwrap()(n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn dsgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    work: *mut f64,
    swork: *mut f32,
    iter: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsgesv_.unwrap()(n, nrhs, A, lda, ipiv, B, ldb, X, ldx, work, swork, iter, info)
}

pub unsafe fn zcgesv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    swork: *mut __BindgenComplex<f32>,
    rwork: *mut f64,
    iter: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zcgesv_.unwrap()(
        n, nrhs, A, lda, ipiv, B, ldb, X, ldx, work, swork, rwork, iter, info,
    )
}

pub unsafe fn cgesvd_(
    jobu: *const c_char,
    jobvt: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f32>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvd_.unwrap()(
        jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info,
    )
}

pub unsafe fn dgesvd_(
    jobu: *const c_char,
    jobvt: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    VT: *mut f64,
    ldvt: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvd_.unwrap()(jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, info)
}

pub unsafe fn sgesvd_(
    jobu: *const c_char,
    jobvt: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    VT: *mut f32,
    ldvt: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvd_.unwrap()(jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, info)
}

pub unsafe fn zgesvd_(
    jobu: *const c_char,
    jobvt: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f64>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvd_.unwrap()(
        jobu, jobvt, m, n, A, lda, S, U, ldu, VT, ldvt, work, lwork, rwork, info,
    )
}

pub unsafe fn cgesvdq_(
    joba: *const c_char,
    jobp: *const c_char,
    jobr: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    numrank: *mut lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    cwork: *mut __BindgenComplex<f32>,
    lcwork: *mut lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvdq_.unwrap()(
        joba, jobp, jobr, jobu, jobv, m, n, A, lda, S, U, ldu, V, ldv, numrank, iwork, liwork,
        cwork, lcwork, rwork, lrwork, info,
    )
}

pub unsafe fn dgesvdq_(
    joba: *const c_char,
    jobp: *const c_char,
    jobr: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    numrank: *mut lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    work: *mut f64,
    lwork: *mut lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvdq_.unwrap()(
        joba, jobp, jobr, jobu, jobv, m, n, A, lda, S, U, ldu, V, ldv, numrank, iwork, liwork,
        work, lwork, rwork, lrwork, info,
    )
}

pub unsafe fn sgesvdq_(
    joba: *const c_char,
    jobp: *const c_char,
    jobr: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    S: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    numrank: *mut lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    work: *mut f32,
    lwork: *mut lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvdq_.unwrap()(
        joba, jobp, jobr, jobu, jobv, m, n, A, lda, S, U, ldu, V, ldv, numrank, iwork, liwork,
        work, lwork, rwork, lrwork, info,
    )
}

pub unsafe fn zgesvdq_(
    joba: *const c_char,
    jobp: *const c_char,
    jobr: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    numrank: *mut lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    cwork: *mut __BindgenComplex<f64>,
    lcwork: *mut lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvdq_.unwrap()(
        joba, jobp, jobr, jobu, jobv, m, n, A, lda, S, U, ldu, V, ldv, numrank, iwork, liwork,
        cwork, lcwork, rwork, lrwork, info,
    )
}

pub unsafe fn cgesvdx_(
    jobu: *const c_char,
    jobvt: *const c_char,
    range: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f32>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvdx_.unwrap()(
        jobu, jobvt, range, m, n, A, lda, vl, vu, il, iu, ns, S, U, ldu, VT, ldvt, work, lwork,
        rwork, iwork, info,
    )
}

pub unsafe fn dgesvdx_(
    jobu: *const c_char,
    jobvt: *const c_char,
    range: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    VT: *mut f64,
    ldvt: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvdx_.unwrap()(
        jobu, jobvt, range, m, n, A, lda, vl, vu, il, iu, ns, S, U, ldu, VT, ldvt, work, lwork,
        iwork, info,
    )
}

pub unsafe fn sgesvdx_(
    jobu: *const c_char,
    jobvt: *const c_char,
    range: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    VT: *mut f32,
    ldvt: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvdx_.unwrap()(
        jobu, jobvt, range, m, n, A, lda, vl, vu, il, iu, ns, S, U, ldu, VT, ldvt, work, lwork,
        iwork, info,
    )
}

pub unsafe fn zgesvdx_(
    jobu: *const c_char,
    jobvt: *const c_char,
    range: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    ns: *mut lapack_int,
    S: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    VT: *mut __BindgenComplex<f64>,
    ldvt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvdx_.unwrap()(
        jobu, jobvt, range, m, n, A, lda, vl, vu, il, iu, ns, S, U, ldu, VT, ldvt, work, lwork,
        rwork, iwork, info,
    )
}

pub unsafe fn cgesvj_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    SVA: *mut f32,
    mv: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    cwork: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvj_.unwrap()(
        joba, jobu, jobv, m, n, A, lda, SVA, mv, V, ldv, cwork, lwork, rwork, lrwork, info,
    )
}

pub unsafe fn dgesvj_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    SVA: *mut f64,
    mv: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvj_.unwrap()(
        joba, jobu, jobv, m, n, A, lda, SVA, mv, V, ldv, work, lwork, info,
    )
}

pub unsafe fn sgesvj_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    SVA: *mut f32,
    mv: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvj_.unwrap()(
        joba, jobu, jobv, m, n, A, lda, SVA, mv, V, ldv, work, lwork, info,
    )
}

pub unsafe fn zgesvj_(
    joba: *const c_char,
    jobu: *const c_char,
    jobv: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    SVA: *mut f64,
    mv: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    cwork: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvj_.unwrap()(
        joba, jobu, jobv, m, n, A, lda, SVA, mv, V, ldv, cwork, lwork, rwork, lrwork, info,
    )
}

pub unsafe fn cgesvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvx_.unwrap()(
        fact, trans, n, nrhs, A, lda, AF, ldaf, ipiv, equed, R, C, B, ldb, X, ldx, rcond, ferr,
        berr, work, rwork, info,
    )
}

pub unsafe fn dgesvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvx_.unwrap()(
        fact, trans, n, nrhs, A, lda, AF, ldaf, ipiv, equed, R, C, B, ldb, X, ldx, rcond, ferr,
        berr, work, iwork, info,
    )
}

pub unsafe fn sgesvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvx_.unwrap()(
        fact, trans, n, nrhs, A, lda, AF, ldaf, ipiv, equed, R, C, B, ldb, X, ldx, rcond, ferr,
        berr, work, iwork, info,
    )
}

pub unsafe fn zgesvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvx_.unwrap()(
        fact, trans, n, nrhs, A, lda, AF, ldaf, ipiv, equed, R, C, B, ldb, X, ldx, rcond, ferr,
        berr, work, rwork, info,
    )
}

pub unsafe fn cgesvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgesvxx_.unwrap()(
        fact,
        trans,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dgesvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgesvxx_.unwrap()(
        fact,
        trans,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sgesvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f32,
    C: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgesvxx_.unwrap()(
        fact,
        trans,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zgesvxx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    R: *mut f64,
    C: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgesvxx_.unwrap()(
        fact,
        trans,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        R,
        C,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cgetf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn dgetf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn sgetf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn zgetf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn cgetrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetrf_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn dgetrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetrf_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn sgetrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetrf_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn zgetrf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetrf_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn cgetrf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetrf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn dgetrf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetrf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn sgetrf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetrf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn zgetrf2_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetrf2_.unwrap()(m, n, A, lda, ipiv, info)
}

pub unsafe fn cgetri_(
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetri_.unwrap()(n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn dgetri_(
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetri_.unwrap()(n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn sgetri_(
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetri_.unwrap()(n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zgetri_(
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetri_.unwrap()(n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn cgetrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetrs_.unwrap()(trans, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn dgetrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetrs_.unwrap()(trans, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn sgetrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetrs_.unwrap()(trans, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zgetrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetrs_.unwrap()(trans, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn cgetsls_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetsls_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn dgetsls_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetsls_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn sgetsls_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetsls_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn zgetsls_(
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetsls_.unwrap()(trans, m, n, nrhs, A, lda, B, ldb, work, lwork, info)
}

pub unsafe fn cgetsqrhrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb1: *const lapack_int,
    nb1: *const lapack_int,
    nb2: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgetsqrhrt_.unwrap()(m, n, mb1, nb1, nb2, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn dgetsqrhrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb1: *const lapack_int,
    nb1: *const lapack_int,
    nb2: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgetsqrhrt_.unwrap()(m, n, mb1, nb1, nb2, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn sgetsqrhrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb1: *const lapack_int,
    nb1: *const lapack_int,
    nb2: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgetsqrhrt_.unwrap()(m, n, mb1, nb1, nb2, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn zgetsqrhrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb1: *const lapack_int,
    nb1: *const lapack_int,
    nb2: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgetsqrhrt_.unwrap()(m, n, mb1, nb1, nb2, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn cggbak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    lscale: *const f32,
    rscale: *const f32,
    m: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggbak_.unwrap()(job, side, n, ilo, ihi, lscale, rscale, m, V, ldv, info)
}

pub unsafe fn dggbak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    lscale: *const f64,
    rscale: *const f64,
    m: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggbak_.unwrap()(job, side, n, ilo, ihi, lscale, rscale, m, V, ldv, info)
}

pub unsafe fn sggbak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    lscale: *const f32,
    rscale: *const f32,
    m: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggbak_.unwrap()(job, side, n, ilo, ihi, lscale, rscale, m, V, ldv, info)
}

pub unsafe fn zggbak_(
    job: *const c_char,
    side: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    lscale: *const f64,
    rscale: *const f64,
    m: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggbak_.unwrap()(job, side, n, ilo, ihi, lscale, rscale, m, V, ldv, info)
}

pub unsafe fn cggbal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f32,
    rscale: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cggbal_.unwrap()(job, n, A, lda, B, ldb, ilo, ihi, lscale, rscale, work, info)
}

pub unsafe fn dggbal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f64,
    rscale: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dggbal_.unwrap()(job, n, A, lda, B, ldb, ilo, ihi, lscale, rscale, work, info)
}

pub unsafe fn sggbal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f32,
    rscale: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sggbal_.unwrap()(job, n, A, lda, B, ldb, ilo, ihi, lscale, rscale, work, info)
}

pub unsafe fn zggbal_(
    job: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f64,
    rscale: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zggbal_.unwrap()(job, n, A, lda, B, ldb, ilo, ihi, lscale, rscale, work, info)
}

pub unsafe fn cgges_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_C_SELECT2,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VSL: *mut __BindgenComplex<f32>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f32>,
    ldvsr: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgges_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR, ldvsr,
        work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn dgges_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_D_SELECT3,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VSL: *mut f64,
    ldvsl: *const lapack_int,
    VSR: *mut f64,
    ldvsr: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgges_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL, ldvsl,
        VSR, ldvsr, work, lwork, BWORK, info,
    )
}

pub unsafe fn sgges_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_S_SELECT3,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VSL: *mut f32,
    ldvsl: *const lapack_int,
    VSR: *mut f32,
    ldvsr: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgges_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL, ldvsl,
        VSR, ldvsr, work, lwork, BWORK, info,
    )
}

pub unsafe fn zgges_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_Z_SELECT2,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VSL: *mut __BindgenComplex<f64>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f64>,
    ldvsr: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgges_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR, ldvsr,
        work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn cgges3_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_C_SELECT2,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VSL: *mut __BindgenComplex<f32>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f32>,
    ldvsr: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgges3_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR, ldvsr,
        work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn dgges3_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_D_SELECT3,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VSL: *mut f64,
    ldvsl: *const lapack_int,
    VSR: *mut f64,
    ldvsr: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgges3_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL, ldvsl,
        VSR, ldvsr, work, lwork, BWORK, info,
    )
}

pub unsafe fn sgges3_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_S_SELECT3,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VSL: *mut f32,
    ldvsl: *const lapack_int,
    VSR: *mut f32,
    ldvsr: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgges3_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL, ldvsl,
        VSR, ldvsr, work, lwork, BWORK, info,
    )
}

pub unsafe fn zgges3_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_Z_SELECT2,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VSL: *mut __BindgenComplex<f64>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f64>,
    ldvsr: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgges3_.unwrap()(
        jobvsl, jobvsr, sort, selctg, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR, ldvsr,
        work, lwork, rwork, BWORK, info,
    )
}

pub unsafe fn cggesx_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_C_SELECT2,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VSL: *mut __BindgenComplex<f32>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f32>,
    ldvsr: *const lapack_int,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggesx_.unwrap()(
        jobvsl, jobvsr, sort, selctg, sense, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR,
        ldvsr, rconde, rcondv, work, lwork, rwork, iwork, liwork, BWORK, info,
    )
}

pub unsafe fn dggesx_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_D_SELECT3,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VSL: *mut f64,
    ldvsl: *const lapack_int,
    VSR: *mut f64,
    ldvsr: *const lapack_int,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggesx_.unwrap()(
        jobvsl, jobvsr, sort, selctg, sense, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL,
        ldvsl, VSR, ldvsr, rconde, rcondv, work, lwork, iwork, liwork, BWORK, info,
    )
}

pub unsafe fn sggesx_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_S_SELECT3,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VSL: *mut f32,
    ldvsl: *const lapack_int,
    VSR: *mut f32,
    ldvsr: *const lapack_int,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggesx_.unwrap()(
        jobvsl, jobvsr, sort, selctg, sense, n, A, lda, B, ldb, sdim, alphar, alphai, beta, VSL,
        ldvsl, VSR, ldvsr, rconde, rcondv, work, lwork, iwork, liwork, BWORK, info,
    )
}

pub unsafe fn zggesx_(
    jobvsl: *const c_char,
    jobvsr: *const c_char,
    sort: *const c_char,
    selctg: LAPACK_Z_SELECT2,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    sdim: *mut lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VSL: *mut __BindgenComplex<f64>,
    ldvsl: *const lapack_int,
    VSR: *mut __BindgenComplex<f64>,
    ldvsr: *const lapack_int,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggesx_.unwrap()(
        jobvsl, jobvsr, sort, selctg, sense, n, A, lda, B, ldb, sdim, alpha, beta, VSL, ldvsl, VSR,
        ldvsr, rconde, rcondv, work, lwork, rwork, iwork, liwork, BWORK, info,
    )
}

pub unsafe fn cggev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cggev_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn dggev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggev_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr, work, lwork,
        info,
    )
}

pub unsafe fn sggev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggev_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr, work, lwork,
        info,
    )
}

pub unsafe fn zggev_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zggev_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn cggev3_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cggev3_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn dggev3_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggev3_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr, work, lwork,
        info,
    )
}

pub unsafe fn sggev3_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggev3_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr, work, lwork,
        info,
    )
}

pub unsafe fn zggev3_(
    jobvl: *const c_char,
    jobvr: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zggev3_.unwrap()(
        jobvl, jobvr, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, work, lwork, rwork, info,
    )
}

pub unsafe fn cggevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f32,
    rscale: *mut f32,
    abnrm: *mut f32,
    bbnrm: *mut f32,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, ilo, ihi,
        lscale, rscale, abnrm, bbnrm, rconde, rcondv, work, lwork, rwork, iwork, BWORK, info,
    )
}

pub unsafe fn dggevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f64,
    rscale: *mut f64,
    abnrm: *mut f64,
    bbnrm: *mut f64,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr,
        ilo, ihi, lscale, rscale, abnrm, bbnrm, rconde, rcondv, work, lwork, iwork, BWORK, info,
    )
}

pub unsafe fn sggevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f32,
    rscale: *mut f32,
    abnrm: *mut f32,
    bbnrm: *mut f32,
    rconde: *mut f32,
    rcondv: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, B, ldb, alphar, alphai, beta, VL, ldvl, VR, ldvr,
        ilo, ihi, lscale, rscale, abnrm, bbnrm, rconde, rcondv, work, lwork, iwork, BWORK, info,
    )
}

pub unsafe fn zggevx_(
    balanc: *const c_char,
    jobvl: *const c_char,
    jobvr: *const c_char,
    sense: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    ilo: *mut lapack_int,
    ihi: *mut lapack_int,
    lscale: *mut f64,
    rscale: *mut f64,
    abnrm: *mut f64,
    bbnrm: *mut f64,
    rconde: *mut f64,
    rcondv: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    BWORK: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggevx_.unwrap()(
        balanc, jobvl, jobvr, sense, n, A, lda, B, ldb, alpha, beta, VL, ldvl, VR, ldvr, ilo, ihi,
        lscale, rscale, abnrm, bbnrm, rconde, rcondv, work, lwork, rwork, iwork, BWORK, info,
    )
}

pub unsafe fn cggglm_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    D: *mut __BindgenComplex<f32>,
    X: *mut __BindgenComplex<f32>,
    Y: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggglm_.unwrap()(n, m, p, A, lda, B, ldb, D, X, Y, work, lwork, info)
}

pub unsafe fn dggglm_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    D: *mut f64,
    X: *mut f64,
    Y: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggglm_.unwrap()(n, m, p, A, lda, B, ldb, D, X, Y, work, lwork, info)
}

pub unsafe fn sggglm_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    D: *mut f32,
    X: *mut f32,
    Y: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggglm_.unwrap()(n, m, p, A, lda, B, ldb, D, X, Y, work, lwork, info)
}

pub unsafe fn zggglm_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    D: *mut __BindgenComplex<f64>,
    X: *mut __BindgenComplex<f64>,
    Y: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggglm_.unwrap()(n, m, p, A, lda, B, ldb, D, X, Y, work, lwork, info)
}

pub unsafe fn cgghd3_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgghd3_.unwrap()(
        compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn dgghd3_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgghd3_.unwrap()(
        compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn sgghd3_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgghd3_.unwrap()(
        compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn zgghd3_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgghd3_.unwrap()(
        compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn cgghrd_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgghrd_.unwrap()(compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, info)
}

pub unsafe fn dgghrd_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgghrd_.unwrap()(compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, info)
}

pub unsafe fn sgghrd_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgghrd_.unwrap()(compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, info)
}

pub unsafe fn zgghrd_(
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgghrd_.unwrap()(compq, compz, n, ilo, ihi, A, lda, B, ldb, Q, ldq, Z, ldz, info)
}

pub unsafe fn cgglse_(
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    D: *mut __BindgenComplex<f32>,
    X: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgglse_.unwrap()(m, n, p, A, lda, B, ldb, C, D, X, work, lwork, info)
}

pub unsafe fn dgglse_(
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    C: *mut f64,
    D: *mut f64,
    X: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgglse_.unwrap()(m, n, p, A, lda, B, ldb, C, D, X, work, lwork, info)
}

pub unsafe fn sgglse_(
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    C: *mut f32,
    D: *mut f32,
    X: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgglse_.unwrap()(m, n, p, A, lda, B, ldb, C, D, X, work, lwork, info)
}

pub unsafe fn zgglse_(
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    D: *mut __BindgenComplex<f64>,
    X: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgglse_.unwrap()(m, n, p, A, lda, B, ldb, C, D, X, work, lwork, info)
}

pub unsafe fn cggqrf_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    taua: *mut __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    taub: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggqrf_.unwrap()(n, m, p, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn dggqrf_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    taua: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    taub: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggqrf_.unwrap()(n, m, p, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn sggqrf_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    taua: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    taub: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggqrf_.unwrap()(n, m, p, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn zggqrf_(
    n: *const lapack_int,
    m: *const lapack_int,
    p: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    taua: *mut __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    taub: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggqrf_.unwrap()(n, m, p, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn cggrqf_(
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    taua: *mut __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    taub: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggrqf_.unwrap()(m, p, n, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn dggrqf_(
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    taua: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    taub: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggrqf_.unwrap()(m, p, n, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn sggrqf_(
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    taua: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    taub: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggrqf_.unwrap()(m, p, n, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn zggrqf_(
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    taua: *mut __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    taub: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggrqf_.unwrap()(m, p, n, A, lda, taua, B, ldb, taub, work, lwork, info)
}

pub unsafe fn cggsvd_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    a: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    b: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut f32,
    beta: *mut f32,
    u: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    v: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().cggsvd_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, a, lda, b, ldb, alpha, beta, u, ldu, v, ldv, q, ldq, work,
        rwork, iwork, info,
    )
}

pub unsafe fn sggsvd_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    a: *mut f32,
    lda: *const lapack_int,
    b: *mut f32,
    ldb: *const lapack_int,
    alpha: *mut f32,
    beta: *mut f32,
    u: *mut f32,
    ldu: *const lapack_int,
    v: *mut f32,
    ldv: *const lapack_int,
    q: *mut f32,
    ldq: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().sggsvd_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, a, lda, b, ldb, alpha, beta, u, ldu, v, ldv, q, ldq, work,
        iwork, info,
    )
}

pub unsafe fn dggsvd_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    a: *mut f64,
    lda: *const lapack_int,
    b: *mut f64,
    ldb: *const lapack_int,
    alpha: *mut f64,
    beta: *mut f64,
    u: *mut f64,
    ldu: *const lapack_int,
    v: *mut f64,
    ldv: *const lapack_int,
    q: *mut f64,
    ldq: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().dggsvd_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, a, lda, b, ldb, alpha, beta, u, ldu, v, ldv, q, ldq, work,
        iwork, info,
    )
}

pub unsafe fn zggsvd_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    a: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    b: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut f64,
    beta: *mut f64,
    u: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    v: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().zggsvd_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, a, lda, b, ldb, alpha, beta, u, ldu, v, ldv, q, ldq, work,
        rwork, iwork, info,
    )
}

pub unsafe fn cggsvd3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut f32,
    beta: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggsvd3_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, A, lda, B, ldb, alpha, beta, U, ldu, V, ldv, Q, ldq, work,
        lwork, rwork, iwork, info,
    )
}

pub unsafe fn dggsvd3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    alpha: *mut f64,
    beta: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggsvd3_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, A, lda, B, ldb, alpha, beta, U, ldu, V, ldv, Q, ldq, work,
        lwork, iwork, info,
    )
}

pub unsafe fn sggsvd3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    alpha: *mut f32,
    beta: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggsvd3_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, A, lda, B, ldb, alpha, beta, U, ldu, V, ldv, Q, ldq, work,
        lwork, iwork, info,
    )
}

pub unsafe fn zggsvd3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    p: *const lapack_int,
    k: *mut lapack_int,
    l: *mut lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut f64,
    beta: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggsvd3_.unwrap()(
        jobu, jobv, jobq, m, n, p, k, l, A, lda, B, ldb, alpha, beta, U, ldu, V, ldv, Q, ldq, work,
        lwork, rwork, iwork, info,
    )
}

pub unsafe fn sggsvp_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    a: *mut f32,
    lda: *const lapack_int,
    b: *mut f32,
    ldb: *const lapack_int,
    tola: *mut f32,
    tolb: *mut f32,
    k: *mut lapack_int,
    l: *mut lapack_int,
    u: *mut f32,
    ldu: *const lapack_int,
    v: *mut f32,
    ldv: *const lapack_int,
    q: *mut f32,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    tau: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().sggsvp_.unwrap()(
        jobu, jobv, jobq, m, p, n, a, lda, b, ldb, tola, tolb, k, l, u, ldu, v, ldv, q, ldq, iwork,
        tau, work, info,
    )
}

pub unsafe fn dggsvp_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    a: *mut f64,
    lda: *const lapack_int,
    b: *mut f64,
    ldb: *const lapack_int,
    tola: *mut f64,
    tolb: *mut f64,
    k: *mut lapack_int,
    l: *mut lapack_int,
    u: *mut f64,
    ldu: *const lapack_int,
    v: *mut f64,
    ldv: *const lapack_int,
    q: *mut f64,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    tau: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().dggsvp_.unwrap()(
        jobu, jobv, jobq, m, p, n, a, lda, b, ldb, tola, tolb, k, l, u, ldu, v, ldv, q, ldq, iwork,
        tau, work, info,
    )
}

pub unsafe fn cggsvp_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    a: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    b: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    tola: *mut f32,
    tolb: *mut f32,
    k: *mut lapack_int,
    l: *mut lapack_int,
    u: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    v: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    rwork: *mut f32,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().cggsvp_.unwrap()(
        jobu, jobv, jobq, m, p, n, a, lda, b, ldb, tola, tolb, k, l, u, ldu, v, ldv, q, ldq, iwork,
        rwork, tau, work, info,
    )
}

pub unsafe fn zggsvp_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    a: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    b: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    tola: *mut f64,
    tolb: *mut f64,
    k: *mut lapack_int,
    l: *mut lapack_int,
    u: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    v: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    rwork: *mut f64,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) -> lapack_int {
    dyload_lib().zggsvp_.unwrap()(
        jobu, jobv, jobq, m, p, n, a, lda, b, ldb, tola, tolb, k, l, u, ldu, v, ldv, q, ldq, iwork,
        rwork, tau, work, info,
    )
}

pub unsafe fn cggsvp3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    tola: *const f32,
    tolb: *const f32,
    k: *mut lapack_int,
    l: *mut lapack_int,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    rwork: *mut f32,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cggsvp3_.unwrap()(
        jobu, jobv, jobq, m, p, n, A, lda, B, ldb, tola, tolb, k, l, U, ldu, V, ldv, Q, ldq, iwork,
        rwork, tau, work, lwork, info,
    )
}

pub unsafe fn dggsvp3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    tola: *const f64,
    tolb: *const f64,
    k: *mut lapack_int,
    l: *mut lapack_int,
    U: *mut f64,
    ldu: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dggsvp3_.unwrap()(
        jobu, jobv, jobq, m, p, n, A, lda, B, ldb, tola, tolb, k, l, U, ldu, V, ldv, Q, ldq, iwork,
        tau, work, lwork, info,
    )
}

pub unsafe fn sggsvp3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    tola: *const f32,
    tolb: *const f32,
    k: *mut lapack_int,
    l: *mut lapack_int,
    U: *mut f32,
    ldu: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sggsvp3_.unwrap()(
        jobu, jobv, jobq, m, p, n, A, lda, B, ldb, tola, tolb, k, l, U, ldu, V, ldv, Q, ldq, iwork,
        tau, work, lwork, info,
    )
}

pub unsafe fn zggsvp3_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    tola: *const f64,
    tolb: *const f64,
    k: *mut lapack_int,
    l: *mut lapack_int,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    iwork: *mut lapack_int,
    rwork: *mut f64,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zggsvp3_.unwrap()(
        jobu, jobv, jobq, m, p, n, A, lda, B, ldb, tola, tolb, k, l, U, ldu, V, ldv, Q, ldq, iwork,
        rwork, tau, work, lwork, info,
    )
}

pub unsafe fn cgtcon_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const __BindgenComplex<f32>,
    D: *const __BindgenComplex<f32>,
    DU: *const __BindgenComplex<f32>,
    DU2: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cgtcon_.unwrap()(norm, n, DL, D, DU, DU2, ipiv, anorm, rcond, work, info)
}

pub unsafe fn dgtcon_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const f64,
    D: *const f64,
    DU: *const f64,
    DU2: *const f64,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgtcon_.unwrap()(norm, n, DL, D, DU, DU2, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn sgtcon_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const f32,
    D: *const f32,
    DU: *const f32,
    DU2: *const f32,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgtcon_.unwrap()(norm, n, DL, D, DU, DU2, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn zgtcon_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const __BindgenComplex<f64>,
    D: *const __BindgenComplex<f64>,
    DU: *const __BindgenComplex<f64>,
    DU2: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zgtcon_.unwrap()(norm, n, DL, D, DU, DU2, ipiv, anorm, rcond, work, info)
}

pub unsafe fn cgtrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f32>,
    D: *const __BindgenComplex<f32>,
    DU: *const __BindgenComplex<f32>,
    DLF: *const __BindgenComplex<f32>,
    DF: *const __BindgenComplex<f32>,
    DUF: *const __BindgenComplex<f32>,
    DU2: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgtrfs_.unwrap()(
        trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn dgtrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f64,
    D: *const f64,
    DU: *const f64,
    DLF: *const f64,
    DF: *const f64,
    DUF: *const f64,
    DU2: *const f64,
    ipiv: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgtrfs_.unwrap()(
        trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn sgtrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f32,
    D: *const f32,
    DU: *const f32,
    DLF: *const f32,
    DF: *const f32,
    DUF: *const f32,
    DU2: *const f32,
    ipiv: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgtrfs_.unwrap()(
        trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn zgtrfs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f64>,
    D: *const __BindgenComplex<f64>,
    DU: *const __BindgenComplex<f64>,
    DLF: *const __BindgenComplex<f64>,
    DF: *const __BindgenComplex<f64>,
    DUF: *const __BindgenComplex<f64>,
    DU2: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgtrfs_.unwrap()(
        trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn cgtsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *mut __BindgenComplex<f32>,
    D: *mut __BindgenComplex<f32>,
    DU: *mut __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgtsv_.unwrap()(n, nrhs, DL, D, DU, B, ldb, info)
}

pub unsafe fn dgtsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *mut f64,
    D: *mut f64,
    DU: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgtsv_.unwrap()(n, nrhs, DL, D, DU, B, ldb, info)
}

pub unsafe fn sgtsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *mut f32,
    D: *mut f32,
    DU: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgtsv_.unwrap()(n, nrhs, DL, D, DU, B, ldb, info)
}

pub unsafe fn zgtsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *mut __BindgenComplex<f64>,
    D: *mut __BindgenComplex<f64>,
    DU: *mut __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgtsv_.unwrap()(n, nrhs, DL, D, DU, B, ldb, info)
}

pub unsafe fn cgtsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f32>,
    D: *const __BindgenComplex<f32>,
    DU: *const __BindgenComplex<f32>,
    DLF: *mut __BindgenComplex<f32>,
    DF: *mut __BindgenComplex<f32>,
    DUF: *mut __BindgenComplex<f32>,
    DU2: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cgtsvx_.unwrap()(
        fact, trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, rcond, ferr,
        berr, work, rwork, info,
    )
}

pub unsafe fn dgtsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f64,
    D: *const f64,
    DU: *const f64,
    DLF: *mut f64,
    DF: *mut f64,
    DUF: *mut f64,
    DU2: *mut f64,
    ipiv: *mut lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgtsvx_.unwrap()(
        fact, trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, rcond, ferr,
        berr, work, iwork, info,
    )
}

pub unsafe fn sgtsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f32,
    D: *const f32,
    DU: *const f32,
    DLF: *mut f32,
    DF: *mut f32,
    DUF: *mut f32,
    DU2: *mut f32,
    ipiv: *mut lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgtsvx_.unwrap()(
        fact, trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, rcond, ferr,
        berr, work, iwork, info,
    )
}

pub unsafe fn zgtsvx_(
    fact: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f64>,
    D: *const __BindgenComplex<f64>,
    DU: *const __BindgenComplex<f64>,
    DLF: *mut __BindgenComplex<f64>,
    DF: *mut __BindgenComplex<f64>,
    DUF: *mut __BindgenComplex<f64>,
    DU2: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zgtsvx_.unwrap()(
        fact, trans, n, nrhs, DL, D, DU, DLF, DF, DUF, DU2, ipiv, B, ldb, X, ldx, rcond, ferr,
        berr, work, rwork, info,
    )
}

pub unsafe fn cgttrf_(
    n: *const lapack_int,
    DL: *mut __BindgenComplex<f32>,
    D: *mut __BindgenComplex<f32>,
    DU: *mut __BindgenComplex<f32>,
    DU2: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgttrf_.unwrap()(n, DL, D, DU, DU2, ipiv, info)
}

pub unsafe fn dgttrf_(
    n: *const lapack_int,
    DL: *mut f64,
    D: *mut f64,
    DU: *mut f64,
    DU2: *mut f64,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgttrf_.unwrap()(n, DL, D, DU, DU2, ipiv, info)
}

pub unsafe fn sgttrf_(
    n: *const lapack_int,
    DL: *mut f32,
    D: *mut f32,
    DU: *mut f32,
    DU2: *mut f32,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgttrf_.unwrap()(n, DL, D, DU, DU2, ipiv, info)
}

pub unsafe fn zgttrf_(
    n: *const lapack_int,
    DL: *mut __BindgenComplex<f64>,
    D: *mut __BindgenComplex<f64>,
    DU: *mut __BindgenComplex<f64>,
    DU2: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgttrf_.unwrap()(n, DL, D, DU, DU2, ipiv, info)
}

pub unsafe fn cgttrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f32>,
    D: *const __BindgenComplex<f32>,
    DU: *const __BindgenComplex<f32>,
    DU2: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cgttrs_.unwrap()(trans, n, nrhs, DL, D, DU, DU2, ipiv, B, ldb, info)
}

pub unsafe fn dgttrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f64,
    D: *const f64,
    DU: *const f64,
    DU2: *const f64,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dgttrs_.unwrap()(trans, n, nrhs, DL, D, DU, DU2, ipiv, B, ldb, info)
}

pub unsafe fn sgttrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const f32,
    D: *const f32,
    DU: *const f32,
    DU2: *const f32,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sgttrs_.unwrap()(trans, n, nrhs, DL, D, DU, DU2, ipiv, B, ldb, info)
}

pub unsafe fn zgttrs_(
    trans: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    DL: *const __BindgenComplex<f64>,
    D: *const __BindgenComplex<f64>,
    DU: *const __BindgenComplex<f64>,
    DU2: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zgttrs_.unwrap()(trans, n, nrhs, DL, D, DU, DU2, ipiv, B, ldb, info)
}

pub unsafe fn chbev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chbev_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, rwork, info)
}

pub unsafe fn zhbev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhbev_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, rwork, info)
}

pub unsafe fn chbev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chbev_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, info,
    )
}

pub unsafe fn zhbev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhbev_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, info,
    )
}

pub unsafe fn chbevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbevd_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zhbevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbevd_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn chbevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbevd_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zhbevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbevd_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn chbevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbevx_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn zhbevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbevx_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn chbevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbevx_2stage_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn zhbevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbevx_2stage_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn chbgst_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    BB: *const __BindgenComplex<f32>,
    ldbb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chbgst_.unwrap()(
        vect, uplo, n, ka, kb, AB, ldab, BB, ldbb, X, ldx, work, rwork, info,
    )
}

pub unsafe fn zhbgst_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    BB: *const __BindgenComplex<f64>,
    ldbb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhbgst_.unwrap()(
        vect, uplo, n, ka, kb, AB, ldab, BB, ldbb, X, ldx, work, rwork, info,
    )
}

pub unsafe fn chbgv_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f32>,
    ldbb: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chbgv_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, rwork, info,
    )
}

pub unsafe fn zhbgv_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f64>,
    ldbb: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhbgv_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, rwork, info,
    )
}

pub unsafe fn chbgvd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f32>,
    ldbb: *const lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbgvd_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, lwork, rwork, lrwork, iwork,
        liwork, info,
    )
}

pub unsafe fn zhbgvd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f64>,
    ldbb: *const lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbgvd_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, lwork, rwork, lrwork, iwork,
        liwork, info,
    )
}

pub unsafe fn chbgvx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f32>,
    ldbb: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chbgvx_.unwrap()(
        jobz, range, uplo, n, ka, kb, AB, ldab, BB, ldbb, Q, ldq, vl, vu, il, iu, abstol, m, W, Z,
        ldz, work, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn zhbgvx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    BB: *mut __BindgenComplex<f64>,
    ldbb: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhbgvx_.unwrap()(
        jobz, range, uplo, n, ka, kb, AB, ldab, BB, ldbb, Q, ldq, vl, vu, il, iu, abstol, m, W, Z,
        ldz, work, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn chbtrd_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chbtrd_.unwrap()(vect, uplo, n, kd, AB, ldab, D, E, Q, ldq, work, info)
}

pub unsafe fn zhbtrd_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhbtrd_.unwrap()(vect, uplo, n, kd, AB, ldab, D, E, Q, ldq, work, info)
}

pub unsafe fn checon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().checon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, info)
}

pub unsafe fn zhecon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhecon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, info)
}

pub unsafe fn checon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().checon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, info)
}

pub unsafe fn zhecon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhecon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, info)
}

pub unsafe fn cheequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cheequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn zheequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zheequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn cheev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cheev_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, rwork, info)
}

pub unsafe fn zheev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zheev_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, rwork, info)
}

pub unsafe fn cheev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cheev_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, rwork, info)
}

pub unsafe fn zheev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zheev_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, rwork, info)
}

pub unsafe fn cheevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevd_.unwrap()(
        jobz, uplo, n, A, lda, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zheevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevd_.unwrap()(
        jobz, uplo, n, A, lda, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cheevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevd_2stage_.unwrap()(
        jobz, uplo, n, A, lda, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zheevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevd_2stage_.unwrap()(
        jobz, uplo, n, A, lda, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cheevr_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevr_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zheevr_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevr_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cheevr_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevr_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zheevr_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevr_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cheevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevx_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn zheevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevx_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn cheevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cheevx_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn zheevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zheevx_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn chegst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chegst_.unwrap()(itype, uplo, n, A, lda, B, ldb, info)
}

pub unsafe fn zhegst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhegst_.unwrap()(itype, uplo, n, A, lda, B, ldb, info)
}

pub unsafe fn chegv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chegv_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, info)
}

pub unsafe fn zhegv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhegv_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, info)
}

pub unsafe fn chegv_2stage_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chegv_2stage_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, info,
    )
}

pub unsafe fn zhegv_2stage_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhegv_2stage_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, info,
    )
}

pub unsafe fn chegvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chegvd_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zhegvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhegvd_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn chegvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chegvx_.unwrap()(
        itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn zhegvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhegvx_.unwrap()(
        itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, rwork, iwork, IFAIL, info,
    )
}

pub unsafe fn cherfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cherfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn zherfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zherfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cherfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f32,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cherfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn zherfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f64,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zherfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn chesv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chesv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zhesv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhesv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn chesv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chesv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zhesv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhesv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn chesv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chesv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn zhesv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhesv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn chesv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chesv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zhesv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhesv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn chesv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chesv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zhesv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhesv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn chesvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chesvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, rwork, info,
    )
}

pub unsafe fn zhesvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhesvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, rwork, info,
    )
}

pub unsafe fn chesvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chesvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn zhesvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhesvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cheswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().cheswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn zheswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().zheswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn chetrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrd_.unwrap()(uplo, n, A, lda, D, E, tau, work, lwork, info)
}

pub unsafe fn zhetrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrd_.unwrap()(uplo, n, A, lda, D, E, tau, work, lwork, info)
}

pub unsafe fn chetrd_2stage_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tau: *mut __BindgenComplex<f32>,
    HOUS2: *mut __BindgenComplex<f32>,
    lhous2: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrd_2stage_.unwrap()(
        vect, uplo, n, A, lda, D, E, tau, HOUS2, lhous2, work, lwork, info,
    )
}

pub unsafe fn zhetrd_2stage_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tau: *mut __BindgenComplex<f64>,
    HOUS2: *mut __BindgenComplex<f64>,
    lhous2: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrd_2stage_.unwrap()(
        vect, uplo, n, A, lda, D, E, tau, HOUS2, lhous2, work, lwork, info,
    )
}

pub unsafe fn chetrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zhetrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn chetrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zhetrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn chetrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn zhetrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn chetrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn zhetrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn chetrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zhetrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn chetri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chetri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn zhetri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhetri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn chetri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zhetri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn chetri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn zhetri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn chetri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn zhetri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn chetrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zhetrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn chetrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn zhetrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn chetrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn zhetrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn chetrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zhetrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn chetrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn zhetrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn chetrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chetrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zhetrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhetrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn chfrk_(
    transr: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    alpha: *const f32,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    beta: *const f32,
    C: *mut __BindgenComplex<f32>,
) {
    dyload_lib().chfrk_.unwrap()(transr, uplo, trans, n, k, alpha, A, lda, beta, C)
}

pub unsafe fn zhfrk_(
    transr: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    alpha: *const f64,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    beta: *const f64,
    C: *mut __BindgenComplex<f64>,
) {
    dyload_lib().zhfrk_.unwrap()(transr, uplo, trans, n, k, alpha, A, lda, beta, C)
}

pub unsafe fn chgeqz_(
    job: *const c_char,
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut __BindgenComplex<f32>,
    ldh: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chgeqz_.unwrap()(
        job, compq, compz, n, ilo, ihi, H, ldh, T, ldt, alpha, beta, Q, ldq, Z, ldz, work, lwork,
        rwork, info,
    )
}

pub unsafe fn dhgeqz_(
    job: *const c_char,
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut f64,
    ldh: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    Q: *mut f64,
    ldq: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dhgeqz_.unwrap()(
        job, compq, compz, n, ilo, ihi, H, ldh, T, ldt, alphar, alphai, beta, Q, ldq, Z, ldz, work,
        lwork, info,
    )
}

pub unsafe fn shgeqz_(
    job: *const c_char,
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut f32,
    ldh: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    Q: *mut f32,
    ldq: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().shgeqz_.unwrap()(
        job, compq, compz, n, ilo, ihi, H, ldh, T, ldt, alphar, alphai, beta, Q, ldq, Z, ldz, work,
        lwork, info,
    )
}

pub unsafe fn zhgeqz_(
    job: *const c_char,
    compq: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut __BindgenComplex<f64>,
    ldh: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhgeqz_.unwrap()(
        job, compq, compz, n, ilo, ihi, H, ldh, T, ldt, alpha, beta, Q, ldq, Z, ldz, work, lwork,
        rwork, info,
    )
}

pub unsafe fn chpcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chpcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, info)
}

pub unsafe fn zhpcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhpcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, info)
}

pub unsafe fn chpev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chpev_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, rwork, info)
}

pub unsafe fn zhpev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhpev_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, rwork, info)
}

pub unsafe fn chpevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chpevd_.unwrap()(
        jobz, uplo, n, AP, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zhpevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhpevd_.unwrap()(
        jobz, uplo, n, AP, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn chpevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chpevx_.unwrap()(
        jobz, range, uplo, n, AP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, rwork, iwork, IFAIL,
        info,
    )
}

pub unsafe fn zhpevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhpevx_.unwrap()(
        jobz, range, uplo, n, AP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, rwork, iwork, IFAIL,
        info,
    )
}

pub unsafe fn chpgst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    BP: *const __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chpgst_.unwrap()(itype, uplo, n, AP, BP, info)
}

pub unsafe fn zhpgst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    BP: *const __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhpgst_.unwrap()(itype, uplo, n, AP, BP, info)
}

pub unsafe fn chpgv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    BP: *mut __BindgenComplex<f32>,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chpgv_.unwrap()(itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, rwork, info)
}

pub unsafe fn zhpgv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    BP: *mut __BindgenComplex<f64>,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhpgv_.unwrap()(itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, rwork, info)
}

pub unsafe fn chpgvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    BP: *mut __BindgenComplex<f32>,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chpgvd_.unwrap()(
        itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn zhpgvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    BP: *mut __BindgenComplex<f64>,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhpgvd_.unwrap()(
        itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn chpgvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    BP: *mut __BindgenComplex<f32>,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chpgvx_.unwrap()(
        itype, jobz, range, uplo, n, AP, BP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn zhpgvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    BP: *mut __BindgenComplex<f64>,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhpgvx_.unwrap()(
        itype, jobz, range, uplo, n, AP, BP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, rwork,
        iwork, IFAIL, info,
    )
}

pub unsafe fn chprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    AFP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn zhprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    AFP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn chpsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chpsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn zhpsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhpsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn chpsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    AFP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().chpsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn zhpsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    AFP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zhpsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn chptrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    D: *mut f32,
    E: *mut f32,
    tau: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chptrd_.unwrap()(uplo, n, AP, D, E, tau, info)
}

pub unsafe fn zhptrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    D: *mut f64,
    E: *mut f64,
    tau: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhptrd_.unwrap()(uplo, n, AP, D, E, tau, info)
}

pub unsafe fn chptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn zhptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn chptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().chptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn zhptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zhptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn chptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn zhptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn chsein_(
    side: *const c_char,
    eigsrc: *const c_char,
    initv: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    H: *const __BindgenComplex<f32>,
    ldh: *const lapack_int,
    W: *mut __BindgenComplex<f32>,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    IFAILL: *mut lapack_int,
    IFAILR: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chsein_.unwrap()(
        side, eigsrc, initv, select, n, H, ldh, W, VL, ldvl, VR, ldvr, mm, m, work, rwork, IFAILL,
        IFAILR, info,
    )
}

pub unsafe fn dhsein_(
    side: *const c_char,
    eigsrc: *const c_char,
    initv: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    H: *const f64,
    ldh: *const lapack_int,
    WR: *mut f64,
    WI: *const f64,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    IFAILL: *mut lapack_int,
    IFAILR: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dhsein_.unwrap()(
        side, eigsrc, initv, select, n, H, ldh, WR, WI, VL, ldvl, VR, ldvr, mm, m, work, IFAILL,
        IFAILR, info,
    )
}

pub unsafe fn shsein_(
    side: *const c_char,
    eigsrc: *const c_char,
    initv: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    H: *const f32,
    ldh: *const lapack_int,
    WR: *mut f32,
    WI: *const f32,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    IFAILL: *mut lapack_int,
    IFAILR: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().shsein_.unwrap()(
        side, eigsrc, initv, select, n, H, ldh, WR, WI, VL, ldvl, VR, ldvr, mm, m, work, IFAILL,
        IFAILR, info,
    )
}

pub unsafe fn zhsein_(
    side: *const c_char,
    eigsrc: *const c_char,
    initv: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    H: *const __BindgenComplex<f64>,
    ldh: *const lapack_int,
    W: *mut __BindgenComplex<f64>,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    IFAILL: *mut lapack_int,
    IFAILR: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhsein_.unwrap()(
        side, eigsrc, initv, select, n, H, ldh, W, VL, ldvl, VR, ldvr, mm, m, work, rwork, IFAILL,
        IFAILR, info,
    )
}

pub unsafe fn chseqr_(
    job: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut __BindgenComplex<f32>,
    ldh: *const lapack_int,
    W: *mut __BindgenComplex<f32>,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().chseqr_.unwrap()(job, compz, n, ilo, ihi, H, ldh, W, Z, ldz, work, lwork, info)
}

pub unsafe fn dhseqr_(
    job: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut f64,
    ldh: *const lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dhseqr_.unwrap()(
        job, compz, n, ilo, ihi, H, ldh, WR, WI, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn shseqr_(
    job: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut f32,
    ldh: *const lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().shseqr_.unwrap()(
        job, compz, n, ilo, ihi, H, ldh, WR, WI, Z, ldz, work, lwork, info,
    )
}

pub unsafe fn zhseqr_(
    job: *const c_char,
    compz: *const c_char,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    H: *mut __BindgenComplex<f64>,
    ldh: *const lapack_int,
    W: *mut __BindgenComplex<f64>,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zhseqr_.unwrap()(job, compz, n, ilo, ihi, H, ldh, W, Z, ldz, work, lwork, info)
}

pub unsafe fn clacgv_(
    n: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    incx: *const lapack_int,
) {
    dyload_lib().clacgv_.unwrap()(n, X, incx)
}

pub unsafe fn zlacgv_(
    n: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    incx: *const lapack_int,
) {
    dyload_lib().zlacgv_.unwrap()(n, X, incx)
}

pub unsafe fn clacn2_(
    n: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    X: *mut __BindgenComplex<f32>,
    est: *mut f32,
    kase: *mut lapack_int,
    ISAVE: *mut lapack_int,
) {
    dyload_lib().clacn2_.unwrap()(n, V, X, est, kase, ISAVE)
}

pub unsafe fn dlacn2_(
    n: *const lapack_int,
    V: *mut f64,
    X: *mut f64,
    ISGN: *mut lapack_int,
    est: *mut f64,
    kase: *mut lapack_int,
    ISAVE: *mut lapack_int,
) {
    dyload_lib().dlacn2_.unwrap()(n, V, X, ISGN, est, kase, ISAVE)
}

pub unsafe fn slacn2_(
    n: *const lapack_int,
    V: *mut f32,
    X: *mut f32,
    ISGN: *mut lapack_int,
    est: *mut f32,
    kase: *mut lapack_int,
    ISAVE: *mut lapack_int,
) {
    dyload_lib().slacn2_.unwrap()(n, V, X, ISGN, est, kase, ISAVE)
}

pub unsafe fn zlacn2_(
    n: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    X: *mut __BindgenComplex<f64>,
    est: *mut f64,
    kase: *mut lapack_int,
    ISAVE: *mut lapack_int,
) {
    dyload_lib().zlacn2_.unwrap()(n, V, X, est, kase, ISAVE)
}

pub unsafe fn clacp2_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
) {
    dyload_lib().clacp2_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn zlacp2_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
) {
    dyload_lib().zlacp2_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn clacpy_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
) {
    dyload_lib().clacpy_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn dlacpy_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
) {
    dyload_lib().dlacpy_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn slacpy_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
) {
    dyload_lib().slacpy_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn zlacpy_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
) {
    dyload_lib().zlacpy_.unwrap()(uplo, m, n, A, lda, B, ldb)
}

pub unsafe fn clacrm_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    rwork: *mut f32,
) {
    dyload_lib().clacrm_.unwrap()(m, n, A, lda, B, ldb, C, ldc, rwork)
}

pub unsafe fn zlacrm_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    rwork: *mut f64,
) {
    dyload_lib().zlacrm_.unwrap()(m, n, A, lda, B, ldb, C, ldc, rwork)
}

pub unsafe fn zlag2c_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    SA: *mut __BindgenComplex<f32>,
    ldsa: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zlag2c_.unwrap()(m, n, A, lda, SA, ldsa, info)
}

pub unsafe fn slag2d_(
    m: *const lapack_int,
    n: *const lapack_int,
    SA: *const f32,
    ldsa: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().slag2d_.unwrap()(m, n, SA, ldsa, A, lda, info)
}

pub unsafe fn dlag2s_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    SA: *mut f32,
    ldsa: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dlag2s_.unwrap()(m, n, A, lda, SA, ldsa, info)
}

pub unsafe fn clag2z_(
    m: *const lapack_int,
    n: *const lapack_int,
    SA: *const __BindgenComplex<f32>,
    ldsa: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().clag2z_.unwrap()(m, n, SA, ldsa, A, lda, info)
}

pub unsafe fn clagge_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    D: *const f32,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().clagge_.unwrap()(m, n, kl, ku, D, A, lda, iseed, work, info)
}

pub unsafe fn dlagge_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    D: *const f64,
    A: *mut f64,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dlagge_.unwrap()(m, n, kl, ku, D, A, lda, iseed, work, info)
}

pub unsafe fn slagge_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    D: *const f32,
    A: *mut f32,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().slagge_.unwrap()(m, n, kl, ku, D, A, lda, iseed, work, info)
}

pub unsafe fn zlagge_(
    m: *const lapack_int,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    D: *const f64,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zlagge_.unwrap()(m, n, kl, ku, D, A, lda, iseed, work, info)
}

pub unsafe fn claghe_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f32,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().claghe_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn zlaghe_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f64,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zlaghe_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn clagsy_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f32,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().clagsy_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn dlagsy_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f64,
    A: *mut f64,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dlagsy_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn slagsy_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f32,
    A: *mut f32,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().slagsy_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn zlagsy_(
    n: *const lapack_int,
    k: *const lapack_int,
    D: *const f64,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    iseed: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zlagsy_.unwrap()(n, k, D, A, lda, iseed, work, info)
}

pub unsafe fn dlamch_(cmach: *const c_char) -> f64 {
    dyload_lib().dlamch_.unwrap()(cmach)
}

pub unsafe fn slamch_(cmach: *const c_char) -> lapack_float_return {
    dyload_lib().slamch_.unwrap()(cmach)
}

pub unsafe fn clangb_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clangb_.unwrap()(norm, n, kl, ku, AB, ldab, work)
}

pub unsafe fn dlangb_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlangb_.unwrap()(norm, n, kl, ku, AB, ldab, work)
}

pub unsafe fn slangb_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slangb_.unwrap()(norm, n, kl, ku, AB, ldab, work)
}

pub unsafe fn zlangb_(
    norm: *const c_char,
    n: *const lapack_int,
    kl: *const lapack_int,
    ku: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlangb_.unwrap()(norm, n, kl, ku, AB, ldab, work)
}

pub unsafe fn clange_(
    norm: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clange_.unwrap()(norm, m, n, A, lda, work)
}

pub unsafe fn dlange_(
    norm: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlange_.unwrap()(norm, m, n, A, lda, work)
}

pub unsafe fn slange_(
    norm: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slange_.unwrap()(norm, m, n, A, lda, work)
}

pub unsafe fn zlange_(
    norm: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlange_.unwrap()(norm, m, n, A, lda, work)
}

pub unsafe fn clangt_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const __BindgenComplex<f32>,
    D: *const __BindgenComplex<f32>,
    DU: *const __BindgenComplex<f32>,
) -> lapack_float_return {
    dyload_lib().clangt_.unwrap()(norm, n, DL, D, DU)
}

pub unsafe fn dlangt_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const f64,
    D: *const f64,
    DU: *const f64,
) -> f64 {
    dyload_lib().dlangt_.unwrap()(norm, n, DL, D, DU)
}

pub unsafe fn slangt_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const f32,
    D: *const f32,
    DU: *const f32,
) -> lapack_float_return {
    dyload_lib().slangt_.unwrap()(norm, n, DL, D, DU)
}

pub unsafe fn zlangt_(
    norm: *const c_char,
    n: *const lapack_int,
    DL: *const __BindgenComplex<f64>,
    D: *const __BindgenComplex<f64>,
    DU: *const __BindgenComplex<f64>,
) -> f64 {
    dyload_lib().zlangt_.unwrap()(norm, n, DL, D, DU)
}

pub unsafe fn clanhb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clanhb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn zlanhb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlanhb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn clanhe_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clanhe_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn zlanhe_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlanhe_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn clanhp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clanhp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn zlanhp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlanhp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn clanhs_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clanhs_.unwrap()(norm, n, A, lda, work)
}

pub unsafe fn dlanhs_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlanhs_.unwrap()(norm, n, A, lda, work)
}

pub unsafe fn slanhs_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slanhs_.unwrap()(norm, n, A, lda, work)
}

pub unsafe fn zlanhs_(
    norm: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlanhs_.unwrap()(norm, n, A, lda, work)
}

pub unsafe fn clanht_(
    norm: *const c_char,
    n: *const lapack_int,
    D: *const f32,
    E: *const __BindgenComplex<f32>,
) -> lapack_float_return {
    dyload_lib().clanht_.unwrap()(norm, n, D, E)
}

pub unsafe fn zlanht_(
    norm: *const c_char,
    n: *const lapack_int,
    D: *const f64,
    E: *const __BindgenComplex<f64>,
) -> f64 {
    dyload_lib().zlanht_.unwrap()(norm, n, D, E)
}

pub unsafe fn clansb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clansb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn dlansb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlansb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn slansb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slansb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn zlansb_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlansb_.unwrap()(norm, uplo, n, k, AB, ldab, work)
}

pub unsafe fn clansp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clansp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn dlansp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlansp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn slansp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slansp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn zlansp_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlansp_.unwrap()(norm, uplo, n, AP, work)
}

pub unsafe fn dlanst_(
    norm: *const c_char,
    n: *const lapack_int,
    D: *const f64,
    E: *const f64,
) -> f64 {
    dyload_lib().dlanst_.unwrap()(norm, n, D, E)
}

pub unsafe fn slanst_(
    norm: *const c_char,
    n: *const lapack_int,
    D: *const f32,
    E: *const f32,
) -> lapack_float_return {
    dyload_lib().slanst_.unwrap()(norm, n, D, E)
}

pub unsafe fn clansy_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clansy_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn dlansy_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlansy_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn slansy_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slansy_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn zlansy_(
    norm: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlansy_.unwrap()(norm, uplo, n, A, lda, work)
}

pub unsafe fn clantb_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clantb_.unwrap()(norm, uplo, diag, n, k, AB, ldab, work)
}

pub unsafe fn dlantb_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlantb_.unwrap()(norm, uplo, diag, n, k, AB, ldab, work)
}

pub unsafe fn slantb_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slantb_.unwrap()(norm, uplo, diag, n, k, AB, ldab, work)
}

pub unsafe fn zlantb_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlantb_.unwrap()(norm, uplo, diag, n, k, AB, ldab, work)
}

pub unsafe fn clantp_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clantp_.unwrap()(norm, uplo, diag, n, AP, work)
}

pub unsafe fn dlantp_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlantp_.unwrap()(norm, uplo, diag, n, AP, work)
}

pub unsafe fn slantp_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slantp_.unwrap()(norm, uplo, diag, n, AP, work)
}

pub unsafe fn zlantp_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlantp_.unwrap()(norm, uplo, diag, n, AP, work)
}

pub unsafe fn clantr_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().clantr_.unwrap()(norm, uplo, diag, m, n, A, lda, work)
}

pub unsafe fn dlantr_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().dlantr_.unwrap()(norm, uplo, diag, m, n, A, lda, work)
}

pub unsafe fn slantr_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    work: *mut f32,
) -> lapack_float_return {
    dyload_lib().slantr_.unwrap()(norm, uplo, diag, m, n, A, lda, work)
}

pub unsafe fn zlantr_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut f64,
) -> f64 {
    dyload_lib().zlantr_.unwrap()(norm, uplo, diag, m, n, A, lda, work)
}

pub unsafe fn clapmr_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().clapmr_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn dlapmr_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().dlapmr_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn slapmr_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().slapmr_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn zlapmr_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().zlapmr_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn clapmt_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().clapmt_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn dlapmt_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().dlapmt_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn slapmt_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().slapmt_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn zlapmt_(
    forwrd: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    K: *mut lapack_int,
) {
    dyload_lib().zlapmt_.unwrap()(forwrd, m, n, X, ldx, K)
}

pub unsafe fn dlapy2_(x: *const f64, y: *const f64) -> f64 {
    dyload_lib().dlapy2_.unwrap()(x, y)
}

pub unsafe fn slapy2_(x: *const f32, y: *const f32) -> lapack_float_return {
    dyload_lib().slapy2_.unwrap()(x, y)
}

pub unsafe fn dlapy3_(x: *const f64, y: *const f64, z: *const f64) -> f64 {
    dyload_lib().dlapy3_.unwrap()(x, y, z)
}

pub unsafe fn slapy3_(x: *const f32, y: *const f32, z: *const f32) -> lapack_float_return {
    dyload_lib().slapy3_.unwrap()(x, y, z)
}

pub unsafe fn clarcm_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    rwork: *mut f32,
) {
    dyload_lib().clarcm_.unwrap()(m, n, A, lda, B, ldb, C, ldc, rwork)
}

pub unsafe fn zlarcm_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    rwork: *mut f64,
) {
    dyload_lib().zlarcm_.unwrap()(m, n, A, lda, B, ldb, C, ldc, rwork)
}

pub unsafe fn clarf_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    incv: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
) {
    dyload_lib().clarf_.unwrap()(side, m, n, V, incv, tau, C, ldc, work)
}

pub unsafe fn dlarf_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const f64,
    incv: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
) {
    dyload_lib().dlarf_.unwrap()(side, m, n, V, incv, tau, C, ldc, work)
}

pub unsafe fn slarf_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const f32,
    incv: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
) {
    dyload_lib().slarf_.unwrap()(side, m, n, V, incv, tau, C, ldc, work)
}

pub unsafe fn zlarf_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    incv: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
) {
    dyload_lib().zlarf_.unwrap()(side, m, n, V, incv, tau, C, ldc, work)
}

pub unsafe fn clarfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    ldwork: *const lapack_int,
) {
    dyload_lib().clarfb_.unwrap()(
        side, trans, direct, storev, m, n, k, V, ldv, T, ldt, C, ldc, work, ldwork,
    )
}

pub unsafe fn dlarfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    ldwork: *const lapack_int,
) {
    dyload_lib().dlarfb_.unwrap()(
        side, trans, direct, storev, m, n, k, V, ldv, T, ldt, C, ldc, work, ldwork,
    )
}

pub unsafe fn slarfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    ldwork: *const lapack_int,
) {
    dyload_lib().slarfb_.unwrap()(
        side, trans, direct, storev, m, n, k, V, ldv, T, ldt, C, ldc, work, ldwork,
    )
}

pub unsafe fn zlarfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    ldwork: *const lapack_int,
) {
    dyload_lib().zlarfb_.unwrap()(
        side, trans, direct, storev, m, n, k, V, ldv, T, ldt, C, ldc, work, ldwork,
    )
}

pub unsafe fn clarfg_(
    n: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    X: *mut __BindgenComplex<f32>,
    incx: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
) {
    dyload_lib().clarfg_.unwrap()(n, alpha, X, incx, tau)
}

pub unsafe fn dlarfg_(
    n: *const lapack_int,
    alpha: *mut f64,
    X: *mut f64,
    incx: *const lapack_int,
    tau: *mut f64,
) {
    dyload_lib().dlarfg_.unwrap()(n, alpha, X, incx, tau)
}

pub unsafe fn slarfg_(
    n: *const lapack_int,
    alpha: *mut f32,
    X: *mut f32,
    incx: *const lapack_int,
    tau: *mut f32,
) {
    dyload_lib().slarfg_.unwrap()(n, alpha, X, incx, tau)
}

pub unsafe fn zlarfg_(
    n: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    X: *mut __BindgenComplex<f64>,
    incx: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
) {
    dyload_lib().zlarfg_.unwrap()(n, alpha, X, incx, tau)
}

pub unsafe fn clarft_(
    direct: *const c_char,
    storev: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
) {
    dyload_lib().clarft_.unwrap()(direct, storev, n, k, V, ldv, tau, T, ldt)
}

pub unsafe fn dlarft_(
    direct: *const c_char,
    storev: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    tau: *const f64,
    T: *mut f64,
    ldt: *const lapack_int,
) {
    dyload_lib().dlarft_.unwrap()(direct, storev, n, k, V, ldv, tau, T, ldt)
}

pub unsafe fn slarft_(
    direct: *const c_char,
    storev: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    tau: *const f32,
    T: *mut f32,
    ldt: *const lapack_int,
) {
    dyload_lib().slarft_.unwrap()(direct, storev, n, k, V, ldv, tau, T, ldt)
}

pub unsafe fn zlarft_(
    direct: *const c_char,
    storev: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
) {
    dyload_lib().zlarft_.unwrap()(direct, storev, n, k, V, ldv, tau, T, ldt)
}

pub unsafe fn clarfx_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
) {
    dyload_lib().clarfx_.unwrap()(side, m, n, V, tau, C, ldc, work)
}

pub unsafe fn dlarfx_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const f64,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
) {
    dyload_lib().dlarfx_.unwrap()(side, m, n, V, tau, C, ldc, work)
}

pub unsafe fn slarfx_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const f32,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
) {
    dyload_lib().slarfx_.unwrap()(side, m, n, V, tau, C, ldc, work)
}

pub unsafe fn zlarfx_(
    side: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
) {
    dyload_lib().zlarfx_.unwrap()(side, m, n, V, tau, C, ldc, work)
}

pub unsafe fn clarnv_(
    idist: *const lapack_int,
    iseed: *mut lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
) {
    dyload_lib().clarnv_.unwrap()(idist, iseed, n, X)
}

pub unsafe fn dlarnv_(
    idist: *const lapack_int,
    iseed: *mut lapack_int,
    n: *const lapack_int,
    X: *mut f64,
) {
    dyload_lib().dlarnv_.unwrap()(idist, iseed, n, X)
}

pub unsafe fn slarnv_(
    idist: *const lapack_int,
    iseed: *mut lapack_int,
    n: *const lapack_int,
    X: *mut f32,
) {
    dyload_lib().slarnv_.unwrap()(idist, iseed, n, X)
}

pub unsafe fn zlarnv_(
    idist: *const lapack_int,
    iseed: *mut lapack_int,
    n: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
) {
    dyload_lib().zlarnv_.unwrap()(idist, iseed, n, X)
}

pub unsafe fn dlartgp_(f: *const f64, g: *const f64, cs: *mut f64, sn: *mut f64, r: *mut f64) {
    dyload_lib().dlartgp_.unwrap()(f, g, cs, sn, r)
}

pub unsafe fn slartgp_(f: *const f32, g: *const f32, cs: *mut f32, sn: *mut f32, r: *mut f32) {
    dyload_lib().slartgp_.unwrap()(f, g, cs, sn, r)
}

pub unsafe fn dlartgs_(
    x: *const f64,
    y: *const f64,
    sigma: *const f64,
    cs: *mut f64,
    sn: *mut f64,
) {
    dyload_lib().dlartgs_.unwrap()(x, y, sigma, cs, sn)
}

pub unsafe fn slartgs_(
    x: *const f32,
    y: *const f32,
    sigma: *const f32,
    cs: *mut f32,
    sn: *mut f32,
) {
    dyload_lib().slartgs_.unwrap()(x, y, sigma, cs, sn)
}

pub unsafe fn clascl_(
    type_: *const c_char,
    kl: *const lapack_int,
    ku: *const lapack_int,
    cfrom: *const f32,
    cto: *const f32,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().clascl_.unwrap()(type_, kl, ku, cfrom, cto, m, n, A, lda, info)
}

pub unsafe fn dlascl_(
    type_: *const c_char,
    kl: *const lapack_int,
    ku: *const lapack_int,
    cfrom: *const f64,
    cto: *const f64,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dlascl_.unwrap()(type_, kl, ku, cfrom, cto, m, n, A, lda, info)
}

pub unsafe fn slascl_(
    type_: *const c_char,
    kl: *const lapack_int,
    ku: *const lapack_int,
    cfrom: *const f32,
    cto: *const f32,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().slascl_.unwrap()(type_, kl, ku, cfrom, cto, m, n, A, lda, info)
}

pub unsafe fn zlascl_(
    type_: *const c_char,
    kl: *const lapack_int,
    ku: *const lapack_int,
    cfrom: *const f64,
    cto: *const f64,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zlascl_.unwrap()(type_, kl, ku, cfrom, cto, m, n, A, lda, info)
}

pub unsafe fn claset_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f32>,
    beta: *const __BindgenComplex<f32>,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
) {
    dyload_lib().claset_.unwrap()(uplo, m, n, alpha, beta, A, lda)
}

pub unsafe fn dlaset_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const f64,
    beta: *const f64,
    A: *mut f64,
    lda: *const lapack_int,
) {
    dyload_lib().dlaset_.unwrap()(uplo, m, n, alpha, beta, A, lda)
}

pub unsafe fn slaset_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const f32,
    beta: *const f32,
    A: *mut f32,
    lda: *const lapack_int,
) {
    dyload_lib().slaset_.unwrap()(uplo, m, n, alpha, beta, A, lda)
}

pub unsafe fn zlaset_(
    uplo: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f64>,
    beta: *const __BindgenComplex<f64>,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
) {
    dyload_lib().zlaset_.unwrap()(uplo, m, n, alpha, beta, A, lda)
}

pub unsafe fn dlasrt_(id: *const c_char, n: *const lapack_int, D: *mut f64, info: *mut lapack_int) {
    dyload_lib().dlasrt_.unwrap()(id, n, D, info)
}

pub unsafe fn slasrt_(id: *const c_char, n: *const lapack_int, D: *mut f32, info: *mut lapack_int) {
    dyload_lib().slasrt_.unwrap()(id, n, D, info)
}

pub unsafe fn classq_(
    n: *const lapack_int,
    X: *const __BindgenComplex<f32>,
    incx: *const lapack_int,
    scale: *mut f32,
    sumsq: *mut f32,
) {
    dyload_lib().classq_.unwrap()(n, X, incx, scale, sumsq)
}

pub unsafe fn dlassq_(
    n: *const lapack_int,
    X: *const f64,
    incx: *const lapack_int,
    scale: *mut f64,
    sumsq: *mut f64,
) {
    dyload_lib().dlassq_.unwrap()(n, X, incx, scale, sumsq)
}

pub unsafe fn slassq_(
    n: *const lapack_int,
    X: *const f32,
    incx: *const lapack_int,
    scale: *mut f32,
    sumsq: *mut f32,
) {
    dyload_lib().slassq_.unwrap()(n, X, incx, scale, sumsq)
}

pub unsafe fn zlassq_(
    n: *const lapack_int,
    X: *const __BindgenComplex<f64>,
    incx: *const lapack_int,
    scale: *mut f64,
    sumsq: *mut f64,
) {
    dyload_lib().zlassq_.unwrap()(n, X, incx, scale, sumsq)
}

pub unsafe fn claswp_(
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    k1: *const lapack_int,
    k2: *const lapack_int,
    ipiv: *const lapack_int,
    incx: *const lapack_int,
) {
    dyload_lib().claswp_.unwrap()(n, A, lda, k1, k2, ipiv, incx)
}

pub unsafe fn dlaswp_(
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    k1: *const lapack_int,
    k2: *const lapack_int,
    ipiv: *const lapack_int,
    incx: *const lapack_int,
) {
    dyload_lib().dlaswp_.unwrap()(n, A, lda, k1, k2, ipiv, incx)
}

pub unsafe fn slaswp_(
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    k1: *const lapack_int,
    k2: *const lapack_int,
    ipiv: *const lapack_int,
    incx: *const lapack_int,
) {
    dyload_lib().slaswp_.unwrap()(n, A, lda, k1, k2, ipiv, incx)
}

pub unsafe fn zlaswp_(
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    k1: *const lapack_int,
    k2: *const lapack_int,
    ipiv: *const lapack_int,
    incx: *const lapack_int,
) {
    dyload_lib().zlaswp_.unwrap()(n, A, lda, k1, k2, ipiv, incx)
}

pub unsafe fn clatms_(
    m: *const lapack_int,
    n: *const lapack_int,
    dist: *const c_char,
    iseed: *mut lapack_int,
    sym: *const c_char,
    D: *mut f32,
    mode: *const lapack_int,
    cond: *const f32,
    dmax: *const f32,
    kl: *const lapack_int,
    ku: *const lapack_int,
    pack: *const c_char,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().clatms_.unwrap()(
        m, n, dist, iseed, sym, D, mode, cond, dmax, kl, ku, pack, A, lda, work, info,
    )
}

pub unsafe fn dlatms_(
    m: *const lapack_int,
    n: *const lapack_int,
    dist: *const c_char,
    iseed: *mut lapack_int,
    sym: *const c_char,
    D: *mut f64,
    mode: *const lapack_int,
    cond: *const f64,
    dmax: *const f64,
    kl: *const lapack_int,
    ku: *const lapack_int,
    pack: *const c_char,
    A: *mut f64,
    lda: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dlatms_.unwrap()(
        m, n, dist, iseed, sym, D, mode, cond, dmax, kl, ku, pack, A, lda, work, info,
    )
}

pub unsafe fn slatms_(
    m: *const lapack_int,
    n: *const lapack_int,
    dist: *const c_char,
    iseed: *mut lapack_int,
    sym: *const c_char,
    D: *mut f32,
    mode: *const lapack_int,
    cond: *const f32,
    dmax: *const f32,
    kl: *const lapack_int,
    ku: *const lapack_int,
    pack: *const c_char,
    A: *mut f32,
    lda: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().slatms_.unwrap()(
        m, n, dist, iseed, sym, D, mode, cond, dmax, kl, ku, pack, A, lda, work, info,
    )
}

pub unsafe fn zlatms_(
    m: *const lapack_int,
    n: *const lapack_int,
    dist: *const c_char,
    iseed: *mut lapack_int,
    sym: *const c_char,
    D: *mut f64,
    mode: *const lapack_int,
    cond: *const f64,
    dmax: *const f64,
    kl: *const lapack_int,
    ku: *const lapack_int,
    pack: *const c_char,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zlatms_.unwrap()(
        m, n, dist, iseed, sym, D, mode, cond, dmax, kl, ku, pack, A, lda, work, info,
    )
}

pub unsafe fn clauum_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().clauum_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn dlauum_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dlauum_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn slauum_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().slauum_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn zlauum_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zlauum_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn ilaver_(
    vers_major: *mut lapack_int,
    vers_minor: *mut lapack_int,
    vers_patch: *mut lapack_int,
) {
    dyload_lib().ilaver_.unwrap()(vers_major, vers_minor, vers_patch)
}

pub unsafe fn dopgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    tau: *const f64,
    Q: *mut f64,
    ldq: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dopgtr_.unwrap()(uplo, n, AP, tau, Q, ldq, work, info)
}

pub unsafe fn sopgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    tau: *const f32,
    Q: *mut f32,
    ldq: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sopgtr_.unwrap()(uplo, n, AP, tau, Q, ldq, work, info)
}

pub unsafe fn dopmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    AP: *const f64,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dopmtr_.unwrap()(side, uplo, trans, m, n, AP, tau, C, ldc, work, info)
}

pub unsafe fn sopmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    AP: *const f32,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sopmtr_.unwrap()(side, uplo, trans, m, n, AP, tau, C, ldc, work, info)
}

pub unsafe fn dorbdb_(
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f64,
    ldx11: *const lapack_int,
    X12: *mut f64,
    ldx12: *const lapack_int,
    X21: *mut f64,
    ldx21: *const lapack_int,
    X22: *mut f64,
    ldx22: *const lapack_int,
    theta: *mut f64,
    phi: *mut f64,
    TAUP1: *mut f64,
    TAUP2: *mut f64,
    TAUQ1: *mut f64,
    TAUQ2: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorbdb_.unwrap()(
        trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21, X22, ldx22, theta, phi, TAUP1,
        TAUP2, TAUQ1, TAUQ2, work, lwork, info,
    )
}

pub unsafe fn sorbdb_(
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f32,
    ldx11: *const lapack_int,
    X12: *mut f32,
    ldx12: *const lapack_int,
    X21: *mut f32,
    ldx21: *const lapack_int,
    X22: *mut f32,
    ldx22: *const lapack_int,
    theta: *mut f32,
    phi: *mut f32,
    TAUP1: *mut f32,
    TAUP2: *mut f32,
    TAUQ1: *mut f32,
    TAUQ2: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorbdb_.unwrap()(
        trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21, X22, ldx22, theta, phi, TAUP1,
        TAUP2, TAUQ1, TAUQ2, work, lwork, info,
    )
}

pub unsafe fn dorcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f64,
    ldx11: *const lapack_int,
    X12: *mut f64,
    ldx12: *const lapack_int,
    X21: *mut f64,
    ldx21: *const lapack_int,
    X22: *mut f64,
    ldx22: *const lapack_int,
    theta: *mut f64,
    U1: *mut f64,
    ldu1: *const lapack_int,
    U2: *mut f64,
    ldu2: *const lapack_int,
    V1T: *mut f64,
    ldv1t: *const lapack_int,
    V2T: *mut f64,
    ldv2t: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21,
        X22, ldx22, theta, U1, ldu1, U2, ldu2, V1T, ldv1t, V2T, ldv2t, work, lwork, iwork, info,
    )
}

pub unsafe fn sorcsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f32,
    ldx11: *const lapack_int,
    X12: *mut f32,
    ldx12: *const lapack_int,
    X21: *mut f32,
    ldx21: *const lapack_int,
    X22: *mut f32,
    ldx22: *const lapack_int,
    theta: *mut f32,
    U1: *mut f32,
    ldu1: *const lapack_int,
    U2: *mut f32,
    ldu2: *const lapack_int,
    V1T: *mut f32,
    ldv1t: *const lapack_int,
    V2T: *mut f32,
    ldv2t: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorcsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21,
        X22, ldx22, theta, U1, ldu1, U2, ldu2, V1T, ldv1t, V2T, ldv2t, work, lwork, iwork, info,
    )
}

pub unsafe fn dorcsd2by1_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f64,
    ldx11: *const lapack_int,
    X21: *mut f64,
    ldx21: *const lapack_int,
    theta: *mut f64,
    U1: *mut f64,
    ldu1: *const lapack_int,
    U2: *mut f64,
    ldu2: *const lapack_int,
    V1T: *mut f64,
    ldv1t: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorcsd2by1_.unwrap()(
        jobu1, jobu2, jobv1t, m, p, q, X11, ldx11, X21, ldx21, theta, U1, ldu1, U2, ldu2, V1T,
        ldv1t, work, lwork, iwork, info,
    )
}

pub unsafe fn sorcsd2by1_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut f32,
    ldx11: *const lapack_int,
    X21: *mut f32,
    ldx21: *const lapack_int,
    theta: *mut f32,
    U1: *mut f32,
    ldu1: *const lapack_int,
    U2: *mut f32,
    ldu2: *const lapack_int,
    V1T: *mut f32,
    ldv1t: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorcsd2by1_.unwrap()(
        jobu1, jobu2, jobv1t, m, p, q, X11, ldx11, X21, ldx21, theta, U1, ldu1, U2, ldu2, V1T,
        ldv1t, work, lwork, iwork, info,
    )
}

pub unsafe fn dorgbr_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgbr_.unwrap()(vect, m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorgbr_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgbr_.unwrap()(vect, m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorghr_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorghr_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorghr_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorghr_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorglq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorglq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorglq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorglq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorgql_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgql_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorgql_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgql_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorgqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgqr_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorgqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgqr_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorgrq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgrq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorgrq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgrq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *const f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgtr_.unwrap()(uplo, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn sorgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *const f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgtr_.unwrap()(uplo, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dorgtsqr_row_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dorgtsqr_row_.unwrap()(m, n, mb, nb, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn sorgtsqr_row_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sorgtsqr_row_.unwrap()(m, n, mb, nb, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn dorhr_col_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    D: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dorhr_col_.unwrap()(m, n, nb, A, lda, T, ldt, D, info)
}

pub unsafe fn sorhr_col_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    D: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sorhr_col_.unwrap()(m, n, nb, A, lda, T, ldt, D, info)
}

pub unsafe fn dormbr_(
    vect: *const c_char,
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormbr_.unwrap()(
        vect, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn sormbr_(
    vect: *const c_char,
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormbr_.unwrap()(
        vect, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn dormhr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormhr_.unwrap()(
        side, trans, m, n, ilo, ihi, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn sormhr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormhr_.unwrap()(
        side, trans, m, n, ilo, ihi, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn dormlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormlq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormlq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn dormql_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormql_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormql_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormql_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn dormqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormqr_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormqr_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn dormrq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormrq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormrq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormrq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn dormrz_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormrz_.unwrap()(side, trans, m, n, k, l, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormrz_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormrz_.unwrap()(side, trans, m, n, k, l, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn dormtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    tau: *const f64,
    C: *mut f64,
    ldc: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dormtr_.unwrap()(side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn sormtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    tau: *const f32,
    C: *mut f32,
    ldc: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sormtr_.unwrap()(side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cpbcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpbcon_.unwrap()(uplo, n, kd, AB, ldab, anorm, rcond, work, rwork, info)
}

pub unsafe fn dpbcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbcon_.unwrap()(uplo, n, kd, AB, ldab, anorm, rcond, work, iwork, info)
}

pub unsafe fn spbcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbcon_.unwrap()(uplo, n, kd, AB, ldab, anorm, rcond, work, iwork, info)
}

pub unsafe fn zpbcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpbcon_.unwrap()(uplo, n, kd, AB, ldab, anorm, rcond, work, rwork, info)
}

pub unsafe fn cpbequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpbequ_.unwrap()(uplo, n, kd, AB, ldab, S, scond, amax, info)
}

pub unsafe fn dpbequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpbequ_.unwrap()(uplo, n, kd, AB, ldab, S, scond, amax, info)
}

pub unsafe fn spbequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spbequ_.unwrap()(uplo, n, kd, AB, ldab, S, scond, amax, info)
}

pub unsafe fn zpbequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpbequ_.unwrap()(uplo, n, kd, AB, ldab, S, scond, amax, info)
}

pub unsafe fn cpbrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpbrfs_.unwrap()(
        uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dpbrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    AFB: *const f64,
    ldafb: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbrfs_.unwrap()(
        uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn spbrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    AFB: *const f32,
    ldafb: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbrfs_.unwrap()(
        uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zpbrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *const __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpbrfs_.unwrap()(
        uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cpbstf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpbstf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn dpbstf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbstf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn spbstf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbstf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn zpbstf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpbstf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn cpbsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpbsv_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn dpbsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbsv_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn spbsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbsv_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn zpbsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpbsv_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn cpbsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f32>,
    ldafb: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpbsvx_.unwrap()(
        fact, uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, equed, S, B, ldb, X, ldx, rcond, ferr, berr,
        work, rwork, info,
    )
}

pub unsafe fn dpbsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    AFB: *mut f64,
    ldafb: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbsvx_.unwrap()(
        fact, uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, equed, S, B, ldb, X, ldx, rcond, ferr, berr,
        work, iwork, info,
    )
}

pub unsafe fn spbsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    AFB: *mut f32,
    ldafb: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbsvx_.unwrap()(
        fact, uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, equed, S, B, ldb, X, ldx, rcond, ferr, berr,
        work, iwork, info,
    )
}

pub unsafe fn zpbsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    AFB: *mut __BindgenComplex<f64>,
    ldafb: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpbsvx_.unwrap()(
        fact, uplo, n, kd, nrhs, AB, ldab, AFB, ldafb, equed, S, B, ldb, X, ldx, rcond, ferr, berr,
        work, rwork, info,
    )
}

pub unsafe fn cpbtrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f32>,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpbtrf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn dpbtrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbtrf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn spbtrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbtrf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn zpbtrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut __BindgenComplex<f64>,
    ldab: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpbtrf_.unwrap()(uplo, n, kd, AB, ldab, info)
}

pub unsafe fn cpbtrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpbtrs_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn dpbtrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpbtrs_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn spbtrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spbtrs_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn zpbtrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpbtrs_.unwrap()(uplo, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn cpftrf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cpftrf_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn dpftrf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpftrf_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn spftrf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spftrf_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn zpftrf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zpftrf_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn cpftri_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cpftri_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn dpftri_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpftri_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn spftri_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spftri_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn zpftri_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zpftri_.unwrap()(transr, uplo, n, A, info)
}

pub unsafe fn cpftrs_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpftrs_.unwrap()(transr, uplo, n, nrhs, A, B, ldb, info)
}

pub unsafe fn dpftrs_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpftrs_.unwrap()(transr, uplo, n, nrhs, A, B, ldb, info)
}

pub unsafe fn spftrs_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spftrs_.unwrap()(transr, uplo, n, nrhs, A, B, ldb, info)
}

pub unsafe fn zpftrs_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpftrs_.unwrap()(transr, uplo, n, nrhs, A, B, ldb, info)
}

pub unsafe fn cpocon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpocon_.unwrap()(uplo, n, A, lda, anorm, rcond, work, rwork, info)
}

pub unsafe fn dpocon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpocon_.unwrap()(uplo, n, A, lda, anorm, rcond, work, iwork, info)
}

pub unsafe fn spocon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spocon_.unwrap()(uplo, n, A, lda, anorm, rcond, work, iwork, info)
}

pub unsafe fn zpocon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpocon_.unwrap()(uplo, n, A, lda, anorm, rcond, work, rwork, info)
}

pub unsafe fn cpoequ_(
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpoequ_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn dpoequ_(
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpoequ_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn spoequ_(
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spoequ_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn zpoequ_(
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpoequ_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn cpoequb_(
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpoequb_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn dpoequb_(
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpoequb_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn spoequb_(
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spoequb_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn zpoequb_(
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpoequb_.unwrap()(n, A, lda, S, scond, amax, info)
}

pub unsafe fn cporfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cporfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dporfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dporfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn sporfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sporfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zporfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zporfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cporfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    S: *const f32,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cporfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dporfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    S: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dporfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sporfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    S: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sporfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zporfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    S: *const f64,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zporfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cposv_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn dposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dposv_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn sposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sposv_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn zposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zposv_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn dsposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    work: *mut f64,
    swork: *mut f32,
    iter: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsposv_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, X, ldx, work, swork, iter, info)
}

pub unsafe fn zcposv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    swork: *mut __BindgenComplex<f32>,
    rwork: *mut f64,
    iter: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zcposv_.unwrap()(
        uplo, n, nrhs, A, lda, B, ldb, X, ldx, work, swork, rwork, iter, info,
    )
}

pub unsafe fn cposvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cposvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn dposvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dposvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn sposvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sposvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work,
        iwork, info,
    )
}

pub unsafe fn zposvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zposvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work,
        rwork, info,
    )
}

pub unsafe fn cposvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cposvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dposvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dposvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn sposvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sposvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zposvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zposvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn cpotf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpotf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn dpotf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpotf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn spotf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spotf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn zpotf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpotf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn cpotrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpotrf_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn dpotrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpotrf_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn spotrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spotrf_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn zpotrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpotrf_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn cpotrf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpotrf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn dpotrf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpotrf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn spotrf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spotrf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn zpotrf2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpotrf2_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn cpotri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpotri_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn dpotri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpotri_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn spotri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spotri_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn zpotri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpotri_.unwrap()(uplo, n, A, lda, info)
}

pub unsafe fn cpotrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpotrs_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn dpotrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpotrs_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn spotrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spotrs_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn zpotrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpotrs_.unwrap()(uplo, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn cppcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cppcon_.unwrap()(uplo, n, AP, anorm, rcond, work, rwork, info)
}

pub unsafe fn dppcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dppcon_.unwrap()(uplo, n, AP, anorm, rcond, work, iwork, info)
}

pub unsafe fn sppcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sppcon_.unwrap()(uplo, n, AP, anorm, rcond, work, iwork, info)
}

pub unsafe fn zppcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zppcon_.unwrap()(uplo, n, AP, anorm, rcond, work, rwork, info)
}

pub unsafe fn cppequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cppequ_.unwrap()(uplo, n, AP, S, scond, amax, info)
}

pub unsafe fn dppequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dppequ_.unwrap()(uplo, n, AP, S, scond, amax, info)
}

pub unsafe fn sppequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sppequ_.unwrap()(uplo, n, AP, S, scond, amax, info)
}

pub unsafe fn zppequ_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zppequ_.unwrap()(uplo, n, AP, S, scond, amax, info)
}

pub unsafe fn cpprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    AFP: *const __BindgenComplex<f32>,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dpprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    AFP: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn spprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    AFP: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zpprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    AFP: *const __BindgenComplex<f64>,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cppsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cppsv_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn dppsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dppsv_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn sppsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sppsv_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn zppsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zppsv_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn cppsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    AFP: *mut __BindgenComplex<f32>,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cppsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work, rwork,
        info,
    )
}

pub unsafe fn dppsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f64,
    AFP: *mut f64,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dppsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work, iwork,
        info,
    )
}

pub unsafe fn sppsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f32,
    AFP: *mut f32,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sppsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work, iwork,
        info,
    )
}

pub unsafe fn zppsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    AFP: *mut __BindgenComplex<f64>,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zppsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, equed, S, B, ldb, X, ldx, rcond, ferr, berr, work, rwork,
        info,
    )
}

pub unsafe fn cpptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cpptrf_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn dpptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpptrf_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn spptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spptrf_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn zpptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zpptrf_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn cpptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cpptri_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn dpptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpptri_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn spptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spptri_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn zpptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zpptri_.unwrap()(uplo, n, AP, info)
}

pub unsafe fn cpptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpptrs_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn dpptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpptrs_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn spptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spptrs_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn zpptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpptrs_.unwrap()(uplo, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn cpstrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    piv: *mut lapack_int,
    rank: *mut lapack_int,
    tol: *const f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpstrf_.unwrap()(uplo, n, A, lda, piv, rank, tol, work, info)
}

pub unsafe fn dpstrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    piv: *mut lapack_int,
    rank: *mut lapack_int,
    tol: *const f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpstrf_.unwrap()(uplo, n, A, lda, piv, rank, tol, work, info)
}

pub unsafe fn spstrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    piv: *mut lapack_int,
    rank: *mut lapack_int,
    tol: *const f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spstrf_.unwrap()(uplo, n, A, lda, piv, rank, tol, work, info)
}

pub unsafe fn zpstrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    piv: *mut lapack_int,
    rank: *mut lapack_int,
    tol: *const f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpstrf_.unwrap()(uplo, n, A, lda, piv, rank, tol, work, info)
}

pub unsafe fn cptcon_(
    n: *const lapack_int,
    D: *const f32,
    E: *const __BindgenComplex<f32>,
    anorm: *const f32,
    rcond: *mut f32,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cptcon_.unwrap()(n, D, E, anorm, rcond, rwork, info)
}

pub unsafe fn dptcon_(
    n: *const lapack_int,
    D: *const f64,
    E: *const f64,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dptcon_.unwrap()(n, D, E, anorm, rcond, work, info)
}

pub unsafe fn sptcon_(
    n: *const lapack_int,
    D: *const f32,
    E: *const f32,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sptcon_.unwrap()(n, D, E, anorm, rcond, work, info)
}

pub unsafe fn zptcon_(
    n: *const lapack_int,
    D: *const f64,
    E: *const __BindgenComplex<f64>,
    anorm: *const f64,
    rcond: *mut f64,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zptcon_.unwrap()(n, D, E, anorm, rcond, rwork, info)
}

pub unsafe fn cpteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cpteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn dpteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dpteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn spteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().spteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn zpteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zpteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn cptrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const __BindgenComplex<f32>,
    DF: *const f32,
    EF: *const __BindgenComplex<f32>,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cptrfs_.unwrap()(
        uplo, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dptrfs_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const f64,
    DF: *const f64,
    EF: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dptrfs_.unwrap()(n, nrhs, D, E, DF, EF, B, ldb, X, ldx, ferr, berr, work, info)
}

pub unsafe fn sptrfs_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const f32,
    DF: *const f32,
    EF: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sptrfs_.unwrap()(n, nrhs, D, E, DF, EF, B, ldb, X, ldx, ferr, berr, work, info)
}

pub unsafe fn zptrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const __BindgenComplex<f64>,
    DF: *const f64,
    EF: *const __BindgenComplex<f64>,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zptrfs_.unwrap()(
        uplo, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cptsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *mut f32,
    E: *mut __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cptsv_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn dptsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dptsv_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn sptsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sptsv_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn zptsv_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *mut f64,
    E: *mut __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zptsv_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn cptsvx_(
    fact: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const __BindgenComplex<f32>,
    DF: *mut f32,
    EF: *mut __BindgenComplex<f32>,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cptsvx_.unwrap()(
        fact, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dptsvx_(
    fact: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const f64,
    DF: *mut f64,
    EF: *mut f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dptsvx_.unwrap()(
        fact, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, rcond, ferr, berr, work, info,
    )
}

pub unsafe fn sptsvx_(
    fact: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const f32,
    DF: *mut f32,
    EF: *mut f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sptsvx_.unwrap()(
        fact, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, rcond, ferr, berr, work, info,
    )
}

pub unsafe fn zptsvx_(
    fact: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const __BindgenComplex<f64>,
    DF: *mut f64,
    EF: *mut __BindgenComplex<f64>,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zptsvx_.unwrap()(
        fact, n, nrhs, D, E, DF, EF, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cpttrf_(
    n: *const lapack_int,
    D: *mut f32,
    E: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cpttrf_.unwrap()(n, D, E, info)
}

pub unsafe fn dpttrf_(n: *const lapack_int, D: *mut f64, E: *mut f64, info: *mut lapack_int) {
    dyload_lib().dpttrf_.unwrap()(n, D, E, info)
}

pub unsafe fn spttrf_(n: *const lapack_int, D: *mut f32, E: *mut f32, info: *mut lapack_int) {
    dyload_lib().spttrf_.unwrap()(n, D, E, info)
}

pub unsafe fn zpttrf_(
    n: *const lapack_int,
    D: *mut f64,
    E: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zpttrf_.unwrap()(n, D, E, info)
}

pub unsafe fn cpttrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cpttrs_.unwrap()(uplo, n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn dpttrs_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dpttrs_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn spttrs_(
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f32,
    E: *const f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().spttrs_.unwrap()(n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn zpttrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    D: *const f64,
    E: *const __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zpttrs_.unwrap()(uplo, n, nrhs, D, E, B, ldb, info)
}

pub unsafe fn dsbev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsbev_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, info)
}

pub unsafe fn ssbev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssbev_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, info)
}

pub unsafe fn dsbev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbev_2stage_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, info)
}

pub unsafe fn ssbev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbev_2stage_.unwrap()(jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, info)
}

pub unsafe fn dsbevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbevd_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ssbevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbevd_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dsbevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbevd_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ssbevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbevd_2stage_.unwrap()(
        jobz, uplo, n, kd, AB, ldab, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dsbevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbevx_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        iwork, IFAIL, info,
    )
}

pub unsafe fn ssbevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbevx_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        iwork, IFAIL, info,
    )
}

pub unsafe fn dsbevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbevx_2stage_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, iwork, IFAIL, info,
    )
}

pub unsafe fn ssbevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbevx_2stage_.unwrap()(
        jobz, range, uplo, n, kd, AB, ldab, Q, ldq, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, iwork, IFAIL, info,
    )
}

pub unsafe fn dsbgst_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    BB: *const f64,
    ldbb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsbgst_.unwrap()(vect, uplo, n, ka, kb, AB, ldab, BB, ldbb, X, ldx, work, info)
}

pub unsafe fn ssbgst_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    BB: *const f32,
    ldbb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssbgst_.unwrap()(vect, uplo, n, ka, kb, AB, ldab, BB, ldbb, X, ldx, work, info)
}

pub unsafe fn dsbgv_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    BB: *mut f64,
    ldbb: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsbgv_.unwrap()(jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, info)
}

pub unsafe fn ssbgv_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    BB: *mut f32,
    ldbb: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssbgv_.unwrap()(jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, info)
}

pub unsafe fn dsbgvd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    BB: *mut f64,
    ldbb: *const lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbgvd_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ssbgvd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    BB: *mut f32,
    ldbb: *const lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbgvd_.unwrap()(
        jobz, uplo, n, ka, kb, AB, ldab, BB, ldbb, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dsbgvx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    BB: *mut f64,
    ldbb: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsbgvx_.unwrap()(
        jobz, range, uplo, n, ka, kb, AB, ldab, BB, ldbb, Q, ldq, vl, vu, il, iu, abstol, m, W, Z,
        ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn ssbgvx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ka: *const lapack_int,
    kb: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    BB: *mut f32,
    ldbb: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssbgvx_.unwrap()(
        jobz, range, uplo, n, ka, kb, AB, ldab, BB, ldbb, Q, ldq, vl, vu, il, iu, abstol, m, W, Z,
        ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn dsbtrd_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f64,
    ldab: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Q: *mut f64,
    ldq: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsbtrd_.unwrap()(vect, uplo, n, kd, AB, ldab, D, E, Q, ldq, work, info)
}

pub unsafe fn ssbtrd_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *mut f32,
    ldab: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Q: *mut f32,
    ldq: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssbtrd_.unwrap()(vect, uplo, n, kd, AB, ldab, D, E, Q, ldq, work, info)
}

pub unsafe fn dsfrk_(
    transr: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    alpha: *const f64,
    A: *const f64,
    lda: *const lapack_int,
    beta: *const f64,
    C: *mut f64,
) {
    dyload_lib().dsfrk_.unwrap()(transr, uplo, trans, n, k, alpha, A, lda, beta, C)
}

pub unsafe fn ssfrk_(
    transr: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    n: *const lapack_int,
    k: *const lapack_int,
    alpha: *const f32,
    A: *const f32,
    lda: *const lapack_int,
    beta: *const f32,
    C: *mut f32,
) {
    dyload_lib().ssfrk_.unwrap()(transr, uplo, trans, n, k, alpha, A, lda, beta, C)
}

pub unsafe fn cspcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cspcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, info)
}

pub unsafe fn dspcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn sspcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn zspcon_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zspcon_.unwrap()(uplo, n, AP, ipiv, anorm, rcond, work, info)
}

pub unsafe fn dspev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dspev_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, info)
}

pub unsafe fn sspev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sspev_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, info)
}

pub unsafe fn dspevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspevd_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn sspevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspevd_.unwrap()(jobz, uplo, n, AP, W, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn dspevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspevx_.unwrap()(
        jobz, range, uplo, n, AP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn sspevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspevx_.unwrap()(
        jobz, range, uplo, n, AP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn dspgst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    BP: *const f64,
    info: *mut lapack_int,
) {
    dyload_lib().dspgst_.unwrap()(itype, uplo, n, AP, BP, info)
}

pub unsafe fn sspgst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    BP: *const f32,
    info: *mut lapack_int,
) {
    dyload_lib().sspgst_.unwrap()(itype, uplo, n, AP, BP, info)
}

pub unsafe fn dspgv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    BP: *mut f64,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dspgv_.unwrap()(itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, info)
}

pub unsafe fn sspgv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    BP: *mut f32,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sspgv_.unwrap()(itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, info)
}

pub unsafe fn dspgvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    BP: *mut f64,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspgvd_.unwrap()(
        itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn sspgvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    BP: *mut f32,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspgvd_.unwrap()(
        itype, jobz, uplo, n, AP, BP, W, Z, ldz, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dspgvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    BP: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspgvx_.unwrap()(
        itype, jobz, range, uplo, n, AP, BP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork,
        IFAIL, info,
    )
}

pub unsafe fn sspgvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    BP: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspgvx_.unwrap()(
        itype, jobz, range, uplo, n, AP, BP, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork,
        IFAIL, info,
    )
}

pub unsafe fn csprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    AFP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dsprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    AFP: *const f64,
    ipiv: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn ssprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    AFP: *const f32,
    ipiv: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zsprfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    AFP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsprfs_.unwrap()(
        uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn cspsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cspsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn dspsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f64,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn sspsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut f32,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn zspsv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zspsv_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn cspsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    AFP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().cspsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dspsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    AFP: *mut f64,
    ipiv: *mut lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dspsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn sspsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    AFP: *mut f32,
    ipiv: *mut lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sspsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zspsvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    AFP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zspsvx_.unwrap()(
        fact, uplo, n, nrhs, AP, AFP, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dsptrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    D: *mut f64,
    E: *mut f64,
    tau: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsptrd_.unwrap()(uplo, n, AP, D, E, tau, info)
}

pub unsafe fn ssptrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    D: *mut f32,
    E: *mut f32,
    tau: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssptrd_.unwrap()(uplo, n, AP, D, E, tau, info)
}

pub unsafe fn csptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn dsptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn ssptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn zsptrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsptrf_.unwrap()(uplo, n, AP, ipiv, info)
}

pub unsafe fn csptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn dsptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    ipiv: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn ssptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    ipiv: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn zsptri_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsptri_.unwrap()(uplo, n, AP, ipiv, work, info)
}

pub unsafe fn csptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn dsptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn ssptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn zsptrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsptrs_.unwrap()(uplo, n, nrhs, AP, ipiv, B, ldb, info)
}

pub unsafe fn dstebz_(
    range: *const c_char,
    order: *const c_char,
    n: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    D: *const f64,
    E: *const f64,
    m: *mut lapack_int,
    nsplit: *mut lapack_int,
    W: *mut f64,
    IBLOCK: *mut lapack_int,
    ISPLIT: *mut lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstebz_.unwrap()(
        range, order, n, vl, vu, il, iu, abstol, D, E, m, nsplit, W, IBLOCK, ISPLIT, work, iwork,
        info,
    )
}

pub unsafe fn sstebz_(
    range: *const c_char,
    order: *const c_char,
    n: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    D: *const f32,
    E: *const f32,
    m: *mut lapack_int,
    nsplit: *mut lapack_int,
    W: *mut f32,
    IBLOCK: *mut lapack_int,
    ISPLIT: *mut lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstebz_.unwrap()(
        range, order, n, vl, vu, il, iu, abstol, D, E, m, nsplit, W, IBLOCK, ISPLIT, work, iwork,
        info,
    )
}

pub unsafe fn cstedc_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cstedc_.unwrap()(
        compz, n, D, E, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn dstedc_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstedc_.unwrap()(compz, n, D, E, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn sstedc_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstedc_.unwrap()(compz, n, D, E, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn zstedc_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zstedc_.unwrap()(
        compz, n, D, E, Z, ldz, work, lwork, rwork, lrwork, iwork, liwork, info,
    )
}

pub unsafe fn cstegr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cstegr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn dstegr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstegr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn sstegr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstegr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn zstegr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zstegr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn cstein_(
    n: *const lapack_int,
    D: *const f32,
    E: *const f32,
    m: *const lapack_int,
    W: *const f32,
    IBLOCK: *const lapack_int,
    ISPLIT: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cstein_.unwrap()(n, D, E, m, W, IBLOCK, ISPLIT, Z, ldz, work, iwork, IFAIL, info)
}

pub unsafe fn dstein_(
    n: *const lapack_int,
    D: *const f64,
    E: *const f64,
    m: *const lapack_int,
    W: *const f64,
    IBLOCK: *const lapack_int,
    ISPLIT: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstein_.unwrap()(n, D, E, m, W, IBLOCK, ISPLIT, Z, ldz, work, iwork, IFAIL, info)
}

pub unsafe fn sstein_(
    n: *const lapack_int,
    D: *const f32,
    E: *const f32,
    m: *const lapack_int,
    W: *const f32,
    IBLOCK: *const lapack_int,
    ISPLIT: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstein_.unwrap()(n, D, E, m, W, IBLOCK, ISPLIT, Z, ldz, work, iwork, IFAIL, info)
}

pub unsafe fn zstein_(
    n: *const lapack_int,
    D: *const f64,
    E: *const f64,
    m: *const lapack_int,
    W: *const f64,
    IBLOCK: *const lapack_int,
    ISPLIT: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zstein_.unwrap()(n, D, E, m, W, IBLOCK, ISPLIT, Z, ldz, work, iwork, IFAIL, info)
}

pub unsafe fn cstemr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    nzc: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    tryrac: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cstemr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, m, W, Z, ldz, nzc, ISUPPZ, tryrac, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn dstemr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    nzc: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    tryrac: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstemr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, m, W, Z, ldz, nzc, ISUPPZ, tryrac, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn sstemr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    nzc: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    tryrac: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstemr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, m, W, Z, ldz, nzc, ISUPPZ, tryrac, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn zstemr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    nzc: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    tryrac: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zstemr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, m, W, Z, ldz, nzc, ISUPPZ, tryrac, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn csteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn dsteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn ssteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn zsteqr_(
    compz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsteqr_.unwrap()(compz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn dsterf_(n: *const lapack_int, D: *mut f64, E: *mut f64, info: *mut lapack_int) {
    dyload_lib().dsterf_.unwrap()(n, D, E, info)
}

pub unsafe fn ssterf_(n: *const lapack_int, D: *mut f32, E: *mut f32, info: *mut lapack_int) {
    dyload_lib().ssterf_.unwrap()(n, D, E, info)
}

pub unsafe fn dstev_(
    jobz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dstev_.unwrap()(jobz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn sstev_(
    jobz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().sstev_.unwrap()(jobz, n, D, E, Z, ldz, work, info)
}

pub unsafe fn dstevd_(
    jobz: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstevd_.unwrap()(jobz, n, D, E, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn sstevd_(
    jobz: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstevd_.unwrap()(jobz, n, D, E, Z, ldz, work, lwork, iwork, liwork, info)
}

pub unsafe fn dstevr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstevr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn sstevr_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstevr_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork, iwork,
        liwork, info,
    )
}

pub unsafe fn dstevx_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dstevx_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn sstevx_(
    jobz: *const c_char,
    range: *const c_char,
    n: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().sstevx_.unwrap()(
        jobz, range, n, D, E, vl, vu, il, iu, abstol, m, W, Z, ldz, work, iwork, IFAIL, info,
    )
}

pub unsafe fn csycon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csycon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, info)
}

pub unsafe fn dsycon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsycon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn ssycon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssycon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn zsycon_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsycon_.unwrap()(uplo, n, A, lda, ipiv, anorm, rcond, work, info)
}

pub unsafe fn csycon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csycon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, info)
}

pub unsafe fn dsycon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    E: *const f64,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsycon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn ssycon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    E: *const f32,
    ipiv: *const lapack_int,
    anorm: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssycon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, iwork, info)
}

pub unsafe fn zsycon_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    anorm: *const f64,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsycon_3_.unwrap()(uplo, n, A, lda, E, ipiv, anorm, rcond, work, info)
}

pub unsafe fn csyconv_(
    uplo: *const c_char,
    way: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    E: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csyconv_.unwrap()(uplo, way, n, A, lda, ipiv, E, info)
}

pub unsafe fn dsyconv_(
    uplo: *const c_char,
    way: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    E: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsyconv_.unwrap()(uplo, way, n, A, lda, ipiv, E, info)
}

pub unsafe fn ssyconv_(
    uplo: *const c_char,
    way: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    E: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssyconv_.unwrap()(uplo, way, n, A, lda, ipiv, E, info)
}

pub unsafe fn zsyconv_(
    uplo: *const c_char,
    way: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    E: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsyconv_.unwrap()(uplo, way, n, A, lda, ipiv, E, info)
}

pub unsafe fn csyequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csyequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn dsyequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsyequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn ssyequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    S: *mut f32,
    scond: *mut f32,
    amax: *mut f32,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssyequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn zsyequb_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    S: *mut f64,
    scond: *mut f64,
    amax: *mut f64,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsyequb_.unwrap()(uplo, n, A, lda, S, scond, amax, work, info)
}

pub unsafe fn dsyev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyev_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, info)
}

pub unsafe fn ssyev_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyev_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, info)
}

pub unsafe fn dsyev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyev_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, info)
}

pub unsafe fn ssyev_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyev_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, info)
}

pub unsafe fn dsyevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevd_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, iwork, liwork, info)
}

pub unsafe fn ssyevd_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevd_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, iwork, liwork, info)
}

pub unsafe fn dsyevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevd_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, iwork, liwork, info)
}

pub unsafe fn ssyevd_2stage_(
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevd_2stage_.unwrap()(jobz, uplo, n, A, lda, W, work, lwork, iwork, liwork, info)
}

pub unsafe fn dsyevr_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevr_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn ssyevr_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevr_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn dsyevr_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevr_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn ssyevr_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    ISUPPZ: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevr_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, ISUPPZ, work, lwork,
        iwork, liwork, info,
    )
}

pub unsafe fn dsyevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevx_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, iwork,
        IFAIL, info,
    )
}

pub unsafe fn ssyevx_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevx_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, iwork,
        IFAIL, info,
    )
}

pub unsafe fn dsyevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyevx_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, iwork,
        IFAIL, info,
    )
}

pub unsafe fn ssyevx_2stage_(
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyevx_2stage_.unwrap()(
        jobz, range, uplo, n, A, lda, vl, vu, il, iu, abstol, m, W, Z, ldz, work, lwork, iwork,
        IFAIL, info,
    )
}

pub unsafe fn dsygst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsygst_.unwrap()(itype, uplo, n, A, lda, B, ldb, info)
}

pub unsafe fn ssygst_(
    itype: *const lapack_int,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssygst_.unwrap()(itype, uplo, n, A, lda, B, ldb, info)
}

pub unsafe fn dsygv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsygv_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info)
}

pub unsafe fn ssygv_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssygv_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info)
}

pub unsafe fn dsygv_2stage_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsygv_2stage_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info)
}

pub unsafe fn ssygv_2stage_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssygv_2stage_.unwrap()(itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, info)
}

pub unsafe fn dsygvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    W: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsygvd_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ssygvd_(
    itype: *const lapack_int,
    jobz: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    W: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssygvd_.unwrap()(
        itype, jobz, uplo, n, A, lda, B, ldb, W, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dsygvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    vl: *const f64,
    vu: *const f64,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f64,
    m: *mut lapack_int,
    W: *mut f64,
    Z: *mut f64,
    ldz: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsygvx_.unwrap()(
        itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, iwork, IFAIL, info,
    )
}

pub unsafe fn ssygvx_(
    itype: *const lapack_int,
    jobz: *const c_char,
    range: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    vl: *const f32,
    vu: *const f32,
    il: *const lapack_int,
    iu: *const lapack_int,
    abstol: *const f32,
    m: *mut lapack_int,
    W: *mut f32,
    Z: *mut f32,
    ldz: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    IFAIL: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssygvx_.unwrap()(
        itype, jobz, range, uplo, n, A, lda, B, ldb, vl, vu, il, iu, abstol, m, W, Z, ldz, work,
        lwork, iwork, IFAIL, info,
    )
}

pub unsafe fn csyr_(
    uplo: *const c_char,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f32>,
    X: *const __BindgenComplex<f32>,
    incx: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
) {
    dyload_lib().csyr_.unwrap()(uplo, n, alpha, X, incx, A, lda)
}

pub unsafe fn zsyr_(
    uplo: *const c_char,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f64>,
    X: *const __BindgenComplex<f64>,
    incx: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
) {
    dyload_lib().zsyr_.unwrap()(uplo, n, alpha, X, incx, A, lda)
}

pub unsafe fn csyrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csyrfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dsyrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyrfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn ssyrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyrfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn zsyrfs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsyrfs_.unwrap()(
        uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn csyrfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f32,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csyrfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dsyrfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *const f64,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsyrfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn ssyrfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *const f32,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssyrfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zsyrfsx_(
    uplo: *const c_char,
    equed: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *const __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *const lapack_int,
    S: *const f64,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsyrfsx_.unwrap()(
        uplo,
        equed,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn csysv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csysv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn dsysv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn ssysv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zsysv_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsysv_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn csysv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csysv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn dsysv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn ssysv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zsysv_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsysv_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn csysv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csysv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn dsysv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    TB: *mut f64,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn ssysv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    TB: *mut f32,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn zsysv_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsysv_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, work, lwork, info,
    )
}

pub unsafe fn csysv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csysv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn dsysv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    E: *mut f64,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn ssysv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    E: *mut f32,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zsysv_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsysv_rk_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn csysv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csysv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn dsysv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn ssysv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zsysv_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsysv_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn csysvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csysvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, rwork, info,
    )
}

pub unsafe fn dsysvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, iwork, info,
    )
}

pub unsafe fn ssysvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, iwork, info,
    )
}

pub unsafe fn zsysvx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsysvx_.unwrap()(
        fact, uplo, n, nrhs, A, lda, AF, ldaf, ipiv, B, ldb, X, ldx, rcond, ferr, berr, work,
        lwork, rwork, info,
    )
}

pub unsafe fn csysvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f32>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f32>,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().csysvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn dsysvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    AF: *mut f64,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut f64,
    ldb: *const lapack_int,
    X: *mut f64,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsysvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn ssysvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    AF: *mut f32,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f32,
    B: *mut f32,
    ldb: *const lapack_int,
    X: *mut f32,
    ldx: *const lapack_int,
    rcond: *mut f32,
    rpvgrw: *mut f32,
    berr: *mut f32,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f32,
    err_bnds_comp: *mut f32,
    nparams: *const lapack_int,
    params: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssysvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        iwork,
        info,
    )
}

pub unsafe fn zsysvxx_(
    fact: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    AF: *mut __BindgenComplex<f64>,
    ldaf: *const lapack_int,
    ipiv: *mut lapack_int,
    equed: *mut c_char,
    S: *mut f64,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *mut __BindgenComplex<f64>,
    ldx: *const lapack_int,
    rcond: *mut f64,
    rpvgrw: *mut f64,
    berr: *mut f64,
    n_err_bnds: *const lapack_int,
    err_bnds_norm: *mut f64,
    err_bnds_comp: *mut f64,
    nparams: *const lapack_int,
    params: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().zsysvxx_.unwrap()(
        fact,
        uplo,
        n,
        nrhs,
        A,
        lda,
        AF,
        ldaf,
        ipiv,
        equed,
        S,
        B,
        ldb,
        X,
        ldx,
        rcond,
        rpvgrw,
        berr,
        n_err_bnds,
        err_bnds_norm,
        err_bnds_comp,
        nparams,
        params,
        work,
        rwork,
        info,
    )
}

pub unsafe fn csyswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().csyswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn dsyswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().dsyswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn ssyswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().ssyswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn zsyswapr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    i1: *const lapack_int,
    i2: *const lapack_int,
) {
    dyload_lib().zsyswapr_.unwrap()(uplo, n, A, lda, i1, i2)
}

pub unsafe fn dsytrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrd_.unwrap()(uplo, n, A, lda, D, E, tau, work, lwork, info)
}

pub unsafe fn ssytrd_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrd_.unwrap()(uplo, n, A, lda, D, E, tau, work, lwork, info)
}

pub unsafe fn dsytrd_2stage_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    D: *mut f64,
    E: *mut f64,
    tau: *mut f64,
    HOUS2: *mut f64,
    lhous2: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrd_2stage_.unwrap()(
        vect, uplo, n, A, lda, D, E, tau, HOUS2, lhous2, work, lwork, info,
    )
}

pub unsafe fn ssytrd_2stage_(
    vect: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    D: *mut f32,
    E: *mut f32,
    tau: *mut f32,
    HOUS2: *mut f32,
    lhous2: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrd_2stage_.unwrap()(
        vect, uplo, n, A, lda, D, E, tau, HOUS2, lhous2, work, lwork, info,
    )
}

pub unsafe fn csytrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn dsytrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn ssytrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zsytrf_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrf_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn csytrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn dsytrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn ssytrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zsytrf_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrf_aa_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn csytrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn dsytrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    TB: *mut f64,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn ssytrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    TB: *mut f32,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn zsytrf_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *mut lapack_int,
    ipiv2: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrf_aa_2stage_.unwrap()(
        uplo, n, A, lda, TB, ltb, ipiv, ipiv2, work, lwork, info,
    )
}

pub unsafe fn csytrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f32>,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn dsytrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    E: *mut f64,
    ipiv: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn ssytrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    E: *mut f32,
    ipiv: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn zsytrf_rk_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *mut __BindgenComplex<f64>,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrf_rk_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn csytrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn dsytrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn ssytrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zsytrf_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrf_rook_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn csytri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csytri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn dsytri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsytri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn ssytri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssytri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn zsytri_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsytri_.unwrap()(uplo, n, A, lda, ipiv, work, info)
}

pub unsafe fn csytri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn dsytri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn ssytri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn zsytri2_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytri2_.unwrap()(uplo, n, A, lda, ipiv, work, lwork, info)
}

pub unsafe fn csytri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn dsytri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f64,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn ssytri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut f32,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn zsytri2x_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    nb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytri2x_.unwrap()(uplo, n, A, lda, ipiv, work, nb, info)
}

pub unsafe fn csytri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn dsytri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    E: *const f64,
    ipiv: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn ssytri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    E: *const f32,
    ipiv: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn zsytri_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytri_3_.unwrap()(uplo, n, A, lda, E, ipiv, work, lwork, info)
}

pub unsafe fn csytrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn dsytrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn ssytrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zsytrs_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn csytrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn dsytrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn ssytrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn zsytrs2_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs2_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, info)
}

pub unsafe fn csytrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn dsytrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    E: *const f64,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn ssytrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    E: *const f32,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn zsytrs_3_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs_3_.unwrap()(uplo, n, nrhs, A, lda, E, ipiv, B, ldb, info)
}

pub unsafe fn csytrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn dsytrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn ssytrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn zsytrs_aa_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs_aa_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, work, lwork, info)
}

pub unsafe fn csytrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f32>,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn dsytrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    TB: *mut f64,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn ssytrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    TB: *mut f32,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn zsytrs_aa_2stage_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    TB: *mut __BindgenComplex<f64>,
    ltb: *const lapack_int,
    ipiv: *const lapack_int,
    ipiv2: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs_aa_2stage_.unwrap()(
        uplo, n, nrhs, A, lda, TB, ltb, ipiv, ipiv2, B, ldb, info,
    )
}

pub unsafe fn csytrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().csytrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn dsytrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dsytrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn ssytrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ssytrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn zsytrs_rook_(
    uplo: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ipiv: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zsytrs_rook_.unwrap()(uplo, n, nrhs, A, lda, ipiv, B, ldb, info)
}

pub unsafe fn ctbcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctbcon_.unwrap()(norm, uplo, diag, n, kd, AB, ldab, rcond, work, rwork, info)
}

pub unsafe fn dtbcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtbcon_.unwrap()(norm, uplo, diag, n, kd, AB, ldab, rcond, work, iwork, info)
}

pub unsafe fn stbcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stbcon_.unwrap()(norm, uplo, diag, n, kd, AB, ldab, rcond, work, iwork, info)
}

pub unsafe fn ztbcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztbcon_.unwrap()(norm, uplo, diag, n, kd, AB, ldab, rcond, work, rwork, info)
}

pub unsafe fn ctbrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctbrfs_.unwrap()(
        uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dtbrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *const f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtbrfs_.unwrap()(
        uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn stbrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *const f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stbrfs_.unwrap()(
        uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn ztbrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztbrfs_.unwrap()(
        uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn ctbtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f32>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctbtrs_.unwrap()(uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn dtbtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f64,
    ldab: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtbtrs_.unwrap()(uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn stbtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const f32,
    ldab: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stbtrs_.unwrap()(uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn ztbtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    kd: *const lapack_int,
    nrhs: *const lapack_int,
    AB: *const __BindgenComplex<f64>,
    ldab: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztbtrs_.unwrap()(uplo, trans, diag, n, kd, nrhs, AB, ldab, B, ldb, info)
}

pub unsafe fn ctfsm_(
    transr: *const c_char,
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f32>,
    A: *const __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
) {
    dyload_lib().ctfsm_.unwrap()(transr, side, uplo, trans, diag, m, n, alpha, A, B, ldb)
}

pub unsafe fn dtfsm_(
    transr: *const c_char,
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const f64,
    A: *const f64,
    B: *mut f64,
    ldb: *const lapack_int,
) {
    dyload_lib().dtfsm_.unwrap()(transr, side, uplo, trans, diag, m, n, alpha, A, B, ldb)
}

pub unsafe fn stfsm_(
    transr: *const c_char,
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const f32,
    A: *const f32,
    B: *mut f32,
    ldb: *const lapack_int,
) {
    dyload_lib().stfsm_.unwrap()(transr, side, uplo, trans, diag, m, n, alpha, A, B, ldb)
}

pub unsafe fn ztfsm_(
    transr: *const c_char,
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    alpha: *const __BindgenComplex<f64>,
    A: *const __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
) {
    dyload_lib().ztfsm_.unwrap()(transr, side, uplo, trans, diag, m, n, alpha, A, B, ldb)
}

pub unsafe fn ctftri_(
    transr: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctftri_.unwrap()(transr, uplo, diag, n, A, info)
}

pub unsafe fn dtftri_(
    transr: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtftri_.unwrap()(transr, uplo, diag, n, A, info)
}

pub unsafe fn stftri_(
    transr: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stftri_.unwrap()(transr, uplo, diag, n, A, info)
}

pub unsafe fn ztftri_(
    transr: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztftri_.unwrap()(transr, uplo, diag, n, A, info)
}

pub unsafe fn ctfttp_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const __BindgenComplex<f32>,
    AP: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctfttp_.unwrap()(transr, uplo, n, ARF, AP, info)
}

pub unsafe fn dtfttp_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const f64,
    AP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtfttp_.unwrap()(transr, uplo, n, ARF, AP, info)
}

pub unsafe fn stfttp_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const f32,
    AP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stfttp_.unwrap()(transr, uplo, n, ARF, AP, info)
}

pub unsafe fn ztfttp_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const __BindgenComplex<f64>,
    AP: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztfttp_.unwrap()(transr, uplo, n, ARF, AP, info)
}

pub unsafe fn ctfttr_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const __BindgenComplex<f32>,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctfttr_.unwrap()(transr, uplo, n, ARF, A, lda, info)
}

pub unsafe fn dtfttr_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const f64,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtfttr_.unwrap()(transr, uplo, n, ARF, A, lda, info)
}

pub unsafe fn stfttr_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const f32,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stfttr_.unwrap()(transr, uplo, n, ARF, A, lda, info)
}

pub unsafe fn ztfttr_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    ARF: *const __BindgenComplex<f64>,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztfttr_.unwrap()(transr, uplo, n, ARF, A, lda, info)
}

pub unsafe fn ctgevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    S: *const __BindgenComplex<f32>,
    lds: *const lapack_int,
    P: *const __BindgenComplex<f32>,
    ldp: *const lapack_int,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctgevc_.unwrap()(
        side, howmny, select, n, S, lds, P, ldp, VL, ldvl, VR, ldvr, mm, m, work, rwork, info,
    )
}

pub unsafe fn dtgevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    S: *const f64,
    lds: *const lapack_int,
    P: *const f64,
    ldp: *const lapack_int,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtgevc_.unwrap()(
        side, howmny, select, n, S, lds, P, ldp, VL, ldvl, VR, ldvr, mm, m, work, info,
    )
}

pub unsafe fn stgevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    S: *const f32,
    lds: *const lapack_int,
    P: *const f32,
    ldp: *const lapack_int,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stgevc_.unwrap()(
        side, howmny, select, n, S, lds, P, ldp, VL, ldvl, VR, ldvr, mm, m, work, info,
    )
}

pub unsafe fn ztgevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    S: *const __BindgenComplex<f64>,
    lds: *const lapack_int,
    P: *const __BindgenComplex<f64>,
    ldp: *const lapack_int,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztgevc_.unwrap()(
        side, howmny, select, n, S, lds, P, ldp, VL, ldvl, VR, ldvr, mm, m, work, rwork, info,
    )
}

pub unsafe fn ctgexc_(
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    ifst: *const lapack_int,
    ilst: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctgexc_.unwrap()(wantq, wantz, n, A, lda, B, ldb, Q, ldq, Z, ldz, ifst, ilst, info)
}

pub unsafe fn dtgexc_(
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    ifst: *mut lapack_int,
    ilst: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtgexc_.unwrap()(
        wantq, wantz, n, A, lda, B, ldb, Q, ldq, Z, ldz, ifst, ilst, work, lwork, info,
    )
}

pub unsafe fn stgexc_(
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    ifst: *mut lapack_int,
    ilst: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stgexc_.unwrap()(
        wantq, wantz, n, A, lda, B, ldb, Q, ldq, Z, ldz, ifst, ilst, work, lwork, info,
    )
}

pub unsafe fn ztgexc_(
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    ifst: *const lapack_int,
    ilst: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztgexc_.unwrap()(wantq, wantz, n, A, lda, B, ldb, Q, ldq, Z, ldz, ifst, ilst, info)
}

pub unsafe fn ctgsen_(
    ijob: *const lapack_int,
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f32>,
    beta: *mut __BindgenComplex<f32>,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f32>,
    ldz: *const lapack_int,
    m: *mut lapack_int,
    pl: *mut f32,
    pr: *mut f32,
    DIF: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctgsen_.unwrap()(
        ijob, wantq, wantz, select, n, A, lda, B, ldb, alpha, beta, Q, ldq, Z, ldz, m, pl, pr, DIF,
        work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn dtgsen_(
    ijob: *const lapack_int,
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    alphar: *mut f64,
    alphai: *mut f64,
    beta: *mut f64,
    Q: *mut f64,
    ldq: *const lapack_int,
    Z: *mut f64,
    ldz: *const lapack_int,
    m: *mut lapack_int,
    pl: *mut f64,
    pr: *mut f64,
    DIF: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtgsen_.unwrap()(
        ijob, wantq, wantz, select, n, A, lda, B, ldb, alphar, alphai, beta, Q, ldq, Z, ldz, m, pl,
        pr, DIF, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn stgsen_(
    ijob: *const lapack_int,
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    alphar: *mut f32,
    alphai: *mut f32,
    beta: *mut f32,
    Q: *mut f32,
    ldq: *const lapack_int,
    Z: *mut f32,
    ldz: *const lapack_int,
    m: *mut lapack_int,
    pl: *mut f32,
    pr: *mut f32,
    DIF: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stgsen_.unwrap()(
        ijob, wantq, wantz, select, n, A, lda, B, ldb, alphar, alphai, beta, Q, ldq, Z, ldz, m, pl,
        pr, DIF, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ztgsen_(
    ijob: *const lapack_int,
    wantq: *const lapack_int,
    wantz: *const lapack_int,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    alpha: *mut __BindgenComplex<f64>,
    beta: *mut __BindgenComplex<f64>,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    Z: *mut __BindgenComplex<f64>,
    ldz: *const lapack_int,
    m: *mut lapack_int,
    pl: *mut f64,
    pr: *mut f64,
    DIF: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztgsen_.unwrap()(
        ijob, wantq, wantz, select, n, A, lda, B, ldb, alpha, beta, Q, ldq, Z, ldz, m, pl, pr, DIF,
        work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ctgsja_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    tola: *const f32,
    tolb: *const f32,
    alpha: *mut f32,
    beta: *mut f32,
    U: *mut __BindgenComplex<f32>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f32>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    ncycle: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctgsja_.unwrap()(
        jobu, jobv, jobq, m, p, n, k, l, A, lda, B, ldb, tola, tolb, alpha, beta, U, ldu, V, ldv,
        Q, ldq, work, ncycle, info,
    )
}

pub unsafe fn dtgsja_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    tola: *const f64,
    tolb: *const f64,
    alpha: *mut f64,
    beta: *mut f64,
    U: *mut f64,
    ldu: *const lapack_int,
    V: *mut f64,
    ldv: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    work: *mut f64,
    ncycle: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtgsja_.unwrap()(
        jobu, jobv, jobq, m, p, n, k, l, A, lda, B, ldb, tola, tolb, alpha, beta, U, ldu, V, ldv,
        Q, ldq, work, ncycle, info,
    )
}

pub unsafe fn stgsja_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    tola: *const f32,
    tolb: *const f32,
    alpha: *mut f32,
    beta: *mut f32,
    U: *mut f32,
    ldu: *const lapack_int,
    V: *mut f32,
    ldv: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    work: *mut f32,
    ncycle: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stgsja_.unwrap()(
        jobu, jobv, jobq, m, p, n, k, l, A, lda, B, ldb, tola, tolb, alpha, beta, U, ldu, V, ldv,
        Q, ldq, work, ncycle, info,
    )
}

pub unsafe fn ztgsja_(
    jobu: *const c_char,
    jobv: *const c_char,
    jobq: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    tola: *const f64,
    tolb: *const f64,
    alpha: *mut f64,
    beta: *mut f64,
    U: *mut __BindgenComplex<f64>,
    ldu: *const lapack_int,
    V: *mut __BindgenComplex<f64>,
    ldv: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    ncycle: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztgsja_.unwrap()(
        jobu, jobv, jobq, m, p, n, k, l, A, lda, B, ldb, tola, tolb, alpha, beta, U, ldu, V, ldv,
        Q, ldq, work, ncycle, info,
    )
}

pub unsafe fn ctgsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    VL: *const __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *const __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    S: *mut f32,
    DIF: *mut f32,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctgsna_.unwrap()(
        job, howmny, select, n, A, lda, B, ldb, VL, ldvl, VR, ldvr, S, DIF, mm, m, work, lwork,
        iwork, info,
    )
}

pub unsafe fn dtgsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    VL: *const f64,
    ldvl: *const lapack_int,
    VR: *const f64,
    ldvr: *const lapack_int,
    S: *mut f64,
    DIF: *mut f64,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtgsna_.unwrap()(
        job, howmny, select, n, A, lda, B, ldb, VL, ldvl, VR, ldvr, S, DIF, mm, m, work, lwork,
        iwork, info,
    )
}

pub unsafe fn stgsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    VL: *const f32,
    ldvl: *const lapack_int,
    VR: *const f32,
    ldvr: *const lapack_int,
    S: *mut f32,
    DIF: *mut f32,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stgsna_.unwrap()(
        job, howmny, select, n, A, lda, B, ldb, VL, ldvl, VR, ldvr, S, DIF, mm, m, work, lwork,
        iwork, info,
    )
}

pub unsafe fn ztgsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    VL: *const __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *const __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    S: *mut f64,
    DIF: *mut f64,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztgsna_.unwrap()(
        job, howmny, select, n, A, lda, B, ldb, VL, ldvl, VR, ldvr, S, DIF, mm, m, work, lwork,
        iwork, info,
    )
}

pub unsafe fn ctgsyl_(
    trans: *const c_char,
    ijob: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    D: *const __BindgenComplex<f32>,
    ldd: *const lapack_int,
    E: *const __BindgenComplex<f32>,
    lde: *const lapack_int,
    F: *mut __BindgenComplex<f32>,
    ldf: *const lapack_int,
    dif: *mut f32,
    scale: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctgsyl_.unwrap()(
        trans, ijob, m, n, A, lda, B, ldb, C, ldc, D, ldd, E, lde, F, ldf, dif, scale, work, lwork,
        iwork, info,
    )
}

pub unsafe fn dtgsyl_(
    trans: *const c_char,
    ijob: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    D: *const f64,
    ldd: *const lapack_int,
    E: *const f64,
    lde: *const lapack_int,
    F: *mut f64,
    ldf: *const lapack_int,
    dif: *mut f64,
    scale: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtgsyl_.unwrap()(
        trans, ijob, m, n, A, lda, B, ldb, C, ldc, D, ldd, E, lde, F, ldf, dif, scale, work, lwork,
        iwork, info,
    )
}

pub unsafe fn stgsyl_(
    trans: *const c_char,
    ijob: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    D: *const f32,
    ldd: *const lapack_int,
    E: *const f32,
    lde: *const lapack_int,
    F: *mut f32,
    ldf: *const lapack_int,
    dif: *mut f32,
    scale: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stgsyl_.unwrap()(
        trans, ijob, m, n, A, lda, B, ldb, C, ldc, D, ldd, E, lde, F, ldf, dif, scale, work, lwork,
        iwork, info,
    )
}

pub unsafe fn ztgsyl_(
    trans: *const c_char,
    ijob: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    D: *const __BindgenComplex<f64>,
    ldd: *const lapack_int,
    E: *const __BindgenComplex<f64>,
    lde: *const lapack_int,
    F: *mut __BindgenComplex<f64>,
    ldf: *const lapack_int,
    dif: *mut f64,
    scale: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztgsyl_.unwrap()(
        trans, ijob, m, n, A, lda, B, ldb, C, ldc, D, ldd, E, lde, F, ldf, dif, scale, work, lwork,
        iwork, info,
    )
}

pub unsafe fn ctpcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctpcon_.unwrap()(norm, uplo, diag, n, AP, rcond, work, rwork, info)
}

pub unsafe fn dtpcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtpcon_.unwrap()(norm, uplo, diag, n, AP, rcond, work, iwork, info)
}

pub unsafe fn stpcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stpcon_.unwrap()(norm, uplo, diag, n, AP, rcond, work, iwork, info)
}

pub unsafe fn ztpcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztpcon_.unwrap()(norm, uplo, diag, n, AP, rcond, work, rwork, info)
}

pub unsafe fn ctplqt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctplqt_.unwrap()(m, n, l, mb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn dtplqt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtplqt_.unwrap()(m, n, l, mb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn stplqt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stplqt_.unwrap()(m, n, l, mb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn ztplqt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztplqt_.unwrap()(m, n, l, mb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn ctplqt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctplqt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn dtplqt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtplqt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn stplqt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stplqt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn ztplqt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztplqt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn ctpmlqt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctpmlqt_.unwrap()(
        side, trans, m, n, k, l, mb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn dtpmlqt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtpmlqt_.unwrap()(
        side, trans, m, n, k, l, mb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn stpmlqt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stpmlqt_.unwrap()(
        side, trans, m, n, k, l, mb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn ztpmlqt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    mb: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztpmlqt_.unwrap()(
        side, trans, m, n, k, l, mb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn ctpmqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctpmqrt_.unwrap()(
        side, trans, m, n, k, l, nb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn dtpmqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtpmqrt_.unwrap()(
        side, trans, m, n, k, l, nb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn stpmqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stpmqrt_.unwrap()(
        side, trans, m, n, k, l, nb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn ztpmqrt_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztpmqrt_.unwrap()(
        side, trans, m, n, k, l, nb, V, ldv, T, ldt, A, lda, B, ldb, work, info,
    )
}

pub unsafe fn ctpqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctpqrt_.unwrap()(m, n, l, nb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn dtpqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtpqrt_.unwrap()(m, n, l, nb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn stpqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stpqrt_.unwrap()(m, n, l, nb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn ztpqrt_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztpqrt_.unwrap()(m, n, l, nb, A, lda, B, ldb, T, ldt, work, info)
}

pub unsafe fn ctpqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctpqrt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn dtpqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtpqrt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn stpqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stpqrt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn ztpqrt2_(
    m: *const lapack_int,
    n: *const lapack_int,
    l: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztpqrt2_.unwrap()(m, n, l, A, lda, B, ldb, T, ldt, info)
}

pub unsafe fn ctprfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    V: *const __BindgenComplex<f32>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    ldwork: *const lapack_int,
) {
    dyload_lib().ctprfb_.unwrap()(
        side, trans, direct, storev, m, n, k, l, V, ldv, T, ldt, A, lda, B, ldb, work, ldwork,
    )
}

pub unsafe fn dtprfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    V: *const f64,
    ldv: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    work: *mut f64,
    ldwork: *const lapack_int,
) {
    dyload_lib().dtprfb_.unwrap()(
        side, trans, direct, storev, m, n, k, l, V, ldv, T, ldt, A, lda, B, ldb, work, ldwork,
    )
}

pub unsafe fn stprfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    V: *const f32,
    ldv: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    work: *mut f32,
    ldwork: *const lapack_int,
) {
    dyload_lib().stprfb_.unwrap()(
        side, trans, direct, storev, m, n, k, l, V, ldv, T, ldt, A, lda, B, ldb, work, ldwork,
    )
}

pub unsafe fn ztprfb_(
    side: *const c_char,
    trans: *const c_char,
    direct: *const c_char,
    storev: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    V: *const __BindgenComplex<f64>,
    ldv: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    ldwork: *const lapack_int,
) {
    dyload_lib().ztprfb_.unwrap()(
        side, trans, direct, storev, m, n, k, l, V, ldv, T, ldt, A, lda, B, ldb, work, ldwork,
    )
}

pub unsafe fn ctprfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctprfs_.unwrap()(
        uplo, trans, diag, n, nrhs, AP, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dtprfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    B: *const f64,
    ldb: *const lapack_int,
    X: *const f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtprfs_.unwrap()(
        uplo, trans, diag, n, nrhs, AP, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn stprfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    B: *const f32,
    ldb: *const lapack_int,
    X: *const f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stprfs_.unwrap()(
        uplo, trans, diag, n, nrhs, AP, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn ztprfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztprfs_.unwrap()(
        uplo, trans, diag, n, nrhs, AP, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn ctptri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctptri_.unwrap()(uplo, diag, n, AP, info)
}

pub unsafe fn dtptri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtptri_.unwrap()(uplo, diag, n, AP, info)
}

pub unsafe fn stptri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stptri_.unwrap()(uplo, diag, n, AP, info)
}

pub unsafe fn ztptri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztptri_.unwrap()(uplo, diag, n, AP, info)
}

pub unsafe fn ctptrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctptrs_.unwrap()(uplo, trans, diag, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn dtptrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f64,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtptrs_.unwrap()(uplo, trans, diag, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn stptrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const f32,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stptrs_.unwrap()(uplo, trans, diag, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn ztptrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztptrs_.unwrap()(uplo, trans, diag, n, nrhs, AP, B, ldb, info)
}

pub unsafe fn ctpttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    ARF: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctpttf_.unwrap()(transr, uplo, n, AP, ARF, info)
}

pub unsafe fn dtpttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    ARF: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtpttf_.unwrap()(transr, uplo, n, AP, ARF, info)
}

pub unsafe fn stpttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    ARF: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().stpttf_.unwrap()(transr, uplo, n, AP, ARF, info)
}

pub unsafe fn ztpttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    ARF: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztpttf_.unwrap()(transr, uplo, n, AP, ARF, info)
}

pub unsafe fn ctpttr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctpttr_.unwrap()(uplo, n, AP, A, lda, info)
}

pub unsafe fn dtpttr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f64,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtpttr_.unwrap()(uplo, n, AP, A, lda, info)
}

pub unsafe fn stpttr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const f32,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stpttr_.unwrap()(uplo, n, AP, A, lda, info)
}

pub unsafe fn ztpttr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztpttr_.unwrap()(uplo, n, AP, A, lda, info)
}

pub unsafe fn ctrcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    rcond: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctrcon_.unwrap()(norm, uplo, diag, n, A, lda, rcond, work, rwork, info)
}

pub unsafe fn dtrcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    rcond: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrcon_.unwrap()(norm, uplo, diag, n, A, lda, rcond, work, iwork, info)
}

pub unsafe fn strcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    rcond: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strcon_.unwrap()(norm, uplo, diag, n, A, lda, rcond, work, iwork, info)
}

pub unsafe fn ztrcon_(
    norm: *const c_char,
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    rcond: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztrcon_.unwrap()(norm, uplo, diag, n, A, lda, rcond, work, rwork, info)
}

pub unsafe fn ctrevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctrevc_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, rwork, info,
    )
}

pub unsafe fn dtrevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtrevc_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, info,
    )
}

pub unsafe fn strevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().strevc_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, info,
    )
}

pub unsafe fn ztrevc_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztrevc_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, rwork, info,
    )
}

pub unsafe fn ctrevc3_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    VL: *mut __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrevc3_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, lwork, rwork, lrwork,
        info,
    )
}

pub unsafe fn dtrevc3_(
    side: *const c_char,
    howmny: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    VL: *mut f64,
    ldvl: *const lapack_int,
    VR: *mut f64,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrevc3_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, lwork, info,
    )
}

pub unsafe fn strevc3_(
    side: *const c_char,
    howmny: *const c_char,
    select: *mut lapack_int,
    n: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    VL: *mut f32,
    ldvl: *const lapack_int,
    VR: *mut f32,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strevc3_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, lwork, info,
    )
}

pub unsafe fn ztrevc3_(
    side: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    VL: *mut __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *mut __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrevc3_.unwrap()(
        side, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, mm, m, work, lwork, rwork, lrwork,
        info,
    )
}

pub unsafe fn ctrexc_(
    compq: *const c_char,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    ifst: *const lapack_int,
    ilst: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrexc_.unwrap()(compq, n, T, ldt, Q, ldq, ifst, ilst, info)
}

pub unsafe fn dtrexc_(
    compq: *const c_char,
    n: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    ifst: *mut lapack_int,
    ilst: *mut lapack_int,
    work: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtrexc_.unwrap()(compq, n, T, ldt, Q, ldq, ifst, ilst, work, info)
}

pub unsafe fn strexc_(
    compq: *const c_char,
    n: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    ifst: *mut lapack_int,
    ilst: *mut lapack_int,
    work: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().strexc_.unwrap()(compq, n, T, ldt, Q, ldq, ifst, ilst, work, info)
}

pub unsafe fn ztrexc_(
    compq: *const c_char,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    ifst: *const lapack_int,
    ilst: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrexc_.unwrap()(compq, n, T, ldt, Q, ldq, ifst, ilst, info)
}

pub unsafe fn ctrrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f32>,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut __BindgenComplex<f32>,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctrrfs_.unwrap()(
        uplo, trans, diag, n, nrhs, A, lda, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn dtrrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    X: *const f64,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut f64,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrrfs_.unwrap()(
        uplo, trans, diag, n, nrhs, A, lda, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn strrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    X: *const f32,
    ldx: *const lapack_int,
    ferr: *mut f32,
    berr: *mut f32,
    work: *mut f32,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strrfs_.unwrap()(
        uplo, trans, diag, n, nrhs, A, lda, B, ldb, X, ldx, ferr, berr, work, iwork, info,
    )
}

pub unsafe fn ztrrfs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    X: *const __BindgenComplex<f64>,
    ldx: *const lapack_int,
    ferr: *mut f64,
    berr: *mut f64,
    work: *mut __BindgenComplex<f64>,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztrrfs_.unwrap()(
        uplo, trans, diag, n, nrhs, A, lda, B, ldb, X, ldx, ferr, berr, work, rwork, info,
    )
}

pub unsafe fn ctrsen_(
    job: *const c_char,
    compq: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    W: *mut __BindgenComplex<f32>,
    m: *mut lapack_int,
    s: *mut f32,
    sep: *mut f32,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrsen_.unwrap()(
        job, compq, select, n, T, ldt, Q, ldq, W, m, s, sep, work, lwork, info,
    )
}

pub unsafe fn dtrsen_(
    job: *const c_char,
    compq: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut f64,
    ldt: *const lapack_int,
    Q: *mut f64,
    ldq: *const lapack_int,
    WR: *mut f64,
    WI: *mut f64,
    m: *mut lapack_int,
    s: *mut f64,
    sep: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrsen_.unwrap()(
        job, compq, select, n, T, ldt, Q, ldq, WR, WI, m, s, sep, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn strsen_(
    job: *const c_char,
    compq: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut f32,
    ldt: *const lapack_int,
    Q: *mut f32,
    ldq: *const lapack_int,
    WR: *mut f32,
    WI: *mut f32,
    m: *mut lapack_int,
    s: *mut f32,
    sep: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strsen_.unwrap()(
        job, compq, select, n, T, ldt, Q, ldq, WR, WI, m, s, sep, work, lwork, iwork, liwork, info,
    )
}

pub unsafe fn ztrsen_(
    job: *const c_char,
    compq: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    W: *mut __BindgenComplex<f64>,
    m: *mut lapack_int,
    s: *mut f64,
    sep: *mut f64,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrsen_.unwrap()(
        job, compq, select, n, T, ldt, Q, ldq, W, m, s, sep, work, lwork, info,
    )
}

pub unsafe fn ctrsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    VL: *const __BindgenComplex<f32>,
    ldvl: *const lapack_int,
    VR: *const __BindgenComplex<f32>,
    ldvr: *const lapack_int,
    S: *mut f32,
    SEP: *mut f32,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f32>,
    ldwork: *const lapack_int,
    rwork: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctrsna_.unwrap()(
        job, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, S, SEP, mm, m, work, ldwork, rwork,
        info,
    )
}

pub unsafe fn dtrsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *const f64,
    ldt: *const lapack_int,
    VL: *const f64,
    ldvl: *const lapack_int,
    VR: *const f64,
    ldvr: *const lapack_int,
    S: *mut f64,
    SEP: *mut f64,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f64,
    ldwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrsna_.unwrap()(
        job, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, S, SEP, mm, m, work, ldwork, iwork,
        info,
    )
}

pub unsafe fn strsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *const f32,
    ldt: *const lapack_int,
    VL: *const f32,
    ldvl: *const lapack_int,
    VR: *const f32,
    ldvr: *const lapack_int,
    S: *mut f32,
    SEP: *mut f32,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut f32,
    ldwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strsna_.unwrap()(
        job, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, S, SEP, mm, m, work, ldwork, iwork,
        info,
    )
}

pub unsafe fn ztrsna_(
    job: *const c_char,
    howmny: *const c_char,
    select: *const lapack_int,
    n: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    VL: *const __BindgenComplex<f64>,
    ldvl: *const lapack_int,
    VR: *const __BindgenComplex<f64>,
    ldvr: *const lapack_int,
    S: *mut f64,
    SEP: *mut f64,
    mm: *const lapack_int,
    m: *mut lapack_int,
    work: *mut __BindgenComplex<f64>,
    ldwork: *const lapack_int,
    rwork: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztrsna_.unwrap()(
        job, howmny, select, n, T, ldt, VL, ldvl, VR, ldvr, S, SEP, mm, m, work, ldwork, rwork,
        info,
    )
}

pub unsafe fn ctrsyl_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    scale: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().ctrsyl_.unwrap()(trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, info)
}

pub unsafe fn dtrsyl_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    scale: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtrsyl_.unwrap()(trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, info)
}

pub unsafe fn strsyl_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    scale: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().strsyl_.unwrap()(trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, info)
}

pub unsafe fn ztrsyl_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    scale: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().ztrsyl_.unwrap()(trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, info)
}

pub unsafe fn ctrsyl3_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f32>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    scale: *mut f32,
    swork: *mut f32,
    ldswork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrsyl3_.unwrap()(
        trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, swork, ldswork, info,
    )
}

pub unsafe fn dtrsyl3_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *const f64,
    ldb: *const lapack_int,
    C: *mut f64,
    ldc: *const lapack_int,
    scale: *mut f64,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    swork: *mut f64,
    ldswork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrsyl3_.unwrap()(
        trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, iwork, liwork, swork, ldswork,
        info,
    )
}

pub unsafe fn strsyl3_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *const f32,
    ldb: *const lapack_int,
    C: *mut f32,
    ldc: *const lapack_int,
    scale: *mut f32,
    iwork: *mut lapack_int,
    liwork: *const lapack_int,
    swork: *mut f32,
    ldswork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strsyl3_.unwrap()(
        trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, iwork, liwork, swork, ldswork,
        info,
    )
}

pub unsafe fn ztrsyl3_(
    trana: *const c_char,
    tranb: *const c_char,
    isgn: *const lapack_int,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *const __BindgenComplex<f64>,
    ldb: *const lapack_int,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    scale: *mut f64,
    swork: *mut f64,
    ldswork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrsyl3_.unwrap()(
        trana, tranb, isgn, m, n, A, lda, B, ldb, C, ldc, scale, swork, ldswork, info,
    )
}

pub unsafe fn ctrtri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrtri_.unwrap()(uplo, diag, n, A, lda, info)
}

pub unsafe fn dtrtri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrtri_.unwrap()(uplo, diag, n, A, lda, info)
}

pub unsafe fn strtri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strtri_.unwrap()(uplo, diag, n, A, lda, info)
}

pub unsafe fn ztrtri_(
    uplo: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrtri_.unwrap()(uplo, diag, n, A, lda, info)
}

pub unsafe fn ctrtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f32>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctrtrs_.unwrap()(uplo, trans, diag, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn dtrtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    B: *mut f64,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtrtrs_.unwrap()(uplo, trans, diag, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn strtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    B: *mut f32,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().strtrs_.unwrap()(uplo, trans, diag, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn ztrtrs_(
    uplo: *const c_char,
    trans: *const c_char,
    diag: *const c_char,
    n: *const lapack_int,
    nrhs: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    B: *mut __BindgenComplex<f64>,
    ldb: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztrtrs_.unwrap()(uplo, trans, diag, n, nrhs, A, lda, B, ldb, info)
}

pub unsafe fn ctrttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    ARF: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctrttf_.unwrap()(transr, uplo, n, A, lda, ARF, info)
}

pub unsafe fn dtrttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    ARF: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtrttf_.unwrap()(transr, uplo, n, A, lda, ARF, info)
}

pub unsafe fn strttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    ARF: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().strttf_.unwrap()(transr, uplo, n, A, lda, ARF, info)
}

pub unsafe fn ztrttf_(
    transr: *const c_char,
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    ARF: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztrttf_.unwrap()(transr, uplo, n, A, lda, ARF, info)
}

pub unsafe fn ctrttp_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    AP: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().ctrttp_.unwrap()(uplo, n, A, lda, AP, info)
}

pub unsafe fn dtrttp_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f64,
    lda: *const lapack_int,
    AP: *mut f64,
    info: *mut lapack_int,
) {
    dyload_lib().dtrttp_.unwrap()(uplo, n, A, lda, AP, info)
}

pub unsafe fn strttp_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const f32,
    lda: *const lapack_int,
    AP: *mut f32,
    info: *mut lapack_int,
) {
    dyload_lib().strttp_.unwrap()(uplo, n, A, lda, AP, info)
}

pub unsafe fn ztrttp_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    AP: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().ztrttp_.unwrap()(uplo, n, A, lda, AP, info)
}

pub unsafe fn ctzrzf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ctzrzf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn dtzrzf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f64,
    lda: *const lapack_int,
    tau: *mut f64,
    work: *mut f64,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().dtzrzf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn stzrzf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut f32,
    lda: *const lapack_int,
    tau: *mut f32,
    work: *mut f32,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().stzrzf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn ztzrzf_(
    m: *const lapack_int,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().ztzrzf_.unwrap()(m, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cunbdb_(
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f32>,
    ldx11: *const lapack_int,
    X12: *mut __BindgenComplex<f32>,
    ldx12: *const lapack_int,
    X21: *mut __BindgenComplex<f32>,
    ldx21: *const lapack_int,
    X22: *mut __BindgenComplex<f32>,
    ldx22: *const lapack_int,
    theta: *mut f32,
    phi: *mut f32,
    TAUP1: *mut __BindgenComplex<f32>,
    TAUP2: *mut __BindgenComplex<f32>,
    TAUQ1: *mut __BindgenComplex<f32>,
    TAUQ2: *mut __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunbdb_.unwrap()(
        trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21, X22, ldx22, theta, phi, TAUP1,
        TAUP2, TAUQ1, TAUQ2, work, lwork, info,
    )
}

pub unsafe fn zunbdb_(
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f64>,
    ldx11: *const lapack_int,
    X12: *mut __BindgenComplex<f64>,
    ldx12: *const lapack_int,
    X21: *mut __BindgenComplex<f64>,
    ldx21: *const lapack_int,
    X22: *mut __BindgenComplex<f64>,
    ldx22: *const lapack_int,
    theta: *mut f64,
    phi: *mut f64,
    TAUP1: *mut __BindgenComplex<f64>,
    TAUP2: *mut __BindgenComplex<f64>,
    TAUQ1: *mut __BindgenComplex<f64>,
    TAUQ2: *mut __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunbdb_.unwrap()(
        trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21, X22, ldx22, theta, phi, TAUP1,
        TAUP2, TAUQ1, TAUQ2, work, lwork, info,
    )
}

pub unsafe fn cuncsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f32>,
    ldx11: *const lapack_int,
    X12: *mut __BindgenComplex<f32>,
    ldx12: *const lapack_int,
    X21: *mut __BindgenComplex<f32>,
    ldx21: *const lapack_int,
    X22: *mut __BindgenComplex<f32>,
    ldx22: *const lapack_int,
    theta: *mut f32,
    U1: *mut __BindgenComplex<f32>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f32>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f32>,
    ldv1t: *const lapack_int,
    V2T: *mut __BindgenComplex<f32>,
    ldv2t: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cuncsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21,
        X22, ldx22, theta, U1, ldu1, U2, ldu2, V1T, ldv1t, V2T, ldv2t, work, lwork, rwork, lrwork,
        iwork, info,
    )
}

pub unsafe fn zuncsd_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    jobv2t: *const c_char,
    trans: *const c_char,
    signs: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f64>,
    ldx11: *const lapack_int,
    X12: *mut __BindgenComplex<f64>,
    ldx12: *const lapack_int,
    X21: *mut __BindgenComplex<f64>,
    ldx21: *const lapack_int,
    X22: *mut __BindgenComplex<f64>,
    ldx22: *const lapack_int,
    theta: *mut f64,
    U1: *mut __BindgenComplex<f64>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f64>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f64>,
    ldv1t: *const lapack_int,
    V2T: *mut __BindgenComplex<f64>,
    ldv2t: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zuncsd_.unwrap()(
        jobu1, jobu2, jobv1t, jobv2t, trans, signs, m, p, q, X11, ldx11, X12, ldx12, X21, ldx21,
        X22, ldx22, theta, U1, ldu1, U2, ldu2, V1T, ldv1t, V2T, ldv2t, work, lwork, rwork, lrwork,
        iwork, info,
    )
}

pub unsafe fn cuncsd2by1_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f32>,
    ldx11: *const lapack_int,
    X21: *mut __BindgenComplex<f32>,
    ldx21: *const lapack_int,
    theta: *mut f32,
    U1: *mut __BindgenComplex<f32>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f32>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f32>,
    ldv1t: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    rwork: *mut f32,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cuncsd2by1_.unwrap()(
        jobu1, jobu2, jobv1t, m, p, q, X11, ldx11, X21, ldx21, theta, U1, ldu1, U2, ldu2, V1T,
        ldv1t, work, lwork, rwork, lrwork, iwork, info,
    )
}

pub unsafe fn zuncsd2by1_(
    jobu1: *const c_char,
    jobu2: *const c_char,
    jobv1t: *const c_char,
    m: *const lapack_int,
    p: *const lapack_int,
    q: *const lapack_int,
    X11: *mut __BindgenComplex<f64>,
    ldx11: *const lapack_int,
    X21: *mut __BindgenComplex<f64>,
    ldx21: *const lapack_int,
    theta: *mut f64,
    U1: *mut __BindgenComplex<f64>,
    ldu1: *const lapack_int,
    U2: *mut __BindgenComplex<f64>,
    ldu2: *const lapack_int,
    V1T: *mut __BindgenComplex<f64>,
    ldv1t: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    rwork: *mut f64,
    lrwork: *const lapack_int,
    iwork: *mut lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zuncsd2by1_.unwrap()(
        jobu1, jobu2, jobv1t, m, p, q, X11, ldx11, X21, ldx21, theta, U1, ldu1, U2, ldu2, V1T,
        ldv1t, work, lwork, rwork, lrwork, iwork, info,
    )
}

pub unsafe fn cungbr_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungbr_.unwrap()(vect, m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn zungbr_(
    vect: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungbr_.unwrap()(vect, m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn cunghr_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunghr_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn zunghr_(
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunghr_.unwrap()(n, ilo, ihi, A, lda, tau, work, lwork, info)
}

pub unsafe fn cunglq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunglq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn zunglq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunglq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn cungql_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungql_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn zungql_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungql_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn cungqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungqr_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn zungqr_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungqr_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn cungrq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungrq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn zungrq_(
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungrq_.unwrap()(m, n, k, A, lda, tau, work, lwork, info)
}

pub unsafe fn cungtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungtr_.unwrap()(uplo, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn zungtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungtr_.unwrap()(uplo, n, A, lda, tau, work, lwork, info)
}

pub unsafe fn cungtsqr_row_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f32>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cungtsqr_row_.unwrap()(m, n, mb, nb, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn zungtsqr_row_(
    m: *const lapack_int,
    n: *const lapack_int,
    mb: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *const __BindgenComplex<f64>,
    ldt: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zungtsqr_row_.unwrap()(m, n, mb, nb, A, lda, T, ldt, work, lwork, info)
}

pub unsafe fn cunhr_col_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f32>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f32>,
    ldt: *const lapack_int,
    D: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cunhr_col_.unwrap()(m, n, nb, A, lda, T, ldt, D, info)
}

pub unsafe fn zunhr_col_(
    m: *const lapack_int,
    n: *const lapack_int,
    nb: *const lapack_int,
    A: *mut __BindgenComplex<f64>,
    lda: *const lapack_int,
    T: *mut __BindgenComplex<f64>,
    ldt: *const lapack_int,
    D: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zunhr_col_.unwrap()(m, n, nb, A, lda, T, ldt, D, info)
}

pub unsafe fn cunmbr_(
    vect: *const c_char,
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmbr_.unwrap()(
        vect, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn zunmbr_(
    vect: *const c_char,
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmbr_.unwrap()(
        vect, side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn cunmhr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmhr_.unwrap()(
        side, trans, m, n, ilo, ihi, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn zunmhr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    ilo: *const lapack_int,
    ihi: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmhr_.unwrap()(
        side, trans, m, n, ilo, ihi, A, lda, tau, C, ldc, work, lwork, info,
    )
}

pub unsafe fn cunmlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmlq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmlq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmlq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cunmql_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmql_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmql_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmql_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cunmqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmqr_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmqr_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmqr_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cunmrq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmrq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmrq_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmrq_.unwrap()(side, trans, m, n, k, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cunmrz_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmrz_.unwrap()(side, trans, m, n, k, l, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmrz_(
    side: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    k: *const lapack_int,
    l: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmrz_.unwrap()(side, trans, m, n, k, l, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cunmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f32>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().cunmtr_.unwrap()(side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn zunmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    A: *const __BindgenComplex<f64>,
    lda: *const lapack_int,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    lwork: *const lapack_int,
    info: *mut lapack_int,
) {
    dyload_lib().zunmtr_.unwrap()(side, uplo, trans, m, n, A, lda, tau, C, ldc, work, lwork, info)
}

pub unsafe fn cupgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    tau: *const __BindgenComplex<f32>,
    Q: *mut __BindgenComplex<f32>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cupgtr_.unwrap()(uplo, n, AP, tau, Q, ldq, work, info)
}

pub unsafe fn zupgtr_(
    uplo: *const c_char,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    tau: *const __BindgenComplex<f64>,
    Q: *mut __BindgenComplex<f64>,
    ldq: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zupgtr_.unwrap()(uplo, n, AP, tau, Q, ldq, work, info)
}

pub unsafe fn cupmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f32>,
    tau: *const __BindgenComplex<f32>,
    C: *mut __BindgenComplex<f32>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f32>,
    info: *mut lapack_int,
) {
    dyload_lib().cupmtr_.unwrap()(side, uplo, trans, m, n, AP, tau, C, ldc, work, info)
}

pub unsafe fn zupmtr_(
    side: *const c_char,
    uplo: *const c_char,
    trans: *const c_char,
    m: *const lapack_int,
    n: *const lapack_int,
    AP: *const __BindgenComplex<f64>,
    tau: *const __BindgenComplex<f64>,
    C: *mut __BindgenComplex<f64>,
    ldc: *const lapack_int,
    work: *mut __BindgenComplex<f64>,
    info: *mut lapack_int,
) {
    dyload_lib().zupmtr_.unwrap()(side, uplo, trans, m, n, AP, tau, C, ldc, work, info)
}
