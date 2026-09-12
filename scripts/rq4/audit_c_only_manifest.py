#!/usr/bin/env python3
"""Independent, read-only audit for the frozen C-only reach manifest.

This checker does not import the running controller.  It reconstructs the
expected boundary sets from the archived funnels and verifies the durable
records required by ``docs/c_guided_c_only_plan.md``.  It is safe to run while
the controller is active; missing FINAL directories are reported as pending.
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
import tarfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_ARCHIVE = Path("/home/yunzez/c2rust_archive/cg_c_only_v3")
MAX_FUZZERS = 28
UNITS = [
    ("bzip2", "c2rust"),
    ("cjson", "c2rust"),
    ("cjson", "ptrtrans"),
    ("genann", "c2rust"),
    ("genann", "sactor"),
    ("lil", "laertes"),
    ("lodepng", "c2rust"),
    ("optipng", "c2saferrust"),
    ("qsort", "sactor"),
    ("quadtree", "c2rust"),
    ("tulip", "c2rust"),
    ("urlparser", "c2rust"),
]


def load(path: Path):
    with open(path) as fh:
        return json.load(fh)


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def expected_boundaries(lib: str, tool: str) -> list[str]:
    rows = load(ROOT / "results/rq3_coverage" / lib / tool / "funnel.json")
    return [row["boundary"] for row in rows if row.get("built")]


def identities(path: Path) -> tuple[dict[str, bool], dict[tuple, bool]]:
    with gzip.open(path, "rt") as fh:
        value = json.load(fh)
    functions = {row["id"]: bool(row["covered"]) for row in value["functions"]}
    regions = {tuple(row["id"]): bool(row["covered"]) for row in value["regions"]}
    if len(functions) != len(value["functions"]):
        raise ValueError(f"duplicate function identities in {path}")
    if len(regions) != len(value["regions"]):
        raise ValueError(f"duplicate region identities in {path}")
    return functions, regions


def merge_or(dst: dict, src: dict) -> None:
    for key, covered in src.items():
        dst[key] = dst.get(key, False) or covered


def exact_export_names(root: Path) -> list[str]:
    return sorted(path.name.removesuffix(".json.gz") for path in root.glob("*.json.gz"))


def corpus_count(result: dict) -> int:
    if result["protocol"] == "single-c-companion-batched":
        return sum(sum(row["corpus_sizes"].values()) for row in result["batches"])
    return sum(result["campaign"]["corpus_sizes"].values())


def live_supervisors() -> tuple[int, list[int]]:
    supervisors, orphans = [], []
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit():
            continue
        try:
            args = [x.decode("utf-8", "replace") for x in
                    entry.joinpath("cmdline").read_bytes().split(b"\0") if x]
            ppid = int(entry.joinpath("stat").read_text().split()[3])
        except (FileNotFoundError, PermissionError, ProcessLookupError, ValueError):
            continue
        is_boundary = any("/tmp/cg_c_only_manifest_" in arg and "/base/bins/" in arg
                          for arg in args)
        if is_boundary and "-fork=1" in args:
            supervisors.append(int(entry.name))
        if is_boundary and ppid == 1:
            orphans.append(int(entry.name))
    return len(supervisors), sorted(orphans)


def audit_unit(archive: Path, lib: str, tool: str) -> tuple[list[str], dict | None]:
    errors: list[str] = []
    name = f"{lib}_{tool}"
    final = archive / name / "FINAL"
    if not final.is_dir():
        return [f"PENDING {name}: FINAL absent"], None
    try:
        done = load(final / "DONE.json")
        result = load(final / "result.json")
    except (FileNotFoundError, json.JSONDecodeError) as exc:
        return [f"ERROR {name}: unreadable final record: {exc}"], None
    if done.get("valid") is not True:
        errors.append(f"ERROR {name}: DONE.valid is not true")
    if not all(done.get("checks", {}).values()):
        errors.append(f"ERROR {name}: a DONE check is false")
    protocol = result.get("protocol")
    if protocol not in {"single-c-companion", "single-c-companion-batched"}:
        errors.append(f"ERROR {name}: non-C-only protocol {protocol!r}")
        return errors, result
    expected = expected_boundaries(lib, tool)
    actual = result.get("boundaries", [])
    if actual != expected:
        errors.append(f"ERROR {name}: boundary order/set differs from archived-built funnel")
    if len(actual) != len(set(actual)):
        errors.append(f"ERROR {name}: duplicate boundaries")
    concurrent = result.get("max_concurrent_fuzzers", result.get("concurrent_fuzzers"))
    if not isinstance(concurrent, int) or not 0 < concurrent <= MAX_FUZZERS:
        errors.append(f"ERROR {name}: invalid recorded concurrency {concurrent!r}")
    params = result.get("campaign_params", result.get("campaign", {}))
    if params.get("max_total_time_s") != 3600:
        errors.append(f"ERROR {name}: campaign budget is {params.get('max_total_time_s')!r}, not 3600")
    replayed = sum(result.get("matrix", {}).get("new_c_guided", {}).get("C_inputs", {}).values())
    try:
        corpus = corpus_count(result)
    except (KeyError, TypeError, AttributeError) as exc:
        errors.append(f"ERROR {name}: cannot count corpus: {exc}")
        corpus = -1
    if replayed != corpus:
        errors.append(f"ERROR {name}: C replay {replayed} != corpus {corpus}")
    cside = result.get("matrix", {}).get("new_c_guided", {}).get("C", {})
    for kind in ("functions", "regions"):
        total, reached = cside.get(f"{kind}_total"), cside.get(f"{kind}_reached")
        if not isinstance(total, int) or total <= 0 or not isinstance(reached, int) or not 0 <= reached <= total:
            errors.append(f"ERROR {name}: invalid C {kind} counts {reached}/{total}")

    merged_f, merged_r = {}, {}
    if protocol == "single-c-companion-batched":
        batches = result.get("batches", [])
        flattened = []
        for row in batches:
            index = row["batch"]
            batch = final / "batches" / f"batch-{index:02d}"
            try:
                bdone = load(batch / "DONE.json")
                bresult = load(batch / "result.json")
                if bdone.get("valid") is not True or not all(bdone.get("checks", {}).values()):
                    errors.append(f"ERROR {name}/batch-{index:02d}: invalid DONE")
                if bresult.get("protocol") != "single-c-companion":
                    errors.append(f"ERROR {name}/batch-{index:02d}: non-C-only protocol")
                if bresult.get("boundaries") != row.get("boundaries"):
                    errors.append(f"ERROR {name}/batch-{index:02d}: aggregate boundary mismatch")
                flattened.extend(row.get("boundaries", []))
                ipath = batch / "c_identities.json.gz"
                if digest(ipath) != row.get("identity_sha256"):
                    errors.append(f"ERROR {name}/batch-{index:02d}: identity hash mismatch")
                bf, br = identities(ipath)
                merge_or(merged_f, bf); merge_or(merged_r, br)
                exports = exact_export_names(batch / "c_exports_c")
                if exports != sorted(row.get("boundaries", [])):
                    errors.append(f"ERROR {name}/batch-{index:02d}: C export names differ from boundaries")
                for sec in (60, 300, 600, 1800):
                    snap = batch / "snapshots" / f"c_snapshot@{sec}s.tar.gz"
                    with tarfile.open(snap, "r:gz") as tf:
                        if not tf.getmembers():
                            errors.append(f"ERROR {name}/batch-{index:02d}: empty {sec}s snapshot")
            except (FileNotFoundError, KeyError, OSError, tarfile.TarError, ValueError) as exc:
                errors.append(f"ERROR {name}/batch-{index:02d}: {exc}")
        if flattened != actual:
            errors.append(f"ERROR {name}: batch union/order differs from unit boundaries")
        try:
            af, ar = identities(final / "c_identities.json.gz")
            if af != merged_f or ar != merged_r:
                errors.append(f"ERROR {name}: aggregate identities are not batch Boolean union")
            if len(af) != cside.get("functions_total") or sum(af.values()) != cside.get("functions_reached"):
                errors.append(f"ERROR {name}: function identities disagree with result counts")
            if len(ar) != cside.get("regions_total") or sum(ar.values()) != cside.get("regions_reached"):
                errors.append(f"ERROR {name}: region identities disagree with result counts")
        except (FileNotFoundError, OSError, ValueError) as exc:
            errors.append(f"ERROR {name}: aggregate identities unreadable: {exc}")
    else:
        exports = exact_export_names(final / "c_exports_c")
        if exports != sorted(actual):
            errors.append(f"ERROR {name}: C export names differ from boundaries")
        for sec in (60, 300, 600, 1800):
            snap = final / "snapshots" / f"c_snapshot@{sec}s.tar.gz"
            try:
                with tarfile.open(snap, "r:gz") as tf:
                    if not tf.getmembers():
                        errors.append(f"ERROR {name}: empty {sec}s snapshot")
            except (FileNotFoundError, tarfile.TarError):
                errors.append(f"ERROR {name}: missing/invalid {sec}s snapshot")

    forbidden = []
    for path in final.rglob("*"):
        rel = path.relative_to(final)
        parts = {part.lower() for part in rel.parts}
        if "rust_exports" in parts or "rust_analysis" in parts or path.name == "RUST_MEASURE_DONE":
            forbidden.append(str(rel))
    if forbidden:
        errors.append(f"ERROR {name}: Rust remeasurement artifacts present: {forbidden[:3]}")
    return errors, result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive", type=Path, default=DEFAULT_ARCHIVE)
    parser.add_argument("--require-complete", action="store_true")
    args = parser.parse_args()
    errors, pending, rows = [], [], []
    for lib, tool in UNITS:
        messages, result = audit_unit(args.archive, lib, tool)
        for message in messages:
            (pending if message.startswith("PENDING") else errors).append(message)
        if result is not None:
            rows.append((f"{lib}_{tool}", len(result.get("boundaries", []))))
    supervisors, orphans = live_supervisors()
    if supervisors > MAX_FUZZERS:
        errors.append(f"ERROR live supervisor cap exceeded: {supervisors}/{MAX_FUZZERS}")
    if orphans:
        errors.append(f"ERROR orphan boundary processes: {orphans}")
    if not pending:
        if len(rows) != len(UNITS) or sum(n for _, n in rows) != 543:
            errors.append(f"ERROR final manifest cardinality: {len(rows)} units, {sum(n for _, n in rows)} boundaries")
        try:
            summary = load(args.archive / "SUMMARY.json")
            mdone = load(args.archive / "MANIFEST_DONE.json")
            if mdone.get("valid") is not True or not all(mdone.get("checks", {}).values()):
                errors.append("ERROR MANIFEST_DONE is invalid")
            if mdone.get("summary_sha256") != digest(args.archive / "SUMMARY.json"):
                errors.append("ERROR SUMMARY.json hash differs from MANIFEST_DONE")
            if summary.get("units_valid") != 12 or summary.get("boundaries") != 543:
                errors.append("ERROR SUMMARY cardinality is not 12 units / 543 boundaries")
        except (FileNotFoundError, json.JSONDecodeError) as exc:
            errors.append(f"ERROR final manifest summary missing/unreadable: {exc}")
    print(f"audited={len(rows)}/12 boundaries={sum(n for _, n in rows)}/543 live_supervisors={supervisors}")
    for message in pending + errors:
        print(message)
    if errors or (args.require_complete and pending):
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
