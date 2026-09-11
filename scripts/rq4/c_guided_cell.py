#!/usr/bin/env python3
"""C-guided companion campaign for one archived RQ4 cell (docs/c_reach_plan.md, section 8).

The archived campaign was Rust-guided (C2R_MODE=rust-only): libFuzzer saw only the translation's edges, so
its corpus CR grew only where the Rust side let it. This runs the SAME harnesses, seed, budget and libFuzzer
parameters with C2R_MODE=c-only -- the C oracle is already sancov-instrumented in every build.rs, so the
fuzzer is now guided by C's edges alone -- producing a C-guided corpus CC, then measures BOTH sides on CC and
reports the 2x2 (corpus x side) plus the matched-function four sets on CR, CC and CR ∪ CC (function level;
C regions unioned by identity; Rust regions on CC only).

Reach only: candidates found by the C-guided campaign are NOT adjudicated here (a divergence needs the full
confirmation channel before it means anything). Side-specific percentages are never subtracted.

usage: c_guided_cell.py --lib L --tool T --work E --out O --cr <sweep out dir of the same cell>
                        [--seconds 3600] [--only b1,b2] [--keep-work]
"""
import argparse, gzip, hashlib, json, os, re, shutil, subprocess, sys, tarfile, time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cell as CELL
import c_reach as CR
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import c2r_funnel as F

ROOT = Path(__file__).resolve().parents[2]


def log(*a):
    print(*a, flush=True)


def build_keep(a, pair: Path, entry: str, private: bool, hd: Path, target: Path, base: Path):
    """Generate (--c-coverage) + fixups + `cargo fuzz build` with the campaign's default sanitizer; keep the
    harness TREE (the Rust coverage build needs it), the unstripped binary, the C objects, edited C copies."""
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


