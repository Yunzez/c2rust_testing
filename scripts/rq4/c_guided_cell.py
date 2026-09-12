#!/usr/bin/env python3
"""Controlled guidance/replay experiment for one archived RQ4 cell.

The archived campaign supplies parameters and initial seed files only. This
driver rebuilds the archived-built boundary set once, grows fresh rust-only and
c-only corpora sequentially on that frozen harness set, and replays both corpora
on both implementations. See docs/c_guided_controlled_plan.md.

Reboot safety (the 2026-09-12 reboot erased a day of /tmp): every stage writes its products to <out> on /home
atomically and drops a marker (BUILD_DONE, CAMPAIGN_DONE, C_MEASURE_DONE, RUST_MEASURE_DONE); DONE.json is written
LAST and is the only completion signal. A rerun restores each completed arm
from its persisted corpus instead of fuzzing it again. The work directory in
/tmp holds only rebuildable products.

Reach only: candidates found by the C-guided campaign are NOT adjudicated here.

The ``--single-c-companion`` mode grows only a fresh C-guided corpus.  It uses
the archived Rust-guided result as the baseline and replays the new corpus on
both C and Rust; it never launches a fresh Rust-guided fuzzing arm.

usage: c_guided_cell.py --lib L --tool T --work E --out O [--seconds N]
                        [--only b1,b2] [--single-c-companion] [--keep-work]
"""
import argparse, fcntl, gzip, hashlib, json, os, re, shutil, subprocess, sys, tarfile, time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cell as CELL
import c_reach as CR
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import c2r_funnel as F

ROOT = Path(__file__).resolve().parents[2]
SHA1 = re.compile(r"^[0-9a-f]{40}$")
ORACLE_LINES = re.compile(r"llvm_profile_runtime|C2R_PROFILE_RUNTIME_REF|C2R_NAN_EQ|c2r_feq|c2r_fslice|nan_equivalent|NaN")


def log(*a):
    print(*a, flush=True)


