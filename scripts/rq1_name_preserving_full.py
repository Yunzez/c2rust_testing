#!/usr/bin/env python3
"""RQ1 group A — re-run the name-preserving artifacts for FULL metrics.

The stored cells (results/rq1_matching/cells/name_preserving_v1.json) hold recall only.
This runner recomputes, per artifact:

    precision = correct / matched            (forced mode)
    recall    = correct / scorable
    coverage  = accepted_on_truth / scorable (deployment mode)
    abstention= ambiguous / scorable         (deployment mode)

Truth = name equality (these translators preserve C function names, so the mapping is
free AND correct; the matcher must recover it without reading names).

Two matcher configurations are scored for every artifact:
  forced      : matcher.match(topo=True)                  -> precision / recall
  deployment  : matcher.match(topo=True, abstain_eps=EPS) -> coverage / abstention

Resumable: an artifact whose row already exists in the output JSON is skipped ONLY if its
recorded fingerprint (matcher commit + matcher/analyzer hashes + artifact content hash +
parameters) equals the current one; otherwise it is recomputed.

Raw evidence per artifact (C analyzer JSON, Rust analyzer JSON, truth map, forced and
deployment matcher output) is archived under results/rq1_matching/raw/group_a/<key>/ so every
aggregate number can be traced back.

Duplicate leaf names (a C name that matches more than one Rust function, or a C name defined
in more than one C translation unit) cannot be scored by name equality and are recorded as
`ambiguous_truth`, excluded from `scorable`.

Usage:
  python3 scripts/rq1_name_preserving_full.py --out results/rq1_matching/rows/group_a_full.json
  python3 scripts/rq1_name_preserving_full.py --only bzip2            # one library
  python3 scripts/rq1_name_preserving_full.py --only optipng --force  # ignore the fingerprint cache

FROZEN 2026-09-01 (version 1): 31 artifacts over the 10 paper libraries, matcher/analyzer
commit 509751d, eps 0.01. The rows in results/rq1_matching/rows/group_a_full.json and the
pooled table in rows/group_a_table.json (scripts/rq1_group_a_table.py) were produced by this
version. Any change to scoring, corpus entries, exclusions or fingerprinting is a new version:
bump this block, re-run with --force, and note it in results/rq1_matching/rq1_assembled_v1.md.
"""
from __future__ import annotations
import argparse, json, os, re, subprocess, sys, tempfile, glob, shutil, hashlib
from collections import Counter

ROOT = "/home/yunzez/c2rust_testing"
STU = f"{ROOT}/tools/stu_selector"
ANALYZER = f"{STU}/analyzer/target/release/analyzer"
FW = f"{ROOT}/tools/frameworks"
sys.path.insert(0, STU)
import matcher  # noqa: E402

EPS = 0.01  # deployment abstention threshold (matches matcher_ablation_v1 Table 2)
RAW_DIR = f"{ROOT}/results/rq1_matching/raw/group_a"


def _sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()[:16]


def _git(*args):
    return subprocess.run(["git", "-C", ROOT] + list(args), capture_output=True, text=True).stdout.strip()


def tool_versions():
    """Identity of the matcher + analyzers used for this run."""
    dirty = _git("status", "--short", "--", "tools/stu_selector/matcher.py",
                 "tools/stu_selector/c_analyzer.py", "tools/stu_selector/analyzer/src")
    return {
        "repo_head": _git("rev-parse", "--short", "HEAD"),
        "matcher_commit": _git("log", "-1", "--format=%h", "--", "tools/stu_selector/matcher.py"),
        "analyzer_src_commit": _git("log", "-1", "--format=%h", "--", "tools/stu_selector/analyzer/src"),
        "matcher_sha256": _sha(f"{STU}/matcher.py"),
        "c_analyzer_sha256": _sha(f"{STU}/c_analyzer.py"),
        "analyzer_bin_sha256": _sha(ANALYZER),
        "stu_selector_dirty": bool(dirty),
    }


