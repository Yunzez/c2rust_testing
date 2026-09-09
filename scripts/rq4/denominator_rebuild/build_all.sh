#!/usr/bin/env bash
# Rebuild the rlib denominator for the 11 cells whose archived universe is bin-route or absent. Sequential.
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
export RQ4_WORK=$S/denom_rebuild
cd /home/yunzez/c2rust_testing
while read lib tool call; do
  echo "=== $lib/$tool  $(date +%H:%M:%S)"
  bash scripts/rq4/denominator.sh $lib $tool "$call" 2>&1 | tail -3
done <<'LIST'
bzip2 c2rust bzip2_c2rust::bzlib::BZ2_bzlibVersion()
genann c2rust genann_c2rust::genann::genann_init(1, 1, 1, 1)
genann laertes genann_laertes::genann::genann_init(1, 1, 1, 1)
genann c2saferrust genann_c2saferrust::genann::genann_init(1, 1, 1, 1)
genann crown genann_crown::src::genann::genann_init(1, 1, 1, 1)
genann sactor genann_sactor::genann_init(1, 1, 1, 1)
cjson c2rust cjson_c2rust::cJSON_CreateObject()
lil c2rust lil_c2rust::lil::lil_new()
lil c2saferrust lil_c2saferrust::lil::lil_new()
lil crown lil_crown::src::lil::lil_new()
lil laertes lil_laertes::lil::lil_new()
LIST
echo "DENOM_REBUILD_DONE $(date +%H:%M:%S)"
