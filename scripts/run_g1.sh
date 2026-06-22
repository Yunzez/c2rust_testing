#!/usr/bin/env bash
# G1 validation: for each "prog:entry", generate a differential harness, build it, fuzz it
# for $DUR seconds on equivalent c2rust output, and report whether any divergence (crash
# artifact) appeared. On equivalent translations the expected result is CLEAN (0 divergence).
#
# The LibAFL libfuzzer shim ignores -runs/-max_total_time, so we stop it by force-killing the
# uniquely-named fuzz binary after $DUR seconds.
#
# usage: DUR=40 scripts/run_g1.sh intmath:intmath_eval rle_codec:rle_encode ...
set -u
export PATH="$HOME/.cargo/bin:$PATH"
ROOT=/home/yunzez/c2rust_testing
cd "$ROOT"
DUR="${DUR:-40}"
TOOLCHAIN=nightly-2025-09-01
declare -A RESULT

for pe in "$@"; do
  prog="${pe%%:*}"; entry="${pe##*:}"
  echo "=== $prog ($entry) ==="
  if ! python3 tools/stu_selector/gen_diff_harness.py --pair "benchmark/pairs/$prog" --entry "$entry" >/dev/null 2>gen.err; then
    echo "  GEN FAIL: $(tail -1 gen.err)"; RESULT[$prog]="GEN_FAIL"; continue
  fi
  pushd "fuzz_gen/$prog" >/dev/null
  if ! cargo +$TOOLCHAIN fuzz build "${prog}_ft" >build.log 2>&1; then
    echo "  BUILD FAIL:"; tail -4 build.log | sed 's/^/    /'; RESULT[$prog]="BUILD_FAIL"; popd >/dev/null; continue
  fi
  rm -rf "fuzz/artifacts/${prog}_ft"
  cargo +$TOOLCHAIN fuzz run "${prog}_ft" >run.log 2>&1 &
  sleep "$DUR"
  pkill -9 -f "fuzz_gen/$prog" 2>/dev/null
  sleep 1
  if ls "fuzz/artifacts/${prog}_ft/"* >/dev/null 2>&1; then
    RESULT[$prog]="DIVERGENCE"
  else
    RESULT[$prog]="CLEAN"
  fi
  echo "  -> ${RESULT[$prog]}"
  popd >/dev/null
done

echo
echo "=== G1 SUMMARY (DUR=${DUR}s each) ==="
for k in "${!RESULT[@]}"; do echo "  $k: ${RESULT[$k]}"; done