def tree_hash(paths):
    """Content hash of a set of files (sorted by path), for artifact identity."""
    h = hashlib.sha256()
    for p in sorted(paths):
        h.update(p.encode()); h.update(b"\0"); h.update(open(p, "rb").read()); h.update(b"\0")
    return h.hexdigest()[:16]


def rust_files(crate_dir):
    out = []
    for d, dirs, files in os.walk(crate_dir):
        dirs[:] = [x for x in dirs if x not in ("target", ".git", "analysis_results")]
        out += [os.path.join(d, f) for f in files if f.endswith(".rs") or f == "Cargo.toml"]
    return out


def c_headers(lib, prov):
    dirs = {os.path.dirname(p) for p in C_SRC[(lib, prov)] if os.path.exists(p)}
    dirs |= set(EXTRA_INC.get(lib, []))
    out = []
    for d in sorted(dirs):
        if os.path.isdir(d):
            out += [os.path.join(d, f) for f in os.listdir(d) if f.endswith(".h")]
    return out


def fingerprint(lib, tool, crate, prov, versions):
    params = {"eps": EPS, "topo": True, "rust_exclude": sorted(RUST_EXCLUDE.get(lib, set())),
              "c_sources": [x.replace(ROOT + "/", "") for x in C_SRC[(lib, prov)]]}
    fp = {
        **versions,
        "rust_artifact_sha256": tree_hash(rust_files(crate)),
        "c_sources_sha256": tree_hash([p for p in C_SRC[(lib, prov)] if os.path.exists(p)]),
        # headers in the source dirs + EXTRA_INC (config headers such as pnglibconf.h change
        # which functions exist; they must invalidate the cache like the .c files do)
        "c_headers_sha256": tree_hash(c_headers(lib, prov)),
        "params": params,
    }
    fp["id"] = hashlib.sha256(json.dumps(fp, sort_keys=True).encode()).hexdigest()[:16]
    return fp

