#!/usr/bin/env python3
"""RQ1 group B — annotation scaffolding for the renaming translators (PtrTrans, SACTOR).

Group B = artifacts where the translator does not preserve function names, so name-equality
truth is unavailable and ground truth must be labeled by hand. This script does NOT label
anything; it lays out, per C function, everything an annotator needs side by side:

    tool claim      the translator's own shipped map, if it ships one
                    (PtrTrans: rust_definition_name in *_trans_metadata.jsonl;
                     SACTOR:   specs/function_name_map.json)
    name-eq         a Rust function with the identical name, if any
    matcher         forced proposal (+ score, confidence) and deployment proposal / ABSTAIN

and leaves `truth` / `truth_note` blank. Per artifact it records whether the crate builds,
the translator's own per-function verdict where one exists, the tool-version block and
content hashes of both inputs (same fingerprint scheme as scripts/rq1_name_preserving_full.py),
and archives the raw analyzer outputs.

Static matching does not require compilation: every artifact with parseable Rust is
scaffolded, buildable or not. `builds` is recorded so the two are never pooled silently.

Outputs (per case):  results/rq1_matching/annotation/<case>/
    README.md           artifact facts, counts, labeling rules
    sheet.csv           one row per C function  <-- the thing to annotate
    sheet.json          same, machine-readable, plus raw matcher output + fingerprint
    rust_inventory.csv  every Rust function in the crate (file, line, signature, who points at it)
and raw analyzer outputs in results/rq1_matching/raw/group_b/<case>/.

Usage:
    python3 scripts/rq1_group_b_scaffold.py --case ptrtrans_quadtree
    python3 scripts/rq1_group_b_scaffold.py --all
"""
from __future__ import annotations
import argparse, csv, glob, hashlib, json, os, re, subprocess, sys, tempfile, shutil

ROOT = "/home/yunzez/c2rust_testing"
STU = f"{ROOT}/tools/stu_selector"
ANALYZER = f"{STU}/analyzer/target/release/analyzer"
sys.path.insert(0, STU)
import matcher  # noqa: E402

EPS = 0.01  # deployment abstention threshold, same as group A / matcher_ablation_v1
OUT = f"{ROOT}/results/rq1_matching/annotation"
RAW = f"{ROOT}/results/rq1_matching/raw/group_b"

FW = f"{ROOT}/tools/frameworks"
DS = f"{FW}/ptrtrans_rebuild/PtrTrans-C2Rust/dataset"
PA, KG, CD = f"{DS}/PA_trans_projects", f"{DS}/Trans_C-Rust-KG", f"{DS}/crown_dataset"
LBC = f"{FW}/c2saferrust/laertes_benchmarks"
SACT = f"{FW}/sactor/tests/c_examples"
BUGS = f"{ROOT}/results/rq4_effectiveness/bugs"
GBR = f"{ROOT}/results/rq1_matching/raw/group_b_runs"   # 2026-09-02 paid runs (archived raw outputs)

