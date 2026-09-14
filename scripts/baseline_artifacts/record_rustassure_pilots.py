#!/usr/bin/env python3
"""Record manually adjudicated released-RustAssure pilot analyses.

The released artifact's graph distances and termination rows are candidate
signals, not scored detections.  This recorder therefore refuses to score a
completed analysis unless ``scoring_decisions/rustassure.json`` contains an
explicit source-level adjudication for that defect.
"""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
EXTERNAL = Path("/home/yunzez/c2rust_baselines/runs/rustassure")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/rustassure"
DECISIONS = ROOT / "results/baseline_artifacts/scoring_decisions/rustassure.json"


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


def numeric_nonzero(value: str | None) -> bool:
    if value in (None, ""):
        return False
    try:
        return float(value) != 0.0
    except ValueError:
        return True


def termination_signal(path: Path) -> bool:
    if not path.is_file():
        return False
    with path.open(newline="") as handle:
        rows = list(csv.DictReader(handle))
    fields = ("reach_max_cases", "dump_error_counts", "terminate_error", "time_out")
    return any(any(numeric_nonzero(row.get(field)) for field in fields) for row in rows)


def record(defect: str, decisions: dict[str, dict]) -> dict | None:
    attempt = attempt_dir(defect)
    if attempt is None:
        return None
    with (attempt / "result.csv").open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    distances = parse_distances(attempt / "edit_distance/best_edit_distances.csv")
    compiled = int(summary["total_rust_functions_compiled"]) > 0 and summary["c_coverage"] != ""
    completed = compiled and bool(distances)
    if not completed:
        raise RuntimeError(
            f"{defect}: this recorder only accepts completed analyses; classify the "
            "released-artifact failure separately"
        )

    distance_signal = any(row["best_edit_distances"] != 0.0 for row in distances)
    has_baseline_signal = distance_signal or termination_signal(
        attempt / "rust_klee_terminate_results.csv"
    ) or termination_signal(attempt / "c_klee_terminate_results.csv")
    if defect not in decisions:
        raise RuntimeError(
            f"{defect}: completed output has not been manually adjudicated in {DECISIONS}"
        )
    decision = decisions[defect]
    outcome = decision["outcome"]
    if outcome not in {"detected", "missed"}:
        raise ValueError(f"{defect}: invalid completed-analysis outcome {outcome!r}")
    matches = decision["baseline_signal_matches_defect"]
    if (outcome == "detected") != matches:
        raise ValueError(f"{defect}: outcome and baseline_signal_matches_defect disagree")
    if matches and not has_baseline_signal:
        raise ValueError(f"{defect}: a detection decision requires an artifact signal")
    for required in ("source_level_validation", "c_oracle_status", "reviewed_by"):
        if not decision.get(required):
            raise ValueError(f"{defect}: manual decision lacks {required}")
    detected = outcome == "detected"
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
        "has_baseline_signal": has_baseline_signal,
        "distances": distances,
        "summary": summary,
        "manual_validation": decision,
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
        "interpretation": decision["paper_summary"],
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
    decision_doc = json.loads(DECISIONS.read_text())
    assert decision_doc["baseline"] == "rustassure"
    decisions = decision_doc["decisions"]
    payloads = {"C1": record_c1_direct_failure()}
    payloads.update(
        {
            defect: payload
            for defect in ["S6", "S21", "S17", "C12", "C2"]
            if (payload := record(defect, decisions))
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
