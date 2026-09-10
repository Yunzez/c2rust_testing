#!/bin/bash
set -u
cd /home/yunzez/c2rust_testing
R=/home/yunzez/c2rust_testing
COMMON="--pair $R/benchmark/pairs/rq4/tulip_c2rust --lib tulip --tool c2rust --c-source tulip.c --shim $R/benchmark/pairs/rq4/darwin_shims.c --defs $R/benchmark/pairs/rq4/tulip_c2rust/translated/tulip_c2rust.rs.defs.json --only ti_sma --seconds 60 --max-len 65536 --preflight-seconds 0"
python3 scripts/rq4/cell.py $COMMON --out /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/base > /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/base.log 2>&1
echo "base rc=$?"
mkdir -p /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/hint && cp -r /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/base/harnesses /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/hint/harnesses
python3 scripts/rq4/cell.py $COMMON --out /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/hint --seeds /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/seeds --reuse-bins > /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/probe_sma/hint.log 2>&1
echo "hint rc=$?"
echo PROBE_DONE
