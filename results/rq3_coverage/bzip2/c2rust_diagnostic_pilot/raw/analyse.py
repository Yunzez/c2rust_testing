#!/usr/bin/env python3
"""RQ4 coverage — set analysis for one (library, tool) artifact.

Reads scope.json, the two llvm-cov export JSONs, and emits:
  covered_by_both.txt / only_tests.txt / only_ours.txt  (function identities)
  result.json with the function- and region-level counts of stage G and its sanity checks.

Function identity  = (source path relative to the crate src/, qualified function name, start line)
Region identity    = (source path, start line, start col, end line, end col)

Neither side's numbers are averaged over harnesses: each side is one merged profile.
"""
import json, os, sys, collections

def load(path):
    with open(path) as f:
        return json.load(f)

def relpath(fn, srcroot):
    fn = os.path.realpath(fn)
    srcroot = os.path.realpath(srcroot)
    return os.path.relpath(fn, srcroot)

def extract(export_path, scope, srcroot):
    """-> (functions: {ident: covered}, regions: {ident: covered}, skipped_files: set)"""
    d = load(export_path)
    assert len(d["data"]) == 1, "expected a single coverage export object"
    data = d["data"][0]
    funcs, regions = {}, {}
    seen_files = set()
    for fdef in data["functions"]:
        files = [relpath(x, srcroot) for x in fdef["filenames"]]
        seen_files.update(files)
        # a function belongs to the file its first region starts in
        home = files[0]
        if home not in scope:
            continue
        start_line = min(r[0] for r in fdef["regions"] if r[5] == 0) if fdef["regions"] else 0
        fid = (home, fdef["name"], start_line)
        covered = fdef["count"] > 0
        # llvm-cov can emit a function more than once (inlined copies); OR the coverage
        funcs[fid] = funcs.get(fid, False) or covered
        for r in fdef["regions"]:
            l1, c1, l2, c2, count, fileid = r[0], r[1], r[2], r[3], r[4], r[5]
            rf = files[fileid] if fileid < len(files) else home
            if rf not in scope:
                continue
            rid = (rf, l1, c1, l2, c2)
            regions[rid] = regions.get(rid, False) or (count > 0)
    return funcs, regions, seen_files

def sets(d):
    return set(d), {k for k, v in d.items() if v}

def main(cell_dir):
    scope_cfg = load(os.path.join(cell_dir, "scope.json"))
    scope = set(scope_cfg["include_files"])
    srcroot = scope_cfg["src_root"]

    tf, tr, tseen = extract(os.path.join(cell_dir, "tests_coverage.json"), scope, srcroot)
    of, orr, oseen = extract(os.path.join(cell_dir, "ours_coverage.json"), scope, srcroot)

    out = {"cell": scope_cfg["cell"], "scope_files": sorted(scope)}
    problems = []

    for label, T, O in (("function", tf, of), ("region", tr, orr)):
        all_t, cov_t = sets(T)
        all_o, cov_o = sets(O)
        if all_t != all_o:
            problems.append(
                f"{label} denominators differ between the two builds: "
                f"tests-only {len(all_t - all_o)}, ours-only {len(all_o - all_t)}")
        universe = all_t | all_o
        both = cov_t & cov_o
        only_t = cov_t - cov_o
        only_o = cov_o - cov_t
        total = len(universe)
        out[label] = {
            "total_in_scope": total,
            "covered_tests": len(cov_t),
            "covered_ours": len(cov_o),
            "covered_both": len(both),
            "only_tests": len(only_t),
            "only_ours": len(only_o),
            "union": len(cov_t | cov_o),
            "tests_coverage": round(len(cov_t) / total, 6) if total else None,
            "ours_coverage": round(len(cov_o) / total, 6) if total else None,
            "growth": round((len(cov_o) - len(cov_t)) / total, 6) if total else None,
        }
        # sanity checks of stage G
        chk = {
            "both_plus_only_tests_eq_covered_tests": len(both) + len(only_t) == len(cov_t),
            "both_plus_only_ours_eq_covered_ours": len(both) + len(only_o) == len(cov_o),
            "both_plus_onlys_eq_union": len(both) + len(only_t) + len(only_o) == len(cov_t | cov_o),
            "all_reported_in_scope": all(i[0] in scope for i in universe),
            "covered_le_denominator": len(cov_t) <= total and len(cov_o) <= total,
        }
        out[label]["sanity"] = chk
        for k, v in chk.items():
            if not v:
                problems.append(f"{label}: sanity check {k} FAILED")

        if label == "function":
            def dump(name, items):
                with open(os.path.join(cell_dir, name), "w") as f:
                    for i in sorted(items):
                        f.write(f"{i[0]}\t{i[1]}\tline {i[2]}\n")
            dump("covered_by_both.txt", both)
            dump("only_tests.txt", only_t)
            dump("only_ours.txt", only_o)
            uncovered = universe - (cov_t | cov_o)
            dump("covered_by_neither.txt", uncovered)
            out["function"]["covered_by_neither"] = len(uncovered)

    out["files_seen_but_out_of_scope"] = sorted((tseen | oseen) - scope)
    out["problems"] = problems
    with open(os.path.join(cell_dir, "result.json"), "w") as f:
        json.dump(out, f, indent=1)
    print(json.dumps(out, indent=1))

if __name__ == "__main__":
    main(sys.argv[1])
