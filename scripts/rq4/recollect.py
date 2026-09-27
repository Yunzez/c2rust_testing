#!/usr/bin/env python3
"""Re-run the coverage collection of cell.py for chosen boundaries of a finished cell, optionally
over a snapshot corpus instead of the final one. Two uses:

  * a boundary whose collection failed for a reason since fixed (default_bzalloc: LSan failed
    every replay until collect() ran with detect_leaks=0);
  * the budget cross-check PROTOCOL.md section 3 asks for: coverage from the 300 s / 1800 s
    hard-linked snapshots of the SAME campaign, so a cell reports the hour and can report the
    five minutes without a second run.

Writes <cell>/ours<suffix>/<entry>.json. With --snapshot 300 the suffix is `@300s`, and the
corpus is <cell>/snapshots/<entry>@300s. Rebuilds the instrumented binary (one cargo build per
boundary), so run it after the cells, never beside one.

usage: scripts/rq4/recollect.py --cell <dir> --only a,b [--snapshot 300]
"""
from __future__ import annotations

import argparse
import json
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "scripts" / "rq4"))
import cell as CELL  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--only", required=True)
    ap.add_argument("--snapshot", type=int)
    a = ap.parse_args()
    cell = Path(a.cell)
    suffix = f"@{a.snapshot}s" if a.snapshot else ""
    out = cell / f"ours{suffix}"
    out.mkdir(exist_ok=True)
    target = cell / "target_recollect"
    modes = {}
    for b in [s.strip() for s in a.only.split(",")]:
        corpus = cell / "snapshots" / f"{b}{suffix}" if a.snapshot else cell / "corpus" / b
        if not corpus.is_dir() or not any(corpus.iterdir()):
            modes[b] = "no corpus"
            print(f"  {b:30s} no corpus at {corpus}", flush=True)
            continue
        m = CELL.collect(b, cell / "harnesses" / b, corpus, out / f"{b}.json", target)
        modes[b] = m
        print(f"  recollect {b:30s} {suffix or 'final':8s} {m}", flush=True)
        shutil.rmtree(cell / "harnesses" / b / "target", ignore_errors=True)
    shutil.rmtree(target, ignore_errors=True)
    log = cell / f"recollect{suffix}.json"
    prev = json.loads(log.read_text()) if log.exists() else {}
    prev.update(modes)
    log.write_text(json.dumps(prev, indent=1) + "\n")
    print("RECOLLECT_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
