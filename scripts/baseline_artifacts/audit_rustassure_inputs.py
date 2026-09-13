#!/usr/bin/env python3
"""Audit prepared RustAssure inputs without preflighting translated Rust.

RustAssure documents preprocessed ``.i`` files as its C-side input.  This
audit verifies package hashes, rejects residual include directives, and asks
the frozen artifact's Clang 14 to compile each C input.  It deliberately does
not compile Rust: formal released-artifact runs must expose Rust compiler or
emitter failures rather than having those failures filtered in advance.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import tempfile
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
ADAPTERS = ROOT / "results/baseline_artifacts/adapters/rustassure"
MANIFEST = ROOT / "results/rq4_effectiveness/defect_manifest.json"
OUT = ROOT / "results/baseline_artifacts/setup/rustassure_input_audit.json"
IMAGE = "c2r-baseline-rustassure:39618406"
UNSUPPORTED = {"C5", "S13"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def expected_hash(metadata: dict, path: Path) -> tuple[bool, str | None]:
    """Match ordinary hashes and C1's preserved original/adapted alternatives."""
    hashes = metadata.get("input_sha256", {})
    actual = sha256(path)
    candidates = [value for key, value in hashes.items() if key == path.name]
    if not candidates and path.suffix == ".rs":
        candidates = [
            value
            for key, value in hashes.items()
            if key.startswith(path.stem + ".rs_")
        ]
    return actual in candidates, actual


def compile_c(path: Path) -> subprocess.CompletedProcess[str]:
    with tempfile.TemporaryDirectory() as temporary:
        return subprocess.run(
            [
                "docker", "run", "--rm", "--cpus", "1",
                "-v", f"{path.parent}:/input:ro",
                "-v", f"{temporary}:/output",
                IMAGE, "clang", "-c", f"/input/{path.name}",
                "-o", "/output/input.o",
            ],
            text=True,
            capture_output=True,
        )


def main() -> None:
    defects = json.loads(MANIFEST.read_text())["defects"]
    ids = {row["id"] for row in defects}
    expected = ids - UNSUPPORTED
    prepared = {path.name for path in ADAPTERS.iterdir() if path.is_dir()}
    assert prepared == expected, {
        "missing": sorted(expected - prepared),
        "unexpected": sorted(prepared - expected),
    }

    rows = []
    for defect in sorted(prepared):
        package = ADAPTERS / defect
        input_dir = package / "input"
        metadata = json.loads((package / "adapter.json").read_text())
        c_files = sorted(input_dir.glob("*.i"))
        rust_files = sorted(input_dir.glob("*.rs"))
        assert len(c_files) == 1 and len(rust_files) == 1, defect
        c_path, rust_path = c_files[0], rust_files[0]
        c_text = c_path.read_text()
        residual_include = any(
            line.lstrip().startswith("#include") for line in c_text.splitlines()
        )
        c_hash_ok, c_hash = expected_hash(metadata, c_path)
        rust_hash_ok, rust_hash = expected_hash(metadata, rust_path)
        mapping = package / "argument_order_map.json"
        map_hash_ok = sha256(mapping) in metadata.get("input_sha256", {}).values()
        completed = compile_c(c_path)
        rows.append(
            {
                "defect_id": defect,
                "target": metadata["target"],
                "c_input": str(c_path.relative_to(ROOT)),
                "rust_input": str(rust_path.relative_to(ROOT)),
                "c_sha256": c_hash,
                "rust_sha256": rust_hash,
                "hashes_match_adapter": c_hash_ok and rust_hash_ok and map_hash_ok,
                "residual_include": residual_include,
                "frozen_clang14_compile_exit": completed.returncode,
                "frozen_clang14_stderr": completed.stderr,
                "rust_preflight_performed": False,
            }
        )

    report = {
        "schema_version": 1,
        "baseline": "rustassure",
        "frozen_image": IMAGE,
        "prepared_count": len(rows),
        "unsupported_not_prepared": sorted(UNSUPPORTED),
        "checks": {
            "all_hashes_match": all(row["hashes_match_adapter"] for row in rows),
            "all_c_inputs_preprocessed": all(not row["residual_include"] for row in rows),
            "all_c_inputs_compile_with_frozen_clang14": all(
                row["frozen_clang14_compile_exit"] == 0 for row in rows
            ),
            "rust_preflight_performed": False,
        },
        "rows": rows,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report["checks"], sort_keys=True))


if __name__ == "__main__":
    main()
