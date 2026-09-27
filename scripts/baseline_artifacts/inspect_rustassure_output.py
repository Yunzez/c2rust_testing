#!/usr/bin/env python3
"""Extract released RustAssure signals without assigning a scored outcome.

Nonzero graph distance or asymmetric termination is only a baseline signal.
Promotion to ``detected`` still requires source-level validation that the
signal corresponds to the catalogued defect and, where relevant, a valid C
execution.  This script intentionally cannot update ``results.json``.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def csv_rows(path: Path) -> list[dict[str, str]]:
    if not path.is_file():
        return []
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


def numeric_nonzero(value: str | None) -> bool:
    if value in (None, ""):
        return False
    try:
        return float(value) != 0.0
    except ValueError:
        return True


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("defect_id")
    parser.add_argument("workdir", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    workdir = args.workdir.resolve()
    runner_root = workdir.parent if workdir.name == "workdir" else workdir
    summary_rows = csv_rows(workdir / "result.csv")
    distances = csv_rows(workdir / "edit_distance/best_edit_distances.csv")
    rust_termination = csv_rows(workdir / "rust_klee_terminate_results.csv")
    c_termination = csv_rows(workdir / "c_klee_terminate_results.csv")
    summary = summary_rows[0] if summary_rows else {}

    compiled = bool(summary) and int(summary.get("total_rust_functions_compiled", "0")) > 0
    completed = compiled and bool(distances)
    distance_signals = [
        row for row in distances if numeric_nonzero(row.get("best_edit_distances"))
    ]
    termination_fields = (
        "reach_max_cases", "dump_error_counts", "terminate_error", "time_out"
    )
    rust_termination_signals = [
        row
        for row in rust_termination
        if any(numeric_nonzero(row.get(field)) for field in termination_fields)
    ]
    c_termination_signals = [
        row
        for row in c_termination
        if any(numeric_nonzero(row.get(field)) for field in termination_fields)
    ]

    evidence_paths = [
        workdir / "result.csv",
        workdir / "edit_distance/best_edit_distances.csv",
        workdir / "rust_klee_terminate_results.csv",
        workdir / "c_klee_terminate_results.csv",
        workdir / "test_llvm_bitcode_emitter_logger.log",
        workdir / "symbol_execution_error.log",
        runner_root / "exit_code.txt",
        runner_root / "console.log",
    ]
    report = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": args.defect_id,
        "workdir": str(workdir),
        "runner_exit_code": (
            int((runner_root / "exit_code.txt").read_text().strip())
            if (runner_root / "exit_code.txt").is_file()
            else None
        ),
        "provisional_gates": {
            "submitted": True,
            "accepted": True,
            "compiled": compiled,
            "completed": completed,
        },
        "official_summary": summary,
        "distance_rows": distances,
        "distance_signals": distance_signals,
        "rust_termination_rows": rust_termination,
        "rust_termination_signals": rust_termination_signals,
        "c_termination_rows": c_termination,
        "c_termination_signals": c_termination_signals,
        "has_baseline_signal": bool(
            distance_signals or rust_termination_signals or c_termination_signals
        ),
        "requires_manual_defect_validation": bool(
            distance_signals or rust_termination_signals or c_termination_signals
        ),
        "scored_outcome": None,
        "hashes": {
            (
                str(path.relative_to(workdir))
                if path.is_relative_to(workdir)
                else "runner/" + path.name
            ): sha256(path)
            for path in evidence_paths
            if path.is_file()
        },
    }
    rendered = json.dumps(report, indent=2) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered)
    else:
        print(rendered, end="")


if __name__ == "__main__":
    main()
