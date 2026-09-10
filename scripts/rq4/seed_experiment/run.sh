#!/bin/bash
# Three-arm seed experiment, tulip x c2rust: baseline (default 64-byte seed) / random (length-matched
# random seeds) / grid (same bytes, options overwritten with {1,2,3,5,10,20}). One harness build,
# 600 s rust-only fork campaign per arm, fresh corpus per arm, libFuzzer seed 42, -max_len 65536.
set -u
cd /home/yunzez/c2rust_testing
R=/home/yunzez/c2rust_testing
E=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/threearm
COMMON="--pair $R/benchmark/pairs/rq4/tulip_c2rust --lib tulip --tool c2rust --c-source tulip.c --shim $R/benchmark/pairs/rq4/darwin_shims.c --defs $R/benchmark/pairs/rq4/tulip_c2rust/translated/tulip_c2rust.rs.defs.json --seconds 600 --max-len 65536 --preflight-seconds 0"
step() { echo; echo "##### $* — $(date +%H:%M:%S), $(df -BG --output=avail /tmp | tail -1 | tr -dc '0-9')G free"; }
analyse() {
  python3 $R/scripts/c2r_coverage.py --linemap $R/benchmark/pairs/rq4/tulip_c2rust/translated/tulip_c2rust.rs.linemap.json       --ours $E/$1/ours --tests $R/results/rq3_coverage/tulip/c2rust/raw/tests_coverage.json       --out $E/$1/analysis --corpus-root $E/$1/corpus > $E/$1/analysis.log 2>&1
  python3 -c "
import json;r=json.load(open('$E/$1/analysis/result.json'))
print('$1 | fn', r['function']['covered_ours'], '/', r['function']['total_in_scope'], round(r['function']['ours_coverage'],3), '| reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3), '| sanity', all(r['region']['sanity'].values()))"
}
step "baseline arm (build 212 + 600 s + coverage)"
python3 scripts/rq4/cell.py $COMMON --out $E/base > $E/base.log 2>&1; echo "base rc=$?"
analyse base
if [ ! -f $E/seeds_ok ]; then echo "seeds_ok flag absent -- stopping before the seeded arms"; echo THREEARM_STOPPED; exit 2; fi
for arm in random grid; do
  step "$arm arm"
  mkdir -p $E/$arm && cp -r $E/base/harnesses $E/$arm/harnesses
  python3 scripts/rq4/cell.py $COMMON --seeds $E/seeds/$arm --reuse-bins --out $E/$arm > $E/$arm.log 2>&1; echo "$arm rc=$?"
  analyse $arm
done
step "compare"
python3 $E/compare_arms.py $E > $E/COMPARE.md; head -30 $E/COMPARE.md
echo THREEARM_DONE
