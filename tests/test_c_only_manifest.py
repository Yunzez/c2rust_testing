#!/usr/bin/env python3
"""Regression tests for the frozen C-only completion controller.

Run: python3 tests/test_c_only_manifest.py
"""

import json
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts" / "rq4"))

import run_c_only_manifest as manifest  # noqa: E402


def check_fresh_attempt() -> None:
    run_id = {"unit": "demo", "files": {"controller": "same"}}
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)

        # An interrupted attempt with the identical run id is immutable.
        old = root / "attempt-1"
        old.mkdir()
        (old / "MANIFEST_RUN.json").write_text(json.dumps(run_id))
        before = {p.relative_to(root): p.read_bytes() for p in old.rglob("*") if p.is_file()}
        assert manifest.next_attempt(root, run_id) == root / "attempt-2"
        after = {p.relative_to(root): p.read_bytes() for p in old.rglob("*") if p.is_file()}
        assert after == before

        # Numbering advances past completed, malformed, and interrupted runs.
        (root / "attempt-2").mkdir()
        (root / "attempt-3").mkdir()
        (root / "attempt-3" / "MANIFEST_RUN.json").write_text("not json")
        assert manifest.next_attempt(root, run_id) == root / "attempt-4"


def main() -> int:
    check_fresh_attempt()
    print("PASS: interrupted C-only attempts are never resumed or mutated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
