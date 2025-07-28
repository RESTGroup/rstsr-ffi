pub(crate) use core::ffi::*;

#[cfg(feature = "ilp64")]
mod mode_ilp64 {
    pub const BLIS_INT_TYPE_SIZE: u32 = 64;
    pub type f77_int = i64;
}
#[cfg(feature = "ilp64")]
pub use mode_ilp64::*;

#[cfg(not(feature = "ilp64"))]
mod mode_lp64 {
    pub const BLIS_INT_TYPE_SIZE: u32 = 32;
    pub type f77_int = i32;
}
#[cfg(not(feature = "ilp64"))]
pub use mode_lp64::*;

pub type blas_int = f77_int;
pub type lapack_int = f77_int;
