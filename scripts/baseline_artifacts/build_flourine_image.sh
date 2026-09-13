#!/usr/bin/env bash
set -euo pipefail

readonly ROOT="/home/yunzez/c2rust_testing"
readonly IMAGE="c2r-baseline-flourine:artifact-20240325"
readonly EVIDENCE="$ROOT/results/baseline_artifacts/setup/flourine"

mkdir -p "$EVIDENCE"
docker build \
  --file "$ROOT/scripts/baseline_artifacts/docker/flourine.Dockerfile" \
  --tag "$IMAGE" \
  "$ROOT/scripts/baseline_artifacts/docker" \
  2>&1 | tee "$EVIDENCE/docker-build.log"

docker image inspect "$IMAGE" > "$EVIDENCE/docker-inspect.json"
printf '%s\n' "$IMAGE" > "$EVIDENCE/image-tag.txt"
docker run --rm "$IMAGE" bash -lc '
  rustc --version
  cargo --version
  cargo bolero --version
  cmake --version | head -n 1
  gcc10-c++ --version | head -n 1
' > "$EVIDENCE/container-toolchain.txt"
