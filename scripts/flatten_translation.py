#!/usr/bin/env python3
"""Flatten the multi-file c2rust bzip2 translation into the single lib.rs the frozen
differential-harness generator expects, and emit the line map that undoes the flattening.

The generator (tools/stu_selector/gen_diff_harness.py) takes `pair/translated/*.rs` and writes it
verbatim as the harness crate's src/lib.rs, and the generated fuzz target calls
`translated::<entry>` at the crate ROOT.  The translation is nine module files whose bodies use
`crate::<module>::<item>` paths, so:

  * each module file is inlined verbatim inside `pub mod <name> { ... }` — the crate-absolute
    paths keep resolving and not one byte of translated code changes;
  * a `pub use` line per *public* translated function re-exports it at the crate root so
    `translated::<fn>` resolves.  C `static` functions are non-`pub` in the translation; the
    generator's own `--expose-entry` makes the targeted one public and the driver appends its
    re-export then.
  * `bzip2` (the translated CLI) is inlined too, because bzlib.rs takes eleven type aliases from
    it (`crate::bzip2::BZFILE`, `Char`, the darwin `_Rune*` types).  It is NOT in the coverage
    scope — only the seven library modules are scored.

`<out>.linemap.json` maps a 1-based line of the flattened file back to (original file, original
line), so coverage measured on the flattened crate can be compared, identity for identity, with
coverage measured on the ordinary multi-file crate that the shipped test suite drives.
"""
import re, sys, json, pathlib

# The library modules that are SCORED, and the extra modules that must be inlined so the crate
# still compiles but are deliberately not scored (a translated CLI, a transpiled test driver).
# Defaults are bzip2's; --lib-modules / --extra-modules override, and --auto derives them from the
# directory, treating anything named like a driver as extra.
LIB_MODULES = ["blocksort", "bzlib", "compress", "crctable", "decompress", "huffman", "randtable"]
EXTRA_MODULES = ["bzip2"]
MODULES = LIB_MODULES + EXTRA_MODULES
_DRIVER_NAMES = {"main", "test", "smoke", "sample", "fuzzer", "bzip2", "bzip2recover",
                 "example1", "example2", "example3", "example4", "benchmark", "pngtest",
                 "minigzip", "example"}
FN_DEF = re.compile(r'(?m)^\s*(?:#\[no_mangle\]\s*)?(?:pub\s+)?(?:unsafe\s+)?extern\s+"C"\s+fn\s+(\w+)')
FN_DEF2 = re.compile(r'(?m)^\s*pub\s+(?:unsafe\s+)?fn\s+(\w+)')
# An idiomatic rewriter may emit a plain, private, non-extern `fn` at module level (C2SaferRust
# turns bzip2's `static __inline__ UChar mmed3(..)` into `fn mmed3(a: u8, ..) -> u8`). Without
# this the function is not seen as defined at all, so it gets neither --expose-entry nor a root
# re-export, and the harness fails with `cannot find function ... in crate translated`.
FN_DEF3 = re.compile(r'(?m)^[ \t]{0,2}(?:unsafe\s+)?fn\s+(\w+)')  # module level may be indented one space
PUB_DEF = r'(?m)^\s*(?:#\[no_mangle\]\s*)?pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{}\b'

HEADER = [
    "// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.",
    "// Module bodies are copied byte-for-byte from",
    "// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).",
    "// Only the module wrappers and the root re-exports below are added.",
    "#![feature(core_intrinsics)]",
    "#![feature(extern_types)]",
    "#![feature(linkage)]",
    "#![feature(c_variadic)]",
    "#![feature(register_tool)]",
    "#![register_tool(c2rust)]",
    "#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,",
    "         non_upper_case_globals, unused_assignments, unused_mut, internal_features,",
    "         unused_imports, unpredictable_function_pointer_comparisons)]",
    "",
]

def auto_modules(src: pathlib.Path):
    """Every module in the crate, split into scored library modules and unscored drivers."""
    names = sorted(p.stem for p in src.glob("*.rs") if p.stem != "lib")
    lib = [n for n in names if n not in _DRIVER_NAMES]
    extra = [n for n in names if n in _DRIVER_NAMES]
    return lib, extra


