#!/usr/bin/env python3
"""Same-corpus C reach for one archived RQ4 cell (docs/c_reach_plan.md).

Replays the cell's ARCHIVED corpus (results/rq3_coverage/<lib>/<tool>/corpus.tar.gz, one directory per
boundary) through freshly rebuilt harnesses in C2R_MODE=c-only, with the C oracle compiled
-fprofile-instr-generate -fcoverage-mapping (generator --c-coverage, 0.9.2), and reports

  * side-specific C reach: functions and code regions of the library's own C files, universe from
    EVERY instrumented C object file (never from the linked binary), union over boundaries by identity;
  * the four matched-function sets over  accepted correspondence pairs (matcher `deployment`)
    ∩ C scope ∩ Rust scope:  both / c_only / rust_only / neither; ambiguous and unmatched functions in
    their own columns, never in `neither`;
  * per-input C-side outcome: completed / crash / timeout, with the harness phase where it is reported.

The Rust side is NOT re-measured: it is the archived analysis/covered_by_*.txt of the same cell.
This is a paired reach diagnostic on a Rust-guided corpus, not a C coverage baseline; a C-side
`completed` shows reach, not C-definedness (that is the sanitizer confirmation's job).

usage: c_reach.py --lib L --tool T --work W --out O [--only b1,b2] [--keep-work]
"""
import argparse, difflib, gzip, json, os, re, shutil, subprocess, sys, tarfile, time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import cell as CELL
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import c2r_funnel as F
import c2r_coverage as CC

ROOT = Path(__file__).resolve().parents[2]
HELPER_FILES = {"ubshim.c", "shims.c", "c2r_plugin.c", "c2r_extract.c", "darwin_shims.c", "c2r_profref.c"}
CHUNK = 1000              # corpus files per replay process (arg-list bound)
PER_INPUT_TIMEOUT = 60    # s, wall, per fallback process (libFuzzer -timeout=25 inside)
PHASES = {0: "decode", 1: "target_entered", 2: "target_completed", 3: "rust", 4: "rust_done",
          5: "compared", 6: "producer", 7: "free"}


def log(*a):
    print(*a, flush=True)


# ----------------------------------------------------------------------------- build
def c_source_of(cell: Path, boundaries) -> str | None:
    """The C translation unit the archived harnesses compiled: build.file("c/<name>") in their build.rs."""
    for b in boundaries:
        brs = cell / "harnesses" / b / "build.rs"
        if brs.exists():
            m = re.search(r'build\.file\("c/([^"]+)"\)', brs.read_text())
            if m:
                return m.group(1)
    return None


def build_boundary(a, pair: Path, entry: str, private: bool, out_dir: Path, target: Path):
    """gen (--c-coverage) + fixups + cargo fuzz build --sanitizer none; keep the UNSTRIPPED binary and
    every C object file of the oracle. Mirrors cell.build_one; differs only in the flag, no strip, objects."""
    shutil.rmtree(out_dir, ignore_errors=True)
    cmd = [sys.executable, str(ROOT / "tools/stu_selector/gen_diff_harness.py"),
           "--pair", str(pair), "--entry", entry, "--rust-entry", F.rust_name(pair, entry),
           "--plan", "--ub-free", "--c-coverage", "--out", str(out_dir)]
    if a.c_source:
        cmd += ["--c-source", a.c_source]
    for p in (a.plugins or []):
        cmd += ["--plugins", p]
    if private:
        cmd += ["--expose-entry"]
    r = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True, errors="replace", timeout=900)
    if r.returncode:
        return None, None, (r.stdout + r.stderr)[-300:]
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    err = F.fixups(a, pair, out_dir, entry, private, defs)
    if err:
        return None, None, err
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), RUSTUP_TOOLCHAIN=CELL.TOOLCHAIN)
    rb = subprocess.run(["cargo", "fuzz", "build", "--sanitizer", "none"], cwd=str(out_dir), env=env,
                        capture_output=True, text=True, errors="replace", timeout=1800)
    if rb.returncode:
        errs = [l for l in (rb.stdout + rb.stderr).splitlines() if l.startswith("error")]
        return None, None, "\n".join(errs[:3]) or rb.stderr[-300:]
    bins = list((target / "x86_64-unknown-linux-gnu" / "release").glob("*_ft"))
    if not bins:
        return None, None, "no binary produced"
    keep = out_dir.parent / f"{entry}.bin"
    shutil.copy(bins[0], keep)
    # the C oracle's object files: OUT_DIR of the harness crate's build script (the dir holding libc_oracle.a)
    outs = sorted((target / "x86_64-unknown-linux-gnu" / "release" / "build").glob("*/out/libc_oracle.a"),
                  key=lambda p: p.stat().st_mtime)
    objs_dir = out_dir.parent / "objs" / entry
    shutil.rmtree(objs_dir, ignore_errors=True)
    objs_dir.mkdir(parents=True)
    n = 0
    if outs:
        od = outs[-1].parent
        for o in od.rglob("*.o"):
            if o.name.endswith(".o"):
                shutil.copy(o, objs_dir / o.relative_to(od).as_posix().replace("/", "__"))
                n += 1
    # keep this boundary's compiled C copies for region-line alignment (--expose-entry edits the entry's file)
    csrc = out_dir.parent / "csrc" / entry
    shutil.rmtree(csrc, ignore_errors=True)
    if (out_dir / "c").exists():
        shutil.copytree(out_dir / "c", csrc)
    CELL._prune_target(target, CELL._crate_name(out_dir))
    shutil.rmtree(out_dir, ignore_errors=True)
    return keep, n, None


