#!/bin/bash
# Scaffold a pure-Rust E3 depth harness.
#   mk_e3_harness.sh <out_dir> <crate_name> <lib_src.rs>   (fuzz target body read from stdin)
# The stdin target may use `translated::<fn>` (the crate is `use <crate_name> as translated;`).
set -eu
OUT="$1"; CRATE="$2"; LIB="$3"
rm -rf "$OUT"; mkdir -p "$OUT/src" "$OUT/fuzz/fuzz_targets" "$OUT/fuzz/corpus/ft"
cp "$LIB" "$OUT/src/lib.rs"
cat > "$OUT/Cargo.toml" <<TOML
[package]
name = "$CRATE"
version = "0.1.0"
edition = "2021"
[dependencies]
TOML
{
  echo '#![no_main]'
  echo '#![allow(unused, non_snake_case, non_camel_case_types)]'
  echo 'use libfuzzer_sys::fuzz_target;'
  echo "use $CRATE as translated;"
  cat   # target body (fuzz_target!{...}) from stdin
} > "$OUT/fuzz/fuzz_targets/ft.rs"
cat > "$OUT/fuzz/Cargo.toml" <<TOML
[package]
name = "$CRATE-fuzz"
version = "0.0.0"
publish = false
edition = "2021"
[package.metadata]
cargo-fuzz = true
[dependencies]
libfuzzer-sys = "0.4"
[dependencies.$CRATE]
path = ".."
[workspace]
members = ["."]
[profile.release]
debug = 1
[[bin]]
name = "ft"
path = "fuzz_targets/ft.rs"
test = false
doc = false
TOML
echo "scaffolded $OUT (crate $CRATE, target 'ft')"
