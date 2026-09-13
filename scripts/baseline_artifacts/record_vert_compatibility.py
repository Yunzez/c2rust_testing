#!/usr/bin/env python3
"""Record VERT's released-input limitation for the frozen 36-defect matrix."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
AUDIT = "results/baseline_artifacts/setup/vert/capability_audit.json"


def main() -> None:
    data = json.loads(RESULTS.read_text())
    changed = 0
    for record in data["records"]:
        if record["baseline"] != "vert":
            continue
        record["gates"] = {
            "submitted": False,
            "accepted": False,
            "compiled": False,
            "completed": False,
            "detected": False,
        }
        record["outcome"] = "unsupported_input"
        record["adapter"] = None
        record["command_record"] = (
            "not submitted: the released evaluator has no documented input for an "
            "already-produced Rust translation"
        )
        record["result_record"] = AUDIT
        record["notes"] = [
            "VERT's released evaluator replays deposited benchmark projects; its rebuild path generates a new Rust candidate with an LLM.",
            "Injecting the frozen translated artifact would require an undocumented pipeline modification, which this comparison forbids.",
        ]
        changed += 1
    if changed != 36:
        raise SystemExit(f"expected 36 VERT records, updated {changed}")
    RESULTS.write_text(json.dumps(data, indent=2) + "\n")
    print(f"recorded unsupported_input for {changed} VERT records")


if __name__ == "__main__":
    main()