def campaign_c_only(binaries: dict, corpus_root: Path, art_root: Path, seconds: int, snap_root: Path) -> dict:
    """cell.campaign() with C2R_MODE=c-only: same seed, fork mode, parameters, snapshots. The in-loop UB
    gate is inert in c-only, so C is explored wherever its own edges lead (C-side crashes are ignored and
    restarted exactly as Rust-side ones were)."""
    procs, logs = {}, {}
    sb = CELL.sandbox_dir(corpus_root.parent)
    for entry, b in binaries.items():
        c = corpus_root / entry
        c.mkdir(parents=True, exist_ok=True)
        seed = c / "seed_fixed"
        if not seed.exists():
            seed.write_bytes(bytes(range(64)))
        art = art_root / entry
        art.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, C2R_MODE="c-only", ASAN_OPTIONS="detect_leaks=0")
        env.pop("C2R_OUTCOME_FILE", None); env.pop("LLVM_PROFILE_FILE", None)
        lg = open(art_root / f"{entry}.fuzz.log", "wb")
        logs[entry] = lg
        procs[entry] = subprocess.Popen(
            [str(b), str(c), "-fork=1", "-ignore_crashes=1", "-ignore_timeouts=1", "-ignore_ooms=1",
             f"-max_total_time={seconds}", "-timeout=25", f"-max_len={CELL.MAX_LEN}", "-rss_limit_mb=2048",
             "-seed=42", f"-artifact_prefix={art}/"],
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
    deadline = t0 + seconds + 240
    for entry, p in procs.items():
        try:
            p.wait(timeout=max(10, deadline - time.time()))
        except subprocess.TimeoutExpired:
            p.kill()
    for f in logs.values():
        f.close()
    return {e: len(list((corpus_root / e).iterdir())) for e in binaries}


def universe_args(cell: Path, E: Path) -> list[str]:
    """The archived cell's universe, as the census driver chose it: the tests build when one was measured
    (a non-empty tests_coverage.json), else raw/denominator.json, else the tarballed bin-route denominator
    relocated with --path-map."""
    tc = cell / "raw" / "tests_coverage.json"
    if tc.exists() and tc.stat().st_size > 0:
        return ["--tests", str(tc)]
    dn = cell / "raw" / "denominator.json"
    if dn.exists():
        return ["--denominator", str(dn)]
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


def rust_measure(E: Path, cell: Path, pair: Path, lib: str, tool: str, boundaries, corpus_root: Path, out_dir: Path) -> dict:
    """Rust reach of a corpus: cell.collect() per harness (the archived procedure: cargo fuzz coverage batch,
    per-input fallback), then c2r_coverage.py against the archived universe. Returns funnel-style status."""
    ours = out_dir / "ours"; ours.mkdir(parents=True, exist_ok=True)
    status = {}
    for b in boundaries:
        hd = E / "base" / "harnesses" / b
        corpus = corpus_root / b
        if not corpus.exists() or not any(corpus.iterdir()):
            status[b] = "empty-corpus"; continue
        status[b] = CELL.collect(b, hd, corpus, ours / f"{b}.json", E / "target")
        log(f"  rust coverage {b:30s} {status[b]}")
        for junk in ("target", "fuzz/target", "fuzz/coverage", "percov"):
            shutil.rmtree(hd / junk, ignore_errors=True)
    for f in list(ours.glob("*.json")):
        try:
            json.load(open(f))
        except Exception:
            f.unlink()
    cmd = [sys.executable, str(ROOT / "scripts/c2r_coverage.py"), "--linemap", str(pair / "translated" / f"{lib}_{tool}.rs.linemap.json"),
           "--ours", str(ours), *universe_args(cell, E), "--out", str(out_dir / "analysis"), "--corpus-root", str(corpus_root)]
    r = subprocess.run(cmd, capture_output=True, text=True, errors="replace")
    (out_dir / "analysis.log").write_text(r.stdout + r.stderr)
    return status


def c_measure(E: Path, pair: Path, boundaries, corpus_root: Path, tag: str, O: Path, empty_pd: Path):
    """C reach of a corpus with the C-coverage binaries: c_reach's replay + export + extract per boundary."""
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
        with gzip.open(O / f"c_exports_{tag}" / f"{b}.json.gz", "wt") as gz:
            gz.write((E / f"export_{tag}_{b}.json").read_text())
        (E / f"export_{tag}_{b}.json").unlink(); shutil.rmtree(E / f"prof_{tag}" / b, ignore_errors=True)
        per_input_all[b] = per_input
        oc = {}
        for r in per_input.values(): oc[r["outcome"]] = oc.get(r["outcome"], 0) + 1
        rows[b] = {"status": st, "corpus": len(per_input), "outcomes": oc, "c_functions_reached": sum(funcs.values()), "c_regions_reached": sum(regs.values())}
        log(f"  C {tag} {b:30s} corpus {len(per_input):5d} {oc} | fn {sum(funcs.values())}/{len(funcs)} reg {sum(regs.values())}/{len(regs)}")
    return funcs_u, regs_u, per_b, per_input_all, rows


def c_sets_from_exports(exp_dir: Path, csrc_dir: Path, source_root: Path, tmp: Path):
    """C function/region reach recomputed from a sweep's archived per-boundary exports (the CR arm)."""
    funcs_u, regs_u, per_b = {}, {}, {}
    for gz in sorted(exp_dir.glob("*.json.gz")):
        b = gz.name[:-8]
        ex = tmp / f"cr_{b}.json"; ex.write_text(gzip.open(gz, "rt").read())
        funcs, regs = CR.extract_c(ex, csrc_dir / b, source_root)
        ex.unlink()
        per_b[b] = {k for k, v in funcs.items() if v}
        for k, v in funcs.items(): funcs_u[k] = funcs_u.get(k, False) or v
        for k, v in regs.items(): regs_u[k] = regs_u.get(k, False) or v
    return funcs_u, regs_u, per_b


def side(fn_u, reg_u):
    return {"functions_total": len(fn_u), "functions_reached": sum(fn_u.values()),
            "regions_total": len(reg_u), "regions_reached": sum(reg_u.values()),
            "function_reach": sum(fn_u.values()) / len(fn_u) if fn_u else None,
            "region_reach": sum(reg_u.values()) / len(reg_u) if reg_u else None}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lib", required=True); ap.add_argument("--tool", required=True)
    ap.add_argument("--work", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--cr", required=True, help="the same-corpus sweep's output dir for this cell (CR arm)")
    ap.add_argument("--seconds", type=int, default=3600); ap.add_argument("--only"); ap.add_argument("--keep-work", action="store_true")
    a = ap.parse_args()
    lib, tool = a.lib, a.tool
    cell = ROOT / "results/rq3_coverage" / lib / tool
    pair = ROOT / "benchmark/pairs/rq4" / f"{lib}_{tool}"
    E, O, CRD = Path(a.work), Path(a.out), Path(a.cr)
    shutil.rmtree(E, ignore_errors=True); E.mkdir(parents=True); O.mkdir(parents=True, exist_ok=True)
    t0 = time.time()
    CELL.GEN_HASH = CELL.generator_hash()
    log(f"c_guided {lib} x {tool}: generator {CELL.GEN_HASH}, budget {a.seconds}s")
    funnel = json.load(open(cell / "funnel.json"))
    built = [r["boundary"] for r in funnel if r.get("built")]
    if a.only:
        built = [b for b in built if b in set(a.only.split(","))]
    csrc_name = CR.c_source_of(cell, built)
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
    subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-o", str(empty_pd), str(empty_txt)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    # 1. build (harness trees kept)
    bins, build_rows = {}, []
    for b in built:
        is_priv = b in private or (bool(rs_text) and not defs and re.search(
            rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b', rs_text) is None)
        binp, err = build_keep(ba, pair, b, is_priv, E / "base" / "harnesses" / b, E / "target", E / "base")
        build_rows.append({"boundary": b, "built": binp is not None, "error": err, "c_static": is_priv})
        log(f"  build {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}")
        if binp:
            bins[b] = binp
    (E / "base" / "funnel.json").write_text(json.dumps(build_rows, indent=1) + "\n")
    measured = sorted(bins)

    # 2. the C-guided campaign
    log(f"##### c-only campaign, {len(bins)} harnesses, {a.seconds}s — {time.strftime('%H:%M:%S')}")
    sizes = campaign_c_only(bins, E / "campaign" / "corpus", E / "campaign" / "candidates", a.seconds, E / "campaign" / "snapshots")
    fuzz = {b: CELL.fuzz_status(E / "campaign" / "candidates" / f"{b}.fuzz.log") for b in bins}
    snaps = {}
    for d in sorted((E / "campaign" / "snapshots").glob("*@*s")):
        e, cp = d.name.rsplit("@", 1); snaps.setdefault(cp[:-1], {})[e] = len(list(d.iterdir()))
    log(f"campaign done: corpus sizes {sum(sizes.values())} inputs over {len(sizes)} boundaries")

    # 3. C side on CC; CR recomputed from the sweep's exports
    cc_fn, cc_reg, cc_per_b, cc_per_input, cc_rows = c_measure(E, pair, measured, E / "campaign" / "corpus", "cc", O, empty_pd)
    cr_fn, cr_reg, cr_per_b = c_sets_from_exports(CRD / "c_exports", CRD / "csrc", pair / "source", E)
    un_fn = {k: cr_fn.get(k, False) or cc_fn.get(k, False) for k in set(cr_fn) | set(cc_fn)}
    un_reg = {k: cr_reg.get(k, False) or cc_reg.get(k, False) for k in set(cr_reg) | set(cc_reg)}

    # 4. Rust side on CC (archived procedure), CR = the archived cell
    log(f"##### rust coverage on CC — {time.strftime('%H:%M:%S')}")
    rstatus = rust_measure(E, cell, pair, lib, tool, measured, E / "campaign" / "corpus", E / "rust_cc")
    r_cc_uni, r_cc_cov = CR.rust_sets(E / "rust_cc" / "analysis")
    r_cr_uni, r_cr_cov = CR.rust_sets(cell / "analysis")
    r_cc = json.load(open(E / "rust_cc" / "analysis" / "result.json")) if (E / "rust_cc" / "analysis" / "result.json").exists() else None
    r_cr = json.load(open(cell / "analysis" / "result.json"))
    r_un_cov = r_cr_cov | r_cc_cov; r_un_uni = r_cr_uni | r_cc_uni

    # 5. four sets per arm
    cmap = CR.load_map(lib, tool)
    rust_pb = CR.rust_per_boundary(cell)
    c_out = {}
    for b, pi in cc_per_input.items():
        for v in pi.values(): c_out.setdefault(b, {})[v["outcome"]] = c_out.setdefault(b, {}).get(v["outcome"], 0) + 1
    arch_status = {x["boundary"]: x.get("coverage") for x in funnel}
    sets = {"cr": CR.four_sets(cr_fn, r_cr_uni, r_cr_cov, cmap, cr_per_b, arch_status),
            "cc": CR.four_sets(cc_fn, r_cc_uni, r_cc_cov, cmap, cc_per_b, rstatus, None, c_out),
            "union": CR.four_sets(un_fn, r_un_uni, r_un_cov, cmap,
                                  {b: cr_per_b.get(b, set()) | cc_per_b.get(b, set()) for b in set(cr_per_b) | set(cc_per_b)},
                                  # a boundary's Rust side is "measured" on the union if either arm's replay produced a profile
                                  {b: (rstatus.get(b) if str(rstatus.get(b, "")).startswith(("batch", "per-input")) else arch_status.get(b))
                                   for b in set(cr_per_b) | set(cc_per_b)})}
    for k, v in sets.items():
        (O / f"matched_sets_{k}.json").write_text(json.dumps(v, indent=1) + "\n")
    tot_oc = {}
    for pi in cc_per_input.values():
        for r in pi.values(): tot_oc[r["outcome"]] = tot_oc.get(r["outcome"], 0) + 1
    res = {"cell": f"{lib}_{tool}", "generator": CELL.GEN_HASH, "budget_s": a.seconds, "boundaries": measured,
           "build": build_rows, "campaign": {"mode": "c-only", "fork": 1, "seed": 42, "timeout_s": 25, "max_len": CELL.MAX_LEN,
                                            "corpus_sizes": sizes, "fuzz_status": fuzz, "snapshots": snaps},
           "matrix": {
               "CR": {"C": side(cr_fn, cr_reg), "Rust": {"functions_total": r_cr["function"]["total_in_scope"], "functions_reached": r_cr["function"]["covered_ours"],
                                                        "regions_total": r_cr["region"]["total_in_scope"], "regions_reached": r_cr["region"]["covered_ours"],
                                                        "region_reach": r_cr["region"]["ours_coverage"]}},
               "CC": {"C": side(cc_fn, cc_reg), "Rust": ({"functions_total": r_cc["function"]["total_in_scope"], "functions_reached": r_cc["function"]["covered_ours"],
                                                         "regions_total": r_cc["region"]["total_in_scope"], "regions_reached": r_cc["region"]["covered_ours"],
                                                         "region_reach": r_cc["region"]["ours_coverage"]} if r_cc else None),
                      "C_inputs": tot_oc, "rust_coverage_status": rstatus},
               "CR_union_CC": {"C": side(un_fn, un_reg), "Rust_functions_reached": len(r_un_cov), "Rust_functions_in_scope": len(r_un_uni),
                               "note": "function level BY NAME (two identities sharing a name count once, so this can be below the "
                                       "identity-level count of a single arm); union of per-input reach is exact; Rust regions not unioned"}},
           "matched": {k: ({"accepted_pairs": v["accepted_pairs"], "counts": v["counts"], "ambiguous": len(v["ambiguous"]),
                            "c_unmatched": len(v["c_unmatched"]), "rust_unmatched": len(v["rust_unmatched"])} if v else None) for k, v in sets.items()},
           "seconds": round(time.time() - t0)}
    (O / "result.json").write_text(json.dumps(res, indent=1) + "\n")
    (O / "per_input_cc.json").write_text(json.dumps(cc_per_input) + "\n")
    (O / "c_rows_cc.json").write_text(json.dumps(cc_rows, indent=1) + "\n")
    write_run_md(O, res, sets)
    if not a.keep_work:
        shutil.rmtree(E, ignore_errors=True)
    m = res["matrix"]
    log(f"C_GUIDED_DONE {lib} x {tool}: C(CR) {m['CR']['C']['functions_reached']}/{m['CR']['C']['functions_total']} R(CR) {m['CR']['Rust']['functions_reached']}/{m['CR']['Rust']['functions_total']} | "
        f"C(CC) {m['CC']['C']['functions_reached']}/{m['CC']['C']['functions_total']} R(CC) {m['CC']['Rust'] and m['CC']['Rust']['functions_reached']} | "
        f"sets cr {sets['cr'] and sets['cr']['counts']} cc {sets['cc'] and sets['cc']['counts']} union {sets['union'] and sets['union']['counts']}")


def write_run_md(O: Path, res, sets):
    m = res["matrix"]
    def fr(s):
        return f"{s['functions_reached']} / {s['functions_total']}" if s else "–"
    def rr(s):
        return (f"{s['regions_reached']} / {s['regions_total']} ({s['region_reach']:.3f})" if s and s.get("region_reach") is not None else "–")
    L = [f"# C-guided companion campaign — {res['cell']}", "",
         f"Same harnesses (generator `{res['generator']}`, `--c-coverage`), same seed and libFuzzer parameters as the archived",
         f"campaign, `C2R_MODE=c-only`, budget {res['budget_s']} s, {len(res['boundaries'])} boundaries. CR = the archived Rust-guided corpus,",
         "CC = this C-guided corpus. Reach only; no candidate from CC is adjudicated here.", "",
         "## 2 x 2 (corpus x side); percentages are side-specific and never subtracted", "",
         "| corpus | C functions | C regions | Rust functions | Rust regions |", "|---|---|---|---|---|",
         f"| Rust-guided CR | {fr(m['CR']['C'])} | {rr(m['CR']['C'])} | {fr(m['CR']['Rust'])} | {rr(m['CR']['Rust'])} |",
         f"| C-guided CC | {fr(m['CC']['C'])} | {rr(m['CC']['C'])} | {fr(m['CC']['Rust'])} | {rr(m['CC']['Rust'])} |",
         f"| CR ∪ CC | {fr(m['CR_union_CC']['C'])} | {rr(m['CR_union_CC']['C'])} | {m['CR_union_CC']['Rust_functions_reached']} / {m['CR_union_CC']['Rust_functions_in_scope']} | – |", "",
         f"C-side inputs on CC: {m['CC']['C_inputs']}; corpus sizes {sum(res['campaign']['corpus_sizes'].values())} inputs.", "",
         "## Matched-function sets (accepted pairs ∩ C scope ∩ Rust scope)", "",
         "| corpus | pairs | both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous |", "|---|---|---|---|---|---|---|"]
    for k, name in (("cr", "CR"), ("cc", "CC"), ("union", "CR ∪ CC")):
        s = sets[k]
        if s:
            cn = s["counts"]
            L.append(f"| {name} | {s['accepted_pairs']} | {cn['both']} | {cn['c_only']} ({cn.get('c_only_rust_terminated', 0)}) | {cn['rust_only']} ({cn.get('rust_only_c_terminated', 0)}) | {cn['neither']} | {len(s['ambiguous'])} |")
        else:
            L.append(f"| {name} | no map | – | – | – | – | – |")
    for k, name in (("cc", "CC"), ("union", "CR ∪ CC")):
        s = sets[k]
        if s and s["sets"]["c_only"]:
            L += ["", f"`c_only` on {name}: " + ", ".join(c for c, _ in s["sets"]["c_only"][:60]) + (" …" if len(s["sets"]["c_only"]) > 60 else "")]
    L += ["", "## Campaign", "", "| boundary | corpus | jobs | cov | crash | timeout |", "|---|---|---|---|---|---|"]
    for b in res["boundaries"]:
        f = res["campaign"]["fuzz_status"].get(b, {})
        L.append(f"| {b} | {res['campaign']['corpus_sizes'].get(b, '')} | {f.get('jobs', '')} | {f.get('cov', '')} | {f.get('crash', '')} | {f.get('timeout', '')} |")
    L += ["", "## Procedure, deviations, and what is not established", "", "<!-- prose -->", ""]
    (O / "RUN.md").write_text("\n".join(L))


if __name__ == "__main__":
    main()
