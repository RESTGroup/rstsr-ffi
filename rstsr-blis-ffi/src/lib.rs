#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod blis_types;

pub use blis_types::*;

#[cfg(feature = "x86_64")]
pub mod blis_x86_64;
#[cfg(feature = "x86_64")]
pub use blis_x86_64 as blis;
