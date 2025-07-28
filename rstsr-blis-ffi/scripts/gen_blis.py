# ---
# jupyter:
#   jupytext:
#     text_representation:
#       extension: .py
#       format_name: light
#       format_version: '1.5'
#       jupytext_version: 1.16.4
#   kernelspec:
#     display_name: Python 3 (ipykernel)
#     language: python
#     name: python3
# ---

# # Bindgen of BLIS (blis.h)

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

# ## Bindgen

subprocess.run([
    "bindgen",
    "blis.h", "-o", "blis.rs",
    "--allowlist-file", "blis.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-Process

# For MKL BLAS,
#
# - remove all type definitions, and use that of `crate::mkl_types`;
# - exclude all upper-case functions, and add fn alias;

with open("blis.rs", "r") as f:
    token_lines = f.readlines()

# +
token = """
pub use crate::blis_types::*;

"""

for l in token_lines:
    if "BLIS_INT_TYPE_SIZE" in l: continue
    if "pub type f77_int" in l: continue
    token += l
# -

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "blis_x86_64"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, string in [
    ("ffi_base_template", files_split["ffi_base"]), # ffi_base needs to be manually modified for CBLAS
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

subprocess.run(["cargo", "fmt", "-p", "rstsr-blis-ffi"])
