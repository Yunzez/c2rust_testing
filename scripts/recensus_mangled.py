#!/usr/bin/env python3
# Re-census a cached llvm-cov json for an IDIOMATIC (mangled-name) translation.
# Keeps all functions whose filename contains the lib-source needle; no _R exclusion.
# Usage: recensus_mangled.py <label> <src_needle> <metric> <library> <tool>
import json, sys, statistics, re
label, needle, metric, library, tool = sys.argv[1:6]
d=json.load(open(f"/tmp/rq3_{label}.json"))["data"][0]
rows={}
for f in d["functions"]:
    fns=f.get("filenames",[])
    if not any(needle in p for p in fns): continue
    if any("fuzz_targets" in p for p in fns): continue      # drop the harness itself
    rows[f["name"]]=max(rows.get(f["name"],0), f.get("count",0))
vals=list(rows.values()); reached=[c for c in vals if c>0]
res={"cell":label,"library":library,"tool":tool,"metric":metric,"theirs":0,
     "n_functions":len(vals),"n_reached":len(reached),
     "median_hits":statistics.median(reached) if reached else 0,
     "min_hits":min(reached) if reached else 0,"max_hits":max(reached) if reached else 0}
json.dump(res,open(f"/home/yunzez/c2rust_testing/results/rq4_effectiveness/reach_cells/{label}.json","w"),indent=1)
def leaf(nm):
    m=re.findall(r'\d+([a-zA-Z_][a-zA-Z0-9_]*)',nm); return (m[-1] if m else nm)[:36]
top=sorted(rows.items(),key=lambda x:-x[1])[:6]
print(f"{label}: {metric} reached {res['n_reached']}/{res['n_functions']} median {res['median_hits']:,.0f} max {res['max_hits']:,}")
for nm,c in top: print(f"   {c:>12,}  {leaf(nm)}")
