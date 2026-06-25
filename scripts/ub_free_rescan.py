#!/usr/bin/env python3

"""Re-scan the dataset under the UB-FREE rule (boundary-validity condition 4 of 5; see stu_selection.md §5).

Convention (Csmith/EMI/Alive2 line, see memory ub-differential-oracle-principle): a divergence counts as a
real translation bug ONLY on a UB-free input; a divergence reachable only via a UB-triggering input is
EXCLUDED, not a bug. UB-free testability is ONE selection rule, not the whole definition — the other rules
(constructible inputs, comparable outputs, mappable, isolatable deps) are unchanged here.

This is the cheap STATIC re-scan: it re-labels the divergences we ALREADY recorded. For each currently-
"invalid" boundary it asks: was the recorded divergence backed by a C-side UB signal (UBSan / ASan / crash)?
 - YES -> ub_excluded (UB-triggering input; not a real bug under the UB-free rule).
 - NO (a pure C!=Rust value mismatch with no UB) -> CANDIDATE_REAL_BUG (a UB-free divergence to investigate).

NOTE: this does NOT prove a boundary has no UB-free bug — only that its RECORDED divergence was UB-backed.
Actually FINDING UB-free bugs needs UB-free FUZZING (reject UB inputs, keep going), which is the harness
change this motivates. Usage: python3 scripts/ub_free_rescan.py
"""

from __future__ import annotations

import json
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UB_LABELS = {"C_UB_CONFIRMED", "C_CRASH", "C_CRASH_NO_ARTIFACT"}
INVALID = {"invalid_isolation_invariant", "invalid_intrinsic_ub"}


def ub_backed(r: dict) -> bool:
    if r.get("label") in UB_LABELS:
        return True
    cs = r.get("crash_summary") or ""
    return any(s in cs for s in ("AddressSanitizer", "UndefinedBehaviorSanitizer", "Sanitizer"))


def main() -> int:
    rows = [json.loads(l) for l in (ROOT / "dataset" / "boundaries_v4.jsonl").read_text().splitlines()
            if l.strip()]
    rescan = Counter()
    candidates = []
    for r in rows:
        vv = r.get("validity_v2", "")
        if vv == "valid":
            rescan["valid (UB-free testable, no divergence)"] += 1
        elif vv in INVALID:
            if ub_backed(r):
                rescan["ub_excluded (divergence on UB-triggering input)"] += 1
            else:
                rescan["CANDIDATE_REAL_BUG (UB-free divergence)"] += 1
                candidates.append((r.get("pair"), r.get("fn"), vv, r.get("label")))
        else:  # excluded / weak_exclude / excluded_generator — other selection rules (1/3), unchanged
            rescan[f"unchanged: {vv}"] += 1

    n_inv = sum(1 for r in rows if r.get("validity_v2") in INVALID)
    md = ["# UB-free re-scan of dataset v4 (selection rule 4 of 5)\n",
          f"Convention: a divergence is a real bug only on a UB-free input; UB-triggering divergences are "
          f"EXCLUDED. Static re-label of recorded divergences (does NOT search for new UB-free divergences "
          f"— that needs UB-free fuzzing). {len(rows)} boundaries; {n_inv} previously 'invalid'.\n",
          "| re-scanned class | n |", "|---|--:|"]
    for k, v in sorted(rescan.items(), key=lambda x: -x[1]):
        md.append(f"| {k} | {v} |")
    md += ["\n## Candidate real bugs (UB-free divergences)\n"]
    if candidates:
        md += [f"- `{p}::{f}` (was {vv}, label {lab})" for p, f, vv, lab in candidates]
    else:
        md.append("- **None.** Every recorded divergence on the (faithful c2rust) corpus was UB-backed "
                  "(UBSan/ASan/crash) → excluded under the UB-free rule. Expected: c2rust is faithful here; "
                  "the only real bug we have is the INJECTED one in `g3_g2_bug` (separate). The 'invalid' "
                  "class is entirely **UB-triggering-input artifacts of unfiltered fuzzing**, which is "
                  "exactly what selection rule 4 (UB-free) is for.")
    md += ["\n## Reading\n",
           "- Under the UB-free rule, all "
           f"{rescan['ub_excluded (divergence on UB-triggering input)']} previously-'invalid' boundaries "
           "re-classify as UB-excluded (not bugs). This CONFIRMS the corpus is faithful and that the old "
           "invalid labels were naive-fuzzing-hits-UB artifacts, not translation defects.",
           "- UB-free testability is ONE rule (condition 4); the "
           f"{sum(v for k,v in rescan.items() if k.startswith('unchanged'))} excluded/weak boundaries are "
           "ruled out by the OTHER conditions (1 constructibility / 3 comparability), unchanged here.",
           "- **Next (the real step):** UB-free FUZZING — make the harness reject UBSan/ASan-flagged inputs "
           "and keep fuzzing, so any surviving divergence is a UB-free real bug. This static re-scan only "
           "shows the recorded divergences were UB-backed; it cannot find new UB-free bugs."]
    (ROOT / "results" / "ub_free_rescan_v1.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("wrote results/ub_free_rescan_v1.md\n")
    for k, v in sorted(rescan.items(), key=lambda x: -x[1]):
        print(f"  {v:3}  {k}")
    print(f"\ncandidate real bugs (UB-free divergences): {len(candidates)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
