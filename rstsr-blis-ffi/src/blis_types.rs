pub(crate) use core::ffi::*;
pub use rstsr_cblas_base::*;

#[cfg(not(feature = "ilp64"))]
pub const BLIS_INT_TYPE_SIZE: u32 = 32;
#[cfg(feature = "ilp64")]
pub const BLIS_INT_TYPE_SIZE: u32 = 64;

#[cfg(not(feature = "ilp64"))]
pub const BLIS_BLAS_INT_TYPE_SIZE: u32 = 32;
#[cfg(feature = "ilp64")]
pub const BLIS_BLAS_INT_TYPE_SIZE: u32 = 64;

pub use blas_int as f77_int;
