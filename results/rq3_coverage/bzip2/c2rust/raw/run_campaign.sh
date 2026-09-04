#!/usr/bin/env bash
# RQ4 coverage — differential fuzzing campaign for one artifact, under ONE fixed wall-clock budget.
#
# The artifact-level budget is split evenly across the harnesses that BUILD, and the harnesses run
# SEQUENTIALLY so each really gets its slice of one machine.  Every harness is the frozen
# differential pipeline: decode one logical input -> C oracle under UBSan -> UB gate -> Rust ->
# compare.  Inputs on which C trips UB are rejected before Rust runs.
set -u
SCR=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov
BUDGET=${BUDGET:-3600}
export RUSTUP_TOOLCHAIN=nightly-2025-09-01

mapfile -t OK < <(grep ' OK$' "$SCR/logs/build_summary.txt" | awk '{print $1}' | sort)
N=${#OK[@]}
SLICE=$(( BUDGET / N ))
echo "artifact budget ${BUDGET}s / ${N} runnable harnesses = ${SLICE}s each"
: > "$SCR/logs/campaign.tsv"
printf 'entry\tslice_s\tactual_s\tcorpus_inputs\texec_total\trc\n' >> "$SCR/logs/campaign.tsv"

for e in "${OK[@]}"; do
  d="$SCR/harness/$e"
  C="$SCR/corpus/$e"; mkdir -p "$C"
  A="$SCR/artifacts/$e"; mkdir -p "$A"
  cd "$d" || continue
  t0=$(date +%s)
  cargo fuzz run bzip2_c2rust_ft "$C" -- \
      -seed=42 -max_total_time="$SLICE" -max_len=4096 -rss_limit_mb=4096 \
      -fork=1 -ignore_crashes=1 -artifact_prefix="$A/" \
      > "$SCR/logs/fuzz_$e.log" 2>&1
  rc=$?
  t1=$(date +%s)
  execs=$(grep -oE '#[0-9]+' "$SCR/logs/fuzz_$e.log" | tr -d '#' | sort -n | tail -1)
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$e" "$SLICE" "$((t1-t0))" "$(ls "$C" | wc -l)" "${execs:-0}" "$rc" \
      >> "$SCR/logs/campaign.tsv"
  echo "$e: $((t1-t0))s corpus=$(ls "$C" | wc -l) rc=$rc"
done
echo "CAMPAIGN DONE"
column -t "$SCR/logs/campaign.tsv"
