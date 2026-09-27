#!/usr/bin/env python3

"""Transpile every C program under benchmark/raw/ into a C<->Rust pair with c2rust.

For each `benchmark/raw/<theme>/<name>.c` it produces:

  benchmark/pairs/<name>/
    source/<name>.c                 # original C
    build/compile_commands.json     # for c2rust + stu_selector
    translated/<name>.rs            # c2rust output

Records per-program success/failure so we can see which inputs c2rust handles.
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RAW = ROOT / "benchmark" / "raw"
PAIRS = ROOT / "benchmark" / "pairs"
C2RUST = os.path.expanduser("~/.cargo/bin/c2rust")


def transpile_one(c_file: Path) -> dict:
    name = c_file.stem
    theme = c_file.parent.name
    pair = PAIRS / name
    src_dir = pair / "source"
    build_dir = pair / "build"
    tr_dir = pair / "translated"
    for d in (src_dir, build_dir, tr_dir):
        d.mkdir(parents=True, exist_ok=True)

    dst_c = src_dir / c_file.name
    shutil.copy2(c_file, dst_c)
    dst_c_abs = str(dst_c.resolve())

    cc = [{
        "directory": str(build_dir.resolve()),
        "file": dst_c_abs,
        "arguments": ["clang", "-c", dst_c_abs],
    }]
    cc_path = build_dir / "compile_commands.json"
    cc_path.write_text(json.dumps(cc, indent=2), encoding="utf-8")

    proc = subprocess.run(
        [C2RUST, "transpile", str(cc_path)],
        capture_output=True, text=True,
    )
    ok = proc.returncode == 0

    # c2rust emits <name>.rs next to the source.
    produced = src_dir / f"{name}.rs"
    rust_out = tr_dir / f"{name}.rs"
    if produced.exists():
        shutil.move(str(produced), str(rust_out))
    else:
        ok = False

    return {
        "name": name,
        "theme": theme,
        "ok": ok and rust_out.exists(),
        "rust": str(rust_out) if rust_out.exists() else None,
        "compile_commands": str(cc_path),
        "error": (proc.stderr.strip().splitlines()[-1] if not ok and proc.stderr.strip() else None),
    }


def main() -> int:
    if not Path(C2RUST).exists():
        print(f"[error] c2rust not found at {C2RUST}", file=sys.stderr)
        return 1
    c_files = sorted(RAW.rglob("*.c"))
    if not c_files:
        print(f"[error] no C files under {RAW}", file=sys.stderr)
        return 1

    results = [transpile_one(f) for f in c_files]
    ok = [r for r in results if r["ok"]]
    bad = [r for r in results if not r["ok"]]

    print(f"transpiled {len(ok)}/{len(results)} programs")
    for r in results:
        mark = "OK " if r["ok"] else "FAIL"
        extra = "" if r["ok"] else f"  ({r['error']})"
        print(f"  [{mark}] {r['theme']}/{r['name']}{extra}")

    (PAIRS / "_transpile_report.json").write_text(json.dumps(results, indent=2), encoding="utf-8")
    return 0 if not bad else 0  # non-fatal; report captures failures


if __name__ == "__main__":
    raise SystemExit(main())
