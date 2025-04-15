# # Bindgen of OpenBLAS

# This python file can also be opened by Jupyter notebook with jupytext extension.

# User must change `path_repo` to the local path of Netlib LAPACK repository.

import subprocess
import os
import shutil

path_cwd = os.path.abspath(os.getcwd())

# ## Bindgen configuration

# Users may change the following fields for their needs.

# Source code of Netlib Lapack
path_repo = f"{os.getenv('HOME')}/Software/source/OpenBLAS-0.3.29"

# Path for storing useful header files
path_header = f"{path_cwd}/../header"

# Path for temporary files
path_temp = f"{path_cwd}/tmp"

# Path for bindgen crate root
path_out = f"{path_cwd}/.."

# ## Copy necessary headers

# ### Copy to header directory

os.makedirs(path_header, exist_ok=True)
os.makedirs(f"{path_out}/src", exist_ok=True)

for name in ["common_interface.h", "cblas.h", "openblas_config_template.h"]:
    shutil.copy(f"{path_repo}/{name}", f"{path_header}")

# +
# make a copy of config files

shutil.move(f"{path_header}/openblas_config_template.h", f"{path_header}/common.h")
# -

# ### Copy to temporary directory

shutil.rmtree(path_temp, ignore_errors=True)
shutil.copytree(path_header, path_temp)

# +
# From now on, we will always work in temporary directory

os.chdir(path_temp)
# -

# ## BLAS handling (common interface)

# ### Pre-processing (common.h)

with open("common.h", "r") as f:
    token = f.read()

# +
# use typedef for xdouble

token = token.replace("#define xdouble double", "typedef double xdouble;")
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

token = token.replace("blasint", "blas_int")

# +
# remove cargo-feature related parts

token = token.replace("pub type xdouble = f64;", "")
token = token.replace("pub type blas_int = ::core::ffi::c_int;", "")

# +
# remove somehow redundant code

token = token.replace("::core::ffi::c_char", "c_char")
token = token.replace("::core::ffi::c_void", "c_void")
token = token.replace("::core::ffi::c_int", "c_int")
token = token.replace("::core::option::Option", "Option")

# +
# add headers

token = """
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void, c_int};

#[cfg(not(feature = "ilp64"))]
pub type blas_int = i32;
#[cfg(feature = "ilp64")]
pub type blas_int = i64;

#[cfg(feature = "quad_precision")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdouble {
    pub x: [::core::os::raw::c_ulong; 2usize],
}
#[cfg(feature = "ex_precision")]
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
""" + "\n\n" + token
# -

with open("f77blas.rs", "w") as f:
    f.write(token)

# ## CBLAS handling

# ### Pre-processing

with open("cblas.h", "r") as f:
    token = f.read()

token = token.replace("common.h", "common_parse.h")

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

token = token.replace("blasint", "blas_int")

# +
# remove cargo-feature related parts

token = token.replace("pub type blas_int = ::core::ffi::c_int;", "")

# +
# remove somehow redundant code

token = token.replace("::core::ffi::c_char", "c_char")
token = token.replace("::core::ffi::c_void", "c_void")
token = token.replace("::core::ffi::c_int", "c_int")
token = token.replace("::core::option::Option", "Option")

# +
# add headers

token = """
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void, c_int};

#[cfg(not(feature = "ilp64"))]
pub type blas_int = i32;
#[cfg(feature = "ilp64")]
pub type blas_int = i64;
""" + "\n\n" + token
# -

with open("cblas.rs", "w") as f:
    f.write(token)

# ## Move FFI binding files to output

for name in ["f77blas.rs", "cblas.rs"]:
    shutil.copy(f"{path_temp}/{name}", f"{path_out}/src/{name}")

# ## Cargo fmt

os.chdir(path_out)

subprocess.run(["cargo", "fmt"])
