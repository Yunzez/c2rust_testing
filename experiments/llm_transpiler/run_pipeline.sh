#!/usr/bin/env bash
# Run the analyzer + c_analyzer + matcher on a transpiled crate.
# Usage: run_pipeline.sh <pair>   (after transpile.py --pair <pair>)
set -euo pipefail

PAIR="${1:?usage: run_pipeline.sh <pair>}"
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
OUT="$HERE/out/$PAIR"
AN="$ROOT/tools/stu_selector/analyzer/target/release/analyzer"

[ -d "$OUT" ] || { echo "no crate at $OUT — run transpile.py --pair $PAIR first" >&2; exit 1; }

echo "== Rust analyzer (rust-analyzer) =="
"$AN" "$OUT" --enable-metrics > "$HERE/out/$PAIR.rust.json"

echo "== C analyzer (libclang) =="
python3 "$ROOT/tools/stu_selector/c_analyzer.py" \
    --compile-commands "$ROOT/benchmark/pairs/$PAIR/build" --enable-metrics \
    > "$HERE/out/$PAIR.c.json"

echo "== matcher (names hidden) =="
python3 "$ROOT/tools/stu_selector/matcher.py" \
    --c "$HERE/out/$PAIR.c.json" --rust "$HERE/out/$PAIR.rust.json" -v
