# # Bindgen of Netlib BLAS

# This python file can also be opened by Jupyter notebook with jupytext extension.

# User must change `path_repo` to the local path of Netlib LAPACK repository.

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

# Source code of Netlib Lapack
path_repo = f"{os.getenv('HOME')}/Git-Others/lapack"

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

for f in os.listdir(f"{path_repo}/CBLAS/include"):
    if f.endswith(".h") or f.endswith(".h.in"):
        shutil.copy(f"{path_repo}/CBLAS/include/{f}", f"{path_header}/{f}")

for f in os.listdir(f"{path_repo}/LAPACKE/include"):
    if f.endswith(".h") or f.endswith(".h.in"):
        shutil.copy(f"{path_repo}/LAPACKE/include/{f}", f"{path_header}/{f}")

# +
# make a copy of mangling files

shutil.move(f"{path_header}/cblas_mangling_with_flags.h.in", f"{path_header}/cblas_mangling.h")
shutil.move(f"{path_header}/lapacke_mangling_with_flags.h.in", f"{path_header}/lapacke_mangling.h")
# -

# ### Copy to temporary directory

shutil.rmtree(path_temp, ignore_errors=True)
shutil.copytree(path_header, path_temp)

# From now on, we will always work in temporary directory
os.chdir(path_temp)

# ## BLAS handling

# ### Pre-processing

# +
# concate files to blas_f77_parse.h

# - stdio.h (only for preventing error message)
# - cblas_f77.h
# - cblas_mangling.h

token = ""
token += "#include <stdio.h>\n"
with open("cblas_mangling.h", "r") as fin:
    token += fin.read()
token += "\n"
with open("cblas_f77.h", "r") as fin:
    token += fin.read()

# +
# change file cblas_f77_parse.h
# This only works when F77_INT is not defined

token = token.replace("#define F77_INT int32_t", "typedef int32_t F77_INT;")

# In C binding, fortran strlen end is probably not required
token = token.replace("#define BLAS_FORTRAN_STRLEN_END", "")

# +
# write file cblas_f77_parse.h

with open("cblas_f77_parse.h", "w") as fout:
    fout.write(token)
# -

# ### Bindgen