# C reference = the copy the translator itself consumed (crown_dataset for the shipped PtrTrans
# artifacts; it is NOT always identical to crown/c-code — quadtree bounds.c/node.c differ).
# `builds` = whole-crate cargo verdict (topology_resolution.md / translation_matrix.md).
# `rust_exclude_fns` = driver functions WE added on the C side to satisfy the translator's
# test requirement (SACTOR needs a `main`), removed on the Rust side so both sides are
# library-core only, the same hygiene as group A.
CASES = {
    "ptrtrans_qsort": dict(
        tool="PtrTrans", lib="qsort", split="eval", builds=True,
        crate=f"{PA}/qsort", map_kind="ptrtrans", map=f"{PA}/qsort_Trans_PA_trans_metadata.jsonl",
        c=[f"{CD}/qsort/qsort.c"]),
    "ptrtrans_quadtree": dict(
        tool="PtrTrans", lib="quadtree", split="eval", builds=True,
        crate=f"{KG}/quadtree", map_kind="ptrtrans", map=f"{KG}/quadtree_Trans_PA_trans_metadata.jsonl",
        c=sorted(glob.glob(f"{CD}/quadtree/src/*.c"))),
    "ptrtrans_bzip2": dict(
        tool="PtrTrans", lib="bzip2", split="eval", builds=False,
        crate=f"{PA}/bzip2", map_kind="ptrtrans", map=f"{PA}/bzip2_Trans_PA_trans_metadata.jsonl",
        c=[f"{CD}/bzip2/{f}" for f in ("blocksort.c", "huffman.c", "crctable.c", "randtable.c",
                                        "compress.c", "decompress.c", "bzlib.c")]),
    "ptrtrans_lodepng": dict(
        tool="PtrTrans", lib="lodepng", split="eval", builds=False,
        crate=f"{PA}/lodepng", map_kind="ptrtrans", map=f"{PA}/lodepng_Trans_PA_trans_metadata.jsonl",
        c=[f"{CD}/lodepng/lodepng.c"]),
    # Our own PtrTrans run (gpt-5.1, Trans_PA) on cJSON 1.7.19 — DEV library. The pipeline's
    # per-function metadata was not retained; only the committed crate survives, so no tool map.
    # src/bin/diffdrv.rs is OUR differential driver and is dropped before analysis.
    "ptrtrans_cjson": dict(
        tool="PtrTrans", lib="cjson", split="dev", builds=True,
        crate=f"{BUGS}/cjson_ptrtrans/translated_crate", map_kind=None, map=None,
        c=[f"{BUGS}/cjson_ptrtrans/c_reference/cJSON.c"], drop_dirs=["src/bin"]),
    # SACTOR (gpt-5.1) qsort: idiomatic output + its function_name_map.json. SACTOR's input
    # qsort.c is laertes_benchmarks qsort.c plus a `main` driver we appended (diff = main only),
    # so the C reference is the LB file and `main` is excluded on the Rust side.
    "sactor_qsort": dict(
        tool="SACTOR", lib="qsort", split="eval", builds=True,
        crate=f"{SACT}/qsort/result/translated_code_idiomatic/combined.rs",
        map_kind="sactor", map=f"{SACT}/qsort/result/translated_code_idiomatic/specs/function_name_map.json",
        c=[f"{LBC}/qsort/qsort.c"], rust_exclude_fns={"main"}),
    # SACTOR genann: per-function outputs assembled into one file (documented repairs, see
    # bugs/genann_sactor/README.md); no name map shipped. C = 2018 15-function genann.c
    # (crown/c-code/genann-1.0.0, md5 506e9d5b), the version SACTOR consumed. `main` is our driver.
    "sactor_genann": dict(
        tool="SACTOR", lib="genann", split="eval", builds=True,
        crate=f"{BUGS}/genann_sactor/assembled_translation.rs", map_kind=None, map=None,
        c=[f"{FW}/crown/c-code/genann-1.0.0/c/genann.c"], rust_exclude_fns={"main"}),
    # ---- 2026-09-02 runs (this repo's own paid runs; raw outputs + RUN.md under GBR) ----
    # PtrTrans urlparser: input = crown_dataset test.c (jwerle url.h + upstream test.c incl.
    # its own `main`, md5-identical to group A's file, which keeps main on both sides). The crate's
    # src/test.rs carries the translated `main` plus two translator-invented helpers
    # (box_i8_eq_str, box_cchar_eq_str); nothing is excluded — same hygiene as group A urlparser.
    "ptrtrans_urlparser": dict(
        tool="PtrTrans", lib="urlparser", split="eval", builds=True,
        crate=f"{GBR}/ptrtrans_urlparser/PA_trans_projects/urlparser", map_kind="ptrtrans",
        map=f"{GBR}/ptrtrans_urlparser/PA_trans_projects/urlparser_Trans_PA_trans_metadata.jsonl",
        c=[f"{GBR}/ptrtrans_urlparser/input/urlparser/test.c"]),
    # PtrTrans genann: C = the copy the tool consumed (crown_dataset genann.c minus the four
    # `unused` parameter attributes, see RUN.md; no semantic change). No driver on either side.
    "ptrtrans_genann": dict(
        tool="PtrTrans", lib="genann", split="eval", builds=True,
        crate=f"{GBR}/ptrtrans_genann/PA_trans_projects/genann", map_kind="ptrtrans",
        map=f"{GBR}/ptrtrans_genann/PA_trans_projects/genann_Trans_PA_trans_metadata.jsonl",
        c=[f"{GBR}/ptrtrans_genann/input/genann/genann.c"]),
    # SACTOR urlparser — PARTIAL (7/22 functions; URL_SCHEMES global never translates, see RUN.md).
    # Rust = idiomatic per-function outputs concatenated verbatim (assembled_idiomatic.rs) + the
    # shipped function_name_map.json. C = run-4 input (driver.c includes url.h with `static`
    # removed from three helpers); `main` there is OUR driver and was never attempted.
    # labels.json must carry artifact_status PARTIAL so the scorer keeps it out of the primary table.
    # PtrTrans lil, 2026-09-02 paid run — PARTIAL (translator crashed at unit 95/131 on an 807k-token
    # prompt; partial crate builds). C scope = lil.c only, as in group A; the 5 CLI functions from
    # main.c (src/main_mod.rs) are excluded on the Rust side for symmetry.  labels.json carries
    # artifact_status PARTIAL -> scored on its own line, never in the primary aggregate.
    "ptrtrans_lil": dict(
        tool="PtrTrans", lib="lil", split="eval", builds=True,
        crate=f"{GBR}/ptrtrans_lil/PA_trans_projects/lil", map_kind="ptrtrans",
        map=f"{GBR}/ptrtrans_lil/PA_trans_projects/lil_Trans_PA_trans_metadata.jsonl",
        c=[f"{GBR}/ptrtrans_lil/input/lil/lil.c"],
        rust_exclude_fns={"do_system", "fnc_system", "fnc_readline", "do_exit", "fnc_writechar"}),
    # SACTOR quadtree, 2026-09-02 paid rerun (July output was produced but lost) — PARTIAL: point/bounds/
    # node TUs translated (12 fns, unidiomatic phase, SACTOR-verified), quadtree.c refused (circular deps),
    # driver.c link-fail. Scored artifact = unidiomatic phase (idiomatic reached only point.c's 2 fns);
    # the only name map SACTOR wrote covers those 2 and is identity, so no map is used. C scope = the
    # library TUs (driver.c excluded on both sides, like group A). artifact_status PARTIAL in labels.json.
    "sactor_quadtree": dict(
        tool="SACTOR", lib="quadtree", split="eval", builds=False,
        crate=f"{GBR}/sactor_quadtree/assembled_unidiomatic.rs", map_kind=None, map=None,
        c=[f"{GBR}/sactor_quadtree/input/{f}" for f in ("point.c", "bounds.c", "node.c", "quadtree.c")]),
    # SACTOR tulip, 2026-09-02 paid run — PARTIAL, non-building: 69 unidiomatic function bodies recovered
    # from the tool log (no TU passed verification: harness link failure on the ti_indicators[] table).
    # C scope = the library TUs SACTOR consumed (sample.c driver excluded); 0.9.1-8 checkout, not the
    # v0.8.4 tree of group A, so C-function counts differ from group A's tulip rows.
    "sactor_lodepng": dict(tool="SACTOR", lib="lodepng", split="eval", builds=False,
        # 2026-09-02 run; SACTOR stopped at 74/235 fns (unidiomatic phase, no idiomatic map);
        # driver.c is our harness and is not scored. Same lodepng.c as ptrtrans_lodepng.
        crate=f"{GBR}/sactor_lodepng/assembled_unidiomatic.rs", map_kind=None, map=None,
        c=[f"{GBR}/sactor_lodepng/input/lodepng.c"]),
    "sactor_bzip2": dict(tool="SACTOR", lib="bzip2", split="eval", builds=False,
        # 2026-09-02 run 2; SACTOR stopped at 32/64 library fns (unidiomatic phase, no idiomatic
        # map). C = the same 7 bzip2 files as ptrtrans_bzip2 (the run's input folded the two
        # table TUs into bzlib.c, a neutral relocation — see RUN.md); driver.c is ours, not scored.
        crate=f"{GBR}/sactor_bzip2/assembled_unidiomatic.rs", map_kind=None, map=None,
        c=[f"{CD}/bzip2/{f}" for f in ("blocksort.c", "huffman.c", "crctable.c", "randtable.c",
                                        "compress.c", "decompress.c", "bzlib.c")]),
    "sactor_tulip": dict(
        tool="SACTOR", lib="tulip", split="eval", builds=False,
        crate=f"{GBR}/sactor_tulip/assembled_unidiomatic.rs", map_kind=None, map=None,
        c=[f"{FW}/tulipindicators/indicators.c", f"{FW}/tulipindicators/utils/buffer.c",
           f"{FW}/tulipindicators/candles.c"] + sorted(glob.glob(f"{FW}/tulipindicators/indicators/*.c"))),
    "sactor_urlparser": dict(
        tool="SACTOR", lib="urlparser", split="eval", builds=False,
        crate=f"{GBR}/sactor_urlparser/assembled_idiomatic.rs", map_kind="sactor",
        map=f"{GBR}/sactor_urlparser/run4_result/driver.c__585b2fa3/translated_code_idiomatic/specs/function_name_map.json",
        c=[f"{GBR}/sactor_urlparser/run4_input/driver.c"]),
}


