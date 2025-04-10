# FFI bindings for math-related libraries

This project is a collection of (yet another) math-related libraries FFI bindings.

This project is certainly developed to serve (or potentially serve) rust tensor toolkit RSTSR and rust electronic structure toolkit REST. However, though there's `rstsr` in FFI binding names, this repository is technically not related to RSTSR (no dependencies related to RSTSR or its components).

## Motivation

Motivation of this repository, is that we want some of the following features:

- **Preprocessor directives support**. This is especially for `ILP64`, where integers can be `int`, `int32_t` or `int64_t`, which will also affect signature in FFI bindings.
- **Utilities from BLAS distributions**. For example, To use BLAS with proper threading, we may need to use utilities that is written from BLAS distributions, instead of reference (netlib) BLAS.
- **Additional BLAS extensions**. Many current BLAS implementations have some useful BLAS extensions (such as batched gemm, complex gemm3m, half-precision gemm, i/omatcopy, etc.). Some of these extensions may be essential (more than non-negligible) to efficiency (such as application in machine learning). However, it is difficult (or not very proper) to declare these BLAS extensions in one crate.

Not all BLAS distributions agree with the same API convention for the points list above. As a result, we cannot simply rely on a single FFI binding crate to cover all these aspects, but need to implement for each BLAS distribution.

Babel tower is awesome for sure, however, when it is not built, we'd better still do something dirty and quick.

## Limitations

- This project just perform FFI bindings. No safe wrappers (especially for any computationally intensive working functions). Possibly minimal utility wrappers (such as results) in future, but that will not be promised.
- We will probably only implement the latest version. Since it is FFI, non-existed function will not cause compile error if not linked; so whenever there is no API breaking changes, latest version should always fulfill demands of FFI callers. It is clearly better that FFI are also versioned along with FFI, so if you encountered problems or inconvenience related to library version, please raise issue for that.
