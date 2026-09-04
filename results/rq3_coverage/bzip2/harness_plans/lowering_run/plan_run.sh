#!/bin/bash
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
mkdir -p $S/plan_runs
for b in $S/plan_bin/*; do
  e=$(basename "$b")
  mkdir -p "$S/plan_corpus/$e"
  timeout 80 "$b" "$S/plan_corpus/$e" -max_total_time=45 -rss_limit_mb=4096 \
      -print_final_stats=1 > "$S/plan_runs/$e.log" 2>&1
  rc=$?
  runs=$(grep -o 'stat::number_of_executed_units: *[0-9]*' "$S/plan_runs/$e.log" | tail -1 | tr -dc 0-9)
  div=$(grep -c 'divergence:' "$S/plan_runs/$e.log")
  san=$(grep -cE 'ERROR: (AddressSanitizer|libFuzzer)' "$S/plan_runs/$e.log")
  printf "%-30s rc=%-3s runs=%-9s divergence=%s sanitizer/libfuzzer=%s\n" "$e" "$rc" "${runs:-0}" "$div" "$san"
done
echo RUNSDONE
