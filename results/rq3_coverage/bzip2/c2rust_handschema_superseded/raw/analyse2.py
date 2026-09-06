#!/usr/bin/env python3
"""RQ4 coverage — artifact-level analysis for one (library, tool) cell.

tests side : one llvm-cov export from the shipped acceptance suite run against the translated
             artifact.
ours side  : one llvm-cov export PER differential harness.  The harnesses are UNIONED at the level
             of function and region IDENTITY — never by summing or averaging per-harness numbers.

Both sides are built from the same flattened crate, so identities already share one coordinate
system; `linemap.json` maps a flattened line back to (original library file, original line) so the
scope filter and the reported names are in the artifact's own terms.

Denominator: the tests-side build carries `-C link-dead-code`, so its in-scope identity set is the
complete one and is used as the universe.  Ours-side identities that are NOT in that universe are
counted and reported rather than silently added -- they come from the generator's --expose-entry,
which rewrites one signature line per harness and shifts the columns of the regions on it.

usage: analyse2.py <linemap.json> <tests_coverage.json> <ours_dir> <out_dir>
"""
import difflib, json, os, sys
import pathlib
from pathlib import Path

def load_map(p):
    m = json.load(open(p))
    return m["scored_modules"], m["ranges"]

def remap(line, ranges, scored):
    for mod in scored:
        first, last = ranges[mod]
        if first <= line <= last:
            return f"{mod}.rs", line - first + 1
    return None, None

def is_lib(fname):
    """The translated crate's own src/lib.rs, and nothing else.

    Matching on the basename alone also matched dependencies' src/lib.rs (libfuzzer-sys ships an
    instrumented one into every ours-side export); their line numbers were then remapped into
    bzip2 modules and polluted the identity sets."""
    return (os.path.basename(fname) == "lib.rs"
            and "/.cargo/" not in fname and "/registry/" not in fname
            and "/rustlib/" not in fname and "/rustc" not in fname)

def align_to_canonical(lib_path, canon_lines):
    """1-based line map from a harness's own src/lib.rs to the canonical flattened file.

    The generator's --expose-entry inserts `#[no_mangle]` as its OWN line before the exposed
    entry (`\\1#[no_mangle]\\n\\1pub \\2`), so every later line of that harness's lib.rs is shifted
    by one against the canonical file, and the driver appends a re-export at the end.  Without
    this alignment every function after the exposure point is attributed to the wrong line.
    Equal blocks map straight through; a same-size `replace` block (the rewritten signature line)
    maps line-for-line, its columns having moved -- regions on it fall outside the universe and
    are reported, never added."""
    lines = pathlib.Path(lib_path).read_text().split("\n")
    sm = difflib.SequenceMatcher(None, lines, canon_lines, autojunk=False)
    m = {}
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal" or (i2 - i1) == (j2 - j1):
            for k in range(i2 - i1):
                m[i1 + k + 1] = j1 + k + 1
        elif j2 > j1:
            # --expose-entry turns ONE canonical line into two (`#[no_mangle]` on its own line
            # plus the `pub `-prefixed signature); difflib folds that into a single 1->2 block.
            # Both harness lines belong to the same canonical line, so clamp into [j1, j2-1].
            for k in range(i2 - i1):
                m[i1 + k + 1] = j1 + min(k, j2 - j1 - 1) + 1
        # a block with no canonical counterpart (the two lines appended by the driver) maps to
        # nothing and is skipped
    return m


def extract(export_path, ranges, scored, canon_lines=None):
    """-> funcs {(file, name, line): covered}, regions {(file,l1,c1,l2,c2): covered}, harness_meta"""
    data = json.load(open(export_path))["data"][0]
    funcs, regions, names = {}, {}, {}
    lm = None
    if canon_lines is not None:
        for fdef in data["functions"]:
            for n in fdef["filenames"]:
                if is_lib(n):
                    lm = align_to_canonical(n, canon_lines)
                    break
            if lm:
                break
    def canon(l):
        return l if lm is None else lm.get(l)
    for fdef in data["functions"]:
        files = fdef["filenames"]
        if not files or not is_lib(files[0]):
            continue
        home_regions = [r for r in fdef["regions"] if r[5] == 0]
        if not home_regions:
            continue
        start = canon(min(r[0] for r in home_regions))
        if start is None:
            continue
        f0, l0 = remap(start, ranges, scored)
        if f0 is None:
            continue                      # in lib.rs but outside the seven scored modules
        # Identity is (file, start line), NOT the symbol name: the two sides are built by
        # different cargo invocations, so the Rust v0 mangling embeds a different crate
        # disambiguator hash and the same function has different symbol names on each side.
        # Two functions cannot start on the same line of the same file, so (file, line) is exact.
        fid = (f0, l0)
        funcs[fid] = funcs.get(fid, False) or (fdef["count"] > 0)
        names.setdefault(fid, fdef["name"])
        for r in fdef["regions"]:
            l1, c1, l2, c2, count, fileid = r[0], r[1], r[2], r[3], r[4], r[5]
            if fileid >= len(files) or not is_lib(files[fileid]):
                continue
            cl1, cl2 = canon(l1), canon(l2)
            if cl1 is None:
                continue
            rf, rl1 = remap(cl1, ranges, scored)
            if rf is None:
                continue
            _, rl2 = remap(cl2, ranges, scored) if cl2 is not None else (None, None)
            rid = (rf, rl1, c1, rl2 if rl2 else rl1, c2)
            regions[rid] = regions.get(rid, False) or (count > 0)
    return funcs, regions, names

