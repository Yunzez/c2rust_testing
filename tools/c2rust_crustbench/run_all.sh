#!/usr/bin/env bash
# Transpile every CRUST-bench CBench C repo with c2rust (free, no LLM tokens).
# Per-repo: copy out, transpile each library .c file (excluding test/example), -I all header dirs.
# Records success rate; produces a corpus of name-preserving c2rust translations.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
SRC="/home/yunzez/c2rust_testing/tools/frameworks/CRUST-bench/datasets/CBench"
OUT="/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out"
SUM="/home/yunzez/c2rust_testing/results/c2rust_crustbench_v1.md"
EXCL='/(test|tests|example|examples|bench|benchmark|demo|fuzz|fuzzing)/|(^|/)(test_|fuzz_)|_test\.c$'
rm -rf "$OUT"; mkdir -p "$OUT"

printf '# CRUST-bench × c2rust 0.22.1 — full transpile sweep (v1)\n\n' > "$SUM"
printf 'Per repo: library .c files (test/example excluded) transpiled file-by-file with header dirs on -I.\n\n' >> "$SUM"
printf '| repo | #c | #transpiled | #rs | status |\n|---|--:|--:|--:|---|\n' >> "$SUM"

tot_repo=0; full=0; partial=0; zero=0; tot_c=0; tot_ok=0
for d in "$SRC"/*/; do
  repo=$(basename "$d"); tot_repo=$((tot_repo+1))
  work="$OUT/$repo"
  rsync -a --exclude='.git' "$d" "$work/" 2>/dev/null || { mkdir -p "$work"; cp -r "$d"/* "$work/" 2>/dev/null; rm -rf "$work/.git"; }

  mapfile -t cfiles < <(find "$work" -name '*.c' 2>/dev/null | grep -vEi "$EXCL")
  mapfile -t incdirs < <(find "$work" -name '*.h' 2>/dev/null | grep -vEi "$EXCL" | xargs -r -n1 dirname | sort -u)
  iflags=(); for i in "${incdirs[@]}"; do iflags+=("-I$i"); done

  nc=${#cfiles[@]}; ok=0
  for c in "${cfiles[@]}"; do
    if c2rust transpile "$c" -- "${iflags[@]}" >>"$work/_c2rust.log" 2>&1; then ok=$((ok+1)); fi
  done
  nrs=$(find "$work" -name '*.rs' 2>/dev/null | wc -l)
  tot_c=$((tot_c+nc)); tot_ok=$((tot_ok+ok))

  if [ "$nc" -gt 0 ] && [ "$ok" -eq "$nc" ]; then st="OK"; full=$((full+1));
  elif [ "$ok" -gt 0 ]; then st="PARTIAL"; partial=$((partial+1));
  else st="FAIL"; zero=$((zero+1)); fi
  printf '| %s | %d | %d | %d | %s |\n' "$repo" "$nc" "$ok" "$nrs" "$st" >> "$SUM"
  echo "[$tot_repo/100] $repo: $ok/$nc c -> $nrs rs ($st)"
done

{
  printf '\n## Summary\n\n'
  printf -- '- repos: %d  (full=%d, partial=%d, fail=%d)\n' "$tot_repo" "$full" "$partial" "$zero"
  printf -- '- C translation units: %d attempted, %d transpiled (%d%%)\n' "$tot_c" "$tot_ok" $(( tot_c>0 ? 100*tot_ok/tot_c : 0 ))
} >> "$SUM"
echo "DONE: full=$full partial=$partial fail=$zero | TUs $tot_ok/$tot_c"
echo "ALLDONE_MARKER"