# C sources, keyed by (library, C provenance). Library-core translation units only: drivers,
# tests, examples and bench harnesses are excluded, because extra C functions become false
# attractors. The SAME hygiene is applied on the Rust side (RUST_EXCLUDE below).
#
# Provenance matters: laertes_benchmarks and crown/c-code ship DIFFERENT versions of some
# libraries (genann.c 2015 vs 2018: 12 vs 15 functions; lil.c: 145 vs 128). The C reference
# for an artifact must be the version that artifact was translated from, otherwise
# version-skew functions are counted as matcher misses.
LBC = f"{FW}/c2saferrust/laertes_benchmarks"
CC = f"{FW}/crown/c-code"
C_SRC = {
    # laertes_benchmarks lineage (c2rust baseline, Laertes, C2SaferRust)
    ("genann", "lb"):    [f"{LBC}/genann/genann.c"],
    ("lil", "lb"):       [f"{LBC}/lil/lil.c"],
    ("urlparser", "lb"): [f"{LBC}/urlparser/test.c"],   # md5-identical to crown/c-code
    ("qsort", "lb"):     [f"{LBC}/qsort/qsort.c"],
    ("bzip2", "lb"):     [f"{CC}/bzip2/{f}" for f in                     # no C in LB dir
                          ("blocksort.c", "huffman.c", "crctable.c", "randtable.c",
                           "compress.c", "decompress.c", "bzlib.c")],
    # crown lineage (crown/benchmark c2rust input, crown/results CROWN output)
    ("bzip2", "crown"):    [f"{CC}/bzip2/{f}" for f in
                            ("blocksort.c", "huffman.c", "crctable.c", "randtable.c",
                             "compress.c", "decompress.c", "bzlib.c")],
    ("lodepng", "crown"):  [f"{CC}/lodepng/lodepng.c"],
    ("quadtree", "crown"): sorted(glob.glob(f"{CC}/quadtree-0.1.0/src/*.c")),
    # CROWN's genann output lacks the 2018-only functions (genann_act_*_indirect,
    # genann_init_sigmoid_lookup): it was translated from the 2015 genann.c, same as LB.
    ("genann", "crown"):   [f"{LBC}/genann/genann.c"],
    ("urlparser", "crown"): [f"{CC}/urlparser/test.c"],
    ("lil", "crown"):      [f"{CC}/lil/lil.c"],
    ("cjson", "crown"):    [f"{FW}/CRUST-bench/datasets/CBench/cJSON/src/cJSON.c"],
    # tulipindicators: every Rust artifact (LB c2rust/Laertes/C2SaferRust and CROWN in+out) reports
    # ti_version()=="0.8.4", TI_BUILD 1537377628 == upstream commit 6b3ff6d. The repo's
    # tools/frameworks/tulipindicators checkout is 0.9.2 (adds ti_stream_*, candles.c, 2 stream
    # indicators), so the reference is `git archive 6b3ff6d` -> tulipindicators_v0.8.4/ (PROVENANCE.txt).
    # Library core = indicators/*.c + indicators_index.c + utils/buffer.c; drivers excluded both sides.
    ("tulip", "lb"):    sorted(glob.glob(f"{FW}/tulipindicators_v0.8.4/indicators/*.c")) +
                        [f"{FW}/tulipindicators_v0.8.4/indicators_index.c",
                         f"{FW}/tulipindicators_v0.8.4/utils/buffer.c"],
    ("tulip", "crown"): sorted(glob.glob(f"{FW}/tulipindicators_v0.8.4/indicators/*.c")) +
                        [f"{FW}/tulipindicators_v0.8.4/indicators_index.c",
                         f"{FW}/tulipindicators_v0.8.4/utils/buffer.c"],
    # optipng: Rust artifact = OptiPNG 0.7.7 (zlib 1.2.11-optipng, libpng 1.6.34, ioutil.c not
    # osys.c). Both C copies in the repo (crown_dataset/optipng, rustassure inputs) are 0.7.6 /
    # libpng 1.6.21, so the reference is the upstream 0.7.7 tarball -> optipng-0.7.7/ (PROVENANCE.txt).
    # One C file per Rust module; libpng/pngtest.c and zlib/test/{example,minigzip}.c are test
    # drivers and are excluded on both sides.
    ("optipng", "lb"): [f"{FW}/optipng-0.7.7/src/{f}" for f in (
        "gifread/gifread.c",
        *[f"libpng/{x}.c" for x in ("png", "pngerror", "pngget", "pngmem", "pngpread", "pngread",
                                    "pngrio", "pngrtran", "pngrutil", "pngset", "pngtrans",
                                    "pngwio", "pngwrite", "pngwtran", "pngwutil")],
        "minitiff/tiffread.c", "minitiff/tiffutil.c", "opngreduc/opngreduc.c",
        *[f"optipng/{x}.c" for x in ("bitset", "ioutil", "optim", "optipng", "ratio", "wildargs")],
        *[f"pngxtern/{x}.c" for x in ("pngxio", "pngxmem", "pngxrbmp", "pngxread", "pngxrgif",
                                      "pngxrjpg", "pngxrpnm", "pngxrtif", "pngxset")],
        "pnmio/pnmin.c", "pnmio/pnmout.c", "pnmio/pnmutil.c",
        *[f"zlib/{x}.c" for x in ("adler32", "compress", "crc32", "deflate", "gzclose", "gzlib",
                                  "gzread", "gzwrite", "infback", "inffast", "inflate", "inftrees",
                                  "trees", "uncompr", "zutil")])],
}

# Extra include directories per library (default: the file's directory and its parent).
EXTRA_INC = {
    "tulip":   [f"{FW}/tulipindicators_v0.8.4", f"{FW}/tulipindicators_v0.8.4/utils"],
    "optipng": [f"{FW}/optipng-0.7.7/src/{d}" for d in
                ("cexcept", "gifread", "libpng", "minitiff", "opngreduc", "optipng",
                 "pngxtern", "pnmio", "zlib")],
}

