# # Bindgen of KML (kml_kblas.h)

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

with open("kblas.h", "r") as f:
    token = f.read()

# ARM __fp16 to GCC
token = token.replace("__fp16", "_Float16")

with open("kblas_rstsr.h", "w") as f:
    f.write(token)

# ## Bindgen

subprocess.run([
    "bindgen",
    "kblas_rstsr.h", "-o", "kblas.rs",
    "--allowlist-file", "kblas_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ## Post-Processing

with open("kblas.rs") as f:
    token = f.read()

# +
# simplify
# remove definition: complex, fp16, cblas-enums

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token = re.sub(r"#\[derive[^=]*?struct __BindgenComplex<T> {[^=]*?}", "", token)
token = re.sub(r"#\[derive[^=]*?struct __BindgenFloat16[^;]*?;", "", token)
token = token.replace("__BindgenComplex<f32>", "Complex<f32>")
token = token.replace("__BindgenComplex<f64>", "Complex<f64>")
token = token.replace("__BindgenFloat16", "f16")

token = token.replace("pub use self::CBLAS_LAYOUT as CBLAS_ORDER;", "")
token = token.replace("pub use self::CBLAS_ORDER as CBLAS_LAYOUT;", "")
token = re.sub(r"\#\[repr[^=]*CBLAS_LAYOUT {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_ORDER {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_TRANSPOSE {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_UPLO {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_DIAG {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_SIDE {[^#]*?}", "", token)

# +
# remove ilp64 related

token = re.sub(r"pub type BLASINT\b.*?;", "", token)
token = re.sub(r"\bBLASINT\b", "blas_int", token)

# +
# add type definition

token = "pub(crate) use crate::kml_types::*;\n\n" + token
# -

files_split = util_dyload.dyload_main(token)

# +
dir_relative = "kblas"

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

subprocess.run(["cargo", "fmt", "-p", "rstsr-kml-ffi"])
