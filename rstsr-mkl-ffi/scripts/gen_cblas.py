# # Bindgen of MKL (mkl_cblas.h)

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

# ## Pre-Processing

with open("mkl_cblas.h", "r") as f:
    token = f.read()

token = token.replace("mkl_types.h", "mkl_types_rstsr.h")

with open("mkl_cblas_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "mkl_cblas_rstsr.h", "-o", "mkl_cblas.rs",
    "--allowlist-file", "mkl_cblas_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-Process

# For MKL CBLAS,
#
# - remove all most type definitions, and use that of `crate::mkl_types` and `rstsr_lapack_ffi::cblas`;

with open("mkl_cblas.rs", "r") as f:
    token = f.read()

parser = Parser(Language(tree_sitter_rust.language()))
tree = parser.parse(bytes(token.replace("unsafe extern", "extern"), "utf8"))
node_extern = tree.root_node.children[-1]
assert(node_extern.type == "foreign_mod_item")

# +
# remove somehow redundant code

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token = """
pub use crate::mkl_types::*;

""" + "\n\n" + token

# +
# remove CBLAS enums

token = token.replace("pub type MKL_INT = c_int;", "")
token = token.replace("pub use self::CBLAS_LAYOUT as CBLAS_ORDER;", "")
token = re.sub(r"\#\[repr[^=]*CBLAS_LAYOUT {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_TRANSPOSE {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_UPLO {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_DIAG {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_SIDE {[^#]*?}", "", token)
# -

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "cblas"

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
