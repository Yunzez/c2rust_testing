#!/bin/bash
# E3 per-function hit-DEPTH runner (hardened for the parallel batch).
#   eval_rq3_depth.sh <fuzz_project_dir> <target> <fuzz_secs> <cell_label> [forks]
# Pure-Rust depth. Robust to CRASH cells: coverage is measured by PER-PROCESS REPLAY of the grown
# corpus through the coverage binary, merging survivors (clean cells simply have 0 crashers). Process
# hygiene: the fuzz run is wrapped in `timeout` in its own process group and hard-killed by pgid.
set -u
PROJ="$1"; TARGET="$2"; SECS="$3"; LABEL="$4"; FORKS="${5:-1}"; FN_LIST="${6:-}"
# FN_LIST (optional): comma-sep translated-fn names for this cell (e.g. swap,partition,quick_sort).
# Match by substring — works for no_mangle plain names AND idiomatic mangled symbols, and excludes
# tool runtime shims. Omit for c2rust-family (no_mangle) where the default plain-name filter suffices.
NIGHTLY=nightly-2025-09-01
OUT=/home/yunzez/c2rust_testing/results/rq3_cells
CRATE=$(basename "$PROJ")
mkdir -p "$OUT"
cd "$PROJ" || exit 1
ulimit -c 0   # no core dumps (crash cells abort a lot)

shopt -s nullglob   # empty corpus glob must expand to nothing, not a literal '*'

# ---- 0) seed the corpus if empty so coverage-guided fuzzing has a starting point ----
mkdir -p fuzz/corpus/$TARGET
if [ -z "$(ls -A fuzz/corpus/$TARGET 2>/dev/null)" ]; then
  echo "[$LABEL] empty corpus -> planting 8 seeds"
  for i in $(seq 1 8); do head -c $((16*i)) /dev/urandom > fuzz/corpus/$TARGET/seed_$i 2>/dev/null; done
fi

# ---- 1) coverage-guided fuzz, ASan OFF (E3 = depth not bug-finding; faster + deeper corpus on
#         non-hard-fault bugs), hard time-boxed + process-group-killed ----
echo "[$LABEL] fuzz ${SECS}s forks=$FORKS (sanitizer=none)"
setsid timeout --kill-after=20 $((SECS+120)) \
  cargo +$NIGHTLY fuzz run "$TARGET" --sanitizer=none -- \
    -fork=$FORKS -ignore_crashes=1 -ignore_ooms=1 -ignore_timeouts=1 \
    -max_total_time=$SECS -rss_limit_mb=4096 -timeout=5 >/dev/null 2>&1 &
FPGID=$!
wait $FPGID 2>/dev/null
# belt-and-suspenders: kill any stragglers in that group + any orphan workers of this target
kill -9 -$FPGID 2>/dev/null
pkill -9 -f "target/.*release/$TARGET .*$CRATE" 2>/dev/null
CORPUS=$(ls fuzz/corpus/$TARGET 2>/dev/null | wc -l)
echo "[$LABEL] corpus=$CORPUS"

# ---- 2) build the coverage binary (its own batch merge may fail on crash cells — we don't use it) ----
echo "[$LABEL] building coverage binary"
setsid timeout --kill-after=20 900 \
  cargo +$NIGHTLY fuzz coverage "$TARGET" --sanitizer=none -- \
    -timeout=5 -ignore_crashes=1 -ignore_timeouts=1 -runs=0 >/dev/null 2>&1
BIN=$(find target fuzz/target -path '*coverage*release*' -name "$TARGET" -type f 2>/dev/null | head -1)
if [ -z "$BIN" ]; then echo "[$LABEL] FAIL: no coverage binary built"; exit 3; fi

