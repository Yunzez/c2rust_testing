#!/usr/bin/env python3
"""Aggregate the seeds-only census: one row per cell -> results/rq4_llm_refinement/census/{census.json, CENSUS.md},
archiving each cell's policy manifest, t=0 results and rebuild funnel. usage: census_table.py <scratchpad> <out_dir>"""
import json, pathlib, shutil, sys
S, OUT = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]); R = pathlib.Path('/home/yunzez/c2rust_testing')
ORDER = ["bzip2","cjson","genann","lil","lodepng","optipng","qsort","quadtree","urlparser","tulip"]
rows = []
for d in sorted(S.glob("census_*")):
    if not d.is_dir() or not (d / "t0/policy/analysis/result.json").exists(): continue
    lib, tool = d.name.replace("census_", "").split("_", 1)
    b = json.load(open(d / "t0/base/analysis/result.json"))["region"]; p = json.load(open(d / "t0/policy/analysis/result.json"))["region"]
    a = json.load(open(R / f"results/rq3_coverage/{lib}/{tool}/analysis/result.json")); ar = a["region"]
    man = json.load(open(d / "seeds/policy/manifest.json")); fb = json.load(open(d / "base/funnel.json")); af = json.load(open(R / f"results/rq3_coverage/{lib}/{tool}/funnel.json"))
    row = {"lib": lib, "tool": tool, "planned": len(fb), "built": sum(1 for r in fb if r["built"]), "archived_built": sum(1 for r in af if r["built"]),
           "plan_guided": man["summary"]["plan_guided"], "fallback": man["summary"]["fallback"], "universe": b["total_in_scope"],
           "default_seed": b["covered_ours"], "policy_seeds": p["covered_ours"], "archived_campaign": ar["covered_ours"],
           "default_cov": b["ours_coverage"], "policy_cov": p["ours_coverage"], "campaign_cov": ar["ours_coverage"],
           "seed_effect_pp": round(100 * (p["ours_coverage"] - b["ours_coverage"]), 1), "policy_minus_campaign_pp": round(100 * (p["ours_coverage"] - ar["ours_coverage"]), 1),
           "generator": fb[0].get("generator", "") if fb else ""}
    row["qualifies"] = row["policy_minus_campaign_pp"] >= 5.0
    rows.append(row)
    cd = OUT / "cells" / f"{lib}_{tool}"; cd.mkdir(parents=True, exist_ok=True)
    shutil.copy(d / "seeds/policy/manifest.json", cd / "policy_manifest.json"); shutil.copy(d / "base/funnel.json", cd / "rebuild_funnel.json")
    for arm in ("base", "policy"): shutil.copy(d / f"t0/{arm}/analysis/result.json", cd / f"t0_{arm}_result.json")
    if (d / "t0/t0_status.json").exists(): shutil.copy(d / "t0/t0_status.json", cd / "t0_status.json")
rows.sort(key=lambda r: (ORDER.index(r["lib"]), r["tool"]))
OUT.mkdir(parents=True, exist_ok=True); (OUT / "census.json").write_text(json.dumps(rows, indent=1) + "\n")
md = ["# Seeds-only census — plan-guided seeds vs the default seed vs the archived campaign\n",
      "Per cell: harnesses rebuilt with generator 0.9.1 (seed-ir branch; emitted code identical to 0.9), one coverage build per harness, coverage of (a) the default 64-byte seed alone and (b) default + plan-guided seeds (frozen policy, `docs/seeding_policy_plan.md` §3), against the cell's archived universe; the archived campaign (3 600 s, generator 0.8) beside them. **seed effect** = policy − default; **rerun criterion** = policy seeds-only ≥ archived campaign + 5 pp.\n",
      "| cell | built (census / archived) | plan-guided / planned | default seed | policy seeds | archived campaign | seed effect | policy − campaign | qualifies |", "|---|---:|---:|---:|---:|---:|---:|---:|:-:|"]
for r in rows:
    md.append(f"| {r['lib']} × {r['tool']} | {r['built']} / {r['archived_built']} | {r['plan_guided']} / {r['planned']} | {r['default_seed']} ({r['default_cov']:.3f}) | {r['policy_seeds']} ({r['policy_cov']:.3f}) | {r['archived_campaign']} ({r['campaign_cov']:.3f}) | {r['seed_effect_pp']:+.1f} pp | {r['policy_minus_campaign_pp']:+.1f} pp | {'**yes**' if r['qualifies'] else 'no'} |")
md.append(f"\n{len(rows)} cells; qualifying: {', '.join(f'{r['lib']} × {r['tool']}' for r in rows if r['qualifies']) or 'none'}.")
(OUT / "CENSUS.md").write_text("\n".join(md) + "\n"); print("\n".join(md[3:]))
