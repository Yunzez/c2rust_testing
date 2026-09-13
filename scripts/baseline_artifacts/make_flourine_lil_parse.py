#!/usr/bin/env python3
"""Package CROWN lil_parse for released FLOURINE without a trigger seed."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/lil_crown"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/C10"
TARGET = "lil_parse_generated"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_includes(source: str) -> str:
    return re.sub(
        r'^\s*#\s*include\s*[<"][^>"]+[>"]\s*$',
        "",
        source,
        flags=re.MULTILINE,
    )


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    raw_path = input_dir / f"{TARGET}_raw.inc"

    c_unit = "\n\n".join(
        without_includes((PAIR / "source" / name).read_text())
        for name in ("lil.h", "lil.c")
    )
    c_wrapper = r'''

int lil_parse_generated(int8_t script[64])
{
    script[63] = 0;
    lil_t lil = lil_new();
    if (lil == 0) return 0;
    lil_value_t value = lil_parse(lil, (const char *)script, 0, 0);
    int result = value != 0;
    if (value != 0) lil_free_value(value);
    lil_free(lil);
    return result;
}
'''
    document = {
        "Includes": [
            "#include <ctype.h>",
            "#include <inttypes.h>",
            "#include <math.h>",
            "#include <stdint.h>",
            "#include <stdio.h>",
            "#include <stdlib.h>",
            "#include <string.h>",
        ],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [f"int {TARGET}(int8_t script[64]);"],
        "Function Implementations": [c_unit + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    translated = (PAIR / "translated/lil_crown.rs").read_text()
    translated = re.sub(r"(?ms)^#!\[[^\]]*\]\s*", "", translated)
    raw_path.write_text(translated.rstrip() + "\n")
    rust_path.write_text(
        f'''#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/{raw_path.name}"];

#[no_mangle]
pub fn {TARGET}(script: &mut [i8; 64]) -> i32 {{
    script[63] = 0;
    unsafe {{
        let lil = lil_new();
        if lil.is_null() {{
            return 0;
        }}
        let value = lil_parse(lil, script.as_ptr(), 0, 0);
        let result = (!value.is_null()) as i32;
        if !value.is_null() {{
            lil_free_value(value);
        }}
        lil_free(lil);
        result
    }}
}}
'''
    )
    metadata = {
        "schema_version": 1,
        "defect": "C10",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_lifecycle_and_string_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": "benchmark/pairs/rq4/lil_crown/source/lil.c",
            "rust": "benchmark/pairs/rq4/lil_crown/translated/lil_crown.rs",
        },
        "wrapper_contract": (
            "Generate all 64 script bytes, set only the final byte to NUL, then "
            "run the exact constructor, parser, value destructor, and interpreter "
            "destructor independently on each side. No command or malformed syntax "
            "is seeded."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
