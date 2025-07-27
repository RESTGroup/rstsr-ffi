//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

pub use crate::mkl_types::*;
pub use rstsr_lapack_ffi::cblas::{
    CBLAS_DIAG, CBLAS_LAYOUT, CBLAS_SIDE, CBLAS_TRANSPOSE, CBLAS_UPLO,
};
pub use CBLAS_LAYOUT as CBLAS_ORDER;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CBLAS_IDENTIFIER {
    CblasAMatrix = 161,
    CblasBMatrix = 162,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CBLAS_OFFSET {
    CblasRowOffset = 171,
    CblasColOffset = 172,
    CblasFixOffset = 173,
}