# ---- 3) PER-PROCESS replay: one corpus input per process, keep only survivors' profraws ----
SYS=$(rustc +$NIGHTLY --print sysroot); LLVM="$SYS/lib/rustlib/x86_64-unknown-linux-gnu/bin"
PD=/tmp/rq3_${LABEL}_prof; rm -rf "$PD"; mkdir -p "$PD"
ok=0; crash=0
for f in fuzz/corpus/$TARGET/*; do
  LLVM_PROFILE_FILE="$PD/$(basename "$f").profraw" timeout 8 "$BIN" "$f" >/dev/null 2>&1
  rc=$?
  if [ $rc -le 1 ]; then ok=$((ok+1)); else crash=$((crash+1)); rm -f "$PD/$(basename "$f").profraw"; fi
done
echo "[$LABEL] replay: survivors=$ok crashers=$crash"
N=$(ls "$PD"/*.profraw 2>/dev/null | wc -l)
if [ "$N" -eq 0 ]; then echo "[$LABEL] FAIL: 0 survivors — harness crashes on every input (check output-buffer caps / entry contract)"; exit 4; fi
if [ "$CORPUS" -le 8 ]; then echo "[$LABEL] WARN: corpus did not grow past seeds ($CORPUS) — harness may crash early or entry unreached"; fi
"$LLVM/llvm-profdata" merge -sparse "$PD"/*.profraw -o /tmp/rq3_${LABEL}.profdata 2>/dev/null
"$LLVM/llvm-cov" export "$BIN" -instr-profile=/tmp/rq3_${LABEL}.profdata 2>/dev/null > /tmp/rq3_${LABEL}.json

# ---- 4) median + min over THIS crate's translated functions ----
python3 - "$LABEL" "$CORPUS" "$SECS" "$FORKS" "$CRATE" "$ok" "$crash" "$FN_LIST" <<'PY'
import json, sys, statistics
label, corpus, secs, forks, crate, surv, crash, fn_list = sys.argv[1:9]
corpus, secs, forks, surv, crash = map(int,(corpus,secs,forks,surv,crash))
d = json.load(open(f"/tmp/rq3_{label}.json"))["data"][0]
needle = f"{crate}/src/"   # crate-specific — avoids libfuzzer_sys/arbitrary own lib.rs
wanted = [w for w in fn_list.split(",") if w]
rows=[]
for f in d["functions"]:
    if not any(needle in p for p in f.get("filenames",[])): continue
    nm = f["name"]; leaf = nm.split("::")[-1]
    if wanted:
        # explicit per-cell list: match by substring (no_mangle exact OR mangled symbol contains it)
        hit = next((w for w in wanted if w in nm), None)
        if not hit: continue
        rows.append((hit, f.get("count",0)))
    else:
        # default: translated C fns = #[no_mangle] plain names; drop Rust-mangled _R* (tool runtime
        # shim, e.g. Laertes CustomSlice/Get helpers) + c_* oracle. Correct for c2rust-family.
        if nm.startswith("_R") or leaf.startswith("c_") or leaf == "__assert_rtn": continue
        rows.append((leaf, f.get("count",0)))
# collapse duplicate substring hits (mangled monomorphisations) to the MAX count per fn
agg={}
for k,v in rows: agg[k]=max(agg.get(k,0), v)
rows=list(agg.items())
counts=[c for _,c in rows]
res={"cell":label,"library":label.split("__")[0],"tool":label.split("__")[-1],
     "n_functions":len(rows),"corpus":corpus,"survivors":surv,"crashers":crash,
     "fuzz_secs":secs,"forks":forks,"theirs":0,
     "median_hits":statistics.median(counts) if counts else 0,
     "min_hits":min(counts) if counts else 0,
     "max_hits":max(counts) if counts else 0,
     "per_fn":dict(sorted(rows,key=lambda x:-x[1]))}
json.dump(res,open(f"/home/yunzez/c2rust_testing/results/rq3_cells/{label}.json","w"),indent=1)
print(f"[{label}] fns={res['n_functions']} MEDIAN={res['median_hits']} min={res['min_hits']} max={res['max_hits']} | corpus={corpus} surv={surv} crash={crash} | theirs=0")
PY

# ---- 5) reclaim the ~9GB of build targets, keep corpus + JSON ----
rm -rf target fuzz/target "$PD"
echo "[$LABEL] done, targets reclaimed"
