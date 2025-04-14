# OpenBLAS FFI bindings

This crate contains OpenBLAS FFI bindings.

Current FFI version is (OpenBLAS v0.3.29)[https://github.com/OpenMathLib/OpenBLAS/releases/tag/v0.3.29].

OpenBLAS (C/Fortran/ASM) source code is available on (github)[https://github.com/OpenMathLib/OpenBLAS].

This crate is not official bindgen project. It is originally intended to serve rust tensor toolkit [RSTSR](https://github.com/RESTGroup/rstsr) and rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

- **Audience**: Anyone uses OpenBLAS function may also find it useful, not only RSTSR or REST program developers.
- **FFI-only**: Program developer is not required to actually install every components in LAPACK (especially LAPACKE). Rust FFI bindings is only declaration.
- **Do not include LAPACK and LAPACKE**: For LAPACK or LAPACKE, we refer to netlib LAPACK FFI bindings (such as (`lapack-sys`)[https://crates.io/crates/lapack-sys] or `rstsr-lapack-ffi`). OpenBLAS's declaration of LAPACK functions is no difference to netlib LAPACK, but only different in implementation.

## Cargo features

- `ilp64`: Use `int64_t` for dimension specification, or lapack error code types if this feature specified. Otherwise, use `int32_t`.
    - Please note that in `f77blas.rs`, error code is returned by `c_int`; in `cblas.rs`, OpenBLAS utility functions use `c_int` for input or output.
- `quad_precision` and `ex_precision`: Specify type extra-precision `xdouble` type in `f77blas.rs`. Probably few people will require these features.

## Crate structure

- `header`: Header files copied (or renamed) from original source.
- `scripts`: Script to generate FFI bindgens.
- `src`: FFI bindings:
    - `f77blas`: Corresponds to `f77blas.h` header in compiled library include (or `common_interface.h` in OpenBLAS original source code). BLAS and some LAPACK functions that implemented by OpenBLAS, no post-suffix underscore.
    - `cblas`: Corresponds to `cblas.h` header. Original CBLAS included, along with OpenBLAS utility functions (e.g. `openblas_get_num_threads`) and some BLAS extensions (e.g. `cblas_zgemm_batch`).
