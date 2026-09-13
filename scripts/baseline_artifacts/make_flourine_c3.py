#!/usr/bin/env python3
"""Package C3 for FLOURINE's documented per-function input format.

The package retains the exact C and Rust ``do_system`` implementations.  Its
only adapter work is to realize the original argc/argv contract with two
generated, independently backed, NUL-terminated strings.  In particular, it
does not replace the external command execution that the released baseline
must analyze or execute on its own.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/C3"
TARGET = "do_system_two"
C_SOURCE = ROOT / "tools/frameworks/crown/c-code/lil/main.c"
RUSTASSURE_INPUT = ROOT / "results/baseline_artifacts/adapters/rustassure/C3/input"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def balanced_function(text: str, marker: str) -> str:
    start = text.index(marker)
    opening = text.index("{", start)
    depth = 0
    for index in range(opening, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise RuntimeError(f"unterminated function after {marker!r}")


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    raw_path = input_dir / f"{TARGET}_raw.inc"

    exact_c = balanced_function(C_SOURCE.read_text(), "static char* do_system(")
    c_wrapper = r'''

char *do_system_two(int8_t first[16], int8_t second[16])
{
    char *argv[2];
    first[15] = 0;
    second[15] = 0;
    argv[0] = (char *)first;
    argv[1] = (char *)second;
    return do_system(2, argv);
}
'''
    document = {
        "Includes": [
            "#include <stddef.h>",
            "#include <stdint.h>",
            "#include <stdio.h>",
            "#include <stdlib.h>",
            "#include <string.h>",
            "#include <sys/types.h>",
        ],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [
            "char *do_system_two(int8_t first[16], int8_t second[16]);"
        ],
        "Function Implementations": [exact_c + c_wrapper],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    # Reuse the already audited exact Rust body, not an independently rewritten
    # copy.  Only the include path changes to FLOURINE's documented input mount.
    source_raw = RUSTASSURE_INPUT / f"{TARGET}_raw.inc"
    source_rs = RUSTASSURE_INPUT / f"{TARGET}.rs"
    raw_path.write_bytes(source_raw.read_bytes())
    rust_path.write_bytes(source_rs.read_bytes())

    metadata = {
        "schema_version": 1,
        "defect": "C3",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_argv_contract_surface",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": {
            "c": str(C_SOURCE.relative_to(ROOT)),
            "rust": "benchmark/pairs/rq4/lil_c2saferrust/translated/lil_c2saferrust.rs",
        },
        "wrapper_contract": (
            "Generate two independent 16-byte strings, set only each final byte "
            "to NUL, assemble the original argc/argv contract, and call the exact "
            "do_system implementation on both sides. External command execution "
            "is neither replaced nor mocked."
        ),
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, raw_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
