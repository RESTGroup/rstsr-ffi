#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const MOD_NAME: &str = module_path!();

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_specific {
    use super::*;
    use crate::get_lib_candidates::*;
    use crate::*;
    use libloading::Library;
    use std::fmt::Debug;
    use std::sync::OnceLock;

    fn check_lib_loaded(lib: &DyLoadLib) -> bool {
        lib.dsyevd_.is_some()
    }

    fn panic_condition_not_met<S: Debug>(candidates: &[S]) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Unable to dynamically load the {LIB_NAME_SHOW} (`{LIB_NAME_LINK}`) shared library, due to condition unfulfilled.
Condition: `dsyevd_` not found.
Found libraries: {candidates:#?}

Please check
- if dynamic-loading is not desired, please disable the `dynamic_loading` feature in your `Cargo.toml` (by something like --no-default-features).
- if you want to provide custom {LIB_NAME_SHOW} library, use environment variable `RSTSR_DYLOAD_{LIB_NAME}` or `RSTSR_DYLOAD` to specify the path to the library.
- sequence of libraries matters: `RSTSR_DYLOAD_{LIB_NAME}` will be tried first, then `RSTSR_DYLOAD`, then system dynamic library search paths.
"#
        )
    }

    pub unsafe fn dyload_lib() -> &'static DyLoadLib {
        static LIB: OnceLock<DyLoadLib> = OnceLock::new();

        LIB.get_or_init(|| {
            let candidates = get_lib_candidates();
            let (mut libraries, mut libraries_path) = (vec![], vec![]);
            for candidate in &candidates {
                if let Ok(l) = Library::new(candidate) {
                    libraries.push(l);
                    libraries_path.push(candidate.to_string());
                    break; // currently multiple lib not allowed in this crate
                }
            }
            let lib = DyLoadLib::new(libraries, libraries_path);
            if lib.__libraries.is_empty() {
                panic_no_lib_found(&candidates);
            }
            if !check_lib_loaded(&lib) {
                panic_condition_not_met(&lib.__libraries_path);
            }
            lib
        })
    }
}

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading_specific::*;

/* #region general configuration */

pub(crate) mod ffi_base;
pub use ffi_base::*;

#[cfg(not(feature = "dynamic_loading"))]
pub(crate) mod ffi_extern;
#[cfg(not(feature = "dynamic_loading"))]
pub use ffi_extern::*;

#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_compatible;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_initializer;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_struct;

#[cfg(feature = "dynamic_loading")]
pub use dyload_compatible::*;
#[cfg(feature = "dynamic_loading")]
pub use dyload_struct::*;

/* #endregion */

#[test]
fn playground() {
    // test libraries loaded
    let time = std::time::Instant::now();
    let l = unsafe { dyload_lib() };
    println!("Time taken to load LAPACK library: {:?}", time.elapsed());
    println!("Loaded LAPACK library: {:?}", l.__libraries);
    println!("Loaded LAPACK library: {:?}", l.dsyevd_);
    unsafe { std::env::set_var("MKL_VERBOSE", "1") };

    // test if the library is loaded correctly
    let mut a: Vec<f64> = vec![2.0, 1.0, 3.0, 4.0];
    let uplo = 'L';
    let n = 2;
    let lda = 2;
    let mut info = 0;
    unsafe {
        dpotrf_(&uplo as *const _ as *const _, &n, a.as_mut_ptr(), &lda, &mut info);
    }
    println!("{a:?}");
    assert_eq!(info, 0);
}
