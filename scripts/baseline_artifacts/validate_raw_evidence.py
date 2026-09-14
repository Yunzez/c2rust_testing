#!/usr/bin/env python3
"""Verify hashes of external raw RustAssure and FLOURINE run evidence."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
RUNS = ROOT / "results/baseline_artifacts/runs"


def sha256(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(block)
    return hasher.hexdigest()


def validate_hashes(owner: str, directory: str | None, hashes: dict) -> int:
    if not directory or not hashes:
        return 0
    root = Path(directory)
    checked = 0
    for relative, expected in hashes.items():
        # FLOURINE groups counterexample hashes under a descriptive `crashes`
        # key; the nested keys remain relative to the attempt directory.
        if isinstance(expected, dict):
            checked += validate_hashes(owner, directory, expected)
            continue
        path = root / relative
        assert path.is_file(), f"missing raw evidence: {owner}: {path}"
        assert sha256(path) == expected, f"raw evidence hash changed: {owner}: {path}"
        checked += 1
    return checked


def main() -> None:
    checked = 0
    for baseline in ("rustassure", "flourine"):
        for result_path in sorted((RUNS / baseline).glob("*/result.json")):
            result = json.loads(result_path.read_text())
            owner = f"{baseline}/{result['defect_id']}"
            checked += validate_hashes(
                owner,
                result.get("external_directory") or result.get("formal_attempt"),
                result.get("hashes", {}),
            )
            for attempt in result.get("attempts", []):
                checked += validate_hashes(
                    f"{owner}/{attempt.get('name', 'attempt')}",
                    attempt.get("external_directory"),
                    attempt.get("hashes", {}),
                )
    assert checked > 0, "no raw evidence hashes were checked"
    print(f"validated {checked} raw evidence files")


if __name__ == "__main__":
    main()
