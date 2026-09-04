#!/usr/bin/env bash
# Run bzip2's shipped acceptance suite against ONE translated artifact, under the same coverage
# instrumentation the ours side uses, and export the result. The suite is unchanged: the Makefile's
# six commands and six cmps. The only adapter is a three-line bin that calls the translated CLI's
# own main, which the crate contains but never declares as a [[bin]].
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
REPO=/home/yunzez/c2rust_testing
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
TOOL=$1
DA=${2:-on}       # on = match the ours side (cargo-fuzz forces -Cdebug-assertions); off = the shipped release configuration
T_SUFFIX=""; [ "$DA" = off ] && T_SUFFIX="_noda"
T=$SCR/tests_$TOOL$T_SUFFIX; rm -rf $T; mkdir -p $T/src/bin $T/profiles $T/run
cp $SCR/pair/bzip2_$TOOL/translated/bzip2_$TOOL.rs $T/src/lib.rs
cp $SCR/shims.c $T/shims.c
cat > $T/build.rs <<'B'
use std::process::Command;
fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=shims.c");
    assert!(Command::new("cc").args(["-O1","-fPIC","-c","shims.c","-o",&format!("{out}/shims.o")]).status().unwrap().success());
    assert!(Command::new("ar").args(["rcs",&format!("{out}/libshims.a"),&format!("{out}/shims.o")]).status().unwrap().success());
    println!("cargo:rustc-link-search=native={out}");
    println!("cargo:rustc-link-lib=static=shims");
}
B
echo "fn main() { bzip2_${TOOL}::bzip2::main() }" > $T/src/bin/bzip2cli.rs
DEP=""; [ "$TOOL" = crown ] && DEP='libc = "0.2"'
cat > $T/Cargo.toml <<C
[package]
name = "bzip2_$TOOL"
version = "0.1.0"
edition = "2021"
autobins = false
build = "build.rs"
[lib]
name = "bzip2_$TOOL"
path = "src/lib.rs"
[[bin]]
name = "bzip2cli"
path = "src/bin/bzip2cli.rs"
[dependencies]
$DEP
[workspace]
C
cd $T
if ! CARGO_TARGET_DIR=$T/target RUSTUP_TOOLCHAIN=nightly-2025-09-01 \
   RUSTFLAGS="-C instrument-coverage -C codegen-units=1 -C link-dead-code --cfg fuzzing $([ "$DA" = on ] && echo -C debug-assertions)" \
   cargo build --release --bin bzip2cli > $T/build.log 2>&1; then
  echo "$TOOL: TEST-ADAPTER-FAILS  $(grep -m1 -E '^error' $T/build.log | head -c 90)"; exit 2
fi
cp $REPO/tools/frameworks/crown/c-code/bzip2/sample{1,2,3}.{ref,bz2} $T/run/
cd $T/run
BIN=$T/target/release/bzip2cli
run(){ LLVM_PROFILE_FILE=$T/profiles/%m-%p.profraw "$BIN" "$@"; }
run -1 < sample1.ref > sample1.rb2; run -2 < sample2.ref > sample2.rb2; run -3 < sample3.ref > sample3.rb2
run -d < sample1.bz2 > sample1.tst; run -d < sample2.bz2 > sample2.tst; run -ds < sample3.bz2 > sample3.tst
pass=0; fail=0
for c in "sample1.bz2 sample1.rb2" "sample2.bz2 sample2.rb2" "sample3.bz2 sample3.rb2" \
         "sample1.tst sample1.ref" "sample2.tst sample2.ref" "sample3.tst sample3.ref"; do
  cmp -s $c && pass=$((pass+1)) || fail=$((fail+1)); done
$TC/llvm-profdata merge -sparse $T/profiles/*.profraw -o $T/tests.profdata 2>/dev/null
$TC/llvm-cov export "$BIN" -instr-profile=$T/tests.profdata > $T/tests_coverage.json 2>/dev/null
echo "$TOOL: 6 discovered / 6 executed / $pass passed / $fail failed   export=$(stat -c%s $T/tests_coverage.json 2>/dev/null||echo 0)B"
