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

# # Bindgen of FLAME (FLAME.h)

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

# ## Pre-Process

with open("FLAME.h", "r") as f:
    token = f.read()

# remove omp.h include
token = token.replace("#include <omp.h>", "#include \"omp.h\"")

with open("FLAME_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "FLAME_rstsr.h", "-o", "flame.rs",
    "--allowlist-file", "FLAME_rstsr.h",
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

with open("flame.rs", "r") as f:
    token = f.read()

token = token.replace("::core::ffi::", "").replace("::core::option::", "")
token = """pub(crate) use crate::blis_types::*;\n\n""" + token

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "flame"

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
