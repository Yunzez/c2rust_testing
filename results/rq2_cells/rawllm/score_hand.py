import json,sys,copy
sys.path.insert(0,"/home/yunzez/c2rust_testing/tools/stu_selector")
import matcher
c=json.load(open(sys.argv[1])); r=json.load(open(sys.argv[2])); truth=json.load(open(sys.argv[3]))
S=len(truth)
neq=sum(1 for k in truth if truth[k]==k)
res=matcher.match(c,r,topo=True)
correct=sum(1 for (cc,rr,s,k) in res["matched"] if truth.get(cc)==rr)
matched_pairs={cc:rr for (cc,rr,s,k) in res["matched"]}
print(f"scorable={S}  name_eq_recall={round(neq/S,3)}  matcher_recall={round(correct/S,3)} ({correct}/{S})")
print("matcher pairs (C->Rust):")
for k in truth:
    got=matched_pairs.get(k,"—"); ok="✓" if got==truth[k] else "✗"
    print(f"  {ok} {k:14s} truth={truth[k]:20s} matcher={got}")
