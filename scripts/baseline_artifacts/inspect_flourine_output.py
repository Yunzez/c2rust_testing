#!/usr/bin/env python3
"""Extract released FLOURINE gates and signals without scoring a defect.

FLOURINE counterexamples, crashes, or status-77 exits are candidates only.
The caller must validate a candidate against the catalogued root cause before
writing ``detected`` to the canonical result matrix.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path: Path) -> str:
    return path.read_text(errors="replace") if path.is_file() else ""


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("defect_id")
    parser.add_argument("attempt", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    attempt = args.attempt.resolve()
    build_log = read(attempt / "cmake-build.log")
    rust_log = read(attempt / "instrument-rust.log")
    verify_log = read(attempt / "verify.log")
    exit_text = read(attempt / "exit_code.txt").strip()
    exit_code = int(exit_text) if exit_text.isdigit() else None
    crashes = sorted(
        path
        for path in attempt.glob("verification/src/__fuzz__/*/crashes/*")
        if path.is_file()
    )

    submitted = (attempt / "input.sha256").is_file()
    c_instrumented = any(attempt.glob("verification*/ground_truth/instrumented.cpp"))
    c_compiled = "Built target ground_truth" in build_log
    rust_instrumented = (
        (attempt / "instrument-rust.log").is_file()
        and rust_log.strip() == ""
        and (attempt / "verification/Cargo.toml").is_file()
    )
    compiled = c_compiled and rust_instrumented and (attempt / "target.txt").is_file()
    accepted = c_instrumented and (not c_compiled or rust_instrumented)
    completed_runs = re.search(r"Done (\d+) runs in (\d+) second", verify_log)
    counterexample_signal = "counter examples:" in verify_log
    crash_signal = bool(crashes)
    status77_signal = "status 77" in verify_log
    has_signal = counterexample_signal or crash_signal or status77_signal
    completed = compiled and bool(completed_runs or has_signal)

    evidence_names = (
        "input.sha256", "instrument-c.log", "cmake-configure.log",
        "cmake-build.log", "instrument-rust.log", "bolero-list.log",
        "target.txt", "verify.log", "console.log", "exit_code.txt",
    )
    report = {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": args.defect_id,
        "attempt": str(attempt),
        "provisional_gates": {
            "submitted": submitted,
            "accepted": accepted,
            "compiled": compiled,
            "completed": completed,
        },
        "released_exit_code": exit_code,
        "completed_runs": (
            {
                "runs": int(completed_runs.group(1)),
                "seconds": int(completed_runs.group(2)),
            }
            if completed_runs
            else None
        ),
        "signals": {
            "counterexamples_in_log": counterexample_signal,
            "status_77_in_log": status77_signal,
            "crash_count": len(crashes),
            "crash_hashes": {
                str(path.relative_to(attempt)): sha256(path) for path in crashes
            },
        },
        "has_baseline_signal": has_signal,
        "requires_manual_defect_validation": has_signal,
        "failure_observations": {
            "c_compile_failed": submitted and not c_compiled,
            "rust_instrumenter_message": rust_log.strip() or None,
            "verification_started": (attempt / "verify.log").is_file(),
        },
        "scored_outcome": None,
        "hashes": {
            name: sha256(attempt / name)
            for name in evidence_names
            if (attempt / name).is_file()
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
