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

# # Bindgen of MKL (mkl_lapack.h)

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

with open("mkl_lapack.h", "r") as f:
    token = f.read()

token = token.replace("mkl_types.h", "mkl_types_rstsr.h")

with open("mkl_lapack_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "mkl_lapack_rstsr.h", "-o", "mkl_lapack.rs",
    "--allowlist-file", "mkl_lapack_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-Process

# For MKL LAPACK,
#
# - remove most type definitions, and use that of `crate::mkl_types`;
# - use types that with `SELECT_FUNCTION`;
# - exclude and add fn alias;

with open("mkl_lapack.rs", "r") as f:
    token = f.read()

parser = Parser(Language(tree_sitter_rust.language()))
tree = parser.parse(bytes(token.replace("unsafe extern", "extern"), "utf8"))
node_extern = tree.root_node.children[-1]
assert(node_extern.type == "foreign_mod_item")

# include only lower-case and underscore suffix functions
nodes_fn = [n for n in node_extern.children[1].children if n.type == "function_signature_item"]
identifiers = [n.children[2].text.decode() for n in nodes_fn]
identifiers_set = set()
for ident in identifiers:
    ident_trans = ident.lower().replace("_64", "")
    if ident_trans.endswith("_"):
        ident_trans = ident_trans[:-1]
    identifiers_set.add(ident_trans)

# +
with open("symbol_table.txt", "r") as f:
    symbol_table = {l.split()[2] for l in f.readlines() if len(l.split()) >= 3}

assert(len({ident.upper() for ident in identifiers_set} - set(symbol_table)) == 0)
assert(len({ident + "_" for ident in identifiers_set} - set(symbol_table)) == 0)
# -

token = """extern "C" {"""
identifiers_added = set()
for node_fn in nodes_fn:
    ident = node_fn.children[2].text.decode()
    ident_trans = ident.lower().replace("_64", "")
    if ident_trans.endswith("_"):
        ident_trans = ident_trans[:-1]
    if ident_trans in identifiers_added: continue
    identifiers_added.add(ident_trans)
    token += node_fn.text.decode().replace(ident, ident_trans + "_")
token += "}"
token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token_alias = "\n"
token_alias += "/* #region upper case alias */"
token_alias += "\n\n"
for ident in identifiers_set:
    token_alias += f"pub use {ident}_ as {ident.upper()};\n"
token_alias += "\n\n"
token_alias += "/* #endregion */"
token_alias += "\n\n"
token_alias += "/* #region lower case with underscore alias */"
token_alias += "\n\n"
for ident in identifiers_set:
    token_alias += f"pub use {ident}_ as {ident};\n"
token_alias += "\n\n"
token_alias += "/* #endregion */"

# The following code handles type definition

nodes_type_select = [
    n for n in tree.root_node.children
    if (n.type == "type_item" and "SELECT" in n.children[2].text.decode())]

# Handles split

# +
files_split = util_dyload.dyload_main(token)

ffi_base = files_split["ffi_base"]
ffi_base += "\n\n"
ffi_base += "pub use crate::mkl_types::*;";
ffi_base += "\n\n"
for node_type_select in nodes_type_select:
    ffi_base += node_type_select.text.decode() + "\n"
ffi_base = ffi_base.replace("::core::ffi::", "").replace("::core::option::", "")

ffi_extern = files_split["ffi_extern"]
ffi_extern += token_alias

dyload_compatible = files_split["dyload_compatible"]
dyload_compatible += token_alias

# +
dir_relative = "lapack"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, string in [
    ("ffi_base", ffi_base),
    ("ffi_extern", ffi_extern),
    ("dyload_initializer", files_split["dyload_initializer"]),
    ("dyload_struct", files_split["dyload_struct"]),
    ("dyload_compatible", dyload_compatible),
    ("mod_template", files_split["mod_template"])
]:
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(string)
# -

shutil.copytree(f"{path_temp}/{dir_relative}", f"{path_out}/src/{dir_relative}", dirs_exist_ok=True)

# ## Finalize

subprocess.run(["cargo", "fmt", "-p", "rstsr-mkl-ffi"])
