#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 DEFECT_ID" >&2
  exit 2
fi

readonly DEFECT_ID="$1"
readonly ROOT="/home/yunzez/c2rust_testing"
readonly BASE="${C2R_BASELINE_DIR:-/home/yunzez/c2rust_baselines}"
readonly ADAPTER="$ROOT/results/baseline_artifacts/adapters/rustassure/$DEFECT_ID"
readonly RUN="$BASE/runs/rustassure/pilot_$DEFECT_ID"
readonly IMAGE="c2r-baseline-rustassure:39618406"
readonly CONTAINER="c2r-rustassure-${DEFECT_ID,,}"
readonly OFFICIAL_MAP="/opt/rustassure/src/Symbolizer/scripts/map/2/argument_order_map.json"
readonly CPU_LIMIT="${C2R_CPU_LIMIT:-8}"

test -d "$ADAPTER/input"
test -f "$ADAPTER/argument_order_map.json"
mkdir -p "$RUN"

docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
docker run --rm --name "$CONTAINER" \
  --cpus "$CPU_LIMIT" \
  --volume "$ADAPTER/input:/input:ro" \
  --volume "$ADAPTER/argument_order_map.json:$OFFICIAL_MAP:ro" \
  --volume "$RUN:/archive" \
  "$IMAGE" \
  bash -lc '
    set -o pipefail
    sha256sum /input/* > /archive/input.sha256
    sha256sum /opt/rustassure/src/Symbolizer/scripts/map/2/argument_order_map.json > /archive/argument_order_map.sha256
    archive_run() {
      set +e
      latest=$(find /opt/rustassure/src/Symbolizer -maxdepth 1 -type d -name "perform_general_execution_*" -printf "%T@ %p\n" | sort -nr | head -1 | cut -d" " -f2-)
      if test -n "$latest"; then
        rm -rf /archive/workdir
        cp -a "$latest" /archive/workdir
      fi
    }
    trap archive_run EXIT
    python3 performSymbolExecution.py --src=/input
  ' 2>&1 | tee "$RUN/console.log"
