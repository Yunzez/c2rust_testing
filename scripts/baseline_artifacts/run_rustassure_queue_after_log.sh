#!/usr/bin/env bash
# Start a serial RustAssure expansion lane after a predecessor queue completes.
#
# Each cell is submitted once to the unmodified released artifact.  A failure
# is archived and the lane proceeds to a different defect; it never retries or
# changes the failed input/baseline.
set -u

if [[ $# -lt 4 ]]; then
  echo "usage: $0 WAIT_LOG WAIT_MARKER QUEUE_ID DEFECT_ID..." >&2
  exit 2
fi

readonly WAIT_LOG="$1"
readonly WAIT_MARKER="$2"
readonly QUEUE_ID="$3"
shift 3
readonly ROOT="/home/yunzez/c2rust_testing"
readonly BASE="${C2R_BASELINE_DIR:-/home/yunzez/c2rust_baselines}"
readonly CPU_LIMIT="${C2R_CPU_LIMIT:-8}"
readonly QUEUE_DIR="$BASE/runs/rustassure/queues"
readonly LOG="$QUEUE_DIR/$QUEUE_ID.log"

mkdir -p "$QUEUE_DIR"
if [[ -e "$LOG" ]]; then
  echo "refusing to overwrite queue log: $LOG" >&2
  exit 2
fi

exec > >(tee "$LOG") 2>&1
printf '%s queue=%s wait_log=%s wait_marker=%s cpu_limit=%s defects=%s\n' \
  "$(date -u +%FT%TZ)" "$QUEUE_ID" "$WAIT_LOG" "$WAIT_MARKER" "$CPU_LIMIT" "$*"

while [[ ! -f "$WAIT_LOG" ]] || ! grep -Fq "$WAIT_MARKER" "$WAIT_LOG"; do
  sleep 30
done
printf '%s predecessor_complete marker=%s\n' "$(date -u +%FT%TZ)" "$WAIT_MARKER"

for defect in "$@"; do
  run_dir="$BASE/runs/rustassure/pilot_$defect"
  if [[ -e "$run_dir/workdir" || -s "$run_dir/console.log" ]]; then
    printf '%s collision defect=%s run_dir=%s; lane_stopped_without_overwrite\n' \
      "$(date -u +%FT%TZ)" "$defect" "$run_dir"
    exit 2
  fi
  printf '%s submit defect=%s\n' "$(date -u +%FT%TZ)" "$defect"
  if C2R_CPU_LIMIT="$CPU_LIMIT" \
      "$ROOT/scripts/baseline_artifacts/run_rustassure_pair.sh" "$defect"; then
    status=0
  else
    status=$?
  fi
  printf '%s complete defect=%s exit=%s\n' \
    "$(date -u +%FT%TZ)" "$defect" "$status"
done

printf '%s queue_complete queue=%s\n' "$(date -u +%FT%TZ)" "$QUEUE_ID"
