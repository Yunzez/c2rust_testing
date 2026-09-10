#!/bin/bash
set -u; cd /home/yunzez/c2rust_testing; R=/home/yunzez/c2rust_testing; S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad; E=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/threearm
until grep -q "AFTER_DONE\|AFTER_STOPPED" $E/after.log; do sleep 20; done
echo "##### t0 fix (per-input replay for harnesses whose batch replay died) — $(date +%H:%M:%S)"
python3 $E/t0_fix.py $E > $E/t0_fix.log 2>&1; echo "t0fix rc=$?"
for arm in base random grid; do
  python3 $R/scripts/c2r_coverage.py --linemap $R/benchmark/pairs/rq4/tulip_c2rust/translated/tulip_c2rust.rs.linemap.json       --ours $E/t0/$arm/ours --tests $R/results/rq3_coverage/tulip/c2rust/raw/tests_coverage.json       --out $E/t0/$arm/analysis --corpus-root $E/t0/$arm/corpus > $E/t0/$arm/analysis.log 2>&1
  python3 -c "
import json;r=json.load(open('$E/t0/$arm/analysis/result.json'))
print('t0(fixed) $arm | fn', r['function']['covered_ours'], '/', r['function']['total_in_scope'], '| reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3))"
done
python3 $E/compare_arms.py $E > /dev/null; echo "COMPARE.md rewritten"; echo T0FIX_DONE
exec $S/twoarm_chain.sh
