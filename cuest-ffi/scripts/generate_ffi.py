# # Generate FFI bindings for cuEST

# This python file can also be opened by Jupyter notebook with jupytext extension.

# This script generates:
# - ffi_static.rs: static linking FFI bindings
# - ffi_dynamic/: dynamic loading module files

# The cuEST public C headers are read from the installation pointed to by the
# `CUEST_INSTALL_DIR` environment variable. Headers are not vendored in this
# repository, since they are proprietary NVIDIA software.

# Requirements: `bindgen` CLI, python packages `tree-sitter` and `tree-sitter-rust`.

import os
import re
import shutil
import subprocess
from collections import Counter

from tree_sitter import Language, Parser
import tree_sitter_rust

path_cwd = (
    os.path.dirname(os.path.abspath(__file__))
    if "__file__" in globals()
    else os.path.abspath(os.getcwd())
)

# ## Bindgen configuration

# Path to the cuEST installation, expected to contain `include/cuest.h`.
path_repo = os.getenv("CUEST_INSTALL_DIR")
if not path_repo:
    raise SystemExit(
        "Environment variable CUEST_INSTALL_DIR must point to the cuEST "
        "installation (containing include/cuest.h)."
    )

# Umbrella header. All cuEST headers are self-contained pure C: no CUDA or
# system types occur in any signature, so bindgen needs no CUDA headers. The
# allowlists keep the generated bindings restricted to the public cuEST API;
# without them bindgen also emits typedefs and constants harvested from the
# system headers (<stdint.h>, <stddef.h>).
path_header = f"{path_repo}/include/cuest.h"

# Path for temporary files
path_temp = f"{path_cwd}/tmp"

# Path for bindgen crate root
path_out = f"{path_cwd}/.."


# ## Post-processing helpers


def strip_enum_prefixes(token):
    """Rename bindgen-prefixed enum constants to their bare C names.

    bindgen names the constants of a typedef'd C enum as `<enum_t>_<VARIANT>`
    (e.g. `cuestStatus_t_CUEST_STATUS_SUCCESS`), while the headers and their
    documentation use the bare variant name.
    """
    results = []
    for name in re.findall(r"pub const (\w+)\s*:", token):
        match = re.match(r"^\w+_t_([A-Z0-9_]+)$", name)
        results.append(match.group(1) if match else name)
    duplicates = [name for name, count in Counter(results).items() if count > 1]
    if duplicates:
        raise RuntimeError(f"Enum constant renaming produces duplicates: {duplicates}")

    def _replace(match):
        match_inner = re.match(r"^\w+_t_([A-Z0-9_]+)$", match.group(1))
        return f"pub const {match_inner.group(1)}" if match_inner else match.group(0)

    return re.sub(r"pub const (\w+)", _replace, token)


def remove_system_constants(token):
    """Drop constants harvested from system headers (glibc header guards etc.)."""
    lines = [
        line for line in token.split("\n") if not re.match(r"pub const _", line)
    ]
    return "\n".join(lines)


def c_ffi_import(token):
    """Import line for the bare `c_*` names used in the given token."""
    used = sorted(set(re.findall(r"\bc_[a-z0-9_]+\b", token)))
    return f"use core::ffi::{{{', '.join(used)}}};\n" if used else ""


# ## Static FFI generation


def generate_static_ffi(token):
    """Generate ffi_static.rs content from the post-processed bindgen output."""
    token = token.replace("::core::ffi::", "")
    header = f"""//! FFI bindings for cuEST (static linking).
//!
//! This file is generated automatically by `scripts/generate_ffi.py` from the
//! cuEST public C headers; do not edit it manually.

#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

{c_ffi_import(token)}
"""
    return header + token


# ## Dynamic loading generation functions


def dyload_parse_file(token):
    """Parse the FFI file and extract the extern block."""
    parser = Parser(Language(tree_sitter_rust.language()))
    token_transformed = token.replace('unsafe extern "C"', 'extern "C"')
    parsed = parser.parse(bytes(token_transformed, "utf8"))
    parsed_ffi = []
    for node in parsed.root_node.children:
        if node.type == "foreign_mod_item":
            parsed_ffi.append(node)
    return parsed, parsed_ffi


def dyload_remove_extern(parsed, node_extern):
    """Remove the extern block from the parsed file."""
    return parsed.root_node.text.decode("utf8").replace(
        node_extern.text.decode("utf8"), ""
    )


def dyload_get_ffi_fn(node):
    """Get (attributes, function signature) pairs from an extern block."""
    assert node.type == "foreign_mod_item"
    fns = []
    pending_attrs = []
    for child in node.children[-1].children:
        if child.type == "attribute_item":
            pending_attrs.append(child.text.decode("utf8"))
        elif child.type == "function_signature_item":
            fns.append((pending_attrs, child))
            pending_attrs = []
    return fns


def dyload_fn_split(node):
    """Split a function signature into its components."""
    assert node.type == "function_signature_item"
    keys = ["visibility_modifier", "identifier", "parameters", "return_type"]
    result = {key: None for key in keys}
    for (idx, child) in enumerate(node.children):
        if child.type == "->":
            result["return_type"] = node.children[idx + 1]
        elif child.type in keys:
            result[child.type] = child
    assert result["identifier"] is not None
    assert result["parameters"] is not None
    return result


def normalize_ffi_types(text):
    """Shorten fully qualified `core::ffi` type paths to bare names."""
    return re.sub(r"::core::ffi::(\w+)", r"\1", text)