def leaf(n):
    return n.split("::")[-1]


def _git(*a):
    return subprocess.run(["git", "-C", ROOT] + list(a), capture_output=True, text=True).stdout.strip()


def _sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()[:16]


def tool_versions():
    return {
        "repo_head": _git("rev-parse", "--short", "HEAD"),
        "matcher_commit": _git("log", "-1", "--format=%h", "--", "tools/stu_selector/matcher.py"),
        "analyzer_src_commit": _git("log", "-1", "--format=%h", "--", "tools/stu_selector/analyzer/src"),
        "matcher_sha256": _sha(f"{STU}/matcher.py"),
        "c_analyzer_sha256": _sha(f"{STU}/c_analyzer.py"),
        "analyzer_bin_sha256": _sha(ANALYZER),
        "stu_selector_dirty": bool(_git("status", "--porcelain", "--", "tools/stu_selector")),
    }


def tree_hash(paths):
    h = hashlib.sha256()
    for p in sorted(paths):
        h.update(p.encode()); h.update(b"\0"); h.update(open(p, "rb").read()); h.update(b"\0")
    return h.hexdigest()[:16]


def rust_files(crate_dir):
    out = []
    for dp, dn, fn in os.walk(crate_dir):
        dn[:] = [d for d in dn if d not in ("target", ".git", "analysis_results")]
        out += [os.path.join(dp, f) for f in fn if f.endswith(".rs") or f == "Cargo.toml"]
    return out


