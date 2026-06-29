#!/usr/bin/env bash
# Rebuild the libopenaptx scalar-function differential PAIRS from the (gitignored) RustAssure
# checkout, so we don't commit copies of RustAssure's GPT-4o translations (that repo ships NO
# license). The C oracles below are de-static'd from upstream libopenaptx (LGPL); the Rust side
# is copied from the checkout at run time.
#
# Prereq: tools/frameworks/rustassure cloned (git clone https://github.com/davsec-lab/rustassure).
# Usage:  bash tools/headtohead/setup_libopenaptx.sh
set -eu
ROOT="/home/yunzez/c2rust_testing"
RA="$ROOT/tools/frameworks/rustassure/src/python/inputs-complex/libopenaptx"
HH="$ROOT/tools/headtohead/libopenaptx"
[ -d "$RA" ] || { echo "RustAssure checkout missing at $RA — clone it first"; exit 1; }

mk_pair() {  # $1=fn  $2=model(gpt4o|flourine)  $3=C-source-body-file
  local fn="$1" model="$2" cbody="$3"
  local p="$HH/${fn}_${model}"
  rm -rf "$p"; mkdir -p "$p"/{source,translated,build}
  cp "$cbody" "$p/source/$fn.c"
  # copy the translation, stripping any `fn main` test driver + its leading attributes
  # (keeps type aliases / structs the target fn needs; drops the standalone main).
  python3 - "$RA/$model/$fn.rs" "$p/translated/$fn.rs" <<'PY'
import sys,re
src,dst=sys.argv[1],sys.argv[2]
lines=open(src).read().split('\n')
mi=next((i for i,l in enumerate(lines) if re.match(r'\s*(pub\s+)?(unsafe\s+)?fn\s+main\b',l)), None)
if mi is not None:
    j=mi
    while j-1>=0 and lines[j-1].strip().startswith('#['): j-=1
    depth=0; started=False; end=len(lines)-1
    for i in range(mi,len(lines)):
        depth+=lines[i].count('{')-lines[i].count('}')
        if '{' in lines[i]: started=True
        if started and depth<=0: end=i; break
    lines=lines[:j]+lines[end+1:]
open(dst,'w').write('\n'.join(lines).rstrip()+'\n')
PY
  cat > "$p/build/compile_commands.json" <<EOF
[ { "directory": "$p/build", "file": "$p/source/$fn.c",
    "arguments": ["clang","-c","$p/source/$fn.c"] } ]
EOF
  echo "built pair $p"
}

# C oracles (de-static'd from libopenaptx openaptx.c) written to a temp, then installed.
TMP=$(mktemp -d)
cat > "$TMP/clip.c" <<'EOF'
#include <stdint.h>
int32_t clip(int32_t a, int32_t amin, int32_t amax)
{
    if      (a < amin) return amin;
    else if (a > amax) return amax;
    else               return a;
}
EOF
cat > "$TMP/sign_extend.c" <<'EOF'
#include <stdint.h>
int32_t sign_extend(int32_t val, unsigned bits)
{
    const unsigned shift = 8 * sizeof(val) - bits;
    union { uint32_t u; int32_t s; } v;
    v.u = (uint32_t)val << shift;
    return v.s >> shift;
}
EOF
cat > "$TMP/aptx_bin_search.c" <<'EOF'
#include <stdint.h>
int32_t aptx_bin_search(int32_t value, int32_t factor, const int32_t *intervals, int nb_intervals)
{
    int32_t idx = 0; int i;
    for (i = nb_intervals >> 1; i > 0; i >>= 1)
        if ((int64_t)factor * (int64_t)intervals[idx + i] <= ((int64_t)value << 24)) idx += i;
    return idx;
}
EOF

for model in gpt4o flourine; do
  mk_pair clip            "$model" "$TMP/clip.c"
  mk_pair sign_extend     "$model" "$TMP/sign_extend.c"
  mk_pair aptx_bin_search "$model" "$TMP/aptx_bin_search.c"
done
rm -rf "$TMP"
echo "DONE. Generate a harness with:"
echo "  python3 tools/stu_selector/gen_diff_harness.py --pair tools/headtohead/libopenaptx/clip_gpt4o --entry clip --expose-entry --ub-free --infer-schema"
