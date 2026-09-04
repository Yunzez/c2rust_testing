#!/usr/bin/env bash
# RQ4 coverage — phase 2, round 2: replay every corpus SNAPSHOT under coverage instrumentation.
#
# For each harness the coverage binary is built once and then each of the 1/5/10/30/60-minute
# snapshots is replayed through it, so the artifact-level coverage curve can be computed as the
# union over harnesses at each checkpoint. Exports are written immediately and the coverage target
# directory is deleted per harness (~3 GB free on this machine).
set -u
SCR=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
export RUSTUP_TOOLCHAIN=nightly-2025-09-01
OUT=$SCR/ours_exports2; rm -rf "$OUT"; mkdir -p "$OUT"
: > "$SCR/logs2/coverage2.tsv"
printf 'entry\tminute\tinputs\trc\texport_bytes\n' >> "$SCR/logs2/coverage2.tsv"

for e in $(ls "$SCR/corpus2"); do
  d="$SCR/harness/$e"
  BIN="$d/target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/bzip2_c2rust_ft"
  PD="$d/fuzz/coverage/bzip2_c2rust_ft/coverage.profdata"
  for m in 1 5 10 30 60; do
    C="$SCR/snap/${e}@${m}min"
    [ -d "$C" ] || continue
    n=$(ls "$C" | wc -l)
    ( cd "$d" && cargo fuzz coverage bzip2_c2rust_ft "$C" ) > "$SCR/logs2/cov_${e}_${m}.log" 2>&1
    rc=$?
    if [ $rc -eq 0 ] && [ -x "$BIN" ] && [ -f "$PD" ]; then
      "$TC/llvm-cov" export "$BIN" -instr-profile="$PD" > "$OUT/${e}@${m}min.json" 2>/dev/null
    fi
    printf '%s\t%s\t%s\t%s\t%s\n' "$e" "$m" "$n" "$rc" \
      "$(stat -c%s "$OUT/${e}@${m}min.json" 2>/dev/null || echo 0)" >> "$SCR/logs2/coverage2.tsv"
    echo "$e @${m}min inputs=$n rc=$rc"
  done
  rm -rf "$d/target/x86_64-unknown-linux-gnu/coverage"
done
echo "COVERAGE2 DONE"
