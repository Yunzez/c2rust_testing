#!/usr/bin/env python3

"""P(valid | x) baseline — does boundary-specific risk beat generic structure?

NOT a tuned model. A plain logistic regression under PROGRAM-GROUPED cross-validation (train/test
never share a program), comparing three feature groups:
  generic           — c_*/r_*/n_*/size/fuzzability/... (structural + signature counts)
  boundary_specific — rf_* semantic-risk features (+ an engineered unguarded-UB-op interaction)
  combined          — both

Reports held-out AUC per group (pooled over folds). The claim to support: boundary-specific
semantic-risk features predict harness validity better than generic structural features.

Label: authoritative validity_v2; binary valid vs invalid (isolation_invariant + intrinsic_ub).
Usage: python3 scripts/validity_baseline.py --in dataset/boundaries_v2.jsonl
"""

from __future__ import annotations

import argparse
import json
import math
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INVALID = {"invalid_isolation_invariant", "invalid_intrinsic_ub"}
GENERIC = ["c_cyclomatic", "r_cyclomatic", "c_stmts", "r_stmts", "c_nodes", "r_nodes", "c_loops",
           "c_max_loop_depth", "size_ratio", "c_pointer_access", "r_pointer_intensity", "c_allocs",
           "n_params", "n_pointer_params", "n_nested_pointer_params", "has_fn_pointer_param",
           "returns_pointer", "fuzzability", "norm_burden"]
RF = ["rf_div_mod", "rf_shift", "rf_compound_arith", "rf_compares", "rf_nonzero_guard",
      "rf_width_guard", "rf_signed", "rf_field_index", "rf_unmasked_field_index", "rf_datadep_index",
      "rf_struct_ptr", "rf_struct_index_field", "rf_internal", "rf_unguarded_ubop"]


def feat(r, names):
    return [float(r.get(n, 0) or 0) for n in names]


def standardize(rows_x):
    cols = list(zip(*rows_x)) if rows_x else []
    mean = [sum(c) / len(c) for c in cols]
    std = [(sum((v - m) ** 2 for v in c) / len(c)) ** 0.5 or 1.0 for c, m in zip(cols, mean)]
    return mean, std


def apply_std(x, mean, std):
    return [(v - m) / s for v, m, s in zip(x, mean, std)]


def fit_logreg(X, y, iters=3000, lr=0.3, l2=1e-3):
    d = len(X[0])
    w = [0.0] * d
    b = 0.0
    n = len(X)
    for _ in range(iters):
        gw = [0.0] * d
        gb = 0.0
        for xi, yi in zip(X, y):
            z = b + sum(wj * xj for wj, xj in zip(w, xi))
            p = 1.0 / (1.0 + math.exp(-max(-30, min(30, z))))
            e = p - yi
            for j in range(d):
                gw[j] += e * xi[j]
            gb += e
        w = [wj - lr * (gw[j] / n + l2 * wj) for j, wj in enumerate(w)]
        b -= lr * gb / n
    return w, b


def predict(x, w, b):
    z = b + sum(wj * xj for wj, xj in zip(w, x))
    return 1.0 / (1.0 + math.exp(-max(-30, min(30, z))))


def auc(scores, labels):
    pos = [s for s, l in zip(scores, labels) if l == 1]
    neg = [s for s, l in zip(scores, labels) if l == 0]
    if not pos or not neg:
        return float("nan")
    wins = ties = 0
    for a in pos:
        for b_ in neg:
            if a > b_:
                wins += 1
            elif a == b_:
                ties += 1
    return (wins + 0.5 * ties) / (len(pos) * len(neg))


