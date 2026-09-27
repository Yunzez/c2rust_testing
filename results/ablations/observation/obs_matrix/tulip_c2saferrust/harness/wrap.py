#!/usr/bin/env python3
"""OBS CLI wrapper driver (used identically for C and Rust `sample`).
usage: wrap.py CLI_BIN silent|print STATEFILE   (stdin = corpus record)
Decoding (shared by both sides): byte0 -> indicator index into indicators.txt (mod 104; 0xFF = NO ARGS at all);
byte1 -> k options in 0..4 (byte1 % 5); next k bytes -> option value 1..20 (b % 20 + 1) as decimal strings.
Runs CLI with that argv. State file written AFTER the CLI returns: 'ret:<exit code>\nglobals:none' (the CLI
boundary has no designated output memory -> O-S == O-R). print mode forwards the CLI stdout; silent discards it.
Wrapper exit code = CLI exit code; if the CLI dies by signal: no state file, exit 128+sig."""
import os, sys, subprocess
H = os.path.dirname(os.path.abspath(__file__))
NAMES = [l.strip() for l in open(os.path.join(H, "indicators.txt")) if l.strip()]
cli, mode, sf = sys.argv[1], sys.argv[2], sys.argv[3]
data = sys.stdin.buffer.read()
def decode(d):
    if len(d) == 0 or d[0] == 0xFF: return []
    argv = [NAMES[d[0] % len(NAMES)]]
    k = (d[1] % 5) if len(d) > 1 else 0
    for i in range(k):
        b = d[2 + i] if len(d) > 2 + i else 0
        argv.append(str(b % 20 + 1))
    return argv
argv = decode(data)
env = dict(os.environ); env["ASAN_OPTIONS"] = "detect_leaks=0:halt_on_error=1:exitcode=99"; env["UBSAN_OPTIONS"] = "print_stacktrace=1:halt_on_error=1:exitcode=99"  # FIX: abort_on_error=1 hung after the ASan report
try:
    p = subprocess.run([cli] + argv, capture_output=True, env=env, timeout=20)
except subprocess.TimeoutExpired as e:  # FIX: timeout -> no state file, exit 137, forward partial stderr
    sys.stderr.write((e.stderr or b"").decode("utf-8", "replace") + "\n[TIMEOUT]\n"); sys.exit(137)
sys.stderr.write(p.stderr.decode("utf-8", "replace"))
if p.returncode < 0:
    sys.exit(128 + (-p.returncode))
with open(sf, "w") as f:
    f.write(f"ret:{p.returncode}\nglobals:none\nargv:{' '.join(argv)}\n")
if mode == "print":
    sys.stdout.buffer.write(p.stdout); sys.stdout.flush()
sys.exit(p.returncode)