# Rust-side hygiene: module names (file stems) that are drivers / examples / tests, not
# library core. They are removed from the crate root's `pub mod` list in a staged copy so
# that module paths of the remaining files are unchanged and cross-module calls still resolve.
RUST_EXCLUDE = {
    "genann":   {"example1", "example2", "example3", "example4", "test"},
    "lil":      {"main"},
    "bzip2":    {"bzip2", "bzip2recover"},
    "quadtree": {"test"},
    "tulip":    {"example1", "example2", "fuzzer", "sample", "smoke"},
    "optipng":  {"pngtest", "example", "minigzip"},
    # urlparser: single test.rs (library + main) mirrors single test.c — kept whole.
    # qsort / lodepng / cjson: single library file, nothing to exclude.
}


def c_provenance(crate_dir):
    return "lb" if crate_dir.startswith(LBC + "/") else "crown"

# Rust artifact per (library, tool).
#
# PROVENANCE (verified 2026-09-01 — an earlier revision of this file conflated two of these
# and silently scored the same crate twice):
#   c2saferrust/laertes_benchmarks/<lib>          = c2rust baseline that Laertes and
#                                                   C2SaferRust were derived from
#   c2saferrust/laertes_benchmarks/<lib>_WIP      = C2SaferRust
#   c2saferrust/laertes_benchmarks/<lib>_laertes  = Laertes
#   crown/benchmark/<lib>   = c2rust baseline CROWN consumes  (raw pointers, zero `Option<&`)
#   crown/results/<lib>     = CROWN output                    (`Option<&` present)
# The c2rust column uses the baseline the row's other tools actually consumed.
LB = f"{FW}/c2saferrust/laertes_benchmarks"
CB = f"{FW}/crown/benchmark"      # c2rust baseline (CROWN's input)
CR = f"{FW}/crown/results"        # CROWN output
RUST = {
    ("bzip2", "c2rust"): f"{LB}/bzip2",           ("bzip2", "laertes"): f"{LB}/bzip2_laertes",
    ("bzip2", "c2saferrust"): f"{LB}/bzip2_WIP",  ("bzip2", "crown"): f"{CR}/bzip2",
    ("genann", "c2rust"): f"{LB}/genann",         ("genann", "laertes"): f"{LB}/genann_laertes",
    ("genann", "c2saferrust"): f"{LB}/genann_WIP", ("genann", "crown"): f"{CR}/genann",
    ("lil", "c2rust"): f"{LB}/lil",               ("lil", "laertes"): f"{LB}/lil_laertes",
    ("lil", "c2saferrust"): f"{LB}/lil_WIP",      ("lil", "crown"): f"{CR}/lil",
    ("urlparser", "c2rust"): f"{LB}/urlparser",   ("urlparser", "laertes"): f"{LB}/urlparser_laertes",
    ("urlparser", "c2saferrust"): f"{LB}/urlparser_WIP", ("urlparser", "crown"): f"{CR}/urlparser",
    ("qsort", "c2rust"): f"{LB}/qsort",           ("qsort", "laertes"): f"{LB}/qsort_laertes",
    ("qsort", "c2saferrust"): f"{LB}/qsort_WIP",
    # laertes_benchmarks has no lodepng/quadtree/cJSON: use the CROWN input/output pair.
    ("lodepng", "c2rust"): f"{CB}/lodepng",       ("lodepng", "crown"): f"{CR}/lodepng",
    ("quadtree", "c2rust"): f"{CB}/quadtree",     ("quadtree", "crown"): f"{CR}/quadtree",
    ("cjson", "c2rust"): f"{FW}/crown/oldc2rust/pilot_out/cJSON",
    ("tulip", "c2rust"): f"{LB}/tulipindicators",  ("tulip", "laertes"): f"{LB}/tulipindicators_laertes",
    ("tulip", "c2saferrust"): f"{LB}/tulipindicators_WIP", ("tulip", "crown"): f"{CR}/tulipindicators",
    ("optipng", "c2rust"): f"{LB}/optipng",        ("optipng", "laertes"): f"{LB}/optipng_laertes",
    ("optipng", "c2saferrust"): f"{LB}/optipng_WIP",
    # optipng × CROWN: no artifact exists — CROWN's `analyse` panicked, analysis_results/ empty
    # (results/rq4_effectiveness/bugs/optipng_crown/). N/A, documented.
}

