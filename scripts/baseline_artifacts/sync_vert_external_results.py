#!/usr/bin/env python3
"""Synchronize the scored VERT external-pair run into the canonical matrix."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/vert_external"

# These outcomes reached a generated project that cargo could execute.  They
# include invalid relational results and reference-side termination; neither is
# a completed analysis, but both passed the compilation gate.
COMPILED_ANALYSIS_FAILURES = {
    "generated_reference_terminates",
    "invalid_counterexample_from_input_mismatch",
}


def main() -> None:
    data = json.loads(SOURCE.read_text())
    summary = json.loads((RUN_ROOT / "summary.json").read_text())
    subtype_by_defect = {
        defect: subtype
        for subtype, item in summary["failure_subtypes"].items()
        for defect in item["defects"]
    }

    replacements: dict[str, dict] = {}
    for result_path in sorted(RUN_ROOT.glob("*/result.json")):
        result = json.loads(result_path.read_text())
        defect = result["defect_id"]
        external_outcome = result["outcome"]
        subtype = subtype_by_defect[defect]
        accepted = external_outcome != "unsupported_input"
        compiled = (
            external_outcome == "completed_miss"
            or subtype in COMPILED_ANALYSIS_FAILURES
        )
        completed = external_outcome == "completed_miss"
        canonical_outcome = {
            "completed_miss": "missed",
            "unsupported_input": "unsupported_input",
            "analysis_failure": "analysis_failure",
            "compile_failure": "compile_failure",
        }[external_outcome]
        replacements[defect] = {
            "baseline": "vert",
            "defect_id": defect,
            "pilot": False,
            "gates": {
                "submitted": True,
                "accepted": accepted,
                "compiled": compiled,
                "completed": completed,
                "detected": False,
            },
            "outcome": canonical_outcome,
            "adapter": "scripts/baseline_artifacts/run_vert_external_pairs.py",
            "command_record": f"run_vert_external_pairs.py --defect {defect}",
            "result_record": str(result_path.relative_to(ROOT)),
            "notes": [
                "The fixed external-pair adapter packages the audited C/Rust payload while preserving VERT's released two-element array model, PARAM1-only reference injection, return-only observation, and Bolero search.",
                f"Scored outcome: {external_outcome}; failure subtype: {subtype}. No VERT analysis, comparison, or search behavior was repaired.",
            ],
        }

    if len(replacements) != 36:
        raise SystemExit(f"expected 36 VERT results, found {len(replacements)}")

    seen: set[str] = set()
    for index, row in enumerate(data["records"]):
        if row["baseline"] != "vert":
            continue
        defect = row["defect_id"]
        data["records"][index] = replacements[defect]
        seen.add(defect)
    if seen != set(replacements):
        raise SystemExit("canonical VERT defect IDs do not match external run")

    SOURCE.write_text(json.dumps(data, indent=2) + "\n")
    print(f"updated {SOURCE}")


if __name__ == "__main__":
    main()
