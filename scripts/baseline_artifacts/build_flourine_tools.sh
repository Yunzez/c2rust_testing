#!/usr/bin/env bash
set -euo pipefail

readonly BASE_DIR="${C2R_BASELINE_DIR:-/home/yunzez/c2rust_baselines}"
readonly ARCHIVE="$BASE_DIR/downloads/flourine-artifact.tar.gz"
readonly SOURCE="$BASE_DIR/checkouts/flourine-artifact/Differential_Tester"
readonly BUILD="$BASE_DIR/build/flourine"
readonly EVIDENCE="/home/yunzez/c2rust_testing/results/baseline_artifacts/setup/flourine"
readonly EXPECTED_SHA256="87e570d4ee6dabd6905d6573f8da10d1645051e521d5ee9d6a19704cd87f0a32"

test -f "$ARCHIVE"
test -f "$SOURCE/instrumentor/Cargo.lock"
test -f "$SOURCE/c-instrumentor/Cargo.lock"

actual_sha256="$(sha256sum "$ARCHIVE" | awk '{print $1}')"
if [[ "$actual_sha256" != "$EXPECTED_SHA256" ]]; then
  echo "FLOURINE artifact checksum mismatch: $actual_sha256" >&2
  exit 1
fi

mkdir -p "$BUILD" "$EVIDENCE"

{
  date -u +'%Y-%m-%dT%H:%M:%SZ'
  printf 'artifact_sha256=%s\n' "$actual_sha256"
  printf 'rustc=%s\n' "$(rustc --version --verbose | tr '\n' ';')"
  printf 'cargo=%s\n' "$(cargo --version --verbose | tr '\n' ';')"
  printf 'host_cxx=%s\n' "$(g++ --version | head -n 1)"
  sha256sum \
    "$SOURCE/instrumentor/Cargo.lock" \
    "$SOURCE/c-instrumentor/Cargo.lock"
} > "$EVIDENCE/toolchain.txt"

cargo build --release --locked \
  --manifest-path "$SOURCE/instrumentor/Cargo.toml" \
  --target-dir "$BUILD/instrument-rust" \
  2>&1 | tee "$EVIDENCE/instrument-rust-build.log"

cargo build --release --locked \
  --manifest-path "$SOURCE/c-instrumentor/Cargo.toml" \
  --target-dir "$BUILD/instrument-c" \
  2>&1 | tee "$EVIDENCE/instrument-c-build.log"

test -x "$BUILD/instrument-rust/release/instrument"
test -x "$BUILD/instrument-c/release/instrument"

{
  sha256sum \
    "$BUILD/instrument-rust/release/instrument" \
    "$BUILD/instrument-c/release/instrument"
  file \
    "$BUILD/instrument-rust/release/instrument" \
    "$BUILD/instrument-c/release/instrument"
} > "$EVIDENCE/binaries.txt"
