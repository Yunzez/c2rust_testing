#!/usr/bin/env python3
"""Reuse audited exact-source FLOURINE packages as RustAssure inputs.

Both released artifacts accept one C/Rust function pair at a time.  This
converter changes only the outer documented container: FLOURINE's JSON fields
become one C `.i` file, while the Rust surface and any `include!` dependency
files are copied byte-for-byte.  It does not rewrite a target, dependency,
signature, or return value.
"""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
FLOURINE = ROOT / "results/baseline_artifacts/adapters/flourine"
OUT_ROOT = ROOT / "results/baseline_artifacts/adapters/rustassure"
RUSTASSURE_IMAGE = "c2r-baseline-rustassure:39618406"

# Defects whose existing package has one exact target and a fixed, explicit
# logical argument list.  S14 is intentionally absent: its scored FLOURINE
# input exposed an ABI failure and must not be replaced by the later workaround.
CASES = {
    "C2": ("C2", 1),
    "C4": ("C4", 1),
    "C7": ("C7", 1),
    "C8": ("C8", 1),
    "C9": ("C9", 0),
    "C10": ("C10", 1),
    "C11": ("C11", 1),
    "C15": ("S20", 1),
    "S1": ("S1", 1),
    "S2": ("S2", 1),
    "S3": ("C8", 1),
    "S4": ("S4", 1),
    "S5": ("S5", 1),
    "S7": ("cjson_parse_string_ascii", 1),
    "S9": ("cjson_parse_string", 1),
    "S10": ("C7", 1),
    "S11": ("S11", 1),
    "S12": ("S12", 1),
    "S15": ("S15", 1),
    "S18": ("S18", 1),
    "S19": ("S19", 1),
    "S20": ("S20", 1),
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def c_source(document: dict) -> str:
    sections: list[str] = []
    for key in (
        "Includes",
        "Defines",
        "TypeDefs",
        "Globals",
        "Structs",
        "Function Declarations",
        "Enums",
        "Function Implementations",
    ):
        values = document.get(key, [])
        if values:
            sections.append("\n".join(values))
    return "\n\n".join(sections).rstrip() + "\n"


def preprocess_c(source: str) -> str:
    """Preprocess with the same frozen Clang toolchain that consumes `.i`."""
    completed = subprocess.run(
        [
            "docker",
            "run",
            "--rm",
            "--cpus",
            "1",
            "-i",
            RUSTASSURE_IMAGE,
            "clang",
            "-E",
            "-P",
            "-x",
            "c",
            "-",
        ],
        input=source,
        text=True,
        capture_output=True,
        check=True,
    )
    return completed.stdout


def emit(defect: str, source_slug: str, arity: int) -> None:
    source = FLOURINE / source_slug
    metadata = json.loads((source / "adapter.json").read_text())
    target = metadata["target"]
    json_inputs = list((source / "input").glob("*.json"))
    rust_inputs = list((source / "input").glob("*.rs"))
    if len(json_inputs) != 1 or len(rust_inputs) != 1:
        raise RuntimeError(f"{defect}: expected one JSON and one Rust surface")

    out = OUT_ROOT / defect
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)
    for old in input_dir.iterdir():
        if old.is_file():
            old.unlink()

    c_path = input_dir / f"{target}.i"
    rust_path = input_dir / f"{target}.rs"
    c_path.write_text(
        preprocess_c(c_source(json.loads(json_inputs[0].read_text())))
    )
    rust_path.write_bytes(rust_inputs[0].read_bytes())
    copied = [c_path, rust_path]
    for dependency in sorted((source / "input").glob("*.inc")):
        destination = input_dir / dependency.name
        shutil.copyfile(dependency, destination)
        copied.append(destination)

    map_path = out / "argument_order_map.json"
    map_path.write_text(
        json.dumps({target: {str(index): str(index) for index in range(arity)}}, indent=2)
        + "\n"
    )
    copied.append(map_path)
    record = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "rustassure",
        "target": target,
        "adapter_kind": "documented_individual_function_input_reusing_audited_exact_source_package",
        "semantic_rewrite": False,
        "sources": metadata["sources"],
        "notes": [
            "The C computation and wrapper are the documented preprocessed rendition of the audited exact-source FLOURINE package; the Rust computation and wrapper are copied byte-for-byte.",
            "FLOURINE's JSON container is flattened and preprocessed with gcc -E -P -x c into RustAssure's documented individual-function .i input; the argument map is positional identity.",
        ],
        "preprocessing": {
            "command": "docker run --rm --cpus 1 -i c2r-baseline-rustassure:39618406 clang -E -P -x c -",
            "purpose": "RustAssure documents preprocessed .i files as its C input format; using its frozen Clang 14 avoids host-header drift",
        },
        "input_sha256": {path.name: sha256(path) for path in copied},
    }
    (out / "adapter.json").write_text(json.dumps(record, indent=2) + "\n")


def main() -> None:
    for defect, (source_slug, arity) in CASES.items():
        emit(defect, source_slug, arity)


if __name__ == "__main__":
    main()
