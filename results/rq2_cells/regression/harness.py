#!/usr/bin/env python3
"""E2 matcher regression harness.
Re-scores every reproducible cell so any matcher/analyzer change can be diff'd against a
saved baseline. Guards the user's hard constraint: signal-C must not drop other libraries.

Two cell kinds:
  - name-preserving pairs (benchmark/pairs/*): truth = name-equality, matcher blind recall.
  - raw-LLM cjson: stored c/r JSON + hand truth.
"""
import json, os, subprocess, sys, tempfile, shutil, pathlib

ROOT = "/home/yunzez/c2rust_testing"
STU = f"{ROOT}/tools/stu_selector"
ANALYZER = f"{STU}/analyzer/target/release/analyzer"
sys.path.insert(0, STU)
import matcher  # noqa

# SIGNALS_OFF=1 disables signal-C + input-scalar => the pre-new-signals control, on the SAME
# (deterministic) matcher/analyzer, so before/after differ only by the new signal blending.
if os.environ.get("SIGNALS_OFF") == "1":
    matcher.USE_CONSTS = False
    matcher._INPUT_W = 0.0

PAIRS = f"{ROOT}/benchmark/pairs"

def analyze_c(build_dir, out):
    subprocess.run([sys.executable, f"{STU}/c_analyzer.py", "--compile-commands", build_dir,
                    "--enable-metrics"], stdout=open(out, "w"), stderr=subprocess.DEVNULL, check=True)

def analyze_rust(translated_dir, out, tmp):
    """Wrap the translated .rs into a minimal lib crate (analyzer needs a cargo project)."""
    rs = [f for f in os.listdir(translated_dir) if f.endswith(".rs")]
    crate = tempfile.mkdtemp(dir=tmp)
    os.makedirs(f"{crate}/src", exist_ok=True)
    if os.path.exists(f"{translated_dir}/Cargo.toml"):
        # already a real crate (Cargo.toml present) — analyze it directly (e.g. lil, whose
        # code lives in src/lil.rs, NOT src/lib.rs; the old src/lib.rs check silently dropped it)
        target = translated_dir
    else:
        # bare .rs file(s): concat into src/lib.rs
        with open(f"{crate}/src/lib.rs", "w") as o:
            for f in rs:
                o.write(open(f"{translated_dir}/{f}").read() + "\n")
        open(f"{crate}/Cargo.toml", "w").write(
            "[package]\nname=\"reg\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[lib]\npath=\"src/lib.rs\"\n")
        target = crate
    subprocess.run([ANALYZER, target, "--enable-metrics"],
                   stdout=open(out, "w"), stderr=subprocess.DEVNULL, check=True)

def recall_name_eq(cj, rj):
    c = json.load(open(cj)); r = json.load(open(rj))
    cn = {f["name"] for f in c["functions"]}
    rn = {f["name"] for f in r["functions"]}
    truth = {n: n for n in cn if n in rn}          # name-equality truth
    if not truth:
        return None, 0
    res = matcher.match(c, r, topo=True)
    got = {cc: rr for (cc, rr, s, k) in res["matched"]}
    correct = sum(1 for k in truth if got.get(k) == truth[k])
    return round(correct / len(truth), 4), len(truth)

def recall_hand(cj, rj, tj):
    c = json.load(open(cj)); r = json.load(open(rj)); truth = json.load(open(tj))
    res = matcher.match(c, r, topo=True)
    got = {cc: rr for (cc, rr, s, k) in res["matched"]}
    correct = sum(1 for k in truth if got.get(k) == truth[k])
    return round(correct / len(truth), 4), len(truth)

def main():
    tmp = tempfile.mkdtemp(prefix="e2reg_")
    rows = {}
    # name-preserving benchmark pairs
    libs = sorted(d for d in os.listdir(PAIRS)
                  if os.path.isdir(f"{PAIRS}/{d}") and os.path.exists(f"{PAIRS}/{d}/build/compile_commands.json"))
    for lib in libs:
        d = f"{PAIRS}/{lib}"
        cj, rj = f"{tmp}/{lib}_c.json", f"{tmp}/{lib}_r.json"
        try:
            analyze_c(f"{d}/build", cj)
            analyze_rust(f"{d}/translated", rj, tmp)
            rec, scor = recall_name_eq(cj, rj)
            if rec is not None:
                rows[lib] = {"recall": rec, "scorable": scor, "kind": "name-eq"}
        except Exception as e:
            rows[lib] = {"error": str(e)[:80], "kind": "name-eq"}
    # raw-LLM cjson (stored)
    cc = f"{ROOT}/results/rq2_cells/rawllm/cjson"
    try:
        rec, scor = recall_hand(f"{cc}/cjson_c.json", f"{cc}/cjson_r.json", f"{cc}/truth.json")
        rows["cjson_rawllm"] = {"recall": rec, "scorable": scor, "kind": "hand"}
    except Exception as e:
        rows["cjson_rawllm"] = {"error": str(e)[:80], "kind": "hand"}
    shutil.rmtree(tmp, ignore_errors=True)
    print(json.dumps(rows, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
