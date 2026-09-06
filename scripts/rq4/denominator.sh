#!/usr/bin/env bash
# The coverage UNIVERSE for a translation whose shipped suite is unavailable or fails (PROTOCOL.md
# section 2/6): a link-dead-code build of the flattened translation with a bin that references the
# crate and runs nothing, exported with the toolchain's own llvm-cov. Generic over libraries: the
# reference call is an argument.
#
# usage: denominator.sh <lib> <tool> "<rust call that references the crate>"
#   e.g. denominator.sh cjson c2rust "cjson_c2rust::cJSON_CreateObject()"
# writes $RQ4_WORK/denom_<lib>_<tool>/denominator.json
set -u
SCR=${RQ4_WORK:-"/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov"}
REPO=/home/yunzez/c2rust_testing
PAIRS=$REPO/benchmark/pairs/rq4
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
LIB=$1; TOOL=$2; CALL=$3
T=$SCR/denom_${LIB}_$TOOL; rm -rf $T; mkdir -p $T/src/bin $T/dprof
cp $PAIRS/${LIB}_$TOOL/translated/${LIB}_$TOOL.rs $T/src/lib.rs
cp $PAIRS/darwin_shims.c $T/shims.c
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
echo "fn main() { unsafe { std::hint::black_box($CALL); } }" > $T/src/bin/denom.rs
DEP=""; grep -q -E '\blibc::|^\s*use libc\b' $T/src/lib.rs && DEP='libc = "0.2"'
cat > $T/Cargo.toml <<C
[package]
name = "${LIB}_$TOOL"
version = "0.1.0"
edition = "2021"
autobins = false
build = "build.rs"
[lib]
name = "${LIB}_$TOOL"
path = "src/lib.rs"
[[bin]]
name = "denom"
path = "src/bin/denom.rs"
[dependencies]
$DEP
[workspace]
C
cd $T
if ! CARGO_TARGET_DIR=$T/target RUSTUP_TOOLCHAIN=nightly-2025-09-01 \
   RUSTFLAGS="-C instrument-coverage -C codegen-units=1 -C link-dead-code --cfg fuzzing -C debug-assertions" \
   cargo build --release --bin denom --message-format=json-render-diagnostics > $T/cargo.json 2> $T/build.log; then
  echo "$LIB/$TOOL: DENOM-BUILD-FAILS  $(grep -m1 -E '^error' $T/build.log | head -c 140)"; exit 2
fi
LLVM_PROFILE_FILE=$T/dprof/d.profraw $T/target/release/denom > /dev/null 2>&1
$TC/llvm-profdata merge -sparse $T/dprof/d.profraw -o $T/denom.profdata 2>/dev/null
$TC/llvm-cov export $T/target/release/denom -instr-profile=$T/denom.profdata > $T/denominator.json 2>/dev/null
# The universe comes from the rlib's own instrumented objects, not from what the bin happened to
# link: a reference call small enough for cross-crate inlining (PtrTrans `cJSON_Version()`) pulls
# nothing from the rlib and the bin-based export collapses to two functions. The bin build stays as
# the link check. (scripts/rq4/rlib_universe.py; verified identical on every earlier cell.)
python3 $REPO/scripts/rq4/rlib_universe.py $T/target ${LIB}_$TOOL $T/denom.profdata $T/denominator.json $T/cargo.json
echo "$LIB/$TOOL: denominator=$(stat -c%s $T/denominator.json 2>/dev/null || echo 0)B  ($(python3 -c "import json;print(len(json.load(open('$T/denominator.json'))['data'][0]['functions']))" 2>/dev/null) functions)"
