#!/usr/bin/env python3
"""Audit the released-baseline result matrix and final generated artifacts."""

from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
MANIFEST = ROOT / "results/rq4_effectiveness/defect_manifest.json"
SUMMARIZER = ROOT / "scripts/baseline_artifacts/summarize.py"
BASELINES = {"rustassure", "flourine", "vert"}
GATES = ("submitted", "accepted", "compiled", "completed", "detected")
OUTCOMES = {
    "detected", "missed", "unsupported_input", "analysis_failure",
    "compile_failure", "not_run",
}


def load(path: Path) -> dict:
    return json.loads(path.read_text())


def load_summarizer():
    spec = importlib.util.spec_from_file_location("baseline_summarizer", SUMMARIZER)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def validate_record(record: dict) -> None:
    defect = record["defect_id"]
    baseline = record["baseline"]
    label = f"{baseline}/{defect}"
    assert record["outcome"] in OUTCOMES, label
    gates = record["gates"]
    assert tuple(gates) == GATES, label
    values = [bool(gates[gate]) for gate in GATES]
    assert values == sorted(values, reverse=True), f"non-monotone gates: {label}"

    outcome = record["outcome"]
    if outcome in {"not_run", "unsupported_input"}:
        assert not any(values), label
    elif outcome == "detected":
        assert all(values), label
    elif outcome == "missed":
        assert values == [True, True, True, True, False], label
    elif outcome == "compile_failure":
        assert gates["submitted"] and not gates["compiled"], label
        assert not gates["completed"] and not gates["detected"], label
    elif outcome == "analysis_failure":
        assert gates["submitted"] and not gates["completed"], label
        assert not gates["detected"], label

    if outcome != "not_run":
        assert record["notes"], f"missing interpretation: {label}"
        result_record = record.get("result_record")
        assert result_record, f"missing result/capability evidence: {label}"
        evidence = ROOT / result_record
        assert evidence.is_file(), f"missing evidence file: {label}: {evidence}"
        if outcome not in {"unsupported_input"}:
            payload = load(evidence)
            assert payload["baseline"] == baseline, label
            assert payload["defect_id"] == defect, label
            assert payload["outcome"] == outcome, label
            assert payload["gates"] == gates, label


def validate_generated(records: list[dict]) -> None:
    module = load_summarizer()
    summary = module.summarize(records)
    assert load(ROOT / "results/baseline_artifacts/funnel.json") == summary
    assert (ROOT / "results/baseline_artifacts/SUMMARY.md").read_text() == module.markdown(summary)
    assert (ROOT / "c2rust_paper/table/baseline_comparison.tex").read_text() == module.latex(summary)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-complete", action="store_true")
    args = parser.parse_args()

    defects = {row["id"] for row in load(MANIFEST)["defects"]}
    assert len(defects) == 36
    records = load(RESULTS)["records"]
    assert len(records) == 108
    keys = {(row["baseline"], row["defect_id"]) for row in records}
    assert len(keys) == 108
    assert {baseline for baseline, _ in keys} == BASELINES
    for baseline in BASELINES:
        assert {defect for owner, defect in keys if owner == baseline} == defects
    for record in records:
        validate_record(record)

    not_run = [
        f"{row['baseline']}/{row['defect_id']}"
        for row in records if row["outcome"] == "not_run"
    ]
    if args.require_complete:
        assert not not_run, "unfinished cells: " + ", ".join(not_run)
        validate_generated(records)
    print(
        f"validated 108 records; not_run={len(not_run)}; "
        f"require_complete={args.require_complete}"
    )


if __name__ == "__main__":
    main()
