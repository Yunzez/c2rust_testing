#!/usr/bin/env python3
"""Package the S17 and C12 pilot pairs for FLOURINE.

The translated computation and defective state are copied from the frozen
pairs.  Small wrappers only realize valid, fixed-size C-string buffers in the
safe-reference vocabulary accepted by FLOURINE's released Rust instrumentor.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def brace_item(text: str, marker: str) -> str:
    start = text.index(marker)
    brace = text.index("{", start)
    depth = 0
    for index in range(brace, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise ValueError(f"unclosed item: {marker}")


def write_adapter(
    defect: str,
    target: str,
    c_json: dict,
    rust: str,
    metadata: dict,
    extra_files: dict[str, str] | None = None,
) -> None:
    out = ROOT / f"results/baseline_artifacts/adapters/flourine/{defect}"
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    c_path = input_dir / f"{target}.json"
    rust_path = input_dir / f"{target}.rs"
    c_path.write_text(json.dumps(c_json, indent=2) + "\n")
    rust_path.write_text(rust.rstrip() + "\n")
    extra_paths = []
    for name, contents in (extra_files or {}).items():
        path = input_dir / name
        path.write_text(contents.rstrip() + "\n")
        extra_paths.append(path)
    metadata.update(
        {
            "schema_version": 1,
            "defect": defect,
            "baseline": "flourine",
            "target": target,
            "adapter_kind": "documented_per_function_json_with_valid_buffer_wrapper",
            "semantic_rewrite": False,
            "known_witness_encoded": False,
            "input_sha256": {
                path.name: sha256(path) for path in [c_path, rust_path, *extra_paths]
            },
        }
    )
    (out / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


def make_s17() -> None:
    c_source = ROOT / "benchmark/pairs/rq4/optipng_laertes/source/optipng/ioutil.c"
    rust_source = ROOT / "benchmark/pairs/rq4/optipng_laertes/translated/optipng_laertes.rs"
    c_text = c_source.read_text()
    rust_text = rust_source.read_text()
    c_fn = brace_item(c_text, "char *\nopng_path_make_backup")
    rust_fn = brace_item(
        rust_text, '#[no_mangle]\npub unsafe extern "C" fn opng_path_make_backup'
    )
    target = "opng_path_make_backup_valid"
    c_wrapper = """
void opng_path_make_backup_valid(int8_t buffer[32], int8_t path[16])
{
    buffer[0] = '\\0';
    path[15] = '\\0';
    (void)opng_path_make_backup((char *)buffer, 32, (const char *)path);
}
""".strip()
    rust_wrapper = r'''
#[no_mangle]
pub fn opng_path_make_backup_valid(buffer: &mut [i8; 32], path: &mut [i8; 16]) {
    buffer[0] = 0;
    path[15] = 0;
    unsafe {
        opng_path_make_backup(buffer.as_mut_ptr(), 32, path.as_ptr());
    }
}
'''.strip()
    c_json = {
        "Includes": ["#include <stddef.h>", "#include <stdint.h>", "#include <string.h>"],
        "Defines": ['#define OPNG_PATH_EXTSEP_STR "."'],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [
            f"void {target}(int8_t buffer[32], int8_t path[16]);"
        ],
        "Function Implementations": [c_fn + "\n\n" + c_wrapper],
        "Enums": [],
    }
    rust = """extern "C" {
    fn strlen(value: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strcpy(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}

""" + 'include!["/input/opng_path_make_backup_raw.inc"];\n\n' + rust_wrapper
    write_adapter(
        "S17",
        target,
        c_json,
        rust,
        {
            "sources": {"c": str(c_source.relative_to(ROOT)), "rust": str(rust_source.relative_to(ROOT))},
            "wrapper_contract": "Generate both byte buffers as int8_t (FLOURINE has no char-array serializer); force NUL termination; cast to the platform-equivalent char pointers; invoke the frozen target with capacity 32 (the largest array extent supported by the released serde dependency); compare the mutable output buffer.",
            "observation": "FLOURINE compares mutable reference arguments after both calls.",
        },
        {"opng_path_make_backup_raw.inc": rust_fn},
    )


def make_c12() -> None:
    c_source = ROOT / "benchmark/pairs/rq4/urlparser_laertes/source/url.h"
    rust_source = ROOT / "benchmark/pairs/rq4/urlparser_laertes/translated/urlparser_laertes.rs"
    c_text = c_source.read_text()
    rust_text = rust_source.read_text()
    global_start = c_text.index("char *URL_SCHEMES[] = {")
    global_end = c_text.index("\n};", global_start) + len("\n};")
    c_global = c_text[global_start:global_end]
    c_fn = brace_item(c_text, "bool\nurl_is_protocol (char *str) {")
    rust_global_start = rust_text.index("#[no_mangle]\npub static mut URL_SCHEMES")
    rust_global_end = rust_text.index("; unsafe fn laertes_init_URL_SCHEMES", rust_global_start) + 1
    rust_global = rust_text[rust_global_start:rust_global_end]
    rust_fn = brace_item(
        rust_text, '#[no_mangle]\npub unsafe extern "C" fn url_is_protocol'
    )
    target = "url_is_protocol_valid"
    c_wrapper = """
bool url_is_protocol_valid(int8_t str[16])
{
    str[15] = '\\0';
    return url_is_protocol((char *)str);
}
""".strip()
    rust_wrapper = r'''
#[no_mangle]
pub fn url_is_protocol_valid(value: &mut [i8; 16]) -> bool {
    value[15] = 0;
    unsafe { url_is_protocol(value.as_mut_ptr()) }
}
'''.strip()
    c_json = {
        "Includes": ["#include <stdbool.h>", "#include <stdint.h>", "#include <string.h>"],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [c_global],
        "Structs": [],
        "Function Declarations": [f"bool {target}(int8_t str[16]);"],
        "Function Implementations": [c_fn + "\n\n" + c_wrapper],
        "Enums": [],
    }
    rust = """extern "C" {
    fn strcmp(left: *const std::os::raw::c_char, right: *const std::os::raw::c_char) -> std::os::raw::c_int;
}

""" + rust_global + '\n\ninclude!["/input/url_is_protocol_raw.inc"];\n\n' + rust_wrapper
    write_adapter(
        "C12",
        target,
        c_json,
        rust,
        {
            "sources": {"c": str(c_source.relative_to(ROOT)), "rust": str(rust_source.relative_to(ROOT))},
            "wrapper_contract": "Generate a 16-byte string as int8_t (FLOURINE has no char-array serializer), force NUL termination, cast to the platform-equivalent char pointer, then invoke the frozen target.",
            "defective_state": "The frozen Rust URL_SCHEMES definition remains the all-NULL table used by url_is_protocol.",
        },
        {"url_is_protocol_raw.inc": rust_fn},
    )


def main() -> None:
    make_s17()
    make_c12()


if __name__ == "__main__":
    main()
