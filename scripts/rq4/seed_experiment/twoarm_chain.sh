#!/bin/bash
# The other three tulip translations, same three arms + t=0 pass, serial, after c2rust's t=0 pass has finished.
set -u
cd /home/yunzez/c2rust_testing
R=/home/yunzez/c2rust_testing
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
until grep -q "AFTER_DONE\|AFTER_STOPPED" $S/threearm/after.log; do sleep 60; done
step() { echo; echo "##### $* — $(date +%H:%M:%S), $(df -BG --output=avail /tmp | tail -1 | tr -dc '0-9')G free"; }
universe() { case $1 in c2rust|crown) echo "--tests $R/results/rq3_coverage/tulip/$1/raw/tests_coverage.json" ;; *) echo "--denominator $R/results/rq3_coverage/tulip/$1/raw/denominator.json" ;; esac; }
for T in laertes c2saferrust crown; do
  E=$S/threearm_$T
  COMMON="--pair $R/benchmark/pairs/rq4/tulip_$T --lib tulip --tool $T --c-source tulip.c --shim $R/benchmark/pairs/rq4/darwin_shims.c --defs $R/benchmark/pairs/rq4/tulip_$T/translated/tulip_$T.rs.defs.json --seconds 600 --max-len 65536 --preflight-seconds 0"
  analyse() {
    python3 $R/scripts/c2r_coverage.py --linemap $R/benchmark/pairs/rq4/tulip_$T/translated/tulip_$T.rs.linemap.json \
        --ours $E/$1/ours $(universe $T) --out $E/$1/analysis --corpus-root $E/$1/corpus > $E/$1/analysis.log 2>&1
    python3 -c "
import json;r=json.load(open('$E/$1/analysis/result.json'))
print('$T $1 | fn', r['function']['covered_ours'], '/', r['function']['total_in_scope'], round(r['function']['ours_coverage'],3), '| reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3), '| sanity', all(r['region']['sanity'].values()))"
  }
  python3 $S/threearm/make_t0_corpora.py $E $R/results/rq3_coverage/tulip/$T/plans.json
  step "$T base arm (build + 600 s + coverage)"
  python3 scripts/rq4/cell.py $COMMON --out $E/base > $E/base.log 2>&1; echo "$T base rc=$?"
  [ -f $E/base/funnel.json ] || { echo "$T base produced no funnel -- skipping this tool"; continue; }
  analyse base
  for arm in grid; do
    step "$T $arm arm"
    mkdir -p $E/$arm && cp -r $E/base/harnesses $E/$arm/harnesses
    python3 scripts/rq4/cell.py $COMMON --seeds $E/seeds/$arm --reuse-bins --out $E/$arm > $E/$arm.log 2>&1; echo "$T $arm rc=$?"
    analyse $arm
  done
  step "$T t0 pass"
  python3 $S/threearm/t0_pass_2arm.py $E > $E/t0_pass.log 2>&1; echo "$T t0 rc=$?"
  python3 $S/threearm/t0_fix.py $E base,grid > $E/t0_fix.log 2>&1; echo "$T t0fix rc=$?"
  for arm in base grid; do
    python3 $R/scripts/c2r_coverage.py --linemap $R/benchmark/pairs/rq4/tulip_$T/translated/tulip_$T.rs.linemap.json \
        --ours $E/t0/$arm/ours $(universe $T) --out $E/t0/$arm/analysis --corpus-root $E/t0/$arm/corpus > $E/t0/$arm/analysis.log 2>&1
    python3 -c "
import json;r=json.load(open('$E/t0/$arm/analysis/result.json'))
print('$T t0 $arm | reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3))"
  done
  python3 $S/threearm/compare_arms_2arm.py $E > /dev/null; echo "$T COMPARE.md written"
done
echo CHAIN2_DONE
