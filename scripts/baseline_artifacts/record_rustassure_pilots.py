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


def main() -> None:
    payloads = {defect: payload for defect in ["C1", "S6", "S21", "S17", "C12"] if (payload := record(defect))}
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
