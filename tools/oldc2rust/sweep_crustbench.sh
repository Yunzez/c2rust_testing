#!/usr/bin/env bash
# Re-transpile CRUST-bench with OLD c2rust (v0.18, libc::-form) in PROJECT mode,
# so the output is CROWN-compatible. Runs the transpiler inside the oldc2rust:0.18
# docker image; output lands on the host (project-mode Cargo crates with c2rust-lib.rs).
#
# Usage: sweep_crustbench.sh [N]   (N = limit #repos, default all)
set -u
SRC="/home/yunzez/c2rust_testing/tools/frameworks/CRUST-bench/datasets/CBench"
OUT="/home/yunzez/c2rust_testing/tools/frameworks/crown/oldc2rust/out"
IMG="oldc2rust:0.18"
EXCL='/(test|tests|example|examples|bench|benchmark|demo|fuzz|fuzzing)/|(^|/)(test_|fuzz_)|_test\.c$'
LIMIT="${1:-100000}"
rm -rf "$OUT"; mkdir -p "$OUT"

n=0; full=0; partial=0; fail=0
for d in "$SRC"/*/; do
  repo=$(basename "$d"); n=$((n+1)); [ "$n" -gt "$LIMIT" ] && break
  work="$OUT/$repo"
  rsync -a --exclude='.git' "$d" "$work/" 2>/dev/null || { mkdir -p "$work"; cp -r "$d"/* "$work/" 2>/dev/null; }

  mapfile -t cfiles < <(find "$work" -name '*.c' 2>/dev/null | grep -vEi "$EXCL")
  [ "${#cfiles[@]}" -eq 0 ] && { echo "[$n] $repo: no .c (skip)"; fail=$((fail+1)); continue; }
  mapfile -t incdirs < <(find "$work" -name '*.h' 2>/dev/null | grep -vEi "$EXCL" | xargs -r -n1 dirname | sort -u)

  # build compile_commands.json with container-internal paths (/work mounts $work)
  {
    echo "["
    first=1
    for c in "${cfiles[@]}"; do
      rel="/work/${c#$work/}"
      args='"cc"'
      for i in "${incdirs[@]}"; do args="$args,\"-I/work/${i#$work/}\""; done
      args="$args,\"-c\",\"$rel\""
      [ $first -eq 1 ] && first=0 || echo ","
      printf '  {"directory":"/work","file":"%s","arguments":[%s]}' "$rel" "$args"
    done
    echo ""; echo "]"
  } > "$work/compile_commands.json"

  # transpile (project mode) inside the container; emit build files (Cargo + c2rust-lib.rs)
  sudo docker run --rm -v "$work":/work "$IMG" \
      /opt/c2rust/target/release/c2rust-transpile /work/compile_commands.json -e \
      >"$work/_oldc2rust.log" 2>&1
  rc=$?
  has_entry=$([ -f "$work/c2rust-lib.rs" ] || find "$work" -name 'c2rust-lib.rs' | grep -q . && echo y || echo n)
  nrs=$(find "$work" -name '*.rs' 2>/dev/null | wc -l)
  if [ "$rc" -eq 0 ] && [ "$has_entry" = y ]; then st=OK; full=$((full+1));
  elif [ "$nrs" -gt 0 ]; then st=PARTIAL; partial=$((partial+1));
  else st=FAIL; fail=$((fail+1)); fi
  echo "[$n] $repo: rc=$rc entry=$has_entry rs=$nrs ($st)"
done
echo "DONE: full=$full partial=$partial fail=$fail"
