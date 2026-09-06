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

LIB_MODULES = ["blocksort", "bzlib", "compress", "crctable", "decompress", "huffman", "randtable"]
MODULES = LIB_MODULES + ["bzip2"]
FN_DEF = re.compile(r'(?m)^\s*(?:#\[no_mangle\]\s*)?(?:pub\s+)?(?:unsafe\s+)?extern\s+"C"\s+fn\s+(\w+)')
FN_DEF2 = re.compile(r'(?m)^\s*pub\s+(?:unsafe\s+)?fn\s+(\w+)')
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

def main(src_dir, out_file):
    src = pathlib.Path(src_dir)
    defs, private, bodies = {}, set(), {}
    for m in MODULES:
        t = (src / f"{m}.rs").read_text()
        bodies[m] = t
        # strip extern "C" { ... } blocks first: c2rust marks declarations the same way it marks
        # definitions, and a declaration is not a definition.
        scan = re.sub(r'(?s)extern\s+"C"\s*\{.*?\n\}', '', t)
        if m not in LIB_MODULES:
            continue
        for pat in (FN_DEF, FN_DEF2):
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
    main(sys.argv[1], sys.argv[2])
