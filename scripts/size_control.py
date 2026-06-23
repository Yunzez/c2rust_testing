#!/usr/bin/env python3

"""Size-confound control for the P(valid|x) baseline.

The lumped baseline showed generic >= boundary-specific. We claim that edge is a SIZE confound
(authored negatives are systematically smaller, so c_nodes/c_stmts "predict" invalid in this
corpus). Two lightweight controls nail it:

  A · ablation   — drop the size/complexity features from `generic` (size_only / generic_ablated)
                   and re-run grouped-CV. If `size_only` ~= `generic` and `generic_ablated`
                   collapses, the edge WAS size.
  B · size-match — pair each invalid with the nearest-size valid (by c_stmts) to equalize the size
                   distribution, then re-run grouped-CV generic vs boundary-specific on that subset.

Goal is NOT "boundary-specific wins everywhere" — it is: (1) generic's lumped edge is size; (2) once
size is controlled, semantic-risk features still carry the mechanism; (3) per-mechanism is stable.

Usage: python3 scripts/size_control.py
"""

from __future__ import annotations

import json
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
from validity_baseline import grouped_cv_auc, GENERIC, RF, INVALID  # noqa: E402

SIZE = ["c_nodes", "r_nodes", "c_stmts", "r_stmts", "c_cyclomatic", "r_cyclomatic",
        "d_nodes", "d_stmts", "size_ratio", "c_loops", "c_max_loop_depth"]
SIZE_ONLY = ["c_nodes", "c_stmts"]
GENERIC_ABLATED = [f for f in GENERIC if f not in SIZE]


def add_interaction(rows):
    for r in rows:
        ub = (r.get("rf_div_mod", 0) + r.get("rf_shift", 0) + r.get("rf_compound_arith", 0)) > 0
        guarded = bool(r.get("rf_nonzero_guard", 0)) or bool(r.get("rf_width_guard", 0))
        r["rf_unguarded_ubop"] = int(ub and r.get("rf_signed", 0) and not guarded)


def size_matched(valid, neg, key="c_stmts"):
    """Pair each invalid with the nearest unused valid by `key` -> a ~50/50 size-balanced subset."""
    pool = list(valid)
    out = []
    for inv in neg:
        if not pool:
            break
        j = min(range(len(pool)), key=lambda i: abs(pool[i].get(key, 0) - inv.get(key, 0)))
        out.append(inv)
        out.append(pool.pop(j))
    return out


