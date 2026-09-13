#!/usr/bin/env python3
"""Package the lil constructor failure for released FLOURINE."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/lil_c2saferrust"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/C9"
TARGET = "lil_new_probe"


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

int lil_new_probe(void)
{
    lil_t value = lil_new();
    int result = value != 0;
    if (value != 0) lil_free(value);
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
        "Function Declarations": [f"int {TARGET}(void);"],
        "Function Implementations": [c_unit + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    translated = (PAIR / "translated/lil_c2saferrust.rs").read_text()
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
pub unsafe extern "C" fn {TARGET}() -> i32 {{
    let value = lil_new();
    let result = (!value.is_null()) as i32;
    if !value.is_null() {{
        lil_free(value);
    }}
    result
}}
'''
    )
    metadata = {
        "schema_version": 1,
        "defect": "C9",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_lifecycle_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": "benchmark/pairs/rq4/lil_c2saferrust/source/lil.c",
            "rust": "benchmark/pairs/rq4/lil_c2saferrust/translated/lil_c2saferrust.rs",
        },
        "wrapper_contract": (
            "Invoke the exact zero-argument constructor, expose nullness, and call "
            "the exact destructor only if construction returns."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
