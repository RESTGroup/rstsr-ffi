#![doc = include_str!("../readme.md")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod blis_types;
pub use blis_types::*;

#[cfg(feature = "x86_64")]
pub mod blis_x86_64;

// determine `blis` by priority
#[cfg(feature = "x86_64")]
pub use blis_x86_64 as blis;

// no architecture specific bindgen, use `blis_x86_64` as default
#[cfg(not(feature = "x86_64"))]
pub mod blis_x86_64;
#[cfg(not(feature = "x86_64"))]
pub use blis_x86_64 as blis;

pub use blis as blas;
pub use blis as cblas;

#[cfg(feature = "lapack")]
pub mod lapack;

#[cfg(feature = "flame")]
pub mod flame;

#[cfg(all(not(clippy), feature = "flame", feature = "ilp64"))]
compile_error!("Feature `flame` and `ilp64` are mutually exclusive, please disable one of them.");
