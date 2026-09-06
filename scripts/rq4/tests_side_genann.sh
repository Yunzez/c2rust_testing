#!/usr/bin/env bash
# genann's shipped test suite (test.c, minctest: 9 groups / 38 assertions) run through the
# TRANSLATED test driver `test.rs` that every genann crate except SACTOR carries, under the same
# coverage instrumentation as the ours side, exported for scripts/c2r_coverage.py.
#
# PROTOCOL.md section 2: the tests side runs iff the shipped target's driver is present in the
# translation. The adapter is a two-line bin calling the translated driver's own entry:
#   c2rust / Laertes / C2SaferRust: `test::main()` (the transpiled `pub fn main`)
#   CROWN:                          `src::test::main_0(argc, argv)` (CROWN commented `main` out)
# Pass = the process exits 0 (minctest's main returns the failure count).
#
# usage: tests_side_genann.sh <tool>          -> $RQ4_WORK/tests_genann_<tool>/{tests_coverage.json,run.log}
set -u
SCR=${RQ4_WORK:-"/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov"}
REPO=/home/yunzez/c2rust_testing
PAIRS=$REPO/benchmark/pairs/rq4
TC=$HOME/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin
TOOL=$1
T=$SCR/tests_genann_$TOOL; rm -rf $T; mkdir -p $T/src/bin $T/profiles $T/run
cp $PAIRS/genann_$TOOL/translated/genann_$TOOL.rs $T/src/lib.rs
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
HAS_DRIVER=1
if ! grep -q -E '^\s*pub mod test \{' $T/src/lib.rs; then
  # No transpiled driver at all (SACTOR ships a single lib.rs): TEST-UNAVAILABLE, denominator only.
  HAS_DRIVER=0
  echo 'fn main() { eprintln!("no transpiled test driver"); std::process::exit(3); }' > $T/src/bin/genann_test.rs
fi
if [ $HAS_DRIVER = 1 ] && grep -q '^pub mod src {' $T/src/lib.rs; then      # CROWN layout, main commented out
  # Packaging only, like --expose-entry for a C `static`: the transpiled DRIVER's private entry
  # is made reachable; no library code is touched (the edit is confined to the `test` module).
  python3 - $T/src/lib.rs <<'P'
import re, sys
p = sys.argv[1]; t = open(p).read()
i = t.find("pub mod test {")
assert i >= 0, "no test module"
head, tail = t[:i], t[i:]
tail = tail.replace("unsafe fn main_0(", "pub unsafe fn main_0(", 1)
open(p, "w").write(head + tail)
P
  NS="src::"
  cat > $T/src/bin/genann_test.rs <<E
fn main() {
    let mut argv: [*mut std::os::raw::c_char; 2] = [b"test\0".as_ptr() as *mut _, std::ptr::null_mut()];
    let rc = unsafe { genann_${TOOL}::src::test::main_0(1, argv.as_mut_ptr()) };
    std::process::exit(rc);
}
E
elif [ $HAS_DRIVER = 1 ]; then
  NS=""
  echo "fn main() { genann_${TOOL}::test::main() }" > $T/src/bin/genann_test.rs
else
  NS=""
fi
# Denominator bin (PROTOCOL.md section 6): links the whole library with -C link-dead-code and runs
# nothing, so a suite that aborts before its profile is written still yields the universe.
GMOD="${NS}genann::"; grep -q -E '^\s*pub mod genann \{' $T/src/lib.rs || GMOD=""   # SACTOR: one flat lib.rs, no module
cat > $T/src/bin/denom.rs <<E
fn main() { unsafe { std::hint::black_box(genann_${TOOL}::${GMOD}genann_init(1, 0, 0, 1)); } }
E
DEP=""; grep -q -E '\blibc::|^\s*use libc\b' $T/src/lib.rs && DEP='libc = "0.2"'
cat > $T/Cargo.toml <<C
[package]
name = "genann_$TOOL"
version = "0.1.0"
edition = "2021"
autobins = false
build = "build.rs"
[lib]
name = "genann_$TOOL"
path = "src/lib.rs"
[[bin]]
name = "genann_test"
path = "src/bin/genann_test.rs"
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
   cargo build --release --bins --message-format=json-render-diagnostics > $T/cargo.json 2> $T/build.log; then
  echo "$TOOL: TEST-ADAPTER-FAILS  $(grep -m1 -E '^error' $T/build.log | head -c 120)"; exit 2
fi
cd $T/run
LLVM_PROFILE_FILE=$T/profiles/%m-%p.profraw timeout 600 $T/target/release/genann_test > $T/run.log 2>&1
RC=$?
$TC/llvm-profdata merge -sparse $T/profiles/*.profraw -o $T/tests.profdata 2>/dev/null
$TC/llvm-cov export $T/target/release/genann_test -instr-profile=$T/tests.profdata > $T/tests_coverage.json 2>/dev/null
# universe from the denom bin, always (identical identity set; used when the suite is not a baseline)
mkdir -p $T/dprof; LLVM_PROFILE_FILE=$T/dprof/d.profraw $T/target/release/denom > /dev/null 2>&1
$TC/llvm-profdata merge -sparse $T/dprof/d.profraw -o $T/denom.profdata 2>/dev/null
$TC/llvm-cov export $T/target/release/denom -instr-profile=$T/denom.profdata > $T/denominator.json 2>/dev/null
# universe from the rlib's instrumented objects (see scripts/rq4/rlib_universe.py): the bin route
# misses the whole crate when the reference call is cross-crate inlined
python3 $REPO/scripts/rq4/rlib_universe.py $T/target genann_$TOOL $T/denom.profdata $T/denominator.json $T/cargo.json > /dev/null
echo "$TOOL: exit=$RC  $(grep -E -m1 -i 'tests?,|passed|fail' $T/run.log | head -c 100)  export=$(stat -c%s $T/tests_coverage.json 2>/dev/null || echo 0)B  denominator=$(stat -c%s $T/denominator.json 2>/dev/null || echo 0)B"
if [ $HAS_DRIVER = 0 ]; then echo "$TOOL: TEST-UNAVAILABLE (no transpiled driver); denominator only"
elif [ $RC -eq 0 ]; then echo "$TOOL: PASS"
else echo "$TOOL: TEST-FAILS (exit $RC): $(grep -m1 -E 'panicked at' $T/run.log | head -c 140)"; fi
