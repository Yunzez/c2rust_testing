#!/usr/bin/env python3
"""Record FLOURINE incompatibilities that require changing its observation model."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
RESULTS = ROOT / "results/baseline_artifacts/results.json"
RUN_ROOT = ROOT / "results/baseline_artifacts/runs/flourine"
GENERATED_ORACLE = Path(
    "/home/yunzez/c2rust_baselines/runs/flourine/pilot_C4/attempt-001/verification/src/lib.rs"
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    records = {
        "S8": {
            "schema_version": 1,
            "baseline": "flourine",
            "defect_id": "S8",
            "target": "parse_string",
            "gates": {
                "submitted": False,
                "accepted": False,
                "compiled": False,
                "completed": False,
                "detected": False,
            },
            "outcome": "unsupported_input",
            "unsupported_dimension": "observation",
            "interpretation": (
                "S8 preserves parse_string's return value and changes only the valuestring field "
                "of an object allocated inside the input adapter. The released FLOURINE oracle "
                "compares termination, the original return value, and submitted arguments after "
                "the call; that local heap state is not an observable. Exposing it would require "
                "a new state comparator or a return-value rewrite, both forbidden by the frozen "
                "baseline policy."
            ),
            "oracle_evidence": {
                "generated_file": str(GENERATED_ORACLE),
                "sha256": sha256(GENERATED_ORACLE),
                "relevant_lines": "223-279 compare C/Rust termination, return, and input0",
            },
        }
    }

    data = json.loads(RESULTS.read_text())
    for defect, payload in records.items():
        out = RUN_ROOT / defect
        out.mkdir(parents=True, exist_ok=True)
        result_path = out / "result.json"
        result_path.write_text(json.dumps(payload, indent=2) + "\n")
        for row in data["records"]:
            if row["baseline"] == "flourine" and row["defect_id"] == defect:
                row.update(
                    {
                        "gates": payload["gates"],
                        "outcome": payload["outcome"],
                        "adapter": None,
                        "command_record": None,
                        "result_record": str(result_path.relative_to(ROOT)),
                        "notes": [payload["interpretation"]],
                    }
                )
                break
        else:
            raise RuntimeError(f"missing FLOURINE result row for {defect}")
    RESULTS.write_text(json.dumps(data, indent=2) + "\n")
    print("recorded FLOURINE compatibility: " + ", ".join(records))


if __name__ == "__main__":
    main()
