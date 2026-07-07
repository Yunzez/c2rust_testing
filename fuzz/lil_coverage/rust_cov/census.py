"""Census over the CLAIMED-PAIRED functions from an llvm-cov export json."""
import json, sys
exp=json.load(open(sys.argv[1] if len(sys.argv)>1 else "/tmp/lilcov_export.json"))
data=exp["data"][0]
hot=[f["name"] for f in data["functions"] if f.get("count",0)>0]
paired=sorted(set(json.load(open("../../../results/rq2_cells/rawllm/lil/claimed_pairs.json")).values()))
def hit(n):
    ps=n.split("::"); return any(all(p in s for p in ps) for s in hot)
cov=[n for n in paired if hit(n)]; miss=[n for n in paired if not hit(n)]
print(f"CLAIMED-PAIRED functions covered: {len(cov)}/{len(paired)} = {len(cov)/len(paired):.1%}")
fnc=[n for n in paired if n.split('::')[-1].startswith('fnc_')]
print(f"  fnc_ handlers: {sum(1 for n in fnc if n in cov)}/{len(fnc)}   missed: {[m.split('::')[-1] for m in miss]}")
t=data["totals"]
print(f"whole-crate: functions {t['functions']['percent']:.1f}%  lines {t['lines']['percent']:.1f}% ({t['lines']['covered']}/{t['lines']['count']})")
