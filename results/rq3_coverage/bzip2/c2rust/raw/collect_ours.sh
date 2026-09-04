#!/usr/bin/env bash
# RQ4 coverage — phase 2: replay each harness's campaign corpus under coverage instrumentation.
#
# `cargo fuzz coverage` rebuilds the SAME fuzz target with -C instrument-coverage and replays the
# corpus once.  The target is the differential harness, so the C oracle runs first and the UB gate
# still rejects inputs before Rust executes: the Rust coverage collected here is, by construction,
# coverage of the inputs that passed the C gate and reached Rust.
#
# Each harness is exported immediately and its coverage target dir deleted, because the machine has
# ~3 GB free and each coverage build is ~150 MB.
set -u
SCR=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
export RUSTUP_TOOLCHAIN=nightly-2025-09-01
OUT=$SCR/ours_exports; mkdir -p "$OUT"
: > "$SCR/logs/coverage_summary.tsv"
printf 'entry\tcorpus\tcov_rc\texport_bytes\n' >> "$SCR/logs/coverage_summary.tsv"

mapfile -t OK < <(grep ' OK$' "$SCR/logs/build_summary.txt" | awk '{print $1}' | sort)
for e in "${OK[@]}"; do
  d="$SCR/harness/$e"; C="$SCR/corpus/$e"
  cd "$d" || continue
  cargo fuzz coverage bzip2_c2rust_ft "$C" > "$SCR/logs/cov_$e.log" 2>&1
  rc=$?
  BIN="$d/fuzz/target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/bzip2_c2rust_ft"
  [ -x "$BIN" ] || BIN=$(find "$d/fuzz/target" -type f -name bzip2_c2rust_ft -path '*coverage*' 2>/dev/null | head -1)
  PD="$d/fuzz/coverage/bzip2_c2rust_ft/coverage.profdata"
  if [ $rc -eq 0 ] && [ -n "${BIN:-}" ] && [ -f "$PD" ]; then
    "$TC/llvm-cov" export "$BIN" -instr-profile="$PD" > "$OUT/$e.json" 2>>"$SCR/logs/cov_$e.log"
    cp "$PD" "$OUT/$e.profdata"
  fi
  printf '%s\t%s\t%s\t%s\n' "$e" "$(ls "$C" 2>/dev/null | wc -l)" "$rc" \
      "$(stat -c%s "$OUT/$e.json" 2>/dev/null || echo 0)" >> "$SCR/logs/coverage_summary.tsv"
  echo "$e rc=$rc export=$(stat -c%s "$OUT/$e.json" 2>/dev/null || echo 0)"
  rm -rf "$d/fuzz/target/x86_64-unknown-linux-gnu/coverage"
done
echo "COVERAGE DONE"; column -t "$SCR/logs/coverage_summary.tsv"
