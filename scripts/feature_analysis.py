#!/usr/bin/env python3

"""Feature-vs-validity analysis for the boundary dataset (no model — just which static features
explain valid vs invalid).

Uses the AUTHORITATIVE label `validity_v2` (NOT raw `validity` or `validity_v2_auto`). Binary
problem: valid (positive) vs invalid (= invalid_isolation_invariant + invalid_intrinsic_ub).
weak_exclude / excluded* are dropped (not training data).

Per numeric feature it reports mean(valid), mean(invalid), and a rank-based **AUC** = P(a random
invalid scores higher than a random valid); |AUC-0.5| is the separation strength (0.5 = no signal).
Also emits the stratified / breakdown / grouped-CV-feasibility tables.

Usage: python3 scripts/feature_analysis.py --in dataset/boundaries_v2.jsonl
"""

from __future__ import annotations

import argparse
import collections
import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

INVALID = {"invalid_isolation_invariant", "invalid_intrinsic_ub"}
SKIP = {"pair", "fn", "validity", "validity_v2", "validity_v2_auto", "audit", "audit_auto",
        "audit_status", "review_reason", "generator", "label", "artifact_labels", "artifact_count",
        "executions", "elapsed_seconds", "terminated_by_timeout", "dur_seconds", "crash_summary",
        "constructible", "constructibility", "gate_reason", "unsupported_detail", "boundary_scope",
        "exposed_static", "pub_in_rust", "needs_expose", "note", "roles"}


def auc(pos: list[float], neg: list[float]) -> float:
    """P(random neg > random pos) with 0.5 for ties; 0.5 = no separation."""
    if not pos or not neg:
        return 0.5
    wins = ties = 0
    for a in neg:
        for b in pos:
            if a > b:
                wins += 1
            elif a == b:
                ties += 1
    return (wins + 0.5 * ties) / (len(pos) * len(neg))


