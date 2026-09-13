#!/usr/bin/env python3
"""Package PtrTrans cJSON parse_string for the released FLOURINE artifact.

The wrapper only realizes a valid parse_buffer and a quoted fixed-capacity byte
string.  It returns the original parse_string result and deliberately does not
canonicalize cJSON fields: doing so would add comparison logic that FLOURINE
does not provide.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/cjson_ptrtrans"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/cjson_parse_string"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def c_translation_unit() -> str:
    header = (PAIR / "source/cJSON.h").read_text()
    source = (PAIR / "source/cJSON.c").read_text()
    # FLOURINE's documented JSON format has a separate Includes field.  Keep
    # system declarations there, before its allocator-interposition macros,
    # rather than embedding includes in Function Implementations.
    header = re.sub(r"^\s*#\s*include\s*<[^>]+>\s*$", "", header, flags=re.MULTILINE)
    source = re.sub(r"^\s*#\s*include\s*<[^>]+>\s*$", "", source, flags=re.MULTILINE)
    source = source.replace('#include "cJSON.h"', header, 1)
    wrapper = r'''

int parse_string_valid(int8_t input[32])
{
    cJSON item;
    parse_buffer buffer;
    cJSON_bool result;

    memset(&item, 0, sizeof(item));
    memset(&buffer, 0, sizeof(buffer));
    input[0] = '"';
    input[30] = '"';
    input[31] = '\0';
    buffer.content = (const unsigned char *)input;
    buffer.length = 31;
    buffer.hooks = global_hooks;
    result = parse_string(&item, &buffer);
    if (item.valuestring != NULL)
    {
        global_hooks.deallocate(item.valuestring);
    }
    return result;
}
'''
    return source + wrapper


def rust_wrapper() -> str:
    return r'''

extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(pointer: *mut core::ffi::c_void);
}

#[no_mangle]
pub fn parse_string_valid(input: &mut [i8; 32]) -> i32 {
    input[0] = b'"' as i8;
    input[30] = b'"' as i8;
    input[31] = 0;
    let mut content = [0u8; 32];
    for (dst, src) in content.iter_mut().zip(input.iter()) {
        *dst = *src as u8;
    }
    let allocate = |size: usize| unsafe { malloc(size) };
    let deallocate = |pointer: *mut core::ffi::c_void| unsafe { free(pointer) };
    let mut item = cJSON {
        next: None,
        prev: None,
        child: None,
        type_: 0,
        valuestring: None,
        valueint: 0,
        valuedouble: 0.0,
        string: None,
    };
    let mut buffer = ParseBuffer {
        content: Some(&content[..31]),
        length: 31,
        offset: 0,
        depth: 0,
        hooks: InternalHooks {
            allocate: Some(&allocate),
            deallocate: Some(&deallocate),
            reallocate: None,
        },
    };
    parse_string(Some(&mut item), Some(&mut buffer))
}
'''


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    c_path = input_dir / "parse_string_valid.json"
    rust_path = input_dir / "parse_string_valid.rs"
    raw_path = input_dir / "parse_string_valid_raw.inc"

    c_path.write_text(
        json.dumps(
            {
                "Includes": [
                    "#include <stdint.h>",
                    "#include <stddef.h>",
                    "#include <string.h>",
                    "#include <stdio.h>",
                    "#include <math.h>",
                    "#include <stdlib.h>",
                    "#include <limits.h>",
                    "#include <ctype.h>",
                    "#include <float.h>",
                ],
                "Defines": [],
                "TypeDefs": [],
                "Globals": [],
                "Structs": [],
                "Function Declarations": [
                    "int parse_string_valid(int8_t input[32]);"
                ],
                "Function Implementations": [c_translation_unit()],
                "Enums": [],
            },
            indent=2,
        )
        + "\n"
    )
    raw_unit = (PAIR / "translated/cjson_ptrtrans.rs").read_text().rstrip() + "\n"
    raw_path.write_text(raw_unit)
    # FLOURINE intentionally sees only the submitted wrapper in the surface
    # AST. The exact translated dependency closure is compiled through
    # include!, matching the artifact's documented per-function packaging.
    rust_path.write_text(
        'include!["/input/parse_string_valid_raw.inc"];\n' + rust_wrapper() + "\n"
    )

    metadata = {
        "schema_version": 1,
        "defects": ["S7", "S9"],
        "baseline": "flourine",
        "target": "parse_string_valid",
        "adapter_kind": "documented_per_function_json_with_input_realization_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": str((PAIR / "source/cJSON.c").relative_to(ROOT)),
            "c_header": str((PAIR / "source/cJSON.h").relative_to(ROOT)),
            "rust": str((PAIR / "translated/cjson_ptrtrans.rs").relative_to(ROOT)),
        },
        "wrapper_contract": (
            "Place artifact-generated bytes inside a fixed 31-byte quoted JSON "
            "string, construct the original item and parse_buffer values, call "
            "the exact parse_string target, and return only its original result."
        ),
        "observation_limit": (
            "The wrapper intentionally does not expose or canonicalize cJSON "
            "fields; it cannot score S8, whose only symptom is valuestring state."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (c_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
