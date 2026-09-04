#!/usr/bin/env python3
"""Scan every Laertes translation for SEVERED STATIC INITIALIZERS.

Laertes rewrites a C compile-time-initialised global

    static mut X: T = <real value>;                      // c2rust base

into a zero-initialised static plus a runtime initialiser

    static mut X: T = <zero>; unsafe fn laertes_init_X() { X = <real value>; }

...and then never emits a call to laertes_init_X. The static therefore keeps its
zero value for the entire life of the program.

A severed initialiser is only a *defect* if the C value is actually non-zero.
It is only *fatal* if no consumer lazily rebuilds the value. This script
separates those cases so the result is a ranked worklist, not an overclaim.

Usage: scan_severed_init.py [laertes_benchmarks_dir]
Writes results/rq4_effectiveness/severed_init_scan.json
"""
import json
import os
import re
import sys

ROOT = sys.argv[1] if len(sys.argv) > 1 else \
    "tools/frameworks/c2saferrust/laertes_benchmarks"

INIT_FN = re.compile(r'\bfn\s+(laertes_init_([A-Za-z_0-9]+))\s*\(')
# `static [mut] NAME : TYPE = INIT ;`  (INIT may span lines / nest brackets)
STATIC = re.compile(r'\bstatic\s+(?:mut\s+)?([A-Za-z_0-9]+)\s*:\s*([^=;]+?)\s*=\s*', re.S)

ZERO_TOK = re.compile(r'^[\s\[\]\{\}\(\),;]*$')


def rs_files(d):
    for dp, _, fns in os.walk(d):
        if "/target" in dp:
            continue
        for f in fns:
            if f.endswith(".rs"):
                yield os.path.join(dp, f)


def read(d):
    out = {}
    for p in rs_files(d):
        try:
            out[p] = open(p, errors="replace").read()
        except OSError:
            pass
    return out