subprocess.run([
    "bindgen",
    "cblas_f77_parse.h", "-o", "blas.rs",
    "--allowlist-file", "cblas_f77_parse.h",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-processing (1)

with open("blas.rs", "r") as f:
    token = f.read()

# +
# change F77_INT to blas_int

token = token.replace("F77_INT", "blas_int")
token = token.replace("pub type blas_int = i32;", "")

# +
# remove somehow redundant code

token = token.replace("::core::ffi::", "").replace("::core::option::", "")

token = """
pub(crate) use core::ffi::*;
pub use rstsr_cblas_base::*;

""" + "\n\n" + token
# -

# ### Post-processing (2) Sub functions

# This only happens in F77 BLAS. Some BLAS are functions (instead of subroutines) by definition; however, Netlib BLAS only handles subroutines in C fashion: those functions are suffixed by `sub_`, and the returned value is located at the last parameter.
#
# After FFI, for example,
# ```rust
# pub fn dnrm2sub_(arg1: *const F77_INT, arg2: *const f64, arg3: *const F77_INT, arg4: *mut f64);
# ```
#
# Many BLAS distributions do not provide such subroutine `dnrm2sub_`, but instead the function:
# ```rust
# pub fn dnrm2_(arg1: *const F77_INT, arg2: *const f64, arg3: *const F77_INT) -> f64;
# ```
#
# So here we need to provide the definitions of those functions.

token = token.replace("unsafe extern", "extern")

parser = Parser(Language(tree_sitter_rust.language()))

parsed = parser.parse(bytes(token, "utf8"))

assert(parsed.root_node.children[-1].type == "foreign_mod_item")

nodes_fn = [n for n in parsed.root_node.children[-1].children[1].children
            if n.type == "function_signature_item"]

nodes_fn_sub = [n for n in nodes_fn if util_dyload.dyload_fn_split(n)["identifier"].text.decode().endswith("sub_")]


def sub_to_func(node):
    assert(node.type == "function_signature_item")
    fn_dict = util_dyload.dyload_fn_split(node)
    assert(fn_dict["identifier"].text.decode().endswith("sub_"))
    assert(fn_dict["return_type"] is None)
    # strip parameters
    parameters = [n for n in fn_dict["parameters"].children if n.type == "parameter"]
    parameters_normal = parameters[:-1]
    parameters_last = parameters[-1]
    assert(parameters_last.children[-1].type == "pointer_type")
    return_type = parameters_last.children[-1].children[-1].text.decode()
    # reformulate function token
    visibility_modifier = fn_dict["visibility_modifier"].text.decode()
    identifier_sub = fn_dict["identifier"].text.decode()
    identifier = identifier_sub[:-4] + "_"
    parameters_token = ", ".join([n.text.decode() for n in parameters_normal])
    return f"{visibility_modifier} fn {identifier}({parameters_token}) -> {return_type};"


token_sub_to_func = (
    "unsafe extern \"C\" {"
    + "\n".join([n.text.decode() for n in nodes_fn])
    + "\n".join([sub_to_func(n) for n in nodes_fn_sub])
    + "}"
)

token = token.replace(parsed.root_node.children[-1].text.decode(), token_sub_to_func)

# ### Dynamic loading

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

token = ""
with open("cblas.h", "r") as fin:
    token += fin.read()

# +
# change file cblas_parse.h
# This only works when CBLAS_INT is not defined

token = token.replace("#define CBLAS_INT int32_t", "typedef int32_t CBLAS_INT;")

# +
# write file cblas_parse.h

with open("cblas_parse.h", "w") as fout:
    fout.write(token)
# -

# ### Bindgen

subprocess.run([
    "bindgen",
    "cblas_parse.h", "-o", "cblas.rs",
    "--allowlist-file", "cblas_parse.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-processing

with open("cblas.rs", "r") as f:
    token = f.read()

# +
# remove CBLAS_IFMT, which seems not useful?

token = "\n".join([i for i in token.split("\n") if "CBLAS_IFMT" not in i])

# +
# change CBLAS_INT to blas_int

token = token.replace("CBLAS_INT", "blas_int")
token = token.replace("pub type blas_int = i32;", "")

# +
# remove CBLAS enums

token = re.sub(r"\#\[repr[^=]*CBLAS_LAYOUT {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_TRANSPOSE {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_UPLO {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_DIAG {[^#]*?}", "", token)
token = re.sub(r"\#\[repr[^=]*CBLAS_SIDE {[^#]*?}", "", token)

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
dir_relative = "cblas"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -

# ## LAPACK handling

# ### Pre-processing

token = ""
with open("lapack.h", "r") as fin:
    token += fin.read()

# +
token = token.replace("#define lapack_int        int32_t", "typedef int32_t lapack_int;")

# In C binding, fortran strlen end is probably not required
token = token.replace("#define LAPACK_FORTRAN_STRLEN_END", "")
# -

with open("lapack_parse.h", "w") as fout:
    fout.write(token)

# ### Bindgen

subprocess.run([
    "bindgen",
    "lapack_parse.h", "-o", "lapack.rs",
    "--allowlist-file", "lapack_parse.h",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-processing

with open("lapack.rs", "r") as f:
    token = f.read()

token = token.replace("pub type lapack_int = i32;", "")

# +
# remove CBLAS_IFMT, which seems not useful?

token = "\n".join([i for i in token.split("\n") if "LAPACK_IFMT" not in i])

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
dir_relative = "lapack"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -

# ## LAPACKE handling

# ### Pre-processing

token = ""
with open("lapacke.h", "r") as fin:
    token += fin.read()

token = token.replace('#include "lapack.h"', '#include "lapack_parse.h"')

with open("lapacke_parse.h", "w") as fout:
    fout.write(token)

# ### Bindgen

subprocess.run([
    "bindgen",
    "lapacke_parse.h", "-o", "lapacke.rs",
    "--allowlist-file", "lapacke_parse.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-processing

with open("lapacke.rs", "r") as f:
    token = f.read()

token = token.replace("pub type lapack_int = i32;", "")
token = token.replace("MAJOR: u32", "MAJOR: c_int")
token = token.replace("ERROR: i32", "ERROR: lapack_int")

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
dir_relative = "lapacke"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -

# ## LAPACKE_utils handling

# ### Pre-processing

token = ""
with open("lapacke_utils.h", "r") as fin:
    token += fin.read()

token = token.replace('#include "lapacke.h"', '#include "lapacke_parse.h"')

with open("lapacke_utils_parse.h", "w") as fout:
    fout.write(token)

# ### Bindgen

subprocess.run([
    "bindgen",
    "lapacke_utils_parse.h", "-o", "lapacke_utils.rs",
    "--allowlist-file", "lapacke_utils_parse.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# ### Post-processing

with open("lapacke_utils.rs", "r") as f:
    token = f.read()

token = token.replace("pub type lapack_int = i32;", "")

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
dir_relative = "lapacke_utils"

shutil.rmtree(dir_relative, ignore_errors=True)
os.makedirs(dir_relative)
for key, item in util_dyload.dyload_main(token).items():
    with open(f"{dir_relative}/{key}.rs", "w") as f:
        f.write(item)
# -

# ## Move FFI binding files to output

for name in ["blas", "cblas", "lapack", "lapacke", "lapacke_utils"]:
    shutil.copytree(f"{path_temp}/{name}", f"{path_out}/src/{name}", dirs_exist_ok=True)

# ## Cargo fmt

subprocess.run(["cargo", "fmt", "-p", "rstsr-lapack-ffi"])
