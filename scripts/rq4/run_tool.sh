#!/usr/bin/env bash
# RQ4 coverage campaign for one bzip2 translation, under the SAME budget the c2rust cell used:
# the two library entry points get an hour each, every other runnable boundary ten minutes, all
# harnesses of one artifact run concurrently. Tools run one after another so each has the machine.
# BZ2_bz__AssertH__fail is never run: the C function calls exit(3), so the boundary is unobservable.
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
mapfile -t OK < <(grep ' OK$' "$SCR/logs/build_$TOOL.txt" | awk '{print $1}' | grep -v AssertH | sort)
mkdir -p "$SCR/corpus_$TOOL" "$SCR/snap_$TOOL" "$SCR/logs_$TOOL"
echo "start $TOOL $(date +%H:%M:%S) with ${#OK[@]} harnesses" > "$SCR/logs_$TOOL/progress.txt"

for e in "${OK[@]}"; do
  case "$e" in BZ2_bzBuffToBuff*) secs=${RQ4_LONG:-3600}; ml=1048576;; *) secs=${RQ4_SHORT:-600}; ml=65536;; esac
  C="$SCR/corpus_$TOOL/$e"; mkdir -p "$C"
  [ -d "$SCR/seeds/$e" ] && cp "$SCR/seeds/$e"/* "$C/" 2>/dev/null
  (
    cd "$SCR/harness_$TOOL/$e" || exit 1
    cargo fuzz run "bzip2_${TOOL}_ft" "$C" -- -seed=42 -max_total_time=$secs -max_len=$ml -rss_limit_mb=8192 -timeout=25 -fork=1 -ignore_crashes=1 -artifact_prefix="$SCR/logs_$TOOL/"
    echo "$e done corpus=$(ls "$C" | wc -l) rc=$?" >> "$SCR/logs_$TOOL/progress.txt"
  ) > "$SCR/logs_$TOOL/fuzz_$e.log" 2>&1 &
done

prev=0
for m in ${RQ4_CKPT:-1 5 10 30 60}; do
  sleep $(( (m-prev)*60 )); prev=$m
  for e in "${OK[@]}"; do [ -d "$SCR/corpus_$TOOL/$e" ] || continue
    cp -al "$SCR/corpus_$TOOL/$e" "$SCR/snap_$TOOL/${e}@${m}min" 2>/dev/null || true; done
  echo "snapshot @${m}min" >> "$SCR/logs_$TOOL/progress.txt"
done
wait
for e in "${OK[@]}"; do printf '%s\t%s\t%s\n' "$e" "$(ls "$SCR/corpus_$TOOL/$e" 2>/dev/null|wc -l)" \
  "$(grep -oE '#[0-9]+' "$SCR/logs_$TOOL/fuzz_$e.log" 2>/dev/null|tr -d '#'|sort -n|tail -1)"; done \
  > "$SCR/logs_$TOOL/campaign.tsv"
echo "CAMPAIGN_${TOOL}_DONE $(date +%H:%M:%S)" >> "$SCR/logs_$TOOL/progress.txt"
