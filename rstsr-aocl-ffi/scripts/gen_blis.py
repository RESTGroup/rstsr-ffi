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
import re
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

# ## Pre-Process

with open("blis.h", "r") as f:
    token = f.read()

# remove omp.h include
token = token.replace("#include <omp.h>", "")

with open("blis_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "blis_rstsr.h", "-o", "blis.rs",
    "--allowlist-file", "blis_rstsr.h",
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
    if "BLIS_BLAS_INT_TYPE_SIZE" in l: continue
    if "pub type f77_int" in l: continue
    token += l
# -

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

# +
# remove CBLAS enums

token = re.sub(r"\#\[repr[^=]*CBLAS_LAYOUT {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_TRANSPOSE {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_UPLO {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_DIAG {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_SIDE {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_ORDER {[^#]*?}", "", token)
# -

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "blis"

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

subprocess.run(["cargo", "fmt", "-p", "rstsr-aocl-ffi"])
