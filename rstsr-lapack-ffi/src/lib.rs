#![doc = include_str!("../readme.md")]

#[cfg(feature = "blas")]
pub mod blas;
#[cfg(feature = "cblas")]
pub mod cblas;
#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "lapacke")]
pub mod lapacke;
#[cfg(feature = "lapacke_utils")]
pub mod lapacke_utils;
