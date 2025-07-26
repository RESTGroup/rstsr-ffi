# # Bindgen of MKL (mkl_types.h)

import subprocess
import os
import shutil

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

with open("mkl_types.h", "r") as f:
    lines = f.readlines()

# use typedef instead of #define, for bindgen easy
for n, l in enumerate(lines):
    l = l.strip()
    if not l.startswith("#define"): continue
    l_split = l.split()
    if len(l_split) < 3: continue             # exclude flag define
    if l_split[2][0].isdigit(): continue      # exclude actual define
    # special exclude
    if not l_split[2][0].isalpha(): continue
    if "MKL_CBWR_OFF" in l: continue
    t = " ".join(l_split[2:])
    lines[n] = f"typedef {t} {l_split[1]};\n"

# +
token = "".join(lines)

with open("mkl_types_rstsr.h", "w") as f:
    f.write(token)
# -

# ## Bindgen

subprocess.run([
    "bindgen",
    "mkl_types_rstsr.h", "-o", "mkl_types.rs",
    "--allowlist-file", "mkl_types_rstsr.h",
    "--default-enum-style", "rust",
    "--no-layout-tests",
    "--use-core",
    "--merge-extern-blocks",
])

# Copy should be manually performed. This generation function does not handle the following automatic processes.

# ## Finalize

subprocess.run(["cargo", "fmt", "-p", "rstsr-mkl-ffi"])
