#!/usr/bin/env bash
# Replace one completed Docker analysis slot with a serial FLOURINE queue.
#
# This is orchestration only: every scored cell still runs the unmodified
# released artifact through run_flourine_pair.sh.  A failed cell is archived
# once and is never retried; the queue may proceed to a different defect.
set -u

if [[ $# -lt 2 ]]; then
  echo "usage: $0 WAIT_CONTAINER DEFECT_ID..." >&2
  exit 2
fi

readonly WAIT_CONTAINER="$1"
shift
readonly ROOT="/home/yunzez/c2rust_testing"
readonly BASE="${C2R_BASELINE_DIR:-/home/yunzez/c2rust_baselines}"
readonly CPU_LIMIT="${C2R_CPU_LIMIT:-8}"
readonly QUEUE_DIR="$BASE/runs/flourine/queues"
readonly QUEUE_ID="${C2R_QUEUE_ID:-after-${WAIT_CONTAINER#c2r-rustassure-}}"
readonly LOG="$QUEUE_DIR/$QUEUE_ID.log"

mkdir -p "$QUEUE_DIR"
if [[ -e "$LOG" ]]; then
  echo "refusing to overwrite queue log: $LOG" >&2
  exit 2
fi

exec > >(tee "$LOG") 2>&1
printf '%s queue=%s wait_container=%s cpu_limit=%s defects=%s\n' \
  "$(date -u +%FT%TZ)" "$QUEUE_ID" "$WAIT_CONTAINER" "$CPU_LIMIT" "$*"

if ! docker inspect "$WAIT_CONTAINER" >/dev/null 2>&1; then
  echo "wait container does not exist: $WAIT_CONTAINER" >&2
  exit 2
fi

wait_status=$(docker wait "$WAIT_CONTAINER")
printf '%s wait_complete container=%s exit=%s\n' \
  "$(date -u +%FT%TZ)" "$WAIT_CONTAINER" "$wait_status"

for defect in "$@"; do
  printf '%s submit defect=%s\n' "$(date -u +%FT%TZ)" "$defect"
  if C2R_CPU_LIMIT="$CPU_LIMIT" \
      "$ROOT/scripts/baseline_artifacts/run_flourine_pair.sh" "$defect"; then
    status=0
  else
    status=$?
  fi
  printf '%s complete defect=%s exit=%s\n' \
    "$(date -u +%FT%TZ)" "$defect" "$status"
done

printf '%s queue_complete queue=%s\n' "$(date -u +%FT%TZ)" "$QUEUE_ID"
