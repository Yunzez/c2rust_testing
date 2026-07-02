#!/usr/bin/env python3
"""OOP harness coverage census over the CRUST-bench c2rust baseline.

For every function in each transpiled c2rust pair, try to auto-generate an out-of-process
harness (gen_oop_harness). Records, per boundary, whether the generator SUPPORTS it (and the
role breakdown) or why not (the SystemExit reason: float buffer, callback, void*, T**, struct,
pointer return, ...). This quantifies the OOP generator's coverage on real code — the number
that decides how strong the RQ1 bug hunt can be — and, for the SUPPORTED ones, optionally runs
a short fuzz campaign to confirm SOUNDNESS (faithful c2rust should be TN; any divergence is a
generator bug or a genuine c2rust bug).

Cheap by default: gen-only (parse + emit + oracle build), NO fuzz. Pass --fuzz to also run a
short campaign per supported boundary (slower; the soundness check).

Usage:
  oop_coverage_census.py --baseline <c2rust_baseline_dir> [--limit N] [--only p1,p2]
                         [--fuzz --secs 10] --json results/rq1_crustbench/oop_coverage.json
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys, shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GEN = ROOT / "tools" / "stu_selector" / "gen_oop_harness.py"
TOOLCHAIN = "nightly-2025-09-01"
WORK = Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/"
            "1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/oop_census")

# every function c2rust emits is `extern "C" fn NAME(` (pub or private) -> the full boundary set
FN_RE = re.compile(r'(?:pub\s+)?(?:unsafe\s+)?extern\s+"C"\s+fn\s+(\w+)\s*\(')


def boundaries_of(pair: Path):
    rs = sorted((pair / "translated").glob("*.rs"))
    if not rs:
        return []
    return sorted(set(FN_RE.findall(rs[0].read_text())))


def gen_one(pair: Path, entry: str, out: Path):
    r = subprocess.run([sys.executable, str(GEN), "--pair", str(pair), "--entry", entry,
                        "--out", str(out)], text=True, capture_output=True, timeout=180)
    return r


def reason_of(stderr: str) -> str:
    """Bucket the gen failure into a coarse unsupported category."""
    s = stderr
    for pat, tag in [
        (r"float buffer", "float_buffer"), (r"float out-scalar", "float_out"),
        (r"callback|function pointer", "callback"),
        (r"pointer-to-pointer|T\*\* param", "ptr_ptr"),
        (r"struct-invariant|non-POD|struct field", "struct"),
        (r"pointer-to-array", "ptr_array"),
        (r"unsupported pointer target", "ptr_target"),
        (r"unsupported param type", "param_type"),
        (r"ORACLE_BUILD_FAIL", "oracle_build_fail"),
        (r"no schema|IndexError|list index", "parse_issue"),
    ]:
        if re.search(pat, s):
            return tag
    return "other"


def run_fuzz(out: Path, crate: str, secs: int):
    r = subprocess.run(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", f"{crate}_ft",
                        "--", f"-max_total_time={secs}"], cwd=str(out), text=True,
                       capture_output=True, timeout=secs + 300)
    txt = r.stdout + r.stderr
    if r.returncode == 0:
        return "TN"
    if "divergence" in txt:
        return "DIVERGENCE"
    if re.search(r"AddressSanitizer|SEGV|panicked", txt):
        return "RUST_CRASH"
    return "BUILD_OR_OTHER_FAIL"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--baseline", required=True, help="dir of transpiled c2rust pairs (per-program subdirs)")
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--only", default=None)
    ap.add_argument("--fuzz", action="store_true")
    ap.add_argument("--secs", type=int, default=10)
    ap.add_argument("--json", default=None)
    args = ap.parse_args()
    base = Path(args.baseline)
    WORK.mkdir(parents=True, exist_ok=True)
    pairs = sorted(p for p in base.iterdir() if p.is_dir() and (p / "translated").exists())
    if args.only:
        keep = set(args.only.split(",")); pairs = [p for p in pairs if p.name in keep]
    if args.limit:
        pairs = pairs[:args.limit]

    rows = []
    n_sup = n_unsup = n_tn = n_div = 0
    print(f"{'program':22} {'fns':>4} {'sup':>4} {'unsup':>5}  {'fuzz(TN/DIV)' if args.fuzz else ''}")
    print("-" * 60)
    for pair in pairs:
        fns = boundaries_of(pair)
        prow = {"program": pair.name, "n_fns": len(fns), "boundaries": []}
        sup = unsup = tn = div = 0
        for fn in fns:
            wd = WORK / f"{pair.name}__{fn}"
            shutil.rmtree(wd, ignore_errors=True)
            try:
                g = gen_one(pair, fn, wd)
            except subprocess.TimeoutExpired:
                prow["boundaries"].append({"fn": fn, "status": "gen_timeout"}); unsup += 1; continue
            if g.returncode != 0:
                prow["boundaries"].append({"fn": fn, "status": "UNSUPPORTED",
                                           "reason": reason_of(g.stderr + g.stdout)})
                unsup += 1
                shutil.rmtree(wd, ignore_errors=True)
                continue
            sup += 1
            b = {"fn": fn, "status": "SUPPORTED"}
            if args.fuzz:
                crate = re.sub(r"[^a-zA-Z0-9_]", "_", f"oop_{pair.name}")
                try:
                    v = run_fuzz(wd, crate, args.secs)
                except subprocess.TimeoutExpired:
                    v = "fuzz_timeout"
                b["fuzz"] = v
                tn += v == "TN"; div += v in ("DIVERGENCE", "RUST_CRASH")
            prow["boundaries"].append(b)
            shutil.rmtree(wd, ignore_errors=True)   # disk hygiene
        prow.update(supported=sup, unsupported=unsup, tn=tn, divergence=div)
        rows.append(prow)
        n_sup += sup; n_unsup += unsup; n_tn += tn; n_div += div
        extra = f"  {tn}/{div}" if args.fuzz else ""
        print(f"{pair.name:22} {len(fns):>4} {sup:>4} {unsup:>5}{extra}")
    print("-" * 60)
    tot = n_sup + n_unsup
    print(f"TOTAL: {n_sup}/{tot} supported ({100*n_sup//max(tot,1)}%)"
          + (f"; fuzz: {n_tn} TN, {n_div} divergence" if args.fuzz else ""))
    # unsupported reason breakdown
    from collections import Counter
    reasons = Counter(b.get("reason") for r in rows for b in r["boundaries"] if b["status"] == "UNSUPPORTED")
    print("unsupported reasons:", dict(reasons))
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(
            {"summary": {"supported": n_sup, "total": tot, "tn": n_tn, "divergence": n_div,
                         "reasons": dict(reasons)}, "programs": rows}, indent=1))
        print(f"wrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
