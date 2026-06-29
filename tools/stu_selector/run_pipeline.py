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

sys.path.insert(0, str(HERE))  # frontier/features/mapping are siblings


def sh(cmd, **kw):
    return subprocess.run(cmd, text=True, capture_output=True, **kw)


def rust_sig(rs_text: str, fn: str) -> tuple[list[str], str] | None:
    """(param types, return type) of `fn` in the translation. Used to STATICALLY detect bridge
    shapes we don't yet support, so they're labelled UNSUPPORTED_BRIDGE (with a reason) up front
    rather than surfacing as an opaque BUILD_FAIL."""
    m = re.search(rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(fn)}\s*\(([^;{{]*?)\)\s*(->\s*([^{{]+?))?\s*\{{', rs_text, re.S)
    if not m:
        return None
    inner, ret = m.group(1).strip(), (m.group(3) or "()").strip()
    parts, depth, cur = [], 0, ""
    for ch in inner:
        depth += ch in "<[(" ; depth -= ch in ">])"
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    ptys = [p.split(":", 1)[1].strip() if ":" in p else p.strip() for p in parts]
    return ptys, ret


def unsupported_reason(rs_text: str, fn: str) -> str | None:
    """Return a stable reason string if the matched Rust fn needs a bridge shape we don't yet
    generate (so the driver can report UNSUPPORTED_BRIDGE instead of attempting + BUILD_FAIL)."""
    sig = rust_sig(rs_text, fn)
    if not sig:
        return None  # let generation try (can't read signature)
    ptys, ret = sig
    rc = ret.replace(" ", "")
    # Bridged now: Option<&[T]>/Option<&mut[T]> slice params; cross-width int returns; and the
    # decode shape `Option<(value, count)>` (C out-param + count return, 0 = None). Still NOT
    # bridged: Result returns, single Option<T> returns, bare multi-tuple returns.
    if rc.startswith("Result<"):
        return "c_retcode_to_rust_result_return"
    if rc.startswith("Option<") and not rc.startswith("Option<("):
        return "c_outparam_to_rust_option_scalar_return"
    if rc.startswith("(") and "," in rc:
        return "c_multi_out_to_rust_tuple_return"
    return None   # Option<(...)> falls through: gen_diff_harness handles the decode shape


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
    ap.add_argument("--json", default=None, help="write a machine-readable summary to this path")
    ap.add_argument("--all", action="store_true",
                    help="test ALL matched functions, bypassing the frontier selector (ablation; "
                    "default only fuzzes the frontier-selected boundaries)")
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
    print(f"   pairs matched: {pairs}")

    # 2b) frontier (C3): pick the fuzzable boundaries; others are recorded SKIPPED_FRONTIER + reason.
    front_names, front_reason = None, {}
    if not args.all:
        try:
            import frontier as fr, features as feat, mapping as mapmod
            # C-only feature rows (generator-agnostic): features_for_pair aligns by name and yields
            # nothing on RENAMED translations; the frontier only needs C-side features.
            rows = feat.c_feature_rows(cpair / "build")
            edges = mapmod.build_c_graph(cpair / "build")["edges"]
            sel = fr.select_frontier(rows, edges)
            front_names = {s["fn"] for s in sel["frontier"]}
            front_reason = {s["fn"]: (s.get("reasons") or []) for s in sel["scored"]}
            print(f"   frontier-selected: {sorted(front_names)}")
        except Exception as e:  # frontier is an optimization; never break the pipeline
            print(f"   (frontier skipped: {e}) — testing all matched")
            front_names = None

    # 3+4) per pair: generate harness (rename + fold bridge) -> fuzz
    env = dict(os.environ, PATH=f"{Path.home()}/.cargo/bin:" + os.environ.get("PATH", ""))
    results = {}
    for c_fn, r_fn in pairs.items():
        print(f"\n== {c_fn} -> {r_fn} ==")
        rec = {"rust": r_fn, "verdict": None, "runs": None, "reason": None}
        # frontier gate (C3): only fuzz selected boundaries unless --all
        if front_names is not None and c_fn not in front_names:
            rec["verdict"] = "SKIPPED_FRONTIER"; rec["reason"] = front_reason.get(c_fn) or ["not on frontier"]
            print(f"   SKIPPED_FRONTIER ({rec['reason']})"); results[c_fn] = rec; continue
        # STATIC pre-check: known-unsupported bridge shapes -> labelled, not attempted (Codex review)
        reason = unsupported_reason(rs_clean, r_fn)
        if reason:
            rec["verdict"] = "UNSUPPORTED_BRIDGE"; rec["reason"] = reason
            print(f"   UNSUPPORTED_BRIDGE ({reason})")
            results[c_fn] = rec; continue
        gen = [PY, str(HERE / "gen_diff_harness.py"), "--pair", str(cpair),
               "--entry", c_fn, "--rust-entry", r_fn, "--expose-entry", "--infer-schema",
               "--out", str(work / c_fn)]
        if not args.no_ub_free:
            gen.append("--ub-free")
        g = sh(gen, env=env)
        if g.returncode != 0 or "generated harness" not in g.stdout:
            rec["verdict"] = "GEN_FAIL"; rec["reason"] = (g.stdout + g.stderr).strip().splitlines()[-1:]
            print(f"   GEN_FAIL: {rec['reason']}"); results[c_fn] = rec; continue
        ft = f"{cpair.name.replace('-', '_')}_ft"   # gen_diff_harness names the target after the pair dir
        run = ["timeout", "-k", "5", "-s", "KILL", str(args.secs + 90),
               "cargo", f"+{args.toolchain}", "fuzz", "run", ft, "--",
               f"-max_total_time={args.secs}", "-timeout=5"]
        fr = sh(run, cwd=str(work / c_fn), env=env)   # cargo-fuzz runs from the project root
        out = fr.stdout + fr.stderr
        if "panicked" in out or "ERROR: libFuzzer" in out or "deadly signal" in out:
            rec["verdict"] = "DIVERGENCE"
        elif re.search(r"Done \d+ runs", out):
            rec["verdict"] = "CLEAN"
        elif "error[" in out or "could not compile" in out or "failed to build" in out:
            rec["verdict"] = "BUILD_FAIL"
        else:
            rec["verdict"] = "UNKNOWN"
        mr = re.search(r"Done (\d+) runs", out)
        rec["runs"] = int(mr.group(1)) if mr else None
        print(f"   {rec['verdict']}  ({rec['runs']} runs)")
        results[c_fn] = rec

    print("\n=== SUMMARY ===")
    for c_fn, rec in results.items():
        extra = f"  [{rec['reason']}]" if rec["reason"] and rec["verdict"] == "UNSUPPORTED_BRIDGE" else ""
        print(f"  {c_fn:22s} -> {rec['rust']:24s}  {rec['verdict']}{extra}")
    summary = {"pair": name, "pairs": results}
    if args.json:
        Path(args.json).write_text(json.dumps(summary, indent=1)); print(f"\nwrote {args.json}")
    else:
        print("\n" + json.dumps(summary))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
