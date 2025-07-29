# Basic `blas_int`, `lapack_int` definition and CBLAS enums

This crate serves as the very elementary definitions for all BLAS distributions (controlled by cargo feature `ilp64`):
- `blas_int` for BLAS integers (used for dimensions)
- `lapack_int` for LAPACK integers (used for dimensions, flags and integer scratchs `iwork`)

This crate also defines CBLAS enums.

## Cargo feature

- `ilp64`: control number of bits of `blas_int` and `lapack_int`.
- `lp64_as_int`: use `core::ffi::c_int` instead of `i32` as integer of LP64 scheme.

Both cargo features are not enabled by default.

| Scheme | `ilp64` | `lp64_as_int` | Integer Type |
|--|--|--|--|
| LP64 | No | No | `i32` |
| LP64 | No | Yes | `core::ffi::c_int` |
| ILP64 | Yes | Ignored | `i64` |
