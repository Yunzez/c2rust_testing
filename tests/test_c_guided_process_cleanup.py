#!/usr/bin/env python3
"""Regression test: campaign teardown must not orphan fork workers."""

import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts" / "rq4"))

import c_guided_cell as driver  # noqa: E402


def alive(pid: int) -> bool:
    try:
        state = Path(f"/proc/{pid}/stat").read_text().split()[2]
        return state != "Z"
    except (FileNotFoundError, ProcessLookupError, IndexError):
        return False


def main() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        child_pid_file = Path(tmp) / "child.pid"
        child_code = (
            "import signal,time; "
            "signal.signal(signal.SIGTERM, signal.SIG_IGN); "
            "time.sleep(60)"
        )
        leader_code = (
            "import pathlib,signal,subprocess,sys,time; "
            "signal.signal(signal.SIGTERM, signal.SIG_IGN); "
            "p=subprocess.Popen([sys.executable,'-c',sys.argv[2]]); "
            "pathlib.Path(sys.argv[1]).write_text(str(p.pid)); "
            "time.sleep(60)"
        )
        leader = subprocess.Popen(
            [sys.executable, "-c", leader_code, str(child_pid_file), child_code],
            start_new_session=True,
        )
        child_pid = None
        try:
            deadline = time.monotonic() + 5
            while time.monotonic() < deadline and not child_pid_file.exists():
                time.sleep(0.02)
            assert child_pid_file.exists(), "test child did not start"
            child_pid = int(child_pid_file.read_text())
            assert alive(leader.pid) and alive(child_pid)

            driver._terminate_process_group(leader, grace_s=0.1)
            deadline = time.monotonic() + 5
            while time.monotonic() < deadline and (alive(leader.pid) or alive(child_pid)):
                time.sleep(0.02)
            assert not alive(leader.pid), "supervisor survived group cleanup"
            assert not alive(child_pid), "fork worker was orphaned"
            assert not driver._process_group_exists(leader.pid), "private process group survived cleanup"
        finally:
            if driver._process_group_exists(leader.pid):
                os.killpg(leader.pid, 9)
            try:
                leader.wait(timeout=2)
            except subprocess.TimeoutExpired:
                leader.kill()
    print("PASS: campaign teardown reaps supervisor and fork workers")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
