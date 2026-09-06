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
import re
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MAX_LEN = 4096
GEN_HASH = "unrecorded"
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import c2r_funnel as F  # noqa: E402

TOOLCHAIN = "nightly-2025-09-01"
TC = Path.home() / ".rustup/toolchains" / f"{TOOLCHAIN}-x86_64-unknown-linux-gnu" \
     / "lib/rustlib/x86_64-unknown-linux-gnu/bin"


class A:
    def __init__(self, **kw):
        self.__dict__.update(kw)


def generator_hash() -> str:
    """sha256 over the generator sources a harness depends on, so every funnel row says WHICH
    generator built it (a cell's boundaries may be re-built after a fix; RUN.md must show that)."""
    import hashlib
    h = hashlib.sha256()
    for f in ("tools/stu_selector/gen_diff_harness.py", "tools/stu_selector/harness_plan.py",
              "scripts/c2r_funnel.py", "scripts/flatten_translation.py"):
        h.update((ROOT / f).read_bytes())
    return h.hexdigest()[:16]


def _crate_name(out_dir: Path) -> str:
    m = re.search(r'(?m)^\s*name\s*=\s*"([^"]+)"', (out_dir / "Cargo.toml").read_text(errors="replace"))
    return m.group(1) if m else out_dir.name


def _prune_target(target: Path, crate: str) -> None:
    """Drop THIS harness's artifacts from the shared cargo target after its binary is kept.

    Every harness crate compiles its own copy of the translated library, so a 213-boundary cell
    (tulip) left 213 rlibs + object files + C-oracle build dirs in one target tree: tens of GB,
    and the scratchpad's byte quota killed all four tulip cells in their build phase. Only the
    shared dependencies (libfuzzer-sys, libc, arbitrary) are worth keeping between builds.
    """
    rel = target / "x86_64-unknown-linux-gnu" / "release"
    pats = [crate, crate.replace("-", "_")]
    for sub in ("deps", "build", ".fingerprint"):
        d = rel / sub
        if not d.is_dir():
            continue
        for p in d.iterdir():
            if any(x in p.name for x in pats):
                shutil.rmtree(p, ignore_errors=True) if p.is_dir() else p.unlink(missing_ok=True)
    for p in rel.glob(f"*{pats[1]}*"):
        if p.is_file():
            p.unlink(missing_ok=True)
    shutil.rmtree(rel / "incremental", ignore_errors=True)


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
    r = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True, errors="replace", timeout=900)
    if r.returncode:
        return None, (r.stdout + r.stderr)[-300:]
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    err = F.fixups(a, pair, out_dir, entry, private, defs)
    if err:
        return None, err
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), RUSTUP_TOOLCHAIN=TOOLCHAIN)
    cargo = ["cargo", "fuzz", "build"] + (["--sanitizer", "none"] if nosan else [])
    rb = subprocess.run(cargo, cwd=str(out_dir), env=env, capture_output=True, text=True, errors="replace",
                        timeout=1800)
    if rb.returncode:
        errs = [l for l in (rb.stdout + rb.stderr).splitlines() if l.startswith("error")]
        return None, "\n".join(errs[:3])
    bins = list((target / "x86_64-unknown-linux-gnu" / "release").glob("*_ft"))
    if not bins:
        return None, "no binary produced"
    keep = out_dir.parent / f"{entry}{'_san' if sanitize else ('_nosan' if nosan else '')}.bin"
    shutil.copy(bins[0], keep)
    if not sanitize and not nosan:
        # The campaign binary is fuzzed and replayed, never symbolised: 87 % of it is debug
        # info (tulip: 18.6 MB -> 2.4 MB). The confirmation's _san/_nosan binaries keep theirs.
        subprocess.run(["strip", "--strip-debug", str(keep)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    _prune_target(target, _crate_name(out_dir))
    return keep, None


def sandbox_dir(anchor: Path) -> Path:
    """Where a harness process RUNS. lil's `store` builtin writes a file named by the script, so
    fuzzed scripts left 855 garbage files in the repository root (the chain's cwd). Every harness
    execution now runs inside the cell's own throwaway directory; nothing else changes."""
    sb = anchor / "sandbox"
    sb.mkdir(parents=True, exist_ok=True)
    return sb


def campaign(binaries: dict, corpus_root: Path, art_root: Path, seconds: int,
             seeds: Path | None = None, snap_root: Path | None = None) -> dict:
    """The one campaign: rust-only, fork mode, every harness of the cell concurrent."""
    procs, logs = {}, {}
    sb = sandbox_dir(corpus_root.parent)
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
             "-ignore_ooms=1", f"-max_total_time={seconds}", "-timeout=25", f"-max_len={MAX_LEN}",
             # 2 GB, not 8: four cells x 19 harnesses share one machine, and a harness that needs
             # more than 2 GB is leaking rather than exploring.
             "-rss_limit_mb=2048", "-seed=42", f"-artifact_prefix={art}/"],
            env=env, stdout=log, stderr=subprocess.STDOUT, cwd=str(sb))
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


