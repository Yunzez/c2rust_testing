#!/bin/bash
# E3 per-function EXECUTION-DEPTH runner (total executions over a uniform run-budget).
#   eval_rq3_depth.sh <fuzz_project_dir> <target> <RUNS> <cell_label> [forks] [fn_list]
# Metric: build one -C instrument-coverage binary, run it as a libFuzzer for RUNS iterations
# (ASan OFF; E3 = depth not bug-finding), and read each function's total execution count via
# `llvm-cov`. Key: libFuzzer exits CLEANLY on -runs=N (unlike -max_total_time which _Exit()s and
# skips the profraw flush), so the counts reflect ALL executions, not just the saved corpus.
# Crash cells (hard fault mid-run lose the flush) fall back to per-process corpus replay (a floor).
set -u
PROJ="$1"; TARGET="$2"; RUNS="$3"; LABEL="$4"; FORKS="${5:-1}"; FN_LIST="${6:-}"; EXCL="${7:-}"
NIGHTLY=nightly-2025-09-01
OUT=/home/yunzez/c2rust_testing/results/rq4_effectiveness/reach_cells
CRATE=$(basename "$PROJ")
mkdir -p "$OUT"; cd "$PROJ" || exit 1
ulimit -c 0
shopt -s nullglob
SYS=$(rustc +$NIGHTLY --print sysroot); LLVM="$SYS/lib/rustlib/x86_64-unknown-linux-gnu/bin"

# ---- 0) seed corpus ----
mkdir -p fuzz/corpus/$TARGET
if [ -z "$(ls -A fuzz/corpus/$TARGET 2>/dev/null)" ]; then
  for i in $(seq 1 8); do head -c $((16*i)) /dev/urandom > fuzz/corpus/$TARGET/seed_$i; done
fi

# ---- 1) build the instrument-coverage binary (cargo fuzz coverage builds + does a -runs=0 replay) ----
echo "[$LABEL] building coverage binary"
setsid timeout --kill-after=20 900 \
  cargo +$NIGHTLY fuzz coverage "$TARGET" --sanitizer=none -- -runs=0 -timeout=5 -ignore_crashes=1 >/dev/null 2>&1
BIN=$(find target fuzz/target -path '*coverage*release*' -name "$TARGET" -type f 2>/dev/null | head -1)
if [ -z "$BIN" ]; then echo "[$LABEL] FAIL: no coverage binary"; exit 3; fi

# ---- 2) run it as a libFuzzer for RUNS iterations; clean exit => profraw flushes with TOTAL counts ----
echo "[$LABEL] fuzzing -runs=$RUNS (sanitizer=none)"
PD=/tmp/rq3_${LABEL}_prof; rm -rf "$PD"; mkdir -p "$PD"
LLVM_PROFILE_FILE="$PD/live.profraw" setsid timeout --kill-after=15 200 \
  "$BIN" -runs=$RUNS -rss_limit_mb=4096 -timeout=5 fuzz/corpus/$TARGET >/tmp/rq3_${LABEL}_run.log 2>&1
RC=$?
CORPUS=$(ls fuzz/corpus/$TARGET | wc -l)
MODE="total-exec"
if [ $RC -ne 0 ] || [ ! -s "$PD/live.profraw" ]; then
  # hard-fault (or no flush) -> floor via per-process replay of the grown corpus, merge survivors
  echo "[$LABEL] run rc=$RC / no flush -> crash-cell fallback: per-process corpus replay (floor)"
  MODE="corpus-replay-floor"; rm -f "$PD/live.profraw"; ok=0; crash=0
  for f in fuzz/corpus/$TARGET/*; do
    LLVM_PROFILE_FILE="$PD/$(basename "$f").profraw" timeout 8 "$BIN" "$f" >/dev/null 2>&1
    [ $? -le 1 ] && ok=$((ok+1)) || { crash=$((crash+1)); rm -f "$PD/$(basename "$f").profraw"; }
  done
  echo "[$LABEL] replay survivors=$ok crashers=$crash"
fi
N=$(ls "$PD"/*.profraw 2>/dev/null | wc -l)
if [ "$N" -eq 0 ]; then echo "[$LABEL] FAIL: no profile (crashes on everything?)"; rm -rf target fuzz/target "$PD"; exit 4; fi
"$LLVM/llvm-profdata" merge -sparse "$PD"/*.profraw -o /tmp/rq3_${LABEL}.profdata 2>/dev/null
"$LLVM/llvm-cov" export "$BIN" -instr-profile=/tmp/rq3_${LABEL}.profdata 2>/dev/null > /tmp/rq3_${LABEL}.json

# ---- 3) median + min over this cell's translated functions ----
python3 - "$LABEL" "$CORPUS" "$RUNS" "$CRATE" "$FN_LIST" "$MODE" "$EXCL" <<'PY'
import json, sys, statistics
label, corpus, runs, crate, fn_list, mode, excl = sys.argv[1:8]
corpus, runs = int(corpus), int(runs)
d = json.load(open(f"/tmp/rq3_{label}.json"))["data"][0]
needle = f"{crate}/src/"
wanted = [w for w in fn_list.split(",") if w]
rows=[]
for f in d["functions"]:
    fns=f.get("filenames",[])
    if not any(needle in p for p in fns): continue
    if excl and any(excl in p for p in fns): continue
    nm=f["name"]; leaf=nm.split("::")[-1]
    if wanted:
        hit=next((w for w in wanted if w in nm), None)
        if not hit: continue
        rows.append((hit, f.get("count",0)))
    else:
        if nm.startswith("_R") or leaf.startswith("c_") or leaf=="__assert_rtn": continue
        rows.append((leaf, f.get("count",0)))
agg={}
for k,v in rows: agg[k]=max(agg.get(k,0), v)
rows=list(agg.items())
# median over the functions the harness actually DRIVES (count>0). Unreached functions are separate
# API surfaces (e.g. bzip2's FILE-I/O BZ2_bzRead/Write) not exercised by this entry; reported as the
# reached/total split, not folded into the median (that would understate depth on the tested surface).
reached=[(k,c) for k,c in rows if c>0]
rc=[c for _,c in reached]
res={"cell":label,"library":label.split("__")[0],"tool":label.split("__")[-1],
     "metric":mode,"runs_budget":runs,"corpus":corpus,"theirs":0,
     "n_functions":len(rows),"n_reached":len(reached),
     "median_hits":statistics.median(rc) if rc else 0,
     "min_hits":min(rc) if rc else 0,"max_hits":max(rc) if rc else 0,
     "per_fn":dict(sorted(rows,key=lambda x:-x[1]))}
json.dump(res,open(f"/home/yunzez/c2rust_testing/results/rq4_effectiveness/reach_cells/{label}.json","w"),indent=1)
print(f"[{label}] {mode} fns={res['n_functions']} MEDIAN={res['median_hits']:,} min={res['min_hits']:,} max={res['max_hits']:,} vs theirs=0")
PY

rm -rf target fuzz/target "$PD"
echo "[$LABEL] done, targets reclaimed"