# ----------------------------------------------------------------------------- replay
def run_files(binp: Path, files, env, cwd: Path, timeout):
    """One libFuzzer process over the given input FILES (each executed once, no fuzzing)."""
    try:
        r = subprocess.run([str(binp), "-timeout=25", *[str(f) for f in files]], env=env, cwd=str(cwd),
                           stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, errors="replace",
                           timeout=timeout)
        return r.returncode, r.stderr[-4000:], False
    except subprocess.TimeoutExpired as e:
        err = e.stderr.decode("utf-8", "replace") if isinstance(e.stderr, bytes) else (e.stderr or "")
        return None, err[-4000:], True


def parse_outcome(outfile: Path):
    """Last C2R_OUTCOME line of a process, if any (normal is never printed)."""
    if not outfile.exists():
        return None
    kind = phase = None
    for line in outfile.read_text(errors="replace").splitlines():
        m = re.match(r"C2R_OUTCOME kind=(\S+) phase=(\d+)", line)
        if m:
            kind, phase = m.group(1), int(m.group(2))
    return (kind, phase) if kind else None


def replay_boundary(binp: Path, corpus: Path, prof: Path, sandbox: Path):
    """Batch in chunks; a chunk that dies is replayed one input per process. Returns (per_input, n_profraw)."""
    files = sorted(p for p in corpus.iterdir() if p.is_file())
    prof.mkdir(parents=True, exist_ok=True)
    base_env = dict(os.environ, C2R_MODE="c-only", ASAN_OPTIONS="detect_leaks=0")
    per_input = {}
    for ci in range(0, len(files), CHUNK):
        chunk = files[ci:ci + CHUNK]
        outf = prof / f"chunk{ci}.outcome"
        env = dict(base_env, LLVM_PROFILE_FILE=str(prof / f"chunk{ci}-%p.profraw"), C2R_OUTCOME_FILE=str(outf))
        rc, err, to = run_files(binp, chunk, env, sandbox, timeout=max(600, 2 * len(chunk)))
        if rc == 0 and not to:
            for f in chunk:
                per_input[f.name] = {"outcome": "completed", "phase": 2, "via": "batch"}
            continue
        # the chunk died (crash or timeout inside libFuzzer): profile lost -> one process per input
        for prf in prof.glob(f"chunk{ci}-*.profraw"):
            prf.unlink()
        for f in chunk:
            outf = prof / f"{f.name}.outcome"
            outf.unlink(missing_ok=True)
            env = dict(base_env, LLVM_PROFILE_FILE=str(prof / f"in-{f.name}-%p.profraw"), C2R_OUTCOME_FILE=str(outf))
            rc, err, to = run_files(binp, [f], env, sandbox, timeout=PER_INPUT_TIMEOUT)
            oc = parse_outcome(outf)
            outf.unlink(missing_ok=True)
            if to or "timeout" in (err or "").lower() and rc not in (0,):
                rec = {"outcome": "timeout", "phase": oc[1] if oc else None, "via": "per-input"}
            elif rc == 0:
                rec = {"outcome": "completed", "phase": 2, "via": "per-input"}
            else:
                sig = re.search(r"deadly signal|SEGV|AddressSanitizer|UndefinedBehaviorSanitizer|panicked|abort", err or "")
                rec = {"outcome": "crash", "phase": oc[1] if oc else None, "kind": oc[0] if oc else (sig.group(0) if sig else f"rc={rc}"),
                       "via": "per-input"}
            per_input[f.name] = rec
    return per_input, len(list(prof.glob("*.profraw")))


