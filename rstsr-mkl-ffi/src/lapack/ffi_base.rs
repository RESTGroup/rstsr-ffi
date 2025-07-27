//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

pub use crate::mkl_types::*;

pub type MKL_C_SELECT_FUNCTION_1 = Option<extern "C" fn(arg1: *const MKL_Complex8) -> MKL_INT>;
pub type MKL_C_SELECT_FUNCTION_2 =
    Option<extern "C" fn(arg1: *const MKL_Complex8, arg2: *const MKL_Complex8) -> MKL_INT>;
pub type MKL_D_SELECT_FUNCTION_2 =
    Option<extern "C" fn(arg1: *const f64, arg2: *const f64) -> MKL_INT>;
pub type MKL_D_SELECT_FUNCTION_3 =
    Option<extern "C" fn(arg1: *const f64, arg2: *const f64, arg3: *const f64) -> MKL_INT>;
pub type MKL_S_SELECT_FUNCTION_2 =
    Option<extern "C" fn(arg1: *const f32, arg2: *const f32) -> MKL_INT>;
pub type MKL_S_SELECT_FUNCTION_3 =
    Option<extern "C" fn(arg1: *const f32, arg2: *const f32, arg3: *const f32) -> MKL_INT>;
pub type MKL_Z_SELECT_FUNCTION_1 = Option<extern "C" fn(arg1: *const MKL_Complex16) -> MKL_INT>;
pub type MKL_Z_SELECT_FUNCTION_2 =
    Option<extern "C" fn(arg1: *const MKL_Complex16, arg2: *const MKL_Complex16) -> MKL_INT>;
pub type MKL_C_SELECT_FUNCTION_1_64 = Option<extern "C" fn(arg1: *const MKL_Complex8) -> MKL_INT64>;
pub type MKL_C_SELECT_FUNCTION_2_64 =
    Option<extern "C" fn(arg1: *const MKL_Complex8, arg2: *const MKL_Complex8) -> MKL_INT64>;
pub type MKL_D_SELECT_FUNCTION_2_64 =
    Option<extern "C" fn(arg1: *const f64, arg2: *const f64) -> MKL_INT64>;
pub type MKL_D_SELECT_FUNCTION_3_64 =
    Option<extern "C" fn(arg1: *const f64, arg2: *const f64, arg3: *const f64) -> MKL_INT64>;
pub type MKL_S_SELECT_FUNCTION_2_64 =
    Option<extern "C" fn(arg1: *const f32, arg2: *const f32) -> MKL_INT64>;
pub type MKL_S_SELECT_FUNCTION_3_64 =
    Option<extern "C" fn(arg1: *const f32, arg2: *const f32, arg3: *const f32) -> MKL_INT64>;
pub type MKL_Z_SELECT_FUNCTION_1_64 =
    Option<extern "C" fn(arg1: *const MKL_Complex16) -> MKL_INT64>;
pub type MKL_Z_SELECT_FUNCTION_2_64 =
    Option<extern "C" fn(arg1: *const MKL_Complex16, arg2: *const MKL_Complex16) -> MKL_INT64>;