# The 24 artifacts recorded in cells/name_preserving_v1.json + tulip (4) and optipng (3), added 2026-09-01.
ARTIFACTS = [
    ("genann", t) for t in ("c2rust", "laertes", "c2saferrust", "crown")
] + [
    ("qsort", t) for t in ("c2rust", "laertes", "c2saferrust")
] + [
    ("lil", t) for t in ("c2rust", "laertes", "c2saferrust", "crown")
] + [
    ("urlparser", t) for t in ("c2rust", "laertes", "c2saferrust", "crown")
] + [
    ("lodepng", t) for t in ("c2rust", "crown")
] + [
    ("quadtree", t) for t in ("c2rust", "crown")
] + [
    ("cjson", "c2rust")
] + [
    ("bzip2", t) for t in ("c2rust", "laertes", "c2saferrust", "crown")
] + [
    ("tulip", t) for t in ("c2rust", "laertes", "c2saferrust", "crown")
] + [
    ("optipng", t) for t in ("c2rust", "laertes", "c2saferrust")
]

DEV_LIBS = {"cjson", "lil"}  # results/rq1_matching/SPLIT.md — frozen, library-disjoint


def make_compile_commands(lib, prov, workdir):
    """Emit a compile_commands.json for the library-core .c files."""
    srcs = [p for p in C_SRC[(lib, prov)] if os.path.exists(p)]
    if not srcs:
        raise FileNotFoundError(f"no C sources found for {lib}/{prov}")
    entries = []
    for s in srcs:
        d = os.path.dirname(s)
        entries.append({
            "directory": d,
            "file": s,
            "arguments": ["clang", "-c", s, f"-I{d}", "-I" + os.path.dirname(d), "-w"]
                         + [f"-I{i}" for i in EXTRA_INC.get(lib, [])],
        })
    os.makedirs(workdir, exist_ok=True)
    with open(f"{workdir}/compile_commands.json", "w") as f:
        json.dump(entries, f, indent=1)
    return len(srcs)


def analyze_c(lib, prov, workdir, out):
    make_compile_commands(lib, prov, workdir)
    with open(out, "w") as o:
        subprocess.run([sys.executable, f"{STU}/c_analyzer.py",
                        "--compile-commands", workdir, "--enable-metrics"],
                       stdout=o, stderr=subprocess.DEVNULL, check=True)


def lib_root(crate_dir):
    """Path of the crate's library root, from Cargo.toml [lib] path (default src/lib.rs)."""
    toml = open(f"{crate_dir}/Cargo.toml").read()
    m = re.search(r"\[lib\][^\[]*?path\s*=\s*\"([^\"]+)\"", toml, re.S)
    return m.group(1) if m else "src/lib.rs"


