#!/usr/bin/env python3
"""Durable controller for the frozen C-only application manifest.

This is the completion controller for ``docs/c_guided_c_only_plan.md``.  It
runs one C realization per frozen unit, partitions units with more than 28
archived-built boundaries into deterministic sequential batches, and unions
C coverage identities only after every batch has a valid result.

The controller deliberately has no Rust arm.  It expects each batch produced
by ``c_guided_cell.py --single-c-companion`` to contain ``c_identities.json.gz``;
that file is the lossless function/region identity set used for cross-batch
union.  A batch ``DONE.json`` without that artifact is not accepted.
"""
from __future__ import annotations

import argparse
import fcntl
import gzip
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_ARCHIVE = Path("/home/yunzez/c2rust_archive/cg_c_only_v3")
MAX_FUZZERS = 28

# One representative per distinct C realization, frozen in the binding plan.
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

EXPECTED_C_SHA_PREFIX = {
    ("bzip2", "c2rust"): "d4378051f781",
    ("cjson", "c2rust"): "aa1b902f9408",
    ("cjson", "ptrtrans"): "298581a04a36",
    ("genann", "c2rust"): "49128acbe4fa",
    ("genann", "sactor"): "df0dee7dc389",
    ("lil", "laertes"): "abd896337a4a",
    ("lodepng", "c2rust"): "19efa9e42f8a",
    ("optipng", "c2saferrust"): "493775c01168",
    ("qsort", "sactor"): "b857f308fbdb",
    ("quadtree", "c2rust"): "19871b2aaea0",
    ("tulip", "c2rust"): "d174602cb962",
    ("urlparser", "c2rust"): "68fc1ce6d2bb",
}


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def fsync_dir(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def atomic_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with open(tmp, "w") as fh:
        json.dump(value, fh, indent=1)
        fh.write("\n")
        fh.flush()
        os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def atomic_text(path: Path, value: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with open(tmp, "w") as fh:
        fh.write(value)
        fh.flush()
        os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def atomic_gzip_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    with gzip.open(tmp, "wt") as fh:
        json.dump(value, fh, separators=(",", ":"))
        fh.write("\n")
    with open(tmp, "rb") as fh:
        os.fsync(fh.fileno())
    os.replace(tmp, path)
    fsync_dir(path.parent)


def valid_done(path: Path) -> bool:
    try:
        return json.load(open(path))["valid"] is True
    except (FileNotFoundError, KeyError, json.JSONDecodeError):
        return False


def expected_boundaries(lib: str, tool: str) -> list[str]:
    funnel = ROOT / "results/rq3_coverage" / lib / tool / "funnel.json"
    rows = json.load(open(funnel))
    return [row["boundary"] for row in rows if row.get("built")]


def c_source_identity(lib: str, tool: str, boundaries: list[str]) -> dict:
    """Resolve and verify the archived compiled C source for a frozen unit."""
    cell = ROOT / "results/rq3_coverage" / lib / tool
    source_name = None
    copies = []
    for boundary in boundaries:
        build = cell / "harnesses" / boundary / "build.rs"
        if not build.exists():
            continue
        match = re.search(r'build\.file\("c/([^"\n]+)"\)', build.read_text(errors="replace"))
        if not match:
            continue
        name = match.group(1)
        if source_name is None:
            source_name = name
        elif name != source_name:
            raise RuntimeError(f"{lib}/{tool} uses multiple compiled C sources: {source_name}, {name}")
        source = cell / "harnesses" / boundary / "c" / name
        if source.exists():
            copies.append(source)
    if source_name is None:
        raise RuntimeError(f"cannot resolve archived C source for {lib}/{tool}")
    if not copies:
        pair_source = ROOT / "benchmark/pairs/rq4" / f"{lib}_{tool}" / "source"
        copies = list(pair_source.rglob(source_name))
    if not copies:
        raise RuntimeError(f"C source named by build.rs is absent for {lib}/{tool}: {source_name}")
    if len(copies) != 1 and not all(path.read_bytes() == copies[0].read_bytes() for path in copies[1:]):
        raise RuntimeError(f"cannot resolve a unique C source for {lib}/{tool}: {source_name}")
    hashes = {sha256(path) for path in copies}
    if len(hashes) != 1:
        raise RuntimeError(f"archived C source copies differ for {lib}/{tool}")
    full_hash = next(iter(hashes))
    expected = EXPECTED_C_SHA_PREFIX[(lib, tool)]
    if not full_hash.startswith(expected):
        raise RuntimeError(f"C source hash drift for {lib}/{tool}: {full_hash[:12]} != {expected}")
    return {"name": source_name, "sha256": full_hash, "copies_checked": len(copies)}


def chunks(items: list[str], size: int = MAX_FUZZERS) -> list[list[str]]:
    return [items[i:i + size] for i in range(0, len(items), size)]


def next_attempt(unit_root: Path, run_id: dict) -> Path:
    attempts = sorted(unit_root.glob("attempt-*"), key=lambda p: int(p.name.split("-")[-1]))
    # Resume only attempts created by this controller with byte-identical inputs.
    for attempt in reversed(attempts):
        manifest = attempt / "MANIFEST_RUN.json"
        if (attempt / "DONE.json").exists() or not manifest.exists():
            continue
        try:
            if json.load(open(manifest)) == run_id:
                return attempt
        except json.JSONDecodeError:
            pass
    number = max([int(p.name.split("-")[-1]) for p in attempts] or [0]) + 1
    return unit_root / f"attempt-{number}"


def read_identities(path: Path) -> tuple[dict[str, bool], dict[tuple, bool]]:
    with gzip.open(path, "rt") as fh:
        value = json.load(fh)
    functions = {row["id"]: bool(row["covered"]) for row in value["functions"]}
    regions = {tuple(row["id"]): bool(row["covered"]) for row in value["regions"]}
    return functions, regions


def merge_or(dst: dict, src: dict) -> None:
    for identity, covered in src.items():
        dst[identity] = dst.get(identity, False) or covered


def side(functions: dict, regions: dict) -> dict:
    return {
        "functions_total": len(functions),
        "functions_reached": sum(functions.values()),
        "regions_total": len(regions),
        "regions_reached": sum(regions.values()),
        "function_reach": sum(functions.values()) / len(functions) if functions else None,
        "region_reach": sum(regions.values()) / len(regions) if regions else None,
    }


def unit_run_id(lib: str, tool: str, boundaries: list[str], c_source: dict, files: dict,
                seconds: int | None) -> dict:
    return {
        "schema": 1,
        "unit": f"{lib}_{tool}",
        "protocol": "single-c-companion-batched",
        "boundaries": boundaries,
        "c_source": c_source,
        "batch_size": MAX_FUZZERS,
        "batches": chunks(boundaries),
        "budget_override_s": seconds,
        "files": files,
    }


def active_fuzzer_supervisors() -> list[dict]:
    """Return live libFuzzer fork supervisors before a new batch is launched.

    This controller owns at most one batch at a time.  Seeing a supervisor at
    the launch boundary therefore means some external run would share the CPU
    budget, so failing closed is safer than inferring how many workers it owns.
    """
    found = []
    proc = Path("/proc")
    for entry in proc.iterdir():
        if not entry.name.isdigit():
            continue
        try:
            cmd = entry.joinpath("cmdline").read_bytes().split(b"\0")
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            continue
        args = [part.decode("utf-8", "replace") for part in cmd if part]
        if any(arg == "-fork=1" for arg in args):
            found.append({"pid": int(entry.name), "argv": args})
    return sorted(found, key=lambda row: row["pid"])


def verify_driver_files(files: dict[str, str]) -> None:
    for relative, expected in files.items():
        path = ROOT / relative
        actual = sha256(path)
        if actual != expected:
            raise RuntimeError(f"driver file changed during manifest run: {relative}")


def aggregate_unit(attempt: Path, run_id: dict) -> tuple[dict, dict]:
    functions, regions = {}, {}
    all_boundaries, batch_rows = [], []
    input_counts: dict[str, int] = {}
    archived_c = None
    campaign_params = None
    max_concurrent = 0
    for index, expected in enumerate(run_id["batches"]):
        batch = attempt / "batches" / f"batch-{index:02d}"
        if not valid_done(batch / "DONE.json") or not (batch / "c_identities.json.gz").exists():
            raise RuntimeError(f"batch {index} is incomplete")
        result = json.load(open(batch / "result.json"))
        if result.get("boundaries") != expected:
            raise RuntimeError(f"batch {index} boundary mismatch")
        if result.get("protocol") != "single-c-companion":
            raise RuntimeError(f"batch {index} is not C-only")
        if not all(result.get("checks", {}).values()):
            raise RuntimeError(f"batch {index} failed its checks")
        bf, br = read_identities(batch / "c_identities.json.gz")
        merge_or(functions, bf)
        merge_or(regions, br)
        all_boundaries.extend(result["boundaries"])
        max_concurrent = max(max_concurrent, int(result["concurrent_fuzzers"]))
        for outcome, count in result["matrix"]["new_c_guided"]["C_inputs"].items():
            input_counts[outcome] = input_counts.get(outcome, 0) + count
        archived_c = archived_c or result["matrix"].get("archived_rust_guided_c_replay")
        params = {k: result["campaign"][k] for k in
                  ("max_total_time_s", "seed", "timeout_s", "rss_limit_mb", "max_len")}
        if campaign_params is None:
            campaign_params = params
        elif params != campaign_params:
            raise RuntimeError(f"batch {index} campaign parameters differ")
        batch_rows.append({
            "batch": index,
            "boundaries": result["boundaries"],
            "corpus_sizes": result["campaign"]["corpus_sizes"],
            "inputs": result["matrix"]["new_c_guided"]["C_inputs"],
            "seconds": result["seconds"],
            "identity_sha256": sha256(batch / "c_identities.json.gz"),
        })

    expected_all = run_id["boundaries"]
    checks = {
        "all_batches_valid": len(batch_rows) == len(run_id["batches"]),
        "boundary_union_exact": all_boundaries == expected_all,
        "fuzzer_cap_respected": max_concurrent <= MAX_FUZZERS,
        "only_c_was_fuzzed": True,
        "c_measurement_nonempty": bool(functions) and bool(regions),
    }
    result = {
        "schema": 4,
        "cell": run_id["unit"],
        "protocol": "single-c-companion-batched",
        "boundaries": expected_all,
        "batch_size": MAX_FUZZERS,
        "batch_count": len(batch_rows),
        "max_concurrent_fuzzers": max_concurrent,
        "campaign_params": campaign_params,
        "batches": batch_rows,
        "matrix": {
            "archived_rust_guided_c_replay": archived_c,
            "new_c_guided": {"C": side(functions, regions), "C_inputs": input_counts},
        },
        "checks": checks,
        "interpretation": "Only C was fuzzed and measured; coverage is the identity union of validated batches.",
    }
    identities = {
        "schema": 1,
        "functions": [{"id": key, "covered": value} for key, value in sorted(functions.items())],
        "regions": [{"id": list(key), "covered": value} for key, value in sorted(regions.items())],
    }
    return result, identities


def run_unit(lib: str, tool: str, archive: Path, driver: Path, controller_files: dict,
             seconds: int | None) -> Path:
    unit_root = archive / f"{lib}_{tool}"
    final = unit_root / "FINAL"
    if valid_done(final / "DONE.json"):
        return final
    boundaries = expected_boundaries(lib, tool)
    c_source = c_source_identity(lib, tool, boundaries)
    run_id = unit_run_id(lib, tool, boundaries, c_source, controller_files, seconds)
    attempt = next_attempt(unit_root, run_id)
    attempt.mkdir(parents=True, exist_ok=True)
    if not (attempt / "MANIFEST_RUN.json").exists():
        atomic_json(attempt / "MANIFEST_RUN.json", run_id)

    for index, batch_boundaries in enumerate(run_id["batches"]):
        batch = attempt / "batches" / f"batch-{index:02d}"
        if valid_done(batch / "DONE.json") and (batch / "c_identities.json.gz").exists():
            continue
        verify_driver_files(controller_files)
        active = active_fuzzer_supervisors()
        if active:
            atomic_json(attempt / "EXTERNAL_FUZZERS.json", {
                "at": time.strftime("%Y-%m-%dT%H:%M:%S"),
                "before_batch": index,
                "processes": active,
            })
            raise RuntimeError(f"refusing to start batch {index}: {len(active)} external fuzzer supervisors")
        batch.mkdir(parents=True, exist_ok=True)
        work = Path("/tmp") / f"cg_c_only_manifest_{lib}_{tool}_{index:02d}"
        cmd = [
            sys.executable, str(driver), "--lib", lib, "--tool", tool,
            "--work", str(work), "--out", str(batch),
            "--only", ",".join(batch_boundaries),
            "--single-c-companion", "--max-fuzzers", str(MAX_FUZZERS),
        ]
        if seconds is not None:
            cmd += ["--seconds", str(seconds)]
        with open(batch / "cell.log", "a") as log:
            proc = subprocess.run(cmd, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
        if proc.returncode != 0 or not valid_done(batch / "DONE.json"):
            raise RuntimeError(f"failed {lib}/{tool} batch {index}: {batch}")
        if not (batch / "c_identities.json.gz").exists():
            raise RuntimeError(f"batch {index} did not persist C identities")
        shutil.rmtree(work, ignore_errors=True)

    result, identities = aggregate_unit(attempt, run_id)
    atomic_gzip_json(attempt / "c_identities.json.gz", identities)
    atomic_json(attempt / "result.json", result)
    atomic_json(attempt / "DONE.json", {
        "at": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "valid": all(result["checks"].values()),
        "checks": result["checks"],
        "run_id": run_id,
    })
    if not valid_done(attempt / "DONE.json"):
        raise RuntimeError(f"aggregate checks failed for {lib}/{tool}")
    if final.exists():
        raise RuntimeError(f"refusing to replace existing {final}")
    os.replace(attempt, final)
    fsync_dir(unit_root)
    return final


def count_c_exports(final: Path, protocol: str) -> int:
    if protocol == "single-c-companion":
        return len(list((final / "c_exports_c").glob("*.json.gz")))
    if protocol == "single-c-companion-batched":
        return len(list((final / "batches").glob("batch-*/c_exports_c/*.json.gz")))
    return 0


def write_manifest_summary(archive: Path, units: list[tuple[str, str]]) -> dict:
    rows = []
    for lib, tool in units:
        final = archive / f"{lib}_{tool}" / "FINAL"
        if not valid_done(final / "DONE.json"):
            raise RuntimeError(f"missing valid FINAL for {lib}/{tool}")
        result = json.load(open(final / "result.json"))
        protocol = result.get("protocol")
        if protocol not in {"single-c-companion", "single-c-companion-batched"}:
            raise RuntimeError(f"non-C-only protocol for {lib}/{tool}: {protocol}")
        boundaries = result["boundaries"]
        exports = count_c_exports(final, protocol)
        if exports != len(boundaries):
            raise RuntimeError(f"C export count mismatch for {lib}/{tool}: {exports} != {len(boundaries)}")
        c_side = result["matrix"]["new_c_guided"]["C"]
        concurrent = (result.get("concurrent_fuzzers") if protocol == "single-c-companion"
                      else result.get("max_concurrent_fuzzers"))
        if concurrent is None or int(concurrent) > MAX_FUZZERS:
            raise RuntimeError(f"invalid concurrency record for {lib}/{tool}: {concurrent}")
        rows.append({
            "unit": f"{lib}_{tool}",
            "protocol": protocol,
            "boundaries": len(boundaries),
            "batches": result.get("batch_count", 1),
            "max_concurrent_fuzzers": int(concurrent),
            "c_exports": exports,
            "c_functions_reached": c_side["functions_reached"],
            "c_functions_total": c_side["functions_total"],
            "c_regions_reached": c_side["regions_reached"],
            "c_regions_total": c_side["regions_total"],
            "c_inputs": result["matrix"]["new_c_guided"]["C_inputs"],
            "final": str(final),
        })
    summary = {
        "schema": 1,
        "protocol": "C-only application manifest",
        "generated": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "units_expected": len(units),
        "units_valid": len(rows),
        "boundaries": sum(row["boundaries"] for row in rows),
        "max_concurrent_fuzzers": max(row["max_concurrent_fuzzers"] for row in rows),
        "rows": rows,
        "checks": {
            "all_units_valid": len(rows) == len(units),
            "only_c_protocols": all(row["protocol"].startswith("single-c-companion") for row in rows),
            "all_exports_exact": all(row["c_exports"] == row["boundaries"] for row in rows),
            "global_fuzzer_cap_respected": max(row["max_concurrent_fuzzers"] for row in rows) <= MAX_FUZZERS,
        },
    }
    lines = [
        "# C-guided application reach — completed manifest", "",
        "Only C was fuzzed and only C coverage was measured in these runs.", "",
        "| C realization | boundaries | batches | max fuzzers | C functions | C regions | inputs |",
        "|---|---:|---:|---:|---:|---:|---:|",
    ]
    for row in rows:
        inputs = sum(row["c_inputs"].values())
        lines.append(
            f"| {row['unit']} | {row['boundaries']} | {row['batches']} | {row['max_concurrent_fuzzers']} | "
            f"{row['c_functions_reached']}/{row['c_functions_total']} | "
            f"{row['c_regions_reached']}/{row['c_regions_total']} | {inputs} |"
        )
    lines += ["", f"Valid units: {len(rows)}/{len(units)}.  " ,
              f"Maximum concurrent boundary fuzzers: {summary['max_concurrent_fuzzers']}/{MAX_FUZZERS}.", ""]
    atomic_json(archive / "SUMMARY.json", summary)
    atomic_text(archive / "SUMMARY.md", "\n".join(lines))
    atomic_json(archive / "MANIFEST_DONE.json", {
        "at": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "valid": all(summary["checks"].values()),
        "checks": summary["checks"],
        "summary_sha256": sha256(archive / "SUMMARY.json"),
    })
    return summary


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--archive", default=str(DEFAULT_ARCHIVE))
    ap.add_argument("--units", help="comma-separated lib/tool subset; default is the frozen manifest")
    ap.add_argument("--seconds", type=int, help="smoke-only budget override")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    allowed = set(range(MAX_FUZZERS))
    os.sched_setaffinity(0, allowed)
    archive = Path(args.archive)
    archive.mkdir(parents=True, exist_ok=True)
    driver = ROOT / "scripts/rq4/c_guided_cell.py"
    controller = Path(__file__).resolve()
    files = {str(driver.relative_to(ROOT)): sha256(driver),
             str(controller.relative_to(ROOT)): sha256(controller)}
    selected = UNITS
    if args.units:
        wanted = set(args.units.split(","))
        selected = [unit for unit in UNITS if f"{unit[0]}/{unit[1]}" in wanted]
        unknown = wanted - {f"{lib}/{tool}" for lib, tool in selected}
        if unknown:
            raise SystemExit(f"not in frozen manifest: {sorted(unknown)}")

    queue = {
        "schema": 1,
        "created": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "protocol": "single C-only controller; deterministic batches; <=28 fuzzers",
        "files": files,
        "units": [],
    }
    queue_path = archive / "c_only_manifest_queue.json"
    if args.dry_run:
        for lib, tool in selected:
            bs = expected_boundaries(lib, tool)
            c_source = c_source_identity(lib, tool, bs)
            queue["units"].append({"unit": f"{lib}_{tool}", "boundaries": len(bs),
                                   "batch_sizes": [len(x) for x in chunks(bs)],
                                   "c_source": c_source})
        print(json.dumps(queue, indent=1))
        return 0

    lock = open(archive / ".manifest-controller.lock", "a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("another C-only manifest controller is active", file=sys.stderr)
        return 4

    for lib, tool in selected:
        row = {"unit": f"{lib}_{tool}", "status": "running",
               "started": time.strftime("%Y-%m-%dT%H:%M:%S")}
        queue["units"].append(row)
        atomic_json(queue_path, queue)
        try:
            final = run_unit(lib, tool, archive, driver, files, args.seconds)
        except Exception as exc:
            row.update(status="failed", error=str(exc), finished=time.strftime("%Y-%m-%dT%H:%M:%S"))
            atomic_json(queue_path, queue)
            raise
        row.update(status="final", final=str(final), finished=time.strftime("%Y-%m-%dT%H:%M:%S"))
        atomic_json(queue_path, queue)
    # A subset run is useful for a smoke or recovery but cannot certify the
    # full frozen manifest.  Only the default all-unit run writes completion.
    if selected == UNITS and args.seconds is None:
        summary = write_manifest_summary(archive, UNITS)
        if not all(summary["checks"].values()):
            raise RuntimeError("manifest summary checks failed")
    queue["completed"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    atomic_json(queue_path, queue)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