def main(src_dir, out_file, lib_modules=None, extra_modules=None):
    global LIB_MODULES, MODULES
    src = pathlib.Path(src_dir)
    if lib_modules is not None:
        LIB_MODULES = list(lib_modules)
        MODULES = LIB_MODULES + list(extra_modules or [])
    defs, private, bodies = {}, set(), {}
    for m in MODULES:
        t = (src / f"{m}.rs").read_text()
        bodies[m] = t
        # strip extern "C" { ... } blocks first: c2rust marks declarations the same way it marks
        # definitions, and a declaration is not a definition.
        scan = re.sub(r'(?s)extern\s+"C"\s*\{.*?\n\}', '', t)
        if m not in LIB_MODULES:
            continue
        for pat in (FN_DEF, FN_DEF2, FN_DEF3):
            for name in pat.findall(scan):
                if name not in defs:
                    defs[name] = m
                    if not re.search(PUB_DEF.format(re.escape(name)), scan):
                        private.add(name)

    lines = list(HEADER)
    ranges = {}          # module -> [first_line, last_line] in the flattened file (1-based)
    for m in MODULES:
        lines.append(f"pub mod {m} {{")
        first = len(lines) + 1
        body = bodies[m].split("\n")
        lines.extend(body)
        ranges[m] = [first, len(lines)]
        lines.append("}")
        lines.append("")

    lines.append("// root re-exports so the generated harness's `translated::<entry>` resolves")
    for name in sorted(defs):
        if name in private:
            lines.append(f"// {name}: private in {defs[name]} — exposed per-entry by --expose-entry")
        else:
            lines.append(f"pub use crate::{defs[name]}::{name};")
    # Some translators put SUPPORT MODULES in the crate root that the translated modules import
    # (Laertes ships `laertes_rt` and `__laertes_array`, used by `use crate::laertes_rt::*;` in
    # every module). Carry over inline `mod` blocks from the original root -- and only those:
    # free items at the root are not carried, because c2rust's own root defines #[no_mangle]
    # stand-ins for the macOS libc symbols and those would collide with the shims we link.
    root = src / "lib.rs"
    if root.exists():
        rt = root.read_text().split("\n")
        carried, depth, taking = [], 0, False
        for ln in rt:
            if not taking and re.match(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*\{', ln):
                taking, depth = True, 0
            if taking:
                carried.append(ln)
                depth += ln.count("{") - ln.count("}")
                if depth <= 0:
                    taking = False
        if carried:
            lines.append("// support modules carried verbatim from the translation's own crate root")
            lines.extend(carried)
            lines.append("")

    lines.append("")

    out = pathlib.Path(out_file)
    out.write_text("\n".join(lines))
    json.dump({"defs": defs, "private": sorted(private)},
              open(str(out) + ".defs.json", "w"), indent=1, sort_keys=True)
    # flattened line L in [first, last] of module m  ->  (m + ".rs", L - first + 1)
    json.dump({"scored_modules": LIB_MODULES, "all_modules": MODULES, "ranges": ranges,
               "rule": "flattened line L in ranges[m]=[first,last] maps to (m + '.rs', L - first + 1); "
                       "columns are unchanged because module bodies are copied verbatim"},
              open(str(out) + ".linemap.json", "w"), indent=1)

    # verify the map on every line of every scored module
    flat = out.read_text().split("\n")
    bad = 0
    for m in LIB_MODULES:
        first, last = ranges[m]
        orig = bodies[m].split("\n")
        for L in range(first, last + 1):
            if flat[L - 1] != orig[L - first]:
                bad += 1
    print(f"wrote {out}: {len(defs)} library functions, {len(private)} private; "
          f"{len(MODULES)} modules inlined; line-map mismatches: {bad}")
    if bad:
        raise SystemExit("line map does not round-trip")

if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("src_dir"); ap.add_argument("out_file")
    ap.add_argument("--lib-modules", help="comma-separated modules to SCORE")
    ap.add_argument("--extra-modules", default="",
                    help="comma-separated modules to inline but NOT score (CLI, test drivers)")
    ap.add_argument("--auto", action="store_true",
                    help="derive both lists from the directory; anything named like a driver "
                         "(main/test/smoke/example*/fuzzer/...) becomes unscored")
    a = ap.parse_args()
    if a.auto:
        lm, em = auto_modules(pathlib.Path(a.src_dir))
        print(f"auto: scoring {lm}; inlining unscored {em}")
    elif a.lib_modules:
        lm = [s.strip() for s in a.lib_modules.split(",") if s.strip()]
        em = [s.strip() for s in a.extra_modules.split(",") if s.strip()]
    else:
        lm = em = None
    main(a.src_dir, a.out_file, lm, em)
