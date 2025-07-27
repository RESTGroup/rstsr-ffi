#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod mkl_types;

#[cfg(feature = "blas")]
pub mod blas;
#[cfg(feature = "cblas")]
pub mod cblas;

pub const CRATE_NAME: &str = "rstsr-mkl-ffi";
pub const LIB_NAME: &str = "MKL"; // for code, e.g. "MKL"
pub const LIB_NAME_SHOW: &str = "oneAPI MKL"; // for display, e.g. "oneMKL"
pub const LIB_NAME_LINK: &str = "mkl_rt"; // for linking, e.g. "mkl_rt"

#[cfg(feature = "dynamic_loading")]
pub(crate) mod get_lib_candidates {
    use super::*;
    use std::{fmt::Debug, path::PathBuf};

    pub(crate) fn get_lib_candidates() -> Vec<String> {
        use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
        let mut candidates = vec![];

        // user defined candidates
        for paths in [format!("RSTSR_DYLOAD_{LIB_NAME}").as_str(), "RSTSR_DYLOAD"] {
            if let Ok(path) = std::env::var(paths) {
                candidates.extend(path.split(":").map(|s| s.to_string()).collect::<Vec<_>>());
            }
        }

        let mkl_rt_name = format!("{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}");
        let env_mkl_root = std::env::var("MKL_ROOT");
        let env_mklroot = std::env::var("MKLROOT");
        let mut possible_dir_paths = vec![
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/opt/intel/oneapi/mkl/latest/lib"),
            PathBuf::from("/opt/miniconda3/lib"),
            PathBuf::from(r"C:\Program Files (x86)\Intel\oneAPI\mkl\latest\lib"),
            PathBuf::from(r"D:\Program Files (x86)\Intel\oneAPI\mkl\latest\lib"),
            PathBuf::from(r"C:\Program Files\Intel\oneAPI\mkl\latest\lib"),
            PathBuf::from(r"D:\Program Files\Intel\oneAPI\mkl\latest\lib"),
        ];
        if let Ok(home) = std::env::var("HOME") {
            possible_dir_paths
                .push(PathBuf::from_iter([&home, "intel", "oneapi", "mkl", "latest", "lib"]));
            possible_dir_paths.push(PathBuf::from_iter([&home, "miniconda3", "lib"]));
        }
        if let Some(home) = std::env::home_dir() {
            let home = home.to_string_lossy().to_string();
            possible_dir_paths
                .push(PathBuf::from_iter([&home, "intel", "oneapi", "mkl", "latest", "lib"]));
            possible_dir_paths.push(PathBuf::from_iter([&home, "miniconda3", "lib"]));
        }
        if let Ok(mkl_root) = env_mkl_root {
            possible_dir_paths.push(PathBuf::from(mkl_root).join("lib"));
        }
        if let Ok(mklroot) = env_mklroot {
            possible_dir_paths.push(PathBuf::from(mklroot).join("lib"));
        }

        let mut possible_lib_paths = vec![mkl_rt_name.clone()];
        for path in possible_dir_paths {
            possible_lib_paths.push(path.join(&mkl_rt_name).to_string_lossy().into());
        }

        candidates.extend(possible_lib_paths);
        candidates
    }

    pub(crate) fn panic_no_lib_found<S: Debug>(candidates: &[S]) -> ! {
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
"#
        )
    }
}
