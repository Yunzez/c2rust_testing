#!/usr/bin/env python3

"""Layer 3 — G3 evaluation: empirical false-divergence per boundary-selection strategy.

For each controlled, semantics-preserving case (ground truth: NO real bug, so any divergence is FALSE),
take each strategy's selected boundaries, differentially fuzz them, and count how many produce a
(false) divergence. The STU frontier should reach 0 false divergences while still covering the logic;
the baselines (leaf / all-constructible) should fuzz the unguarded helper and light up.

This is pure EVALUATION of the static selector (Layer 2) — dynamic fuzzing is the oracle here, it does
NOT participate in selection. Scope: controlled cases only (g3_case_*), where every divergence is false
by construction; real deep libs are NOT counted here (a divergence there could be a real c2rust bug).

Usage: DUR=20 python3 scripts/g3_eval.py [--cases g3_case_a,g3_case_c]
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import stu_frontier as S  # noqa: E402

# strategy display name -> (members key in analyze(), strat-count key in analyze())
STRATEGIES = [
    ("root", "root_members", "root"),
    ("all-constructible", "all_members", "all_constructible"),
    ("leaf-only", "leaf_members", "leaf"),
    ("frontier v1", "frontier_members", "frontier"),
    ("frontier v2", "frontier_v2_members", "frontier_v2"),
]


def main() -> int:
    ap = argparse.ArgumentParser(description="G3 false-divergence evaluation")
    ap.add_argument("--cases", default="g3_case_a,g3_case_c,g3_three_level")
    a = ap.parse_args()
    cases = a.cases.split(",")
    dur = os.environ.get("DUR", "20")

    # 1. static selection per case
    analyses = {}
    for c in cases:
        r = S.analyze(ROOT / "benchmark" / "pairs" / c)
        if r.get("error"):
            print(f"[skip] {c}: {r['error']}", file=sys.stderr)
            continue
        analyses[c] = r

    # 2. union of every boundary any strategy would fuzz, then differentially fuzz each ONCE
    tags = sorted({f"{c}__{fn}" for c, r in analyses.items()
                   for _, mkey, _ in STRATEGIES for fn in r[mkey]})
    with tempfile.TemporaryDirectory() as td:
        census = Path(td) / "census.jsonl"
        outcomes = Path(td) / "outcomes.jsonl"
        subprocess.run(["python3", str(ROOT / "scripts" / "harvest_census.py"),
                        "--out", str(census)], check=True, capture_output=True)
        env = {**os.environ, "DUR": dur}
        subprocess.run(["python3", str(ROOT / "scripts" / "harvest_stage_b.py"),
                        "--only", ",".join(tags), "--static", str(census), "--out", str(outcomes)],
                       check=True, env=env, capture_output=True)
        verdict = {}
        for l in outcomes.read_text().splitlines():
            if not l.strip():
                continue
            row = json.loads(l)
            verdict[(row["pair"], row["fn"])] = row.get("validity")

    # 3. per case x strategy: harness / covered / false-divergences
    INVALID = "invalid"
    md = ["# G3 evaluation — false divergences per boundary-selection strategy (Layer 3)\n",
          "Controlled semantics-preserving cases (any divergence is FALSE by construction). Each "
          "strategy's selected boundaries are differentially fuzzed; **false-div** = boundaries that "
          "produced a (false) divergence. Lower is better; the STU frontier should be 0 at good "
          "coverage. Dynamic is the ORACLE here, not the selector.\n"]
    for c, r in analyses.items():
        md += [f"## {c}  ({r['n_funcs']} funcs)\n",
               "| strategy | #harness | covered | **false-div** | boundaries |",
               "|---|--:|--:|--:|---|"]
        for disp, mkey, skey in STRATEGIES:
            fns = r[mkey]
            fdiv = [fn for fn in fns if verdict.get((c, fn)) == INVALID]
            cov = r[skey]["covered"]
            blist = ", ".join(f"`{fn}`" + ("⚠" if verdict.get((c, fn)) == INVALID else "")
                              for fn in fns) or "—"
            md.append(f"| {disp} | {len(fns)} | {cov} | **{len(fdiv)}** | {blist} |")
        md.append("")
    # summary
    md += ["## Reading\n"]
    for c, r in analyses.items():
        fr1 = sum(1 for fn in r["frontier_members"] if verdict.get((c, fn)) == INVALID)
        fr2 = sum(1 for fn in r["frontier_v2_members"] if verdict.get((c, fn)) == INVALID)
        leaf_fd = sum(1 for fn in r["leaf_members"] if verdict.get((c, fn)) == INVALID)
        allfd = sum(1 for fn in r["all_members"] if verdict.get((c, fn)) == INVALID)
        md.append(f"- **{c}**: leaf false-div {leaf_fd}, all-constructible {allfd}; frontier v1 "
                  f"{fr1} (covers {r['frontier']['covered']}), v2 {fr2} (covers "
                  f"{r['frontier_v2']['covered']}).")
    (ROOT / "results" / "g3_eval_v1.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("wrote results/g3_eval_v1.md\n")
    for c, r in analyses.items():
        print(f"=== {c} ===")
        for disp, mkey, skey in STRATEGIES:
            fns = r[mkey]
            fdiv = sum(1 for fn in fns if verdict.get((c, fn)) == INVALID)
            print(f"  {disp:18} harness={len(fns)} covered={r[skey]['covered']} false_div={fdiv}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
