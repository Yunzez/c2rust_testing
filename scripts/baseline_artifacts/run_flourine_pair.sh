#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 DEFECT_ID" >&2
  exit 2
fi

readonly DEFECT_ID="$1"
readonly ROOT="/home/yunzez/c2rust_testing"
readonly BASE="${C2R_BASELINE_DIR:-/home/yunzez/c2rust_baselines}"
readonly ADAPTER="${C2R_ADAPTER_ROOT:-$ROOT/results/baseline_artifacts/adapters/flourine/$DEFECT_ID}"
readonly INPUT_DIR="${C2R_INPUT_DIR:-$ADAPTER/input}"
readonly RUN="${C2R_RUN_ROOT:-$BASE/runs/flourine/pilot_$DEFECT_ID}"
readonly ATTEMPT_NAME="${C2R_ATTEMPT:-attempt-001}"
readonly ATTEMPT="$RUN/$ATTEMPT_NAME"
readonly IMAGE="c2r-baseline-flourine:artifact-20240325"
readonly ARTIFACT="$BASE/checkouts/flourine-artifact"
readonly TOOLS="$BASE/build/flourine-container"
readonly ENGINE_ARGS="${C2R_ENGINE_ARGS:--rss_limit_mb=8096 -max_len=131072}"
readonly CPU_LIMIT="${C2R_CPU_LIMIT:-8}"

test -d "$INPUT_DIR"
test -x "$TOOLS/instrument-c/release/instrument"
test -x "$TOOLS/instrument-rust/release/instrument"
if [[ -e "$ATTEMPT" ]]; then
  echo "refusing to overwrite existing attempt: $ATTEMPT" >&2
  exit 2
fi
mkdir -p "$ATTEMPT"

set +e
docker run --rm \
  --cpus "$CPU_LIMIT" \
  --volume "$ARTIFACT:/artifact:ro" \
  --volume "$TOOLS:/baseline/tools:ro" \
  --volume "$INPUT_DIR:/input:ro" \
  --volume "$ATTEMPT:/output" \
  --env "C2R_ENGINE_ARGS=$ENGINE_ARGS" \
  --env "CARGO_BUILD_JOBS=$CPU_LIMIT" \
  --env "CMAKE_BUILD_PARALLEL_LEVEL=$CPU_LIMIT" \
  "$IMAGE" bash -lc '
    set -euo pipefail
    set -o pipefail
    trap '\''rm -rf /output/verification/target 2>/dev/null || true'\'' EXIT
    sha256sum /input/* > /output/input.sha256
    mkdir /output/verification-tmp
    /baseline/tools/instrument-c/release/instrument \
      -f /input/*.json -o /output/verification-tmp/ground_truth \
      2>&1 | tee /output/instrument-c.log
    cmake -DCMAKE_CXX_COMPILER=gcc10-c++ \
      -S /output/verification-tmp/ground_truth \
      -B /output/verification-tmp/ground_truth/_build \
      2>&1 | tee /output/cmake-configure.log
    cmake --build /output/verification-tmp/ground_truth/_build \
      2>&1 | tee /output/cmake-build.log
    /baseline/tools/instrument-rust/release/instrument \
      -f /input/*.rs -o /output/verification \
      --arbitrary-precision --capture-stdout --wrapper-structs \
      --ground-truth /output/verification-tmp/ground_truth/_build/libground_truth.so \
      --multi-examples 1000 \
      2>&1 | tee /output/instrument-rust.log
    cp /output/verification-tmp/ground_truth/_build/libground_truth.so /output/verification
    mv /output/verification-tmp/ground_truth /output/verification
    rmdir /output/verification-tmp
    export LD_LIBRARY_PATH=/output/verification
    export RUSTFLAGS="-L /output/verification"
    cargo bolero list --manifest-path /output/verification/Cargo.toml \
      2>&1 | tee /output/bolero-list.log
    target=$(cargo bolero list --manifest-path /output/verification/Cargo.toml \
      | jq -r ".test // empty" | head -n 1)
    test -n "$target"
    printf "%s\n" "$target" > /output/target.txt
    timeout --preserve-status 420 \
      cargo bolero test --manifest-path /output/verification/Cargo.toml \
      --features fuzzing "$target" \
      --target-dir /output/verification/target/__fuzz__ \
      --sanitizer NONE \
      --engine-args="$C2R_ENGINE_ARGS" \
      2>&1 | tee /output/verify.log
  ' 2>&1 | tee "$ATTEMPT/console.log"
status=${PIPESTATUS[0]}
set -e
printf '%s\n' "$status" > "$ATTEMPT/exit_code.txt"
exit "$status"