def dyload_main(token):
    """
    Generate dynamic loading files from the post-processed bindgen output.

    Returns a dict with keys:
    - ffi_base: base types and constants, extern block removed
    - dyload_struct: struct with Option<extern fn> fields
    - dyload_initializer: DyLoadLib::new implementation
    - dyload_compatible: wrapper functions calling through dyload_lib()
    """
    parsed, parsed_ffi = dyload_parse_file(token)

    token_ffi_base = token

    nodes_fn = []
    for node_extern in parsed_ffi:
        nodes_fn.extend(dyload_get_ffi_fn(node_extern))

    token_dyload_struct = ""
    token_dyload_initializer = ""
    token_dyload_compatible = ""

    for (attrs, node_fn) in nodes_fn:
        dict_fn = dyload_fn_split(node_fn)

        visibility = (
            dict_fn["visibility_modifier"].text.decode("utf8")
            if dict_fn["visibility_modifier"]
            else "pub"
        )
        identifier = dict_fn["identifier"].text.decode("utf8")

        return_type = ""
        if dict_fn["return_type"] is not None:
            return_type = " -> " + normalize_ffi_types(
                dict_fn["return_type"].text.decode("utf8")
            )

        nodes_para = [n for n in dict_fn["parameters"].children if n.type == "parameter"]
        parameters = (
            "(" + ", ".join([normalize_ffi_types(n.text.decode("utf8")) for n in nodes_para]) + ")"
        )
        parameters_called = ", ".join(
            [n.children[0].text.decode("utf8") for n in nodes_para]
        )

        token_dyload_struct += (
            f'{visibility} {identifier}: Option<unsafe extern "C" fn{parameters}{return_type}>,\n'
        )

        token_dyload_initializer += (
            f'{identifier}: get_symbol(&libs, b"{identifier}\\0").map(|sym| *sym),\n'
        )

        attrs_text = "".join([f"{attr}\n" for attr in attrs])
        expect_msg = f"cuEST function `{identifier}` not found in the loaded library"
        token_dyload_compatible += (
            f"{attrs_text}{visibility} unsafe fn {identifier}{parameters}{return_type} {{\n"
            f'    dyload_lib().{identifier}.expect("{expect_msg}")({parameters_called})\n'
            f"}}\n\n"
        )

    for node_extern in parsed_ffi:
        token_ffi_base = dyload_remove_extern(parsed, node_extern)

    output_ffi_base = f"""//! Base types and constants for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.

#![allow(non_camel_case_types)]

{token_ffi_base}"""

    output_dyload_struct = f"""//! Library struct definition for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.
//!
//! All functions are resolved when the shared library is loaded; a runtime
//! panic occurs if a function is not found.

use super::*;
{c_ffi_import(token_dyload_struct)}
pub struct DyLoadLib {{
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub __error: Option<String>,
{token_dyload_struct}}}"""

    output_dyload_initializer = f"""//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.

use super::*;
use libloading::{{Library, Symbol}};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {{
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}}

impl DyLoadLib {{
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {{
        let mut result = DyLoadLib {{
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
{token_dyload_initializer}        }};
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }}
}}"""

    output_dyload_compatible = f"""//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.
//!
//! All functions are resolved when the shared library is loaded; a runtime
//! panic occurs if a function is not found.

use super::*;
{c_ffi_import(token_dyload_compatible)}
{token_dyload_compatible}"""

    return {
        "ffi_base": output_ffi_base,
        "dyload_struct": output_dyload_struct,
        "dyload_initializer": output_dyload_initializer,
        "dyload_compatible": output_dyload_compatible,
    }


# ## Main execution


def main():
    os.makedirs(f"{path_out}/src/ffi_dynamic", exist_ok=True)

    # Prepare temporary directory
    shutil.rmtree(path_temp, ignore_errors=True)
    os.makedirs(path_temp, exist_ok=True)
    os.chdir(path_temp)

    # Run bindgen on the umbrella header. Enums use the `newtype_global` style:
    # `#[repr(transparent)]` newtypes with global constants (same ABI as the C
    # enum; nominally typed, and any integer value — including future enum
    # members — stays valid, unlike Rust enums).
    subprocess.run(
        [
            "bindgen",
            path_header,
            "-o", "ffi.rs",
            "--no-layout-tests",
            "--use-core",
            "--merge-extern-blocks",
            "--allowlist-function", "cuest.*",
            "--allowlist-type", "cuest.*",
            "--allowlist-var", "CUEST.*",
            "--default-enum-style", "newtype_global",
        ],
        check=True,
    )

    with open("ffi.rs", "r") as f:
        bindgen_output = f.read()

    token = strip_enum_prefixes(bindgen_output)
    token = remove_system_constants(token)

    # Generate static FFI (ffi_static.rs)
    static_ffi = generate_static_ffi(token)
    with open(f"{path_out}/src/ffi_static.rs", "w") as f:
        f.write(static_ffi)

    # Generate dynamic loading files (ffi_dynamic/)
    dyload_files = dyload_main(token)
    for name, content in dyload_files.items():
        with open(f"{path_out}/src/ffi_dynamic/{name}.rs", "w") as f:
            f.write(content)

    # Format the generated files
    os.chdir(path_out)
    subprocess.run(["cargo", "fmt"], check=True)

    print("Generated:")
    print(f"  - {path_out}/src/ffi_static.rs")
    print(f"  - {path_out}/src/ffi_dynamic/")


if __name__ == "__main__":
    main()
