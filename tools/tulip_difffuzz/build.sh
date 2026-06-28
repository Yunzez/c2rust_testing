#!/usr/bin/env bash
# Build two differential fuzzers: C oracle vs c2rust, and C oracle vs C2SaferRust.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
TI="/home/yunzez/c2rust_testing/tools/frameworks/tulipindicators"               # upstream C (0.9.2)
C2R="/home/yunzez/c2rust_testing/tools/frameworks/c2saferrust/laertes_benchmarks/tulipindicators/target/release/libc2rust_out.a"
SAFER="/home/yunzez/c2rust_testing/tools/frameworks/c2saferrust/laertes_benchmarks/tulipindicators_WIP/target/release/libc2rust_out.a"

cd "$HERE"
cp -f "$TI/indicators.h" .

# 1) C oracle static lib
rm -f obj/*.o; mkdir -p obj
for f in "$TI"/indicators/*.c "$TI"/indicators.c "$TI"/candles.c "$TI"/utils/*.c; do
  [ -f "$f" ] || continue
  cc -O2 -c "$f" -o "obj/$(echo "$f"|tr '/' '_').o" -I"$TI"
done
ar rcs libti_c.a obj/*.o
echo "C oracle: $(nm libti_c.a | grep -cE ' T ti_') ti_ symbols"

# 2) macOS assert shim (the c2rust crates were generated on macOS)
cat > shim.c <<'EOF'
#include <stdio.h>
#include <stdlib.h>
void __assert_rtn(const char* fn,const char* f,int l,const char* e){
    fprintf(stderr,"assert_rtn %s %s:%d %s\n",fn?fn:"?",f?f:"?",l,e?e:"?"); abort();
}
EOF

# 3) rename ti_* -> r_ti_* in a copy of each Rust lib, then link a fuzzer
build_one(){
  local which="$1" lib="$2"
  [ -f "$lib" ] || { echo "MISSING $lib"; return 1; }
  nm "$lib" 2>/dev/null | awk '{print $NF}' | grep '^ti_' | sort -u \
    | awk '{print $1" r_"$1}' > "redef_$which.txt"
  cp -f "$lib" "lib_${which}_orig.a"
  objcopy --redefine-syms="redef_$which.txt" "lib_${which}_orig.a" "lib_${which}_r.a"
  echo "$which: renamed $(wc -l < redef_$which.txt) ti_ symbols; r_ti_ now: $(nm lib_${which}_r.a|grep -cE ' [TDBR] r_ti_')"
  cc -O2 -rdynamic fuzz.c libti_c.a -Wl,--whole-archive "lib_${which}_r.a" -Wl,--no-whole-archive -lpthread -ldl -lm -o "fuzz_$which"
  echo "  built fuzz_$which"
}
build_one c2r   "$C2R"
build_one safer "$SAFER"
echo "DONE."