def gate_stats(export_path, entry, corpus_n):
    """Inputs replayed vs inputs that actually reached Rust.

    `cargo fuzz coverage` replays each archived corpus input exactly once, so the number of inputs
    replayed is the corpus size, and the execution count of the RUST ENTRY FUNCTION is the number
    of those inputs that passed the C-side UB gate and went on to execute Rust.  The difference is
    what the gate excluded.

    (An earlier version tried to read the gate's own `return` arm out of the fuzz target's
    coverage; libfuzzer's `fuzz_target!` closure has no separate coverage record there, so that
    heuristic was silently reading the byte-cursor helpers instead and always reported 0.)"""
    data = json.load(open(export_path))["data"][0]
    reached = None
    for fdef in data["functions"]:
        if not fdef["filenames"] or not is_lib(fdef["filenames"][0]):
            continue
        if demangle(fdef["name"]) == entry:
            reached = max(reached or 0, fdef["count"])
    return {"inputs_replayed": corpus_n, "reached_rust": reached,
            "ub_gate_excluded": (corpus_n - reached) if reached is not None else None}


def demangle(sym):
    """Recover the plain name from a Rust v0 symbol; c2rust #[no_mangle] names pass through."""
    import re
    m = re.match(r"^_R.*?(\d+)([A-Za-z_][A-Za-z0-9_]*)(?:C[a-zA-Z0-9_]+)?$", sym)
    if m:
        return m.group(2)
    m = re.findall(r"(\d+)([A-Za-z_][A-Za-z0-9_]*)", sym)
    return m[-1][1] if sym.startswith("_R") and m else sym


def sets(d):
    return set(d), {k for k, v in d.items() if v}

def main(linemap, tests_json, ours_dir, out_dir, corpus_root=None):
    scored, ranges = load_map(linemap)
    out_dir = Path(out_dir); out_dir.mkdir(parents=True, exist_ok=True)

    canon_lines = pathlib.Path(str(linemap).replace(".linemap.json", "")).read_text().split("\n")
    tf, tr, tnames = extract(tests_json, ranges, scored, canon_lines)
    uni_f, cov_tf = sets(tf)
    uni_r, cov_tr = sets(tr)

    per_harness, cov_of, cov_or = [], set(), set()
    dropped_f = dropped_r = 0
    for jp in sorted(Path(ours_dir).glob("*.json")):
        of, orr, _ = extract(jp, ranges, scored, canon_lines)
        _, cf = sets(of); _, cr = sets(orr)
        dropped_f += len(cf - uni_f); dropped_r += len(cr - uni_r)
        cf &= uni_f; cr &= uni_r
        cov_of |= cf; cov_or |= cr
        n = len(list((Path(corpus_root)/jp.stem).iterdir())) if corpus_root else None
        per_harness.append({"harness": jp.stem, "functions_covered": len(cf),
                            "regions_covered": len(cr), **gate_stats(jp, jp.stem, n)})

    res = {"scope_files": scored, "harnesses_unioned": len(per_harness),
           "per_harness": per_harness,
           "ours_identities_outside_universe": {"functions": dropped_f, "regions": dropped_r,
               "note": "--expose-entry rewrites one signature line per harness, shifting the "
                       "columns of the regions on it; such identities are excluded, never added"}}

    for label, uni, ct, co in (("function", uni_f, cov_tf, cov_of),
                               ("region", uni_r, cov_tr, cov_or)):
        both, only_t, only_o = ct & co, ct - co, co - ct
        total = len(uni)
        res[label] = {"total_in_scope": total, "covered_tests": len(ct), "covered_ours": len(co),
                      "covered_both": len(both), "only_tests": len(only_t),
                      "only_ours": len(only_o), "union": len(ct | co),
                      "covered_by_neither": total - len(ct | co),
                      "tests_coverage": round(len(ct)/total, 6),
                      "ours_coverage": round(len(co)/total, 6),
                      "growth": round((len(co)-len(ct))/total, 6),
                      "sanity": {
                          "both_plus_only_tests_eq_covered_tests": len(both)+len(only_t) == len(ct),
                          "both_plus_only_ours_eq_covered_ours": len(both)+len(only_o) == len(co),
                          "both_plus_onlys_eq_union": len(both)+len(only_t)+len(only_o) == len(ct | co),
                          "all_reported_in_scope": all(i[0] in [f"{m}.rs" for m in scored] for i in uni),
                          "covered_le_denominator": len(ct) <= total and len(co) <= total}}
        if label == "function":
            for nm, s in (("covered_by_both.txt", both), ("only_tests.txt", only_t),
                          ("only_ours.txt", only_o), ("covered_by_neither.txt", uni-(ct | co))):
                (out_dir/nm).write_text("".join(
                    f"{a}\tline {b}\t{demangle(tnames.get((a, b), '?'))}\n" for a, b in sorted(s)))

    json.dump(res, open(out_dir/"result.json", "w"), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k != "per_harness"}, indent=1))
    print("\nper harness:")
    for h in per_harness:
        print(f"  {h['harness']:28s} fn {h['functions_covered']:3d}  reg {h['regions_covered']:5d}  "
              f"replayed {h['inputs_replayed']}  reached-Rust {h['reached_rust']}  "
              f"gate-excluded {h['ub_gate_excluded']}")

if __name__ == "__main__":
    main(*sys.argv[1:6])