_STATUS = re.compile(r"^#(\d+): cov: (\d+) ft: (\d+) corp: (\d+) exec/s: (\d+) oom/timeout/crash: (\d+)/(\d+)/(\d+)")


def fuzz_status(log: Path) -> dict:
    """The last fork-mode status line of a libFuzzer log: jobs, corpus, oom/timeout/crash."""
    last = None
    try:
        for line in log.read_text(errors="replace").splitlines():
            m = _STATUS.match(line)
            if m:
                last = m
    except OSError:
        pass
    if not last:
        return {}
    j, cov, ft, corp, ex, oom, to, cr = (int(x) for x in last.groups())
    return {"jobs": j, "cov": cov, "corp": corp, "oom": oom, "timeout": to, "crash": cr,
            "crash_all": j >= 5 and cr >= 0.9 * j and corp <= 1}


def empty_input_probe(binary: Path, mode: str, scratch: Path) -> str:
    """Run the harness ONCE on the empty input in one mode: ok | crash | timeout."""
    empty = scratch / "empty_input"
    empty.write_bytes(b"")
    env = dict(os.environ, C2R_MODE=mode, ASAN_OPTIONS="detect_leaks=0")
    env.pop("C2R_OUTCOME_FILE", None)
    try:
        r = subprocess.run([str(binary), str(empty)], env=env, capture_output=True, timeout=30,
                           cwd=str(sandbox_dir(scratch)))
    except subprocess.TimeoutExpired:
        return "timeout"
    return "ok" if r.returncode == 0 else "crash"


def preflight(binaries: dict, out: Path, seconds: int, accepted: set) -> tuple[dict, list]:
    """The one-minute test run the user asked for after lil_parse burned an hour crashing.

    Two signals per built harness, before the real campaign:
      * the EMPTY input, once in c-only and once in rust-only mode -- a harness that crashes on
        the empty input in C-only mode is a harness bug until proven otherwise (a dangling
        length-0 buffer) or an unconstructible precondition (a parser internal);
      * a short fork-mode run with the campaign's own parameters -- `crashes ~ jobs` with a
        corpus that never grows is the same symptom seen from the other side.
    Nothing from it is merged into the campaign (its corpus is deleted; the budget stays the
    campaign's). Flagged boundaries not in `accepted` stop the cell with exit code 3 so a person
    looks before the hour is spent; the chain moves on to the next cell meanwhile.
    """
    pre = out / "preflight"
    shutil.rmtree(pre, ignore_errors=True)
    (pre / "snapshots").mkdir(parents=True)
    report, flagged = {}, []
    for b, binp in binaries.items():
        report[b] = {"empty_c_only": empty_input_probe(binp, "c-only", pre),
                     "empty_rust_only": empty_input_probe(binp, "rust-only", pre)}
    if seconds > 0:
        campaign(binaries, pre / "corpus", pre / "candidates", seconds, None, pre / "snapshots")
        for b in binaries:
            report[b]["run"] = fuzz_status(pre / "candidates" / f"{b}.fuzz.log")
    for b, r in report.items():
        why = []
        if r["empty_c_only"] == "crash":
            why.append("C side crashes on the EMPTY input")
        if r.get("run", {}).get("crash_all"):
            why.append(f"{seconds}s run: {r['run']['crash']} of {r['run']['jobs']} jobs crashed, corpus {r['run']['corp']}")
        r["flag"] = why
        r["accepted"] = b in accepted
        if why and b not in accepted:
            flagged.append(b)
    (pre / "preflight.json").write_text(json.dumps(
        {"seconds": seconds, "accepted": sorted(accepted), "flagged": flagged, "boundaries": report},
        indent=1) + "\n")
    for d in ("corpus", "snapshots"):
        shutil.rmtree(pre / d, ignore_errors=True)       # the campaign is the campaign
    for b, r in sorted(report.items()):
        if r["flag"]:
            print(f"  preflight {b:30s} {'ACCEPTED' if r['accepted'] else 'FLAGGED '} -- {'; '.join(r['flag'])}", flush=True)
    print(f"preflight: {len(flagged)} flagged, {sum(1 for r in report.values() if r['accepted'] and r['flag'])} accepted crash-all, "
          f"{len(report) - sum(1 for r in report.values() if r['flag'])} clean", flush=True)
    return report, flagged


