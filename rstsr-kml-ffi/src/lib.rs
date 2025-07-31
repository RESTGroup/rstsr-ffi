#![doc = include_str!("../readme.md")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod kml_types;
pub use kml_types::*;

pub mod service;

#[cfg(feature = "kblas")]
pub mod kblas;
#[cfg(feature = "kblas")]
pub use kblas as cblas;
#[cfg(feature = "kblas")]
pub mod blas;

#[cfg(feature = "lapack")]
pub mod lapack;
