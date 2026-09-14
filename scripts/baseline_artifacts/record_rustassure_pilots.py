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
        try:
            row["best_edit_distances"] = float(row["best_edit_distances"])
        except ValueError:
            # The released artifact uses strings such as `Rust Empty!` when
            # symbolic execution produced no graph. Preserve that native
            # signal for manual adjudication rather than coercing it.
            pass
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

    distance_signal = any(
        numeric_nonzero(str(row["best_edit_distances"])) for row in distances
    )
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


def record_s7_compile_failure() -> dict:
    """Record the released compiler's failure on the exact S7 package."""
    attempt = EXTERNAL / "pilot_S7/workdir"
    logger = attempt / "test_llvm_bitcode_emitter_logger.log"
    rust_input = attempt / "testcase/Rust/parse_string_ascii.rs"
    c_input = attempt / "testcase/C/parse_string_ascii.i"
    log_text = logger.read_text()
    assert "Compilation failed for testcase/Rust/parse_string_ascii.rs" in log_text
    assert "Out of 1 total Rust files 0 compiled" in log_text
    with (attempt / "result.csv").open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    assert summary["total_rust_functions_compiled"] == "0"
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": "S7",
        "target": "parse_string_ascii",
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
        "summary": summary,
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in (logger, rust_input, c_input, attempt / "result.csv")
        },
        "interpretation": (
            "The released RustAssure emitter accepted the exact dependency package, "
            "but its pinned Rust compiler failed on the frozen PtrTrans source and "
            "compiled 0/1 Rust functions. The subsequent `Rust Empty!` graph rows "
            "contain no Rust execution and are not a defect signal; no compiler or "
            "source workaround is attempted."
        ),
    }
    out = RUN_ROOT / "S7"
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def record_s9_compile_failure() -> dict:
    """Record the released compiler's failure on the exact S9 package."""
    attempt = EXTERNAL / "pilot_S9/workdir"
    logger = attempt / "test_llvm_bitcode_emitter_logger.log"
    rust_input = attempt / "testcase/Rust/parse_string_valid.rs"
    c_input = attempt / "testcase/C/parse_string_valid.i"
    log_text = logger.read_text()
    assert "Compilation failed for testcase/Rust/parse_string_valid.rs" in log_text
    assert "Out of 1 total Rust files 0 compiled" in log_text
    with (attempt / "result.csv").open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    assert summary["total_rust_functions_compiled"] == "0"
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": "S9",
        "target": "parse_string_valid",
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
        "summary": summary,
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in (logger, rust_input, c_input, attempt / "result.csv")
        },
        "interpretation": (
            "The released RustAssure emitter accepted the exact S9 package, but "
            "its pinned Rust compiler compiled 0/1 Rust functions. The subsequent "
            "`Rust Empty!` graph rows contain no Rust execution and are not a defect "
            "signal; no compiler or source workaround is attempted."
        ),
    }
    out = RUN_ROOT / "S9"
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def record_compile_failure(
    defect: str,
    target: str,
    *,
    extra_interpretation: str = "",
    extra_paths: tuple[str, ...] = (),
) -> dict:
    """Record a released Rust compiler failure without attempting a retry."""
    attempt = EXTERNAL / f"pilot_{defect}/workdir"
    logger = attempt / "test_llvm_bitcode_emitter_logger.log"
    rust_input = attempt / f"testcase/Rust/{target}.rs"
    c_input = attempt / f"testcase/C/{target}.i"
    log_text = logger.read_text()
    assert f"Compilation failed for testcase/Rust/{target}.rs" in log_text
    assert "Out of 1 total Rust files 0 compiled" in log_text
    with (attempt / "result.csv").open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    assert summary["total_rust_functions_compiled"] == "0"
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": defect,
        "target": target,
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
        "summary": summary,
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in (
                logger,
                rust_input,
                c_input,
                attempt / "result.csv",
                *(attempt / path for path in extra_paths),
            )
        },
        "interpretation": (
            f"The released RustAssure emitter accepted the exact {defect} package, "
            "but its pinned Rust compiler compiled 0/1 Rust functions. The "
            "subsequent `Rust Empty!` rows contain no Rust execution and are not "
            "a defect signal; no compiler or source workaround is attempted."
            + extra_interpretation
        ),
    }
    out = RUN_ROOT / defect
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def record_c3_analysis_failure() -> dict:
    """Record C3 after the released full-budget analysis produced no result."""
    defect = "C3"
    target = "do_system_two"
    attempt = EXTERNAL / f"pilot_{defect}/workdir"
    result_csv = attempt / "result.csv"
    distances_path = attempt / "edit_distance/best_edit_distances.csv"
    rust_termination = attempt / "rust_klee_terminate_results.csv"
    c_termination = attempt / "c_klee_terminate_results.csv"
    rust_info = attempt / "klee_ir_files/Rust/klee-out-0/info"
    rust_messages = attempt / "klee_ir_files/Rust/klee-out-0/messages.txt"
    c_info = attempt / "klee_ir_files/C/klee-out-0/info"
    c_messages = attempt / "klee_ir_files/C/klee-out-0/messages.txt"
    rust_input = attempt / f"testcase/Rust/{target}.rs"
    c_input = attempt / f"testcase/C/{target}.i"

    with result_csv.open(newline="") as handle:
        summary = next(csv.DictReader(handle))
    distances = parse_distances(distances_path)
    assert summary["total_rust_functions_compiled"] == "1"
    assert summary["c_coverage"] != "" and summary["rust_coverage"] != ""
    assert summary["execution_time"] == "180m"
    assert not distances
    assert not termination_signal(rust_termination)
    assert not termination_signal(c_termination)

    rust_info_text = rust_info.read_text()
    rust_messages_text = rust_messages.read_text()
    c_info_text = c_info.read_text()
    assert "--max-time=10800" in rust_info_text
    assert "completed paths = 0" in rust_info_text
    assert "generated tests = 0" in rust_info_text
    assert "HaltTimer invoked" in rust_messages_text
    assert "completed paths = 0" in c_info_text
    assert "generated tests = 0" in c_info_text

    evidence = (
        result_csv,
        distances_path,
        rust_termination,
        c_termination,
        rust_info,
        rust_messages,
        c_info,
        c_messages,
        rust_input,
        c_input,
    )
    payload = {
        "schema_version": 1,
        "baseline": "rustassure",
        "defect_id": defect,
        "target": target,
        "gates": {
            "submitted": True,
            "accepted": True,
            "compiled": True,
            "completed": False,
            "detected": False,
        },
        "outcome": "analysis_failure",
        "artifact_native_symbolic_analysis": True,
        "formal_attempt": str(attempt),
        "summary": summary,
        "distances": distances,
        "hashes": {
            str(path.relative_to(attempt)): sha256(path)
            for path in evidence
        },
        "interpretation": (
            "The released RustAssure artifact compiled both sides and ran its "
            "documented 180-minute symbolic-analysis budget. Rust KLEE exhausted "
            "the budget with 959,919 partial paths, zero completed paths, and zero "
            "generated tests; the official pipeline emitted no graph-distance or "
            "termination result. Three locationless `unreachable` messages are "
            "retained as unscored partial evidence because they were not promoted "
            "by the artifact into a completed result. No runtime, model, or source "
            "workaround is attempted."
        ),
    }
    out = RUN_ROOT / defect
    out.mkdir(parents=True, exist_ok=True)
    (out / "result.json").write_text(json.dumps(payload, indent=2) + "\n")
    return payload


