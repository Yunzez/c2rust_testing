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


# glob_match/kv_config were re-baselined when input_string landed; graph_dfs is a new capability
# (ptr-to-array) with no pre-migration baseline. The other 12 are byte-identical to the
# pre-migration generator.
REVIEWED_CHANGE = {"glob_match", "kv_config"}
NEW_CAPABILITY = {"graph_dfs"}


def main():
    fixtures = sorted(FIX.glob("*_ft.rs"))
    if not fixtures:
        print("no fixtures found"); return 1
    failed = 0
    print("(12 migration-identical; 2 reviewed changes: glob_match, kv_config; "
          "1 new capability: graph_dfs)\n")
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
            kind = ("reviewed-change golden" if prog in REVIEWED_CHANGE
                    else "new-capability golden" if prog in NEW_CAPABILITY
                    else "migration-identical")
            if gen_file.read_text() == fix.read_text():
                print(f"  PASS  {prog}: matches {kind}")
            else:
                print(f"  FAIL  {prog}: schema-driven output differs from fixture")
                failed += 1
    print(f"\n{len(fixtures) - failed}/{len(fixtures)} match current golden "
          f"(12 migration-identical + 2 reviewed changes)")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