def stage_crate(crate_dir, lib, tmp):
    """Copy the crate and prune driver/example/test modules (RUST_EXCLUDE).

    Pruning is done by dropping `pub mod X;` lines from the lib root and every `[[bin]]`
    target from Cargo.toml, so the surviving modules keep their exact paths.
    Returns (staged_dir, list_of_excluded_modules_actually_found).
    """
    stage = tempfile.mkdtemp(dir=tmp)
    shutil.rmtree(stage)
    shutil.copytree(crate_dir, stage, symlinks=True,
                    ignore=shutil.ignore_patterns("target", ".git", "analysis_results"))
    toml_p = f"{stage}/Cargo.toml"
    if not os.path.exists(toml_p):
        # laertes_benchmarks-style loose crate without a manifest: single-file library.
        rs = sorted(f for f in os.listdir(stage) if f.endswith(".rs"))
        rs = [f for f in rs if f[:-3] not in RUST_EXCLUDE.get(lib, set())]
        if len(rs) != 1:
            raise RuntimeError(f"cannot infer lib root among {rs} in {crate_dir}")
        open(toml_p, "w").write(
            f'[package]\nname="stage"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="{rs[0]}"\n')
        return stage, []
    toml = open(toml_p).read()
    toml = re.sub(r"\[\[bin\]\][^\[]*", "", toml)
    open(toml_p, "w").write(toml)
    excl = RUST_EXCLUDE.get(lib, set())
    root = f"{stage}/{lib_root(stage)}"
    src = open(root, encoding="utf-8", errors="replace").read()
    found = []
    for m in excl:
        pat = re.compile(rf"^\s*(pub\s+)?mod\s+{re.escape(m)}\s*;\s*$", re.M)
        if pat.search(src):
            found.append(m)
            src = pat.sub("", src)
    open(root, "w").write(src)
    return stage, sorted(found)


def analyze_rust(crate_dir, lib, out, tmp):
    target, pruned = stage_crate(crate_dir, lib, tmp)
    with open(out, "w") as o:
        subprocess.run([ANALYZER, target, "--enable-metrics"],
                       stdout=o, stderr=subprocess.DEVNULL, check=True)
    return pruned


def leaf(n):
    return n.split("::")[-1]


