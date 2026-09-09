#!/usr/bin/env python3
"""Compare each rebuilt rlib universe with the archived one.
 - bin-route cells (archived raw/denominator.json): identity = (basename, name w/o hash, every region) -- exact.
 - no-denominator cells (bzip2/c2rust, genann/c2rust, genann/laertes): the archived universe is the tests
   build's, kept as (module file, line) identity lists + region count; the rebuilt export is remapped
   through the pair's linemap with c2r_coverage.extract and compared at that level."""
import json, os, sys, glob
sys.path.insert(0, "/home/yunzez/c2rust_testing/scripts")
import c2r_coverage as cc
R = "/home/yunzez/c2rust_testing"; S = os.path.dirname(os.path.abspath(__file__))
CELLS = [("bzip2","c2rust"),("genann","c2rust"),("genann","laertes"),("genann","c2saferrust"),("genann","crown"),
         ("genann","sactor"),("cjson","c2rust"),("lil","c2rust"),("lil","c2saferrust"),("lil","crown"),("lil","laertes")]
def ids_raw(p):
    d = json.load(open(p)); out = set()
    for f in d["data"][0]["functions"]:
        out.add((f["name"].split("17h")[0], tuple(os.path.basename(x) for x in f["filenames"]), tuple(tuple(r[:4]) for r in f["regions"])))
    return out
def txt_ids(p):
    out = set()
    for l in open(p):
        parts = l.rstrip("\n").split("\t")
        if len(parts) >= 2: out.add((parts[0], int(parts[1].split()[1])))
    return out
report = []
for lib, tool in CELLS:
    new = f"{S}/denom_{lib}_{tool}/denominator.json"; A = f"{R}/results/rq3_coverage/{lib}/{tool}"
    if not os.path.exists(new): report.append((lib, tool, "NO REBUILD", "")); continue
    src = json.load(open(new)).get("_source", {}).get("selected_by", "?")
    arch = f"{A}/raw/denominator.json"
    if os.path.exists(arch):
        a, b = ids_raw(arch), ids_raw(new)
        same = a == b
        report.append((lib, tool, "SAME" if same else "DIFF", f"archived(bin) {len(a)} fns vs rlib {len(b)} fns; only-archived {len(a-b)}, only-rlib {len(b-a)} [{src}]"))
        if not same:
            for x in sorted(a-b)[:5]: print("   only archived:", x[0][:70], x[1])
            for x in sorted(b-a)[:5]: print("   only rlib    :", x[0][:70], x[1])
    else:
        lm = f"{R}/benchmark/pairs/rq4/{lib}_{tool}/translated/{lib}_{tool}.rs.linemap.json"
        scored, ranges = cc.load_map(lm)
        canon = open(lm.replace(".linemap.json", "")).read().split("\n")
        tf, tr, _ = cc.extract(new, ranges, scored, canon)
        uni_f, _ = cc.sets(tf); uni_r, _ = cc.sets(tr)
        newf = {(f, l) for (f, n, l) in uni_f} if uni_f and len(next(iter(uni_f))) == 3 else set(uni_f)
        arch_f = set()
        for t in ("covered_by_both", "only_tests", "only_ours", "covered_by_neither"):
            arch_f |= txt_ids(f"{A}/analysis/{t}.txt")
        res = json.load(open(f"{A}/analysis/result.json"))
        rt = res["region"]["total_in_scope"]; ft = res["function"]["total_in_scope"]
        same = (newf == arch_f) and (len(uni_r) == rt)
        report.append((lib, tool, "SAME" if same else "DIFF",
                       f"archived(tests build) fn {ft} / reg {rt}  vs rlib fn {len(newf)} / reg {len(uni_r)}; fn only-archived {len(arch_f-newf)}, only-rlib {len(newf-arch_f)} [{src}]"))
        if newf != arch_f:
            for x in sorted(arch_f-newf)[:5]: print("   only archived:", x)
            for x in sorted(newf-arch_f)[:5]: print("   only rlib    :", x)
print()
for r in report: print("%-8s %-12s %-5s %s" % r)
json.dump([dict(lib=a, tool=b, verdict=c, detail=d) for a, b, c, d in report], open(f"{S}/compare.json", "w"), indent=1)
