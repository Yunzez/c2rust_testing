#!/usr/bin/env python3
"""Recompute E3 medians honestly.

The original cells reported a median over REACHED functions only (count>0), which
hides unreached functions. For a claim of the form "our fuzzing executes functions
N times vs the tool's 0", excluding the never-executed functions is exactly the
wrong convention: those functions are the ones where we agree with `theirs`.

This script adds, for every cell:
  median_reached  -- the old number (median over count>0)
  median_all      -- median over ALL censused functions, zeros included
  reach_frac      -- n_reached / n_functions
  median_all_src  -- how median_all was obtained: computed | derived-zero | unavailable

`derived-zero`: per_fn was not stored (mangled-name cells re-censused by
recensus_mangled.py), but n_reached <= n_functions/2 forces the all-function
median to 0 regardless of the distribution.
`unavailable`: per_fn missing and reached > half, so the value cannot be
reconstructed without re-running the cell.
"""
import glob
import json
import statistics

rows = []
for f in sorted(glob.glob("results/rq4_effectiveness/reach_cells/*.json")):
    d = json.load(open(f))
    pf = d.get("per_fn")
    nf, nr = d.get("n_functions"), d.get("n_reached")

    if pf:
        vals = list(pf.values())
        nz = [v for v in vals if v > 0]
        d["median_reached"] = statistics.median(nz) if nz else 0
        d["median_all"] = statistics.median(vals)
        d["median_all_src"] = "computed"
        d["n_functions"], d["n_reached"] = len(vals), len(nz)
        nf, nr = len(vals), len(nz)
    else:
        d["median_reached"] = d.get("median_hits")
        if d.get("metric") == "crash-all":
            d["median_all"] = None
            d["median_all_src"] = "crash-all"
        elif nf and nr is not None and nr <= nf / 2:
            d["median_all"] = 0
            d["median_all_src"] = "derived-zero"
        else:
            d["median_all"] = None
            d["median_all_src"] = "unavailable"

    d["reach_frac"] = f"{nr}/{nf}" if (nf and nr is not None) else None
    json.dump(d, open(f, "w"), indent=1)
    rows.append((d.get("cell"), d.get("metric"), d["reach_frac"],
                 d["median_reached"], d["median_all"], d["median_all_src"]))

print(f"{'cell':26} {'metric':22} {'reach':>9} {'med_reached':>14} {'med_all':>14}  src")
for c, m, rf, mr, ma, src in rows:
    fr = f"{mr:,.0f}" if isinstance(mr, (int, float)) else str(mr)
    fa = f"{ma:,.0f}" if isinstance(ma, (int, float)) else str(ma)
    print(f"{c:26} {m:22} {str(rf):>9} {fr:>14} {fa:>14}  {src}")

zeros = [c for c, _, _, _, ma, _ in rows if ma == 0]
print(f"\nHONEST median == 0 (identical to theirs) in {len(zeros)} cells: {', '.join(zeros)}")
