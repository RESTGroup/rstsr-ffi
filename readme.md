# FFI bindings for math-related libraries

This project is a collection of (yet another) math-related libraries FFI bindings.

This project is originally intended to serve rust tensor toolkit [RSTSR](https://github.com/RESTGroup/rstsr) and rust electronic structure toolkit [REST](https://gitee.com/RESTGroup/rest). However, though there's `rstsr` in FFI binding names, this repository is technically not related to RSTSR (no dependencies related to RSTSR or its components).

## Current bindings

| crate | crate version | FFI library | FFI version |
|--|--|--|--|
| rstsr-cblas-base | [![Crate](https://img.shields.io/crates/v/rstsr-cblas-base.svg)](https://crates.io/crates/rstsr-cblas-base) |
| rstsr-lapack-ffi | [![Crate](https://img.shields.io/crates/v/rstsr-lapack-ffi.svg)](https://crates.io/crates/rstsr-lapack-ffi) | [Reference LAPACK](https://github.com/Reference-LAPACK/lapack) | v3.12.1 |
| rstsr-openblas-ffi | [![Crate](https://img.shields.io/crates/v/rstsr-openblas-ffi.svg)](https://crates.io/crates/rstsr-openblas-ffi) | [OpenBLAS](https://github.com/OpenMathLib/OpenBLAS/) | v0.3.30 |
| rstsr-mkl-ffi | [![Crate](https://img.shields.io/crates/v/rstsr-mkl-ffi.svg)](https://crates.io/crates/rstsr-mkl-ffi) | Intel oneAPI MKL | 2025.2 |
| rstsr-blis-ffi | [![Crate](https://img.shields.io/crates/v/rstsr-blis-ffi.svg)](https://crates.io/crates/rstsr-blis-ffi) | [BLIS](https://github.com/flame/blis), [FLAME](https://github.com/flame/libflame) | v2.0 |
| rstsr-aocl-ffi | [![Crate](https://img.shields.io/crates/v/rstsr-aocl-ffi.svg)](https://crates.io/crates/rstsr-aocl-ffi) | [AOCL](https://www.amd.com/en/developer/aocl.html) | v5.1 |

## Motivation

Motivation of this repository, is that we want some of the following features:

- **Dynamic loading support**. We support either usual FFI (requires library linking) or dynamic loading (load library in runtime) by switching crate feature `dynamic_loading`. Dynamic loading scheme is the default.
- **Preprocessor directives support**. This is especially for `ILP64`, where integers can be `int`, `int32_t` or `int64_t`, which will also affect signature in FFI bindings.
- **Utilities from BLAS distributions**. For example, To use BLAS with proper threading, we may need to use utilities that is written from BLAS distributions, instead of reference (netlib) BLAS.
- **Additional BLAS extensions**. Many current BLAS implementations have some useful BLAS extensions (such as batched gemm, complex gemm3m, half-precision gemm, i/omatcopy, etc.). Some of these extensions may be essential (more than non-negligible) to efficiency (such as application in machine learning). However, it is difficult (or not very proper) to declare these BLAS extensions in one crate.

Not all BLAS distributions agree with the same API convention for the points list above. As a result, we cannot simply rely on a single FFI binding crate to cover all these aspects, but need to implement for each BLAS distribution.

Babel tower is awesome for sure, however, when it is not built, we'd better still do something dirty and quick.

## Limitations

- This project just perform FFI bindings. No safe wrappers (especially for any computationally intensive working functions). Possibly minimal utility wrappers (such as results) in future, but that will not be promised.
- We will probably only implement the latest version. Since it is FFI, non-existed function will not cause compile error if not linked; so whenever there is no API breaking changes, latest version should always fulfill demands of FFI callers. It is clearly better that FFI are also versioned along with FFI, so if you encountered problems or inconvenience related to library version, please raise issue for that.
- If you encountered large compile time or disk consumption, you may consider add these lines in your Cargo.toml (for example of MKL bindgens):

    ```toml
    # this is only for MKL bindgens
    # for other bindgen crates or compile targets, the configuration is similar
    [profile.dev.package.rstsr-mkl-ffi]
    opt-level = 0
    debug = false
    ```

## License

This project is licensed to Apache v2.

It should be noted that, this project includes some headers directly copied from other open-source or closed-source libraries. These files should be considered to have its original license. We include these files only for convenience/clarification and for FFI binding generation, but not actually using these files.
- Netlib (reference) LAPACK: BSD-3-Clause
- OpenBLAS: BSD-3-Clause
- oneMKL (of oneAPI): Intel Simplified Software License (Version October 2022)
- BLIS: BSD-3-Clause
- AOCL: Mostly BSD-3-Clause
- KML: Kunpeng BoostKit License Agreement 2.0

### Note on licenses

For Apache, BSD-3, or Intel Simplified Software License, it is not so easy to breach license restrictions, if you just use them for academic purpose or efficiency benchmark in the domain of scientific computation.

However, **Kunpeng BoostKit License Agreement is a relatively restrictive license**. Even usual scientific computation user can breach license in some cases, for example:
- 3.1 Without Huawei's prior written consent, you shall not sell, rent, lend, share, disclose, sublicense, or distribute all or part of the Kunpeng BoostKit to third parties for profit purposes.
    - Please do not use `rstsr-kml-ffi` for profiting products, unless consented by Huawei.
- 3.2.7 Disclosing Kunpeng BoostKit-related data, including but not limited to performance evaluation data, without Huawei's written consent;
    - Do not perform efficiency benchmark with `rstsr-kml-ffi`, unless consented by Huawei.
