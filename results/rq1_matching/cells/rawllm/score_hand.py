import json,sys,os
sys.path.insert(0,"/home/yunzez/c2rust_testing/tools/stu_selector")
import matcher
c=json.load(open(sys.argv[1])); r=json.load(open(sys.argv[2])); truth=json.load(open(sys.argv[3]))

# --- PARTITION GUARD: every C fn must be scorable (in truth) XOR dissolved. No fn may vanish. ---
# If a dissolved.json sits next to truth.json, assert truth ∪ dissolved == all C fns (no gap/overlap),
# so a function can never silently fall out of the denominator accounting.
allc=set(f["name"] for f in c["functions"])
dpath=os.path.join(os.path.dirname(os.path.abspath(sys.argv[3])),"dissolved.json")
dissolved=set(json.load(open(dpath))) if os.path.exists(dpath) else set()
if dissolved:
    gap=allc-set(truth)-dissolved
    overlap=set(truth)&dissolved
    assert not gap and not overlap, f"PARTITION BROKEN — gap(dropped)={gap} overlap={overlap}"

S=len(truth)
leaf=lambda x:x.split("::")[-1]
neq=sum(1 for k in truth if truth[k]==k)                 # mechanical name-eq (Type::method fails)
neq_leaf=sum(1 for k in truth if leaf(truth[k])==k)      # fair naive leaf-name baseline
res=matcher.match(c,r,topo=True)
correct=sum(1 for (cc,rr,s,k) in res["matched"] if truth.get(cc)==rr)
matched_pairs={cc:rr for (cc,rr,s,k) in res["matched"]}
# TWO denominators bracket the truth: scorable (matching-possible) vs gross (dissolved=auto-miss)
print(f"scorable={S}  dissolved={len(dissolved)}  allC={len(allc)}")
print(f"matcher_recall(scorable) = {round(correct/S,3)} ({correct}/{S})   "
      f"gross(allC) = {round(correct/len(allc),3) if allc else 0} ({correct}/{len(allc)})")
print(f"name_eq: mechanical={round(neq/S,3)}  leaf={round(neq_leaf/S,3)}")
print("matcher pairs (C->Rust):")
for k in truth:
    got=matched_pairs.get(k,"—"); ok="✓" if got==truth[k] else "✗"
    print(f"  {ok} {k:16s} truth={leaf(truth[k]):22s} matcher={leaf(got) if got!='—' else '—'}")
