#!/usr/bin/env python3
"""Run one RQ4 cell end to end: plan -> build -> campaign -> coverage -> candidates.

Protocol: `results/rq3_coverage/PROTOCOL.md`. The flow it implements is §4, "one campaign, one
corpus":

    Rust-only campaign at the pre-registered budget
      +- save the corpus
           +- coverage from that corpus              -> the four-set partition
           +- combined replay of the SAME corpus     -> divergence candidates

Coverage and candidates therefore come from one budget and one corpus, and are never two
separately-budgeted experiments sharing a name.

Each planned boundary keeps its own binary: `cargo fuzz` writes one target name per crate, so a
shared CARGO_TARGET_DIR leaves only the last boundary's binary behind (which is how the funnel run
lost every binary it built).

usage:
  scripts/rq4/cell.py --pair <dir> --lib <name> --tool <name> --out <dir>
                      [--plugins <toml>] [--c-source <name>] [--shim <file>] [--defs <json>]
                      [--seconds 300] [--only a,b,c]
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import c2r_funnel as F  # noqa: E402

TOOLCHAIN = "nightly-2025-09-01"
TC = Path.home() / ".rustup/toolchains" / f"{TOOLCHAIN}-x86_64-unknown-linux-gnu" \
     / "lib/rustlib/x86_64-unknown-linux-gnu/bin"


class A:
    def __init__(self, **kw):
        self.__dict__.update(kw)


def build_one(a, pair, entry, private, out_dir, target, sanitize=False, nosan=False):
    """Generate + fix up + build one harness, and keep its binary."""
    shutil.rmtree(out_dir, ignore_errors=True)
    cmd = [sys.executable, str(ROOT / "tools/stu_selector/gen_diff_harness.py"),
           "--pair", str(pair), "--entry", entry, "--rust-entry", entry,
           "--plan", "--ub-free", "--out", str(out_dir)]
    if a.c_source:
        cmd += ["--c-source", a.c_source]
    for p in (a.plugins or []):
        cmd += ["--plugins", p]
    if private:
        cmd += ["--expose-entry"]
    if sanitize:
        cmd += ["--c-sanitize"]
    r = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True, timeout=900)
    if r.returncode:
        return None, (r.stdout + r.stderr)[-300:]
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    err = F.fixups(a, pair, out_dir, entry, private, defs)
    if err:
        return None, err
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), RUSTUP_TOOLCHAIN=TOOLCHAIN)
    cargo = ["cargo", "fuzz", "build"] + (["--sanitizer", "none"] if nosan else [])
    rb = subprocess.run(cargo, cwd=str(out_dir), env=env, capture_output=True, text=True,
                        timeout=1800)
    if rb.returncode:
        errs = [l for l in (rb.stdout + rb.stderr).splitlines() if l.startswith("error")]
        return None, "\n".join(errs[:3])
    bins = list((target / "x86_64-unknown-linux-gnu" / "release").glob("*_ft"))
    if not bins:
        return None, "no binary produced"
    keep = out_dir.parent / f"{entry}{'_san' if sanitize else ('_nosan' if nosan else '')}.bin"
    shutil.copy(bins[0], keep)
    return keep, None


def campaign(binaries: dict, corpus_root: Path, art_root: Path, seconds: int,
             seeds: Path | None = None, snap_root: Path | None = None) -> dict:
    """The one campaign: rust-only, fork mode, every harness of the cell concurrent."""
    procs, logs = {}, {}
    for entry, b in binaries.items():
        c = corpus_root / entry
        c.mkdir(parents=True, exist_ok=True)
        # One fixed seed per corpus. Without it a boundary whose inputs are all equivalent (a
        # zero-argument, deterministic entry) or all fatal (mainSort) finishes with an EMPTY
        # corpus, and its coverage is then simply lost -- five of nineteen bzip2 boundaries on the
        # first run. The seed is deterministic, so the campaign stays reproducible.
        seed = c / "seed_fixed"
        if not seed.exists():
            seed.write_bytes(bytes(range(64)))
        # The library's own shipped samples, encoded into the harness input format. Without them a
        # format-consuming boundary never gets past its header: bzip2's decompressor went from 438
        # corpus inputs to 39 when they were dropped. PROTOCOL.md section 3.
        if seeds and (seeds / entry).is_dir():
            for f in (seeds / entry).iterdir():
                if f.is_file():
                    shutil.copy(f, c / f.name)
        art = art_root / entry
        art.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, C2R_MODE="rust-only", ASAN_OPTIONS="detect_leaks=0")
        env.pop("C2R_OUTCOME_FILE", None)
        log = open(art_root / f"{entry}.fuzz.log", "wb")
        logs[entry] = log
        procs[entry] = subprocess.Popen(
            [str(b), str(c), "-fork=1", "-ignore_crashes=1", "-ignore_timeouts=1",
             "-ignore_ooms=1", f"-max_total_time={seconds}", "-timeout=25",
             # 2 GB, not 8: four cells x 19 harnesses share one machine, and a harness that needs
             # more than 2 GB is leaking rather than exploring.
             "-rss_limit_mb=2048", "-seed=42", f"-artifact_prefix={art}/"],
            env=env, stdout=log, stderr=subprocess.STDOUT)
    # Snapshots make the budget CHECKABLE instead of assumed: if a cell is still growing at the
    # last checkpoint, its budget was insufficient under whatever contention it ran with, and it
    # has to be re-run alone. Hardlink copies, so they cost no disk. PROTOCOL.md section 3.
    snaps = snap_root
    checkpoints = [c for c in (60, 300, 600, 1800, 3600) if c < seconds] if seconds > 120 else []
    t0 = time.time()
    for cp in checkpoints:
        wait = cp - (time.time() - t0)
        if wait > 0:
            time.sleep(wait)
        for e in binaries:
            dst = snaps / f"{e}@{cp}s"
            if not dst.exists():
                try:
                    subprocess.run(["cp", "-al", str(corpus_root / e), str(dst)],
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                except Exception:
                    pass
    deadline = t0 + seconds + 240
    for entry, p in procs.items():
        try:
            p.wait(timeout=max(10, deadline - time.time()))
        except subprocess.TimeoutExpired:
            p.kill()
    for f in logs.values():
        f.close()
    return {e: len(list((corpus_root / e).iterdir())) for e in binaries}


def collect(entry, harness_dir: Path, corpus: Path, out_json: Path, target: Path) -> str:
    """Rust coverage over the saved corpus. rust-only: the C reference is not called."""
    env = dict(os.environ, RUSTUP_TOOLCHAIN=TOOLCHAIN, C2R_MODE="rust-only",
               CARGO_TARGET_DIR=str(target))
    name = next(p.stem for p in (harness_dir / "fuzz" / "fuzz_targets").glob("*.rs"))
    r = subprocess.run(["cargo", "fuzz", "coverage", name, str(corpus)], cwd=str(harness_dir),
                       env=env, capture_output=True, text=True, timeout=3600)
    # `cargo fuzz coverage` IGNORES CARGO_TARGET_DIR: it writes the instrumented binary under the
    # crate's own target/ and the profile under fuzz/coverage/<name>/. Pointing at the shared
    # target dir finds nothing and reports a failure on a run that actually succeeded.
    covdir = harness_dir / "target" / "x86_64-unknown-linux-gnu" / "coverage"
    cov = covdir / "x86_64-unknown-linux-gnu" / "release" / name
    pd = harness_dir / "fuzz" / "coverage" / name / "coverage.profdata"
    try:
        if r.returncode == 0 and cov.exists() and pd.exists():
            with open(out_json, "w") as fh:
                subprocess.run([str(TC / "llvm-cov"), "export", str(cov), f"-instr-profile={pd}"],
                               stdout=fh, stderr=subprocess.DEVNULL, timeout=1800)
            return "batch" if out_json.stat().st_size > 0 else "empty-export"
        if r.returncode and cov.exists():
            # The translation itself crashed on some input, so the batch replay died and took the
            # profile with it. Replay one process per input and keep whatever the survivors
            # produced -- the same fallback the archived collect_tool.sh carries.
            per = harness_dir / "percov"
            shutil.rmtree(per, ignore_errors=True)
            per.mkdir()
            ok = 0
            for f in sorted(corpus.iterdir()):
                e2 = dict(env, LLVM_PROFILE_FILE=str(per / "%m-%p.profraw"))
                try:
                    if subprocess.run([str(cov), "-runs=1", str(f)], env=e2,
                                      stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                                      timeout=60).returncode == 0:
                        ok += 1
                except subprocess.TimeoutExpired:
                    pass
            raws = list(per.glob("*.profraw"))
            if raws:
                pd2 = per / "coverage.profdata"
                subprocess.run([str(TC / "llvm-profdata"), "merge", "-sparse",
                                *[str(x) for x in raws], "-o", str(pd2)],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
                if pd2.exists():
                    with open(out_json, "w") as fh:
                        subprocess.run([str(TC / "llvm-cov"), "export", str(cov),
                                        f"-instr-profile={pd2}"], stdout=fh,
                                       stderr=subprocess.DEVNULL, timeout=1800)
                    shutil.rmtree(per, ignore_errors=True)
                    n = len(list(corpus.iterdir()))
                    return f"per-input ({ok}/{n} completed)"
            shutil.rmtree(per, ignore_errors=True)
        return (f"failed rc={r.returncode}" if r.returncode
                else f"missing {'binary' if not cov.exists() else 'profdata'}")
    finally:
        # each coverage build is ~150 MB and the disk is at 97 %
        shutil.rmtree(covdir, ignore_errors=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pair", required=True)
    ap.add_argument("--lib", required=True)
    ap.add_argument("--tool", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--plugins", action="append")
    ap.add_argument("--c-source")
    ap.add_argument("--shim")
    ap.add_argument("--defs")
    ap.add_argument("--seeds", help="directory of <entry>/ seed corpora, encoded into the "
                                    "harness input format")
    ap.add_argument("--seconds", type=int, default=300)
    ap.add_argument("--only")
    a = ap.parse_args()

    pair, out = Path(a.pair), Path(a.out)
    (out / "harnesses").mkdir(parents=True, exist_ok=True)
    target = out / "target"

    plans = F.plan_all(pair, out)
    if a.only:
        keep = {s.strip() for s in a.only.split(",")}
        plans = [p for p in plans if p["boundary"] in keep]
    planned = [p for p in plans if p["status"] == "planned"]
    print(f"{a.lib} x {a.tool}: {len(planned)} planned of {len(plans)} boundaries", flush=True)

    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    private_set = set(defs.get("private", []))
    rs = next(iter(sorted((pair / "translated").glob("*.rs"))), None)
    rs_text = rs.read_text(errors="replace") if rs else ""
    import re
    binaries, rows = {}, []
    for p in planned:
        b = p["boundary"]
        is_priv = b in private_set or (
            bool(rs_text) and not defs and re.search(
                rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b',
                rs_text) is None)
        binp, err = build_one(a, pair, b, is_priv, out / "harnesses" / b, target)
        rows.append({"boundary": b, "c_static": is_priv, "built": binp is not None,
                     "error": err, "inputs": len(p["inputs"])})
        if binp:
            binaries[b] = binp
        print(f"  build {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}", flush=True)
    print(f"built {len(binaries)}/{len(planned)}", flush=True)

    snap = out / "snapshots"
    snap.mkdir(exist_ok=True)
    sizes = campaign(binaries, out / "corpus", out / "candidates", a.seconds,
                     Path(a.seeds) if a.seeds else None, snap)
    for r in rows:
        r["corpus"] = sizes.get(r["boundary"], 0)
        r["artifacts"] = len(list((out / "candidates" / r["boundary"]).iterdir())) \
            if (out / "candidates" / r["boundary"]).is_dir() else 0
    print("campaign done: " + ", ".join(f"{k}={v}" for k, v in sizes.items()), flush=True)

    (out / "ours").mkdir(exist_ok=True)
    for b in binaries:
        if sizes.get(b, 0) == 0:
            rows_b = next(r for r in rows if r["boundary"] == b)
            rows_b["coverage"] = "empty-corpus"
            continue
        mode = collect(b, out / "harnesses" / b, out / "corpus" / b,
                       out / "ours" / f"{b}.json", target)
        next(r for r in rows if r["boundary"] == b)["coverage"] = mode
        print(f"  coverage {b:30s} {mode}", flush=True)

    snaps = {}
    for d in sorted((out / "snapshots").glob("*@*s")):
        e, cp = d.name.rsplit("@", 1)
        snaps.setdefault(cp[:-1], {})[e] = len(list(d.iterdir()))
    (out / "snapshots.json").write_text(json.dumps(snaps, indent=1) + "\n")
    (out / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
    print(f"\nwrote {out/'funnel.json'} and {len(list((out/'ours').glob('*.json')))} exports")
    print("CELL_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
