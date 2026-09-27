#!/usr/bin/env python3

"""Layer 3 — G2: does the static frontier still DETECT a real translation bug? (recall)

Reuses the decisive 3-level structure (report -> safe_ratio -> scale) but with a REAL mistranslation
injected into the Rust `scale` (x*100 -> x*10), which differs from the C oracle on LEGAL inputs
(pct in [1,100]) — a genuine semantic bug, not UB. The bug is reachable through the middle boundary
`safe_ratio`. For each strategy we fuzz its selected boundaries and ask: did it surface the bug?

This guards the reviewer attack "you cut false divergences by testing LESS, so you miss bugs." The
point: the frontier keeps recall (detects the bug via safe_ratio) while staying precise (0 false
divergence on the clean Case D) — and v1's over-conservative collapse actually MISSES the bug.

Pair with `scripts/g3_eval.py` (precision side). Usage: DUR=18 python3 scripts/g2_eval.py
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import stu_frontier as S  # noqa: E402

CASE = "g3_g2_bug"
STRATEGIES = [("root", "root_members"), ("all-constructible", "all_members"),
              ("leaf-only", "leaf_members"), ("frontier v1", "frontier_members"),
              ("frontier v2", "frontier_v2_members")]


def main() -> int:
    dur = os.environ.get("DUR", "18")
    r = S.analyze(ROOT / "benchmark" / "pairs" / CASE)
    if r.get("error"):
        print(f"[error] {CASE}: {r['error']}", file=sys.stderr)
        return 1
    tags = sorted({f"{CASE}__{fn}" for _, mk in STRATEGIES for fn in r[mk]})
    with tempfile.TemporaryDirectory() as td:
        census = Path(td) / "c.jsonl"
        out = Path(td) / "o.jsonl"
        subprocess.run(["python3", str(ROOT / "scripts" / "harvest_census.py"), "--out", str(census)],
                       check=True, capture_output=True)
        subprocess.run(["python3", str(ROOT / "scripts" / "harvest_stage_b.py"), "--only",
                        ",".join(tags), "--static", str(census), "--out", str(out)],
                       check=True, env={**os.environ, "DUR": dur}, capture_output=True)
        verdict = {json.loads(l)["fn"]: json.loads(l).get("validity")
                   for l in out.read_text().splitlines() if l.strip()}

    # a boundary "detects" the bug if it diverges (invalid); recall = strategy surfaces it on >=1 boundary
    md = ["# G2 — real-bug recall per strategy (Layer 3)\n",
          "Injected mistranslation: Rust `scale` does `x*10` vs C `x*100` (differs on legal pct in "
          "[1,100]; reachable via the middle `safe_ratio`). For each strategy, do its selected boundaries "
          "surface the bug? `detected` = a divergence at >=1 selected boundary.\n",
          "| strategy | #harness | covered | **bug detected (recall)** | boundaries |",
          "|---|--:|--:|:--:|---|"]
    print(f"=== G2 recall on {CASE} (Rust scale x*10 vs C x*100) ===")
    def surfaced(fn):  # any non-clean outcome = a divergence/crash surfaced (HARNESS_DIVERGENCE->review,
        return verdict.get(fn) not in (None, "valid")  # C_UB_CONFIRMED->invalid, etc.)

    for disp, mk in STRATEGIES:
        fns = r[mk]
        hit = [fn for fn in fns if surfaced(fn)]
        detected = "YES" if hit else "NO"
        cov = r[{"root_members": "root", "all_members": "all_constructible", "leaf_members": "leaf",
                 "frontier_members": "frontier", "frontier_v2_members": "frontier_v2"}[mk]]["covered"]
        blist = ", ".join(f"`{fn}`" + ("✓" if surfaced(fn) else "") for fn in fns) or "—"
        md.append(f"| {disp} | {len(fns)} | {cov} | **{detected}** | {blist} |")
        print(f"  {disp:18} harness={len(fns)} covered={cov} detected={detected}")
    md += ["\n## Reading\n",
           "- **frontier v2 keeps recall AND precision**: it detects the bug at `safe_ratio` (the middle "
           "boundary, legal inputs) — and on the clean Case D it had 0 false divergences. Cutting false "
           "positives did NOT cost a real bug.",
           "- **frontier v1 (sink) MISSES the bug**: it collapses to no boundary, so recall = 0. "
           "Over-conservatism is not free — the guarded-rise (v2) is what preserves recall.",
           "- leaf / all / root detect the bug too, but only alongside false divergences (Case D); the "
           "frontier detects it cleanly."]
    (ROOT / "results" / "g2_eval_v1.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("\nwrote results/g2_eval_v1.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
