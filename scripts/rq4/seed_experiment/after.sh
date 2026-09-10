#!/bin/bash
set -u; cd /home/yunzez/c2rust_testing; R=/home/yunzez/c2rust_testing; E=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/threearm
until grep -q "THREEARM_DONE\|THREEARM_STOPPED" $E/run.log; do sleep 60; done
grep -q THREEARM_STOPPED $E/run.log && { echo "main run stopped; t0 pass not run"; echo AFTER_STOPPED; exit 2; }
echo "##### t0 pass — $(date +%H:%M:%S)"
python3 $E/t0_pass.py $E > $E/t0_pass.log 2>&1; echo "t0 rc=$?"; grep -c '"ok"\|same as base' $E/t0_pass.log
for arm in base random grid; do
  python3 $R/scripts/c2r_coverage.py --linemap $R/benchmark/pairs/rq4/tulip_c2rust/translated/tulip_c2rust.rs.linemap.json       --ours $E/t0/$arm/ours --tests $R/results/rq3_coverage/tulip/c2rust/raw/tests_coverage.json       --out $E/t0/$arm/analysis --corpus-root $E/t0/$arm/corpus > $E/t0/$arm/analysis.log 2>&1
  python3 -c "
import json;r=json.load(open('$E/t0/$arm/analysis/result.json'))
print('t0 $arm | fn', r['function']['covered_ours'], '/', r['function']['total_in_scope'], '| reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3), '| sanity', all(r['region']['sanity'].values()))"
done
python3 $E/compare_arms.py $E > /dev/null; head -20 $E/COMPARE.md
echo AFTER_DONE