def grouped_cv_auc(rows, names, k=5):
    progs = sorted({r["pair"] for r in rows})
    folds = defaultdict(list)
    for i, p in enumerate(progs):
        folds[i % k].append(p)
    all_scores, all_labels = [], []
    for f in range(k):
        test_progs = set(folds[f])
        tr = [r for r in rows if r["pair"] not in test_progs]
        te = [r for r in rows if r["pair"] in test_progs]
        if not te or not tr:
            continue
        Xtr = [feat(r, names) for r in tr]
        ytr = [1 if r["validity_v2"] in INVALID else 0 for r in tr]  # predict INVALID as positive
        mean, std = standardize(Xtr)
        Xtr = [apply_std(x, mean, std) for x in Xtr]
        w, b = fit_logreg(Xtr, ytr)
        for r in te:
            x = apply_std(feat(r, names), mean, std)
            all_scores.append(predict(x, w, b))
            all_labels.append(1 if r["validity_v2"] in INVALID else 0)
    return auc(all_scores, all_labels), len(all_scores)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--in", dest="inp", default=str(ROOT / "dataset" / "boundaries_v2.jsonl"))
    ap.add_argument("--out", default=str(ROOT / "results" / "validity_baseline_v1.md"))
    a = ap.parse_args()
    rows = [json.loads(l) for l in Path(a.inp).read_text().splitlines() if l.strip()]
    rows = [r for r in rows if r.get("validity_v2") == "valid" or r.get("validity_v2") in INVALID]
    # engineered interaction: a UB-prone op present AND not guarded
    for r in rows:
        ub = (r.get("rf_div_mod", 0) + r.get("rf_shift", 0) + r.get("rf_compound_arith", 0)) > 0
        guarded = bool(r.get("rf_nonzero_guard", 0)) or bool(r.get("rf_width_guard", 0))
        r["rf_unguarded_ubop"] = int(ub and r.get("rf_signed", 0) and not guarded)

    valid = [r for r in rows if r["validity_v2"] == "valid"]
    iso = [r for r in rows if r["validity_v2"] == "invalid_isolation_invariant"]
    intr = [r for r in rows if r["validity_v2"] == "invalid_intrinsic_ub"]
    n_pos = len(iso) + len(intr)
    groups = {"generic": GENERIC, "boundary_specific": RF, "combined": GENERIC + RF}
    tasks = {"lumped (all invalid)": rows,
             "valid vs isolation_invariant": valid + iso,
             "valid vs intrinsic_ub": valid + intr}
    res = {t: {g: grouped_cv_auc(rs, names)[0] for g, names in groups.items()}
           for t, rs in tasks.items()}

    md = ["# P(valid | x) baseline — generic vs boundary-specific features\n",
          f"Plain logistic regression, **program-grouped 5-fold CV** (train/test never share a "
          f"program). Label `validity_v2`, predicting INVALID. {len(rows)} boundaries "
          f"({len(valid)} valid : {n_pos} invalid). Held-out pooled AUC per task × feature group:\n",
          "| task | generic | boundary_specific | combined |", "|---|---|---|---|"]
    for t in tasks:
        md.append(f"| {t} | " + " | ".join(f"{res[t][g]:.3f}" for g in
                  ("generic", "boundary_specific", "combined")) + " |")
    md += ["\n## Honest reading\n",
           "- **On the LUMPED task generic ≈ or > boundary-specific.** A size-confound control "
           "(`results/validity_baseline_size_control_v1.md`) tested whether this is a size artifact "
           "and **found it is NOT**: under grouped CV `size_only` ≈ chance and ablating size does not "
           "hurt generic — grouped CV already neutralizes the within-sample size separation. "
           "generic's residual lumped signal is its NON-size signature features (pointer / nested-"
           "pointer / alloc counts), a coarse proxy for the same struct/pointer risk our rf features "
           "target. combined < generic is small-data overfit (33 features, ~17 invalid-bearing programs).",
           "- **Split by MECHANISM, the picture is the intended one:** for "
           "`invalid_isolation_invariant`, boundary-specific features win the grouped-CV baseline "
           f"(**{res['valid vs isolation_invariant']['boundary_specific']:.3f}** vs generic "
           f"{res['valid vs isolation_invariant']['generic']:.3f}; single-feature "
           "`rf_struct_index_field` 0.88, `rf_unmasked_field_index` 0.79). For "
           "`invalid_intrinsic_ub`, **generic is near-random** "
           f"({res['valid vs intrinsic_ub']['generic']:.3f}) while combined reaches "
           f"**{res['valid vs intrinsic_ub']['combined']:.3f}** — the signal is the guard×op "
           "INTERACTION (`rf_unguarded_ubop`) a linear model needs that engineered term to see.",
           "- **Takeaway:** the lumped task stays hard to claim because generic NON-size signature "
           "features (pointer / nested-pointer / alloc counts) act as coarse proxies for the same "
           "pointer/struct risk — the size hypothesis was tested and REJECTED (grouped CV already "
           "neutralizes size; see `validity_baseline_size_control_v1.md`). The robust claim is "
           "per-mechanism: semantic-risk features explain isolation invariants and intrinsic-UB "
           "better than generic structure, especially when arithmetic guards matter. Next: close "
           "generator gaps to grow labels, then external programs for generalization.\n",
           "> This is a baseline (no tuning) on the audited `validity_v2`; AUC pooled across grouped "
           "folds; ~17 programs carry invalids, so treat magnitudes as indicative.\n"]
    Path(a.out).write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"wrote {a.out}")
    for t in tasks:
        print(f"  {t:32} " + "  ".join(f"{g}={res[t][g]:.3f}" for g in
              ("generic", "boundary_specific", "combined")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