def stage_crate(cfg, tmp):
    """Return an analyzable crate dir: a copy of the crate with driver dirs dropped, or a
    one-file crate wrapped around a single .rs output."""
    src = cfg["crate"]
    stage = f"{tmp}/crate"
    if os.path.isfile(src):
        os.makedirs(f"{stage}/src")
        shutil.copy(src, f"{stage}/src/lib.rs")
        open(f"{stage}/Cargo.toml", "w").write(
            '[package]\nname="stage"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="src/lib.rs"\n')
        return stage
    shutil.copytree(src, stage, symlinks=True,
                    ignore=shutil.ignore_patterns("target", ".git", "analysis_results"))
    for d in cfg.get("drop_dirs", []):
        shutil.rmtree(f"{stage}/{d}", ignore_errors=True)
    toml = open(f"{stage}/Cargo.toml").read()
    open(f"{stage}/Cargo.toml", "w").write(re.sub(r"\[\[bin\]\][^\[]*", "", toml))
    return stage


def analyze_c(srcs, workdir):
    entries = []
    for s in srcs:
        d = os.path.dirname(s)
        entries.append({"directory": d, "file": s,
                        "arguments": ["clang", "-c", s, f"-I{d}", "-I" + os.path.dirname(d), "-w"]})
    os.makedirs(workdir, exist_ok=True)
    json.dump(entries, open(f"{workdir}/compile_commands.json", "w"), indent=1)
    out = subprocess.run([sys.executable, f"{STU}/c_analyzer.py", "--compile-commands", workdir,
                          "--enable-metrics"], capture_output=True, text=True, check=True).stdout
    return json.loads(out)


def analyze_rust(crate):
    out = subprocess.run([ANALYZER, crate, "--enable-metrics"], capture_output=True, text=True,
                         check=True).stdout
    return json.loads(out)


def drop_rust_fns(r, names):
    """Remove driver functions (and everything nested in them) from the analyzer output."""
    def hit(n):
        parts = n.split("::")
        return any(p in names for p in parts)
    gone = {f["name"] for f in r["functions"] if hit(f["name"])}
    r["functions"] = [f for f in r["functions"] if f["name"] not in gone]
    r["raw_edges"] = [e for e in r.get("raw_edges", []) if e["from"] not in gone and e["to"] not in gone]
    return sorted(gone)


def c_function_files(srcs, cfn):
    """name -> (file, line). The C analyzer records the definition line but not the file;
    recover the file by finding which source has `name(` at (or within 2 lines of) it."""
    texts = {s: open(s, encoding="utf-8", errors="replace").read().split("\n") for s in srcs}
    loc = {}
    for name, f in cfn.items():
        ln = int(f.get("line") or 0)
        pat = re.compile(rf"\b{re.escape(name)}\s*($|[\(\)])")   # `)` for BZ_API(name) (...); $ for name-at-EOL
        for s, lines in texts.items():
            for j in range(max(1, ln - 2), min(len(lines), ln + 2) + 1):
                if pat.search(lines[j - 1]) and ";" not in lines[j - 1]:
                    loc[name] = (os.path.basename(s), ln)
                    break
            if name in loc:
                break
        if name not in loc:
            for s, lines in texts.items():
                for j, line in enumerate(lines, 1):
                    if pat.search(line) and ";" not in line and not line.lstrip().startswith(("//", "*", "/*")):
                        if re.match(r"^[\w\*\s\(]*\b" + re.escape(name) + r"\s*[\(\)]", line):
                            loc[name] = (os.path.basename(s), j)
                            break
                if name in loc:
                    break
    return loc