def collect(entry, harness_dir: Path, corpus: Path, out_json: Path, target: Path) -> str:
    """Rust coverage over the saved corpus. rust-only: the C reference is not called."""
    # detect_leaks=0 exactly as the campaign runs: `cargo fuzz coverage` links ASan, and a boundary
    # that hands back a malloc'd pointer (default_bzalloc) "leaks" it on every replay, so LSan
    # failed the batch AND every per-input run, and the boundary's coverage was lost on three
    # tools before coverage_cmd.log existed to say why.
    env = dict(os.environ, RUSTUP_TOOLCHAIN=TOOLCHAIN, C2R_MODE="rust-only",
               CARGO_TARGET_DIR=str(target), ASAN_OPTIONS="detect_leaks=0")
    name = next(p.stem for p in (harness_dir / "fuzz" / "fuzz_targets").glob("*.rs"))
    # `-timeout=25` is the same per-input bound the campaign ran under. Without it an input that
    # loops (hbMakeCodeLengths on c2rust bzip2: 137 fuzz-time timeouts, and the instrumented
    # binary is slower still) spins the batch replay for the full hour, and the TimeoutExpired
    # that then escaped took the whole cell -- and its 3600 s campaign -- with it. A libFuzzer
    # timeout aborts the batch, which is exactly the crash-style failure the per-input fallback
    # below already handles.
    try:
        r = subprocess.run(["cargo", "fuzz", "coverage", name, str(corpus), "--", "-timeout=25"],
                           cwd=str(harness_dir), env=env, capture_output=True, text=True, errors="replace",
                           timeout=900)
    except subprocess.TimeoutExpired:
        r = subprocess.CompletedProcess(args=[], returncode=124, stdout="", stderr="batch timeout")
    # Keep the tool's own output: `failed rc=1` on three c2rust boundaries could not be explained
    # afterwards because nothing recorded whether the coverage BUILD or the batch REPLAY failed.
    (harness_dir / "coverage_cmd.log").write_text(
        f"rc={r.returncode}\n--- stdout ---\n{r.stdout[-20000:]}\n--- stderr ---\n{r.stderr[-20000:]}\n")
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
                                      timeout=60, cwd=str(sandbox_dir(harness_dir))).returncode == 0:
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
    # libFuzzer's input length cap. 4096 (its default) is enough for string/scalar inputs; a
    # buffer-table boundary (tulip: up to 4 rows x 1024 doubles) needs the fuzzer to be ABLE to
    # control whole rows, so those cells pass a larger cap. Recorded per cell in RUN.md.
    ap.add_argument("--max-len", type=int, default=4096)
    ap.add_argument("--only")
    ap.add_argument("--preflight-seconds", type=int, default=60,
                    help="short fork-mode test run before the campaign (0 = probe the empty input only)")
    ap.add_argument("--preflight-accept", default="",
                    help="comma-separated boundaries whose crash-all is reviewed and accepted "
                         "(also read from <pair>/preflight_accept.txt)")
    ap.add_argument("--reuse-bins", action="store_true",
                    help="use harnesses/<b>.bin when present instead of rebuilding (a killed campaign)")
    ap.add_argument("--no-preflight-stop", action="store_true",
                    help="report the preflight but never stop the cell on a flagged boundary")
    a = ap.parse_args()
    global MAX_LEN, GEN_HASH
    MAX_LEN = a.max_len
    GEN_HASH = generator_hash()
    print(f"generator sources sha256[:16] = {GEN_HASH}", flush=True)

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
        kept = out / "harnesses" / f"{b}.bin"
        if a.reuse_bins and kept.exists() and (out / "harnesses" / b / "Cargo.toml").exists():
            # a cell whose build phase completed but whose campaign was killed with it (the
            # session teardown took the detached chain): the harnesses are the harnesses
            binp, err = kept, None
        else:
            binp, err = build_one(a, pair, b, is_priv, out / "harnesses" / b, target)
        rows.append({"boundary": b, "c_static": is_priv, "built": binp is not None,
                     "error": err, "inputs": len(p["inputs"]),
                     "generator": GEN_HASH if not (a.reuse_bins and binp == kept) else f"{GEN_HASH} (bin reused from the killed run)"})
        if binp:
            binaries[b] = binp
        print(f"  build {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}", flush=True)
    print(f"built {len(binaries)}/{len(planned)}", flush=True)

    accepted = {x.strip() for x in a.preflight_accept.split(",") if x.strip()}
    acc_file = pair / "preflight_accept.txt"
    if acc_file.exists():
        accepted |= {ln.split("#", 1)[0].strip() for ln in acc_file.read_text().splitlines()
                     if ln.split("#", 1)[0].strip()}
    pf_report, pf_flagged = preflight(binaries, out, a.preflight_seconds, accepted)
    for r in rows:
        if r["boundary"] in pf_report:
            r["preflight"] = pf_report[r["boundary"]]
    if pf_flagged and not a.no_preflight_stop:
        (out / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
        print(f"PREFLIGHT_REVIEW {a.lib}/{a.tool}: {', '.join(pf_flagged)} -- cell stopped before the "
              f"campaign; review out/preflight/preflight.json, then re-run with --preflight-accept "
              f"or fix the generator", flush=True)
        return 3

    snap = out / "snapshots"
    snap.mkdir(exist_ok=True)
    sizes = campaign(binaries, out / "corpus", out / "candidates", a.seconds,
                     Path(a.seeds) if a.seeds else None, snap)
    for r in rows:
        r["corpus"] = sizes.get(r["boundary"], 0)
        r["artifacts"] = len(list((out / "candidates" / r["boundary"]).iterdir())) \
            if (out / "candidates" / r["boundary"]).is_dir() else 0
    for r in rows:
        st = fuzz_status(out / "candidates" / f"{r['boundary']}.fuzz.log")
        if st:
            r["campaign_status"] = st
    print("campaign done: " + ", ".join(f"{k}={v}" for k, v in sizes.items()), flush=True)
    _ca = [r["boundary"] for r in rows if r.get("campaign_status", {}).get("crash_all")]
    if _ca:
        print(f"campaign crash-all boundaries (corpus never grew): {', '.join(_ca)}", flush=True)
    # The scratchpad has a FILE-COUNT quota that df cannot see: cJSON's producer inherited the
    # reference's `\u` overflow and 39 boundaries wrote 99 999 crash artifacts, and the first
    # coverage export then failed with EDQUOT. The totals are recorded above; a gzipped manifest
    # (sha256 per artifact) keeps the identity of every one; the first 500 per channel stay,
    # which is more than the 200-per-channel confirmation sample reads.
    import finish_cell as FC
    FC.prune_candidates(out / "candidates", 500)

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
        # the export is kept; the instrumented build behind it (~200 MB per harness) is not
        for junk in ("target", "fuzz/target", "fuzz/coverage", "percov"):
            shutil.rmtree(out / "harnesses" / b / junk, ignore_errors=True)

    snaps = {}
    for d in sorted((out / "snapshots").glob("*@*s")):
        e, cp = d.name.rsplit("@", 1)
        snaps.setdefault(cp[:-1], {})[e] = len(list(d.iterdir()))
    (out / "snapshots.json").write_text(json.dumps(snaps, indent=1) + "\n")
    (out / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
    # the campaign's libFuzzer parameters, so RUN.md states them rather than assuming defaults
    (out / "campaign_params.json").write_text(json.dumps(
        {"mode": "rust-only", "fork": 1, "max_total_time_s": a.seconds, "seed": 42,
         "timeout_s": 25, "rss_limit_mb": 2048, "max_len": MAX_LEN,
         "ignore": ["crashes", "timeouts", "ooms"], "snapshots_s": [60, 300, 600, 1800]},
        indent=1) + "\n")
    print(f"\nwrote {out/'funnel.json'} and {len(list((out/'ours').glob('*.json')))} exports")
    print("CELL_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
