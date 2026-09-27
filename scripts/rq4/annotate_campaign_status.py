#!/usr/bin/env python3
"""Add each boundary's fork-mode campaign status (jobs / crashes / corpus, crash_all) to
funnel.json from the campaign's own libFuzzer logs. cell.py records this itself since the
preflight change; this backfills cells that ran before it (idempotent).

A `crash_all` boundary (>= 90 % of jobs crashed, corpus never grew) is one whose harness -- or
the translation -- fails on essentially every input: its coverage export fails too, and the
funnel must say why rather than show `failed rc=1`.

usage: annotate_campaign_status.py --cell <dir>
"""
import argparse, json, sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from cell import fuzz_status  # noqa: E402

ap = argparse.ArgumentParser(); ap.add_argument("--cell", required=True); a = ap.parse_args()
cell = Path(a.cell); rows = json.loads((cell / "funnel.json").read_text())
n = 0
for r in rows:
    st = fuzz_status(cell / "candidates" / f"{r['boundary']}.fuzz.log")
    if st:
        r["campaign_status"] = st; n += 1
        if st["crash_all"] and str(r.get("coverage", "")).startswith("failed"):
            r["coverage_note"] = "crash-all boundary: every execution of the campaign crashed, so no profile could be written"
(cell / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
ca = [r["boundary"] for r in rows if r.get("campaign_status", {}).get("crash_all")]
print(f"{cell.name}: {n} rows annotated; crash-all: {len(ca)} {ca}")
