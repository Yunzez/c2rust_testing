#!/usr/bin/env python3
"""Record completed released-RustAssure pilot analyses without guessing timeouts."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
EXTERNAL = Path("/home/yunzez/c2rust_baselines/runs/rustassure")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/rustassure"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def attempt_dir(defect: str) -> Path | None:
    pilot = EXTERNAL / f"pilot_{defect}"
    explicit = pilot / "attempt-002"
    if explicit.is_dir() and (explicit / "result.csv").is_file():
        return explicit
    workdir = pilot / "workdir"
    if workdir.is_dir() and (workdir / "result.csv").is_file():
        return workdir
    return None


def parse_distances(path: Path) -> list[dict]:
    with path.open(newline="") as handle:
        rows = list(csv.DictReader(handle))
    for row in rows:
        row["best_edit_distances"] = float(row["best_edit_distances"])
    return rows


def record(defect: str) -> dict | None:
    attempt = attempt_dir(defect)
    if attempt is None:
        return None
    with (attempt / "result.csv").open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    distances = parse_distances(attempt / "edit_distance/best_edit_distances.csv")
    compiled = int(summary["total_rust_functions_compiled"]) > 0 and summary["c_coverage"] != ""
    completed = compiled and bool(distances)
    detected = completed and any(row["best_edit_distances"] != 0.0 for row in distances)
    outcome = "detected" if detected else "missed" if completed else "compile_failure"
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": defect,
        "target": distances[0]["function_name"] if distances else None,
        "gates": {
            "submitted": True,
            "accepted": True,
            "compiled": compiled,
            "completed": completed,
            "detected": detected,
        },
        "outcome": outcome,
        "artifact_native_symbolic_analysis": True,
        "distances": distances,
        "summary": summary,
        "external_directory": str(attempt),
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in [
                attempt / "result.csv",
                attempt / "edit_distance/best_edit_distances.csv",
                attempt / "rust_klee_terminate_results.csv",
                attempt / "c_klee_terminate_results.csv",
            ]
            if path.is_file()
        },
        "interpretation": (
            "At least one RustAssure symbolic-value graph has nonzero C/Rust edit distance."
            if detected
            else "All symbolic argument and return graphs reported by RustAssure have zero C/Rust edit distance."
        ),
    }
    out = RUN_ROOT / defect
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def record_c1_direct_failure() -> dict:
    """Record C1's first released-artifact result, never its adapted retry.

    RustAssure's pinned stable compiler rejected the submitted Rust file before
    symbolic comparison.  A later file with crate-level nightly attributes
    removed is retained only as exploratory evidence; it must not replace this
    scored failure.
    """
    attempt = EXTERNAL / "pilot_C1_direct/workdir"
    logger = attempt / "test_llvm_bitcode_emitter_logger.log"
    rust_input = attempt / "testcase/Rust/quickSort.rs"
    c_input = attempt / "testcase/C/quickSort.i"
    text = logger.read_text()
    assert "Compilation failed for testcase/Rust/quickSort.rs" in text
    assert "Out of 1 total Rust files 0 compiled" in text
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": "C1",
        "target": "quickSort",
        "gates": {
            "submitted": True,
            "accepted": True,
            "compiled": False,
            "completed": False,
            "detected": False,
        },
        "outcome": "compile_failure",
        "artifact_native_symbolic_analysis": False,
        "formal_attempt": str(attempt),
        "exploratory_retry": str(EXTERNAL / "pilot_C1"),
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in (logger, rust_input, c_input)
        },
        "interpretation": (
            "The released RustAssure emitter accepted the submitted pair but "
            "its pinned stable Rust compiler compiled 0/1 Rust functions. "
            "The later retry that removes crate-level nightly attributes is "
            "exploratory and cannot replace this scored compile failure."
        ),
    }
    out = RUN_ROOT / "C1"
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def main() -> None:
    payloads = {"C1": record_c1_direct_failure()}
    payloads.update(
        {
            defect: payload
            for defect in ["S6", "S21", "S17", "C12"]
            if (payload := record(defect))
        }
    )
    data = json.loads(RESULTS.read_text())
    for row in data["records"]:
        defect = row["defect_id"]
        if row["baseline"] != "rustassure" or defect not in payloads:
            continue
        payload = payloads[defect]
        row.update(
            {
                "gates": payload["gates"],
                "outcome": payload["outcome"],
                "adapter": f"results/baseline_artifacts/adapters/rustassure/{defect}/adapter.json",
                "command_record": f"scripts/baseline_artifacts/run_rustassure_pair.sh {defect}",
                "result_record": f"results/baseline_artifacts/runs/rustassure/{defect}/result.json",
                "notes": [payload["interpretation"]],
            }
        )
    RESULTS.write_text(json.dumps(data, indent=2) + "\n")
    print(f"recorded {len(payloads)} completed RustAssure pilots: {', '.join(sorted(payloads))}")


if __name__ == "__main__":
    main()