def merge_export(prof: Path, objs: list[Path], out_json: Path, empty_profdata: Path) -> str:
    raws = sorted(prof.glob("*.profraw"))
    pd = prof / "merged.profdata"
    if raws:
        r = subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-sparse", *[str(x) for x in raws], "-o", str(pd)],
                           capture_output=True, text=True, timeout=1200)
        if r.returncode or not pd.exists():
            return f"profdata merge failed: {r.stderr[-200:]}"
    else:
        pd = empty_profdata
    cmd = [str(CELL.TC / "llvm-cov"), "export", str(objs[0])] + [x for o in objs[1:] for x in ("-object", str(o))] + [f"-instr-profile={pd}"]
    with open(out_json, "w") as fh:
        r = subprocess.run(cmd, stdout=fh, stderr=subprocess.PIPE, text=True, timeout=1800)
    if r.returncode or out_json.stat().st_size == 0:
        return f"llvm-cov export failed: {r.stderr[-200:]}"
    for x in raws:
        x.unlink()
    return "ok" if raws else "ok (no profile: universe only)"


# ----------------------------------------------------------------------------- analysis
def c_name(sym: str) -> str:
    """llvm-cov names static functions `<file>:<name>`; the oracle renames every function `c_<name>`."""
    n = sym.rsplit(":", 1)[-1]
    return n[2:] if n.startswith("c_") else n


def in_scope(fname: str) -> bool:
    b = os.path.basename(fname)
    return b not in HELPER_FILES and not fname.startswith("/usr") and not fname.startswith("/lib")


def line_map(harness_copy: Path, canonical: Path) -> dict | None:
    """1-based line map from a boundary's compiled C copy to the pair's canonical file (difflib, equal and
    same-size blocks map through; --expose-entry's `static` strip can delete one line)."""
    if not harness_copy.exists() or not canonical.exists():
        return None
    a = harness_copy.read_text(errors="replace").split("\n")
    b = canonical.read_text(errors="replace").split("\n")
    if a == b:
        return None
    m = {}
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal":
            for k in range(i2 - i1):
                m[i1 + k + 1] = j1 + k + 1
        # an edited line (the `static`-stripped signature) maps to nothing: its regions have moved
        # columns and would be NEW identities; as in c2r_coverage.align_to_canonical they are dropped,
        # never added to the universe (bzip2 pilot: 2356 vs 2304 regions before this)
    return m


def extract_c(export_json: Path, csrc_dir: Path, source_root: Path):
    """(functions: name -> covered, regions: id -> covered) of the in-scope C files of one boundary."""
    d = json.load(open(export_json))["data"][0]
    funcs, regions = {}, {}
    maps = {}
    # Region identities live in the files that DEFINE functions. Macro-body code regions attributed to
    # headers that define nothing (bzlib_private.h) vary per boundary with the fixups' header edits and
    # inflated the bzip2 universe (2356 vs 2304); a header-only library (urlparser's url.h) still counts
    # because its functions are defined there.
    def_files = {os.path.basename(fn["filenames"][0]) for fn in d["functions"] if fn["filenames"] and in_scope(fn["filenames"][0])}
    for fn in d["functions"]:
        files = fn["filenames"]
        f0 = files[0] if files else ""
        if not in_scope(f0):
            continue
        base = os.path.basename(f0)
        nm = c_name(fn["name"])
        funcs[nm] = funcs.get(nm, False) or fn["count"] > 0
        if base not in maps:
            hc = next(iter(csrc_dir.rglob(base)), None) if csrc_dir.exists() else None
            cn = next(iter(source_root.rglob(base)), None)
            maps[base] = line_map(hc, cn) if hc and cn else None
        lm = maps[base]
        for r in fn["regions"]:
            l1, c1, l2, c2, cnt, fid, _efid, kind = r[:8]
            if kind != 0 or fid >= len(files) or not in_scope(files[fid]):
                continue
            fb = os.path.basename(files[fid])
            if fb not in def_files:
                continue
            if lm is not None:
                if l1 not in lm or l2 not in lm:
                    continue          # a line with no canonical counterpart (the edited signature line)
                l1, l2 = lm[l1], lm[l2]
            rid = (fb, l1, c1, l2, c2)
            regions[rid] = regions.get(rid, False) or cnt > 0
    return funcs, regions


