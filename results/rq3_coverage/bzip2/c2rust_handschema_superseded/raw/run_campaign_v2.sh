#!/usr/bin/env bash
# RQ4 coverage — bzip2 x c2rust campaign, round 2 (corrected input model).
#
# Budget, per the plan: the two library entry points get an hour each, every other runnable
# boundary gets ten minutes. The harnesses run CONCURRENTLY (32 cores), so the artifact's wall
# clock is one hour rather than three.
#
# Corpus snapshots are taken at 1/5/10/30/60 minutes so the coverage curve shows when the campaign
# saturates. Snapshots are hardlink copies, so they cost no disk.
#
# -timeout=25 caps a single input; round 1 lost 1202 s of a 327 s slice to one hanging input.
set -u
SCR=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov
export RUSTUP_TOOLCHAIN=nightly-2025-09-01

LONG="BZ2_bzBuffToBuffCompress BZ2_bzBuffToBuffDecompress"
SHORT="BZ2_hbAssignCodes BZ2_hbCreateDecodeTables BZ2_hbMakeCodeLengths BZ2_indexIntoF fallbackSort fallbackQSort3 fallbackSimpleSort mmed3"
# BZ2_bz__AssertH__fail is excluded: the C function calls exit(3) on every input, so the boundary
# is not observable. It is reported as EXECUTION-FAILED, never as 0 % coverage.

rm -rf "$SCR/corpus2" "$SCR/snap" "$SCR/artifacts2"
mkdir -p "$SCR/corpus2" "$SCR/snap" "$SCR/artifacts2" "$SCR/logs2"

run_one() {
  local e=$1 secs=$2 maxlen=$3
  local C="$SCR/corpus2/$e"; mkdir -p "$C"
  [ -d "$SCR/seeds/$e" ] && cp "$SCR/seeds/$e"/* "$C/" 2>/dev/null
  mkdir -p "$SCR/artifacts2/$e"
  ( cd "$SCR/harness/$e" && \
    cargo fuzz run bzip2_c2rust_ft "$C" -- -seed=42 -max_total_time="$secs" \
      -max_len="$maxlen" -rss_limit_mb=8192 -timeout=25 -fork=1 -ignore_crashes=1 \
      -artifact_prefix="$SCR/artifacts2/$e/" ) > "$SCR/logs2/fuzz_$e.log" 2>&1
  echo "$e done: corpus=$(ls "$C" | wc -l)" >> "$SCR/logs2/progress.txt"
}

snapshot() {   # snapshot every live corpus at minute $1
  local m=$1
  for e in $LONG $SHORT; do
    [ -d "$SCR/corpus2/$e" ] || continue
    cp -al "$SCR/corpus2/$e" "$SCR/snap/${e}@${m}min" 2>/dev/null || \
      cp -a "$SCR/corpus2/$e" "$SCR/snap/${e}@${m}min"
  done
  echo "snapshot @${m}min: $(du -sh "$SCR/snap" | cut -f1)" >> "$SCR/logs2/progress.txt"
}

echo "start $(date +%H:%M:%S)" > "$SCR/logs2/progress.txt"
for e in $LONG;  do run_one "$e" 3600 1048576 & done
for e in $SHORT; do run_one "$e"  600   65536 & done

prev=0
for m in 1 5 10 30 60; do
  sleep $(( (m - prev) * 60 )); prev=$m
  snapshot "$m"
done
wait
echo "CAMPAIGN2 DONE $(date +%H:%M:%S)" >> "$SCR/logs2/progress.txt"
for e in $LONG $SHORT; do
  printf '%s\t%s\t%s\n' "$e" "$(ls "$SCR/corpus2/$e" 2>/dev/null | wc -l)" \
    "$(grep -oE '#[0-9]+' "$SCR/logs2/fuzz_$e.log" | tr -d '#' | sort -n | tail -1)"
done > "$SCR/logs2/campaign2.tsv"
cat "$SCR/logs2/campaign2.tsv"
