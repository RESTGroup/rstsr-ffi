# # Bindgen of MKL (mkl_service.h)

import subprocess
import os
import shutil
from tree_sitter import Language, Parser
import tree_sitter_rust

import sys
sys.path.append("../..")
import util_dyload

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

shutil.copytree(path_header, path_temp, dirs_exist_ok=True)

# +
# From now on, we will always work in temporary directory

os.chdir(path_temp)
# -

# ## Pre-Processing

with open("mkl_service.h", "r") as f:
    token = f.read()

token = token.replace("mkl_types.h", "mkl_types_rstsr.h")
token = token.replace("unsigned MKL_INT64", "MKL_UINT64")

with open("mkl_service_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "mkl_service_rstsr.h", "-o", "mkl_service.rs",
    "--allowlist-file", "mkl_service_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-Process

with open("mkl_service.rs", "r") as f:
    token = f.read()

# +
token = token.replace("""
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKLVersion {
    pub MajorVersion: ::core::ffi::c_int,
    pub MinorVersion: ::core::ffi::c_int,
    pub UpdateVersion: ::core::ffi::c_int,
    pub PatchVersion: ::core::ffi::c_int,
    pub ProductStatus: *mut ::core::ffi::c_char,
    pub Build: *mut ::core::ffi::c_char,
    pub Processor: *mut ::core::ffi::c_char,
    pub Platform: *mut ::core::ffi::c_char,
}
pub type MKL_INT64 = ::core::ffi::c_longlong;
pub type MKL_UINT64 = ::core::ffi::c_ulonglong;
""".strip(), "")

token = """pub use crate::mkl_types::*;\n\n""" + token
token = token.replace("::core::ffi::", "").replace("::core::option::", "")
# -

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "service"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, string in [
    ("ffi_base", files_split["ffi_base"]),
    ("ffi_extern", files_split["ffi_extern"]),
    ("dyload_initializer", files_split["dyload_initializer"]),
    ("dyload_struct", files_split["dyload_struct"]),
    ("dyload_compatible", files_split["dyload_compatible"]),
    ("mod_template", files_split["mod_template"])
]:
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(string)
# -

shutil.copytree(f"{path_temp}/{dir_relative}", f"{path_out}/src/{dir_relative}", dirs_exist_ok=True)

# ## Finalize

subprocess.run(["cargo", "fmt", "-p", "rstsr-mkl-ffi"])
