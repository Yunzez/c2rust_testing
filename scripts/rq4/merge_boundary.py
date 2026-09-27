#!/usr/bin/env python3
"""Merge one boundary's re-run (cell.py --only <b> --out <tmp>) back into its cell.

Used when a HARNESS bug (not the translation) made one boundary's campaign void -- lil_parse's
length-0 buffer was a dangling pointer, so every execution SEGV'd in strlen. The boundary is
re-fuzzed alone under the same budget and libFuzzer parameters, then its corpus, candidates,
coverage export, binary and funnel row replace the cell's. The deviation (the re-run had the
machine to itself, unlike the cell's concurrent campaign) is recorded in the cell's
deviations.json for RUN.md.

usage: merge_boundary.py --cell <cell dir> --rerun <tmp cell dir> --boundary <name> --why "<text>"
"""
import argparse, json, shutil, time
from pathlib import Path

ap = argparse.ArgumentParser()
ap.add_argument("--cell", required=True); ap.add_argument("--rerun", required=True)
ap.add_argument("--boundary", required=True); ap.add_argument("--why", required=True)
a = ap.parse_args()
cell, rr, b = Path(a.cell), Path(a.rerun), a.boundary
for sub in ("corpus", "candidates", "harnesses"):
    src, dst = rr / sub / b, cell / sub / b
    if src.exists():
        shutil.rmtree(dst, ignore_errors=True); shutil.copytree(src, dst)
for f in (f"harnesses/{b}.bin", f"ours/{b}.json", f"candidates/{b}.fuzz.log"):
    if (rr / f).exists():
        (cell / f).parent.mkdir(exist_ok=True); shutil.copy(rr / f, cell / f)
for snap in (rr / "snapshots").glob(f"{b}@*"):
    dst = cell / "snapshots" / snap.name
    shutil.rmtree(dst, ignore_errors=True); shutil.copytree(snap, dst)
rows = json.loads((cell / "funnel.json").read_text())
new = [r for r in json.loads((rr / "funnel.json").read_text()) if r["boundary"] == b]
assert new, f"{b} not in the re-run's funnel"
rows = [new[0] if r["boundary"] == b else r for r in rows]
(cell / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
dev = cell / "deviations.json"
lst = json.loads(dev.read_text()) if dev.exists() else []
lst.append({"boundary": b, "kind": "boundary_rerun_after_harness_fix", "why": a.why,
            "generator_rerun": new[0].get("generator", "unrecorded"),
            "generator_cell": next((r.get("generator", "unrecorded") for r in rows if r["boundary"] != b), "unrecorded"),
            "when": time.strftime("%Y-%m-%d %H:%M"), "rerun_params": json.loads((rr / "campaign_params.json").read_text()) if (rr / "campaign_params.json").exists() else None})
dev.write_text(json.dumps(lst, indent=1) + "\n")
print(f"merged {b}: corpus {new[0]['corpus']}, artifacts {new[0].get('artifacts')}, coverage {new[0].get('coverage')}")
