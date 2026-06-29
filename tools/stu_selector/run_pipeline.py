#!/usr/bin/env python3
"""End-to-end differential-testing driver (generator-agnostic comparator).

Given a diff-pair (C source + a renamed/restructured Rust translation + compile_commands), this:
  1. analyzes both sides (C: libclang via c_analyzer.py; Rust: rust-analyzer via the `analyzer` bin);
  2. MATCHES C<->Rust functions by structure (matcher.py, names hidden) -> {c_name: rust_name};
  3. for each matched pair, AUTO-GENERATES a differential fuzz harness (gen_diff_harness.py with
     --rust-entry = the matched Rust name; idiomatic C-ABI bridge handles rename + arg folding);
  4. FUZZES each with the in-loop UB-free gate (real libFuzzer) and reports PASS / DIVERGENCE.

This is the pipeline behind the paper: a comparator that works on ANY translator's output, including
ones that rename/fold/restructure (where name-based oracles like Fluorine/RustAssure cannot pair).

Usage:
  run_pipeline.py --pair <dir>           # dir has source/ translated/ build/
                  [--secs 25] [--only fn1,fn2] [--no-ub-free] [--keep]
"""
from __future__ import annotations
import argparse, json, subprocess, sys, os, re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ANALYZER = HERE / "analyzer" / "target" / "release" / "analyzer"
PY = sys.executable


def sh(cmd, **kw):
    return subprocess.run(cmd, text=True, capture_output=True, **kw)