def rust_function_files(crate, names):
    loc = {}
    for path in glob.glob(f"{crate}/src/**/*.rs", recursive=True):
        rel = os.path.relpath(path, crate)
        for i, line in enumerate(open(path, encoding="utf-8", errors="replace"), 1):
            m = re.search(r"\bfn\s+([A-Za-z_]\w*)\s*[<(]", line)
            if m and m.group(1) in names:
                loc.setdefault(m.group(1), (rel, i))
    return loc


def sig_str(f):
    sig = f.get("signature") or {}
    ps = ", ".join(f"{p.get('name','_')}: {p.get('ty','?')}" for p in sig.get("params", []))
    ret = sig.get("ret") or sig.get("return") or ""
    return f"({ps}) -> {ret}" if ret else f"({ps})"


def load_ptrtrans_map(meta_path):
    """C function name -> dict(claim=[rust names], status, verifi, rust_path)."""
    m = {}
    for line in open(meta_path):
        d = json.loads(line)
        if d.get("map_tag") != "code" or d.get("source_code_type", "function") != "function":
            continue
        cid = d["source_c_code_id"][0]
        cname = cid.split("#")[0]
        trc = d.get("trans_rust_code") or ""
        rdn = d.get("rust_definition_name") or []
        if isinstance(rdn, str):
            rdn = [rdn] if rdn else []
        if trc == "Free_Function":
            status = "ELIDED"          # PtrTrans dropped it on purpose (free()/Drop)
        elif not trc:
            status = "EMPTY"           # record exists, no translation emitted
        elif not rdn:
            status = "UNNAMED"         # translation emitted, no definition name recorded
        else:
            status = "CLAIMED"
        m[cname] = dict(claim=rdn, status=status, verifi=d.get("Verfi_Tag", ""),
                        rust_path=d.get("trans_rust_path", ""), c_id=cid)
    return m


def load_sactor_map(path):
    """function_name_map.json: {c_name: rust_name}. SACTOR has no per-function verdict here;
    its verification is pass/fail for the whole run."""
    return {c: dict(claim=[r], status="CLAIMED", verifi="", rust_path="", c_id=c)
            for c, r in json.load(open(path)).items()}


