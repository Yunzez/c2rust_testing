#!/bin/bash
# E3 depth census for qsort x C2SaferRust.
# Runs AFTER a real coverage-guided libFuzzer run has grown fuzz/corpus/fuzz_target_1.
# Builds an instr-coverage profile over that REAL corpus and reports per-fn entry counts.
set -e
cd "$(dirname "$0")"
export CARGO_TERM_COLOR=never RUST_BACKTRACE=0
cargo +nightly-2025-09-01 fuzz coverage fuzz_target_1 --sanitizer=none -- \
  -timeout=2 -detect_leaks=0 -ignore_crashes=1 -ignore_timeouts=1 >/dev/null 2>&1 || true
SYS=$(rustc +nightly-2025-09-01 --print sysroot)
BIN="$SYS/lib/rustlib/x86_64-unknown-linux-gnu/bin"
PROF=$(find fuzz/coverage -name '*.profdata' | head -1)
TARGET=$(find fuzz/target -name fuzz_target_1 -type f | head -1)
echo "profdata=$PROF"
echo "target=$TARGET"
"$BIN/llvm-cov" export "$TARGET" -instr-profile="$PROF" > /tmp/qsort_sr_cov.json 2>/dev/null
python3 - <<'PY'
import json
d=json.load(open("/tmp/qsort_sr_cov.json"))["data"][0]
fns=["swap","partition","quickSort"]
m={}
for f in d["functions"]:
    leaf=f["name"].split("::")[-1]
    # skip the C oracle (c_quickSort etc.) and harness glue
    if leaf.startswith("c_") or "fuzz_target" in f["name"]: continue
    m[leaf]=m.get(leaf,0)+f.get("count",0)
print("per-fn ENTRY count over REAL fuzz corpus:")
for fn in fns: print(f"  {fn:12s} {m.get(fn,0)}")
t=d["totals"]
print(f"crate cov: fn {t['functions']['percent']:.0f}%  line {t['lines']['percent']:.0f}%")
PY
