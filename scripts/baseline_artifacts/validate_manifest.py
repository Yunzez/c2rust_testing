#!/usr/bin/env python3
"""Validate the canonical defect/pilot manifests and initialize funnel records."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


BASELINES = ("rustassure", "flourine", "vert")
GATES = ("submitted", "accepted", "compiled", "completed", "detected")


def read_json(path: Path):
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def write_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--initialize", action="store_true")
    args = parser.parse_args()

    repo = args.repo.resolve()
    canonical_path = repo / "results/rq4_effectiveness/defect_manifest.json"
    pilot_path = repo / "results/baseline_artifacts/pilot_manifest.json"
    lock_path = repo / "results/baseline_artifacts/artifact_lock.json"
    results_path = repo / "results/baseline_artifacts/results.json"
    adapter_policy_path = repo / "results/baseline_artifacts/adapter_policy.json"

    canonical = read_json(canonical_path)["defects"]
    pilot = read_json(pilot_path)["pilot"]
    artifact_lock = read_json(lock_path)

    assert len(canonical) == 36, f"expected 36 defects, found {len(canonical)}"
    by_id = {entry["id"]: entry for entry in canonical}
    assert len(by_id) == 36, "defect IDs are not unique"
    assert len(pilot) == 5, f"expected five pilots, found {len(pilot)}"

    for entry in pilot:
        defect_id = entry["id"]
        assert defect_id in by_id, f"pilot {defect_id} absent from canonical manifest"
        evidence_root = repo / entry["evidence_root"]
        assert evidence_root.exists(), f"pilot {defect_id} evidence missing: {evidence_root}"
        assert by_id[defect_id]["provenance"] == "exact-source", (
            f"pilot {defect_id} is not exact-source provenance"
        )

    for baseline in BASELINES:
        assert baseline in artifact_lock, f"missing lock for {baseline}"

    if results_path.exists() and adapter_policy_path.exists():
        records = read_json(results_path)["records"]
        policies = read_json(adapter_policy_path)
        for baseline in ("rustassure", "flourine"):
            policy = policies[baseline]
            scored = {
                record["defect_id"]: record
                for record in records
                if record["baseline"] == baseline
                and record["outcome"] not in ("not_run", "unsupported_input")
            }
            assert set(scored) == set(policy), (
                f"{baseline}: adapter audit must cover every submitted result exactly"
            )
            for defect_id, record in scored.items():
                audit = policy[defect_id]
                if audit["scored_class"] == "released_baseline_failure":
                    assert record["outcome"] == audit["outcome"], (
                        f"{baseline}/{defect_id}: baseline workaround must not replace scored failure"
                    )
                    assert not record["gates"]["detected"], (
                        f"{baseline}/{defect_id}: failure record cannot be scored as detected"
                    )
                else:
                    assert record["adapter"], (
                        f"{baseline}/{defect_id}: scored adapter is missing"
                    )
                    assert (repo / record["adapter"]).is_file(), (
                        f"{baseline}/{defect_id}: scored adapter metadata is missing"
                    )

    if args.initialize:
        output = results_path
        if output.exists():
            raise SystemExit(f"refusing to replace existing result records: {output}")
        pilot_ids = {entry["id"] for entry in pilot}
        records = []
        for baseline in BASELINES:
            for defect in canonical:
                records.append(
                    {
                        "baseline": baseline,
                        "defect_id": defect["id"],
                        "pilot": defect["id"] in pilot_ids,
                        "gates": {gate: False for gate in GATES},
                        "outcome": "not_run",
                        "adapter": None,
                        "command_record": None,
                        "result_record": None,
                        "notes": [],
                    }
                )
        write_json(
            output,
            {
                "schema_version": 1,
                "canonical_manifest": str(canonical_path.relative_to(repo)),
                "records": records,
            },
        )

    print(f"validated {len(canonical)} defects, {len(pilot)} pilots, {len(BASELINES)} baselines")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
