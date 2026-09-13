#!/usr/bin/env python3
"""Package Tulip's lookup-initialization defect for released FLOURINE."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/tulip_laertes"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/C11"
TARGET = "ti_find_indicator_valid"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_includes(source: str) -> str:
    return re.sub(
        r'^\s*#\s*include\s*[<"][^>"]+[>"]\s*$',
        "",
        source,
        flags=re.MULTILINE,
    )


def c_dependency_unit() -> str:
    source = PAIR / "source"
    files = [source / "indicators.h", source / "utils/buffer.h", source / "utils/minmax.h"]
    files += sorted((source / "indicators").glob("*.h"))
    files += sorted((source / "indicators").glob("*.c"))
    files += [source / "utils/buffer.c", source / "indicators_index.c"]
    return "\n\n".join(without_includes(path.read_text()) for path in files)


def rust_dependency_unit() -> str:
    source = (PAIR / "translated/tulip_laertes.rs").read_text()
    return re.sub(r"(?ms)^#!\[[^\]]*\]\s*", "", source).rstrip() + "\n"


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    raw_path = input_dir / f"{TARGET}_raw.inc"

    c_wrapper = r'''

int ti_find_indicator_valid(int8_t name[16])
{
    name[15] = 0;
    return ti_find_indicator((const char *)name) != 0;
}
'''
    document = {
        "Includes": [
            "#include <assert.h>",
            "#include <math.h>",
            "#include <stdint.h>",
            "#include <stdlib.h>",
            "#include <string.h>",
        ],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [f"int {TARGET}(int8_t name[16]);"],
        "Function Implementations": [c_dependency_unit() + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")
    raw_path.write_text(rust_dependency_unit())
    rust_path.write_text(
        f'''#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/{raw_path.name}"];

#[no_mangle]
pub fn {TARGET}(name: &mut [i8; 16]) -> i32 {{
    name[15] = 0;
    unsafe {{ (!ti_find_indicator(name.as_ptr()).is_null()) as i32 }}
}}
'''
    )
    metadata = {
        "schema_version": 1,
        "defect": "C11",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_nul_terminated_name_wrapper",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": "benchmark/pairs/rq4/tulip_laertes/source/indicators_index.c",
            "rust": "benchmark/pairs/rq4/tulip_laertes/translated/tulip_laertes.rs",
        },
        "wrapper_contract": (
            "Generate 16 name bytes, set only the final byte to NUL, call the exact "
            "lookup, and expose nullness while preserving termination."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