def rust_name(raw: str) -> str:
    """Last path segment of a demangled name. The archived lists also carry half-demangled v0 tails for
    private functions (`bzip2_c2rust5bzlib12bz_config_okB3_`, `VOnCZ0b1bB_15lil_c2saferrust3lil8strclone`):
    a length-prefixed identifier chain, possibly followed by a back-reference / crate suffix (`B3_`, `C..`).
    Take the RIGHTMOST length-prefixed identifier whose remainder is empty, another length prefix, or such
    a suffix. A plain C-style name (`BZ2_bzWriteClose64`) has no valid candidate and passes through."""
    if "::" in raw:
        return raw.rsplit("::", 1)[-1]
    best = None
    for m in re.finditer(r"\d+", raw):
        n = int(m.group(0)); j = m.end()
        seg = raw[j:j + n]
        if n == 0 or len(seg) != n or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", seg):
            continue
        rest = raw[j + n:]
        if rest == "" or rest[0].isdigit() or re.fullmatch(r"[BC][A-Za-z0-9_]*", rest):
            best = seg
    return best or raw


def rust_sets(analysis: Path):
    """(universe names, covered names) from the archived four-set files of the cell (Rust side)."""
    uni, cov = set(), set()
    for fn, covered in (("covered_by_both.txt", True), ("only_ours.txt", True),
                        ("only_tests.txt", False), ("covered_by_neither.txt", False)):
        p = analysis / fn
        if not p.exists():
            continue
        for line in p.read_text().splitlines():
            parts = line.split("\t")
            if len(parts) < 3:
                continue
            nm = rust_name(parts[2].strip())
            uni.add(nm)
            if covered:
                cov.add(nm)
    return uni, cov


def load_map(lib: str, tool: str):
    for p in (ROOT / "results/rq1_matching/raw/group_a" / f"{lib}__{tool}" / "matcher_output.json",
              ROOT / "results/rq1_matching/raw/group_b" / f"{tool}_{lib}" / "matcher_output.json"):
        if p.exists():
            d = json.load(open(p))
            return {"path": str(p.relative_to(ROOT)), "deployment": [(c, r) for c, r, *_ in d["deployment"]],
                    "ambiguous": [(c, r) for c, r, *_ in d.get("deployment_ambiguous", [])],
                    "config": d.get("config")}
    return None


def rust_per_boundary(cell: Path) -> dict:
    """boundary -> Rust functions reached from its corpus, from the archived per-harness llvm-cov exports
    (harness_exports.tar.gz, ours/<b>.json); names decoded as in rust_name."""
    out = {}
    tp = cell / "harness_exports.tar.gz"
    if not tp.exists():
        return out
    with tarfile.open(tp) as tf:
        for m in tf.getmembers():
            if m.name.startswith("ours/") and m.name.endswith(".json") and m.isfile():
                try:
                    d = json.load(tf.extractfile(m))["data"][0]
                except Exception:
                    continue
                out[m.name[5:-5]] = {rust_name(CC.demangle(f["name"])) for f in d["functions"]
                                     if f["count"] > 0 and f["filenames"] and CC.is_lib(f["filenames"][0])}
    return out


