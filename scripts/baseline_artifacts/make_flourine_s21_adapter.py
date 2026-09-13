#!/usr/bin/env python3
"""Package the frozen S21 pair in FLOURINE's documented per-function format.

The computation bodies are sliced verbatim from the frozen evaluation pair.  The
only added text is local type/macro scaffolding required to compile those bodies
outside their original zlib translation unit.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
C_SOURCE = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source/zlib/crc32.c"
RUST_SOURCE = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/S21/input"


def between(text: str, start: str, end: str) -> str:
    left = text.index(start)
    right = text.index(end, left)
    return text[left:right].rstrip() + "\n"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    c_text = C_SOURCE.read_text()
    rust_text = RUST_SOURCE.read_text()

    c_body = between(
        c_text,
        "local unsigned long gf2_matrix_times(mat, vec)",
        "uLong ZEXPORT crc32_combine64",
    )
    rust_body = between(
        rust_text,
        'unsafe extern "C" fn gf2_matrix_times',
        "pub fn crc32_combine64",
    )

    c_input = {
        "Includes": ["#include <stdint.h>\n"],
        "Defines": ["#define GF2_DIM 32\n", "#define local static\n", "#define ZEXPORT\n"],
        "TypeDefs": [
            "typedef unsigned long uLong;\n",
            "typedef long z_off_t;\n",
            "typedef long z_off64_t;\n",
        ],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [
            "uLong crc32_combine(uLong crc1, uLong crc2, z_off_t len2);",
        ],
        "Function Implementations": [c_body],
        "Enums": [],
    }
    rust_input = (
        "type c_ulong = std::os::raw::c_ulong;\n"
        "type uLong = std::os::raw::c_ulong;\n"
        "type off_t = std::os::raw::c_long;\n\n"
        + rust_body
    )

    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "crc32_combine.json").write_text(json.dumps(c_input, indent=2) + "\n")
    (OUT / "crc32_combine.rs").write_text(rust_input)

    adapter = {
        "schema_version": 1,
        "defect": "S21",
        "baseline": "flourine",
        "target": "crc32_combine",
        "adapter_kind": "documented_per_function_json_and_rust_source",
        "semantic_rewrite": False,
        "sources": {
            "c": str(C_SOURCE.relative_to(ROOT)),
            "rust": str(RUST_SOURCE.relative_to(ROOT)),
        },
        "added_scaffolding": [
            "C typedefs and empty zlib linkage macros needed outside crc32.c",
            "Rust aliases for the translated c_ulong, uLong, and off_t names",
            "the target prototype required by FLOURINE's C JSON schema",
        ],
        "notes": [
            "All helper and target computation bodies are copied byte-for-byte from the frozen pair.",
            "crc32_combine64 is omitted so FLOURINE's call-graph entry selection has one top-level target.",
        ],
    }
    metadata = OUT.parent / "adapter.json"
    metadata.write_text(json.dumps(adapter, indent=2) + "\n")
    adapter["input_sha256"] = {
        "crc32_combine.json": sha256(OUT / "crc32_combine.json"),
        "crc32_combine.rs": sha256(OUT / "crc32_combine.rs"),
    }
    metadata.write_text(json.dumps(adapter, indent=2) + "\n")


if __name__ == "__main__":
    main()