def main() -> None:
    decision_doc = json.loads(DECISIONS.read_text())
    assert decision_doc["baseline"] == "rustassure"
    decisions = decision_doc["decisions"]
    payloads = {
        "C1": record_c1_direct_failure(),
        "S7": record_s7_compile_failure(),
        "S9": record_s9_compile_failure(),
        "S2": record_compile_failure("S2", "adler32_z_packet"),
        "S1": record_compile_failure("S1", "crc32_z_packet"),
        "S4": record_compile_failure("S4", "crc32_z_packet"),
        "S8": record_compile_failure("S8", "parse_string"),
        "S5": record_compile_failure("S5", "genann_cached_initialized"),
        "S19": record_compile_failure("S19", "zlib_compress_observe"),
        "C7": record_compile_failure("C7", "bzbuff_compress_observe"),
        "S10": record_compile_failure("S10", "bzbuff_compress_observe"),
        "C11": record_compile_failure("C11", "ti_find_indicator_valid"),
        "C6": record_compile_failure(
            "C6",
            "sample_main_packet",
            extra_interpretation=(
                " Independently, the released C symbolic stage also fails while "
                "linking the emitted bitcode against its bundled libc model."
            ),
            extra_paths=("symbol_execution_error.log",),
        ),
        "S20": record_compile_failure("S20", "zlib_uncompress_observe"),
        "C15": record_compile_failure("C15", "zlib_uncompress_observe"),
        "S11": record_compile_failure("S11", "bzbuff_decompress_observe"),
        "S12": record_compile_failure("S12", "bzbuff_compress_packet"),
        "C3": record_c3_analysis_failure(),
        "C9": record_compile_failure("C9", "lil_new_probe"),
        "C10": record_compile_failure("C10", "lil_parse_generated"),
        "S18": record_compile_failure("S18", "zlib_uncompress_observe"),
        "C8": record_compile_failure("C8", "bzbuff_compress_observe"),
        "S3": record_compile_failure("S3", "bzbuff_compress_observe"),
    }
    payloads.update(
        {
            defect: payload
            for defect in [
                "S6", "S21", "S14", "S17", "C12", "C2", "C13", "C16",
                "C4", "S15", "S16",
            ]
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
