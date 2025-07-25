//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

pub(crate) use core::ffi::{c_char, c_int};

#[cfg(not(feature = "ilp64"))]
pub type lapack_int = i32;
#[cfg(feature = "ilp64")]
pub type lapack_int = i64;

/* automatically generated by rust-bindgen 0.71.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
