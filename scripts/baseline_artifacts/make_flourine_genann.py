#!/usr/bin/env python3
"""Package genann's initialized cached activation for released FLOURINE."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
PAIR = ROOT / "benchmark/pairs/rq4/genann_sactor"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/S5"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def c_unit() -> str:
    header = (PAIR / "source/genann.h").read_text()
    source = (PAIR / "source/genann.c").read_text()
    header = re.sub(r"^\s*#\s*include\s*[<\"][^>\"]+[>\"]\s*$", "", header, flags=re.MULTILINE)
    source = source.replace('#include "genann.h"', header, 1)
    source = re.sub(r"^\s*#\s*include\s*[<\"][^>\"]+[>\"]\s*$", "", source, flags=re.MULTILINE)
    return source + r'''

double genann_cached_initialized(double input[1])
{
    double value = isnan(input[0]) ? 0.0 : input[0];
    genann_init_sigmoid_lookup(NULL);
    return genann_act_sigmoid_cached(NULL, value);
}
'''


def rust_unit(raw_name: str) -> str:
    return f'''include!["/input/{raw_name}"];

#[no_mangle]
pub fn genann_cached_initialized(input: &mut [f64; 1]) -> f64 {{
    let value = if input[0].is_nan() {{ 0.0 }} else {{ input[0] }};
    unsafe {{
        genann_init_sigmoid_lookup(std::ptr::null());
        genann_act_sigmoid_cached(std::ptr::null(), value)
    }}
}}
'''


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / "genann_cached_initialized.json"
    rust_path = input_dir / "genann_cached_initialized.rs"
    raw_path = input_dir / "genann_cached_initialized_raw.inc"
    json_path.write_text(
        json.dumps(
            {
                "Includes": [
                    "#include <assert.h>",
                    "#include <errno.h>",
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
                "Function Declarations": [
                    "double genann_cached_initialized(double input[1]);"
                ],
                "Function Implementations": [c_unit()],
                "Enums": [],
            },
            indent=2,
        )
        + "\n"
    )
    raw_path.write_text((PAIR / "translated/genann_sactor.rs").read_text().rstrip() + "\n")
    rust_path.write_text(rust_unit(raw_path.name))
    metadata = {
        "schema_version": 1,
        "defect": "S5",
        "baseline": "flourine",
        "target": "genann_cached_initialized",
        "adapter_kind": "documented_per_function_json_with_library_initializer",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": "benchmark/pairs/rq4/genann_sactor/source/genann.c",
            "c_header": "benchmark/pairs/rq4/genann_sactor/source/genann.h",
            "rust": "benchmark/pairs/rq4/genann_sactor/translated/genann_sactor.rs",
        },
        "wrapper_contract": (
            "Call each implementation's exact lookup initializer, then its exact cached "
            "activation with one artifact-generated f64. NaN is mapped identically to 0.0 to "
            "satisfy the target's explicit assert(!isnan(a)) contract. The unused ann argument "
            "is NULL, as in the library's shipped C tests."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
