#!/usr/bin/env python3
"""The end-to-end funnel for one pair: boundaries -> planned -> built -> executed.

Every count quoted so far stops at `planned`. That is not the same as a working harness: a boundary
whose input model puts every input out of bounds still plans and still builds, and then dies on
execution #1. This script measures the whole chain so the difference is visible.

    boundaries        functions defined in the pair's translation unit
    planned           a complete HarnessPlan: InputPlan + a lossless Rust bridge for every param
    built             the generated harness compiles
    executed          it runs, and how far it gets before the first artifact
    oracle_strength   termination-only | partial(nullness) | observable-state | structured-state

`executed` is deliberately reported as (executions, artifacts), not as a boolean: a harness with one
execution and one artifact is built-but-unproductive, and a single column cannot say that.

Usage:
  scripts/c2r_funnel.py --pair <dir> --out <dir> [--seconds 20] [--plugins <toml>]
                        [--c-source <name>] [--shim <file>] [--defs <defs.json>] [--jobs 1]
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
TOOLCHAIN = "nightly-2025-09-01"


def sh(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True, errors="replace", **kw)


def plan_all(pair: Path, out: Path) -> list[dict]:
    r = sh([sys.executable, str(ROOT / "tools/stu_selector/harness_plan.py"),
            "--pair", str(pair), "--all", "--json", str(out / "plans.json")],
           cwd=str(ROOT), timeout=1800)
    (out / "plan.log").write_text(r.stdout + r.stderr)
    return json.loads((out / "plans.json").read_text())


def pair_excludes(pair: Path) -> set[str]:
    """`<pair>/exclude.txt`: boundaries that are not library functions (a test driver's `main`
    when the translation unit IS the test program, as urlparser's test.c is), one per line, `#`
    comments. They are dropped before planning and never counted in the funnel."""
    p = Path(pair) / "exclude.txt"
    if not p.exists():
        return set()
    return {ln.split("#")[0].strip() for ln in p.read_text().splitlines() if ln.split("#")[0].strip()}


def rust_name(pair: Path, entry: str) -> str:
    """The entry's name in the Rust translation: `<pair>/translated/renames.json` (the RQ1
    matcher's map for a renaming translator) or the C name itself."""
    p = Path(pair) / "translated" / "renames.json"
    m = json.loads(p.read_text()) if p.exists() else {}
    return m.get(entry, entry)


def generate(a, pair: Path, entry: str, out_dir: Path, private: bool) -> tuple[bool, str]:
    cmd = [sys.executable, str(ROOT / "tools/stu_selector/gen_diff_harness.py"),
           "--pair", str(pair), "--entry", entry, "--rust-entry", rust_name(pair, entry),
           "--plan", "--ub-free", "--out", str(out_dir)]
    if a.c_source:
        cmd += ["--c-source", a.c_source]
    for pl in (a.plugins or []):
        cmd += ["--plugins", pl]
    if private:
        cmd += ["--expose-entry"]
    r = sh(cmd, cwd=str(ROOT), timeout=900)
    return r.returncode == 0, (r.stdout + r.stderr)


def fixups(a, pair: Path, out_dir: Path, entry: str, private: bool, defs: dict) -> str | None:
    """Pair-level packaging the generator does not own. Returns an error string, or None."""
    import gen_diff_harness as gdh
    if a.c_source:
        # The pair ships an amalgamation plus its siblings; the generator copied only the named
        # translation unit, so the siblings it #includes have to come too.
        stripped = None
        # source SUBDIRECTORIES first (tulip: `tulip.c` includes indicators/*.c by relative path;
        # optipng: 52 units in nine directories), then every .c/.h at any depth gets the static strip
        for sub in sorted(p for p in (pair / "source").iterdir() if p.is_dir()):
            shutil.copytree(sub, out_dir / "c" / sub.name, dirs_exist_ok=True)
        for extra in sorted(list((pair / "source").rglob("*.c")) + list((pair / "source").rglob("*.h"))):
            if extra.name == a.c_source and extra.parent == pair / "source":
                continue                   # (a header-only library keeps its statics in the .h: urlparser's strff)
            rel = extra.relative_to(pair / "source")
            text = extra.read_text()
            # Internal linkage is a fact about the C SIDE, so the strip is driven by the C side and
            # not by what the translation did with the function: PtrTrans emits quadtree's
            # `static find_` as a `pub fn`, so the Rust-driven `private` flag was False, the oracle
            # kept its `static`, and the harness failed to LINK (`undefined symbol: c_find_`).
            # On a non-static definition the substitution simply does not match.
            # ORDER MATTERS (2026-09-10): the `static\n__inline__\n` shape (bzip2's blocksort.c: mmed3,
            # mainGtU, fallbackSimpleSort) must be handled FIRST. strip_static_c alone drops `static`
            # and leaves a bare `__inline__`, which under C99/gnu11 provides no external definition,
            # so `c_mmed3` was undefined at link time -- a regression against the archived bzip2 cells
            # introduced when strip_static_c was put in front of this pattern for PtrTrans's find_.
            pat = re.compile(
                rf'(?m)^[ \t]*static[ \t]*\n(?:[ \t]*(?:__inline__|inline)[ \t]*\n)?'
                rf'([ \t]*[A-Za-z_][\w \t\*]*\b{re.escape(entry)}[ \t]*\()')
            text, n = pat.subn(r'\1', text, count=1)
            changed = bool(n)
            if not changed:
                text, changed = gdh.strip_static_c(text, entry)
            if changed:
                stripped = str(rel)
            (out_dir / "c" / rel).parent.mkdir(parents=True, exist_ok=True)
            (out_dir / "c" / rel).write_text(text)
        # A multi-TU pair (compile_commands.json with one entry per unit): the oracle is every unit
        # compiled separately with the same rename defines, as the library's own build does.
        # build.rs gets the units, their include directories and their -D/-std flags.
        ccj = pair / "build" / "compile_commands.json"
        cmds = json.loads(ccj.read_text()) if ccj.exists() else []
        if len(cmds) > 1:
            src_root = (pair / "source").resolve()
            files, incs, flags = [], [], []
            for c in cmds:
                if c["file"] == a.c_source:
                    continue
                files.append(f'c/{c["file"]}')
                args = c.get("arguments") or c.get("command", "").split()
                it = iter(range(len(args)))
                for i in it:
                    x = args[i]
                    if x == "-I" and i + 1 < len(args):
                        d = Path(args[i + 1]).resolve()
                        rel = d.relative_to(src_root) if d != src_root and src_root in d.parents else Path(".")
                        inc = "c" if str(rel) == "." else f"c/{rel}"
                        if inc not in incs:
                            incs.append(inc)
                        next(it, None)
                    elif x.startswith(("-D", "-std=", "-U")) and x not in flags:
                        flags.append(x)
            b = out_dir / "build.rs"
            bt = b.read_text()
            anchor = f'    build.file("c/{a.c_source}");'
            assert anchor in bt, "build.rs anchor not found for the multi-TU rewrite"
            extra = "".join(f'\n    build.file("{f}");' for f in files)
            extra += "".join(f'\n    build.include("{d}");' for d in incs)
            extra += "".join(f'\n    build.flag("{f}");' for f in flags)
            b.write_text(bt.replace(anchor, anchor + extra, 1))
        if private and stripped is None:
            # Single-TU pair (cJSON): the static lives in the oracle TU itself, and the generator's
            # --expose-entry already dropped its `static` there. Only a static that is STILL
            # static in the harness copy is a failure.
            main = out_dir / "c" / a.c_source
            still_static = main.exists() and gdh.strip_static_c(main.read_text(), entry)[1]
            if still_static:
                return f"could not give the C `static` {entry} external linkage (not in the oracle TU nor any sibling .c)"
    if a.shim:
        shutil.copy(a.shim, out_dir / "c" / "shims.c")
        b = out_dir / "build.rs"
        t = b.read_text()
        t = t.replace('    build.compile("c_oracle");',
                      '    build.file("c/shims.c");\n    build.compile("c_oracle");\n'
                      '    println!("cargo:rustc-link-arg=-Wl,-u,__maskrune");\n'
                      '    println!("cargo:rustc-link-arg=-Wl,-u,_DefaultRuneLocale");')
        b.write_text(t)
    if private and defs:
        mod = defs.get("defs", {}).get(entry)
        lib = out_dir / "src" / "lib.rs"
        # A flat single-file translation (cJSON, SACTOR's genann) has no module: --expose-entry
        # already made the entry pub at the crate root, and a re-export through a module that does
        # not exist is an unresolved import.
        text = lib.read_text()
        # a directory module (CROWN quadtree `src/quadtree`, tulip `indicators/abs`) is declared by
        # its leaf name and addressed by its `::` path
        leaf = mod.rpartition("/")[2] if mod else mod
        if mod and re.search(rf'(?m)^\s*(?:pub\s+)?mod\s+{re.escape(leaf)}\s*\{{', text):
            # CROWN wraps the modules in a namespace (`pub mod src { pub mod lil {..} }`): the
            # flatten's own re-exports say what the path prefix is; a C `static` exposed here
            # takes the same one (15 lil x CROWN builds failed with `unresolved import crate::lil`).
            modpath = mod.replace("/", "::")
            m = re.search(rf'(?m)^pub use crate::((?:\w+::)*){re.escape(modpath)}::\w+;', text)
            prefix = m.group(1) if m else ""
            lib.write_text(text + f"\npub use crate::{prefix}{modpath}::{entry};\n")
    # Types the generated target names at the crate root (`translated::quadtree_point_t` for a
    # POD struct input) live in the ENTRY's module: c2rust's per-file output re-declares every
    # type in each module and CROWN aliases them per module, so the entry's own module is the one
    # spelling its signature uses. Re-export exactly those, from that module, when the root does
    # not already provide them. Functions are re-exported by the flatten; this is for types.
    if defs:
        mod = defs.get("defs", {}).get(entry)
        lib = out_dir / "src" / "lib.rs"
        ft = out_dir / "fuzz" / "fuzz_targets"
        srcs = [p.read_text() for p in ft.glob("*.rs")] if ft.exists() else []
        if mod and srcs and lib.exists():
            text = lib.read_text()
            modpath = mod.replace("/", "::")
            m = re.search(rf'(?m)^pub use crate::((?:\w+::)*){re.escape(modpath)}::\w+;', text)
            prefix = m.group(1) if m else ""
            names = set()
            for s in srcs:
                names.update(re.findall(r'\btranslated::([A-Za-z_]\w*)\b', s))
            add = []
            for nm in sorted(names):
                if re.search(rf'(?m)^pub use crate::(?:\w+::)*{re.escape(nm)};', text):
                    continue                                   # a function the flatten re-exported
                if re.search(rf'(?m)^\s*pub\s+(?:struct|type|enum|union)\s+{re.escape(nm)}\b', text) is None:
                    continue                                   # not a type the translation defines
                if re.search(rf'(?m)^(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(nm)}\b', text):
                    continue
                add.append(f"pub use crate::{prefix}{modpath}::{nm};")
            if add:
                lib.write_text(text + "\n// types the generated target names, from the entry's own module\n"
                               + "\n".join(add) + "\n")
    for d in (out_dir, out_dir / "fuzz"):
        (d / "rust-toolchain").write_text(TOOLCHAIN + "\n")
    return None


def build(out_dir: Path, target_dir: Path) -> tuple[bool, str]:
    env = dict(os.environ, CARGO_TARGET_DIR=str(target_dir), RUSTUP_TOOLCHAIN=TOOLCHAIN)
    r = sh(["cargo", "fuzz", "build"], cwd=str(out_dir), env=env, timeout=1800)
    errs = [l for l in (r.stdout + r.stderr).splitlines() if l.startswith("error")]
    return r.returncode == 0, "\n".join(errs[:4])


def execute(binary: Path, corpus: Path, art: Path, seconds: int) -> dict:
    corpus.mkdir(parents=True, exist_ok=True)
    art.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    env["ASAN_OPTIONS"] = "detect_leaks=0"
    env.pop("C2R_OUTCOME_FILE", None)
    cmd = [str(binary), str(corpus), "-fork=1", "-ignore_crashes=1", "-ignore_timeouts=1",
           "-ignore_ooms=1", f"-max_total_time={seconds}", "-timeout=5", "-rss_limit_mb=4096",
           f"-artifact_prefix={art}/"]
    try:
        r = subprocess.run(cmd, env=env, capture_output=True, text=True, errors="replace", timeout=seconds + 240)
        log = r.stdout + r.stderr
    except subprocess.TimeoutExpired as e:
        log = ((e.stdout or b"").decode("replace") if isinstance(e.stdout, bytes) else (e.stdout or "")) \
            + ((e.stderr or b"").decode("replace") if isinstance(e.stderr, bytes) else (e.stderr or ""))
    # In fork mode the parent prints `#<total execs>: cov: .. oom/timeout/crash: o/t/c ..`.
    # The `#N` is followed by a COLON, and the crash counter is more honest than counting artifact
    # files, which libFuzzer dedupes by content hash.
    runs = [int(m) for m in re.findall(r"(?m)^#(\d+)[:\s]", log)]
    execs = [int(m) for m in re.findall(r"stat::number_of_executed_units:\s*(\d+)", log)]
    otc = re.findall(r"oom/timeout/crash:\s*(\d+)/(\d+)/(\d+)", log)
    o, ti, c = (int(otc[-1][0]), int(otc[-1][1]), int(otc[-1][2])) if otc else (0, 0, 0)
    return {"executions": max(runs + execs + [0]),
            "artifacts": len(list(art.glob("*"))),
            "ooms": o, "timeouts": ti, "crashes": c, "log_tail": log[-1200:]}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pair", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--seconds", type=int, default=20)
    ap.add_argument("--plugins", action="append")
    ap.add_argument("--c-source")
    ap.add_argument("--shim")
    ap.add_argument("--defs", help="<translated>.rs.defs.json: which entries are C `static`")
    ap.add_argument("--only", help="comma-separated subset, for smoke tests")
    a = ap.parse_args()

    pair, out = Path(a.pair), Path(a.out)
    (out / "harnesses").mkdir(parents=True, exist_ok=True)
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    private = set(defs.get("private", []))
    # A C `static` boundary is non-`pub` in the translation, so the harness cannot call it without
    # --expose-entry. A multi-module translation carries that in a defs file (it also needs a root
    # re-export); a single-file one does not, so read it straight from the .rs. Without this the
    # boundary builds nowhere and looks like a capability gap instead of missing packaging.
    _rs = next(iter(sorted((pair / "translated").glob("*.rs"))), None)
    _rs_text = _rs.read_text(encoding="utf-8", errors="replace") if _rs else ""

    plans = plan_all(pair, out)
    plans = [p for p in plans if p["boundary"] not in pair_excludes(pair)]
    if a.only:
        keep = {s.strip() for s in a.only.split(",")}
        plans = [p for p in plans if p["boundary"] in keep]

    rows, t0 = [], time.time()
    for p in plans:
        b = p["boundary"]
        is_private = b in private or (
            bool(_rs_text) and not defs
            and re.search(rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b',
                          _rs_text) is None)
        row = {"boundary": b, "planned": p["status"] == "planned", "c_static": is_private,
               "plan_failure": (p["failures"][0] if p["failures"] else None),
               "inputs": len(p["inputs"])}
        if row["planned"]:
            d = out / "harnesses" / b
            shutil.rmtree(d, ignore_errors=True)
            ok, log = generate(a, pair, b, d, is_private)
            row["generated"] = ok
            m = re.search(r"oracle_strength=([\w\-]+(?:\([\w\-]+\))?)", log)
            row["oracle_strength"] = m.group(1) if m else None
            if ok:
                err = fixups(a, pair, d, b, is_private, defs)
                if err:
                    row["generated"], row["error"] = False, err
            if not row.get("generated"):
                row.setdefault("error", log.strip()[-300:])
            else:
                built, errs = build(d, out / "target")
                row["built"] = built
                if not built:
                    row["error"] = errs
                else:
                    binaries = list((out / "target" / "x86_64-unknown-linux-gnu" /
                                     "release").glob("*_ft"))
                    row.update(execute(binaries[0], out / "corpus" / b, out / "candidates" / b,
                                       a.seconds) if binaries else {"error": "no binary"})
        rows.append(row)
        print(f"{b:32s} planned={row['planned']!s:5s} built={row.get('built', '-')!s:5s} "
              f"execs={row.get('executions', '-')!s:>9s} crashes={row.get('crashes', '-')!s:>5s} "
              f"oracle={row.get('oracle_strength')}", flush=True)

    (out / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
    n = len(rows)
    planned = [r for r in rows if r["planned"]]
    built = [r for r in planned if r.get("built")]
    ran = [r for r in built if (r.get("executions") or 0) > 0]
    prod = [r for r in ran if (r.get("executions") or 0) >= 100]
    print(f"\nboundaries {n} | planned {len(planned)} | built {len(built)} | "
          f"executed {len(ran)} | >=100 executions {len(prod)} | {time.time()-t0:.0f}s")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
