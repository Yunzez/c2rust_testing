#!/usr/bin/env python3
"""Durable, strictly serial controller for the pre-registered guidance pilots."""
import argparse
import fcntl
import hashlib
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_ARCHIVE = Path("/home/yunzez/c2rust_archive/cg_controlled")
PILOTS = [
    ("qsort", "c2rust", "rust,c"),
    ("lodepng", "c2rust", "c,rust"),
    ("lil", "c2saferrust", "rust,c"),
    ("tulip", "c2rust", "c,rust"),
]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def atomic_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    with open(tmp, "w") as fh:
        json.dump(value, fh, indent=1)
        fh.write("\n"); fh.flush(); os.fsync(fh.fileno())
    os.replace(tmp, path)
    fd = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def next_attempt(cell_root: Path) -> Path:
    attempts = sorted(cell_root.glob("attempt-*"), key=lambda p: int(p.name.split("-")[-1]))
    for attempt in reversed(attempts):
        if not (attempt / "DONE.json").exists() and any(attempt.glob("*_CAMPAIGN_DONE")):
            return attempt
    number = max([int(p.name.split("-")[-1]) for p in attempts] or [0]) + 1
    return cell_root / f"attempt-{number}"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--archive", default=str(DEFAULT_ARCHIVE))
    ap.add_argument("--seconds", type=int, help="smoke-only override; omit for archived full budget")
    ap.add_argument("--cells", help="comma-separated lib/tool subset of the frozen pilot list")
    args = ap.parse_args()
    archive = Path(args.archive)
    archive.mkdir(parents=True, exist_ok=True)
    lock = open(archive / ".controller.lock", "a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("another controlled-guidance controller is active", file=sys.stderr)
        return 4

    selected = PILOTS
    if args.cells:
        wanted = set(args.cells.split(","))
        selected = [x for x in PILOTS if f"{x[0]}/{x[1]}" in wanted]
        unknown = wanted - {f"{x[0]}/{x[1]}" for x in selected}
        if unknown:
            print(f"not in frozen pilot manifest: {sorted(unknown)}", file=sys.stderr)
            return 2
    driver = ROOT / "scripts/rq4/c_guided_cell.py"
    controller = Path(__file__).resolve()
    queue_path = archive / ("smoke_queue.json" if args.seconds is not None else "formal_queue.json")
    queue = {
        "schema": 1,
        "created": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "policy": "strictly serial; one cell owns the controller; arms sequential",
        "seconds_override": args.seconds,
        "files": {str(driver.relative_to(ROOT)): sha256(driver),
                  str(controller.relative_to(ROOT)): sha256(controller)},
        "cells": [{"lib": lib, "tool": tool, "arm_order": order, "status": "pending"}
                  for lib, tool, order in selected],
    }
    atomic_json(queue_path, queue)

    for index, (lib, tool, order) in enumerate(selected):
        record = queue["cells"][index]
        cell_root = archive / f"{lib}_{tool}"
        final = cell_root / "FINAL"
        if (final / "DONE.json").exists() and json.load(open(final / "DONE.json")).get("valid"):
            record["status"] = "already-final"
            atomic_json(queue_path, queue)
            continue
        attempt = next_attempt(cell_root)
        attempt.mkdir(parents=True, exist_ok=True)
        record.update(status="running", attempt=attempt.name, started=time.strftime("%Y-%m-%dT%H:%M:%S"))
        atomic_json(queue_path, queue)
        work = Path("/tmp") / f"cg_controlled_{lib}_{tool}"
        cmd = [sys.executable, str(driver), "--lib", lib, "--tool", tool,
               "--work", str(work), "--out", str(attempt), "--arm-order", order]
        if args.seconds is not None:
            cmd += ["--seconds", str(args.seconds)]
        with open(attempt / "cell.log", "a") as log:
            proc = subprocess.run(cmd, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
        record.update(returncode=proc.returncode, finished=time.strftime("%Y-%m-%dT%H:%M:%S"))
        done_path = attempt / "DONE.json"
        valid = done_path.exists() and json.load(open(done_path)).get("valid") is True
        if proc.returncode != 0 or not valid:
            record["status"] = "failed"
            atomic_json(queue_path, queue)
            print(f"stopping after failed cell {lib}/{tool}: {attempt}", file=sys.stderr)
            return proc.returncode or 3
        if final.exists():
            record["status"] = "failed-final-exists"
            atomic_json(queue_path, queue)
            return 3
        os.replace(attempt, final)
        record["status"] = "final"
        record["final"] = str(final)
        atomic_json(queue_path, queue)
        print(f"FINAL {lib}/{tool}", flush=True)
    queue["completed"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    atomic_json(queue_path, queue)
    print("CONTROLLED_GUIDANCE_QUEUE_DONE", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
