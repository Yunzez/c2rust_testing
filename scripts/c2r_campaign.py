#!/usr/bin/env python3
"""Two-phase differential campaign: fork-mode discovery, then per-candidate confirmation.

Design: `docs/harness_oracle_plan.md` section 3.

    DISCOVERY                              CONFIRMATION (per candidate)
    C + Rust, no C sanitizer, fast         A. C ONLY, C built with ASan  -> c_clean / c_dirty
      -> fixed comparison ladder           B. RUST ONLY                  -> rust_clean / rust_dirty
      -> divergence: marker + abort        C. COMBINED replay            -> re-observe the ladder
      -> libFuzzer fork mode keeps going

Discovery MUST run in fork mode: a divergence aborts, a Rust panic is turned into an abort by
libfuzzer-sys, a bad input can segfault C, and some inputs time out. Without
`-fork=1 -ignore_crashes=1 -ignore_timeouts=1 -ignore_ooms=1` the first of those ends the campaign
and the harness yields exactly one artifact.

Fork mode does NOT fix a harness. If a boundary's input model puts every input out of bounds it
still contributes no coverage; fork mode only stops one bad input from killing the run.

C-DEFINEDNESS CHECKING IS THE ADJUDICATION ORACLE, and it lives in confirmation, not in the fuzz
loop. The in-loop UBSan gate is a discovery-side noise filter -- useful, but it is not what decides
anything. A candidate may be confirmed as a translation defect only when NO UB check fires on the
C-only replay -- which raises check coverage, and does not prove C is UB-free. Phase A therefore builds the C oracle with ASan **and full UBSan** (`--c-sanitize`) and runs
it with `C2R_MODE=c-only`: sanitizing both sides at once produces reports that cannot be attributed,
and ASan alone misses value-level UB (an out-of-range double->int cast is UB but not a memory
error, so an ASan-only replay calls C `clean`).

Usage:
  # discovery
  scripts/c2r_campaign.py discover --bin <harness> --corpus <dir> --out <dir> [--seconds 300]
  # confirmation
  scripts/c2r_campaign.py confirm  --bin <harness> --asan-bin <confirmation harness> \\
                                   --candidates <dir> --out <dir>
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import signal
import subprocess
import sys
from pathlib import Path

# The outcome vocabulary is fixed and shared with the emitted harness (gen_diff_harness.py).
OUTCOMES = ("normal", "divergence", "panic", "ub-gated", "signal", "nonzero-exit", "timeout")

_SAN = re.compile(r"(ERROR: AddressSanitizer: [a-z\-]+|ERROR: libFuzzer: [a-z ]+|"
                  r"runtime error: [^\n]{0,120}|SEGV on unknown address)")
_OUT = re.compile(r"C2R_OUTCOME kind=(\w[\w-]*) phase=(\d+) detail=(.*)")


def run_once(binary: Path, mode: str, inp: Path, timeout_s: float, out_dir: Path,
             tag: str) -> dict:
    """One replay of one input in one mode. Never raises; the result IS the observation."""
    ofile = out_dir / f"{tag}.outcome"
    ofile.unlink(missing_ok=True)
    env = dict(os.environ)
    env["C2R_MODE"] = mode
    env["C2R_OUTCOME_FILE"] = str(ofile)
    # Leaks are not a finding here: ownership is not derived, so the harness deliberately does not
    # free what the boundary allocated (docs/harness_oracle_plan.md, rule 8 weakened).
    # symbolize=0 and NO ASAN_SYMBOLIZER_PATH: the in-process symbolizer hangs against this
    # runtime (a replay that takes 110 ms without it sits for the full timeout with it). Frames are
    # symbolized OFFLINE below, from the `(module+0xoffset)` pairs ASan prints anyway.
    env["ASAN_OPTIONS"] = (env.get("ASAN_OPTIONS", "") or "").strip(":") \
        + (":" if env.get("ASAN_OPTIONS") else "") + "detect_leaks=0:symbolize=0"
    env.pop("ASAN_SYMBOLIZER_PATH", None)
    # stderr goes to a FILE, never a pipe: ASan's llvm-symbolizer is a long-lived child that
    # inherits the pipe, so `capture_output=True` never sees EOF and the replay hangs for the
    # whole timeout even though the target exited in milliseconds.
    errf = out_dir / f"{tag}.stderr"
    timed_out = False
    with open(errf, "wb") as fh:
        try:
            p = subprocess.run([str(binary), str(inp), "-runs=1"], env=env, timeout=timeout_s,
                               stdout=subprocess.DEVNULL, stderr=fh)
            rc = p.returncode
        except subprocess.TimeoutExpired:
            timed_out, rc = True, None
    err = errf.read_text(errors="replace")

    reported, phase = None, None
    if ofile.exists():
        for line in ofile.read_text(errors="replace").splitlines():
            m = _OUT.match(line.strip())
            if m and (reported is None or m.group(1) != "normal"):
                reported, phase = m.group(1), int(m.group(2))
    if reported is None:
        for m in _OUT.finditer(err):
            if reported is None or m.group(1) != "normal":
                reported, phase = m.group(1), int(m.group(2))

    if timed_out:
        outcome = "timeout"
    elif reported in ("divergence", "panic", "ub-gated"):
        outcome = reported
    elif rc is not None and rc < 0:
        outcome = "signal"
    elif rc:
        outcome = "signal" if _SAN.search(err) else "nonzero-exit"
    else:
        outcome = "normal"

    san = _SAN.search(err)
    # Detection of a FAR out-of-bounds access is heap-layout dependent, and the two sides' buffers
    # have different neighbours: the same overrun that lands in a poisoned redzone on one side can
    # land inside another live allocation on the other, where ASan says nothing. So asymmetric
    # detection of a far access is not evidence of asymmetric behaviour.
    #
    # ASan says "is a wild pointer" when it cannot attribute the address at all, and otherwise
    # "is located N bytes before/after ... region". Only N == 0 -- an access in the buffer's OWN
    # redzone, which exists identically on both sides -- is symmetric enough to compare.
    _loc = re.search(r"is located (\d+) bytes (?:after|before)", err)
    far = int(_loc.group(1)) if _loc else None
    wild = ("is a wild pointer" in err
            or (far is not None and far > 0)
            or (bool(san) and "AddressSanitizer" in err and _loc is None
                and "is a wild pointer" not in err))
    frames = _symbolize(err, 4)
    return {"mode": mode, "outcome": outcome, "reported": reported, "phase": phase,
            "returncode": rc, "signal": (-rc if rc is not None and rc < 0 else None),
            "sanitizer": san.group(0) if san else None, "wild_address": wild,
            "oob_distance": far,
            "top_frames": frames,
            "stderr_tail": err[-1500:] if outcome != "normal" else ""}


# The same frames repeat across every candidate of a cluster; without this cache confirmation
# spawns one llvm-symbolizer per frame per replay (~10k processes for 845 candidates).
_SYM_CACHE: dict[tuple, str] = {}
_FRAME = re.compile(r"^\s+#(\d+) 0x[0-9a-f]+\s+\(([^)+]+)\+0x([0-9a-f]+)\)")


def _symbolize(err: str, limit: int) -> list[str]:
    """Resolve the top ASan frames offline, one batched addr2line call per module.

    NOT llvm-symbolizer: on this toolchain it never exits, in either `--obj` or stdin mode, so a
    replay that takes 110 ms sits until the timeout. (An earlier check appeared to work only
    because `| head` closed the pipe and killed it.) addr2line returns in ~100 ms and takes all
    the addresses at once.
    """
    hits = [(m.group(1), m.group(2), m.group(3)) for m in
            (_FRAME.match(l) for l in err.splitlines()) if m][:limit]
    if not hits:
        return [l.strip() for l in err.splitlines() if re.match(r"^\s+#\d+ 0x", l)][:limit]
    want: dict[str, list[str]] = {}
    for _, obj, off in hits:
        if (obj, off) not in _SYM_CACHE:
            want.setdefault(obj, []).append(off)
    tool = shutil.which("addr2line")
    for obj, offs in want.items():
        offs = sorted(set(offs))
        if not tool or not Path(obj).exists():
            for o in offs:
                _SYM_CACHE[(obj, o)] = f"{Path(obj).name}+0x{o}"
            continue
        try:
            r = subprocess.run([tool, "-f", "-C", "-e", obj] + [f"0x{o}" for o in offs],
                               stdin=subprocess.DEVNULL, capture_output=True, timeout=30,
                               text=True)
            lines = r.stdout.splitlines()
            for i, o in enumerate(offs):     # addr2line emits two lines per address
                fn = lines[2 * i].strip() if 2 * i < len(lines) else "??"
                loc = lines[2 * i + 1].strip() if 2 * i + 1 < len(lines) else "??"
                _SYM_CACHE[(obj, o)] = (f"{fn} {Path(loc).name}" if fn != "??"
                                        else f"{Path(obj).name}+0x{o}")
        except Exception:
            for o in offs:
                _SYM_CACHE[(obj, o)] = f"{Path(obj).name}+0x{o}"
    return [f"#{n} {_SYM_CACHE.get((obj, off), obj + '+0x' + off)}" for n, obj, off in hits]


def _symbolizer() -> str | None:
    return shutil.which("addr2line")


# ---------------------------------------------------------------------------
def discover(a) -> int:
    out = Path(a.out)
    (out / "candidates").mkdir(parents=True, exist_ok=True)
    corpus = Path(a.corpus)
    corpus.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    env["ASAN_OPTIONS"] = env.get("ASAN_OPTIONS", "") + ":detect_leaks=0"
    env.pop("C2R_OUTCOME_FILE", None)      # one file write per execution would dominate the loop
    cmd = [str(Path(a.bin).resolve()), str(corpus.resolve()),
           "-fork=1", "-ignore_crashes=1", "-ignore_timeouts=1", "-ignore_ooms=1",
           f"-max_total_time={a.seconds}", f"-timeout={a.timeout}",
           f"-rss_limit_mb={a.rss_mb}",
           f"-artifact_prefix={(out / 'candidates').resolve()}/"]
    log = out / "discovery.log"
    print("  " + " ".join(cmd))
    with open(log, "wb") as fh:
        subprocess.run(cmd, env=env, stdout=fh, stderr=subprocess.STDOUT,
                       cwd=str(out), timeout=a.seconds + 300)
    cands = sorted((out / "candidates").glob("*"))
    print(f"discovery: {len(cands)} candidates -> {out / 'candidates'}   (log: {log})")
    return 0


# ---------------------------------------------------------------------------
VERDICTS = {
    "ub_associated": "C itself is not memory-safe on this input: not a finding",
    "ub_associated_termination": (
        "C's `normal` was only apparent -- replaying C alone shows it had already gone out of "
        "bounds. The same illegal access is SILENT UB in C and an explicit panic/abort in the "
        "translation. Not a translation defect: the difference is in how one illegal access "
        "surfaces, not in what the translation computes"),
    "ub_associated_value": (
        "the comparison really did differ, but a UB check fired on C alone for this input, so the "
        "difference is not attributable to the translation. The demonstrated shape: an "
        "out-of-range double->int cast, where C yields INT_MIN and Rust's `as i32` saturates"),
    "confirmed_divergence": "no UB check fired on C and the ladder still differs",
    "confirmed_termination": "C returned normally and the translation did not",
    "not_reproducible": "no UB check fired on C and the ladder agrees on replay",
    "inconclusive": "both sides fail the same way, or the outcome is unstable",
    "out_of_contract_divergence": (
        "the input violates an invariant the C function assumes but never checks (a permutation, an "
        "index range). C goes out of bounds and CONTINUES; the translation traps ON ITS OWN, with no "
        "sanitizer present -- a safety lifter's slice bound, a Rust panic. A real, deterministic "
        "TERMINATION divergence, and NOT a translation defect: C's side of it rests on undefined "
        "behaviour, so there is no defined behaviour for the translation to have got wrong"),
    "out_of_contract_access": (
        "both sides make the same access outside the function's implicit contract. With no "
        "sanitizer the translation takes a SIGSEGV rather than panicking, and a wild read faults "
        "only when its page is unmapped -- so which side dies is memory-layout luck, not "
        "behaviour. Report it as an out-of-contract input, never as a divergence"),
    "instrument_only": (
        "the difference is the INSTRUMENT's, not the program's: with no sanitizer the translation "
        "does not fail either. Both sides make the same out-of-contract access and both continue; "
        "only one side's sanitizer happened to notice, because the two sides' buffers have "
        "different neighbours. Not a divergence at all -- do not report it as one"),
    "ub_gated": "the in-loop UB gate rejects this input: C hits UB before anything is compared",
}


def _program_trap(d: dict | None) -> bool:
    """A trap the PROGRAM raises: our panic hook fired. A SIGSEGV is not one -- whether a wild read
    faults depends on the page map, which is exactly the layout dependence we are trying to rule
    out."""
    return bool(d) and d.get("reported") == "panic"


def classify(a_c: dict, b_rust: dict, c_comb: dict, d_nosan: dict | None = None) -> tuple[str, str]:
    """`d_nosan` is the Rust side replayed with NO sanitizer.

    It separates three things that all look alike in the discovery loop:
      * the translated PROGRAM traps -- a panic from a slice bound or an overflow check. Layout
        independent, deterministic, and the only one that supports a claim about behaviour.
      * the process takes a SIGSEGV on a wild read. Whether a wild address faults depends on
        whether that page is mapped, so this is layout luck, exactly like an ASan report.
      * nothing happens: the failure was the sanitizer's alone.
    """
    if a_c["outcome"] in ("signal", "nonzero-exit") or a_c["sanitizer"]:
        why = f"C alone: {a_c['outcome']}" + (f" ({a_c['sanitizer']})" if a_c["sanitizer"] else "")
        # A panic or a reported divergence on top of a dirty C is NOT a translation defect: the
        # same illegal access is silent UB on one side and an explicit failure on the other.
        if c_comb["outcome"] == "panic" or _program_trap(d_nosan):
            return "ub_associated_termination", (
                why + "; the translation PANICS on its own, with no sanitizer")
        if c_comb["outcome"] == "divergence":
            return "ub_associated_value", why + "; the comparison differed on UB input"
        return "ub_associated", why
    if a_c["outcome"] == "timeout":
        return ("inconclusive" if b_rust["outcome"] == "timeout" else "confirmed_termination",
                "C alone times out")
    # No UB check fired on C from here on. That is check coverage, not a proof of definedness.
    if c_comb["outcome"] == "ub-gated":
        # The in-loop gate saw UB that ASan cannot: an out-of-range double->int cast, a signed
        # overflow. Not a finding, and not `ub_associated` either -- nothing was compared.
        return "ub_gated", "the in-loop UB gate rejected the input"
    if c_comb["outcome"] == "divergence":
        return "confirmed_divergence", f"ladder: {c_comb.get('reported')} at phase {c_comb['phase']}"
    if c_comb["outcome"] == "panic":
        if (c_comb["phase"] or 0) >= 2:
            return "confirmed_termination", "C returned normally, the translation panicked"
        return "inconclusive", "panic before the C call completed"
    if c_comb["outcome"] in ("signal", "nonzero-exit"):
        if b_rust["outcome"] in ("signal", "nonzero-exit") or b_rust["sanitizer"]:
            if d_nosan is not None:
                if _program_trap(d_nosan):
                    return "confirmed_termination", (
                        "no UB check fired on C alone; the translation PANICS on its own, with no "
                        "sanitizer")
                if d_nosan["outcome"] == "normal":
                    return "instrument_only", (
                        "with no sanitizer the translation does not fail either; the failure is "
                        "the instrument's, not the program's")
                return "out_of_contract_access", (
                    f"with no sanitizer the translation takes a {d_nosan['outcome']} rather than "
                    f"panicking; a wild read faults only if its page is unmapped, so the outcome "
                    f"is memory-layout luck on both sides")
            if b_rust.get("wild_address") or c_comb.get("wild_address"):
                d = b_rust.get("oob_distance")
                return "inconclusive_wild_address", (
                    "Rust alone fails on a far/unattributable out-of-bounds access"
                    + (f" ({d} bytes past a region)" if d else " (wild address)")
                    + "; without a sanitizer-free replay (--nosan-bin) this cannot be told apart "
                      "from an instrument artifact")
            return "confirmed_termination", (f"no UB check fired on C alone, Rust alone fails: "
                                             f"{b_rust['sanitizer'] or b_rust['outcome']}")
        return "inconclusive", "fails only when both sides run; neither side fails alone"
    if c_comb["outcome"] == "timeout":
        return "confirmed_termination", "C alone returns, the differential run times out"
    return "not_reproducible", "no UB check fired on C; the ladder agrees on replay"


def confirm(a) -> int:
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    work = out / "replays"
    work.mkdir(exist_ok=True)
    cands = sorted(p for p in Path(a.candidates).glob("*") if p.is_file())
    rows = []
    for i, c in enumerate(cands):
        tag = f"{i:03d}_{c.name[:24]}"
        A = run_once(Path(a.asan_bin), "c-only", c, a.timeout, work, tag + "_A")
        B = run_once(Path(a.asan_bin), "rust-only", c, a.timeout, work, tag + "_B")
        C = run_once(Path(a.bin), "combined", c, a.timeout, work, tag + "_C")
        # Phase D: the translation alone with NO sanitizer. This is what decides whether a trap
        # belongs to the program or to the instrument, and it is cheap.
        D = run_once(Path(a.nosan_bin), "rust-only", c, a.timeout, work, tag + "_D") \
            if a.nosan_bin else None
        verdict, why = classify(A, B, C, D)
        row = {"artifact": c.name, "size": c.stat().st_size, "verdict": verdict,
               "why": why, "c_only": A, "rust_only": B, "combined": C}
        if D is not None:
            row["rust_no_sanitizer"] = D
        rows.append(row)
        print(f"{verdict:22s} {c.name[:44]:46s} {why[:80]}")
    (out / "verdicts.json").write_text(json.dumps(rows, indent=1) + "\n")

    # Candidates are inputs, not defects: one root cause produces hundreds of them, so the report
    # is by CLUSTER (verdict + the frame the failure is in), with the artifact count beside it.
    clusters: dict[tuple, list] = {}
    for r in rows:
        side = r["c_only"] if r["verdict"] == "ub_associated" else r["combined"]
        top = (side.get("top_frames") or ["<no frame>"])[0]
        clusters.setdefault((r["verdict"], side.get("sanitizer"), top), []).append(r["artifact"])
    summary = [{"verdict": k[0], "sanitizer": k[1], "frame": k[2],
                "artifacts": len(v), "example": v[0]}
               for k, v in sorted(clusters.items(), key=lambda kv: -len(kv[1]))]
    (out / "clusters.json").write_text(json.dumps(summary, indent=1) + "\n")

    tally: dict[str, int] = {}
    for r in rows:
        tally[r["verdict"]] = tally.get(r["verdict"], 0) + 1
    print(f"\n{len(rows)} candidates -> {len(summary)} clusters")
    for s in summary:
        print(f"  {s['artifacts']:5d}x  {s['verdict']:22s} {(s['frame'] or '')[:90]}")
    print(json.dumps(tally))
    print(f"wrote {out / 'verdicts.json'} and {out / 'clusters.json'}")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    d = sub.add_parser("discover")
    d.add_argument("--bin", required=True)
    d.add_argument("--corpus", required=True)
    d.add_argument("--out", required=True)
    d.add_argument("--seconds", type=int, default=300)
    d.add_argument("--timeout", type=int, default=10, help="libFuzzer per-input timeout (s)")
    d.add_argument("--rss-mb", type=int, default=4096)
    d.set_defaults(fn=discover)

    c = sub.add_parser("confirm")
    c.add_argument("--bin", required=True, help="the discovery harness")
    c.add_argument("--asan-bin", required=True, help="the --c-sanitize confirmation harness (ASan + full UBSan on the C side)")
    c.add_argument("--nosan-bin",
                   help="the same harness built with `cargo fuzz build --sanitizer none`. Phase D "
                        "replays the translation alone with no sanitizer, which is what separates "
                        "a termination divergence the PROGRAM raises from one only the instrument "
                        "sees. Strongly recommended: without it those two are indistinguishable.")
    c.add_argument("--candidates", required=True)
    c.add_argument("--out", required=True)
    c.add_argument("--timeout", type=float, default=20.0)
    c.set_defaults(fn=confirm)

    a = ap.parse_args()
    return a.fn(a)


if __name__ == "__main__":
    raise SystemExit(main())
