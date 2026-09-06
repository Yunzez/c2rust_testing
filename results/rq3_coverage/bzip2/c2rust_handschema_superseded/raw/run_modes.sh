#!/usr/bin/env bash
# RQ4 — UB-gate ablation by REPLAY, not by re-fuzzing.
#
# Every harness is replayed over the SAME archived campaign corpus in all three C2R_MODE settings.
# One binary, one coverage map, one set of identities: the only difference between the three
# measurements is which regions executed, so no cross-binary identity alignment is involved.
#
#   gated      C runs, UB-tripping inputs rejected before Rust  -> the validator's real coverage
#   nogate     C runs, nothing rejected                          -> ceiling for this corpus
#   rust-only  C never called                                    -> same ceiling, C removed
#
# nogate and rust-only should give IDENTICAL Rust coverage (the two sides use separate buffers, so
# running C cannot change what Rust executes). Any difference is evidence of interference between
# the sides and must be investigated, not averaged away.
set -u
SCR=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
export RUSTUP_TOOLCHAIN=nightly-2025-09-01
OUT=$SCR/mode_exports; rm -rf "$OUT"; mkdir -p "$OUT"
: > "$SCR/logs2/modes.tsv"
printf 'entry\tmode\tinputs\trc\texport_bytes\n' >> "$SCR/logs2/modes.tsv"

for e in $(ls "$SCR/corpus2"); do
  d="$SCR/harness_v3/$e"
  [ -d "$d/fuzz" ] || continue
  grep -q "^$e OK$" "$SCR/logs/build_v3.txt" || continue
  C="$SCR/corpus2/$e"; n=$(ls "$C" | wc -l)
  BIN="$d/target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/bzip2_c2rust_ft"
  PD="$d/fuzz/coverage/bzip2_c2rust_ft/coverage.profdata"
  for m in gated nogate rust-only; do
    ( cd "$d" && C2R_MODE=$m cargo fuzz coverage bzip2_c2rust_ft "$C" ) \
        > "$SCR/logs2/mode_${e}_${m}.log" 2>&1
    rc=$?
    if [ $rc -eq 0 ] && [ -x "$BIN" ] && [ -f "$PD" ]; then
      "$TC/llvm-cov" export "$BIN" -instr-profile="$PD" > "$OUT/${e}__${m}.json" 2>/dev/null
    fi
    printf '%s\t%s\t%s\t%s\t%s\n' "$e" "$m" "$n" "$rc" \
      "$(stat -c%s "$OUT/${e}__${m}.json" 2>/dev/null || echo 0)" >> "$SCR/logs2/modes.tsv"
    echo "$e $m inputs=$n rc=$rc"
  done
  rm -rf "$d/target/x86_64-unknown-linux-gnu/coverage"
done
echo "MODES DONE"