def main() -> int:
    rows = [json.loads(l) for l in (ROOT / "dataset" / "boundaries_v2.jsonl")
            .read_text().splitlines() if l.strip()]
    rows = [r for r in rows if r.get("validity_v2") == "valid" or r.get("validity_v2") in INVALID]
    add_interaction(rows)
    valid = [r for r in rows if r["validity_v2"] == "valid"]
    iso = [r for r in rows if r["validity_v2"] == "invalid_isolation_invariant"]
    intr = [r for r in rows if r["validity_v2"] == "invalid_intrinsic_ub"]
    neg = iso + intr

    groups = {"size_only": SIZE_ONLY, "generic (full)": GENERIC,
              "generic_ablated (no size)": GENERIC_ABLATED, "boundary_specific": RF,
              "combined_ablated (rf + non-size generic)": GENERIC_ABLATED + RF}
    tasks = {"lumped": rows, "valid vs isolation": valid + iso, "valid vs intrinsic_ub": valid + intr}
    res = {t: {g: grouped_cv_auc(rs, names)[0] for g, names in groups.items()}
           for t, rs in tasks.items()}

    md = ["# Size-confound control for P(valid|x)\n",
          "Same plain logistic regression, program-grouped 5-fold CV, label `validity_v2` "
          "(predicting INVALID). Held-out pooled AUC.\n",
          "## A · Ablation — is the generic edge just size?\n",
          "| task | size_only | generic(full) | generic_ablated | boundary_specific | combined_ablated |",
          "|---|---|---|---|---|---|"]
    order = ["size_only", "generic (full)", "generic_ablated (no size)", "boundary_specific",
             "combined_ablated (rf + non-size generic)"]
    for t in tasks:
        md.append(f"| {t} | " + " | ".join(f"{res[t][g]:.3f}" for g in order) + " |")

    # B · size-matched
    mvalid_pre = statistics.mean(r.get("c_stmts", 0) for r in valid)
    mneg_pre = statistics.mean(r.get("c_stmts", 0) for r in neg)
    matched = size_matched(valid, neg)
    mvalid_post = statistics.mean(r.get("c_stmts", 0) for r in matched if r["validity_v2"] == "valid")
    mneg_post = statistics.mean(r.get("c_stmts", 0) for r in matched if r["validity_v2"] in INVALID)
    m_gen = grouped_cv_auc(matched, GENERIC)[0]
    m_size = grouped_cv_auc(matched, SIZE_ONLY)[0]
    m_bs = grouped_cv_auc(matched, RF)[0]

    md += ["\n## B · Size-matched subset (each invalid ↔ nearest-size valid by c_stmts)\n",
           f"Mean c_stmts before match — valid {mvalid_pre:.1f} vs invalid {mneg_pre:.1f}; "
           f"after match — valid {mvalid_post:.1f} vs invalid {mneg_post:.1f} (size equalized). "
           f"{len(matched)} boundaries.\n",
           "| feature group | held-out AUC (size-matched) |", "|---|---|",
           f"| size_only | {m_size:.3f} |",
           f"| generic (full) | {m_gen:.3f} |",
           f"| boundary_specific | {m_bs:.3f} |"]

    L = res["lumped"]
    md += ["\n## Reading (the control OVERTURNS the size-confound hypothesis)\n",
           f"- **Size is NOT the confound.** Under program-grouped CV `size_only` scores "
           f"**{L['size_only']:.3f}** (lumped) and {m_size:.3f} (size-matched) — at/below chance. "
           f"The descriptive single-feature size separation (c_nodes single-feature |AUC−0.5|≈0.27) "
           f"was a WITHIN-SAMPLE artifact that grouped CV already neutralizes (different programs "
           f"have different size baselines, so it does not transfer).",
           f"- **The generic edge is NOT size either.** Ablating all size/complexity features gives "
           f"`generic_ablated` **{L['generic_ablated (no size)']:.3f}** ≥ `generic(full)` "
           f"{L['generic (full)']:.3f} — removing size does not hurt. So generic's lumped signal "
           f"comes from the NON-size signature features (pointer/nested-pointer/alloc counts), which "
           f"are a coarse proxy for the same struct/pointer risk our rf features target.",
           f"- **The real claim survives, but it is per-MECHANISM, not lumped.** On the lumped task "
           f"the coarse pointer-count proxy keeps generic competitive ({L['generic_ablated (no size)']:.3f} "
           f"vs boundary_specific {L['boundary_specific']:.3f}). Split by mechanism, semantic-risk "
           f"features are decisive: isolation boundary_specific "
           f"{res['valid vs isolation']['boundary_specific']:.3f} > generic_ablated "
           f"{res['valid vs isolation']['generic_ablated (no size)']:.3f}; and for intrinsic-UB "
           f"generic is **near-random** ({res['valid vs intrinsic_ub']['generic_ablated (no size)']:.3f}) "
           f"while combined_ablated reaches "
           f"{res['valid vs intrinsic_ub']['combined_ablated (rf + non-size generic)']:.3f} — generic "
           f"structure simply cannot see whether arithmetic is guarded.",
           "\n> Corrected conclusion: the earlier 'size confound' explanation does NOT hold — "
           "grouped CV already controls size, and generic's residual signal is a pointer-signature "
           "proxy. The contribution rests on the PER-MECHANISM result (and especially intrinsic-UB, "
           "which generic cannot capture), not on the lumped task. Small-data caveats apply "
           "(~17 invalid-bearing programs); external corpora are the next robustness step.\n"]
    (ROOT / "results" / "validity_baseline_size_control_v1.md").write_text("\n".join(md) + "\n",
                                                                           encoding="utf-8")
    print("wrote results/validity_baseline_size_control_v1.md")
    print("\n=== ablation (lumped) ===")
    for g in order:
        print(f"  {g:42} {res['lumped'][g]:.3f}")
    print(f"\n=== size-matched (valid c_stmts {mvalid_post:.1f} vs invalid {mneg_post:.1f}) ===")
    print(f"  size_only {m_size:.3f} | generic {m_gen:.3f} | boundary_specific {m_bs:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
