#!/usr/bin/env python3
"""Rebuild a finished cell's DISCOVERY binaries with the current generator, keeping its campaign.

Used when a generator fix changes only what the harness REPORTS (the producer-stage UB gate), not
what it fuzzes: the corpus, snapshots and candidates stay as the campaign produced them; replay and
confirmation are then re-run against binaries every one of which came from one generator version.

usage: scripts/rq4/rebuild_bins.py --cell <dir> --pair <dir> --c-source X.c [--shim F] [--plugins T] [--defs J]
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
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--pair", required=True)
    ap.add_argument("--plugins", action="append")
    ap.add_argument("--c-source")
    ap.add_argument("--shim")
    ap.add_argument("--defs")
    a = ap.parse_args()
    cell, pair = Path(a.cell), Path(a.pair)
    funnel = json.loads((cell / "funnel.json").read_text())
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    private = set(defs.get("private", []))
    target = cell / "target_rebuild"
    ok = 0
    for row in funnel:
        if not row.get("built"):
            continue
        b = row["boundary"]
        binp, err = CELL.build_one(a, pair, b, b in private, cell / "harnesses" / b, target)
        print(f"  rebuild {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}", flush=True)
        ok += bool(binp)
        for stale in (cell / "harnesses" / f"{b}_san.bin", cell / "harnesses" / f"{b}_nosan.bin"):
            stale.unlink(missing_ok=True)      # confirmation rebuilds these with the same generator
    shutil.rmtree(target, ignore_errors=True)
    print(f"rebuilt {ok} binaries")
    print("REBUILD_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
