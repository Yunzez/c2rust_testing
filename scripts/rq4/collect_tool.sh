#!/usr/bin/env bash
# RQ4 coverage collection for one artifact.
#
# The measurement is RUST coverage, so the replay runs with C2R_MODE=rust-only: the C reference is
# not called, there is no gate and no comparison. C's contribution is to the CAMPAIGN (its sancov
# edges guide the fuzzer and its UB gate excludes incomparable inputs); replaying the corpus it
# produced needs only the Rust side. This is measured, not assumed: the three-mode ablation on
# bzip2 x c2rust gave identical coverage under gated / nogate / rust-only (45 fns, 7018 regions;
# gate cost 0), results/rq3_coverage/bzip2/c2rust/ubgate_ablation.json.
#
# rust-only removes divergence panics but NOT a crash in the translation itself, so a failed batch
# replay falls back to one process per input and merges whatever each produced.
#
# PROMOTED OUT OF A SESSION SCRATCHPAD, 2026-09-04. This was the only copy: the archived bzip2
# c2rust cell keeps its own hardcoded variants under `raw/`, and nothing generic existed in the
# repo. `SCR` is now the working directory, taken from $RQ4_WORK, and defaults to the scratchpad
# it came from so an in-flight run still resolves.
#
# bzip2-specific: the acceptance suite in tests_side.sh, and the `bzip2_<tool>_ft` target name.
# Everything else is per-tool generic.
set -u
SCR=${RQ4_WORK:-"/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov"}
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
export RUSTUP_TOOLCHAIN=nightly-2025-09-01 C2R_MODE=rust-only
TOOL=$1
OUT=$SCR/ours_$TOOL; rm -rf "$OUT"; mkdir -p "$OUT"
: > "$SCR/logs/cov_$TOOL.tsv"
printf 'entry\tinputs\tmode\trc\texport_bytes\n' >> "$SCR/logs/cov_$TOOL.tsv"
for e in $(ls "$SCR/corpus_$TOOL"); do
  d="$SCR/harness_$TOOL/$e"; C="$SCR/corpus_$TOOL/$e"; n=$(ls "$C" | wc -l)
  grep -q "^$e OK$" "$SCR/logs/build_$TOOL.txt" || { printf '%s\t%s\t%s\t%s\t%s\n' "$e" "$n" "not-built" "-" 0 >> "$SCR/logs/cov_$TOOL.tsv"; continue; }
  [ "$n" -eq 0 ] && { printf '%s\t%s\t%s\t%s\t%s\n' "$e" "$n" "empty-corpus" "-" 0 >> "$SCR/logs/cov_$TOOL.tsv"; continue; }
  ( cd "$d" && cargo fuzz coverage "bzip2_${TOOL}_ft" "$C" ) > "$SCR/logs/cov_${TOOL}_${e}.log" 2>&1
  rc=$?; mode=batch
  BIN="$d/target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/bzip2_${TOOL}_ft"
  PD="$d/fuzz/coverage/bzip2_${TOOL}_ft/coverage.profdata"
  if [ $rc -ne 0 ] && [ -x "$BIN" ]; then
    # the translation itself crashed on some input; replay one process per input and keep the rest
    mode=per-input; P="$d/percov"; rm -rf "$P"; mkdir -p "$P"; ok=0
    for f in "$C"/*; do LLVM_PROFILE_FILE="$P/%m-%p.profraw" timeout 60 "$BIN" -runs=1 "$f" >/dev/null 2>&1 && ok=$((ok+1)); done
    if ls "$P"/*.profraw >/dev/null 2>&1; then
      "$TC/llvm-profdata" merge -sparse "$P"/*.profraw -o "$P/coverage.profdata" 2>/dev/null && PD="$P/coverage.profdata" && rc=0
    fi
    echo "$e per-input fallback: $ok/$n inputs completed" >> "$SCR/logs/cov_${TOOL}_${e}.log"
  fi
  [ $rc -eq 0 ] && [ -x "$BIN" ] && [ -f "$PD" ] && "$TC/llvm-cov" export "$BIN" -instr-profile="$PD" > "$OUT/$e.json" 2>/dev/null
  printf '%s\t%s\t%s\t%s\t%s\n' "$e" "$n" "$mode" "$rc" "$(stat -c%s "$OUT/$e.json" 2>/dev/null || echo 0)" >> "$SCR/logs/cov_$TOOL.tsv"
  echo "$TOOL $e n=$n mode=$mode rc=$rc"
  rm -rf "$d/target/x86_64-unknown-linux-gnu/coverage" "$d/percov"
done
echo "COLLECT_${TOOL}_DONE"
