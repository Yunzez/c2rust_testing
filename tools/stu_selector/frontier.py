#!/usr/bin/env python3

"""Stage 4 (v1) — STU frontier selection + validation against `// ENTRY` ground truth.

This is the *interpretable baseline* selector (spec docs/stu_selection.md §6: a learned
`P(valid harness | x_f)` model comes later; for now we use transparent gates + a score so we
can sanity-check that STU recognition works on the controlled c2rust single-file case).

The selection axis here is **harness validity**, not structural divergence (c2rust single-TU is
1:1, so the structural axis is trivial — see results/feature_study_v1.md v2):

  - fuzzability  : can the entry inputs be synthesized by a fuzzer?
  - comparability: can the outputs be compared (scalar easy; pointer/raw output harder)?
  - certainty    : is the boundary free of unresolved indirect dispatch?

HARD gate: a function whose signature takes a **function pointer** is not a standalone STU —
it cannot be fuzzed without first binding the callback. Such programs should be flagged, not
silently fuzzed at a bogus boundary.

Validation: each benchmark C file carries a `// ENTRY: <signature>` comment = the human-intended
STU. We check whether the selected frontier recovers it, and explain every disagreement.

Usage:
  python3 tools/stu_selector/frontier.py --pairs benchmark/pairs --raw benchmark/raw
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import mapping as mapmod  # noqa: E402
import features as feat  # noqa: E402

VALIDITY_THRESHOLD = 0.5

# Match ENTRY: in any comment style (// ENTRY, /* ENTRY, * ENTRY).
ENTRY_RE = re.compile(r"ENTRY:\s*(.+)")
# Pull the function identifier out of a C signature "<ret> name(args)": the name is the
# FIRST identifier immediately followed by '(' (the return type may also precede a '(' in
# function-pointer params, so we take the first match, not the last).
NAME_RE = re.compile(r"([A-Za-z_]\w*)\s*\(")


def parse_entry(c_file: Path) -> str | None:
    for line in c_file.read_text(encoding="utf-8", errors="ignore").splitlines():
        m = ENTRY_RE.search(line)
        if m:
            names = NAME_RE.findall(m.group(1))
            if names:
                return names[0]
    return None


def score_row(r: dict) -> dict:
    """Score a function as a differential-fuzz STU boundary.

    Distinction that matters (learned the hard way on this corpus): cost should RANK
    candidates, not GATE them. Only a genuine blocker excludes a boundary:

      HARD gate — function-pointer parameter: a fuzzer cannot synthesize a function value,
      so the boundary is not fuzzable until the callback is bound. Exclude it.

    Everything else (nested-pointer inputs, pointer outputs, internal indirect dispatch) is a
    synthesis/alignment COST that lowers a confidence score but does NOT exclude the boundary —
    those inputs can be constructed by a structured generator and internal dispatch is inside
    the black box.
    """
    hard_block = bool(int(r["has_fn_pointer_param"]))

    confidence = 1.0
    confidence -= 0.15 * int(r["n_pointer_params"])
    confidence -= 0.30 * int(r["n_nested_pointer_params"])
    confidence -= 0.30 * int(r["returns_pointer"])
    confidence -= 0.20 * int(r["c_indirect_calls"])
    confidence = max(0.0, round(confidence, 3))

    reasons = []
    if hard_block:
        reasons.append("fn-pointer param (needs callback bound)")
    if int(r["n_nested_pointer_params"]):
        reasons.append("nested-pointer input (costly to synthesize)")
    if int(r["returns_pointer"]):
        reasons.append("returns pointer (output needs normalization)")
    if int(r["c_indirect_calls"]):
        reasons.append(f"{int(r['c_indirect_calls'])} internal indirect call(s)")

    return {
        "fn": r["fn"],
        "confidence": 0.0 if hard_block else confidence,
        "coverage": int(r["c_nodes"]),  # logic covered if we fuzz at this boundary
        "is_candidate": not hard_block,
        "reasons": reasons,
    }


def _ancestors(target: str, redj: dict[str, list[str]]) -> set[str]:
    """All functions that can reach `target` along caller->callee edges."""
    seen: set[str] = set()
    stack = list(redj.get(target, []))
    while stack:
        n = stack.pop()
        if n in seen:
            continue
        seen.add(n)
        stack.extend(redj.get(n, []))
    return seen


def select_frontier(rows: list[dict], edges: list[dict]) -> dict:
    """Frontier = call-graph-highest valid boundaries (a risk-bounded antichain).

    A valid candidate is on the frontier iff none of its transitive *callers* is also a
    valid candidate — i.e. it is the entry-most fuzzable boundary, covering the most logic
    without being dominated by a higher valid boundary.
    """
    scored = [score_row(r) for r in rows]
    valid = {s["fn"] for s in scored if s["is_candidate"]}

    # reverse adjacency: callee -> [callers]
    redj: dict[str, list[str]] = {}
    for e in edges:
        redj.setdefault(e["to"], []).append(e["from"])

    frontier = []
    for s in scored:
        if not s["is_candidate"]:
            continue
        anc = _ancestors(s["fn"], redj)
        if not (anc & (valid - {s["fn"]})):
            frontier.append(s)
    frontier.sort(key=lambda s: (-s["coverage"], s["fn"]))
    return {"scored": scored, "frontier": frontier}


def main() -> int:
    ap = argparse.ArgumentParser(description="STU frontier selection + ENTRY validation")
    ap.add_argument("--pairs", required=True)
    ap.add_argument("--raw", required=True, help="benchmark/raw (for // ENTRY ground truth)")
    ap.add_argument("--rust-bin", default=str(mapmod._DEFAULT_RUST_BIN))
    args = ap.parse_args()

    pairs_dir = Path(args.pairs)
    raw_dir = Path(args.raw)
    rust_bin = Path(args.rust_bin)

    entry_by_name = {}
    for c_file in raw_dir.rglob("*.c"):
        entry_by_name[c_file.stem] = parse_entry(c_file)

    hits = misses = no_gt = 0
    print(f"{'program':22} {'ground-truth STU':24} {'selected':24} {'result'}")
    print("-" * 86)
    for pair in sorted(p for p in pairs_dir.iterdir() if p.is_dir() and not p.name.startswith("_")):
        cc = pair / "build"
        rs = next((pair / "translated").glob("*.rs"), None)
        if not cc.exists() or rs is None:
            continue
        try:
            rows = feat.features_for_pair(cc, rs, rust_bin, pair.name)
            edges = mapmod.build_c_graph(cc)["edges"]
        except Exception as e:  # noqa: BLE001
            print(f"{pair.name:22} [feature error: {e}]")
            continue
        sel = select_frontier(rows, edges)
        front_names = [s["fn"] for s in sel["frontier"]]
        gt = entry_by_name.get(pair.name)
        chosen = ", ".join(front_names) if front_names else "(none — no fuzzable STU)"

        if gt is None:
            verdict = "no ground truth"
            no_gt += 1
        elif gt in front_names:
            verdict = "HIT"
            hits += 1
        else:
            gt_score = next((s for s in sel["scored"] if s["fn"] == gt), None)
            if gt_score is None:
                why = "entry not in matched set"
            elif gt_score["reasons"]:
                why = "; ".join(gt_score["reasons"])
            else:
                why = "dominated by higher valid boundary"
            verdict = f"MISS ({why})"
            misses += 1
        print(f"{pair.name:22} {str(gt):24} {chosen:28} {verdict}")

    total = hits + misses
    print("-" * 86)
    print(f"STU recognition: {hits}/{total} entries recovered"
          + (f"  ({no_gt} without ground truth)" if no_gt else ""))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
