pub mod blas;
pub mod cblas;

#[cfg(all(not(clippy), feature = "quad_precision", feature = "ex_precision"))]
compile_error!("Cannot enable both quad and extended precision features at the same time.");
