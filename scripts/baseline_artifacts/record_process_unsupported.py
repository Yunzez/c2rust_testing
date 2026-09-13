#!/usr/bin/env python3
"""Record process-entry defects unsupported by released function validators."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
RESULTS = ROOT / "results/baseline_artifacts/results.json"
AUDIT = "results/baseline_artifacts/setup/process_entry_capability_audit.json"

NOTES = {
    "C5": (
        "The defect is in process-argv acquisition/caller-side byte-to-str conversion before "
        "opng_os_create_dir. Direct function submission bypasses it; copying the conversion "
        "into an adapter would replace the artifact boundary with hand-written defect logic."
    ),
    "S13": (
        "The defect is in the no-argument Rust main wrapper's process-argument count. Calling "
        "main_0 with shared argc/argv bypasses it; copying args.len()-1 into an adapter would "
        "reimplement the defect."
    ),
}


def write_json(path: Path, value: object) -> None:
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    temporary.replace(path)


def main() -> None:
    data = json.loads(RESULTS.read_text())
    changed = 0
    for record in data["records"]:
        if record["baseline"] not in ("rustassure", "flourine"):
            continue
        defect = record["defect_id"]
        if defect not in NOTES:
            continue
        if record["outcome"] not in ("not_run", "unsupported_input"):
            raise SystemExit(
                f"refusing to replace {record['baseline']}/{defect} outcome "
                f"{record['outcome']}"
            )
        record.update(
            {
                "gates": {
                    "submitted": False,
                    "accepted": False,
                    "compiled": False,
                    "completed": False,
                    "detected": False,
                },
                "outcome": "unsupported_input",
                "adapter": None,
                "command_record": (
                    "not submitted: released interface cannot execute the original "
                    "process-entry semantic boundary"
                ),
                "result_record": AUDIT,
                "notes": [NOTES[defect]],
            }
        )
        changed += 1
    if changed != 4:
        raise SystemExit(f"expected four records, updated {changed}")
    write_json(RESULTS, data)
    print("recorded C5 and S13 as unsupported_input for RustAssure and FLOURINE")


if __name__ == "__main__":
    main()
