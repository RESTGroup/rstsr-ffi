# # Bindgen of MKL (mkl_lapacke.h)

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

with open("mkl_lapacke.h", "r") as f:
    token = f.read()

token = token.replace("mkl_types.h", "mkl_types_rstsr.h")

with open("mkl_lapacke_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "mkl_lapacke_rstsr.h", "-o", "mkl_lapacke.rs",
    "--allowlist-file", "mkl_lapacke_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-process

# For MKL LAPACK,
#
# - remove most type definitions, and use that of `crate::lapack`;
# - exclude all f64 functions;

with open("mkl_lapacke.rs", "r") as f:
    token = f.read()

parser = Parser(Language(tree_sitter_rust.language()))
tree = parser.parse(bytes(token.replace("unsafe extern", "extern"), "utf8"))
node_extern = tree.root_node.children[-1]
assert(node_extern.type == "foreign_mod_item")
nodes_fn = [n for n in node_extern.children[1].children if n.type == "function_signature_item"]

nodes_type_select = [
    n for n in tree.root_node.children
    if (n.type == "type_item" and "SELECT" in n.children[2].text.decode())]

token = """
pub use crate::mkl_types::*;

pub const LAPACK_ROW_MAJOR: u32 = 101;
pub const LAPACK_COL_MAJOR: u32 = 102;
pub const LAPACK_WORK_MEMORY_ERROR: i32 = -1010;
pub const LAPACK_TRANSPOSE_MEMORY_ERROR: i32 = -1011;

"""
for node_type_select in nodes_type_select:
    token += node_type_select.text.decode() + "\n"
token += """
extern "C" {
"""
for node_fn in nodes_fn:
    if node_fn.children[2].text.decode().endswith("_64"): continue
    token += node_fn.text.decode()
token += "}"
token = token.replace("::core::ffi::", "").replace("::core::option::", "")

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "lapacke"

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
