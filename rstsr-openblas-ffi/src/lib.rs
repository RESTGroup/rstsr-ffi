pub mod cblas;
pub mod f77blas;

#[cfg(all(feature = "quad_precision", feature = "ex_precision"))]
compile_error!("Cannot enable both quad and extended precision features at the same time.");
