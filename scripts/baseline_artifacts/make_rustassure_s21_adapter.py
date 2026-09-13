#!/usr/bin/env python3
"""Package the frozen S21 function pair for RustAssure."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
C_SOURCE = ROOT / "benchmark/pairs/rq4/optipng_c2saferrust/source/zlib/crc32.c"
RUST_INPUT = ROOT / "results/baseline_artifacts/adapters/flourine/S21/input/crc32_combine.rs"
OUT = ROOT / "results/baseline_artifacts/adapters/rustassure/S21"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    c_text = C_SOURCE.read_text()
    left = c_text.index("local unsigned long gf2_matrix_times(mat, vec)")
    right = c_text.index("uLong ZEXPORT crc32_combine64", left)
    c_body = c_text[left:right].rstrip() + "\n"
    c_input = (
        "#define GF2_DIM 32\n"
        "#define local static\n"
        "#define ZEXPORT\n"
        "typedef unsigned long uLong;\n"
        "typedef long z_off_t;\n"
        "typedef long z_off64_t;\n\n"
        + c_body
    )

    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    (input_dir / "crc32_combine.i").write_text(c_input)
    (input_dir / "crc32_combine.rs").write_bytes(RUST_INPUT.read_bytes())
    (OUT / "argument_order_map.json").write_text(
        json.dumps({"crc32_combine": {"0": "0", "1": "1", "2": "2"}}, indent=2)
        + "\n"
    )
    metadata = {
        "schema_version": 1,
        "defect": "S21",
        "baseline": "rustassure",
        "target": "crc32_combine",
        "adapter_kind": "documented_individual_function_input_manual_argument_order",
        "semantic_rewrite": False,
        "sources": {
            "c": str(C_SOURCE.relative_to(ROOT)),
            "rust": "benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:64882-64962",
        },
        "notes": [
            "All helper and target computation bodies are copied byte-for-byte from the frozen pair.",
            "Only local typedef/macro scaffolding and an identity positional argument map are added.",
            "crc32_combine64 is omitted to give the released analysis one explicit target boundary.",
        ],
        "input_sha256": {
            "crc32_combine.i": sha256(input_dir / "crc32_combine.i"),
            "crc32_combine.rs": sha256(input_dir / "crc32_combine.rs"),
            "argument_order_map.json": sha256(OUT / "argument_order_map.json"),
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
