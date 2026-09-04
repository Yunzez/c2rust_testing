#!/bin/bash
# Generate a harness from a HarnessPlan (no schema), apply the platform fixups, build, run.
set -u
REPO=/home/yunzez/c2rust_testing
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
P=$S/rq4_cov/pair/bzip2_c2rust
ENTRY=$1; OUT=$S/plan_h/$ENTRY
EXPOSE=${2:-}
mkdir -p $S/plan_h; rm -rf "$OUT"
python3 $REPO/tools/stu_selector/gen_diff_harness.py --pair $P --entry $ENTRY --rust-entry $ENTRY \
    --plan --ub-free --c-source bzip2lib.c --out "$OUT" $EXPOSE > "$OUT.gen.log" 2>&1 || \
    { echo "GEN FAIL $ENTRY"; tail -3 "$OUT.gen.log"; exit 1; }
# platform fixups (identical to the round-2 campaign driver): sibling .c files + libc shims
python3 - "$P" "$OUT" "$ENTRY" "$EXPOSE" <<'PY'
import sys, shutil, pathlib
sys.path.insert(0,"/home/yunzez/c2rust_testing/tools/stu_selector")
import gen_diff_harness as gdh
pair, out, entry, expose = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]), sys.argv[3], sys.argv[4]
S = pathlib.Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad")
for extra in sorted((pair/"source").glob("*.c")):
    if extra.name == "bzip2lib.c": continue
    text = extra.read_text()
    if expose:
        text, ch = gdh.strip_static_c(text, entry)
        if not ch:
            import re
            pat = re.compile(r'(?m)^[ \t]*static[ \t]*\n(?:[ \t]*(?:__inline__|inline)[ \t]*\n)?'
                             r'([ \t]*[A-Za-z_][\w \t\*]*\b'+re.escape(entry)+r'[ \t]*\()')
            text, n = pat.subn(r'\1', text, count=1)
    (out/"c"/extra.name).write_text(text)
shutil.copy(S/"rq4_cov"/"shims.c", out/"c"/"shims.c")
b = out/"build.rs"; t = b.read_text()
t = t.replace('    build.compile("c_oracle");',
  '    build.file("c/shims.c");\n    build.compile("c_oracle");\n'
  '    println!("cargo:rustc-link-arg=-Wl,-u,__maskrune");\n'
  '    println!("cargo:rustc-link-arg=-Wl,-u,_DefaultRuneLocale");')
b.write_text(t)
(out/"rust-toolchain").write_text("nightly-2025-09-01\n")
(out/"fuzz"/"rust-toolchain").write_text("nightly-2025-09-01\n")
if expose:
    import json
    defs = json.load(open(next((pair/"translated").glob("*.rs.defs.json"))))
    if entry in defs["private"]:
        lib = out/"src"/"lib.rs"
        lib.write_text(lib.read_text()+f"\npub use crate::{defs['defs'][entry]}::{entry};\n")
PY
export CARGO_TARGET_DIR=$S/plan_target
cd "$OUT" && RUSTUP_TOOLCHAIN=nightly-2025-09-01 cargo fuzz build bzip2_c2rust_ft > "$OUT.build.log" 2>&1
rc=$?
if [ $rc -ne 0 ]; then echo "BUILD FAIL $ENTRY"; grep -E "^error" "$OUT.build.log" | head -5; exit 2; fi
mkdir -p $S/plan_bin
cp $S/plan_target/x86_64-unknown-linux-gnu/release/bzip2_c2rust_ft $S/plan_bin/$ENTRY
echo "BUILD OK $ENTRY"
