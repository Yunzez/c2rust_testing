#!/usr/bin/env python3
"""Validate frozen artifacts, shipped smokes, and prepared-input evidence."""

from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path("/home/yunzez/c2rust_testing")
BASE = Path("/home/yunzez/c2rust_baselines")


def load(relative: str) -> dict:
    return json.loads((ROOT / relative).read_text())


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def digest(path: Path, algorithm: str) -> str:
    hasher = hashlib.new(algorithm)
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(block)
    return hasher.hexdigest()


def git_output(directory: Path, *args: str) -> str:
    return subprocess.check_output(
        ("git", "-C", str(directory), *args), text=True
    ).strip()


def validate_lock() -> None:
    lock = load("results/baseline_artifacts/artifact_lock.json")
    assert len(lock["rustassure"]["commit"]) == 40
    assert len(lock["rustassure"]["dependency"]["commit"]) == 40
    assert lock["flourine"]["sha256"] and lock["flourine"]["bytes"] > 0
    assert lock["vert"]["doi"] == "10.5281/zenodo.10927704"
    assert lock["vert"]["checksum"].startswith("md5:")

    checkouts = BASE / "checkouts"
    repositories = {
        checkouts / "rustassure-official": lock["rustassure"]["commit"],
        checkouts / "rustify-klee-official": lock["rustassure"]["dependency"]["commit"],
        checkouts / "rwasm-official": lock["vert"]["omitted_generation_dependencies"]["rwasm"]["commit"],
    }
    for directory, expected_commit in repositories.items():
        assert git_output(directory, "rev-parse", "HEAD") == expected_commit, directory
        assert git_output(directory, "status", "--porcelain") == "", (
            f"official checkout was modified: {directory}"
        )

    downloads = BASE / "downloads"
    flourine = downloads / "flourine-artifact.tar.gz"
    vert = downloads / lock["vert"]["file"]
    wasi = downloads / "wasi-sdk-12.0-linux.tar.gz"
    assert flourine.stat().st_size == lock["flourine"]["bytes"]
    assert digest(flourine, "sha256") == lock["flourine"]["sha256"]
    assert vert.stat().st_size == lock["vert"]["bytes"]
    assert digest(vert, "md5") == lock["vert"]["checksum"].removeprefix("md5:")
    assert digest(wasi, "sha256") == (
        lock["vert"]["omitted_generation_dependencies"]["wasi_sdk"]["sha256"]
    )


def validate_rustassure_smoke() -> None:
    smoke = load("results/baseline_artifacts/setup/rustassure/shipped-smoke.json")
    assert all(smoke["gates"][gate] for gate in ("submitted", "accepted", "compiled", "completed"))
    assert smoke["gates"]["detected_difference"] is False
    evidence = smoke["evidence"]
    directory = Path(evidence["external_directory"])
    paths = {
        "console_sha256": directory / "console.log",
        "result_csv_sha256": directory / "workdir/result.csv",
        "best_edit_distances_sha256": directory / "workdir/edit_distance/best_edit_distances.csv",
    }
    for key, path in paths.items():
        assert path.is_file(), path
        assert sha256(path) == evidence[key], path


def validate_flourine_smoke() -> None:
    smoke = load("results/baseline_artifacts/setup/flourine/shipped-smoke.json")
    assert all(smoke["gates"][gate] for gate in ("submitted", "accepted", "compiled", "completed"))
    assert smoke["gates"]["detected_difference"] is False
    directory = Path(smoke["evidence"]["external_directory"])
    paths = {
        "verify_log_sha256": directory / "verify.log",
        "console_log_sha256": directory / "console.log",
    }
    for key, path in paths.items():
        assert path.is_file(), path
        assert sha256(path) == smoke["evidence"][key], path


def validate_vert_smoke() -> None:
    smoke = load("results/baseline_artifacts/setup/vert/shipped_smoke.json")
    assert smoke["completed"] is True
    assert smoke["counterexample"] is False
    assert smoke["exit_status"] == 0 and smoke["runs"] > 0
    assert Path(smoke["log"]).is_file()


def validate_prepared_inputs() -> None:
    audit = load("results/baseline_artifacts/setup/rustassure_input_audit.json")
    assert audit["prepared_count"] == 34
    assert audit["unsupported_not_prepared"] == ["C5", "S13"]
    checks = audit["checks"]
    assert checks["all_hashes_match"] is True
    assert checks["all_c_inputs_preprocessed"] is True
    assert checks["all_c_inputs_compile_with_frozen_clang14"] is True
    assert checks["rust_preflight_performed"] is False


def main() -> None:
    validate_lock()
    validate_rustassure_smoke()
    validate_flourine_smoke()
    validate_vert_smoke()
    validate_prepared_inputs()
    print("validated frozen locks, 3 shipped smokes, and 34 prepared RustAssure inputs")


if __name__ == "__main__":
    main()
