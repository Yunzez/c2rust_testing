#!/usr/bin/env bash
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
TOOL=$1
export RUSTUP_TOOLCHAIN=nightly-2025-09-01
# keep the campaign corpora; regenerate only the harness sources, then rebuild the fuzz binaries
RQ4_SCHEMA_DIR=$SCR/schemas_$TOOL python3 $SCR/gen_harnesses.py $SCR/pair/bzip2_$TOOL \
    $SCR/eligibility_bzip2_$TOOL.json $SCR/harness_$TOOL > /dev/null 2>&1
[ "$TOOL" = crown ] && for d in $SCR/harness_crown/*/; do
  [ -f "$d/Cargo.toml" ] && grep -q '^libc' "$d/Cargo.toml" || sed -i 's|^\[dependencies\]$|[dependencies]\nlibc = "0.2"|' "$d/Cargo.toml"; done
: > $SCR/logs/build_$TOOL.txt
for d in $SCR/harness_$TOOL/*/; do e=$(basename $d); [ -d "$d/fuzz" ] || continue
  ( cd "$d" && cargo fuzz build "bzip2_${TOOL}_ft" ) >/tmp/rb_$TOOL.log 2>&1 \
    && echo "$e OK" >> $SCR/logs/build_$TOOL.txt \
    || echo "$e FAIL :: $(grep -m1 -E 'undefined symbol|^error(\[[A-Z0-9]+\])?:' /tmp/rb_$TOOL.log|sed 's/^ *//'|head -c 70)" >> $SCR/logs/build_$TOOL.txt
done
echo "REBUILD_${TOOL}: $(grep -c ' OK$' $SCR/logs/build_$TOOL.txt)/14"
