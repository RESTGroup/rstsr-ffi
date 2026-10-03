# # Bindgen of OpenBLAS

# This python file can also be opened by Jupyter notebook with jupytext extension.

# The vendored headers in `../header` are the binding input; refresh them from the
# OpenBLAS checkout at the target release tag before running (see skill `update-ffi-blas`).

import subprocess
import os
import shutil
import re

import sys
sys.path.append("../..")
import util_dyload


def replace_required(token, old, new):
    """Text patch that must fire; a silent no-op means upstream changed format."""
    assert old in token, f"patch target missing: {old!r}"
    return token.replace(old, new)


def sub_required(pattern, repl, token):
    """Regex patch that must fire; a silent no-op means upstream changed format."""
    token, n = re.subn(pattern, repl, token)
    assert n > 0, f"patch pattern matched nothing: {pattern!r}"
    return token


def assert_absent(pattern, token, what):
    """Post-condition for a removal patch: the target must be gone."""
    match = re.search(pattern, token)
    assert match is None, f"{what} still present after post-processing: {match.group(0)!r}"


path_cwd = os.path.abspath(os.getcwd())

# ## Bindgen configuration

# Users may change the following fields for their needs.

# Path for storing useful header files
path_header = f"{path_cwd}/../header"

# Path for temporary files
path_temp = f"{path_cwd}/tmp"

# Path for bindgen crate root
path_out = f"{path_cwd}/.."

# ## Copy necessary headers

# ### Copy to temporary directory

shutil.rmtree(path_temp, ignore_errors=True)
shutil.copytree(path_header, path_temp)

# +
# From now on, we will always work in temporary directory

os.chdir(path_temp)
# -

# ## BLAS handling (common interface)

# ### Pre-processing (common.h)

with open("openblas_config_template.h", "r") as f:
    token = f.read()

# +
# use typedef for xdouble

token = replace_required(token, "#define xdouble double", "typedef double xdouble;")
token = "#define OPENBLAS_NEEDBUNDERSCORE\n" + token
# -

with open("common_parse.h", "w") as f:
    f.write(token)

# ### Pre-processing

with open("common_interface.h", "r") as f:
    token = f.read()

# +
# use typedef for xdouble

token = """
#include "common_parse.h"
""" + token;
# -

with open("common_interface_parse.h", "w") as f:
    f.write(token)

# ### Bindgen

subprocess.run([
    "bindgen",
    "common_interface_parse.h", "-o", "f77blas.rs",
    "--allowlist-file", "common_interface_parse.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
    "--",
    "-DFORCE_OPENBLAS_COMPLEX_STRUCT=1"
])

# ### Post-processing

with open("f77blas.rs", "r") as f:
    token = f.read()

# +
# rename blasint to blas_int

token = replace_required(token, "blasint", "blas_int")

# +
# remove cargo-feature related parts

token = replace_required(token, "pub type xdouble = f64;", "")
assert_absent(r"pub type\s+xdouble\s*=", token, "local xdouble type alias")
token = replace_required(token, "pub type blas_int = ::core::ffi::c_int;", "")
assert_absent(r"pub type\s+(blas_int|F77_INT)\s*=", token, "local blas_int type alias")

# +
# add headers

token = """

#[cfg(all(feature = "quad_precision", not(feature = "ex_precision")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdouble {
    pub x: [::core::ffi::c_ulong; 2usize],
}
#[cfg(all(feature = "ex_precision", not(feature = "quad_precision")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdouble {
    pub x: u128,
}
#[cfg(all(not(feature = "quad_precision"), not(feature = "ex_precision")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdouble {
    pub x: f64,
}
// This is a workaround for cargo feature conflict
#[cfg(all(feature = "quad_precision", feature = "ex_precision"))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdouble {
    pub _phantom: (),
}
""" + "\n\n" + token

# +
# remove somehow redundant code

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token = """
pub(crate) use core::ffi::*;
pub use rstsr_cblas_base::*;

""" + "\n\n" + token
# -

# ### Dynamic-loading

# +
dir_relative = "blas"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -
# ## CBLAS handling

# ### Pre-processing

with open("cblas.h", "r") as f:
    token = f.read()

