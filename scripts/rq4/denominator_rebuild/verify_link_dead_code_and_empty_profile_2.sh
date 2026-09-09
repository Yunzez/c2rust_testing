#!/usr/bin/env bash
set -u
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
REPO=/home/yunzez/c2rust_testing
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
T=$S/denom_check/denom_qsort_laertes
RLIB=$T/target/release/deps/libqsort_laertes-84b8cf85ac6f0f1a.rlib
RLIB2=$(ls -t $T/target_nodead/release/deps/libqsort_laertes-*.rlib | head -1)
echo "--- full error of --empty-profile on the archive:"; $TC/llvm-cov export --empty-profile "$RLIB" 2>&1 >/dev/null | head -3
echo "--- archive members:"; ar t "$RLIB"
unpack() { d=$2; rm -rf $d; mkdir -p $d; (cd $d && ar x "$1"); ls $d/*.rcgu.o; }
O1=$(unpack "$RLIB" $T/objs_dead); O2=$(unpack "$RLIB2" $T/objs_nodead)
echo "--- B'. --empty-profile on the unpacked object (link-dead-code build):"; $TC/llvm-cov export --empty-profile $O1 > $T/B_obj_empty.json 2>$T/B2.err; echo "rc=$? $(head -c 300 $T/B2.err)"
echo "--- D'. --empty-profile on the unpacked object (NO link-dead-code build):"; $TC/llvm-cov export --empty-profile $O2 > $T/D_obj_empty.json 2>$T/D2.err; echo "rc=$? $(head -c 300 $T/D2.err)"
python3 - <<P
import json
def load(p):
    try: return json.load(open(p))
    except Exception as e: print(p.split('/')[-1], "UNREADABLE", e); return None
def ids(d):
    out=set()
    for f in d["data"][0]["functions"]:
        r=f["regions"][0] if f["regions"] else None
        out.add((f["name"].split('17h')[0], tuple(f["filenames"]), (r[0],r[1],r[2],r[3]) if r else None, len(f["regions"])))
    return out
regs=lambda d: sum(len(f["regions"]) for f in d["data"][0]["functions"])
names={"A_bin":"$T/A_bin.json","B_obj_empty(dead)":"$T/B_obj_empty.json","C_unpack+zeroed(dead)":"$T/denominator.json","D_obj_empty(NOdead)":"$T/D_obj_empty.json","G_archived":"$REPO/results/rq3_coverage/qsort/laertes/raw/denominator.json"}
D={k:load(v) for k,v in names.items()}; D={k:v for k,v in D.items() if v}
for k,v in D.items(): print("%-24s fns=%-4d regions=%d"%(k,len(v["data"][0]["functions"]),regs(v)))
I={k:ids(v) for k,v in D.items()}
ks=list(I)
for i in range(len(ks)):
    for j in range(i+1,len(ks)):
        a,b=I[ks[i]],I[ks[j]]
        print("%-24s vs %-24s equal=%s  only-left=%d only-right=%d"%(ks[i],ks[j],a==b,len(a-b),len(b-a)))
        if a!=b and len(a^b)<=4:
            for x in (a-b): print("    only in",ks[i],":",x[0][:60],x[2])
            for x in (b-a): print("    only in",ks[j],":",x[0][:60],x[2])
# does --empty-profile zero everything?
if "B_obj_empty(dead)" in D:
    d=D["B_obj_empty(dead)"]; print("empty-profile counts all zero:", all(f["count"]==0 and all(r[4]==0 for r in f["regions"]) for f in d["data"][0]["functions"]))
P
