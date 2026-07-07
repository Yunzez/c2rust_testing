#!/usr/bin/env bash
# End-to-end pilot: CRUST-bench --(old c2rust v0.18, project mode)--> fixup --> CROWN --> compile check.
# Usage: pilot.sh <N repos>   (default 6)
set -u
export PATH="$HOME/.cargo/bin:$PATH"
SRC="/home/yunzez/c2rust_testing/tools/frameworks/CRUST-bench/datasets/CBench"
OUT="/home/yunzez/c2rust_testing/tools/frameworks/crown/oldc2rust/pilot_out"
CROWN_DIR="/home/yunzez/c2rust_testing/tools/frameworks/crown"
CROWN="$CROWN_DIR/target/release/crown"
IMG="oldc2rust:0.18"
EXCL='/(test|tests|example|examples|bench|benchmark|demo|fuzz|fuzzing)/|(^|/)(test_|fuzz_)|_test\.c$'
N="${1:-6}"
export LD_LIBRARY_PATH="$(rustc +nightly-2023-01-26 --print sysroot)/lib"
rm -rf "$OUT"; mkdir -p "$OUT"

apply_fixup() {  # $1 = crate dir
  local d="$1" entry
  entry=$(find "$d" -maxdepth 2 -name 'c2rust-lib.rs' | head -1)
  [ -z "$entry" ] && return 1
  sed -i 's/edition = "2021"/edition = "2018"/' "$d/Cargo.toml" 2>/dev/null
  python3 - "$entry" <<'PY'
import sys
p=sys.argv[1]; s=open(p).read()
gates='#![feature(c_variadic)]\n#![feature(register_tool)]\n#![register_tool(c2rust)]\n#![feature(core_intrinsics)]\n#![feature(strict_provenance)]\n#![feature(raw_ref_op)]\nextern crate core;\nextern crate libc;\n'
lines=s.splitlines(True)
head=[l for l in lines if l.startswith('#![allow')]
rest=[l for l in lines if not l.startswith('#![allow') and 'feature(c_variadic)' not in l and 'extern crate libc' not in l and 'extern crate core' not in l]
open(p,'w').write(''.join(head)+gates+''.join(rest))
PY
}

printf "%-26s %-7s %-7s %-7s %-7s\n" repo transpile crown_run lifted compile
n=0; t_ok=0; c_ok=0; comp_ok=0
for d in "$SRC"/*/; do
  repo=$(basename "$d"); n=$((n+1)); [ "$n" -gt "$N" ] && break
  work="$OUT/$repo"
  rsync -a --exclude='.git' "$d" "$work/" 2>/dev/null
  mapfile -t cfiles < <(find "$work" -name '*.c' 2>/dev/null | grep -vEi "$EXCL")
  [ "${#cfiles[@]}" -eq 0 ] && { printf "%-26s %-7s\n" "$repo" "no_c"; continue; }
  mapfile -t incdirs < <(find "$work" -name '*.h' 2>/dev/null | grep -vEi "$EXCL" | xargs -r -n1 dirname | sort -u)
  { echo "["; first=1
    for c in "${cfiles[@]}"; do
      rel="/work/${c#$work/}"; args='"cc"'
      for i in "${incdirs[@]}"; do args="$args,\"-I/work/${i#$work/}\""; done
      args="$args,\"-c\",\"$rel\""
      [ $first -eq 1 ] && first=0 || echo ","
      printf '  {"directory":"/work","file":"%s","arguments":[%s]}' "$rel" "$args"
    done; echo ""; echo "]"; } > "$work/compile_commands.json"

  sudo docker run --rm --user "$(id -u):$(id -g)" -v "$work":/work "$IMG" \
      /opt/c2rust/target/release/c2rust-transpile /work/compile_commands.json \
      --emit-modules --fail-on-error --reduce-type-annotations --emit-build-files >"$work/_tr.log" 2>&1
  tr_ok=$([ -f "$work/c2rust-lib.rs" ] && echo y || echo n)
  [ "$tr_ok" = y ] && t_ok=$((t_ok+1)) || { printf "%-26s %-7s\n" "$repo" "FAIL"; continue; }

  apply_fixup "$work"
  entry="$work/c2rust-lib.rs"; mkdir -p "$work/analysis_results"
  $CROWN "$entry" preprocess in-place   >>"$work/_crown.log" 2>&1
  $CROWN "$entry" explicit-addr in-place >>"$work/_crown.log" 2>&1
  $CROWN "$entry" rewrite --results-path "$work/analysis_results" in-place >>"$work/_crown.log" 2>&1
  cr_ok=$([ -s "$work/analysis_results/ownership.json" ] && echo y || echo n)
  [ "$cr_ok" = y ] && c_ok=$((c_ok+1))
  lifted=$([ "$cr_ok" = y ] && echo y || echo n)
  comp=$( cd "$work" && cargo +nightly-2023-01-26 check >/dev/null 2>&1 && echo y || echo n )
  [ "$comp" = y ] && comp_ok=$((comp_ok+1))
  printf "%-26s %-7s %-7s %-7s %-7s\n" "$repo" "$tr_ok" "$cr_ok" "$lifted" "$comp"
done
echo "SUMMARY: transpile=$t_ok crown_ran=$c_ok compile=$comp_ok  (of $((n-1)) tried)"