def four_sets(c_funcs: dict, r_uni: set, r_cov: set, cmap, per_b_funcs=None, rust_cov_status=None,
              per_b_rust=None, c_outcomes=None):
    """per_b_funcs: boundary -> set of C functions reached from its corpus; rust_cov_status: boundary -> the
    archived cell's `coverage` field. A `c_only` function reached ONLY through boundaries whose archived
    Rust replay lost its profile (`failed rc=..`, the Rust side crashed on the input) is tagged
    rust_terminated: the Rust side TERMINATED (crash / panic / timeout) on those inputs while C went on,
    so its reach there is unmeasured rather than measured-unreached. Whether that termination is a
    translation defect (lil x C2SaferRust: construction crash, C9) or UB-associated (urlparser x c2rust
    get_part: C silently out of contract, Rust trapped) is the cell's CONFIRMATION verdict, never this
    replay's. Symmetrically a `rust_only` function reached on the Rust side (archived per-harness
    exports, per_b_rust) only through boundaries whose EVERY input crashed or timed out in the C-only
    replay (c_outcomes) is tagged c_terminated (quadtree x PtrTrans find_/get_quadrant_/quadtree_search)."""
    if cmap is None:
        return None
    c_uni = set(c_funcs)
    acc = [(c, r) for c, r in cmap["deployment"] if c in c_uni and r in r_uni]
    out_of_scope = [(c, r) for c, r in cmap["deployment"] if not (c in c_uni and r in r_uni)]
    sets = {"both": [], "c_only": [], "rust_only": [], "neither": []}
    prov = {}
    for c, r in acc:
        cc, rc = c_funcs[c], r in r_cov
        key = "both" if cc and rc else "c_only" if cc else "rust_only" if rc else "neither"
        sets[key].append([c, r])
        if key == "c_only" and per_b_funcs is not None:
            via = sorted(b for b, fs in per_b_funcs.items() if c in fs)
            st = {b: (rust_cov_status or {}).get(b) for b in via}
            lost = bool(via) and all(not str(s).startswith(("batch", "per-input")) for s in st.values())
            prov[c] = {"reached_via": via, "archived_rust_coverage": st, "rust_terminated": lost}
        if key == "rust_only" and per_b_rust is not None:
            via = sorted(b for b, fs in per_b_rust.items() if r in fs)
            oc = {b: (c_outcomes or {}).get(b, {}) for b in via}
            lost = bool(via) and all(o.get("completed", 0) == 0 for o in oc.values())
            prov[r] = {"side": "rust_only", "reached_via_rust": via, "c_outcomes_of_those": oc, "c_terminated": lost}
    amb = [[c, r, {"c": c_funcs.get(c), "rust": (r in r_cov) if r in r_uni else None}] for c, r in cmap["ambiguous"]]
    paired_c = {c for c, _ in acc} | {c for c, _ in cmap["ambiguous"]}
    paired_r = {r for _, r in acc} | {r for _, r in cmap["ambiguous"]}
    c_un = sorted(c for c in c_uni if c not in paired_c)
    r_un = sorted(r for r in r_uni if r not in paired_r)
    return {"map": cmap["path"], "config": cmap["config"], "accepted_pairs": len(acc),
            "accepted_pairs_out_of_scope": out_of_scope, "sets": sets,
            "counts": {k: len(v) for k, v in sets.items()} | {
                "c_only_rust_terminated": sum(1 for v in prov.values() if v.get("rust_terminated")),
                "rust_only_c_terminated": sum(1 for v in prov.values() if v.get("c_terminated"))},
            "exclusive_provenance": prov,
            "ambiguous": amb, "c_unmatched": c_un, "rust_unmatched": r_un,
            "c_unmatched_reached": sorted(c for c in c_un if c_funcs[c]),
            "rust_unmatched_reached": sorted(r for r in r_un if r in r_cov)}


