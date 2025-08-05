#![doc = include_str!("../readme.md")]
#![allow(clashing_extern_declarations)]

#[cfg(feature = "blas")]
pub mod blas;
#[cfg(feature = "cblas")]
pub mod cblas;
#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "lapacke")]
pub mod lapacke;

pub const CRATE_NAME: &str = "rstsr-openblas-ffi";
pub const LIB_NAME: &str = "OPENBLAS"; // for code, e.g. "MKL"
pub const LIB_NAME_SHOW: &str = "OpenBLAS"; // for display, e.g. "oneMKL"
pub const LIB_NAME_LINK: &str = "openblas"; // for linking, e.g. "mkl_rt"

#[cfg(all(not(clippy), feature = "quad_precision", feature = "ex_precision"))]
compile_error!("Cannot enable both quad and extended precision features at the same time.");

#[cfg(feature = "dynamic_loading")]
pub(crate) mod get_lib_candidates {
    use super::*;
    use std::fmt::Debug;

    pub(crate) fn get_lib_candidates() -> Vec<String> {
        use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
        let mut candidates = vec![];

        // user defined candidates
        for paths in [format!("RSTSR_DYLOAD_{LIB_NAME}").as_str(), "RSTSR_DYLOAD"] {
            if let Ok(path) = std::env::var(paths) {
                candidates.extend(path.split(":").map(|s| s.to_string()).collect::<Vec<_>>());
            }
        }

        // other candidates configuration
        let int_type = if cfg!(feature = "ilp64") { "64" } else { "32" };
        let int_name = if cfg!(feature = "ilp64") { "ilp64" } else { "lp64" };

        candidates.extend(vec![
            format!("{DLL_PREFIX}openblas{int_type}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}openblas_{int_type}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}openblas-{int_type}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}openblas_{int_name}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}openblas-{int_name}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}openblas{DLL_SUFFIX}"),
        ]);
        candidates
    }

    pub(crate) fn panic_no_lib_found<S: Debug>(candidates: &[S], err_msg: &str) -> ! {
        panic!(
            r#"
This happens in crate `{CRATE_NAME}`.
Unable to dynamically load the {LIB_NAME_SHOW} (`{LIB_NAME_LINK}`) shared library.
Candidates: {candidates:#?}

Please check
- if dynamic-loading is not desired, please disable the `dynamic_loading` feature in your `Cargo.toml` (by something like --no-default-features).
- if you want to provide custom {LIB_NAME_SHOW} library, use environment variable `RSTSR_DYLOAD_{LIB_NAME}` or `RSTSR_DYLOAD` to specify the path to the library.
- if `lib{LIB_NAME_LINK}.so` (linux) or `lib{LIB_NAME_LINK}.dylib` (macOS) or `lib{LIB_NAME_LINK}.dll` (Windows) is installed on your system.
- if `LD_LIBRARY_PATH` (linux) or `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows) environment variable is set correctly (any path that's visible to linker).
- this crate does not use things like `LD_PRELOAD` or `DYLD_INSERT_LIBRARIES` to load the library.
- this crate does not support static linking of libraries when dynamic-loading.

Error message(s):
{err_msg}
"#
        )
    }
}
