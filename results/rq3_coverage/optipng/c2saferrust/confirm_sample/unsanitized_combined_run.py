#!/usr/bin/env python3
"""Unsanitized COMBINED comparison: rebuild the no-sanitizer harness (C side plain, Rust side no ASan,
debug assertions as in the campaign) for `uncompress` on two cells and replay the archived inputs with
BOTH sides running (C2R_MODE unset = the ladder), so a value difference is observed without any instrument."""
import sys, os, json, gzip, subprocess, argparse, shutil, time
from pathlib import Path
ROOT = Path("/home/yunzez/c2rust_testing"); sys.path.insert(0, str(ROOT / "scripts" / "rq4")); sys.path.insert(0, str(ROOT / "scripts"))
import cell as CELL
S = Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad")
OUT = S / "unsan_combined"
JOBS = [("optipng", "laertes"), ("optipng", "c2saferrust")]
for lib, tool in JOBS:
    A = ROOT / "results/rq3_coverage" / lib / tool; pair = ROOT / "benchmark/pairs/rq4" / f"{lib}_{tool}"
    cell = OUT / f"cell_{lib}_{tool}"; (cell / "harnesses").mkdir(parents=True, exist_ok=True)
    a = argparse.Namespace(c_source="optipng_multi.c", plugins=None, shim=str(ROOT / "benchmark/pairs/rq4/darwin_shims.c"),
                           defs=str(pair / "translated" / f"{lib}_{tool}.rs.defs.json"), pair=str(pair))
    row = [r for r in json.load(open(A / "funnel.json")) if r["boundary"] == "uncompress"][0]
    t0 = time.time()
    nos, err = CELL.build_one(a, pair, "uncompress", row["c_static"], cell / "harnesses" / "uncompress_nosan", cell / "target", nosan=True)
    print(f"{lib}/{tool}: nosan build {'OK' if nos else 'FAILED '+str(err)[:200]} in {time.time()-t0:.0f}s", flush=True)
    if not nos: continue
    shutil.rmtree(cell / "target", ignore_errors=True)
    # inputs: the sampled candidates + the divergence inputs of the boundary
    inputs = sorted(p for p in (A / "candidates_sample" / "uncompress").glob("*") if p.is_file() and p.suffix != ".log")
    inputs += sorted(p for p in (A / "divergences" / "uncompress").glob("divergence-*") if p.is_file())
    # which archived rows were instrument_only / confirmed_divergence (to cross-tabulate)
    rows = {r["artifact"]: r["verdict"] for r in json.load(gzip.open(A / "confirm_sample/uncompress_verdicts/verdicts.json.gz"))}
    tally = {}; per = []
    for p in inputs:
        env = dict(os.environ); env.pop("C2R_MODE", None); env["ASAN_OPTIONS"] = "detect_leaks=0"
        try:
            r = subprocess.run([str(nos), str(p)], env=env, capture_output=True, text=True, errors="replace", timeout=25, cwd=str(cell))
            kind = "normal"; detail = ""
            for l in (r.stdout + r.stderr).splitlines():
                if l.startswith("C2R_OUTCOME"):
                    kind = l.split("kind=")[1].split()[0]; detail = l.split("detail=")[1] if "detail=" in l else ""
            if r.returncode not in (0,) and kind == "normal": kind = f"exit{r.returncode}"
        except subprocess.TimeoutExpired:
            kind, detail = "timeout", ""
        v = rows.get(p.name, "?")
        tally[(v, kind, detail[:40])] = tally.get((v, kind, detail[:40]), 0) + 1
        per.append({"input": p.name, "archived_verdict": v, "unsanitized_combined": kind, "detail": detail[:80]})
    json.dump({"cell": f"{lib}/{tool}", "boundary": "uncompress", "n": len(per), "tally": [{"archived": k[0], "unsan_combined": k[1], "detail": k[2], "n": n} for k, n in sorted(tally.items())], "rows": per},
              open(OUT / f"{lib}_{tool}_uncompress.json", "w"), indent=1)
    print(f"{lib}/{tool}: {len(per)} inputs replayed unsanitized+combined:", flush=True)
    for k, n in sorted(tally.items()): print(f"   archived={k[0]:22s} unsan_combined={k[1]:12s} {k[2]:40s} {n}", flush=True)
    for junk in (cell / "harnesses" / "uncompress_nosan",): shutil.rmtree(junk, ignore_errors=True)
print("UNSAN_COMBINED_DONE", flush=True)