def score(cj, rj, raw_dir=None):
    c = json.load(open(cj))
    r = json.load(open(rj))
    c_count = Counter(f["name"] for f in c["functions"])
    cn = set(c_count)
    rn = {f["name"] for f in r["functions"]}
    rleaf = {}
    for n in rn:
        rleaf.setdefault(leaf(n), []).append(n)
    # name-equality truth: a C function whose name survives into the Rust artifact, EXACTLY
    # once on each side. A leaf defined twice in Rust (e.g. `foo` and `Impl::foo`) or a C name
    # defined in two translation units is ambiguous under name equality and is not scored.
    ambiguous = {}
    truth = {}
    for n in cn:
        if n not in rleaf:
            continue
        if len(rleaf[n]) > 1:
            ambiguous[n] = {"why": "duplicate Rust leaf", "rust": sorted(rleaf[n])}
        elif c_count[n] > 1:
            ambiguous[n] = {"why": "duplicate C name", "c_count": c_count[n]}
        else:
            truth[n] = rleaf[n][0]
    scorable = len(truth)
    if scorable == 0:
        return None

    forced = matcher.match(c, r, topo=True)
    got = {cc: rr for (cc, rr, _s, _k) in forced["matched"]}
    matched = len(got)
    correct = sum(1 for k, v in truth.items() if leaf(got.get(k, "")) == leaf(v))

    dep = matcher.match(c, r, topo=True, abstain_eps=EPS)
    dgot = {cc: rr for (cc, rr, _s, _k) in dep["matched"]}
    damb = len(dep.get("ambiguous", []))
    daccept_on_truth = sum(1 for k in truth if k in dgot)
    dcorrect = sum(1 for k, v in truth.items() if leaf(dgot.get(k, "")) == leaf(v))

    if raw_dir:
        os.makedirs(raw_dir, exist_ok=True)
        shutil.copy(cj, f"{raw_dir}/c_analyzer.json")
        shutil.copy(rj, f"{raw_dir}/rust_analyzer.json")
        json.dump({"truth": truth, "ambiguous_truth": ambiguous},
                  open(f"{raw_dir}/truth.json", "w"), indent=1, sort_keys=True)
        json.dump({"forced": forced["matched"], "forced_ambiguous": forced.get("ambiguous", []),
                   "deployment": dep["matched"], "deployment_ambiguous": dep.get("ambiguous", []),
                   "config": {"topo": True, "abstain_eps": EPS}},
                  open(f"{raw_dir}/matcher_output.json", "w"), indent=1)

    return {
        "scorable": scorable,
        "ambiguous_truth": len(ambiguous),
        "ambiguous_truth_detail": ambiguous,
        "c_functions": len(cn),
        "rust_functions": len(rn),
        "forced": {
            "matched": matched,
            "correct": correct,
            "precision": round(correct / matched, 4) if matched else None,
            "recall": round(correct / scorable, 4),
        },
        "deployment_eps": EPS,
        "deployment": {
            "matched": len(dgot),
            "correct": dcorrect,
            "ambiguous": damb,
            "precision": round(dcorrect / len(dgot), 4) if dgot else None,
            "recall": round(dcorrect / scorable, 4),
            "coverage": round(daccept_on_truth / scorable, 4),
            "abstention_rate": round(damb / scorable, 4),
        },
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=f"{ROOT}/results/rq1_matching/rows/group_a_full.json")
    ap.add_argument("--only", help="restrict to one library")
    ap.add_argument("--force", action="store_true", help="recompute even if fingerprint unchanged")
    args = ap.parse_args()

    rows = {}
    if os.path.exists(args.out):
        rows = json.load(open(args.out))

    versions = tool_versions()
    rows["_meta"] = {"runner": "scripts/rq1_name_preserving_full.py", "tool_versions": versions,
                     "raw_dir": RAW_DIR.replace(ROOT + "/", ""), "eps": EPS}
    tmp = tempfile.mkdtemp(prefix="rq1a_")
    try:
        for lib, tool in ARTIFACTS:
            if args.only and lib != args.only:
                continue
            key = f"{lib}__{tool}"
            crate = RUST.get((lib, tool))
            try:
                if not crate or not os.path.isdir(crate):
                    raise FileNotFoundError(f"missing Rust artifact for {key}: {crate}")
                prov = c_provenance(crate)
                fp = fingerprint(lib, tool, crate, prov, versions)
                old = rows.get(key, {})
                if "error" not in old and old.get("fingerprint", {}).get("id") == fp["id"] \
                        and not args.force:
                    print(f"skip {key} (fingerprint {fp['id']} unchanged)", flush=True)
                    continue
                cj, rj = f"{tmp}/{lib}_{prov}_c.json", f"{tmp}/{key}_r.json"
                if not os.path.exists(cj):
                    analyze_c(lib, prov, f"{tmp}/{lib}_{prov}_build", cj)
                pruned = analyze_rust(crate, lib, rj, tmp)
                res = score(cj, rj, raw_dir=f"{RAW_DIR}/{key}")
                if res is None:
                    rows[key] = {"error": "no name-equality truth", "split":
                                 "dev" if lib in DEV_LIBS else "eval"}
                else:
                    res["split"] = "dev" if lib in DEV_LIBS else "eval"
                    res["rust_artifact"] = crate.replace(ROOT + "/", "")
                    res["c_provenance"] = prov
                    res["c_sources"] = [x.replace(ROOT + "/", "") for x in C_SRC[(lib, prov)]]
                    res["rust_modules_pruned"] = pruned
                    res["fingerprint"] = fp
                    rows[key] = res
                print(f"done {key}: {json.dumps(rows[key].get('forced', rows[key]))}", flush=True)
            except Exception as e:
                rows[key] = {"error": str(e)[:200],
                             "split": "dev" if lib in DEV_LIBS else "eval"}
                print(f"FAIL {key}: {str(e)[:160]}", flush=True)
            with open(args.out, "w") as f:
                json.dump(rows, f, indent=1, sort_keys=True)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    print(f"\nwrote {args.out}")


if __name__ == "__main__":
    main()
