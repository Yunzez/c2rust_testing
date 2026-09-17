#!/usr/bin/env python3
"""Summarize the frozen-pair external-adapter experiment for VERT."""

from __future__ import annotations

from collections import Counter, defaultdict
import json
from pathlib import Path


REPO = Path(__file__).resolve().parents[2]
ROOT = REPO / "results/baseline_artifacts/runs/vert_external"
DEFECTS = [
    *(f"C{i}" for i in range(1, 17) if i != 14),
    *(f"S{i}" for i in range(1, 22)),
]


def failure_subtype(record: dict) -> str:
    reason = record.get("reason", "")
    outcome = record["outcome"]
    if outcome == "completed_miss":
        return "valid_completed_comparison"
    if outcome == "unsupported_input":
        if "process-entry" in reason:
            return "process_entry_without_function_payload"
        if "pointer-return" in reason:
            return "pointer_return_unrepresented"
        if "does not match" in reason:
            return "no_common_audited_wrapper"
        return "unsupported_other"
    if outcome == "compile_failure":
        if "reference construction" in reason:
            return "c_to_wasm_compile_failure"
        return "generated_project_compile_failure"
    if "reported a counterexample" in reason:
        return "invalid_counterexample_from_input_mismatch"
    if "fallback window" in reason:
        return "released_fallback_cannot_locate_call"
    if "no reference mutation" in reason:
        return "released_injection_has_no_mutation"
    if "checker exited unsuccessfully" in reason:
        return "generated_reference_terminates"
    return "analysis_failure_other"


def main() -> None:
    records = []
    for defect in DEFECTS:
        path = ROOT / defect / "result.json"
        if not path.exists():
            raise SystemExit(f"missing result: {path}")
        record = json.loads(path.read_text())
        if record.get("status") != "complete":
            raise SystemExit(f"incomplete result: {path}")
        record = dict(record)
        record["failure_subtype"] = failure_subtype(record)
        records.append(record)

    counts = Counter(record["outcome"] for record in records)
    subtypes: dict[str, list[str]] = defaultdict(list)
    for record in records:
        subtypes[record["failure_subtype"]].append(record["defect_id"])

    summary = {
        "schema_version": 1,
        "baseline": "vert",
        "experiment": "frozen_external_pair_v1",
        "denominator": len(records),
        "detected": [],
        "detected_count": 0,
        "outcome_counts": dict(sorted(counts.items())),
        "outcomes": {
            key: sorted(
                record["defect_id"]
                for record in records
                if record["outcome"] == key
            )
            for key in sorted(counts)
        },
        "failure_subtypes": {
            key: {"count": len(ids), "defects": sorted(ids)}
            for key, ids in sorted(subtypes.items())
        },
        "candidate_counterexamples": {
            "count": len(
                subtypes.get("invalid_counterexample_from_input_mismatch", [])
            ),
            "valid_detections": 0,
            "reason": (
                "The released VERT wrapper injects only PARAM1 into the C/rWasm "
                "reference while the candidate receives every dynamic argument."
            ),
            "control": (
                "For S14, the untouched correct deposited candidate fails the same "
                "comparison; the remaining cases use the same released injection path."
            ),
        },
        "valid_completed_comparisons": {
            "count": 1,
            "defects": ["C13"],
            "result": "completed_miss",
            "note": (
                "The one-scalar opng_free contract wrapper completes in VERT's "
                "release test configuration without exposing C13."
            ),
        },
        "interpretation": (
            "All 36 frozen defects were submitted through one fixed external-pair "
            "adapter without repairing VERT. VERT validly completed one comparison "
            "and missed it; every other pair stopped at applicability, compilation, "
            "or analysis. Eight apparent counterexamples were rejected because the "
            "two sides did not receive the same logical input. End-to-end detection "
            "is therefore 0/36."
        ),
    }
    (ROOT / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")

    lines = [
        "# VERT external frozen-pair experiment",
        "",
        "All 36 frozen defect pairs were submitted through the same external adapter. ",
        "The adapter packages audited C/Rust payloads but leaves VERT's released ",
        "two-element array model, PARAM1-only reference injection, and return-only ",
        "comparison unchanged.",
        "",
        "| Outcome | Count |",
        "|---|---:|",
    ]
    display = [
        ("Detected", 0),
        ("Completed miss", counts.get("completed_miss", 0)),
        ("Analysis failure", counts.get("analysis_failure", 0)),
        ("Compile failure", counts.get("compile_failure", 0)),
        ("Unsupported input", counts.get("unsupported_input", 0)),
    ]
    lines.extend(f"| {name} | {value} |" for name, value in display)
    lines.extend(
        [
            "",
            "VERT emitted apparent counterexamples for eight pairs, including S14. ",
            "None is a valid detection: the C/rWasm side receives only `PARAM1`, ",
            "whereas the candidate receives all dynamic parameters. The untouched ",
            "correct S14 control fails identically.",
            "",
            "C13 is the only valid completed comparison. Its one-scalar wrapper fits ",
            "VERT's native input envelope, but the release-mode comparison completes ",
            "without revealing the frozen `free(NULL)` translation defect.",
            "",
            "Raw generated projects and logs are under ",
            "`/home/yunzez/c2rust_baselines/runs/vert/external_pairs/`; each small ",
            "per-defect result is stored in this directory.",
        ]
    )
    (ROOT / "README.md").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