token = replace_required(token, '#include "common.h"', '#include "common_parse.h"')

# Upstream declares the thread-affinity API under `#ifdef OPENBLAS_OS_LINUX`, and the
# config template includes <sched.h> for cpu_set_t under the same macro. Define it so
# the pair is bound, then gate the Rust items with #[cfg(target_os = "linux")] after
# bindgen; generation therefore requires a Linux host. The define stays valueless:
# a valued `#define` would leak into bindgen output as a `pub const` (upstream only
# tests definedness).
assert re.search(r"#ifdef\s+OPENBLAS_OS_LINUX\b", token), "OPENBLAS_OS_LINUX block not found in cblas.h"
token = "#define OPENBLAS_OS_LINUX\n" + token

with open("cblas_parse.h", "w") as f:
    f.write(token)

# ### Bindgen

subprocess.run([
    "bindgen",
    "cblas_parse.h", "-o", "cblas.rs",
    "--allowlist-file", "cblas_parse.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
    "--",
    "-DFORCE_OPENBLAS_COMPLEX_STRUCT=1"
])

# ### Post-processing

with open("cblas.rs", "r") as f:
    token = f.read()

# +
# rename blasint to blas_int

token = replace_required(token, "blasint", "blas_int")

# the OPENBLAS_OS_LINUX define injected above is build configuration, not API
assert_absent(r"OPENBLAS_OS_LINUX", token, "injected OPENBLAS_OS_LINUX macro leak")

# +
# remove cargo-feature related parts

token = replace_required(token, "pub type blas_int = ::core::ffi::c_int;", "")
assert_absent(r"pub type\s+(blas_int|CBLAS_INT)\s*=", token, "local blas_int type alias")

# +
# remove CBLAS enums

token = replace_required(token, "pub use self::CBLAS_ORDER as CBLAS_LAYOUT;", "")
# bindgen names the layout enum CBLAS_ORDER (CBLAS_LAYOUT appears only as the alias
# above), so each enum is matched by its own name.
token = sub_required(r"\#\[repr[^=]*CBLAS_TRANSPOSE {[^#]*?}", "", token)
token = sub_required(r"\#\[repr[^=]*CBLAS_UPLO {[^#]*?}", "", token)
token = sub_required(r"\#\[repr[^=]*CBLAS_DIAG {[^#]*?}", "", token)
token = sub_required(r"\#\[repr[^=]*CBLAS_SIDE {[^#]*?}", "", token)
token = sub_required(r"\#\[repr[^=]*CBLAS_ORDER {[^#]*?}", "", token)
assert_absent(
    r"pub (enum|struct|type)\s+CBLAS_(LAYOUT|TRANSPOSE|UPLO|DIAG|SIDE|ORDER)\b",
    token,
    "local CBLAS enum definition",
)
assert_absent(r"CBLAS_LAYOUT\b", token, "CBLAS_LAYOUT alias")

# Linux-only items: gate the affinity API and the sched.h types it uses.
for pattern in [
    r"pub fn openblas_setaffinity\b",
    r"pub fn openblas_getaffinity\b",
    r"pub struct cpu_set_t\b",
    r"pub type __cpu_mask\b",
]:
    token, n = re.subn(
        rf'(?m)^([ \t]*)({pattern})', r'\1#[cfg(target_os = "linux")]\n\1\2', token
    )
    assert n == 1, f"expected exactly one occurrence to gate, found {n}: {pattern!r}"

# +
# remove somehow redundant code

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token = """
pub(crate) use core::ffi::*;
pub use rstsr_cblas_base::*;

""" + "\n\n" + token
# -

# ### Dynamic-loading

# +
# openmp threading control

token_extra = """
extern "C" {
    pub fn omp_set_num_threads(arg1: c_int);
    pub fn omp_get_max_threads() -> c_int;
}
"""

# +
dir_relative = "cblas"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token, token_extra).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -

# ## Move FFI binding files to output

for name in ["blas", "cblas"]:
    shutil.copytree(f"{path_temp}/{name}", f"{path_out}/src/{name}", dirs_exist_ok=True)

# ## Cargo fmt

subprocess.run(["cargo", "fmt", "-p", "rstsr-openblas-ffi"])