# ----------------------------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lib", required=True); ap.add_argument("--tool", required=True)
    ap.add_argument("--work", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--only"); ap.add_argument("--keep-work", action="store_true")
    ap.add_argument("--analyze-only", action="store_true",
                    help="no rebuild / replay: recompute the analysis from <out>/c_exports, <out>/csrc and <out>/per_input.json")
    a = ap.parse_args()
    lib, tool = a.lib, a.tool
    cell = ROOT / "results/rq3_coverage" / lib / tool
    pair = ROOT / "benchmark/pairs/rq4" / f"{lib}_{tool}"
    W, O = Path(a.work), Path(a.out)
    shutil.rmtree(W, ignore_errors=True); W.mkdir(parents=True)
    O.mkdir(parents=True, exist_ok=True); (O / "c_exports").mkdir(exist_ok=True)
    t0 = time.time()
    CELL.GEN_HASH = CELL.generator_hash()
    log(f"c_reach {lib} x {tool}: generator {CELL.GEN_HASH}")

    funnel = json.load(open(cell / "funnel.json"))
    built = [r["boundary"] for r in funnel if r.get("built")]
    if a.only:
        built = [b for b in built if b in set(a.only.split(","))]
    with tarfile.open(cell / "corpus.tar.gz") as tf:
        tf.extractall(W)
    corpus_root = W / "corpus"
    csrc_name = c_source_of(cell, built)
    if not csrc_name:
        sys.exit("cannot determine the C translation unit from the archived harnesses")
    defs_path = pair / "translated" / f"{lib}_{tool}.rs.defs.json"
    defs = json.loads(defs_path.read_text()) if defs_path.exists() else {}
    private = set(defs.get("private", []))
    rs = next(iter(sorted((pair / "translated").glob("*.rs"))), None)
    rs_text = rs.read_text(errors="replace") if rs else ""
    plugins = [str(ROOT / "plugins/cjson/plugin.toml")] if lib == "cjson" else []
    ba = argparse.Namespace(c_source=csrc_name, plugins=plugins, defs=str(defs_path) if defs_path.exists() else None,
                            shim=str(ROOT / "benchmark/pairs/rq4/darwin_shims.c"), pair=str(pair))
    # an empty profile for boundaries whose every input died (universe only)
    empty_txt = W / "empty.proftext"; empty_txt.write_text("")
    empty_pd = W / "empty.profdata"
    subprocess.run([str(CELL.TC / "llvm-profdata"), "merge", "-o", str(empty_pd), str(empty_txt)],
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    rows, per_input_all = [], {}
    c_funcs_u, c_regs_u, per_b_funcs = {}, {}, {}
    target = W / "target"
    if a.analyze_only:
        old = json.load(open(O / "result.json"))
        per_input_all = json.load(open(O / "per_input.json"))
        for row in old["rows"]:
            if not str(row.get("status", "")).startswith("ok"):
                rows.append(row); continue
            b = row["boundary"]
            ex = W / f"export_{b}.json"
            ex.write_text(gzip.open(O / "c_exports" / f"{b}.json.gz", "rt").read())
            funcs, regs = extract_c(ex, O / "csrc" / b, pair / "source")
            per_b_funcs[b] = {k for k, v in funcs.items() if v}
            for k, v in funcs.items(): c_funcs_u[k] = c_funcs_u.get(k, False) or v
            for k, v in regs.items(): c_regs_u[k] = c_regs_u.get(k, False) or v
            row.update({"c_functions_reached": sum(funcs.values()), "c_functions_total": len(funcs),
                        "c_regions_reached": sum(regs.values()), "c_regions_total": len(regs)})
            rows.append(row)
        built = [r["boundary"] for r in rows]
    for b in ([] if a.analyze_only else built):
        corpus = corpus_root / b
        if not corpus.exists():
            rows.append({"boundary": b, "status": "no archived corpus"}); continue
        is_priv = b in private or (bool(rs_text) and not defs and re.search(
            rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b', rs_text) is None)
        binp, nobj, err = build_boundary(ba, pair, b, is_priv, W / "harness" / b, target)
        if not binp:
            rows.append({"boundary": b, "status": "rebuild failed", "error": err, "corpus": len(list(corpus.iterdir()))})
            log(f"  {b:30s} REBUILD FAILED {(err or '')[:100]}"); continue
        # cc names objects `<hash>-<unit>.o`; the helper units (ubshim, shims, plugin) are not the library
        objs = sorted(p for p in (W / "harness" / "objs" / b).glob("*.o")
                      if p.name.split("-", 1)[-1].replace("__", "/").rsplit("/", 1)[-1][:-2] + ".c" not in HELPER_FILES)
        if not objs:
            rows.append({"boundary": b, "status": "no C objects"}); log(f"  {b:30s} NO C OBJECTS"); continue
        per_input, nraw = replay_boundary(binp, corpus, W / "prof" / b, CELL.sandbox_dir(W))
        st = merge_export(W / "prof" / b, objs, W / f"export_{b}.json", empty_pd)
        if not st.startswith("ok"):
            rows.append({"boundary": b, "status": st, "corpus": len(per_input)}); log(f"  {b:30s} {st}"); continue
        funcs, regs = extract_c(W / f"export_{b}.json", W / "harness" / "csrc" / b, pair / "source")
        per_b_funcs[b] = {k for k, v in funcs.items() if v}
        # keep the compiled C copies that DIFFER from the pair's source (the fixups' edited files), so
        # the line alignment can be redone with --analyze-only; identical copies are not archived
        for f in (W / "harness" / "csrc" / b).rglob("*"):
            if f.is_file() and f.name not in HELPER_FILES:
                cn = next(iter((pair / "source").rglob(f.name)), None)
                if cn is not None and cn.read_bytes() != f.read_bytes():
                    dst = O / "csrc" / b / f.relative_to(W / "harness" / "csrc" / b)
                    dst.parent.mkdir(parents=True, exist_ok=True); shutil.copy(f, dst)
        for k, v in funcs.items(): c_funcs_u[k] = c_funcs_u.get(k, False) or v
        for k, v in regs.items(): c_regs_u[k] = c_regs_u.get(k, False) or v
        with gzip.open(O / "c_exports" / f"{b}.json.gz", "wt") as gz:
            gz.write((W / f"export_{b}.json").read_text())
        (W / f"export_{b}.json").unlink()
        oc = {}
        for r in per_input.values(): oc[r["outcome"]] = oc.get(r["outcome"], 0) + 1
        per_input_all[b] = per_input
        rows.append({"boundary": b, "status": st, "c_static": is_priv, "objects": len(objs), "corpus": len(per_input),
                     "outcomes": oc, "profiles": nraw, "c_functions_reached": sum(funcs.values()),
                     "c_functions_total": len(funcs), "c_regions_reached": sum(regs.values()), "c_regions_total": len(regs)})
        log(f"  {b:30s} {st:8s} corpus {len(per_input):5d} {oc} | C fn {sum(funcs.values())}/{len(funcs)} reg {sum(regs.values())}/{len(regs)}")
        binp.unlink(missing_ok=True); shutil.rmtree(W / "prof" / b, ignore_errors=True); shutil.rmtree(W / "harness" / "objs" / b, ignore_errors=True)

    # Rust side: the archived cell, never re-measured
    r_uni, r_cov = rust_sets(cell / "analysis")
    arch = json.load(open(cell / "analysis" / "result.json"))
    cmap = load_map(lib, tool)
    c_out = {}
    for b, pi in per_input_all.items():
        for v in pi.values(): c_out.setdefault(b, {})[v["outcome"]] = c_out.setdefault(b, {}).get(v["outcome"], 0) + 1
    fs = four_sets(c_funcs_u, r_uni, r_cov, cmap, per_b_funcs, {x["boundary"]: x.get("coverage") for x in funnel},
                   rust_per_boundary(cell), c_out)
    tot_oc = {}
    for pi in per_input_all.values():
        for r in pi.values(): tot_oc[r["outcome"]] = tot_oc.get(r["outcome"], 0) + 1
    res = {"cell": f"{lib}_{tool}", "generator": CELL.GEN_HASH, "c_source": csrc_name, "mode": "c-only",
           "build": "cargo fuzz build --sanitizer none; C: -O1 -g sancov + ub-free + -fprofile-instr-generate -fcoverage-mapping",
           "boundaries_archived_built": len(built), "boundaries_measured": sum(1 for r in rows if str(r.get("status", "")).startswith("ok")),
           "rows": rows, "inputs": tot_oc,
           "c": {"functions_total": len(c_funcs_u), "functions_reached": sum(c_funcs_u.values()),
                 "regions_total": len(c_regs_u), "regions_reached": sum(c_regs_u.values()),
                 "function_reach": (sum(c_funcs_u.values()) / len(c_funcs_u)) if c_funcs_u else None,
                 "region_reach": (sum(c_regs_u.values()) / len(c_regs_u)) if c_regs_u else None,
                 "universe": "all instrumented C object files of each boundary (libc_oracle.a members), helper units excluded, union over boundaries",
                 "region_kind": "llvm-cov CodeRegion (kind 0) in files that define functions (macro bodies attributed to headers excluded); lines aligned to the pair's source where the fixups edited a file, edited lines dropped"},
           "rust_archived": {"functions_total": arch["function"]["total_in_scope"], "functions_reached": arch["function"]["covered_ours"],
                             "regions_total": arch["region"]["total_in_scope"], "regions_reached": arch["region"]["covered_ours"],
                             "function_reach": arch["function"]["covered_ours"] / arch["function"]["total_in_scope"] if arch["function"]["total_in_scope"] else None,
                             "region_reach": arch["region"]["ours_coverage"], "names_in_scope": len(r_uni), "names_reached": len(r_cov)},
           "matched": ({k: v for k, v in fs.items() if k in ("map", "config", "accepted_pairs", "counts")} | {
                        "exclusive_provenance": fs["exclusive_provenance"],
                        "ambiguous": len(fs["ambiguous"]), "c_unmatched": len(fs["c_unmatched"]), "rust_unmatched": len(fs["rust_unmatched"]),
                        "c_unmatched_reached": len(fs["c_unmatched_reached"]), "rust_unmatched_reached": len(fs["rust_unmatched_reached"]),
                        "accepted_pairs_out_of_scope": len(fs["accepted_pairs_out_of_scope"])}) if fs else None,
           "seconds": round(time.time() - t0)}
    (O / "result.json").write_text(json.dumps(res, indent=1) + "\n")
    (O / "per_input.json").write_text(json.dumps(per_input_all, indent=0) + "\n")
    (O / "matched_sets.json").write_text(json.dumps(fs, indent=1) + "\n")
    (O / "c_functions.json").write_text(json.dumps({"reached": sorted(k for k, v in c_funcs_u.items() if v),
                                                    "unreached": sorted(k for k, v in c_funcs_u.items() if not v)}, indent=1) + "\n")
    write_run_md(O, res, fs, rows)
    if not a.keep_work:
        shutil.rmtree(W, ignore_errors=True)
    log(f"C_REACH_DONE {lib} x {tool}: C fn {res['c']['functions_reached']}/{res['c']['functions_total']} "
        f"reg {res['c']['regions_reached']}/{res['c']['regions_total']} | inputs {tot_oc} | matched {res['matched'] and res['matched']['counts']}")


def write_run_md(O: Path, res, fs, rows):
    c, r = res["c"], res["rust_archived"]
    L = [f"# Same-corpus C reach — {res['cell']}", "",
         "Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through",
         f"harnesses rebuilt with generator `{res['generator']}` (`--c-coverage`, C oracle `{res['c_source']}` with",
         "`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:",
         "this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`",
         "shows reach, not C-definedness.", "",
         "## Side-specific reach (not comparable across sides)", "",
         "| side | functions | regions |", "|---|---|---|",
         f"| C (this replay) | {c['functions_reached']} / {c['functions_total']}" + (f" ({c['function_reach']:.3f})" if c['function_reach'] is not None else "") +
         f" | {c['regions_reached']} / {c['regions_total']}" + (f" ({c['region_reach']:.3f})" if c['region_reach'] is not None else "") + " |",
         f"| Rust (archived campaign) | {r['functions_reached']} / {r['functions_total']}" + (f" ({r['function_reach']:.3f})" if r['function_reach'] is not None else "") +
         f" | {r['regions_reached']} / {r['regions_total']} ({r['region_reach']:.3f}) |", "",
         f"Inputs replayed on the C side: {res['inputs']} over {res['boundaries_measured']} / {res['boundaries_archived_built']} archived-built boundaries.", ""]
    if fs:
        cn = fs["counts"]
        L += ["## Matched functions — accepted pairs ∩ C scope ∩ Rust scope", "",
              f"Map: `{fs['map']}` (`deployment`, {fs['config']}); accepted pairs in both scopes: {fs['accepted_pairs']}"
              f" (out of scope: {len(fs['accepted_pairs_out_of_scope'])}).", "",
              "| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |", "|---|---|---|---|---|---|---|",
              f"| {cn['both']} | {cn['c_only']} ({cn.get('c_only_rust_terminated', 0)}) | {cn['rust_only']} ({cn.get('rust_only_c_terminated', 0)}) | {cn['neither']} | {len(fs['ambiguous'])} | "
              f"{len(fs['c_unmatched'])} ({len(fs['c_unmatched_reached'])}) | {len(fs['rust_unmatched'])} ({len(fs['rust_unmatched_reached'])}) |", ""]
        for k in ("c_only", "rust_only"):
            if fs["sets"][k]:
                L += [f"`{k}`: " + ", ".join(f"{c}→{r}" for c, r in fs["sets"][k][:40]) + (" …" if len(fs["sets"][k]) > 40 else ""), ""]
    else:
        L += ["## Matched functions", "", "No accepted correspondence map for this cell (side-specific numbers only).", ""]
    L += ["## Per boundary", "", "| boundary | status | corpus | outcomes | C fn | C reg |", "|---|---|---|---|---|---|"]
    for x in rows:
        L.append(f"| {x['boundary']} | {x.get('status')} | {x.get('corpus', '')} | {x.get('outcomes', x.get('error', ''))} | "
                 f"{x.get('c_functions_reached', '')}/{x.get('c_functions_total', '')} | {x.get('c_regions_reached', '')}/{x.get('c_regions_total', '')} |")
    L += ["", "## Procedure, deviations, and what is not established", "", "<!-- prose -->", ""]
    (O / "RUN.md").write_text("\n".join(L))


if __name__ == "__main__":
    main()
