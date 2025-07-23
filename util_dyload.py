"""
Python script to handle dynamic loading

This is a utility python package that may be used across FFI bindings, for RSTSR-FFI project only.

This package handles
- splitting `unsafe extern` block out from FFI file;
- creating library struct for dynamic loading;
- creating FFI-compatible functions for dynamic loading.
"""

from tree_sitter import Language, Parser
import tree_sitter_rust


def dyload_parse_file(token):
    parser = Parser(Language(tree_sitter_rust.language()))
    token_transformed = token.replace("unsafe extern \"C\"", "extern \"C\"")
    parsed = parser.parse(bytes(token_transformed, "utf8"))
    parsed_ffi = []
    for node in parsed.root_node.children:
        if node.type == "foreign_mod_item":
            parsed_ffi.append(node)
    assert(len(parsed_ffi) == 1)
    return parsed, parsed_ffi[0]


def dyload_remove_extern(parsed, node_extern):
    return parsed.root_node.text.decode("utf8").replace(node_extern.text.decode("utf8"), "")


def dyload_get_ffi_fn(node):
    assert(node.type == "foreign_mod_item")
    return [node for node in node.children[-1].children if node.type == "function_signature_item"]


def dyload_fn_split(node):
    assert(node.type == "function_signature_item")
    keys = ["visibility_modifier", "identifier", "parameters", "return_type"]
    result = { key: None for key in keys }
    for (idx, child) in enumerate(node.children):
        if child.type == "->":
            result["return_type"] = node.children[idx + 1]
        elif child.type in keys:
            result[child.type] = child
    assert(result["identifier"] is not None)
    assert(result["parameters"] is not None)
    return result


def dyload_parameter_identifiers(node):
    assert(node.type == "parameters")
    return [n.children[0] for n in node.children if n.type == "parameter"]


def dyload_main(token, token_extra=None):
    # 1. obtain all stuffs for usual ffi use cases
    parsed, node_extern = dyload_parse_file(token)
    token_ffi_base = dyload_remove_extern(parsed, node_extern)

    # 2. prepare necessary tokens for output
    token_ffi_extern = ""
    token_dyload_struct = ""
    token_dyload_initializer = ""
    token_dyload_compatible = ""
    
    nodes_fn = dyload_get_ffi_fn(node_extern)
    nodes_fn_extra = None
    if token_extra is not None:
        parser = Parser(Language(tree_sitter_rust.language()))
        parsed_extra = parser.parse(bytes(token_extra, "utf8"))
        nodes_fn_extra = dyload_get_ffi_fn(parsed_extra.root_node.children[0])
        nodes_fn = nodes_fn + nodes_fn + nodes_fn_extra
    identifiers_fn = []

    # 3. iterate by functions
    for node_fn in nodes_fn:
        dict_fn = dyload_fn_split(node_fn)
        para_fn = dyload_parameter_identifiers(dict_fn["parameters"])

        visibility_modifier = dict_fn["visibility_modifier"].text.decode("utf8")
        identifier = dict_fn["identifier"].text.decode("utf8")
        parameters = dict_fn["parameters"].text.decode("utf8")
        return_type_string = ""
        if dict_fn["return_type"] is not None:
            return_type_string = " -> " + dict_fn["return_type"].text.decode("utf8")
        parameters_called = ", ".join([n.text.decode("utf8") for n in para_fn])

        part_dyload_struct = f"""
            {visibility_modifier} {identifier}: Option<unsafe extern "C" fn{parameters}{return_type_string}>,
        """.strip()
        part_dyload_initializer = f"""
            {identifier}: get_symbol(&libs, b"{identifier}\\0").map(|sym| *sym),
        """.strip()
        part_dyload_compatible = f"""
            {visibility_modifier} unsafe fn {identifier}{parameters}{return_type_string} {{
                dyload_lib().{identifier}.unwrap()({parameters_called})
            }}
        """.strip()

        token_dyload_struct += part_dyload_struct + "\n"
        token_dyload_initializer += part_dyload_initializer + "\n"
        token_dyload_compatible += part_dyload_compatible + "\n\n"
        identifiers_fn.append(identifier)

    # 4. finish all other stuffs
    
    # ffi_base.rs
    output_ffi_base = f"""
//! Base of current FFI.
//!
//! Declaration of types, enums, cargo feature controls, etc.
//!
//! This file is generated automatically.

{token_ffi_base}
    """

    output_ffi_extern = f"""
//! FFI function declarations for non-dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

unsafe {node_extern.text.decode("utf8")}
    """

    output_dyload_struct = f"""
//! Library struct definition for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;

pub struct Lib {{
    pub __libraries: Vec<libloading::Library>,
    {token_dyload_struct}
}}
    """

    output_dyload_initializer = f"""
//! Library initializer implementation for dynamic-loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{{Library, Symbol}};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {{
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}}

impl Lib {{
    pub unsafe fn new(libs: Vec<libloading::Library>) -> Lib {{
        let mut result = Lib {{
            __libraries: vec![], // dummy here, set this field later
            {token_dyload_initializer}
        }};
        result.__libraries = libs;
        result
    }}
}}
    """

    output_dyload_compatible = f"""
//! Compatible implementation for dynamic-loading.
//!
//! This requires custom `dyload_lib` definition in mod.rs, or visible from current layer of module.
//!
//! This file is generated automatically.

use super::*;

{token_dyload_compatible}
    """

    return {
        "ffi_base": output_ffi_base,
        "ffi_extern": output_ffi_extern,
        "dyload_struct": output_dyload_struct,
        "dyload_initializer": output_dyload_initializer,
        "dyload_compatible": output_dyload_compatible,
    }
