#!/bin/bash
# after fuzzing: build coverage profile over the (grown) corpus, census the paired fns
set -e
export CARGO_TERM_COLOR=never RUST_BACKTRACE=0
cargo +nightly fuzz coverage eval --sanitizer=none -- -timeout=2 -detect_leaks=0 >/dev/null 2>&1 || true
SYS=$(rustc --print sysroot); BIN=$SYS/lib/rustlib/x86_64-unknown-linux-gnu/bin
PROF=$(find fuzz/coverage -name '*.profdata' | head -1)
TARGET=$(find fuzz/target -name eval -type f | head -1)
$BIN/llvm-cov export "$TARGET" -instr-profile="$PROF" > /tmp/lilcov_export.json 2>/dev/null
python3 census.py
