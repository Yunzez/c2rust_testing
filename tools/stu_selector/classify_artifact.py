#!/usr/bin/env python3

"""P0+P1: artifact replay + conservative, evidence-backed divergence classifier.

For a fuzz artifact produced by a differential STU harness, this independently establishes WHAT
happened, instead of recording "an artifact exists". It:

  1. Replays the differential target on the artifact (>=2x) to confirm reproducibility and to
     read the primary outcome (harness comparison failed / Rust panic / crash).
  2. Runs the C entry ALONE on the same decoded input under UBSan+ASan -> sanitizer findings.
  3. Runs the Rust translation ALONE on the same input (overflow-checks on) -> panic or not.
  4. Applies explicit, conservative rules to emit one label. Anything uncertain -> UNKNOWN.

Label set. Auto-emitted by this classifier (conservative):
  C_UB_CONFIRMED  HARNESS_DIVERGENCE  RUST_PANIC  C_CRASH  NON_REPRODUCIBLE  UNKNOWN
Human-only (never auto-assigned — require human review or an independent oracle):
  TRANSLATION_BUG_CANDIDATE  TRANSLATION_BUG_CONFIRMED
A reproducible value difference with no C-UB and no crash is reported as the conservative
HARNESS_DIVERGENCE (cause not yet isolated: could be output normalization / harness semantics);
a human promotes it to TRANSLATION_BUG_CANDIDATE/CONFIRMED.

All three executions decode the artifact bytes with the SAME cursor semantics as the diff harness
(scalar = W little-endian bytes, 0-padded past end; vec = 1 length byte % 64, then elements), so
the same bytes map to the same arguments everywhere.

Output: a result.json (schema in RESULT_KEYS) with toolchain, compile flags, artifact sha256,
per-stage stderr tails and exit signals, plus the label and its evidence.

Usage:
  python3 tools/stu_selector/classify_artifact.py --pair benchmark/pairs/rpn_eval --entry rpn_eval \
      --artifact fuzz_gen/rpn_eval/fuzz/artifacts/rpn_eval_ft/<crash> --out result.json
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import gen_diff_harness as gdh  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
TOOLCHAIN = "nightly-2025-09-01"

RUST_TO_C = {
    "i8": "int8_t", "u8": "uint8_t", "i16": "int16_t", "u16": "uint16_t",
    "i32": "int32_t", "u32": "uint32_t", "i64": "int64_t", "u64": "uint64_t",
    "usize": "size_t", "bool": "_Bool",
}

UBSAN_RE = re.compile(r"runtime error: ([a-z][a-z0-9 \-]+)")


def sha256(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def tail(s: str, n: int = 1500) -> str:
    return s[-n:] if len(s) > n else s


# ---------- C-only driver (UBSan + ASan) ----------

def _c_call_args(abi: list[dict]) -> list[str]:
    """C call arguments in strict ABI order (mirrors the harness)."""
    call = []
    for p in abi:
        role, n = p["role"], p["name"]
        if role == "out_scalar":
            call.append(f"&{n}")
        else:  # scalar / length / capacity / *_buffer / input_string all pass by name
            call.append(n)
    return call


def gen_c_driver(items: list[dict], abi: list[dict], entry: str, source_abs: str) -> str:
    decode = []  # storage/decode in byte-consume order (from items)
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            ct = RUST_TO_C[it["rust"]]
            if it.get("decode") == "bounded_scalar":
                lo, hi = it["min_value"], it["max_value"]
                decode.append(f"    {ct} {n} = ({ct})({lo} + (cuint({it['w']}) % ({hi} - {lo} + 1)));")
            else:
                decode.append(f"    {ct} {n} = ({ct})cuint({it['w']});")
        elif it["role"] in ("in_buf", "io_buf"):
            ct = RUST_TO_C[it["elem"]]
            ln = it["len_name"]
            decode.append(f"    size_t {ln} = (size_t)(cb() % 64);")
            decode.append(f"    {ct}* {n} = ({ct}*)malloc(({ln} ? {ln} : 1) * sizeof({ct}));")
            decode.append(f"    for (size_t i = 0; i < {ln}; i++) {n}[i] = ({ct})cuint({it['elem_w']});")
        elif it["role"] == "out_scalar":
            ct = RUST_TO_C[it["elem"]]
            decode.append(f"    {ct} {n} = 0;")
        elif it["role"] == "in_str":
            ct = RUST_TO_C[it["elem"]]
            decode.append(f"    size_t {n}_len = (size_t)(cb() % 64);")
            decode.append(f"    {ct}* {n} = ({ct}*)malloc({n}_len + 1);")
            decode.append(f"    for (size_t i = 0; i < {n}_len; i++) {n}[i] = ({ct})cuint({it['elem_w']});")
            decode.append(f"    {n}[{n}_len] = 0;")
        elif it["role"] == "in_arr":
            ct = RUST_TO_C[it["elem"]]
            ext = it["inner_extent"]
            ln = it["len_name"]
            decode.append(f"    size_t {ln} = (size_t)(cb() % 64);")
            decode.append(f"    {ct} (*{n})[{ext}] = ({ct}(*)[{ext}])malloc(({ln} ? {ln} : 1) * {ext} * sizeof({ct}));")
            decode.append(f"    for (size_t i = 0; i < {ln}; i++) for (size_t j = 0; j < {ext}; j++) "
                          f"{n}[i][j] = ({ct})cuint({it['elem_w']});")
    call = _c_call_args(abi)
    return f'''#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "{source_abs}"

static unsigned char* DATA = 0; static size_t DLEN = 0, DP = 0;
static unsigned char cb(void) {{ unsigned char b = DP < DLEN ? DATA[DP] : 0; DP++; return b; }}
static unsigned long long cuint(int w) {{
    unsigned long long v = 0; for (int i = 0; i < w; i++) v |= ((unsigned long long)cb()) << (8*i); return v;
}}

int main(int argc, char** argv) {{
    if (argc < 2) return 2;
    FILE* f = fopen(argv[1], "rb"); if (!f) return 2;
    fseek(f, 0, SEEK_END); long sz = ftell(f); fseek(f, 0, SEEK_SET);
    DATA = (unsigned char*)malloc(sz > 0 ? sz : 1); DLEN = fread(DATA, 1, sz > 0 ? sz : 0, f); fclose(f);
{chr(10).join(decode)}
    (void){entry}({", ".join(call)});
    return 0;
}}
'''


def run_c(items, abi, entry, source_abs, artifact, workdir) -> dict:
    drv = workdir / "cdriver.c"
    drv.write_text(gen_c_driver(items, abi, entry, source_abs), encoding="utf-8")
    binp = workdir / "cdriver"
    flags = ["clang", "-g", "-O0", "-fsanitize=undefined,address",
             "-fno-omit-frame-pointer", str(drv), "-o", str(binp)]
    comp = subprocess.run(flags, capture_output=True, text=True)
    if comp.returncode != 0:
        return {"compiled": False, "stderr": tail(comp.stderr), "flags": " ".join(flags),
                "sanitizers": [], "crashed": False}
    env = dict(os.environ, UBSAN_OPTIONS="halt_on_error=0:print_stacktrace=0",
               ASAN_OPTIONS="halt_on_error=0:abort_on_error=0:detect_leaks=0")
    run = subprocess.run([str(binp), str(artifact)], capture_output=True, text=True, env=env)
    err = run.stderr
    sans = sorted(set(m.group(1).strip() for m in UBSAN_RE.finditer(err)))
    asan = "AddressSanitizer" in err
    return {"compiled": True, "flags": " ".join(flags), "sanitizers": sans,
            "asan_crash": asan, "exit": run.returncode, "stderr": tail(err)}


# ---------- Rust-only driver (overflow checks on) ----------

def run_rust(items, abi, entry, crate_dir: Path, crate_name: str, artifact, workdir) -> dict:
    """Build a tiny binary that depends on the translated crate and calls only the entry."""
    proj = workdir / "rustonly"
    (proj / "src").mkdir(parents=True, exist_ok=True)
    (proj / "Cargo.toml").write_text(f'''[package]
name = "rustonly"
version = "0.0.0"
edition = "2021"
[dependencies]
{crate_name} = {{ path = "{crate_dir}" }}
[[bin]]
name = "rustonly"
path = "src/main.rs"
[workspace]
''', encoding="utf-8")

    decode = []  # storage/decode in byte-consume order (from items)
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            if it.get("decode") == "bounded_scalar":
                lo, hi, ty = it["min_value"], it["max_value"], it["rust"]
                decode.append(f"    let {n} = ({lo} as {ty}) + (cur.take_{ty}() % (({hi} - {lo} + 1) as {ty}));")
            else:
                decode.append(f"    let {n} = cur.take_{it['rust']}();")
        elif it["role"] in ("in_buf", "io_buf"):
            decode.append(f"    let mut {n}: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}.len();")
        elif it["role"] == "out_scalar":
            decode.append(f"    let mut {n}: {it['elem']} = 0 as {it['elem']};")
        elif it["role"] == "in_str":
            decode.append(f"    let mut {n}: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    {n}.push(0 as {it['elem']});")
        elif it["role"] == "in_arr":
            ext = it["inner_extent"]
            decode.append(f"    let {n}_cnt = (cur.byte() as usize) % 64;")
            decode.append(f"    let mut {n}: Vec<[{it['elem']}; {ext}]> = Vec::with_capacity({n}_cnt);")
            decode.append(f"    for _ in 0..{n}_cnt {{ let mut a = [0 as {it['elem']}; {ext}];"
                          f" for j in 0..{ext} {{ a[j] = cur.take_{it['elem']}(); }} {n}.push(a); }}")
            decode.append(f"    let {it['len_name']} = {n}.len();")
    call = []  # call arguments in strict ABI order
    for p in abi:
        role, n = p["role"], p["name"]
        if role in ("input_buffer", "inout_buffer", "output_buffer"):
            call.append(f"{n}.as_mut_ptr()")
        elif role == "out_scalar":
            call.append(f"&mut {n}")
        elif role in ("input_string", "input_fixed_array_buffer"):
            call.append(f"{n}.as_ptr()")
        else:  # scalar / length / capacity
            call.append(n)
    takes = "\n".join(
        f"    fn take_{t}(&mut self) -> {t} {{ let mut v=[0u8;{w}]; for i in 0..{w}{{v[i]=self.byte();}} {t}::from_le_bytes(v) }}"
        for t, w in [("u8",1),("i8",1),("u16",2),("i16",2),("u32",4),("i32",4),("u64",8),("i64",8),("usize",8)])
    vtakes = "\n".join(
        f"    fn take_vec_{t}(&mut self) -> Vec<{t}> {{ let n=(self.byte() as usize)%64; (0..n).map(|_| self.take_{t}()).collect() }}"
        for t in ["u8","i8","u16","i16","u32","i32","u64","i64"])
    (proj / "src" / "main.rs").write_text(f'''use std::fs;
use {crate_name} as translated;
struct Cur {{ d: Vec<u8>, p: usize }}
impl Cur {{
    fn byte(&mut self) -> u8 {{ let b = if self.p < self.d.len() {{ self.d[self.p] }} else {{ 0 }}; self.p += 1; b }}
{takes}
{vtakes}
}}
fn main() {{
    let path = std::env::args().nth(1).unwrap();
    let d = fs::read(path).unwrap();
    let mut cur = Cur {{ d, p: 0 }};
{chr(10).join(decode)}
    unsafe {{ let _ = translated::{entry}({", ".join(call)}); }}
}}
''', encoding="utf-8")

    # Isolate this build's target dir: a shared CARGO_TARGET_DIR may be inherited from the
    # caller (e.g. the G1 matrix), which would put the binary somewhere we don't look.
    target = proj / "target"
    env = dict(os.environ, RUSTFLAGS="-Cdebug-assertions=on -Coverflow-checks=on",
               CARGO_TARGET_DIR=str(target))
    env["PATH"] = os.path.expanduser("~/.cargo/bin") + ":" + env.get("PATH", "")
    build = subprocess.run(["cargo", f"+{TOOLCHAIN}", "build", "-q"], cwd=proj,
                           capture_output=True, text=True, env=env)
    if build.returncode != 0:
        return {"built": False, "stderr": tail(build.stderr)}
    binp = target / "debug" / "rustonly"
    run = subprocess.run([str(binp), str(artifact)], capture_output=True, text=True, env=env)
    panicked = "panicked at" in run.stderr
    msg = ""
    m = re.search(r"panicked at [^\n]*\n([^\n]*)", run.stderr)
    if m:
        msg = m.group(1).strip()
    return {"built": True, "panicked": panicked, "message": msg,
            "exit": run.returncode, "stderr": tail(run.stderr)}


# ---------- diff target replay ----------

def replay_diff(crate_dir: Path, crate_name: str, artifact, runs: int = 2) -> dict:
    # The diff binary may live under the per-crate fuzz/target or a shared CARGO_TARGET_DIR.
    search_dirs = [crate_dir / "fuzz" / "target"]
    if os.environ.get("CARGO_TARGET_DIR"):
        search_dirs.append(Path(os.environ["CARGO_TARGET_DIR"]))
    binp = None
    for d in search_dirs:
        if d.exists():
            binp = next((b for b in d.rglob(f"{crate_name}_ft") if b.is_file() and os.access(b, os.X_OK)), None)
            if binp:
                break
    if binp is None or not binp.exists():
        return {"available": False}
    env = dict(os.environ, PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""))
    outcomes = []
    last = ""
    for _ in range(runs):
        r = subprocess.run([str(binp), str(artifact)], capture_output=True, text=True, env=env)
        err = r.stderr
        last = err
        if re.search(r"panicked at[^\n]*src/lib\.rs", err):
            outcomes.append("rust_panic")
        elif "divergence:" in err:
            outcomes.append("harness_divergence")
        elif "AddressSanitizer" in err or "SEGV" in err or r.returncode in (-11, 139, -6, 134):
            outcomes.append("crash")
        elif "panicked at" in err:
            outcomes.append("other_panic")
        else:
            outcomes.append("clean")
    reproducible = len(set(outcomes)) == 1 and outcomes[0] != "clean"
    return {"available": True, "outcomes": outcomes, "reproducible": reproducible,
            "outcome": outcomes[0], "stderr": tail(last)}


def classify(diff: dict, c: dict, rust: dict) -> tuple[str, str]:
    if diff.get("available") and not diff.get("reproducible"):
        return "NON_REPRODUCIBLE", f"diff replay outcomes varied: {diff.get('outcomes')}"
    outcome = diff.get("outcome") if diff.get("available") else None
    ubsan = c.get("sanitizers", [])
    rust_panicked = rust.get("panicked") or outcome == "rust_panic"

    if ubsan and (rust_panicked or outcome in ("harness_divergence", "rust_panic", "crash")):
        return "C_UB_CONFIRMED", f"C UBSan reported {ubsan}; rust/diff outcome={outcome}"
    if c.get("asan_crash"):
        return "C_CRASH", "C AddressSanitizer reported a memory error on the same input"
    if outcome == "crash":
        return "C_CRASH", "diff replay crashed (C called first); no UBSan/ASan signal captured -> conservative C_CRASH"
    if outcome == "rust_panic" or rust.get("panicked"):
        return "RUST_PANIC", f"Rust translation panicked ({rust.get('message','')}) with no C UB found"
    if outcome == "harness_divergence":
        return ("HARNESS_DIVERGENCE",
                "outputs differ; no C UB and no crash found — cause not isolated (could be output "
                "normalization / harness semantics). Human review to promote to TRANSLATION_BUG_CANDIDATE.")
    return "UNKNOWN", f"insufficient evidence (diff={outcome}, ubsan={ubsan}, rust={rust})"


def toolchain_info() -> dict:
    def v(cmd):
        try:
            return subprocess.run(cmd, capture_output=True, text=True).stdout.strip().splitlines()[0]
        except Exception:
            return "?"
    return {
        "clang": v(["clang", "--version"]),
        "rustc": v(["rustc", f"+{TOOLCHAIN}", "--version"]),
        "c2rust": v([os.path.expanduser("~/.cargo/bin/c2rust"), "--version"]),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="Replay + conservatively classify a fuzz artifact")
    ap.add_argument("--pair", required=True)
    ap.add_argument("--entry", required=True)
    ap.add_argument("--artifact", required=True)
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    pair = Path(args.pair)
    name = pair.name
    crate_name = name.replace("-", "_")
    cc = pair / "build"
    source = next((pair / "source").glob("*.c"))
    artifact = Path(args.artifact)
    crate_dir = ROOT / "fuzz_gen" / name

    params, ret, _, items, abi = gdh.resolve(name, cc, args.entry)

    with tempfile.TemporaryDirectory() as td:
        work = Path(td)
        c_res = run_c(items, abi, args.entry, str(source.resolve()), artifact, work)
        rust_res = (run_rust(items, abi, args.entry, crate_dir, crate_name, artifact, work)
                    if crate_dir.exists() else {"built": False, "stderr": "no translated crate dir"})
    diff_res = replay_diff(crate_dir, crate_name, artifact)
    label, evidence = classify(diff_res, c_res, rust_res)

    result = {
        "program": name, "entry": args.entry,
        "artifact": str(artifact), "artifact_sha256": sha256(artifact),
        "toolchain": toolchain_info(),
        "diff_replay": diff_res,
        "c_run": c_res,
        "rust_run": rust_res,
        "label": label,
        "evidence": evidence,
    }
    text = json.dumps(result, indent=2)
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
    print(text)
    print(f"\n==> {name}/{args.entry}: {label}  ({evidence})", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
