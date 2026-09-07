#!/usr/bin/env python3
"""Build one RQ4 pair directory the way the first nineteen were built by hand, in one command,
recording where every byte came from.

  benchmark/pairs/rq4/<lib>_<tool>/
    source/                 the C the translator consumed (files and directories, copied verbatim)
    source/<main>           the translation unit the planner parses (an existing file, or an
                            amalgamation of #includes written here with --amalgamate)
    build/compile_commands.json   one entry, absolute paths, `clang -c -O1 -I source <cflags>`
    translated/<lib>_<tool>.rs (+ .defs.json, .linemap.json)   scripts/flatten_translation.py
    translated/renames.json (optional)   C name -> Rust name, from the RQ1 labels
    drivers/                (optional)   fixtures the tests side needs, copied verbatim
    PROVENANCE.json         sha256 of every input, the paths they were copied from, and the notes

Rust input (--rust): a crate `src/` directory (module files at one or two levels, with its
lib.rs) or a single .rs file (staged as one module named --module). --split-root-mods handles a
single-file crate whose root carries support modules used through `crate::` paths (Laertes'
`laertes_rt` / `__laertes_array`): those blocks are moved verbatim into a synthesized lib.rs so
the flatten carries them at the crate root; the module file is the original minus those blocks
and nothing else is touched. --rust-extra adds a file as a named module (a driver kept beside the
library, e.g. `test=out/quadtree/test.rs`).

usage:
  scripts/rq4/make_pair.py --lib qsort --tool laertes \\
      --c-src .../qsort/qsort.c --main qsort.c \\
      --rust .../qsort_laertes/qsort.rs --module qsort --split-root-mods \\
      --note "..."
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import flatten_translation as FT  # noqa: E402

PAIRS = ROOT / "benchmark" / "pairs" / "rq4"
ROOT_MOD = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{')


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


CRATE_ATTR = re.compile(r'^\s*#!\[(?:feature|register_tool|no_std|no_main|crate_type|crate_name)\b')


def strip_crate_attrs(text: str) -> tuple[str, int]:
    """A single-file translation is a crate ROOT: its `#![feature(..)]` / `#![register_tool]`
    lines are crate-level and are rejected inside `pub mod <m> { .. }`. The flatten's own header
    carries the features every translation needs; `#![allow(..)]` stays (legal in a module)."""
    keep, n = [], 0
    for ln in text.split("\n"):
        if CRATE_ATTR.match(ln):
            n += 1
            continue
        keep.append(ln)
    return "\n".join(keep), n


def split_root_mods(text: str) -> tuple[str, str]:
    """(module text without root-level `mod X { .. }` blocks, those blocks verbatim)."""
    out, carried, taking, depth = [], [], False, 0
    for ln in text.split("\n"):
        if not taking and ROOT_MOD.match(ln):
            taking, depth = True, 0
            # the block's own outer attributes (`#[allow(dead_code)]` above `pub(crate) mod`) go with it
            while out and re.match(r'^\s*#\[', out[-1]):
                carried.append(out.pop())
        if taking:
            carried.append(ln)
            depth += ln.count("{") - ln.count("}")
            if depth <= 0:
                taking = False
        else:
            out.append(ln)
    return "\n".join(out), "\n".join(carried)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lib", required=True)
    ap.add_argument("--tool", required=True)
    ap.add_argument("--c-src", nargs="+", required=True, help="files or directories copied into source/")
    ap.add_argument("--main", required=True, help="the TU the planner parses (name inside source/)")
    ap.add_argument("--amalgamate", help="comma-separated files (relative to source/) to #include, "
                                         "written as source/<main>")
    ap.add_argument("--amalgamate-text", help="verbatim text for source/<main> instead (macro-scoped renames)")
    ap.add_argument("--cflags", default="", help="extra clang flags (e.g. -std=c99 -DFOO)")
    ap.add_argument("--tus", help="comma-separated translation units (relative to source/) compiled "
                    "SEPARATELY, as the library's own build does (optipng: 52 TUs whose private zlib headers "
                    "have no include guards, so no amalgamation is possible). --main is then a stub the "
                    "generator copies; compile_commands.json gets one entry per TU with --cflags and "
                    "-I source/<dir> for every directory")
    ap.add_argument("--rust", required=True, help="crate src dir or a single .rs file")
    ap.add_argument("--module", help="module name for a single .rs file (default: the file stem)")
    ap.add_argument("--rust-extra", action="append", default=[], help="name=path, staged as <name>.rs")
    ap.add_argument("--lib-modules", help="override: comma-separated modules to SCORE")
    ap.add_argument("--extra-modules", help="override: comma-separated modules inlined, unscored")
    ap.add_argument("--split-root-mods", action="store_true")
    ap.add_argument("--namespace", help="the crate's wrapper module when the flatten cannot detect it")
    ap.add_argument("--root-glob", action="store_true", help="glob re-export the scored module(s) at the root "
                    "(a single-file crate that addresses its own items as crate::<item>)")
    ap.add_argument("--renames", help='JSON {"C name": "Rust name"}')
    ap.add_argument("--renames-source", default="", help="where the map comes from (recorded)")
    ap.add_argument("--drivers", help="directory copied to drivers/")
    ap.add_argument("--note", action="append", default=[])
    ap.add_argument("--force", action="store_true")
    a = ap.parse_args()

    pair = PAIRS / f"{a.lib}_{a.tool}"
    if pair.exists():
        if not a.force:
            sys.exit(f"{pair} exists (use --force to rebuild)")
        shutil.rmtree(pair)
    (pair / "source").mkdir(parents=True)
    (pair / "build").mkdir()
    (pair / "translated").mkdir()
    prov = {"lib": a.lib, "tool": a.tool, "source": {}, "rust": {}, "notes": list(a.note)}

    for s in a.c_src:
        sp = Path(s)
        if sp.is_dir():
            shutil.copytree(sp, pair / "source" / sp.name)
            for f in sorted(p for p in (pair / "source" / sp.name).rglob("*") if p.is_file()):
                prov["source"][str(f.relative_to(pair))] = {"from": str(sp / f.relative_to(pair / "source" / sp.name)),
                                                            "sha256": sha(f)}
        else:
            shutil.copy(sp, pair / "source" / sp.name)
            prov["source"][f"source/{sp.name}"] = {"from": str(sp), "sha256": sha(sp)}
    if a.amalgamate_text:
        (pair / "source" / a.main).write_text(a.amalgamate_text)
        prov["source"][f"source/{a.main}"] = {"from": "amalgamation (verbatim text, see notes)",
                                              "sha256": sha(pair / "source" / a.main)}
    elif a.amalgamate:
        files = [f.strip() for f in a.amalgamate.split(",") if f.strip()]
        for f in files:
            assert (pair / "source" / f).exists(), f"amalgamation member missing: {f}"
        text = f"/* {a.lib}: one translation unit over the files the translator consumed (RQ4 pair). */\n" + \
               "".join(f'#include "{f}"\n' for f in files)
        (pair / "source" / a.main).write_text(text)
        prov["source"][f"source/{a.main}"] = {"from": f"amalgamation of {files}", "sha256": sha(pair / "source" / a.main)}
    src_abs = str((pair / "source").resolve())
    if a.tus:
        tus = [x.strip() for x in a.tus.split(",") if x.strip()]
        for f in tus:
            assert (pair / "source" / f).exists(), f"TU missing: {f}"
        if not (pair / "source" / a.main).exists():
            (pair / "source" / a.main).write_text(
                f"/* {a.lib}: multi-TU pair -- the library is compiled as {len(tus)} translation units "
                f"(build/compile_commands.json), exactly as its own build does. This file is the "
                f"generator's nominal main unit and defines nothing. */\n")
        incs = []
        for d in sorted({str(Path(f).parent) for f in tus} | {"."}):
            incs += ["-I", src_abs if d == "." else f"{src_abs}/{d}"]
        cc = [{"directory": src_abs, "file": a.main,
               "arguments": ["clang", "-c", "-O1"] + incs + a.cflags.split() + [a.main, "-o", "main_stub.o"]}]
        for f in tus:
            cc.append({"directory": src_abs, "file": f,
                       "arguments": ["clang", "-c", "-O1"] + incs + a.cflags.split() + [f, "-o", f.replace("/", "_")[:-2] + ".o"]})
        (pair / "build" / "compile_commands.json").write_text(json.dumps(cc, indent=1))
        prov["source"]["_translation_units"] = tus
    else:
        assert (pair / "source" / a.main).exists(), f"main TU missing: {a.main}"
        args = ["clang", "-c", "-O1", "-I", src_abs] + a.cflags.split() + [a.main, "-o", Path(a.main).stem + ".o"]
        (pair / "build" / "compile_commands.json").write_text(json.dumps(
            [{"directory": src_abs, "file": a.main, "arguments": args}], indent=1))

    # ---- Rust: stage a flat module directory for the flatten
    rp = Path(a.rust)
    with tempfile.TemporaryDirectory() as td:
        st = Path(td)
        if rp.is_dir():
            shutil.copytree(rp, st, dirs_exist_ok=True)
            for f in sorted(p for p in rp.rglob("*.rs") if "target" not in p.parts):
                prov["rust"][str(f.relative_to(rp))] = {"from": str(f), "sha256": sha(f)}
        else:
            mod = a.module or rp.stem
            text = rp.read_text()
            prov["rust"][f"{mod}.rs"] = {"from": str(rp), "sha256": sha(rp)}
            if a.split_root_mods:
                body, carried = split_root_mods(text)
                (st / "lib.rs").write_text(carried + "\n")
                prov["rust"][f"{mod}.rs"]["split_root_mods"] = \
                    f"{carried.count(chr(10))} lines of root `mod` blocks moved to lib.rs verbatim"
                text = body
            text, n = strip_crate_attrs(text)
            if n:
                prov["rust"][f"{mod}.rs"]["crate_attrs_removed"] = n
            (st / f"{mod}.rs").write_text(text)
        for spec in a.rust_extra:
            name, _, path = spec.partition("=")
            text, n = strip_crate_attrs(Path(path).read_text())
            (st / f"{name}.rs").write_text(text)
            prov["rust"][f"{name}.rs"] = {"from": path, "sha256": sha(Path(path)), "crate_attrs_removed": n}
        # module files copied from a crate directory that were themselves transpiled as roots
        # (c2rust's per-file output for quadtree carries `#![feature(raw_ref_op)]` in src/*.rs)
        for f in sorted(st.rglob("*.rs")):
            if f.name == "lib.rs" or f.parent.name in ("bin", "target"):
                continue
            text, n = strip_crate_attrs(f.read_text())
            if n:
                f.write_text(text)
                prov["rust"].setdefault(str(f.relative_to(st)), {})["crate_attrs_removed"] = n
        out = pair / "translated" / f"{a.lib}_{a.tool}.rs"
        if a.lib_modules:
            lib = a.lib_modules.split(",")
            extra = a.extra_modules.split(",") if a.extra_modules else []
        else:
            lib, extra = FT.auto_modules(st)
        prov["rust"]["modules"] = {"scored": lib, "unscored": extra}
        FT.main(st, out, lib, extra, namespace=a.namespace, root_glob=a.root_glob)

    if a.renames:
        m = json.loads(a.renames)
        (pair / "translated" / "renames.json").write_text(json.dumps(m, indent=1) + "\n")
        prov["renames"] = {"map": m, "source": a.renames_source}
    if a.drivers:
        shutil.copytree(a.drivers, pair / "drivers")
        prov["drivers"] = {"from": a.drivers}
    for f in sorted((pair / "translated").glob("*")):
        prov["rust"][f"translated/{f.name}"] = {"sha256": sha(f)}
    (pair / "PROVENANCE.json").write_text(json.dumps(prov, indent=1) + "\n")
    print(f"pair {pair}: main TU {a.main}; scored modules {lib}; unscored {extra}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