def strip_extern_shims(rs_text: str) -> str:
    """Remove hand-written `extern "C"` C-ABI bridge shims from an idiomatic translation, so the
    matcher sees the real renamed idiomatic fn (and our generator AUTO-bridges it) rather than a
    manual shim. Only strips when the file ALSO has non-extern `fn`s (idiomatic translation); a
    pure-C-ABI translation like c2rust — where every fn is `extern "C"` — is left untouched."""
    lines = rs_text.split("\n")
    has_idiomatic = any(re.match(r'\s*(pub\s+)?fn\s+\w+', l) and 'extern' not in l for l in lines)
    if not has_idiomatic:
        return rs_text
    out, i = [], 0
    while i < len(lines):
        # an extern "C" fn (with optional leading #[...] attrs) -> drop through its closing brace
        j = i
        while j < len(lines) and lines[j].strip().startswith('#['):
            j += 1
        if j < len(lines) and re.match(r'\s*(pub\s+)?(unsafe\s+)?extern\s+"C"\s+fn\s+', lines[j]):
            depth, started = 0, False
            while j < len(lines):
                depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j]:
                    started = True
                if started and depth <= 0:
                    break
                j += 1
            i = j + 1
            continue
        out.append(lines[i]); i += 1
    return "\n".join(out)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--pair", required=True, help="dir with source/ translated/ build/")
    ap.add_argument("--secs", type=int, default=25, help="fuzz seconds per function")
    ap.add_argument("--only", default=None, help="comma-separated C entry names to test (default: all matched)")
    ap.add_argument("--no-ub-free", action="store_true", help="disable the in-loop UB-free gate (ablation)")
    ap.add_argument("--toolchain", default="nightly-2025-09-01")
    args = ap.parse_args()

    pair = Path(args.pair).resolve()
    name = pair.name
    rs_in = next((pair / "translated").glob("*.rs"))
    work = ROOT / "fuzz_gen" / "_pipeline" / name
    work.mkdir(parents=True, exist_ok=True)
    c_json, r_json, pairs_json = work / "c.json", work / "rust.json", work / "pairs.json"

    # Build a CLEAN pair: strip hand-written extern "C" shims so we exercise our AUTO bridge, not a
    # manual one. Both the analyzer and gen_diff_harness read this clean pair.
    rs_clean = strip_extern_shims(rs_in.read_text())
    cpair = work / "pair"
    for sub in ("source", "translated", "build"):
        (cpair / sub).mkdir(parents=True, exist_ok=True)
    for f in (pair / "source").glob("*"):
        (cpair / "source" / f.name).write_text(f.read_text())
    for f in (pair / "build").glob("*"):
        (cpair / "build" / f.name).write_text(f.read_text().replace(str(pair), str(cpair)))
    (cpair / "translated" / rs_in.name).write_text(rs_clean)

    # 1) analyze both sides. The rust-analyzer-based `analyzer` needs a cargo CRATE, so wrap the
    # (shim-stripped) translation .rs into a minimal lib crate.
    print(f"== analyze ({name}) ==")
    crate = work / "crate"
    (crate / "src").mkdir(parents=True, exist_ok=True)
    (crate / "Cargo.toml").write_text(
        '[package]\nname = "t"\nversion = "0.1.0"\nedition = "2021"\n'
        '[lib]\npath = "src/lib.rs"\n[dependencies]\nlibc = "0.2"\n')
    (crate / "src" / "lib.rs").write_text(rs_clean)
    r = sh([str(ANALYZER), str(crate), "--enable-metrics"])
    if r.returncode != 0 or not r.stdout.strip() or '"load_failed"' in r.stdout:
        print("rust analyzer failed:\n" + (r.stdout or r.stderr)[-2000:]); return 1
    r_json.write_text(r.stdout)
    r = sh([PY, str(HERE / "c_analyzer.py"), "--compile-commands", str(pair / "build"), "--enable-metrics"])
    if r.returncode != 0:
        print("c analyzer failed:\n" + r.stderr[-2000:]); return 1
    c_json.write_text(r.stdout)

    # 2) match (names hidden) -> {c_name: rust_name}
    print("== match (structure, names hidden) ==")
    r = sh([PY, str(HERE / "matcher.py"), "--c", str(c_json), "--rust", str(r_json),
            "--emit-pairs", str(pairs_json)])
    print(r.stdout.strip().splitlines()[-1] if r.stdout else r.stderr[-500:])
    if not pairs_json.exists():
        print("matcher emitted no pairs"); return 1
    pairs = json.loads(pairs_json.read_text())
    only = set(args.only.split(",")) if args.only else None
    if only:
        pairs = {c: r for c, r in pairs.items() if c in only}
    print(f"   pairs to test: {pairs}")

    # 3+4) per pair: generate harness (rename + fold bridge) -> fuzz
    env = dict(os.environ, PATH=f"{Path.home()}/.cargo/bin:" + os.environ.get("PATH", ""))
    results = []
    for c_fn, r_fn in pairs.items():
        print(f"\n== {c_fn} -> {r_fn} ==")
        gen = [PY, str(HERE / "gen_diff_harness.py"), "--pair", str(cpair),
               "--entry", c_fn, "--rust-entry", r_fn, "--expose-entry", "--infer-schema",
               "--out", str(work / c_fn)]
        if not args.no_ub_free:
            gen.append("--ub-free")
        g = sh(gen, env=env)
        if g.returncode != 0 or "generated harness" not in g.stdout:
            print(f"   GEN FAILED: {(g.stdout + g.stderr).strip().splitlines()[-1:]}")
            results.append((c_fn, r_fn, "GEN_FAIL")); continue
        ft = f"{cpair.name.replace('-', '_')}_ft"   # gen_diff_harness names the target after the pair dir
        run = ["timeout", "-k", "5", "-s", "KILL", str(args.secs + 90),
               "cargo", f"+{args.toolchain}", "fuzz", "run", ft, "--",
               f"-max_total_time={args.secs}", "-timeout=5"]
        fr = sh(run, cwd=str(work / c_fn), env=env)   # cargo-fuzz runs from the project root
        out = fr.stdout + fr.stderr
        if "panicked" in out or "ERROR: libFuzzer" in out or "deadly signal" in out:
            verdict = "DIVERGENCE"
        elif re.search(r"Done \d+ runs", out):
            verdict = "CLEAN"
        elif "error[" in out or "could not compile" in out:
            verdict = "BUILD_FAIL"
        else:
            verdict = "UNKNOWN"
        runs = (re.search(r"Done (\d+) runs", out) or [None, "?"])[1]
        print(f"   {verdict}  ({runs} runs)")
        results.append((c_fn, r_fn, verdict))

    print("\n=== SUMMARY ===")
    for c_fn, r_fn, v in results:
        print(f"  {c_fn:22s} -> {r_fn:24s}  {v}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
