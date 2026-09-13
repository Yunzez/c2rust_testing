#!/usr/bin/env python3
"""Record audited FLOURINE pilot outcomes and update the shared funnel."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
EXTERNAL = Path("/home/yunzez/c2rust_baselines/runs/flourine")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/flourine"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def hashes(directory: Path, names: list[str]) -> dict[str, str]:
    return {name: sha256(directory / name) for name in names if (directory / name).is_file()}


def require(path: Path, pattern: str) -> str:
    text = path.read_text(errors="replace")
    if not re.search(pattern, text, re.MULTILINE):
        raise RuntimeError(f"missing expected evidence {pattern!r} in {path}")
    return text


def write_result(defect: str, payload: dict) -> None:
    out = RUN_ROOT / defect
    out.mkdir(parents=True, exist_ok=True)
    path = out / "result.json"
    path.write_text(json.dumps(payload, indent=2) + "\n")


def record_c1() -> dict:
    attempt = EXTERNAL / "pilot_C1/attempt-003"
    log = require(attempt / "verify.log", r"overflowed its stack")
    executions = re.findall(r"#(\d+)\s", log)
    payload = {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": "C1",
        "target": "quickSort_valid",
        "gates": {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": True},
        "outcome": "detected",
        "artifact_native_search": True,
        "witness_replay": False,
        "detection": "Rust stack overflow on a fixed 16-element valid qsort range; the corresponding C recursion is bounded.",
        "executions_before_detection_upper_bound": int(executions[-1]) if executions else None,
        "attempts": [
            {"name": "attempt-001", "outcome": "compile_failure", "detail": "The released C emitter generated an incomplete unsized array declaration."},
            {"name": "attempt-002", "outcome": "analysis_failure", "detail": "Independent arbitrary low/high values drove the C reference outside its input contract and triggered the artifact crash handler."},
            {
                "name": "attempt-003",
                "outcome": "detected",
                "external_directory": str(attempt),
                "adapter": "fixed 16-element array with the valid inclusive range [0,15]; array contents remain artifact-generated",
                "hashes": hashes(attempt, ["input.sha256", "target.txt", "verify.log", "console.log"]),
            },
        ],
    }
    write_result("C1", payload)
    return payload


def record_s6() -> dict:
    attempt = EXTERNAL / "pilot_S6/attempt-004"
    log = require(attempt / "verify.log", r"assertion `left == right` failed")
    witness = re.search(r"Input:\s*(\[[\s\S]*?\])\s*Error:", log)
    payload = {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": "S6",
        "target": "quick_sort_valid",
        "gates": {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": True},
        "outcome": "detected",
        "artifact_native_search": True,
        "witness_replay": False,
        "observation": "FLOURINE compared the mutable array after both void-returning calls and reported different permutations.",
        "shrunk_input": witness.group(1) if witness else None,
        "attempts": [
            {"name": "attempt-001", "outcome": "compile_failure", "detail": "The released C emitter generated an incomplete unsized array declaration."},
            {"name": "attempt-002", "outcome": "compile_failure", "detail": "The released Rust emitter introduced an undeclared elided lifetime for Option<&mut [i32]>."},
            {"name": "attempt-003", "outcome": "runner_invocation_failure", "detail": "A relative host volume path was rejected by Docker before submission."},
            {
                "name": "attempt-004",
                "outcome": "detected",
                "external_directory": str(attempt),
                "adapter": "fixed 16-element array with the valid inclusive range [0,15]; array contents remain artifact-generated",
                "hashes": hashes(attempt, ["input.sha256", "target.txt", "verify.log", "console.log"]),
            },
        ],
    }
    write_result("S6", payload)
    return payload


def record_c12() -> dict:
    attempt = EXTERNAL / "pilot_C12/attempt-002"
    log = require(attempt / "verify.log", r"process exited with no status code")
    runs = re.findall(r"#(\d+)\s+(?:NEW|INITED|REDUCE)", log)
    payload = {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": "C12",
        "target": "url_is_protocol_valid",
        "gates": {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": True},
        "outcome": "detected",
        "artifact_native_search": True,
        "witness_replay": False,
        "detection": "The native fuzz process terminated by signal while executing the Rust call. The generated harness catches C-side signals separately; the C table is initialized, whereas the frozen Rust table is all NULL and url_is_protocol dereferences it through strcmp.",
        "executions_before_signal_upper_bound": int(runs[-1]) if runs else None,
        "attempts": [
            {"name": "attempt-001", "outcome": "adapter_packaging_failure", "detail": "The first source extractor matched a prototype and copied an unterminated conditional block."},
            {
                "name": "attempt-002",
                "outcome": "detected",
                "external_directory": str(attempt),
                "status_interpretation": "Rust-side signal (Unix ExitStatus had no numeric code); FLOURINE's generated harness wraps only the C call in its signal-to-longjmp handler.",
                "hashes": hashes(attempt, ["input.sha256", "target.txt", "verify.log", "console.log"]),
            },
        ],
    }
    write_result("C12", payload)
    return payload


def record_s17() -> dict:
    attempt = EXTERNAL / "pilot_S17/attempt-005"
    log = require(attempt / "verify.log", r"Done \d+ runs in \d+ second")
    if "Test Failure" in log or "counter examples:" in log:
        raise RuntimeError("S17 unexpectedly contains a reported discrepancy")
    done = re.search(r"Done (\d+) runs", log)
    payload = {
        "schema_version": 1,
        "baseline": "flourine",
        "defect_id": "S17",
        "target": "opng_path_make_backup_valid",
        "gates": {"submitted": True, "accepted": True, "compiled": True, "completed": True, "detected": False},
        "outcome": "missed",
        "artifact_native_search": True,
        "witness_replay": False,
        "executions": int(done.group(1)) if done else None,
        "miss_argument": "The valid wrapper forces NUL termination and capacity 32. C appends the four-byte .bak suffix; the frozen Laertes target uses its all-zero uninitialized bak_extname and does not append it. Thus the submitted mutable output buffer contains a discrepancy within the generated domain, but the released run reported none.",
        "attempts": [
            {"name": "attempt-001", "outcome": "compile_failure", "detail": "The released C serializer does not support char array elements."},
            {"name": "attempt-002", "outcome": "analysis_failure", "detail": "A raw-pointer helper appeared as a syntactic top-level function outside the accepted C signature set."},
            {"name": "attempt-003", "outcome": "analysis_failure", "detail": "The first hidden-helper package retained crate attributes that the released parser rejected."},
            {"name": "attempt-004", "outcome": "compile_failure", "detail": "The artifact's pinned serde version cannot serialize arrays longer than 32 elements."},
            {
                "name": "attempt-005",
                "outcome": "missed",
                "external_directory": str(attempt),
                "adapter": "two fixed int8_t arrays of lengths 32 and 16; exact raw-pointer target included unchanged; both strings forced NUL-terminated",
                "hashes": hashes(attempt, ["input.sha256", "target.txt", "verify.log", "console.log"]),
            },
        ],
    }
    write_result("S17", payload)
    return payload


def update_funnel(payloads: dict[str, dict]) -> None:
    data = json.loads(RESULTS.read_text())
    for record in data["records"]:
        defect = record["defect_id"]
        if record["baseline"] != "flourine" or defect not in payloads:
            continue
        payload = payloads[defect]
        record.update(
            {
                "gates": payload["gates"],
                "outcome": payload["outcome"],
                "adapter": f"results/baseline_artifacts/adapters/flourine/{defect}/adapter.json",
                "command_record": f"scripts/baseline_artifacts/run_flourine_pair.sh {defect}",
                "result_record": f"results/baseline_artifacts/runs/flourine/{defect}/result.json",
                "notes": [
                    payload.get("detection")
                    or payload.get("observation")
                    or payload.get("miss_argument")
                ],
            }
        )
    RESULTS.write_text(json.dumps(data, indent=2) + "\n")


def main() -> None:
    payloads = {
        "C1": record_c1(),
        "S6": record_s6(),
        "C12": record_c12(),
        "S17": record_s17(),
    }
    update_funnel(payloads)


if __name__ == "__main__":
    main()
