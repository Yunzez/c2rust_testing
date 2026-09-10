"""Three-arm comparison with the t=0 / t=600 split.
  t=0   : coverage of the arm's INITIAL seeds alone (E/t0/<arm>/...)
  t=600 : coverage of the arm's final campaign corpus (E/<arm>/...)
  delta : regions the fuzzer explored beyond its seeds
Application level from c2r_coverage result.json (identity union over all harnesses); per boundary the
regions of the boundary's own function from the llvm-cov exports. usage: compare_arms.py EXPDIR"""
import json, sys, pathlib
E = pathlib.Path(sys.argv[1]); ARMS = ["base", "grid"]
man = json.load(open(E / "seeds" / "manifest.json")); seeded = man["seeded"]
def own_fn(oursdir, b):
    f = oursdir / f"{b}.json"
    if not f.exists(): return None
    for fn in json.load(open(f))["data"][0]["functions"]:
        n = fn["name"]
        if f"{len(b)}{b}17h" in n or n.endswith(b) or n == b:
            regs = fn["regions"]; return (sum(1 for r in regs if r[4] > 0), len(regs))
    return None
def load(p):
    return json.load(open(p)) if p.exists() else None
res = {a: load(E / a / "analysis" / "result.json") for a in ARMS}
t0 = {a: load(E / "t0" / a / "analysis" / "result.json") for a in ARMS}
corp = lambda root: sum(len(list(d.glob("*"))) for d in root.iterdir() if d.is_dir()) if root.is_dir() else None
out = [f"# Three-arm seed experiment — tulip × {E.name.replace('threearm_','') if '_' in E.name else 'c2rust'} (600 s per arm, seed 42, -max_len 65536, one harness build)\n",
       "## Application level (c2r_coverage union over all harnesses; universe = the cell's archived universe)\n",
       "| arm | initial corpus | final corpus | fn t=600 | regions t=0 (seeds only) | regions t=600 | Δ fuzzing beyond seeds | region cov t=0 → t=600 |",
       "|---|---:|---:|---:|---:|---:|---:|---:|"]
for a in ARMS:
    r, z = res[a], t0[a]
    if not r: out.append(f"| {a} | — | — | — | — | — | — | — |"); continue
    reg, fun = r["region"], r["function"]
    r0 = z["region"]["covered_ours"] if z else None
    out.append(f"| {a} | {corp(E/'t0'/a/'corpus')} | {corp(E/a/'corpus')} | {fun['covered_ours']}/{fun['total_in_scope']} | "
               f"{r0 if r0 is not None else '—'} | {reg['covered_ours']} | {(reg['covered_ours']-r0) if r0 is not None else '—'} | "
               f"{(z['region']['ours_coverage'] if z else float('nan')):.3f} → **{reg['ours_coverage']:.3f}** |")
out += ["\n## Per boundary: regions of the boundary's own function, seeded boundaries only (covered/total; t=0 → t=600)\n",
        "| boundary | seed len | base | grid | grid−base t=600 | Δfuzz base / grid |", "|---|---:|---:|---:|---:|---:|"]
tot = {a: [0, 0, 0] for a in ARMS}; inc_gr, inc_gb = [], []; hit = {"grid": 0}; fuzz_gain = {a: 0 for a in ARMS}
for b in sorted(seeded):
    v = {a: own_fn(E / a / "ours", b) for a in ARMS}; v0 = {a: own_fn(E / "t0" / a / "ours", b) for a in ARMS}
    if any(x is None for x in v.values()):
        out.append(f"| {b} | {seeded[b]['length']} | " + " | ".join("—" if v[a] is None else f"{v[a][0]}/{v[a][1]}" for a in ARMS) + " | — | — |"); continue
    cell = lambda a: (f"{v0[a][0]}→" if v0[a] else "") + f"{v[a][0]}/{v[a][1]}"
    for a in ARMS:
        tot[a][0] += v[a][0]; tot[a][1] += v[a][1]
        if v0[a]: tot[a][2] += v0[a][0]; fuzz_gain[a] += v[a][0] - v0[a][0]
    dgb = v["grid"][0] - v["base"][0]; inc_gb.append(dgb); inc_gr.append(dgb)
    if v["grid"][0] > v["base"][0]: hit["grid"] += 1
    dz = " / ".join(f"{v[a][0]-v0[a][0]:+d}" if v0[a] else "—" for a in ARMS)
    out.append(f"| {b} | {seeded[b]['length']} | {cell('base')} | {cell('grid')} | {dgb:+d} | {dz} |")
n = len(inc_gr) or 1
out.append(f"| **sum over {n}** | | " + " | ".join(f"{tot[a][2]}→{tot[a][0]}/{tot[a][1]}" for a in ARMS)
           + f" | {sum(inc_gb):+d} | " + " / ".join(f"{fuzz_gain[a]:+d}" for a in ARMS) + " |")
big = [(b, s["length"]) for b, s in sorted(seeded.items()) if s["length"] > 65536]
out += ["", f"- boundaries where grid > base at t=600: {hit['grid']} of {n}",
        f"- grid − base per boundary at t=600: min {min(inc_gb)}, median {sorted(inc_gb)[n//2]}, max {max(inc_gb)}",
        f"- initial seeds longer than -max_len 65536: {len(big)} boundaries × 6 seeds, longest {max(s for _, s in big) if big else 0} bytes "
        f"({', '.join(b for b, _ in big)}). Measured on ti_stoch (layout validation): such a seed is executed and its options decoded, "
        f"so the seed breaks the INITIAL reachability barrier; whether the fuzzer keeps mutating at that length is not established "
        f"(libFuzzer bounds generated inputs by -max_len), so no claim is made that the max_len limit is gone."]
(E / "COMPARE.md").write_text("\n".join(out) + "\n"); print("\n".join(out[:8]))