# ----------------------------------------------------------------------------- atomic persistence
def wjson(path: Path, obj) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with open(tmp, "w") as fh:
        json.dump(obj, fh, indent=1)
        fh.write("\n"); fh.flush(); os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def wtext(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with open(tmp, "w") as fh:
        fh.write(text); fh.flush(); os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def wtar(path: Path, src_dir: Path, arcname: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with tarfile.open(tmp, "w:gz") as tf:
        tf.add(src_dir, arcname=arcname)
    with open(tmp, "rb") as fh:
        os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def fsync_dir(path: Path) -> None:
    """Make the preceding rename durable across a sudden reboot."""
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def file_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def marker(O: Path, name: str, extra=None) -> None:
    wjson(O / name, {"at": time.strftime("%Y-%m-%dT%H:%M:%S"), **(extra or {})})


# ----------------------------------------------------------------------------- archived parameters
def archived_params(cell: Path) -> dict:
    """The archived campaign's libFuzzer parameters: campaign_params.json when present, else the
    `libFuzzer parameters:` line of RUN.md; cell.py's defaults otherwise (recorded as such)."""
    p = cell / "campaign_params.json"
    if p.exists():
        d = json.load(open(p)); d["source"] = "campaign_params.json"; return d
    d = {"max_total_time_s": 3600, "seed": 42, "timeout_s": 25, "rss_limit_mb": 2048, "max_len": CELL.MAX_LEN, "source": "cell.py defaults"}
    run = cell / "RUN.md"
    if run.exists():
        t = run.read_text(errors="replace")
        m = re.search(r"libFuzzer parameters:.*", t)
        if m:
            line = m.group(0); d["source"] = "RUN.md"
            for k, key in (("max_total_time_s", "max_total_time_s"), ("seed", "seed"), ("timeout_s", "timeout_s"),
                           ("rss_limit_mb", "rss_limit_mb"), ("max_len", "max_len")):
                mm = re.search(rf"{k}=(\d+)", line)
                if mm:
                    d[key] = int(mm.group(1))
    return d


def initial_corpus(cr_root: Path, b: Path) -> list[str]:
    """The archived campaign's INITIAL inputs for a boundary = the archived corpus files that are not
    libFuzzer-generated (sha1 names): `seed_fixed` and the shipped samples (bzip2 `words_*`, ...)."""
    d = cr_root / b
    return sorted(f.name for f in d.iterdir() if f.is_file() and not SHA1.match(f.name)) if d.exists() else []


# ----------------------------------------------------------------------------- build
def build_keep(a, pair: Path, entry: str, private: bool, hd: Path, target: Path, base: Path):
    """Generate (--c-coverage) + fixups + `cargo fuzz build` (the campaign's default sanitizer); keep the harness
    TREE (the Rust coverage build needs it), the stripped binary, the C objects and the edited C copies."""
    shutil.rmtree(hd, ignore_errors=True)
    cmd = [sys.executable, str(ROOT / "tools/stu_selector/gen_diff_harness.py"),
           "--pair", str(pair), "--entry", entry, "--rust-entry", F.rust_name(pair, entry),
           "--plan", "--ub-free", "--c-coverage", "--out", str(hd)]
    if a.c_source:
        cmd += ["--c-source", a.c_source]
    for p in (a.plugins or []):
        cmd += ["--plugins", p]
    if private:
        cmd += ["--expose-entry"]
    r = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True, errors="replace", timeout=900)
    if r.returncode:
        return None, (r.stdout + r.stderr)[-300:]
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    err = F.fixups(a, pair, hd, entry, private, defs)
    if err:
        return None, err
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), RUSTUP_TOOLCHAIN=CELL.TOOLCHAIN)
    rb = subprocess.run(["cargo", "fuzz", "build"], cwd=str(hd), env=env, capture_output=True, text=True,
                        errors="replace", timeout=1800)
    if rb.returncode:
        errs = [l for l in (rb.stdout + rb.stderr).splitlines() if l.startswith("error")]
        return None, "\n".join(errs[:3]) or rb.stderr[-300:]
    bins = list((target / "x86_64-unknown-linux-gnu" / "release").glob("*_ft"))
    if not bins:
        return None, "no binary produced"
    keep = base / "bins" / f"{entry}.bin"
    keep.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy(bins[0], keep)
    # the C coverage export reads the OBJECT files, never this binary; its debug info only costs tmpfs
    subprocess.run(["strip", "--strip-debug", str(keep)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    outs = sorted((target / "x86_64-unknown-linux-gnu" / "release" / "build").glob("*/out/libc_oracle.a"),
                  key=lambda p: p.stat().st_mtime)
    od = base / "objs" / entry
    shutil.rmtree(od, ignore_errors=True); od.mkdir(parents=True)
    if outs:
        src = outs[-1].parent
        for o in src.rglob("*.o"):
            shutil.copy(o, od / o.relative_to(src).as_posix().replace("/", "__"))
    for f in (hd / "c").rglob("*") if (hd / "c").exists() else []:
        if f.is_file() and f.name not in CR.HELPER_FILES:
            cn = next(iter((pair / "source").rglob(f.name)), None)
            if cn is not None and cn.read_bytes() != f.read_bytes():
                dst = base / "csrc" / entry / f.relative_to(hd / "c")
                dst.parent.mkdir(parents=True, exist_ok=True); shutil.copy(f, dst)
    CELL._prune_target(target, CELL._crate_name(hd))
    return keep, None


def harness_drift(cell: Path, hd: Path, entry: str) -> dict:
    """Regenerated fuzz target vs the archived one, ignoring the lines the --c-coverage flag and the 0.9 NaN
    oracle add: identical means the decoder/materialization the archived corpus was fuzzed under is the one
    both corpora are replayed under now."""
    arch = next(iter((cell / "harnesses" / entry).glob("*_ft.rs")), None)
    gen = next(iter((hd / "fuzz" / "fuzz_targets").glob("*.rs")), None)
    if not arch or not gen:
        return {"archived_source": arch is not None, "regenerated_source": gen is not None}
    A = [l for l in arch.read_text(errors="replace").splitlines() if l.strip() and not l.strip().startswith("//") and not ORACLE_LINES.search(l)]
    G = [l for l in gen.read_text(errors="replace").splitlines() if l.strip() and not l.strip().startswith("//") and not ORACLE_LINES.search(l)]
    import difflib
    diff = [l for l in difflib.unified_diff(A, G, lineterm="", n=0) if l.startswith(("+", "-")) and not l.startswith(("+++", "---"))]
    return {"identical_modulo_oracle": not diff, "diff_lines": len(diff), "sample": diff[:6]}


# ----------------------------------------------------------------------------- campaign
def campaign_guided(binaries: dict, corpus_root: Path, art_root: Path, params: dict, snap_root: Path,
                    seeds: dict, O: Path, mode: str, tag: str) -> dict:
    """Run one fresh guidance arm on the frozen build.

    ``mode`` is rust-only or c-only. Both arms receive byte-identical initial
    corpora and archived campaign parameters. Snapshots are persisted under
    the arm tag; they are evidence checkpoints, not resume points.
    """
    if mode not in {"rust-only", "c-only"}:
        raise ValueError(f"unsupported guidance mode: {mode}")
    procs, logs = {}, {}
    sb = CELL.sandbox_dir(corpus_root.parent)
    seconds = int(params["max_total_time_s"])
    for entry, b in binaries.items():
        c = corpus_root / entry
        c.mkdir(parents=True, exist_ok=True)
        for name, data in seeds.get(entry, {}).items():
            (c / name).write_bytes(data)
        if not (c / "seed_fixed").exists():
            (c / "seed_fixed").write_bytes(bytes(range(64)))
        art = art_root / entry
        art.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, C2R_MODE=mode, ASAN_OPTIONS="detect_leaks=0")
        env.pop("C2R_OUTCOME_FILE", None); env.pop("LLVM_PROFILE_FILE", None)
        lg = open(art_root / f"{entry}.fuzz.log", "wb")
        logs[entry] = lg
        procs[entry] = subprocess.Popen(
            [str(b), str(c), "-fork=1", "-ignore_crashes=1", "-ignore_timeouts=1", "-ignore_ooms=1",
             f"-max_total_time={seconds}", f"-timeout={params['timeout_s']}", f"-max_len={params['max_len']}",
             f"-rss_limit_mb={params['rss_limit_mb']}", f"-seed={params['seed']}", f"-artifact_prefix={art}/"],
            env=env, stdout=lg, stderr=subprocess.STDOUT, cwd=str(sb))
    checkpoints = [c for c in (60, 300, 600, 1800, 3600) if c < seconds] if seconds > 120 else []
    t0 = time.time()
    for cp in checkpoints:
        wait = cp - (time.time() - t0)
        if wait > 0:
            time.sleep(wait)
        for e in binaries:
            dst = snap_root / f"{e}@{cp}s"
            if not dst.exists():
                subprocess.run(["cp", "-al", str(corpus_root / e), str(dst)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        try:
            wtar(O / "snapshots" / f"{tag}_snapshot@{cp}s.tar.gz", corpus_root, "corpus")
        except Exception as ex:
            log(f"  snapshot persistence at {cp}s failed: {ex}")
    deadline = t0 + seconds + 240
    for entry, p in procs.items():
        try:
            p.wait(timeout=max(10, deadline - time.time()))
        except subprocess.TimeoutExpired:
            p.kill()
    for f in logs.values():
        f.close()
    return {e: len(list((corpus_root / e).iterdir())) for e in binaries}


def campaign_c_only(binaries: dict, corpus_root: Path, art_root: Path, params: dict, snap_root: Path,
                    seeds: dict, O: Path) -> dict:
    """Compatibility wrapper for older one-arm callers."""
    return campaign_guided(binaries, corpus_root, art_root, params, snap_root, seeds, O, "c-only", "c")


# ----------------------------------------------------------------------------- universe / rust
def _lib_path_map(export: Path, pair: Path, tag: str) -> list[str]:
    """c2r_coverage aligns every export's lines through the `src/lib.rs` the export names. The archived
    universe exports name the tests/denom crate under the OLD scratchpad (/tmp, wiped by the 2026-09-12 reboot).
    That crate compiled the pair's flattened translation unchanged, so a stand-in dir on /home holding a copy
    of `translated/<pair>.rs` as src/lib.rs gives the same (identity) alignment; mapped with --path-map."""
    try:
        j = json.load(open(export))
        p = next(f["filenames"][0] for f in j["data"][0]["functions"] if f["filenames"][0].endswith("src/lib.rs"))
    except Exception:
        return []
    old = os.path.dirname(os.path.dirname(p))
    if os.path.exists(p):
        return []
    rs = next(iter(sorted((pair / "translated").glob("*.rs"))))
    stand = Path("/home/yunzez/c2rust_archive/universe_libs") / f"{tag}_{pair.name}" / "src"
    stand.mkdir(parents=True, exist_ok=True)
    if not (stand / "lib.rs").exists() or (stand / "lib.rs").read_bytes() != rs.read_bytes():
        shutil.copy(rs, stand / "lib.rs")
    return ["--path-map", f"{old}={stand.parent}"]


def universe_args(cell: Path, E: Path, pair: Path | None = None) -> list[str]:
    tc = cell / "raw" / "tests_coverage.json"
    if tc.exists() and tc.stat().st_size > 0:
        return ["--tests", str(tc)] + (_lib_path_map(tc, pair, "tests") if pair else [])
    dn = cell / "raw" / "denominator.json"
    if dn.exists():
        return ["--denominator", str(dn)] + (_lib_path_map(dn, pair, "denom") if pair else [])
    tars = sorted((cell / "raw").glob("denom_*.tar.gz"))
    if not tars:
        sys.exit(f"no universe for {cell}")
    ud = E / "universe"; ud.mkdir(parents=True, exist_ok=True)
    with tarfile.open(tars[0]) as tf:
        tf.extractall(ud)
    d = next(iter(ud.rglob("denominator.json")))
    j = json.load(open(d))
    p = [f["filenames"][0] for f in j["data"][0]["functions"] if f["filenames"][0].endswith("src/lib.rs")][0]
    old = os.path.dirname(os.path.dirname(p))
    return ["--denominator", str(d), "--path-map", f"{old}={d.parent}"]


def _per_input(cov: Path, corpus: Path, per: Path, env: dict, cwd: Path):
    shutil.rmtree(per, ignore_errors=True); per.mkdir(parents=True)
    ok = 0; files = sorted(p for p in corpus.iterdir() if p.is_file())
    for f in files:
        e2 = dict(env, LLVM_PROFILE_FILE=str(per / "%m-%p.profraw"))
        try:
            if subprocess.run([str(cov), "-runs=1", "-timeout=25", str(f)], env=e2, stdout=subprocess.DEVNULL,
                              stderr=subprocess.DEVNULL, timeout=60, cwd=str(cwd)).returncode == 0:
                ok += 1
        except subprocess.TimeoutExpired:
            pass
    raws = list(per.glob("*.profraw"))
    if not raws:
        return None, f"per-input (0/{len(files)} completed, no profile)"
    pd = per / "coverage.profdata"
    subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-sparse", *map(str, raws), "-o", str(pd)],
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
    return (pd if pd.exists() else None), f"per-input ({ok}/{len(files)} completed)"


def rust_measure_two(E: Path, cell: Path, pair: Path, lib: str, tool: str, boundaries, corpora: dict, out_root: Path, O: Path):
    """Rust reach of SEVERAL corpora on the SAME rebuilt harness: one `cargo fuzz coverage` build per harness
    (its batch replay serves the first corpus), the other corpora replayed on that instrumented binary
    (-runs=0 batch, per-input fallback); c2r_coverage.py per corpus against the archived universe. Exports are
    persisted to <out>/rust_<tag>/ours as they are produced."""
    status = {tag: {} for tag in corpora}
    for b in boundaries:
        hd = E / "base" / "harnesses" / b
        name = next(p.stem for p in (hd / "fuzz" / "fuzz_targets").glob("*.rs"))
        env = dict(os.environ, RUSTUP_TOOLCHAIN=CELL.TOOLCHAIN, C2R_MODE="rust-only",
                   CARGO_TARGET_DIR=str(E / "target"), ASAN_OPTIONS="detect_leaks=0")
        covdir = hd / "target" / "x86_64-unknown-linux-gnu" / "coverage"
        cov = covdir / "x86_64-unknown-linux-gnu" / "release" / name
        built = False
        try:
            for tag, root in corpora.items():
                corpus = root / b
                outj = out_root / f"rust_{tag}" / "ours" / f"{b}.json"; outj.parent.mkdir(parents=True, exist_ok=True)
                if not corpus.exists() or not any(p.is_file() for p in corpus.iterdir()):
                    status[tag][b] = "empty-corpus"; continue
                pd = None
                if not built:
                    r = subprocess.run(["cargo", "fuzz", "coverage", name, str(corpus), "--", "-timeout=25"], cwd=str(hd), env=env,
                                       capture_output=True, text=True, errors="replace", timeout=1800)
                    built = cov.exists()
                    pd0 = hd / "fuzz" / "coverage" / name / "coverage.profdata"
                    if r.returncode == 0 and built and pd0.exists():
                        pd, st = pd0, "batch"
                    elif built:
                        pd, st = _per_input(cov, corpus, hd / f"per_{tag}", env, CELL.sandbox_dir(hd))
                    else:
                        status[tag][b] = f"coverage build failed rc={r.returncode}"; continue
                else:
                    per = hd / f"batch_{tag}"; shutil.rmtree(per, ignore_errors=True); per.mkdir()
                    e2 = dict(env, LLVM_PROFILE_FILE=str(per / "%m-%p.profraw"))
                    try:
                        rr = subprocess.run([str(cov), "-runs=0", "-timeout=25", str(corpus)], env=e2, cwd=str(CELL.sandbox_dir(hd)),
                                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=900).returncode
                    except subprocess.TimeoutExpired:
                        rr = 124
                    raws = list(per.glob("*.profraw"))
                    if rr == 0 and raws:
                        pd = per / "coverage.profdata"
                        subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-sparse", *map(str, raws), "-o", str(pd)],
                                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
                        st = "batch"
                        if not pd.exists():
                            pd = None
                    if pd is None:
                        pd, st = _per_input(cov, corpus, hd / f"per_{tag}", env, CELL.sandbox_dir(hd))
                if pd is None:
                    status[tag][b] = st; continue
                with open(outj, "w") as fh:
                    subprocess.run([str(CELL.TC / "llvm-cov"), "export", str(cov), f"-instr-profile={pd}"], stdout=fh,
                                   stderr=subprocess.DEVNULL, timeout=1800)
                status[tag][b] = st if outj.stat().st_size > 0 else "empty-export"
                (O / f"rust_exports_{tag}").mkdir(parents=True, exist_ok=True)
                tmpg = O / f"rust_exports_{tag}" / f"{b}.json.gz.tmp"
                with gzip.open(tmpg, "wt") as gz:
                    gz.write(outj.read_text())
                os.replace(tmpg, O / f"rust_exports_{tag}" / f"{b}.json.gz")
            log(f"  rust coverage {b:30s} " + " ".join(f"{tag}={status[tag].get(b)}" for tag in corpora))
        finally:
            # keep src/lib.rs (c2r_coverage aligns lines through the export's own lib.rs path); drop the rest
            for child in list(hd.iterdir()):
                if child.name != "src":
                    shutil.rmtree(child, ignore_errors=True) if child.is_dir() else child.unlink(missing_ok=True)
            for child in list((hd / "src").iterdir()) if (hd / "src").exists() else []:
                if child.name != "lib.rs":
                    shutil.rmtree(child, ignore_errors=True) if child.is_dir() else child.unlink(missing_ok=True)
    for tag, root in corpora.items():
        ours = out_root / f"rust_{tag}" / "ours"
        for f in list(ours.glob("*.json")):
            try:
                json.load(open(f))
            except Exception:
                f.unlink()
        cmd = [sys.executable, str(ROOT / "scripts/c2r_coverage.py"), "--linemap", str(pair / "translated" / f"{lib}_{tool}.rs.linemap.json"),
               "--ours", str(ours), *universe_args(cell, E, pair), "--out", str(out_root / f"rust_{tag}" / "analysis"), "--corpus-root", str(root)]
        r = subprocess.run(cmd, capture_output=True, text=True, errors="replace")
        wtext(O / f"rust_{tag}_analysis.log", r.stdout + r.stderr)
        an = out_root / f"rust_{tag}" / "analysis"
        if an.exists():
            shutil.copytree(an, O / f"rust_{tag}_analysis", dirs_exist_ok=True)
    return status


# ----------------------------------------------------------------------------- C
def c_measure(E: Path, pair: Path, boundaries, corpus_root: Path, tag: str, O: Path, empty_pd: Path):
    funcs_u, regs_u, per_b, per_input_all, rows = {}, {}, {}, {}, {}
    (O / f"c_exports_{tag}").mkdir(parents=True, exist_ok=True)
    for b in boundaries:
        binp = E / "base" / "bins" / f"{b}.bin"; corpus = corpus_root / b
        objs = sorted(p for p in (E / "base" / "objs" / b).glob("*.o")
                      if p.name.split("-", 1)[-1].replace("__", "/").rsplit("/", 1)[-1][:-2] + ".c" not in CR.HELPER_FILES)
        if not binp.exists() or not objs or not corpus.exists():
            rows[b] = "missing binary/objects/corpus"; continue
        per_input, nraw = CR.replay_boundary(binp, corpus, E / f"prof_{tag}" / b, CELL.sandbox_dir(E))
        st = CR.merge_export(E / f"prof_{tag}" / b, objs, E / f"export_{tag}_{b}.json", empty_pd)
        if not st.startswith("ok"):
            rows[b] = st; continue
        funcs, regs = CR.extract_c(E / f"export_{tag}_{b}.json", E / "base" / "csrc" / b, pair / "source")
        per_b[b] = {k for k, v in funcs.items() if v}
        for k, v in funcs.items(): funcs_u[k] = funcs_u.get(k, False) or v
        for k, v in regs.items(): regs_u[k] = regs_u.get(k, False) or v
        tmp = O / f"c_exports_{tag}" / f"{b}.json.gz.tmp"
        with gzip.open(tmp, "wt") as gz:
            gz.write((E / f"export_{tag}_{b}.json").read_text())
        os.replace(tmp, O / f"c_exports_{tag}" / f"{b}.json.gz")
        (E / f"export_{tag}_{b}.json").unlink(); shutil.rmtree(E / f"prof_{tag}" / b, ignore_errors=True)
        per_input_all[b] = per_input
        oc = {}
        for r in per_input.values(): oc[r["outcome"]] = oc.get(r["outcome"], 0) + 1
        rows[b] = {"status": st, "corpus": len(per_input), "outcomes": oc, "c_functions_reached": sum(funcs.values()),
                   "c_functions_total": len(funcs), "c_regions_reached": sum(regs.values()), "c_regions_total": len(regs)}
        log(f"  C {tag} {b:30s} corpus {len(per_input):5d} {oc} | fn {sum(funcs.values())}/{len(funcs)} reg {sum(regs.values())}/{len(regs)}")
    return funcs_u, regs_u, per_b, per_input_all, rows


def side(fn_u, reg_u):
    return {"functions_total": len(fn_u), "functions_reached": sum(fn_u.values()),
            "regions_total": len(reg_u), "regions_reached": sum(reg_u.values()),
            "function_reach": sum(fn_u.values()) / len(fn_u) if fn_u else None,
            "region_reach": sum(reg_u.values()) / len(reg_u) if reg_u else None}


def rust_side(an: Path):
    p = an / "result.json"
    if not p.exists():
        return None
    r = json.load(open(p))
    return {"functions_total": r["function"]["total_in_scope"], "functions_reached": r["function"]["covered_ours"],
            "regions_total": r["region"]["total_in_scope"], "regions_reached": r["region"]["covered_ours"],
            "region_reach": r["region"]["ours_coverage"]}


# ----------------------------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lib", required=True); ap.add_argument("--tool", required=True)
    ap.add_argument("--work", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--seconds", type=int, default=None, help="override the archived budget (default: the archived value)")
    ap.add_argument("--only"); ap.add_argument("--keep-work", action="store_true")
    ap.add_argument("--single-c-companion", action="store_true",
                    help="fuzz only C; replay the resulting corpus on C and Rust")
    ap.add_argument("--max-fuzzers", type=int, default=28,
                    help="hard upper bound on concurrently launched boundary fuzzers")
    ap.add_argument("--arm-order", default="rust,c", choices=("rust,c", "c,rust"),
                    help="both arms always run; this controls their sequential order")
    a = ap.parse_args()
    lib, tool = a.lib, a.tool
    cell = ROOT / "results/rq3_coverage" / lib / tool
    pair = ROOT / "benchmark/pairs/rq4" / f"{lib}_{tool}"
    E, O = Path(a.work), Path(a.out)
    O.mkdir(parents=True, exist_ok=True)
    lock_fh = open(O.parent / ".controlled-guidance.lock", "a+")
    try:
        fcntl.flock(lock_fh, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        log(f"another controlled run owns {O.parent}")
        return 4
    if (O / "DONE.json").exists():
        done = json.load(open(O / "DONE.json"))
        if done.get("valid"):
            log(f"{lib} x {tool}: valid DONE.json present in {O}; nothing to do")
            return 0
        log(f"refusing to reuse invalid DONE.json in {O}")
        return 3

    shutil.rmtree(E, ignore_errors=True); E.mkdir(parents=True)
    t0 = time.time()
    CELL.GEN_HASH = CELL.generator_hash()
    driver_files = [Path(__file__), ROOT / "scripts/rq4/c_reach.py", ROOT / "scripts/rq4/cell.py",
                    ROOT / "tools/stu_selector/gen_diff_harness.py"]
    provenance = {"git_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                  "generator": CELL.GEN_HASH,
                  "protocol": "single-c-companion" if a.single_c_companion else "fresh-two-arm",
                  "files": {str(p.relative_to(ROOT)): file_sha256(p) for p in driver_files}}
    params = archived_params(cell)
    if a.seconds is not None:
        params["max_total_time_s"] = a.seconds; params["budget_override"] = True
    log(f"controlled guidance {lib} x {tool}: {provenance}; params {params}")

    funnel = json.load(open(cell / "funnel.json"))
    expected = [r["boundary"] for r in funnel if r.get("built")]
    if a.only:
        requested = set(a.only.split(","))
        expected = [b for b in expected if b in requested]
    if not expected:
        wjson(O / "FAILED.json", {"reason": "no archived-built boundaries selected", "provenance": provenance})
        return 3

    # The archived automatic corpus supplies only the initial seed files. It is
    # never used as either experimental arm.
    with tarfile.open(cell / "corpus.tar.gz") as tf:
        tf.extractall(E / "archived")
    archived_root = E / "archived" / "corpus"
    seeds = {b: {n: (archived_root / b / n).read_bytes() for n in initial_corpus(archived_root, b)}
             for b in expected}
    seed_manifest = {b: {n: hashlib.sha256(v).hexdigest() for n, v in sorted(xs.items())}
                     for b, xs in seeds.items()}

    csrc_name = CR.c_source_of(cell, expected)
    defs_path = pair / "translated" / f"{lib}_{tool}.rs.defs.json"
    defs = json.loads(defs_path.read_text()) if defs_path.exists() else {}
    private = set(defs.get("private", []))
    rs = next(iter(sorted((pair / "translated").glob("*.rs"))), None)
    rs_text = rs.read_text(errors="replace") if rs else ""
    plugins = [str(ROOT / "plugins/cjson/plugin.toml")] if lib == "cjson" else []
    ba = argparse.Namespace(c_source=csrc_name, plugins=plugins, defs=str(defs_path) if defs_path.exists() else None,
                            shim=str(ROOT / "benchmark/pairs/rq4/darwin_shims.c"), pair=str(pair))
    empty_txt = E / "empty.proftext"; empty_txt.write_text("")
    empty_pd = E / "empty.profdata"
    subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-o", str(empty_pd), str(empty_txt)],
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)

    # Build exactly the archived-built boundary set once. Any loss invalidates
    # the whole cell instead of silently improving the denominator.
    bins, build_rows, drift = {}, [], {}
    for b in expected:
        is_priv = b in private or (bool(rs_text) and not defs and re.search(
            rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b', rs_text) is None)
        binp, err = build_keep(ba, pair, b, is_priv, E / "base/harnesses" / b, E / "target", E / "base")
        build_rows.append({"boundary": b, "built": binp is not None, "error": err,
                           "c_static": is_priv, "initial_corpus": sorted(seeds.get(b, {}))})
        if binp:
            bins[b] = binp
            drift[b] = harness_drift(cell, E / "base/harnesses" / b, b)
        log(f"  build {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}")
    build_record = {"provenance": provenance, "expected": expected, "rows": build_rows,
                    "harness_drift": drift, "seed_manifest": seed_manifest}
    wjson(O / "build.json", build_record)
    if set(bins) != set(expected):
        wjson(O / "FAILED.json", {"reason": "rebuilt boundary set differs from archived-built set",
                                   "missing": sorted(set(expected) - set(bins)), "provenance": provenance})
        log("build-set mismatch; cell invalid")
        return 3
    marker(O, "BUILD_DONE", {"built": len(bins), "expected": len(expected)})
    measured = sorted(bins)
    if len(measured) > a.max_fuzzers:
        wjson(O / "FAILED.json", {"reason": "boundary fuzzer count exceeds the frozen cap",
                                   "built": len(measured), "max_fuzzers": a.max_fuzzers,
                                   "provenance": provenance})
        log(f"refusing to launch {len(measured)} fuzzers; cap is {a.max_fuzzers}")
        return 3

    arm_defs = {"rust": "rust-only", "c": "c-only"}
    order = ["c"] if a.single_c_companion else a.arm_order.split(",")
    campaigns, corpus_roots = {}, {}
    for tag in order:
        mode = arm_defs[tag]
        root = E / f"arm_{tag}/corpus"
        corpus_roots[tag] = root
        tar_path, json_path = O / f"{tag}_corpus.tar.gz", O / f"{tag}_campaign.json"
        done_path = O / f"{tag.upper()}_CAMPAIGN_DONE"
        if done_path.exists() and tar_path.exists() and json_path.exists():
            camp = json.load(open(json_path))
            if camp.get("provenance") != provenance or camp.get("boundaries") != measured:
                wjson(O / "FAILED.json", {"reason": f"{tag} checkpoint provenance mismatch"})
                return 3
            with tarfile.open(tar_path) as tf:
                tf.extractall(E / f"arm_{tag}")
            log(f"restored {tag} arm from {tar_path}")
        else:
            log(f"##### {mode}, {len(bins)} harnesses, {params['max_total_time_s']}s — {time.strftime('%H:%M:%S')}")
            sizes = campaign_guided(bins, root, E / f"arm_{tag}/candidates", params,
                                    E / f"arm_{tag}/snapshots", seeds, O, mode, tag)
            fuzz = {b: CELL.fuzz_status(E / f"arm_{tag}/candidates/{b}.fuzz.log") for b in measured}
            snaps = {}
            for d in sorted((E / f"arm_{tag}/snapshots").glob("*@*s")):
                entry, cp = d.name.rsplit("@", 1)
                snaps.setdefault(cp[:-1], {})[entry] = len(list(d.iterdir()))
            camp = {"mode": mode, "fork": 1, "boundaries": measured, "provenance": provenance,
                    **{k: params[k] for k in ("max_total_time_s", "seed", "timeout_s", "rss_limit_mb", "max_len")},
                    "params_source": params.get("source"), "budget_override": params.get("budget_override", False),
                    "initial_corpus": {b: sorted(seeds.get(b, {})) for b in measured},
                    "seed_manifest": seed_manifest, "corpus_sizes": sizes, "fuzz_status": fuzz, "snapshots": snaps}
            wtar(tar_path, root, "corpus")
            wjson(json_path, camp)
            marker(O, done_path.name, {"inputs": sum(sizes.values()), "mode": mode})
        campaigns[tag] = camp
        if tag not in corpus_roots:
            corpus_roots[tag] = E / f"arm_{tag}/corpus"

    if a.single_c_companion:
        # The archived campaign is the Rust-guided baseline.  The only new
        # fuzzing above was c-only; below, the same saved CC bytes are replayed
        # on both implementations for side-specific reach.
        cdata = c_measure(E, pair, measured, corpus_roots["c"], "c", O, empty_pd)
        wjson(O / "per_input_c.json", cdata[3])
        wjson(O / "c_rows.json", {"c": cdata[4]})
        marker(O, "C_MEASURE_DONE")
        rstatus_all = rust_measure_two(E, cell, pair, lib, tool, measured,
                                       {"c": corpus_roots["c"]}, E / "rust", O)
        rstatus = rstatus_all["c"]
        marker(O, "RUST_MEASURE_DONE")

        r_uni, r_cov = CR.rust_sets(E / "rust/rust_c/analysis")
        cmap = CR.load_map(lib, tool)
        out_by_boundary = {}
        for boundary, rows in cdata[3].items():
            for row in rows.values():
                d = out_by_boundary.setdefault(boundary, {})
                d[row["outcome"]] = d.get(row["outcome"], 0) + 1
        matched = CR.four_sets(cdata[0], r_uni, r_cov, cmap, cdata[2], rstatus,
                               None, out_by_boundary)
        if matched is not None:
            wjson(O / "matched_sets_c.json", matched)

        archived = json.load(open(cell / "analysis/result.json"))
        archived_rust = {
            "functions_total": archived["function"]["total_in_scope"],
            "functions_reached": archived["function"]["covered_ours"],
            "regions_total": archived["region"]["total_in_scope"],
            "regions_reached": archived["region"]["covered_ours"],
            "region_reach": archived["region"]["ours_coverage"],
        }
        same_corpus_path = ROOT / "results/rq4_c_reach" / f"{lib}_{tool}" / "result.json"
        archived_c = None
        if same_corpus_path.exists():
            archived_c = json.load(open(same_corpus_path)).get("c")

        c_inputs = {}
        for rows in cdata[3].values():
            for row in rows.values():
                c_inputs[row["outcome"]] = c_inputs.get(row["outcome"], 0) + 1
        cc_rust = rust_side(E / "rust/rust_c/analysis")
        checks = {
            "build_set_exact": set(measured) == set(expected),
            "only_c_was_fuzzed": list(campaigns) == ["c"],
            "fuzzer_cap_respected": len(measured) <= a.max_fuzzers,
            "c_measurement_nonempty": len(cdata[0]) > 0,
            "rust_replay_nonempty": cc_rust is not None,
            "c_input_count_exact": sum(c_inputs.values()) == sum(campaigns["c"]["corpus_sizes"].values()),
        }
        valid = all(checks.values())
        result = {
            "schema": 3,
            "cell": f"{lib}_{tool}",
            "protocol": "single-c-companion",
            "provenance": provenance,
            "boundaries": measured,
            "concurrent_fuzzers": len(measured),
            "max_fuzzers": a.max_fuzzers,
            "campaign": campaigns["c"],
            "matrix": {
                "archived_rust_guided": {"C": archived_c, "Rust": archived_rust},
                "new_c_guided": {"C": side(cdata[0], cdata[1]), "Rust": cc_rust,
                                  "C_inputs": c_inputs},
            },
            "matched_c_guided": ({"accepted_pairs": matched["accepted_pairs"],
                                   "counts": matched["counts"],
                                   "ambiguous": len(matched["ambiguous"])} if matched else None),
            "checks": checks,
            "seconds": round(time.time() - t0),
            "interpretation": "CC is newly C-guided; CR is the archived Rust-guided baseline. Rust(CC) is replay, not a Rust fuzzing arm.",
        }
        wjson(O / "result.json", result)
        wtext(O / "RUN.md", single_c_run_md(result))
        wjson(O / "DONE.json", {"at": time.strftime("%Y-%m-%dT%H:%M:%S"),
                                 "valid": valid, "checks": checks,
                                 "provenance": provenance})
        if not a.keep_work:
            shutil.rmtree(E, ignore_errors=True)
        log(f"C_COMPANION_DONE {lib} x {tool}: valid={valid}; "
            f"C={result['matrix']['new_c_guided']['C']['functions_reached']} "
            f"Rust={cc_rust and cc_rust['functions_reached']}")
        return 0 if valid else 3

    # Measure both fresh corpora on both sides of the same build.
    cdata = {}
    for tag in ("rust", "c"):
        cdata[tag] = c_measure(E, pair, measured, corpus_roots[tag], tag, O, empty_pd)
        wjson(O / f"per_input_{tag}.json", cdata[tag][3])
    wjson(O / "c_rows.json", {tag: cdata[tag][4] for tag in ("rust", "c")})
    marker(O, "C_MEASURE_DONE")
    rstatus = rust_measure_two(E, cell, pair, lib, tool, measured,
                               {tag: corpus_roots[tag] for tag in ("rust", "c")}, E / "rust", O)
    marker(O, "RUST_MEASURE_DONE")

    rust_sets = {tag: CR.rust_sets(E / f"rust/rust_{tag}/analysis") for tag in ("rust", "c")}
    union_fn = {k: cdata["rust"][0].get(k, False) or cdata["c"][0].get(k, False)
                for k in set(cdata["rust"][0]) | set(cdata["c"][0])}
    union_reg = {k: cdata["rust"][1].get(k, False) or cdata["c"][1].get(k, False)
                 for k in set(cdata["rust"][1]) | set(cdata["c"][1])}
    r_union_uni = rust_sets["rust"][0] | rust_sets["c"][0]
    r_union_cov = rust_sets["rust"][1] | rust_sets["c"][1]
    cmap = CR.load_map(lib, tool)

    def outcomes(per_input):
        out = {}
        for b, rows in per_input.items():
            for row in rows.values():
                d = out.setdefault(b, {}); d[row["outcome"]] = d.get(row["outcome"], 0) + 1
        return out

    sets = {}
    for tag in ("rust", "c"):
        sets[tag] = CR.four_sets(cdata[tag][0], *rust_sets[tag], cmap, cdata[tag][2], rstatus[tag],
                                 None, outcomes(cdata[tag][3]))
    sets["union"] = CR.four_sets(
        union_fn, r_union_uni, r_union_cov, cmap,
        {b: cdata["rust"][2].get(b, set()) | cdata["c"][2].get(b, set()) for b in measured},
        {b: rstatus["c"].get(b) if str(rstatus["c"].get(b, "")).startswith(("batch", "per-input"))
         else rstatus["rust"].get(b) for b in measured})
    for tag, value in sets.items():
        wjson(O / f"matched_sets_{tag}.json", value)

    def totals(per_input):
        out = {}
        for rows in per_input.values():
            for row in rows.values(): out[row["outcome"]] = out.get(row["outcome"], 0) + 1
        return out

    matrix = {}
    for tag in ("rust", "c"):
        matrix[tag] = {"C": side(cdata[tag][0], cdata[tag][1]),
                       "Rust": rust_side(E / f"rust/rust_{tag}/analysis"),
                       "C_inputs": totals(cdata[tag][3]), "rust_coverage_status": rstatus[tag]}
    matrix["union"] = {"C": side(union_fn, union_reg), "Rust_functions_reached": len(r_union_cov),
                       "Rust_functions_in_scope": len(r_union_uni),
                       "note": "function-level union; side-specific regions are not structurally unioned"}
    archived = json.load(open(cell / "analysis/result.json"))
    matrix["archived_reference"] = {
        "functions_total": archived["function"]["total_in_scope"],
        "functions_reached": archived["function"]["covered_ours"],
        "regions_total": archived["region"]["total_in_scope"],
        "regions_reached": archived["region"]["covered_ours"],
        "region_reach": archived["region"]["ours_coverage"],
        "note": "old-harness Rust-guided result; reference only, never an experimental arm"}

    checks = {"build_set_exact": set(measured) == set(expected), "arm_boundaries_equal": True,
              "campaign_params_equal": all({k: campaigns["rust"][k] for k in ("seed", "timeout_s", "rss_limit_mb", "max_len", "max_total_time_s")} ==
                                           {k: campaigns["c"][k] for k in ("seed", "timeout_s", "rss_limit_mb", "max_len", "max_total_time_s")} for _ in [0]),
              "initial_seeds_equal": campaigns["rust"]["seed_manifest"] == campaigns["c"]["seed_manifest"],
              "c_measurements_nonempty": all(matrix[tag]["C"]["functions_total"] > 0 for tag in ("rust", "c")),
              "rust_measurements_nonempty": all(matrix[tag]["Rust"] is not None for tag in ("rust", "c")),
              "c_input_counts_exact": all(sum(matrix[tag]["C_inputs"].values()) == sum(campaigns[tag]["corpus_sizes"].values())
                                          for tag in ("rust", "c"))}
    valid = all(checks.values())
    control = None
    if tool == "c2rust" and cmap is not None:
        control = {tag: sets[tag]["counts"] for tag in ("rust", "c", "union")}

    result = {"schema": 2, "cell": f"{lib}_{tool}", "provenance": provenance, "boundaries": measured,
              "arm_order": order, "seed_source": "archived automatic campaign's non-SHA1 initial files",
              "tulip_seed_note": "automatic seed_fixed baseline; not the later grid-refined campaign" if lib == "tulip" else None,
              "harness_drift_reference_only": {"identical_modulo_oracle": sum(bool(x.get("identical_modulo_oracle")) for x in drift.values()),
                                               "differing": sorted(b for b, x in drift.items() if not x.get("identical_modulo_oracle"))},
              "campaigns": campaigns, "matrix": matrix, "checks": checks, "c2rust_control": control,
              "matched": {k: ({"accepted_pairs": v["accepted_pairs"], "counts": v["counts"],
                                "ambiguous": len(v["ambiguous"]), "c_unmatched": len(v["c_unmatched"]),
                                "rust_unmatched": len(v["rust_unmatched"])} if v else None) for k, v in sets.items()},
              "seconds": round(time.time() - t0)}
    wjson(O / "result.json", result)
    wtext(O / "RUN.md", run_md(result, sets))
    wjson(O / "DONE.json", {"at": time.strftime("%Y-%m-%dT%H:%M:%S"), "valid": valid,
                             "checks": checks, "provenance": provenance})
    if not a.keep_work:
        shutil.rmtree(E, ignore_errors=True)
    log(f"CONTROLLED_GUIDANCE_DONE {lib} x {tool}: valid={valid}; "
        f"rust={matrix['rust']['Rust'] and matrix['rust']['Rust']['functions_reached']} "
        f"c={matrix['c']['Rust'] and matrix['c']['Rust']['functions_reached']}")
    return 0 if valid else 3


def run_md(res, sets) -> str:
    m = res["matrix"]

    def fr(side):
        return f"{side['functions_reached']} / {side['functions_total']}" if side else "–"

    def rr(side):
        if not side or side.get("region_reach") is None:
            return "–"
        return f"{side['regions_reached']} / {side['regions_total']} ({side['region_reach']:.3f})"

    lines = [
        f"# Controlled C/Rust-guided reach — {res['cell']}", "",
        "Both corpora were generated afresh on the same rebuilt harness set. The archived campaign is reference-only.",
        f"Arm order: `{' → '.join(res['arm_order'])}`. Initial seeds: {res['seed_source']}.",
        f"Checks: `{res['checks']}`.", "",
        "## Corpus × execution side", "",
        "| guidance | C functions | C regions | Rust functions | Rust regions |",
        "|---|---:|---:|---:|---:|",
        f"| Rust-guided | {fr(m['rust']['C'])} | {rr(m['rust']['C'])} | {fr(m['rust']['Rust'])} | {rr(m['rust']['Rust'])} |",
        f"| C-guided | {fr(m['c']['C'])} | {rr(m['c']['C'])} | {fr(m['c']['Rust'])} | {rr(m['c']['Rust'])} |",
        f"| union | {fr(m['union']['C'])} | {rr(m['union']['C'])} | {m['union']['Rust_functions_reached']} / {m['union']['Rust_functions_in_scope']} | – |",
        "", "The C and Rust region denominators are side-specific and are never subtracted.",
        "The C-guided arm is a reach diagnostic; its candidates are not promoted without the normal confirmation pipeline.",
        "", "## Matched-function sets", "",
        "| guidance | pairs | both | C only | Rust only | neither | ambiguous |",
        "|---|---:|---:|---:|---:|---:|---:|"
    ]
    for tag, label in (("rust", "Rust-guided"), ("c", "C-guided"), ("union", "union")):
        value = sets[tag]
        if value is None:
            lines.append(f"| {label} | no map | – | – | – | – | – |")
            continue
        counts = value["counts"]
        lines.append(f"| {label} | {value['accepted_pairs']} | {counts['both']} | {counts['c_only']} | "
                     f"{counts['rust_only']} | {counts['neither']} | {len(value['ambiguous'])} |")
    lines += ["", "## Campaigns", ""]
    for tag, label in (("rust", "Rust-guided"), ("c", "C-guided")):
        camp = res["campaigns"][tag]
        lines += [f"### {label}", "",
                  f"Mode `{camp['mode']}`, {camp['max_total_time_s']} s wall per cell, max_len {camp['max_len']}, "
                  f"seed {camp['seed']}, timeout {camp['timeout_s']} s, RSS limit {camp['rss_limit_mb']} MB.", "",
                  "| boundary | initial files | final corpus | jobs | cov | crash | timeout |",
                  "|---|---|---:|---:|---:|---:|---:|"]
        for boundary in res["boundaries"]:
            stat = camp["fuzz_status"].get(boundary, {})
            lines.append(f"| {boundary} | {', '.join(camp['initial_corpus'].get(boundary, []))} | "
                         f"{camp['corpus_sizes'].get(boundary, '')} | {stat.get('jobs', '')} | "
                         f"{stat.get('cov', '')} | {stat.get('crash', '')} | {stat.get('timeout', '')} |")
        lines.append("")
    lines += ["## Interpretation boundary", "",
              "This experiment compares guidance under a fixed application-level wall-clock campaign. "
              "It does not establish a theoretical reach maximum or that C-side executions are defined. "
              "Any exclusive reach caused by termination requires the existing UB-aware confirmation channels.", ""]
    return "\n".join(lines)


def single_c_run_md(res) -> str:
    def fmt(side, unit):
        if not side:
            return "–"
        reached = side[f"{unit}_reached"]
        total = side[f"{unit}_total"]
        ratio = side.get("function_reach" if unit == "functions" else "region_reach")
        return f"{reached} / {total}" + (f" ({ratio:.3f})" if ratio is not None else "")

    old = res["matrix"]["archived_rust_guided"]
    new = res["matrix"]["new_c_guided"]
    return "\n".join([
        f"# C-guided companion — {res['cell']}", "",
        "Only the C side was fuzzed in this run. The resulting corpus was then replayed on both C and Rust.",
        "The archived Rust-guided campaign is a labelled baseline; it was not rerun.", "",
        f"Concurrent boundary fuzzers: {res['concurrent_fuzzers']} (hard cap {res['max_fuzzers']}).", "",
        "| corpus | C functions | C regions | Rust functions | Rust regions |", "|---|---:|---:|---:|---:|",
        f"| archived Rust-guided | {fmt(old['C'], 'functions')} | {fmt(old['C'], 'regions')} | {fmt(old['Rust'], 'functions')} | {fmt(old['Rust'], 'regions')} |",
        f"| new C-guided | {fmt(new['C'], 'functions')} | {fmt(new['C'], 'regions')} | {fmt(new['Rust'], 'functions')} | {fmt(new['Rust'], 'regions')} |",
        "", f"Checks: `{res['checks']}`.", "",
        "This is a reach diagnostic. C-side completion is not proof of defined behavior, and new candidates are not defects without confirmation.", "",
    ])


if __name__ == "__main__":
    raise SystemExit(main())
