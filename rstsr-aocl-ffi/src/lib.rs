#![doc = include_str!("../readme.md")]
#![allow(clashing_extern_declarations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// bindgen 0.73.2 emits `__BindgenBitfieldUnit` for system-header bitfields (glibc), which trips
// these.
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::ptr_offset_with_cast)]

pub mod blis_types;
pub use blis_types::*;

#[cfg(feature = "blis")]
pub mod blis;

#[cfg(feature = "blis")]
pub use blis as blas;
#[cfg(feature = "blis")]
pub use blis as cblas;

#[cfg(feature = "flame")]
pub mod flame;
#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "lapacke")]
pub mod lapacke;
