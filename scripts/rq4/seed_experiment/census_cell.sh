#!/bin/bash
# Seeds-only census for one cell (docs/seeding_policy_plan.md section 4): plan-guided seeds from the frozen policy,
# harness binaries rebuilt with the current generator (no campaign), one coverage build per harness replaying
# (a) the default seed alone and (b) default + plan-guided seeds, application-level coverage against the cell's
# archived universe. usage: census_cell.sh <lib> <tool> <c_source> <workdir>
set -u
LIB=$1; TOOL=$2; CSRC=$3; E=$4
W=/home/yunzez/c2rust_seedir; R=/home/yunzez/c2rust_testing; P=$R/benchmark/pairs/rq4/${LIB}_${TOOL}; A=$R/results/rq3_coverage/$LIB/$TOOL
S=/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad
universe() {   # the archived cell's universe: the tests build when one was measured, else the rlib / bin-route denominator (possibly inside a tarball)
  if [ -s $A/raw/tests_coverage.json ]; then echo "--tests $A/raw/tests_coverage.json";   # -s: a failed tests build leaves an EMPTY file (quadtree x crown) -> denominator
  elif [ -f $A/raw/denominator.json ]; then echo "--denominator $A/raw/denominator.json";
  else T=$(ls $A/raw/denom_*.tar.gz 2>/dev/null | head -1); mkdir -p $E/universe; tar xzf $T -C $E/universe 2>/dev/null; D=$(find $E/universe -name denominator.json | head -1)
       # the old bin-route export records the denom crate's ABSOLUTE lib.rs path of the day; map it onto the extracted copy
       OLD=$(python3 -c "import json,os;d=json.load(open('$D'));p=[f['filenames'][0] for f in d['data'][0]['functions'] if f['filenames'][0].endswith('src/lib.rs')][0];print(os.path.dirname(os.path.dirname(p)))")
       echo "--denominator $D --path-map $OLD=$(dirname $D)"; fi; }
PLUG=""; [ "$LIB" = cjson ] && PLUG="--plugins $R/plugins/cjson/plugin.toml"
step() { echo "##### $LIB x $TOOL $* — $(date +%H:%M:%S), files $(find $S -type f | wc -l)"; }
rm -rf $E; mkdir -p $E/seeds
step "rebuild bins"; python3 $W/scripts/rq4/seed_experiment/rebuild_bins.py --pair $P --lib $LIB --tool $TOOL --out $E/base --c-source $CSRC --shim $R/benchmark/pairs/rq4/darwin_shims.c --defs $P/translated/${LIB}_${TOOL}.rs.defs.json $PLUG > $E/rebuild.log 2>&1; grep -o "built [0-9/]*" $E/rebuild.log
# seeds are lowered from the plans the rebuilt harnesses were generated from (base/plans.json), never from the archived plans
step "policy seeds"; python3 $W/scripts/rq4/seed_policy.py --plans $E/base/plans.json --pair $P --out $E/seeds/policy | tail -1
python3 - <<PY
import json, pathlib, shutil
E=pathlib.Path("$E"); man=json.load(open(E/"seeds/policy/manifest.json")); rows=json.load(open(E/"base/funnel.json"))
for r in rows:
    if not r["built"]: continue
    b=r["boundary"]
    for arm in ("base","policy"):
        d=E/"t0"/arm/"corpus"/b; d.mkdir(parents=True, exist_ok=True); (d/"seed").write_bytes(bytes(range(64)))
        if arm=="policy" and (E/"seeds"/"policy"/b).is_dir():
            for f in (E/"seeds"/"policy"/b).iterdir(): shutil.copy(f, d/f.name)
print("t0 corpora written")
PY
step "t0 pass (coverage build per harness, both seed sets)"; python3 $W/scripts/rq4/seed_experiment/t0_pass_census.py $E > $E/t0_pass.log 2>&1; echo "t0 rc=$?"
python3 $W/scripts/rq4/seed_experiment/t0_fix.py $E base,policy > $E/t0_fix.log 2>&1; echo "t0fix rc=$?"
for arm in base policy; do
  python3 - <<PY   # an export that is empty or not JSON (the replay produced no usable profile) carries no coverage and would abort the analysis
import json, pathlib
for f in pathlib.Path("$E/t0/$arm/ours").glob("*.json"):
    try: json.load(open(f))
    except Exception: print("dropping unreadable export", f.name); f.unlink()
PY
  python3 $R/scripts/c2r_coverage.py --linemap $P/translated/${LIB}_${TOOL}.rs.linemap.json --ours $E/t0/$arm/ours $(universe) --out $E/t0/$arm/analysis --corpus-root $E/t0/$arm/corpus > $E/t0/$arm/analysis.log 2>&1
  python3 -c "
import json;r=json.load(open('$E/t0/$arm/analysis/result.json'));a=json.load(open('$A/analysis/result.json'))
print('$LIB x $TOOL t0 $arm | fn', r['function']['covered_ours'], '/', r['function']['total_in_scope'], '| reg', r['region']['covered_ours'], '/', r['region']['total_in_scope'], round(r['region']['ours_coverage'],3), '| archived campaign reg', a['region']['covered_ours'], round(a['region']['ours_coverage'],3))"
  [ -f $E/t0/$arm/analysis/result.json ] && rm -rf $E/t0/$arm/ours   # exports consumed; kept when the analysis failed
done
step "pack"; if [ -f $E/t0/base/analysis/result.json ] && [ -f $E/t0/policy/analysis/result.json ]; then rm -rf $E/base/harnesses; fi; rm -rf $E/base/target $E/t0/base/corpus $E/t0/policy/corpus; echo "$LIB x $TOOL CENSUS_CELL_DONE"