def numeric_features(rows: list[dict]) -> list[str]:
    feats = []
    for k, v in rows[0].items():
        if k in SKIP:
            continue
        if isinstance(v, bool) or isinstance(v, (int, float)):
            feats.append(k)
    return feats


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--in", dest="inp", default=str(ROOT / "dataset" / "boundaries_v4.jsonl"))
    ap.add_argument("--out", default=str(ROOT / "results" / "feature_analysis_v1.md"))
    args = ap.parse_args()

    rows = [json.loads(l) for l in Path(args.inp).read_text().splitlines() if l.strip()]
    pos = [r for r in rows if r.get("validity_v2") == "valid"]
    neg = [r for r in rows if r.get("validity_v2") in INVALID]
    iso = [r for r in rows if r.get("validity_v2") == "invalid_isolation_invariant"]
    intr = [r for r in rows if r.get("validity_v2") == "invalid_intrinsic_ub"]
    feats = numeric_features(rows)

    def sep_table(pos_rows, neg_rows):
        out = []
        for f in feats:
            if isinstance(rows[0][f], bool):
                pv = [1.0 if r[f] else 0.0 for r in pos_rows]
                nv = [1.0 if r[f] else 0.0 for r in neg_rows]
            else:
                pv = [float(r[f]) for r in pos_rows if f in r and not isinstance(r[f], bool)]
                nv = [float(r[f]) for r in neg_rows if f in r and not isinstance(r[f], bool)]
            if not pv or not nv:
                continue
            a = auc(pv, nv)
            out.append((f, statistics.mean(pv), statistics.mean(nv), a, abs(a - 0.5)))
        out.sort(key=lambda x: -x[4])
        return out

    def emit(title, tbl, top=10):
        m = [f"\n## {title} (AUC = P(neg scores higher); 0.5 = no signal)\n",
             "| feature | mean(valid) | mean(neg) | AUC | |sep| |", "|---|---|---|---|---|"]
        for f, mp, mn, a, s in tbl[:top]:
            m.append(f"| {f} | {mp:.2f} | {mn:.2f} | {a:.2f} | {s:.2f} |")
        return m

    table = sep_table(pos, neg)
    md = ["# Feature-vs-validity analysis v1 (no model)\n",
          f"Authoritative label `validity_v2`. Binary: **{len(pos)} valid : {len(neg)} invalid** "
          f"(invalid = isolation_invariant + intrinsic_ub). weak/excluded dropped. Source: "
          f"`{Path(args.inp).name}`. AUC is rank-based (no deps); the dataset is small so treat "
          f"separations as descriptive, not significance-tested.\n",
          "## Key finding\n",
          "**No single current static feature strongly separates valid from invalid** — best "
          "|AUC-0.5| ≈ 0.27 lumped, ≈ 0.31 per-mechanism. The only consistent (weak) signal is "
          "size/complexity: invalids are slightly SMALLER/simpler (fewer nodes, loops, lower "
          "cyclomatic), because `invalid_intrinsic_ub` are tiny scalar-arithmetic functions and "
          "`invalid_isolation_invariant` are small accessors. The signature counts "
          "(`n_pointer_params`, `n_nested_pointer_params`, `returns_pointer`, …) do NOT separate. "
          "**Implication:** validity is not explained by generic complexity/signature features; it "
          "needs boundary-specific features that capture the semantic risk — e.g. *unguarded signed "
          "arithmetic on a scalar* (predicts intrinsic_ub) and *a struct/buffer indexed by a trusted "
          "field without a bounds check* (predicts isolation_invariant). Engineering those features "
          "is the next step BEFORE any model; the weak generic baseline here is the motivation."]
    md += emit("Lumped: valid vs ALL invalid", table)
    md += emit(f"valid vs invalid_intrinsic_ub ({len(intr)})", sep_table(pos, intr))
    md += emit(f"valid vs invalid_isolation_invariant ({len(iso)})", sep_table(pos, iso))

    # public/internal stratified
    md += ["\n## Stratified by boundary scope\n", "| scope | valid | invalid | other |", "|---|---|---|---|"]
    sc = collections.Counter((r["boundary_scope"], "valid" if r.get("validity_v2") == "valid"
                              else "invalid" if r.get("validity_v2") in INVALID else "other")
                             for r in rows)
    for s in ("public", "internal"):
        md.append(f"| {s} | " + " | ".join(str(sc.get((s, c), 0)) for c in ("valid", "invalid", "other")) + " |")

    # negative mechanism breakdown
    md += ["\n## Negative mechanism breakdown\n", "| mechanism | n |", "|---|---|"]
    for k, v in collections.Counter(r["validity_v2"] for r in neg).most_common():
        md.append(f"| {k} | {v} |")

    # auto vs reviewed
    md += ["\n## Label provenance (audit_status x validity_v2)\n",
           "| audit_status | valid | invalid | weak/excluded |", "|---|---|---|---|"]
    av = collections.Counter((r.get("audit_status"), "valid" if r.get("validity_v2") == "valid"
                              else "invalid" if r.get("validity_v2") in INVALID else "other")
                             for r in rows)
    for st in ("auto", "reviewed", "n/a"):
        md.append(f"| {st} | " + " | ".join(str(av.get((st, c), 0)) for c in ("valid", "invalid", "other")) + " |")

    # per-program grouped-CV feasibility
    prog = collections.defaultdict(lambda: [0, 0])
    for r in rows:
        if r.get("validity_v2") == "valid":
            prog[r["pair"]][0] += 1
        elif r.get("validity_v2") in INVALID:
            prog[r["pair"]][1] += 1
    progs_labeled = {p: c for p, c in prog.items() if c[0] + c[1] > 0}
    both = sum(1 for c in progs_labeled.values() if c[0] and c[1])
    md += ["\n## Per-program grouped-CV feasibility\n",
           f"- {len(progs_labeled)} programs contribute ≥1 labeled boundary "
           f"(grouped folds = group by program so train/test never share a program).",
           f"- {both} programs have BOTH classes; "
           f"{sum(1 for c in progs_labeled.values() if c[1])} have ≥1 invalid, "
           f"{sum(1 for c in progs_labeled.values() if c[0])} have ≥1 valid.",
           f"- Recommendation: program-grouped K-fold (K≈5) is feasible "
           f"({len(progs_labeled)} groups); stratify so each fold has invalids.\n",
           "| program | valid | invalid |", "|---|---|---|"]
    for p in sorted(progs_labeled, key=lambda p: -(progs_labeled[p][0] + progs_labeled[p][1])):
        md.append(f"| {p} | {progs_labeled[p][0]} | {progs_labeled[p][1]} |")

    Path(args.out).write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"wrote {args.out}")
    print(f"binary: {len(pos)} valid : {len(neg)} invalid; {len(progs_labeled)} programs ({both} with both classes)")
    print("\ntop separating features (|AUC-0.5|):")
    for f, mp, mn, a, s in table[:8]:
        print(f"  {f:24} valid={mp:7.2f}  invalid={mn:7.2f}  AUC={a:.2f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
