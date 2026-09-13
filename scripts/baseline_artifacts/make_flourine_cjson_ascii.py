#!/usr/bin/env python3
"""Package an ASCII-domain cJSON parse_string input for FLOURINE.

The broad printable-ASCII mapping removes the already detected non-UTF-8 S9
class without inserting a backslash, ``u``, or hex digits.  A Unicode escape
therefore remains something the released artifact must discover itself.
"""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/cjson_ptrtrans"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/cjson_parse_string_ascii"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def c_translation_unit() -> str:
    header = (PAIR / "source/cJSON.h").read_text()
    source = (PAIR / "source/cJSON.c").read_text()
    header = re.sub(r"^\s*#\s*include\s*<[^>]+>\s*$", "", header, flags=re.MULTILINE)
    source = re.sub(r"^\s*#\s*include\s*<[^>]+>\s*$", "", source, flags=re.MULTILINE)
    source = source.replace('#include "cJSON.h"', header, 1)
    wrapper = r'''

int parse_string_ascii(int8_t input[32])
{
    cJSON item;
    parse_buffer buffer;
    unsigned char content[32];
    cJSON_bool result;
    size_t index;

    memset(&item, 0, sizeof(item));
    memset(&buffer, 0, sizeof(buffer));
    content[0] = '"';
    for (index = 1; index < 30; index++)
    {
        content[index] = (unsigned char)(32 + ((unsigned char)input[index] % 95));
    }
    content[30] = '"';
    content[31] = '\0';
    buffer.content = content;
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
pub fn parse_string_ascii(input: &mut [i8; 32]) -> i32 {
    let mut content = [0u8; 32];
    content[0] = b'"';
    for index in 1..30 {
        content[index] = 32 + ((input[index] as u8) % 95);
    }
    content[30] = b'"';
    content[31] = 0;
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
    c_path = input_dir / "parse_string_ascii.json"
    rust_path = input_dir / "parse_string_ascii.rs"
    raw_path = input_dir / "parse_string_ascii_raw.inc"

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
                "Function Declarations": ["int parse_string_ascii(int8_t input[32]);"],
                "Function Implementations": [c_translation_unit()],
                "Enums": [],
            },
            indent=2,
        )
        + "\n"
    )
    raw_path.write_text((PAIR / "translated/cjson_ptrtrans.rs").read_text().rstrip() + "\n")
    rust_path.write_text(
        'include!["/input/parse_string_ascii_raw.inc"];\n' + rust_wrapper() + "\n"
    )
    metadata = {
        "schema_version": 1,
        "defects": ["S7"],
        "baseline": "flourine",
        "target": "parse_string_ascii",
        "adapter_kind": "documented_per_function_json_with_input_domain_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "domain": (
            "Each generated interior byte is mapped independently to printable ASCII. "
            "The adapter does not force a backslash, the letter u, or hexadecimal digits."
        ),
        "sources": {
            "c": str((PAIR / "source/cJSON.c").relative_to(ROOT)),
            "c_header": str((PAIR / "source/cJSON.h").relative_to(ROOT)),
            "rust": str((PAIR / "translated/cjson_ptrtrans.rs").relative_to(ROOT)),
        },
        "input_sha256": {
            path.name: sha256(path) for path in (c_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
