//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

pub(crate) use core::ffi::{c_char, c_int, c_void};

#[cfg(not(feature = "ilp64"))]
pub type blas_int = i32;
#[cfg(feature = "ilp64")]
pub type blas_int = i64;

pub use rstsr_lapack_ffi::cblas::{
    CBLAS_DIAG, CBLAS_LAYOUT, CBLAS_SIDE, CBLAS_TRANSPOSE, CBLAS_UPLO,
};
pub use CBLAS_LAYOUT as CBLAS_ORDER;

/* automatically generated by rust-bindgen 0.71.1 */

pub const OPENBLAS_SEQUENTIAL: u32 = 0;
pub const OPENBLAS_THREAD: u32 = 1;
pub const OPENBLAS_OPENMP: u32 = 2;
pub type bfloat16 = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct openblas_complex_float {
    pub real: f32,
    pub imag: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct openblas_complex_double {
    pub real: f64,
    pub imag: f64,
}
pub type openblas_dojob_callback =
    Option<extern "C" fn(thread_num: c_int, jobdata: *mut c_void, dojob_data: c_int)>;
pub type openblas_threads_callback = Option<
    extern "C" fn(
        sync: c_int,
        dojob: openblas_dojob_callback,
        numjobs: c_int,
        jobdata_elsize: usize,
        jobdata: *mut c_void,
        dojob_data: c_int,
    ),
>;
