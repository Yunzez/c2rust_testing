#!/usr/bin/env python3
"""Finish a cell that died AFTER its campaign (cell.py's coverage phase hit the scratchpad's
file-count quota on cJSON: 99 999 crash artifacts). Nothing is re-fuzzed: the corpus, snapshots,
candidates and harness binaries on disk are the campaign; this reconstructs what cell.py would have
written after them.

  1. candidates: a gzipped manifest (count + sha256 per boundary) is written, then every boundary
     keeps its first KEEP artifacts per channel (name order = the confirmation sample order); the
     TOTAL is what funnel.json reports, the manifest is what the archive keeps.
  2. funnel.json: planned boundaries from plans.json, built = a kept .bin, corpus/artifact counts.
  3. snapshots.json from the snapshot directories.
  4. coverage for every built boundary via cell.collect (the fixed one: timeout, detect_leaks=0).

usage: scripts/rq4/finish_cell.py --cell <dir> [--keep 500]
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "scripts" / "rq4"))
import cell as CELL  # noqa: E402

_PREFIX = re.compile(r"^(crash|oom|timeout|leak|slow-unit|divergence|panic|signal|nonzero-exit)-")


def prune_candidates(cands: Path, keep: int) -> dict:
    """Manifest first, then prune to `keep` per (boundary, channel). Idempotent."""
    manifest = {}
    if not cands.is_dir():
        return manifest
    for b in sorted(p for p in cands.iterdir() if p.is_dir()):
        files = sorted(p for p in b.iterdir() if p.is_file())
        by_chan: dict = {}
        for p in files:
            m = _PREFIX.match(p.name)
            by_chan.setdefault(m.group(1) if m else "other", []).append(p)
        manifest[b.name] = {"count": len(files),
                            "by_channel": {k: len(v) for k, v in by_chan.items()},
                            "sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in files}}
        for k, v in by_chan.items():
            for p in v[keep:]:
                p.unlink()
        manifest[b.name]["kept"] = sum(min(len(v), keep) for v in by_chan.values())
    with gzip.open(cands / "_manifest_full.json.gz", "wt") as fh:
        json.dump(manifest, fh)
    return manifest


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--keep", type=int, default=500)
    ap.add_argument("--seconds", type=int, default=3600, help="the campaign's budget, for campaign_params.json")
    ap.add_argument("--max-len", type=int, default=4096, help="the campaign's -max_len, for campaign_params.json")
    ap.add_argument("--defs", help="<translated>.rs.defs.json: which entries are C `static` "
                                   "(confirm_cell reads funnel.json's c_static to pass --expose-entry)")
    a = ap.parse_args()
    cell = Path(a.cell)
    plans = json.loads((cell / "plans.json").read_text())
    private = set(json.loads(Path(a.defs).read_text()).get("private", [])) if a.defs else set()
    manifest = prune_candidates(cell / "candidates", a.keep)
    print(f"candidates: {sum(m['count'] for m in manifest.values())} total, "
          f"{sum(m['kept'] for m in manifest.values())} kept", flush=True)

    rows = []
    for p in plans:
        if p.get("status") != "planned":
            continue
        b = p["boundary"]
        binp = cell / "harnesses" / f"{b}.bin"
        corpus = cell / "corpus" / b
        rows.append({"boundary": b, "c_static": b in private, "built": binp.exists(),
                     "error": None if binp.exists() else "no binary kept",
                     "inputs": len(p.get("inputs", [])),
                     "corpus": len([x for x in corpus.iterdir() if x.is_file()]) if corpus.is_dir() else 0,
                     "artifacts": manifest.get(b, {}).get("count", 0),
                     "artifacts_kept": manifest.get(b, {}).get("kept", 0)})
    snaps: dict = {}
    for d in sorted((cell / "snapshots").glob("*@*s")):
        e, cp = d.name.rsplit("@", 1)
        snaps.setdefault(cp[:-1], {})[e] = len(list(d.iterdir()))
    (cell / "snapshots.json").write_text(json.dumps(snaps, indent=1) + "\n")

    (cell / "ours").mkdir(exist_ok=True)
    target = cell / "target_finish"
    for r in rows:
        if not r["built"]:
            continue
        b = r["boundary"]
        if r["corpus"] == 0:
            r["coverage"] = "empty-corpus"
            continue
        if (cell / "ours" / f"{b}.json").exists() and (cell / "ours" / f"{b}.json").stat().st_size > 0:
            r["coverage"] = "kept"          # exported before the cell died; not recollected
            continue
        mode = CELL.collect(b, cell / "harnesses" / b, cell / "corpus" / b, cell / "ours" / f"{b}.json", target)
        r["coverage"] = mode
        print(f"  coverage {b:30s} {mode}", flush=True)
        for junk in ("target", "fuzz/target", "fuzz/coverage", "percov"):
            shutil.rmtree(cell / "harnesses" / b / junk, ignore_errors=True)
    shutil.rmtree(target, ignore_errors=True)
    (cell / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
    if not (cell / "campaign_params.json").exists():
        (cell / "campaign_params.json").write_text(json.dumps(
            {"mode": "rust-only", "fork": 1, "max_total_time_s": a.seconds, "seed": 42,
             "timeout_s": 25, "rss_limit_mb": 2048, "max_len": a.max_len,
             "ignore": ["crashes", "timeouts", "ooms"], "snapshots_s": [60, 300, 600, 1800],
             "note": "written by finish_cell.py (the cell died after its campaign)"}, indent=1) + "\n")
    print(f"wrote {cell/'funnel.json'} ({sum(1 for r in rows if r['built'])} built) and "
          f"{len(list((cell/'ours').glob('*.json')))} exports")
    print("FINISH_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
