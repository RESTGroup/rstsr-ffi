#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod blis_types;
pub use blis_types::*;

#[cfg(feature = "blis")]
pub mod blis;

#[cfg(feature = "blis")]
pub use blis as blas;
#[cfg(feature = "blis")]
pub use blis as cblas;

#[cfg(any(feature = "lapack", feature = "flame"))]
pub mod flame;
#[cfg(any(feature = "lapack", feature = "flame"))]
pub use flame as lapack;
