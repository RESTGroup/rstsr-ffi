//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

pub use crate::mkl_types::*;

pub const LAPACK_ROW_MAJOR: u32 = 101;
pub const LAPACK_COL_MAJOR: u32 = 102;
pub const LAPACK_WORK_MEMORY_ERROR: i32 = -1010;
pub const LAPACK_TRANSPOSE_MEMORY_ERROR: i32 = -1011;

pub type LAPACK_S_SELECT2 = Option<extern "C" fn(arg1: *const f32, arg2: *const f32) -> MKL_INT>;
pub type LAPACK_S_SELECT3 =
    Option<extern "C" fn(arg1: *const f32, arg2: *const f32, arg3: *const f32) -> MKL_INT>;
pub type LAPACK_D_SELECT2 = Option<extern "C" fn(arg1: *const f64, arg2: *const f64) -> MKL_INT>;
pub type LAPACK_D_SELECT3 =
    Option<extern "C" fn(arg1: *const f64, arg2: *const f64, arg3: *const f64) -> MKL_INT>;
pub type LAPACK_C_SELECT1 = Option<extern "C" fn(arg1: *const MKL_Complex8) -> MKL_INT>;
pub type LAPACK_C_SELECT2 =
    Option<extern "C" fn(arg1: *const MKL_Complex8, arg2: *const MKL_Complex8) -> MKL_INT>;
pub type LAPACK_Z_SELECT1 = Option<extern "C" fn(arg1: *const MKL_Complex16) -> MKL_INT>;
pub type LAPACK_Z_SELECT2 =
    Option<extern "C" fn(arg1: *const MKL_Complex16, arg2: *const MKL_Complex16) -> MKL_INT>;
