#!/usr/bin/env python3
"""RQ2 — UB-free gate precision, via ARTIFACT REPLAY classification (not two-fuzz comparison).

Per boundary (matched C fn / Rust fn), for a fuzz budget:
  1. generate gate-OFF + gate-ON harnesses (gen_diff_harness.py [--ub-free]);
  2. fuzz gate-OFF -> collect crash artifacts (the UB-blind oracle's reports);
  3. for EACH artifact, REPLAY it under gate-ON (deterministic):
       gate-ON clean  -> UB_SUPPRESSED   (the gate rejected it => C hit UB on that input)
       gate-ON crash  -> UB_FREE_DIVERGENCE (candidate real bug) / GATE_MISS (hard-trap signal)
       no repro       -> REPRO_FAIL
  4. EVIDENCE (best-effort, scalar boundaries): decode the artifact via the harness's own
     take_* recipe, emit a standalone full-UBSan C driver, run it -> UB kind + file:line.

Suppression is defined PER ARTIFACT by replay, never by "gate-ON fuzz found nothing".
See results/rq2_eval_plan.md. Emits JSON + a per-boundary table.

Usage:
  eval_rq2_ubgate.py --boundaries scripts/rq2_boundaries.json --secs 25 \
      --json results/rq2_rows/controls.json
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys, shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GEN = ROOT / "tools" / "stu_selector" / "gen_diff_harness.py"
TOOLCHAIN = "nightly-2025-09-01"
RUST2C = {"i8": "int8_t", "u8": "uint8_t", "i16": "int16_t", "u16": "uint16_t",
          "i32": "int32_t", "u32": "uint32_t", "i64": "int64_t", "u64": "uint64_t",
          "usize": "size_t", "isize": "ssize_t", "i128": "__int128", "void": "void"}
TAKE_W = {"u8": 1, "i8": 1, "u16": 2, "i16": 2, "u32": 4, "i32": 4,
          "u64": 8, "i64": 8, "usize": 8, "isize": 8}
UB_FLAGS = ["-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable",
            "-fsanitize-recover=all"]


def sh(cmd, cwd=None, secs=400):
    try:
        return subprocess.run(cmd, cwd=cwd, text=True, capture_output=True, timeout=secs)
    except subprocess.TimeoutExpired as e:
        dec = lambda x: x.decode(errors="replace") if isinstance(x, bytes) else (x or "")
        return subprocess.CompletedProcess(cmd, 124, dec(e.stdout), dec(e.stderr) + "\n[timeout]")


def gen_harness(pair, entry, out, ub_free, rust_entry):
    # --ignore-schema = always infer the schema from the C signature; RQ2 harvests arbitrary
    # non-entry boundaries, so a program-level schemas/<prog>.json (keyed to its designated entry)
    # would spuriously mismatch. Inference handles scalar/buffer sigs (Table A scope).
    cmd = [sys.executable, str(GEN), "--pair", str(pair), "--entry", entry,
           "--expose-entry", "--ignore-schema", "--out", str(out)]
    if rust_entry:
        cmd += ["--rust-entry", rust_entry]
    if ub_free:
        cmd.append("--ub-free")
    return sh(cmd, secs=120)


def fuzz(harness_dir, target, secs, extra_args=None):
    """Run the fuzz target; return (crashed: bool, stdout+stderr)."""
    cmd = ["cargo", f"+{TOOLCHAIN}", "fuzz", "run", target, "--"] + \
          (extra_args or [f"-max_total_time={secs}"])
    r = sh(cmd, cwd=str(harness_dir), secs=secs + 300)
    return (r.returncode != 0), (r.stdout + r.stderr)


def replay(harness_dir, target, artifact):
    """Replay one artifact once under this harness; return exit code (0 = clean)."""
    r = sh(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", target, str(artifact)],
           cwd=str(harness_dir), secs=300)
    return r.returncode


def artifacts_of(harness_dir, target):
    d = harness_dir / "fuzz" / "artifacts" / target
    if not d.exists():
        return []
    return sorted(f for f in d.iterdir() if f.is_file() and not f.name.endswith(".seed"))


def decode_recipe(target_rs: Path):
    """Extract the ordered (name, take_type) decode calls from the fuzz target."""
    txt = target_rs.read_text()
    body = txt.split("fuzz_target!", 1)[-1]
    return re.findall(r"let\s+(\w+)\s*=\s*cur\.take_(\w+)\(\);", body)


def extern_sig(target_rs: Path, entry):
    """(ret_rust, [(argname, rust_ty)]) from the `fn c_<entry>(...)` extern decl."""
    m = re.search(rf"fn c_{re.escape(entry)}\(([^)]*)\)\s*(->\s*(\w+))?;", target_rs.read_text())
    if not m:
        return None, None
    args = [tuple(p.split(":")) for p in m.group(1).split(",") if ":" in p]
    args = [(a.strip(), b.strip()) for a, b in args]
    return (m.group(3) or "void"), args


def decode_artifact(recipe, data: bytes):
    """Mirror Cur::take_* (LE, zero-fill past end) to recover scalar arg values."""
    vals, p = [], 0
    for _name, ty in recipe:
        w = TAKE_W.get(ty)
        if w is None:
            return None  # non-scalar (vec_*) -> evidence unsupported here
        chunk = bytes(data[p:p + w]) + bytes(max(0, w - (len(data) - p)))
        signed = ty[0] == "i"
        vals.append(int.from_bytes(chunk[:w], "little", signed=signed))
        p += w
    return vals


def evidence(pair, entry, target_rs, artifact, tmp):
    """Best-effort: standalone full-UBSan replay of the decoded input -> UB diagnostic line."""
    recipe = decode_recipe(target_rs)
    ret_rs, sig = extern_sig(target_rs, entry)
    if not recipe or sig is None or len(recipe) != len(sig):
        return None
    vals = decode_artifact(recipe, artifact.read_bytes())
    if vals is None:
        return None
    csrcs = sorted((pair / "source").glob("*.c"))
    if not csrcs:
        return None
    ctypes = ", ".join(RUST2C.get(ty, "long") for _n, ty in sig)
    cret = RUST2C.get(ret_rs, "int")
    lits = ", ".join(str(v) for v in vals)
    drv = tmp / f"ubev_{entry}.c"
    drv.write_text(
        "#include <stdint.h>\n#include <stdio.h>\n#include <sys/types.h>\n"
        f"extern {cret} {entry}({ctypes});\n"
        f"int main(void){{ volatile {cret} r = {entry}({lits}); (void)r; return 0; }}\n")
    binp = tmp / f"ubev_{entry}"
    cc = sh(["clang", *UB_FLAGS, "-g", "-fno-omit-frame-pointer",
             *[str(c) for c in csrcs], str(drv), "-o", str(binp)], secs=120)
    if cc.returncode != 0:
        return {"args": vals, "error": "compile_failed", "detail": cc.stderr[-300:]}
    run = sh([str(binp)], secs=8)
    ub = [l for l in (run.stdout + run.stderr).splitlines() if "runtime error:" in l]
    # returncode != 0 = the C reference did NOT cleanly return on this input: a SIGFPE/SIGSEGV
    # signal (hard-trap UB, e.g. div-by-zero / INT_MIN/-1) or a timeout. Either way C is UB here.
    trapped = run.returncode != 0
    return {"args": vals, "ub_reports": ub, "is_ub": bool(ub) or trapped,
            "tier": "hard-trap" if trapped else ("recoverable" if ub else None),
            "ret": run.returncode}


def classify(gate_on_exit, ev):
    if gate_on_exit == 0:
        return "UB_SUPPRESSED"             # gate rejected -> C hit (recoverable) UB on this input
    # gate-ON still crashed on this input. Only call it a candidate bug with CONCLUSIVE UB-free evidence.
    if ev is None or ev.get("args") is None or "error" in ev:
        return "NEEDS_REVIEW"              # no conclusive evidence (array input, compile/exec failure)
    if ev.get("is_ub"):
        return "GATE_MISS"                 # C IS UB here (hard-trap div/overflow) -> gate couldn't stop it
    return "UB_FREE_DIVERGENCE"            # C provably UB-free AND still diverges -> candidate real bug


def run_boundary(b, secs, workdir):
    name, entry = b["name"], b["entry"]
    pair = ROOT / b["pair"]
    rust_entry = b.get("rust_entry")
    target = f"{pair.name}_ft"
    off, on = workdir / f"{name}_off", workdir / f"{name}_on"
    for d, ub in ((off, False), (on, True)):
        if not (d / "fuzz").exists():
            g = gen_harness(pair, entry, d, ub, rust_entry)
            if g.returncode != 0:
                return {"name": name, "error": "gen_failed", "detail": g.stderr[-300:]}
    off_crash, _log = fuzz(off, target, secs)
    tgt_rs = off / "fuzz" / "fuzz_targets" / f"{target}.rs"
    arts = artifacts_of(off, target)
    results = []
    for art in arts:
        ex = replay(on, target, art)
        ev = None
        try:
            ev = evidence(pair, entry, tgt_rs, art, workdir)
        except Exception as e:  # evidence is best-effort; never break classification
            ev = {"error": f"evidence_exc: {e}"}
        results.append({"artifact": art.name, "gate_on_exit": ex,
                        "class": classify(ex, ev), "evidence": ev})
    cls = {r["class"] for r in results}
    verdict = ("TN" if not off_crash and not arts else
               "SUPPRESSED" if results and cls == {"UB_SUPPRESSED"} else
               "BUG_KEPT" if results and cls == {"UB_FREE_DIVERGENCE"} else
               "GATE_MISS(hard-trap)" if results and cls == {"GATE_MISS"} else
               "NEEDS_REVIEW" if results and cls == {"NEEDS_REVIEW"} else
               "MIXED")
    return {"name": name, "kind": b.get("kind"), "off_crash": off_crash,
            "artifacts": results, "verdict": verdict}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--boundaries", required=True, help="JSON list of {name,pair,entry,rust_entry?,kind}")
    ap.add_argument("--secs", type=int, default=25)
    ap.add_argument("--workdir", default=None)
    ap.add_argument("--json", default=None)
    args = ap.parse_args()
    workdir = Path(args.workdir) if args.workdir else Path(
        "/tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/rq2run")
    workdir.mkdir(parents=True, exist_ok=True)
    boundaries = json.loads(Path(args.boundaries).read_text())
    rows = []
    print(f"{'boundary':20} {'kind':18} {'off':>5} {'#art':>5} {'verdict':18} classes")
    print("-" * 78)
    for b in boundaries:
        r = run_boundary(b, args.secs, workdir)
        rows.append(r)
        if "error" in r:
            print(f"{r['name']:20} [ERROR {r['error']}] {r.get('detail','')[:40]}"); continue
        classes = ",".join(sorted({a["class"] for a in r["artifacts"]})) or "-"
        print(f"{r['name']:20} {str(r.get('kind')):18} {str(r['off_crash']):>5} "
              f"{len(r['artifacts']):>5} {r['verdict']:18} {classes}")
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(rows, indent=1))
        print(f"\nwrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
