#!/usr/bin/env bash
# Matcher/analyzer regression gate.
#
# Runs the FULL name-independent matching pipeline on committed FAITHFUL c2rust
# fixtures (names preserved -> name-equality is ground truth) and fails if accuracy
# drops below the recorded baseline. This is the safety net for changes to
# matcher.py or the rust-analyzer `analyzer` -- in particular the deferred df-cap /
# hub-aware topology, which could weaken the homogeneous-cluster topology that lil
# (55 identical `fnc_*` handlers) depends on.
#
# Usage: bash scripts/matcher_regression.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AN="$ROOT/tools/stu_selector/analyzer/target/release/analyzer"
PAIRS="$ROOT/benchmark/pairs"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

[ -x "$AN" ] || { echo "analyzer not built: (cd tools/stu_selector/analyzer && cargo build --release)" >&2; exit 1; }

# fixture: name  c_source_subpath  crate_dir  min_correct  total
FIXTURES=(
  "lil source/lil.c translated 126 128"
)

fail=0
for fx in "${FIXTURES[@]}"; do
  read -r name csrc crate minc total <<<"$fx"
  dir="$PAIRS/$name"
  # regenerate compile_commands.json with this machine's abspath (robust to checkout location)
  cfile="$dir/$csrc"
  printf '[{"directory":"%s/build","file":"%s","arguments":["clang","-c","%s"]}]\n' \
    "$dir" "$cfile" "$cfile" > "$dir/build/compile_commands.json"

  "$AN" "$dir/$crate" --enable-metrics > "$TMP/$name.rust.json" 2>/dev/null
  python3 "$ROOT/tools/stu_selector/c_analyzer.py" \
    --compile-commands "$dir/build" --enable-metrics > "$TMP/$name.c.json" 2>/dev/null
  line=$(python3 "$ROOT/tools/stu_selector/matcher.py" \
    --c "$TMP/$name.c.json" --rust "$TMP/$name.rust.json" 2>&1 | grep '^CORRECT')
  got=$(echo "$line" | grep -oE 'CORRECT \([^)]*\): [0-9]+' | grep -oE '[0-9]+$')

  if [ "${got:-0}" -ge "$minc" ]; then
    echo "  PASS  $name: $got/$total (>= baseline $minc)"
  else
    echo "  FAIL  $name: $got/$total (< baseline $minc)  <<< REGRESSION"
    fail=1
  fi
done

[ "$fail" -eq 0 ] && echo "regression gate: PASS" || { echo "regression gate: FAIL" >&2; exit 1; }
