# Netlib LAPACK FFI bindings

This crate contains netlib (reference) LAPACK FFI bindings.

Current FFI version is [LAPACK v3.12.1](https://github.com/Reference-LAPACK/lapack/releases/tag/v3.12.1).

Netlib LAPACK (C/Fortran) source code is available on [github](https://github.com/Reference-LAPACK/lapack).

This crate is not official bindgen project. It is originally intended to serve rust tensor toolkit [RSTSR](https://github.com/RESTGroup/rstsr) and rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest).

- **Audience**: Anyone uses BLAS and LAPACK function may also find it useful, not only RSTSR or REST program developers.
- **FFI-only**: Program developer is not required to actually install every components in LAPACK (especially LAPACKE). Rust FFI bindings is only declaration. For example, you only need to install LAPACKE when you link LAPACKE functions into main program or library. In other words, as long as you do not link LAPACKE functions, compiler will not raise error if symbols of these functions are not found.

## Cargo features

- `ilp64`: Use `int64_t` for dimension specification, or lapack error code types if this feature specified. Otherwise, use `int32_t`.
    - Please note that in LAPACKE, matrix layout (mostly the first argument in LAPACKE functions) is always `core::ffi::c_int`, dependent to c compiler.

## Crate structure

- `header`: Header files copied (or renamed) from original source.
- `scripts`: Script to generate FFI bindgens.
- `src`: FFI bindings:
    - `blas`
    - `cblas`
    - `lapack`
    - `lapacke`

## Changelog

- v0.2

    - **API breaking change**: In v0.1, Fortran strlen option is included in f77blas and lapack binding. In v0.2, these arguments are muted. So for example, In v0.1 `dgemm_` will have 15 arguments: 13 arguments of usual usage + 2 arguments denotes string length of `transa` and `transb` arguments. In v0.2, `dgemm_` will have 13 arguments, the same to usual usage of Fortran equilvant.
