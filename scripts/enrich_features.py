#!/usr/bin/env python3

"""Attach boundary-specific semantic-risk features (rf_*) to each row of a boundary dataset.

Reads dataset/boundaries_v2.jsonl, computes risk_features per (pair, fn), and writes the enriched
rows back (idempotent). rf_* columns sit alongside the generic c_*/r_*/n_* features so the analysis
can compare generic vs boundary-specific vs combined.

Usage: python3 scripts/enrich_features.py --in dataset/boundaries_v2.jsonl --out dataset/boundaries_v2.jsonl
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import risk_features as rf  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--in", dest="inp", default=str(ROOT / "dataset" / "boundaries_v4.jsonl"))
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    out = a.out or a.inp

    rows = [json.loads(l) for l in Path(a.inp).read_text().splitlines() if l.strip()]
    cache: dict[str, dict] = {}
    n_feat = 0
    for r in rows:
        pair = r["pair"]
        if pair not in cache:
            try:
                cache[pair] = rf.risk_features_for_pair(ROOT / "benchmark" / "pairs" / pair / "build")
            except Exception as e:  # noqa: BLE001
                print(f"[warn] {pair}: {e}", file=sys.stderr)
                cache[pair] = {}
        feats = cache[pair].get(r["fn"])
        if feats:
            r.update(feats)
            n_feat = len(feats)
        else:
            print(f"[warn] no risk features for {pair}__{r['fn']}", file=sys.stderr)

    Path(out).write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")
    print(f"enriched {len(rows)} rows with {n_feat} rf_* features -> {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
