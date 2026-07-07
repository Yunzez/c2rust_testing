#!/usr/bin/env python3
"""Auto-sweep a base/WIP Rust module pair: enumerate value functions, differential-fuzz each.

Wraps rust_diff.py. Discovers every `pub fn NAME` present in BOTH the base and WIP module, generates a
differential per function, builds, fuzzes for a short budget, and records the outcome
(CLEAN / DIVERGENCE / UNSUPPORTED / BUILD-FAIL). This is the corpus-scale auto-sweep for silent bugs.

Usage: rust_diff_sweep.py --base base.rs --wip wip.rs --outdir DIR [--secs 20] [--only fn1,fn2]
"""
import argparse, re, subprocess, sys, os
from pathlib import Path

HERE = Path(__file__).resolve().parent

def pub_fns(src: str):
    return set(re.findall(r'\bpub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)', src))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True); ap.add_argument("--wip", required=True)
    ap.add_argument("--outdir", required=True); ap.add_argument("--secs", type=int, default=20)
    ap.add_argument("--only", default=""); ap.add_argument("--toolchain", default="nightly-2025-09-01")
    a = ap.parse_args()
    base_src = Path(a.base).read_text(); wip_src = Path(a.wip).read_text()
    common = sorted(pub_fns(base_src) & pub_fns(wip_src))
    if a.only: common = [f for f in common if f in set(a.only.split(","))]
    out = Path(a.outdir); out.mkdir(parents=True, exist_ok=True)
    results = []
    for i, fn in enumerate(common):
        cname = f"rs{i}"
        d = out / fn
        gen = subprocess.run([sys.executable, str(HERE/"rust_diff.py"), "--base", a.base, "--wip", a.wip,
                              "--entry", fn, "--out", str(d), "--crate-name", cname],
                             capture_output=True, text=True)
        if gen.returncode != 0:
            reason = (gen.stdout + gen.stderr).strip().splitlines()[-1][:80] if (gen.stdout+gen.stderr).strip() else "gen-fail"
            results.append((fn, "UNSUPPORTED", reason)); print(f"[{fn}] UNSUPPORTED: {reason}"); continue
        b = subprocess.run(["cargo", f"+{a.toolchain}", "fuzz", "build", f"{fn}_ft"], cwd=d, capture_output=True, text=True)
        if b.returncode != 0:
            err = [l for l in b.stderr.splitlines() if l.startswith("error")]
            results.append((fn, "BUILD-FAIL", (err[0] if err else "build")[:80])); print(f"[{fn}] BUILD-FAIL"); continue
        r = subprocess.run(["cargo", f"+{a.toolchain}", "fuzz", "run", f"{fn}_ft", "--",
                            f"-max_total_time={a.secs}", "-timeout=15"], cwd=d, capture_output=True, text=True)
        div = re.search(r'divergence: (base=.*)', r.stdout + r.stderr)
        if div:
            results.append((fn, "DIVERGENCE", div.group(1)[:80])); print(f"[{fn}] *** DIVERGENCE *** {div.group(1)[:80]}")
        else:
            results.append((fn, "CLEAN", f"~{a.secs}s")); print(f"[{fn}] clean")
    print("\n==== SWEEP SUMMARY ====")
    for fn, st, note in results:
        print(f"  {st:12} {fn:28} {note}")
    n_div = sum(1 for _,s,_ in results if s=="DIVERGENCE")
    print(f"\n{len(results)} fns: {n_div} DIVERGENCE, "
          f"{sum(1 for _,s,_ in results if s=='CLEAN')} clean, "
          f"{sum(1 for _,s,_ in results if s=='UNSUPPORTED')} unsupported, "
          f"{sum(1 for _,s,_ in results if s=='BUILD-FAIL')} build-fail")

if __name__ == "__main__":
    main()
