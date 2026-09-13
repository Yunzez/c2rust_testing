#!/usr/bin/env python3
"""Package the exact Tulip C6 CLI pair for released FLOURINE.

The corresponding RustAssure package already freezes the historical C sample,
the C2SaferRust WIP translation, their dependency closures, and a generated
valid argv surface.  FLOURINE consumes the same exact source package through
its documented per-function JSON/Rust input format; no target logic or witness
is added here.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
SOURCE = ROOT / "results/baseline_artifacts/adapters/rustassure/C6"
OUT = ROOT / "results/baseline_artifacts/adapters/flourine/C6"
TARGET = "sample_main_packet"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    input_dir = OUT / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    json_path = input_dir / f"{TARGET}.json"
    rust_path = input_dir / f"{TARGET}.rs"
    c_exact_path = input_dir / f"{TARGET}_preprocessed.inc"

    source_c = SOURCE / "input" / f"{TARGET}.i"
    source_rust = SOURCE / "input" / f"{TARGET}.rs"
    c_exact_path.write_bytes(source_c.read_bytes())
    rust_path.write_bytes(source_rust.read_bytes())

    document = {
        "Includes": [],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [
            "int sample_main_packet(unsigned char argc_selector, "
            "signed char arg0[32], signed char arg1[32], "
            "signed char arg2[32], signed char arg3[32]);"
        ],
        "Function Implementations": [source_c.read_text()],
        "Enums": [],
    }
    json_path.write_text(json.dumps(document, indent=2) + "\n")

    source_metadata = json.loads((SOURCE / "adapter.json").read_text())
    metadata = {
        "schema_version": 1,
        "defect": "C6",
        "baseline": "flourine",
        "target": TARGET,
        "adapter_kind": "documented_per_function_json_with_argv_contract_surface",
        "semantic_rewrite": False,
        "known_witness_encoded": False,
        "sources": source_metadata["sources"],
        "identity_evidence": source_metadata["identity_evidence"],
        "wrapper_contract": (
            "A generated selector ranges over argc 1..4; four independently "
            "generated 32-byte strings are only final-byte NUL terminated and "
            "assembled into a valid argv before the exact historical C sample "
            "and exact C2SaferRust main_0 are called."
        ),
        "packaging_provenance": {
            "source_adapter": str((SOURCE / "adapter.json").relative_to(ROOT)),
            "c_input": (
                "The exact C package is already preprocessed by the frozen "
                "RustAssure Clang 14; FLOURINE receives those bytes verbatim "
                "inside its documented Function Implementations field."
            ),
            "rust_input": (
                "The exact flattened WIP translation and its original nightly "
                "attributes are copied byte-for-byte; a released instrumenter "
                "failure is scored and must not be repaired."
            ),
        },
        "input_sha256": {
            path.name: sha256(path) for path in (json_path, rust_path, c_exact_path)
        },
    }
    (OUT / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
