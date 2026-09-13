#!/usr/bin/env python3
"""Package the two qsort pilots for FLOURINE without changing bodies."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def emit(defect: str, c_path: Path, rust_path: Path, c_target: str, rust_target: str) -> None:
    out = ROOT / f"results/baseline_artifacts/adapters/flourine/{defect}"
    input_dir = out / "input"
    input_dir.mkdir(parents=True, exist_ok=True)

    c_body = c_path.read_text().rstrip() + "\n"
    wrapper = ""
    notes = ["C and Rust computation bodies are copied from the frozen defect evidence."]
    if c_target != rust_target:
        wrapper = (
            f"\nvoid {rust_target}(int arr[], int low, int high)\n"
            "{\n"
            f"    {c_target}(arr, low, high);\n"
            "}\n"
        )
        notes.append(
            f"A C name adapter {rust_target} calls {c_target} because FLOURINE has no correspondence-map input; it changes no arguments, state, or result."
        )

    c_json = {
        "Includes": [],
        "Defines": [],
        "TypeDefs": [],
        "Globals": [],
        "Structs": [],
        "Function Declarations": [
            f"void {rust_target}(int arr[], int low, int high);"
        ],
        "Function Implementations": [c_body + wrapper],
        "Enums": [],
    }
    c_out = input_dir / f"{rust_target}.json"
    c_out.write_text(json.dumps(c_json, indent=2) + "\n")

    rust_text = rust_path.read_text()
    if defect == "C1":
        rust_text = rust_text[rust_text.index("#[no_mangle]") :]
        notes.append("An unused crate-level lint attribute is omitted so FLOURINE can embed the file's AST; function bodies and signatures are unchanged.")
    rust_out = input_dir / f"{rust_target}.rs"
    rust_out.write_text(rust_text)

    # FLOURINE's released C instrumentor needs an explicit extent in an array
    # parameter in order to allocate the local array used by its wrapper.  A C
    # array bound in a function parameter does not change the C function type;
    # this is an input-shape adapter for the artifact, not a body rewrite.
    fixed_input_dir = out / "input_fixed_64"
    fixed_input_dir.mkdir(parents=True, exist_ok=True)
    fixed_c_json = dict(c_json)
    fixed_c_json["Function Declarations"] = [
        f"void {rust_target}(int arr[64], int low, int high);"
    ]
    fixed_c_out = fixed_input_dir / f"{rust_target}.json"
    fixed_c_out.write_text(json.dumps(fixed_c_json, indent=2) + "\n")
    fixed_rust_out = fixed_input_dir / f"{rust_target}.rs"
    fixed_rust_out.write_text(rust_text)

    # A second documented adapter realizes qsort's valid-call precondition:
    # the indices describe the complete supplied array.  It does not encode a
    # known counterexample; FLOURINE still generates every array element.
    valid_target = f"{rust_target}_valid"
    valid_extent = 16
    valid_input_dir = out / "input_valid_16"
    valid_input_dir.mkdir(parents=True, exist_ok=True)
    valid_call = f"{rust_target}(arr, 0, {valid_extent - 1});"
    valid_c_json = dict(c_json)
    valid_c_json["Function Declarations"] = [
        f"void {valid_target}(int arr[{valid_extent}]);"
    ]
    valid_c_json["Function Implementations"] = [
        c_body
        + wrapper
        + f"\nvoid {valid_target}(int arr[{valid_extent}])\n"
        + "{\n"
        + f"    {valid_call}\n"
        + "}\n"
    ]
    valid_c_out = valid_input_dir / f"{valid_target}.json"
    valid_c_out.write_text(json.dumps(valid_c_json, indent=2) + "\n")
    if defect == "C1":
        valid_rust_wrapper = (
            f"\n#[no_mangle]\npub fn {valid_target}(arr: &mut [i32; {valid_extent}]) {{\n"
            f"    {rust_target}(arr, 0, {valid_extent - 1});\n"
            "}\n"
        )
    else:
        valid_rust_wrapper = (
            f"\n#[no_mangle]\npub fn {valid_target}(arr: &mut [i32; {valid_extent}]) {{\n"
            f"    {rust_target}(Some(&mut arr[..]), 0, {valid_extent - 1});\n"
            "}\n"
        )
    valid_rust_out = valid_input_dir / f"{valid_target}.rs"
    valid_rust_out.write_text(rust_text + valid_rust_wrapper)

    metadata = {
        "schema_version": 1,
        "defect": defect,
        "baseline": "flourine",
        "target": rust_target,
        "adapter_kind": "documented_per_function_json_and_rust_source",
        "semantic_rewrite": False,
        "sources": {
            "c": str(c_path.relative_to(ROOT)),
            "rust": str(rust_path.relative_to(ROOT)),
        },
        "notes": notes,
        "alternative_input_adapters": {
            "input_fixed_64": {
                "purpose": "Give FLOURINE's released wrapper generator the storage extent that an unsized C array parameter omits.",
                "semantic_rewrite": False,
                "c_language_note": "In a function parameter, int arr[64] is adjusted by C to int *arr; the computation body and callable C type are unchanged.",
                "capacity": 64,
                "input_sha256": {
                    fixed_c_out.name: sha256(fixed_c_out),
                    fixed_rust_out.name: sha256(fixed_rust_out),
                },
            },
            "input_valid_16": {
                "purpose": "Realize qsort's valid-call precondition while leaving all array elements under artifact-native generation.",
                "semantic_rewrite": False,
                "known_witness_encoded": False,
                "precondition": "The array has 16 elements and qsort is called on the inclusive range [0, 15].",
                "input_sha256": {
                    valid_c_out.name: sha256(valid_c_out),
                    valid_rust_out.name: sha256(valid_rust_out),
                },
            },
        },
        "input_sha256": {
            c_out.name: sha256(c_out),
            rust_out.name: sha256(rust_out),
        },
    }
    (out / "adapter.json").write_text(json.dumps(metadata, indent=2) + "\n")


def main() -> None:
    emit(
        "C1",
        ROOT / "results/rq4_effectiveness/bugs/qsort_c2saferrust/source/qsort.c",
        ROOT / "results/baseline_artifacts/adapters/rustassure/C1/input/quickSort.rs",
        "quickSort",
        "quickSort",
    )
    emit(
        "S6",
        ROOT / "results/rq4_effectiveness/bugs/qsort_ptrtrans/original_qsort.c",
        ROOT / "results/rq4_effectiveness/bugs/qsort_ptrtrans/translated_qsort.rs",
        "quickSort",
        "quick_sort",
    )


if __name__ == "__main__":
    main()
