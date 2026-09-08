#!/usr/bin/env python3
"""Is a cell's archive complete enough to delete its scratch directory?

Written 2026-09-08 after three deletions that each broke something silently: a confirmation that
never ran (the post script's confirm line had been given an argument it rejects), a coverage
analysis that failed because the denominator build's `src/lib.rs` had been pruned, and a cell whose
ours-side exports could no longer be line-aligned because the cell itself was gone. The pipeline
reports none of these as errors -- a `&&` chain simply prints one line less.

usage: scripts/rq4/check_cell.py <lib>/<tool> [...]      (no arguments = every archived cell)
exit 0 = every cell listed is complete; exit 1 = at least one is not.
"""
from __future__ import annotations
import json, sys, glob
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RQ4 = ROOT / "results" / "rq3_coverage"


def check(cell: Path) -> list[str]:
    bad = []
    r = cell / "analysis" / "result.json"
    if not r.exists():
        bad.append("no analysis/result.json (the coverage step failed or never ran)")
    else:
        d = json.loads(r.read_text())
        if d["region"]["covered_ours"] == 0 and d["function"]["covered_ours"] == 0:
            bad.append("analysis says 0 covered -- the ours side contributed nothing")
        if not all(d["region"].get("sanity", {}).values()):
            bad.append("analysis sanity checks fail")
    # how many candidates the campaign actually produced: an empty tally is CORRECT when there was
    # nothing to adjudicate, and wrong only when there was.
    ncand = 0
    m = cell / "candidates_manifest.json.gz"
    if m.exists():
        import gzip
        try:
            man = json.loads(gzip.open(m, "rt").read())
            # {boundary: {"count": n, "sha256": [...]}}
            ncand = sum(int(v.get("count", len(v.get("sha256", [])))) if isinstance(v, dict) else len(v)
                        for v in man.values()) if isinstance(man, dict) else len(man)
        except Exception:
            ncand = 0
    ncand += len([f for f in cell.glob("divergences/*/*") if f.suffix != ".json"])
    s = list(cell.glob("confirm*/summary.json"))
    if not s:
        if ncand:
            bad.append(f"no confirm*/summary.json and {ncand} candidates exist (confirmation never ran)")
    else:
        tot = json.loads(s[0].read_text()).get("total") or {}
        if not tot and ncand:
            bad.append(f"confirmation tally is empty although {ncand} candidates exist")
    c = cell / "corpus.tar.gz"
    if not c.exists() or c.stat().st_size < 1024:
        bad.append("corpus.tar.gz missing or empty (the campaign never ran, or tar failed)")
    for f in ("funnel.json", "plans.json", "artifact_hashes.json", "harness_exports.tar.gz"):
        if not (cell / f).exists():
            bad.append(f"missing {f}")
    return bad


def main() -> int:
    names = sys.argv[1:]
    if not names:
        # the two retired bzip2 artifacts are kept for provenance and are not cells
        skip = {"bzip2/c2rust_diagnostic_pilot", "bzip2/c2rust_handschema_superseded"}
        names = [n for n in sorted(str(Path(p).parent.relative_to(RQ4)) for p in glob.glob(str(RQ4 / "*/*/RUN.md")))
                 if n not in skip]
    worst = 0
    for n in names:
        cell = RQ4 / n
        if not cell.is_dir():
            print(f"  ?? {n}: no such archive"); worst = 1; continue
        bad = check(cell)
        print(f"  {'OK ' if not bad else 'BAD'} {n}" + ("" if not bad else "\n      - " + "\n      - ".join(bad)))
        worst = max(worst, 1 if bad else 0)
    return worst


if __name__ == "__main__":
    raise SystemExit(main())
