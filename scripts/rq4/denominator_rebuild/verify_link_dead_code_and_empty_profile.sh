#!/usr/bin/env bash
set -u
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
REPO=/home/yunzez/c2rust_testing
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
export RQ4_WORK=$S/denom_check
bash $REPO/scripts/rq4/denominator.sh qsort laertes "qsort_laertes::quickSort(std::ptr::null_mut(), 0, 0)"
T=$RQ4_WORK/denom_qsort_laertes
RLIB=$(python3 - <<P
import json
for l in open("$T/cargo.json"):
    if not l.startswith("{"): continue
    m=json.loads(l)
    if m.get("reason")=="compiler-artifact" and m["target"]["name"]=="qsort_laertes" and "lib" in m["target"]["kind"]:
        print([f for f in m["filenames"] if f.endswith(".rlib")][0])
P
)
echo "RLIB(link-dead-code)=$RLIB"
echo "--- A. bin route (denom binary):"; $TC/llvm-cov export $T/target/release/denom -instr-profile=$T/denom.profdata 2>/dev/null > $T/A_bin.json; python3 -c "import json;print(len(json.load(open('$T/A_bin.json'))['data'][0]['functions']),'fns')"
echo "--- B. rlib via --empty-profile on the ARCHIVE directly:"; $TC/llvm-cov export --empty-profile "$RLIB" > $T/B_rlib_empty.json 2> $T/B.err; echo "rc=$? $(head -c 200 $T/B.err)"; python3 -c "import json;print(len(json.load(open('$T/B_rlib_empty.json'))['data'][0]['functions']),'fns')" 2>&1
echo "--- C. rlib via ar-unpack + unrelated profdata (current rlib_universe.py):"; python3 -c "import json;print(len(json.load(open('$T/denominator.json'))['data'][0]['functions']),'fns')"
echo "--- D. rebuild WITHOUT -C link-dead-code:"
cd $T && CARGO_TARGET_DIR=$T/target_nodead RUSTUP_TOOLCHAIN=nightly-2025-09-01 RUSTFLAGS="-C instrument-coverage -C codegen-units=1 --cfg fuzzing -C debug-assertions" cargo build --release --lib --message-format=json-render-diagnostics > $T/cargo_nodead.json 2> $T/build_nodead.log || { echo BUILD-FAIL; tail -3 $T/build_nodead.log; }
RLIB2=$(ls -t $T/target_nodead/release/deps/libqsort_laertes-*.rlib | head -1); echo "RLIB(no link-dead-code)=$RLIB2"
$TC/llvm-cov export --empty-profile "$RLIB2" > $T/D_rlib_nodead.json 2>/dev/null; python3 -c "import json;print(len(json.load(open('$T/D_rlib_nodead.json'))['data'][0]['functions']),'fns')"
echo "--- identity-set comparison (name, file, first-region lines):"
python3 - <<P
import json
def ids(p):
    d=json.load(open(p)); out=set()
    for f in d["data"][0]["functions"]:
        r=f["regions"][0] if f["regions"] else None
        out.add((f["name"], tuple(f["filenames"]), (r[0],r[1],r[2],r[3]) if r else None, len(f["regions"])))
    return out
A=ids("$T/A_bin.json"); B=ids("$T/B_rlib_empty.json"); C=ids("$T/denominator.json"); D=ids("$T/D_rlib_nodead.json")
G=ids("$REPO/results/rq3_coverage/qsort/laertes/raw/denominator.json")
def strip(s): return {(n.split('17h')[0] if '17h' in n else n, f, r, k) for n,f,r,k in s}   # drop the crate-disambiguator hash suffix
print("A bin      ", len(A)); print("B rlib-empty", len(B)); print("C rlib-unpack", len(C)); print("D no-dead-code", len(D)); print("G archived  ", len(G))
print("B==C (same build, two readers):", B==C)
print("A==C (bin vs rlib, link-dead-code):", A==C, " A-C:", len(A-C), " C-A:", len(C-A))
print("D vs C (no link-dead-code vs with), names hash-stripped:", strip(D)==strip(C), " D-C:", len(strip(D)-strip(C)), " C-D:", len(strip(C)-strip(D)))
print("G vs C (archived vs today), hash-stripped:", strip(G)==strip(C))
regs=lambda p: sum(len(f["regions"]) for f in json.load(open(p))["data"][0]["functions"])
print("regions: A",regs("$T/A_bin.json"),"C",regs("$T/denominator.json"),"D",regs("$T/D_rlib_nodead.json"),"G",regs("$REPO/results/rq3_coverage/qsort/laertes/raw/denominator.json"))
P
