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

#[cfg(feature = "ilp64")]
pub type uinteger = u64;
#[cfg(all(not(feature = "ilp64"), feature = "lp64_as_int"))]
pub type uinteger = u32;
#[cfg(all(not(feature = "ilp64"), not(feature = "lp64_as_int")))]
pub type uinteger = c_ulong;

pub use blas_int as f77_int;
pub use blas_int as integer;
