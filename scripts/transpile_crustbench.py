#!/usr/bin/env python3
"""Batch-transpile CRUST-bench CBench programs with c2rust (Linux-native baseline).

The shipped C2SaferRust/Laertes Rust is macOS-c2rust and won't build on Linux; we re-transpile
the real C ourselves. For each CBench program we take its library .c files (excluding test/bench/
main drivers), build a compile_commands.json with header include dirs, run `c2rust transpile`
(PLAIN, not --emit-build-files which panics on some inputs), and apply the bitfields recipe
(insert `#[macro_use] extern crate c2rust_bitfields;` after the leading inner-attr block; the
harness Cargo.toml pins c2rust-bitfields = "0.22" to match c2rust 0.22.1).

Produces, per program that transpiles:
  <out>/<name>/source/*.c   build/compile_commands.json   translated/<name>.rs
and a report.json with per-program status (ok / transpile_fail / multi_tu / no_lib_c).

This is the faithful c2rust BASELINE column of the RQ1 (program x tool) bug matrix; c2rust is
faithful so these are expected TN under the differential pipeline — the value is the control
column plus the harnessability filter for the deviating tools (SACTOR/CROWN/C2SaferRust).

Usage: transpile_crustbench.py [--limit N] [--only name1,name2] --out <dir>
"""
from __future__ import annotations
import argparse, json, os, shutil, subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CBENCH = ROOT / "tools" / "frameworks" / "CRUST-bench" / "datasets" / "CBench"
C2RUST = os.path.expanduser("~/.cargo/bin/c2rust")


def lib_c_files(prog: Path):
    """Library .c files: exclude test/bench dirs and obvious main drivers."""
    out = []
    for c in sorted(prog.rglob("*.c")):
        # check parts RELATIVE to the program dir (the absolute prefix contains "CRUST-bench"/"CBench")
        rel = {p.lower() for p in c.relative_to(prog).parts}
        if any("test" in p or "bench" in p or "example" in p for p in rel):
            continue
        if c.name in ("main.c",):
            continue
        out.append(c)
    return out


def include_dirs(prog: Path):
    dirs = {str(prog.resolve())}
    for h in prog.rglob("*.h"):
        dirs.add(str(h.parent.resolve()))
    return sorted(dirs)


def apply_bitfields(rs: Path):
    lines = rs.read_text().splitlines()
    if not any("BitfieldStruct" in l for l in lines):
        return False
    last = -1
    for idx, l in enumerate(lines):
        s = l.strip()
        if s.startswith("#!"):
            depth = l.count("(") - l.count(")"); last = idx; j = idx
            while depth > 0 and j + 1 < len(lines):
                j += 1; depth += lines[j].count("(") - lines[j].count(")"); last = j
        elif last >= 0 and idx > last:
            break
    if "c2rust_bitfields" not in "\n".join(lines[:max(last + 3, 3)]):
        lines.insert(last + 1, "#[macro_use] extern crate c2rust_bitfields;")
        rs.write_text("\n".join(lines) + "\n")
    return True


def transpile_one(prog: Path, out: Path) -> dict:
    name = prog.name
    libs = lib_c_files(prog)
    if not libs:
        return {"name": name, "ok": False, "status": "no_lib_c"}
    if len(libs) > 1:
        # multi-TU: c2rust emits one .rs per file; single-.rs pairing not handled here (defer)
        return {"name": name, "ok": False, "status": "multi_tu", "n_c": len(libs)}
    c = libs[0]
    pair = out / name
    src, build, tr = pair / "source", pair / "build", pair / "translated"
    for d in (src, build, tr):
        d.mkdir(parents=True, exist_ok=True)
    # copy the whole program's headers next to the source so includes resolve
    for h in prog.rglob("*.h"):
        try:
            shutil.copy2(h, src / h.name)
        except Exception:
            pass
    dst_c = src / c.name
    shutil.copy2(c, dst_c)
    incs = [f"-I{d}" for d in include_dirs(prog)] + [f"-I{src.resolve()}"]
    cc = [{"directory": str(build.resolve()), "file": str(dst_c.resolve()),
           "arguments": ["clang", "-c", str(dst_c.resolve()), *incs]}]
    (build / "compile_commands.json").write_text(json.dumps(cc, indent=2))
    proc = subprocess.run([C2RUST, "transpile", str(build / "compile_commands.json")],
                          capture_output=True, text=True, timeout=300)
    produced = src / f"{c.stem}.rs"
    if proc.returncode != 0 or not produced.exists():
        return {"name": name, "ok": False, "status": "transpile_fail",
                "detail": (proc.stderr or proc.stdout)[-200:]}
    rust_out = tr / f"{name}.rs"
    shutil.move(str(produced), str(rust_out))
    bf = apply_bitfields(rust_out)
    externc = rust_out.read_text().count('extern "C" fn')
    return {"name": name, "ok": True, "status": "ok", "rust": str(rust_out),
            "bitfields": bf, "extern_c_fns": externc, "lines": len(rust_out.read_text().splitlines())}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--only", default=None, help="comma-separated program names")
    args = ap.parse_args()
    out = Path(args.out); out.mkdir(parents=True, exist_ok=True)
    progs = sorted(p for p in CBENCH.iterdir() if p.is_dir())
    if args.only:
        keep = set(args.only.split(","))
        progs = [p for p in progs if p.name in keep]
    if args.limit:
        progs = progs[:args.limit]
    rows, ok = [], 0
    print(f"{'program':28} {'status':16} {'externC':>7} {'bf':>3} {'lines':>6}")
    print("-" * 66)
    for p in progs:
        try:
            r = transpile_one(p, out)
        except subprocess.TimeoutExpired:
            r = {"name": p.name, "ok": False, "status": "timeout"}
        except Exception as e:
            r = {"name": p.name, "ok": False, "status": f"exc:{type(e).__name__}", "detail": str(e)[:150]}
        rows.append(r)
        ok += 1 if r.get("ok") else 0
        print(f"{r['name']:28} {r['status']:16} {str(r.get('extern_c_fns','-')):>7} "
              f"{'Y' if r.get('bitfields') else '-':>3} {str(r.get('lines','-')):>6}")
    print("-" * 66)
    print(f"transpiled OK: {ok}/{len(progs)}")
    (out / "report.json").write_text(json.dumps(rows, indent=1))
    print(f"wrote {out/'report.json'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