def build(case, outdir):
    cfg = CASES[case]
    lib, tool = cfg["lib"], cfg["tool"]
    tmp = tempfile.mkdtemp(prefix=f"gbscaf_{case}_")
    try:
        c = analyze_c(cfg["c"], f"{tmp}/cbuild")
        stage = stage_crate(cfg, tmp)
        r = analyze_rust(stage)
        rloc = rust_function_files(stage, {leaf(f["name"]) for f in r["functions"]})
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    dropped_fns = drop_rust_fns(r, cfg.get("rust_exclude_fns", set()))
    if cfg["map_kind"] == "ptrtrans":
        tool_map = load_ptrtrans_map(cfg["map"])
    elif cfg["map_kind"] == "sactor":
        tool_map = load_sactor_map(cfg["map"])
    else:
        tool_map = {}

    versions = tool_versions()
    crate_files = ([cfg["crate"]] if os.path.isfile(cfg["crate"]) else rust_files(cfg["crate"]))
    fp = {**versions,
          "rust_artifact_sha256": tree_hash(crate_files),
          "c_sources_sha256": tree_hash(cfg["c"]),
          "tool_map_sha256": _sha(cfg["map"]) if cfg["map"] else None,
          "params": {"eps": EPS, "topo": True,
                     "rust_exclude_fns": sorted(cfg.get("rust_exclude_fns", set())),
                     "drop_dirs": cfg.get("drop_dirs", []),
                     "c_sources": [os.path.relpath(p, ROOT) for p in cfg["c"]]}}
    fp["id"] = hashlib.sha256(json.dumps(fp, sort_keys=True).encode()).hexdigest()[:16]

    cfn = {f["name"]: f for f in c["functions"]}
    rfn = {f["name"]: f for f in r["functions"]}
    # leaf -> [full names]; a leaf carried by more than one Rust function is ambiguous for
    # name-equality and for claim resolution (never silently pick one).
    rleaf = {}
    for n in rfn:
        rleaf.setdefault(leaf(n), []).append(n)
    dup_leafs = {k: v for k, v in rleaf.items() if len(v) > 1}
    cloc = c_function_files(cfg["c"], cfn)

    forced = matcher.match(c, r, topo=True)
    dep = matcher.match(c, r, topo=True, abstain_eps=EPS)
    fmap = {cc: (rr, s, k) for (cc, rr, s, k) in forced["matched"]}
    dmap = {cc: (rr, s, k) for (cc, rr, s, k) in dep["matched"]}
    damb = set()
    for a in dep.get("ambiguous", []):
        damb.add(a[0] if isinstance(a, (list, tuple)) else a)

    rows = []
    for cname in sorted(cfn, key=lambda n: (cloc.get(n, ("~", 0)), n)):
        t = tool_map.get(cname)
        if t is None:
            claim, status, verifi = "", ("NO_RECORD" if tool_map else "NO_MAP"), ""
        else:
            claim, status, verifi = ";".join(t["claim"]), t["status"], t["verifi"]
        claim_leafs = [leaf(x) for x in (t["claim"] if t else [])]
        # PtrTrans sometimes records a list mixing type names with the function name
        # (e.g. "BzFile;BZ2_bzWriteClose64;Bool;UChar;Int32"): resolve to the element(s)
        # that are actually Rust functions in the crate.
        claim_fns = [x for x in claim_leafs if x in rleaf]
        if not claim_leafs:
            claim_defined = ""
        elif not claim_fns:
            claim_defined = "NO"
        elif any(x in dup_leafs for x in claim_fns):
            claim_defined = "DUPLICATE"
        else:
            claim_defined = "yes"
        claim_resolved = claim_fns[0] if claim_fns and claim_defined == "yes" else ""
        if cname in dup_leafs:
            name_eq = "DUPLICATE"
        elif cname in rleaf:
            name_eq = cname
        else:
            name_eq = ""
        fr = fmap.get(cname)
        dr = dmap.get(cname)
        m_forced = leaf(fr[0]) if fr else ""
        m_dep = leaf(dr[0]) if dr else ("ABSTAIN" if cname in damb else "NONE")
        votes = {v for v in (claim_resolved, name_eq if name_eq != "DUPLICATE" else "", m_forced) if v}
        if status == "ELIDED":
            prior = "tool-says-elided"
        elif len(votes) == 1 and claim_resolved and name_eq and m_forced:
            prior = "unanimous"
        elif len(votes) == 1:
            prior = "single-source"
        else:
            prior = "CONFLICT"
        rows.append({
            "c_function": cname,
            "c_file": cloc.get(cname, ("", ""))[0],
            "c_line": cloc.get(cname, ("", ""))[1],
            "c_signature": sig_str(cfn[cname]),
            "tool_claim": claim,
            "tool_claim_resolved": claim_resolved,
            "tool_claim_status": status,
            "tool_claim_defined_in_rust": claim_defined,
            "tool_verifi_tag": verifi,
            "name_eq": name_eq,
            "matcher_forced": m_forced,
            "matcher_forced_score": round(fr[1], 3) if fr else "",
            "matcher_forced_conf": round(fr[2], 3) if fr and fr[2] is not None else "",
            "matcher_deploy": m_dep,
            "prior": prior,
            "truth": "",
            "truth_note": "",
        })

    # Rust inventory: who points at each Rust function.
    claimed_by, proposed_by = {}, {}
    for row in rows:
        if row["tool_claim_resolved"]:
            claimed_by.setdefault(row["tool_claim_resolved"], []).append(row["c_function"])
        if row["matcher_forced"]:
            proposed_by.setdefault(row["matcher_forced"], []).append(row["c_function"])
    inv = []
    for n in sorted(rfn, key=lambda n: (rloc.get(leaf(n), ("~", 0)), n)):
        ln = leaf(n)
        inv.append({
            "rust_function": n if ln in dup_leafs else ln,
            "file": rloc.get(ln, ("", ""))[0],
            "line": rloc.get(ln, ("", ""))[1],
            "signature": sig_str(rfn[n]),
            "same_name_in_c": "yes" if ln in cfn else "",
            "duplicate_leaf": "yes" if ln in dup_leafs else "",
            "tool_claimed_by": ";".join(claimed_by.get(ln, [])),
            "matcher_proposed_for": ";".join(proposed_by.get(ln, [])),
        })

    os.makedirs(outdir, exist_ok=True)
    with open(f"{outdir}/sheet.csv", "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        w.writeheader(); w.writerows(rows)
    with open(f"{outdir}/rust_inventory.csv", "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=list(inv[0].keys()))
        w.writeheader(); w.writerows(inv)
    raw = f"{RAW}/{case}"
    os.makedirs(raw, exist_ok=True)
    json.dump(c, open(f"{raw}/c_analyzer.json", "w"))
    json.dump(r, open(f"{raw}/rust_analyzer.json", "w"))
    json.dump({"forced": forced["matched"], "forced_ambiguous": forced.get("ambiguous", []),
               "deployment": dep["matched"], "deployment_ambiguous": dep.get("ambiguous", []),
               "config": {"topo": True, "abstain_eps": EPS}},
              open(f"{raw}/matcher_output.json", "w"), indent=1)

    # counts for the README
    st = {}
    for row in rows:
        st[row["tool_claim_status"]] = st.get(row["tool_claim_status"], 0) + 1
    vt = {}
    for t in tool_map.values():
        vt[t["verifi"] or "-"] = vt.get(t["verifi"] or "-", 0) + 1
    n_c, n_r = len(cfn), len(rfn)
    n_nameeq = sum(1 for row in rows if row["name_eq"] and row["name_eq"] != "DUPLICATE")
    n_claimed = sum(1 for row in rows if row["tool_claim_status"] == "CLAIMED")
    n_claim_missing = sum(1 for row in rows if row["tool_claim_defined_in_rust"] == "NO")
    n_claim_dup = sum(1 for row in rows if row["tool_claim_defined_in_rust"] == "DUPLICATE")
    n_claim_renamed = sum(1 for row in rows if row["tool_claim_resolved"] and
                          row["tool_claim_resolved"] != row["c_function"])
    n_abst = sum(1 for row in rows if row["matcher_deploy"] == "ABSTAIN")
    pri = {}
    for row in rows:
        pri[row["prior"]] = pri.get(row["prior"], 0) + 1
    n_local_edges = len({(e["from"], e["to"]) for e in r["raw_edges"]
                         if e["from"] != e["to"] and (e["to"] in rfn or leaf(e["to"]) in rleaf)})
    map_desc = {"ptrtrans": "PtrTrans `rust_definition_name` (per-function metadata jsonl)",
                "sactor": "SACTOR `specs/function_name_map.json`",
                None: "**none shipped / not retained** — `tool_claim` columns are empty"}[cfg["map_kind"]]

    readme = f"""# {tool} × {lib} — annotation scaffold (RQ1 group B, {cfg['split']})

Generated by `scripts/rq1_group_b_scaffold.py --case {case}`. **Nothing here is
labeled yet**: `truth` / `truth_note` in `sheet.csv` are blank and must be filled by hand.

## Artifact facts

| | |
|---|---|
| translator | **{tool}** |
| split | **{cfg['split']}** (`results/rq1_matching/SPLIT.md`) |
| crate builds | **{'yes' if cfg['builds'] else 'NO'}** (static matching does not need it; recorded so buildable / non-buildable are never pooled silently) |
| Rust artifact | `{os.path.relpath(cfg['crate'], ROOT)}` |
| tool map | {map_desc}{' — `' + os.path.relpath(cfg['map'], ROOT) + '`' if cfg['map'] else ''} |
| C reference | {', '.join('`' + os.path.relpath(p, ROOT) + '`' for p in cfg['c'])} |
| driver functions dropped on the Rust side | {', '.join('`' + d + '`' for d in dropped_fns) or 'none'} |
| driver dirs dropped | {', '.join('`' + d + '`' for d in cfg.get('drop_dirs', [])) or 'none'} |
| C functions | {n_c} |
| Rust functions (analyzer) | {n_r} |
| duplicate Rust leaf names | {len(dup_leafs)}{' — ' + ', '.join(sorted(dup_leafs)) if dup_leafs else ''} |
| unique local call edges | {n_local_edges} ({n_local_edges / max(n_r, 1):.2f} per fn) |
| fingerprint | `{fp['id']}` (matcher {versions['matcher_commit']}, analyzer {versions['analyzer_src_commit']}, repo {versions['repo_head']}; full block in `sheet.json`) |
| raw analyzer / matcher output | `{os.path.relpath(raw, ROOT)}/` |

Translator per-function verdict (own bookkeeping; PtrTrans `Verfi_Tag`, SACTOR none): {json.dumps(vt) if vt else 'n/a'}

## What the sheet contains

One row per C function (`sheet.csv`, {len(rows)} rows). Columns:

- `tool_claim` / `tool_claim_status` — the translator's shipped map. Status: `CLAIMED` (a Rust
  definition name is recorded), `ELIDED` (PtrTrans marked it `Free_Function` and emitted no
  translation — a deliberate drop, not a miss), `EMPTY` (record, no code), `UNNAMED` (code, no
  name), `NO_RECORD` (the C function does not appear in the map at all), `NO_MAP` (this
  artifact has no map; the tool-map audit does not apply).
- `tool_claim_resolved` — the element of `tool_claim` that is a Rust function defined in the
  crate (PtrTrans sometimes records a `;`-list mixing type names with the function name).
- `tool_claim_defined_in_rust` — `NO` = no element of the claim is a defined Rust function;
  the map points at nothing, which is a map defect regardless of truth. `DUPLICATE` = the
  claimed leaf name is defined more than once in the crate, so the claim is not resolvable by
  name; the annotator must pick by file:line (see `rust_inventory.csv`).
- `name_eq` — a Rust function with the identical name exists (name-equality baseline);
  `DUPLICATE` when that name is defined more than once (name-eq abstains).
- `matcher_forced` (+score, confidence) — the matcher's assignment when forced to answer.
- `matcher_deploy` — the matcher at eps={EPS}: a name, `ABSTAIN`, or `NONE`.
- `prior` — labeling aid only: `unanimous` = tool, name-eq and matcher agree;
  `single-source` = only one of them proposes, nothing contradicts; `CONFLICT` = they
  disagree; `tool-says-elided`. **Start labeling with CONFLICT rows**; unanimous rows still
  need a human eye but rarely flip.

Counts: tool status {json.dumps(st)}; name-eq available for {n_nameeq}/{n_c};
tool claims that are renames {n_claim_renamed}; tool claims pointing at an undefined Rust
name {n_claim_missing}; tool claims pointing at a duplicated name {n_claim_dup};
matcher abstains on {n_abst}; prior {json.dumps(pri)}.

`rust_inventory.csv` lists every Rust function with file:line and signature, and which C
functions the tool map and the matcher point at it — use it to find the right target when
the three sources disagree, and to spot Rust functions nobody points at (candidates for
merged / split / invented functions). Nested helper functions (`fn` inside `fn`) appear as
`outer::inner`.

## Labeling rules (fill `truth`)

- `truth` = the leaf name of the Rust function that implements this C function's behavior
  (`outer::inner` for a nested helper), decided by reading both bodies, **never by name
  similarity alone**.
- `NONE` if the C function has no Rust counterpart (dropped, or inlined into a caller —
  say which in `truth_note`).
- `SPLIT:a;b` if the C function's behavior is spread over several Rust functions;
  `MERGED:x` if several C functions collapsed into Rust function `x` (then every one of
  them gets `MERGED:x`).
- `AMBIGUOUS` only when two Rust functions are behaviorally interchangeable for this C
  function; write why in `truth_note`.
- A claimed name that is not defined in the crate can never be the truth.
- For a non-building crate a function whose `Verfi_Tag` says `Compile_Failed` still counts:
  the question is structural correspondence, not whether the Rust compiles.

## How the labels are consumed

Matcher P/R/coverage/abstention are computed against `truth` exactly as for group A.
Name-eq recall = rows whose `truth` equals the C name (real renamed pairs are the rows where
it does not — report both counts). The tool-map audit (only where a map exists) classifies
each CLAIMED row against `truth` as **correct** / **confirmed wrong** / **ambiguous** (truth
is `AMBIGUOUS`, or a SPLIT/MERGED case the map cannot express) / **missing-abstained** (tool
has no usable claim); ELIDED rows are reported separately, not as map errors.
"""
    open(f"{outdir}/README.md", "w").write(readme)
    json.dump({"case": case, "tool": tool, "lib": lib, "builds": cfg["builds"],
               "split": cfg["split"], "eps": EPS, "fingerprint": fp,
               "rust_functions_dropped": dropped_fns, "duplicate_leafs": dup_leafs,
               "rows": rows, "rust_inventory": inv, "verifi_tags": vt,
               "matcher_forced_raw": forced["matched"], "matcher_deploy_raw": dep["matched"],
               "matcher_deploy_ambiguous": dep.get("ambiguous", [])},
              open(f"{outdir}/sheet.json", "w"), indent=1)
    print(f"{case}: {n_c} C fns, {n_r} Rust fns (dropped {dropped_fns}), builds={cfg['builds']}, "
          f"dup-leafs={len(dup_leafs)}, prior={pri}, tool status={st}, "
          f"claims-undefined={n_claim_missing}, claims-dup={n_claim_dup}, "
          f"renamed-claims={n_claim_renamed}, name-eq={n_nameeq}, abstain={n_abst}, fp={fp['id']}")
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", choices=sorted(CASES))
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--outdir", default=None)
    a = ap.parse_args()
    cases = sorted(CASES) if a.all else [a.case]
    if not cases or cases == [None]:
        ap.error("--case or --all")
    for case in cases:
        build(case, a.outdir or f"{OUT}/{case}")


if __name__ == "__main__":
    main()
