#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
lock_file="$repo_root/results/baseline_artifacts/artifact_lock.json"
dockerfile="$repo_root/scripts/baseline_artifacts/docker/rustassure.Dockerfile"
log_dir="$repo_root/results/baseline_artifacts/setup/rustassure"
image_tag="c2r-baseline-rustassure:39618406"

mkdir -p "$log_dir"

rustassure_commit=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["rustassure"]["commit"])' "$lock_file")
klee_commit=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["rustassure"]["dependency"]["commit"])' "$lock_file")

docker build \
    --build-arg "RUSTASSURE_COMMIT=$rustassure_commit" \
    --build-arg "RUSTIFY_KLEE_COMMIT=$klee_commit" \
    --file "$dockerfile" \
    --tag "$image_tag" \
    "$(dirname "$dockerfile")" 2>&1 | tee "$log_dir/docker-build.log"

docker inspect "$image_tag" > "$log_dir/docker-inspect.json"
docker run --rm "$image_tag" bash -lc \
    'rustc -vV; clang --version; opt --version; klee --version; git -C /opt/rustassure rev-parse HEAD; git -C /opt/src/rustify-klee rev-parse HEAD' \
    > "$log_dir/toolchain.txt"

printf '%s\n' "$image_tag" > "$log_dir/image-tag.txt"