def balanced(src, start):
    """Return the initialiser expression beginning at `start` up to its top-level ';'."""
    depth, i, n = 0, start, len(src)
    while i < n:
        c = src[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == ";" and depth <= 0:
            return src[start:i]
        i += 1
    return src[start:start + 4000]


def initializer_of(texts, name):
    """Find `static name: T = <init>` and return (init_expr, type).

    The type may itself contain ';' (array types are `[T; N]`), so the '=' is
    located by scanning forward at bracket-depth 0 rather than by a regex that
    forbids ';'. Missing this is why an earlier revision reported 0 poisoned
    statics for bzip2 and tulipindicators, whose defects are both array tables.
    """
    pat = re.compile(r'\bstatic\s+(?:mut\s+)?' + re.escape(name) + r'\s*:\s*')
    for path, src in texts.items():
        for m in pat.finditer(src):
            i, depth, n = m.end(), 0, len(src)
            while i < n:
                c = src[i]
                if c in "([{":
                    depth += 1
                elif c in ")]}":
                    depth -= 1
                elif c == "=" and depth == 0 and src[i + 1:i + 2] != "=":
                    ty = src[m.end():i].strip()
                    ln = src.count("\n", 0, m.start()) + 1
                    return balanced(src, i + 1), ty, f"{path}:{ln}"
                elif c == ";" and depth == 0:
                    break          # a declaration without an initialiser (extern block)
                i += 1
    return None, None, None


def is_zeroish(expr):
    """True if the initialiser evaluates to all-zero / all-null / empty."""
    if expr is None:
        return False
    e = expr
    # `[<elem>; <len>]` is a repeat-initialiser: the LENGTH is not a value.
    # Drop it, else `[0.; 4096]` (a zero-filled array) reads as non-zero — which
    # is what wrongly flagged genann's `lookup`, the scan's harmless control.
    for _ in range(6):
        e2 = re.sub(r'\[([^;\[\]]*?);\s*[0-9_]+\s*\]', r'[\1]', e)
        if e2 == e:
            break
        e = e2
    # strip casts, type paths, and common c2rust noise
    e = re.sub(r'as\s+[A-Za-z_0-9:<>\* ]+', ' ', e)
    # `Path::To::Type::new()` is a zero/default constructor. Match the FULLY
    # QUALIFIED path -- matching only the last segment leaves `crate::example2::`
    # behind, which is why `ti_indicators` (105x `ti_indicator_info::new()`) was
    # missed by an earlier revision.
    e = re.sub(r'[A-Za-z_0-9]+(?:::[A-Za-z_0-9]+)*::(?:new|default)\s*\(\s*\)', '0', e)
    e = re.sub(r'\b(?:std::os::raw::|core::|std::|libc::|crate::|self::)[A-Za-z_0-9:]+', ' ', e)
    e = re.sub(r'\bNone\b', '0', e)
    e = re.sub(r'\bnull(_mut)?\s*\(\s*\)', '0', e)
    e = re.sub(r'\bfalse\b', '0', e)
    e = re.sub(r'0\s*\.\s*0*', '0', e)          # 0.0 / 0.
    e = re.sub(r'\b0+\b', '0', e)
    e = re.sub(r'[;\s]+$', '', e)
    # what remains must be only zeros and structural punctuation
    e = e.replace("0", " ")
    return bool(ZERO_TOK.match(e))


report = {}
for d in sorted(os.listdir(ROOT)):
    if not d.endswith("_laertes"):
        continue
    lp = os.path.join(ROOT, d)
    bp = os.path.join(ROOT, d[:-len("_laertes")])
    if not os.path.isdir(bp):
        continue
    lt, bt = read(lp), read(bp)

    inits, calls = set(), 0
    for src in lt.values():
        for m in INIT_FN.finditer(src):
            inits.add(m.group(2))
        # a call site is the name NOT preceded by `fn `
        for m in re.finditer(r'laertes_init_[A-Za-z_0-9]+', src):
            seg = src[max(0, m.start() - 6):m.start()]
            if not seg.rstrip().endswith("fn"):
                calls += 1

    entries = []
    for name in sorted(inits):
        li, lty, lloc = initializer_of(lt, name)
        bi, _, _ = initializer_of(bt, name)
        lz, bz = is_zeroish(li), is_zeroish(bi)
        poisoned = lz and (bi is not None) and not bz
        # consumers of the static in the Laertes crate (excluding its own init fn)
        cons = 0
        for p, src in lt.items():
            for m in re.finditer(r'\b' + re.escape(name) + r'\b', src):
                ctx = src[max(0, m.start() - 200):m.start()]
                if "laertes_init_" + name in ctx[-120:]:
                    continue
                cons += 1
        loc = (lloc or "")
        rel = loc[len(ROOT) + 1:] if loc.startswith(ROOT) else loc
        # example/ and test fixture data are not library statics; a poisoned
        # fixture is not a shipped-library defect and must not be counted as one.
        # Driver/fixture files: a poisoned test fixture is not a shipped-library
        # defect. tulipindicators keeps its drivers as top-level sample.rs /
        # fuzzer.rs rather than under an example/ directory, so match names too.
        fixture = bool(re.search(
            r'(^|/)(example|examples|tests?|bench)', rel)) or bool(re.search(
            r'(^|/)(sample|fuzzer|smoke|demo|main_test)[A-Za-z_0-9]*\.rs(:\d+)?$', rel))
        entries.append({
            "static": name, "type": (lty or "")[:60],
            "loc": rel, "fixture": fixture,
            "laertes_init": (li or "")[:120].replace("\n", " "),
            "base_init": (bi or "")[:120].replace("\n", " "),
            "laertes_zero": lz, "base_zero": bz,
            "poisoned": poisoned, "consumer_refs": cons,
        })

    report[d] = {
        "init_fns": len(inits),
        "call_sites": calls,
        "poisoned": sum(1 for e in entries if e["poisoned"]),
        "poisoned_lib": sum(1 for e in entries if e["poisoned"] and not e["fixture"]),
        "entries": entries,
    }

os.makedirs("results/rq4_effectiveness", exist_ok=True)
json.dump(report, open("results/rq4_effectiveness/severed_init_scan.json", "w"), indent=1)

print(f"{'crate':28} {'init_fns':>9} {'calls':>6} {'poisoned':>9} {'lib-only':>9}")
print("-" * 66)
tot_i = tot_c = tot_p = tot_l = 0
for k, v in report.items():
    print(f"{k:28} {v['init_fns']:>9} {v['call_sites']:>6} {v['poisoned']:>9} {v['poisoned_lib']:>9}")
    tot_i += v["init_fns"]; tot_c += v["call_sites"]
    tot_p += v["poisoned"]; tot_l += v["poisoned_lib"]
print("-" * 66)
print(f"{'TOTAL':28} {tot_i:>9} {tot_c:>6} {tot_p:>9} {tot_l:>9}")
