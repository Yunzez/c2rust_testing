#!/usr/bin/env python3
"""P3 step-2 regression: schema-driven harness generation must be BYTE-IDENTICAL.

The golden fixtures in tests/fixtures/harness/<prog>_ft.rs were captured from the pre-migration
(adjacency-based) generator. After switching the generator to read schemas/<prog>.json, the
generated fuzz-target source for every one of the 14 entries must match its fixture exactly.

Run: python3 tests/test_harness_regression.py
"""

import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIX = ROOT / "tests" / "fixtures" / "harness"
GEN = ROOT / "tools" / "stu_selector" / "gen_diff_harness.py"


def main():
    fixtures = sorted(FIX.glob("*_ft.rs"))
    if not fixtures:
        print("no fixtures found"); return 1
    failed = 0
    with tempfile.TemporaryDirectory() as td:
        for fix in fixtures:
            prog = fix.name[:-len("_ft.rs")]
            entry = json.loads((ROOT / "schemas" / f"{prog}.json").read_text())["entry"]
            outdir = Path(td) / prog
            r = subprocess.run(
                ["python3", str(GEN), "--pair", str(ROOT / "benchmark" / "pairs" / prog),
                 "--entry", entry, "--out", str(outdir)],
                capture_output=True, text=True)
            gen_file = outdir / "fuzz" / "fuzz_targets" / f"{prog}_ft.rs"
            if r.returncode != 0 or not gen_file.exists():
                print(f"  FAIL  {prog}: generation failed ({r.stderr.strip()[-120:]})")
                failed += 1
                continue
            if gen_file.read_text() == fix.read_text():
                print(f"  PASS  {prog}: byte-identical")
            else:
                print(f"  FAIL  {prog}: schema-driven output differs from fixture")
                failed += 1
    print(f"\n{len(fixtures) - failed}/{len(fixtures)} byte-identical")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
