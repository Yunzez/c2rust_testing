#!/usr/bin/env python3
"""RQ1b independent equivalence/reach oracle (DoD #5): decide the recall DENOMINATOR.

For each mutation, determine WITHOUT our fuzzer whether it is a valid recall target:
a mutation counts only if SOME UB-free input makes C and mutated-Rust differ. We compile
the C entry (renamed c_<entry> via -D so it does not clash with the Rust no_mangle symbol)
and the mutated Rust translation (staticlib), link both into ONE Rust binary that sweeps a
FIXED, structured grid of interesting inputs and tallies divergences. This is independent of
libFuzzer's coverage-guided search (the potential circularity in a recall study).

All M2 base entries are UB-free over their WHOLE input domain (guarded/masked/wrapping by
construction), so no per-input UB filter is needed here; every sampled input is a legal
comparison. Output per mutation:
  valid   = divergences > 0  (a real recall target)
  equivalent = divergences == 0 over the whole grid
  density = divergences / sampled  (high -> easily fuzzable; ~0 -> boundary-only; explains misses)

Usage: mut_equiv_oracle.py --muts scripts/mut_m2.json --json results/ablations/attribution/mut_rows/m2_oracle.json
"""
from __future__ import annotations
import argparse, json, re, subprocess, shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLCHAIN = "nightly-2025-09-01"
# c2rust rust type -> (primitive rust, C type) for extern decls
TYMAP = {"int32_t": ("i32", "int32_t"), "uint32_t": ("u32", "uint32_t"),
         "int64_t": ("i64", "int64_t"), "uint64_t": ("u64", "uint64_t"),
         "size_t": ("usize", "size_t"), "ssize_t": ("isize", "ssize_t"),
         "::core::ffi::c_int": ("i32", "int"), "c_int": ("i32", "int")}
# structured, fixed sample sets per primitive (interesting values + a few spread points)
SAMPLE = {
    "i32": [-2147483648, -2147483647, -100, -3, -2, -1, 0, 1, 2, 3, 100, 65535,
            2147483646, 2147483647, 12345, -54321, 1000000, -1000000],
    "u32": [0, 1, 2, 3, 15, 16, 31, 32, 63, 100, 255, 65535, 0x7fffffff,
            0x80000000, 0xffffffff, 12345, 3000000000, 0xdeadbeef],
    "i64": [-9223372036854775808, -1, 0, 1, 2, 100, 9223372036854775807, 123456789],
    "u64": [0, 1, 2, 3, 7, 8, 100, 255, 1000000007, 4294967296, 18446744073709551615,
            123456789, 999999937, 2],
    "usize": [0, 1, 2, 8, 100],
}
# op-selector args (first c_int of a dispatch entry) get a small exact set incl. all valid arms
OP_SAMPLE = [0, 1, 2, 3, 4, -1, 5]


def parse_sig(rs_text: str, entry: str):
    m = re.search(rf'extern "C" fn {re.escape(entry)}\(([^)]*)\)\s*->\s*([\w:]+)', rs_text)
    if not m:
        return None
    args = []
    for p in m.group(1).split(","):
        p = p.strip()
        if not p:
            continue
        nm = p.split(":")[0].replace("mut", "").strip()
        ty = p.split(":", 1)[1].strip()
        args.append((nm, ty))
    return args, m.group(2).strip()


def apply_patch(rs: Path, find: str, replace: str) -> bool:
    txt = rs.read_text()
    if txt.count(find) != 1:
        return False
    rs.write_text(txt.replace(find, replace, 1))
    return True


def gen_sweep_main(entry, args, ret_prim):
    decls_args = ", ".join(f"{n}: {TYMAP[t][0]}" for n, t in args)
    ret_p = TYMAP[ret_prim][0]
    externs = (f'extern "C" {{\n'
               f'    fn {entry}({decls_args}) -> {ret_p};\n'
               f'    fn c_{entry}({decls_args}) -> {ret_p};\n}}\n')
    # nested loops over each arg's sample set
    loops, closes, call_args = "", "", []
    for i, (n, t) in enumerate(args):
        prim = TYMAP[t][0]
        # first c_int arg of a dispatch entry -> op sample; else the primitive's sample
        is_op = (i == 0 and t in ("::core::ffi::c_int", "c_int"))
        vals = OP_SAMPLE if is_op else SAMPLE[prim]
        arr = ", ".join(f"{v}{prim}" for v in vals)
        loops += f'{"    "*(i+1)}for &{n} in &[{arr}] {{\n'
        closes = f'{"    "*(i+1)}}}\n' + closes
        call_args.append(n)
    ca = ", ".join(call_args)
    body = (f'{loops}'
            f'{"    "*(len(args)+1)}total += 1;\n'
            f'{"    "*(len(args)+1)}let rc = unsafe {{ c_{entry}({ca}) }};\n'
            f'{"    "*(len(args)+1)}let rr = unsafe {{ {entry}({ca}) }};\n'
            f'{"    "*(len(args)+1)}if rc != rr {{ diverge += 1; }}\n'
            f'{closes}')
    return (externs +
            "fn main() {\n    let mut total: u64 = 0;\n    let mut diverge: u64 = 0;\n" +
            body +
            '    println!("{} {}", diverge, total);\n}\n')


def run_oracle(m, workdir):
    mid, entry = m["id"], m["entry"]
    src_pair = ROOT / m["pair"]
    d = workdir / f"orc_{mid}"
    if d.exists():
        shutil.rmtree(d)
    d.mkdir(parents=True)
    rs = d / "mutated.rs"
    shutil.copy(sorted((src_pair / "translated").glob("*.rs"))[0], rs)
    if not apply_patch(rs, m["find"], m["replace"]):
        return {"id": mid, "oracle": "PATCH_FAIL"}
    sig = parse_sig(rs.read_text(), entry)
    if sig is None:
        return {"id": mid, "oracle": "SIG_FAIL"}
    args, ret = sig
    if any(t not in TYMAP for _n, t in args) or ret not in TYMAP:
        return {"id": mid, "oracle": "UNSUPPORTED_SIG", "detail": str(sig)}
    # 1) mutated Rust -> staticlib
    lib = d / "libmutated.a"
    r = subprocess.run(["rustc", f"+{TOOLCHAIN}", "--edition", "2021", "--crate-type", "staticlib",
                        "--crate-name", "mutated", "-A", "warnings",
                        "-C", "opt-level=2", "-C", "overflow-checks=off",  # wrap like release, not debug-panic
                        "-o", str(lib), str(rs)],
                       text=True, capture_output=True)
    if r.returncode != 0:
        return {"id": mid, "oracle": "RUST_BUILD_FAIL", "detail": r.stderr[-200:]}
    # 2) C entry -> object. Rename EVERY exported (no_mangle) symbol to c_<name>: c2rust emits a whole
    # family of no_mangle fns and the Rust staticlib exports all of them, so renaming only the entry
    # leaves the siblings colliding at link. `pub ... extern "C" fn NAME` == the exported set.
    exported = set(re.findall(r'pub\s+unsafe\s+extern "C" fn (\w+)', rs.read_text()))
    exported.add(entry)
    cobj = d / "c.o"
    csrcs = [str(c) for c in sorted((src_pair / "source").glob("*.c"))]
    renames = [f"-D{n}=c_{n}" for n in sorted(exported)]
    r = subprocess.run(["clang", *renames, "-O1", "-c", *csrcs, "-o", str(cobj)],
                       text=True, capture_output=True)
    if r.returncode != 0:
        return {"id": mid, "oracle": "C_BUILD_FAIL", "detail": r.stderr[-200:]}
    # 3) sweep binary links both
    (d / "sweep.rs").write_text(gen_sweep_main(entry, args, ret))
    sweep = d / "sweep"
    # opt-level=2 + overflow-checks=off so plain arithmetic WRAPS like the release fuzz build (and C),
    # instead of debug-mode overflow panics aborting the sweep. Genuine hard-trap UB (e.g. INT_MIN/-1
    # division) still traps and is caught as "crashes on some input" -> valid/non-equivalent.
    r = subprocess.run(["rustc", f"+{TOOLCHAIN}", "--edition", "2021", "-A", "warnings",
                        "-C", "opt-level=2", "-C", "overflow-checks=off",
                        "-o", str(sweep), str(d / "sweep.rs"),
                        "-L", str(d), "-l", "static=mutated",
                        "-C", f"link-arg={cobj}", "-C", "link-arg=-lm"],
                       text=True, capture_output=True)
    if r.returncode != 0:
        return {"id": mid, "oracle": "LINK_FAIL", "detail": r.stderr[-300:]}
    try:
        run = subprocess.run([str(sweep)], text=True, capture_output=True, timeout=60)
    except subprocess.TimeoutExpired:
        shutil.rmtree(d, ignore_errors=True)
        return {"id": mid, "oracle": "valid", "diverge": -1, "total": -1, "density": None,
                "reach_hint": m.get("reach"), "detail": "non-terminating on some input (divergent)"}
    shutil.rmtree(d, ignore_errors=True)
    if run.returncode != 0:
        # a mutant that crashes/hangs on some input (e.g. stack overflow from non-termination) is
        # non-equivalent by definition: it diverges from the C reference on that input.
        why = "stack overflow" if "stack overflow" in run.stderr else run.stderr[-160:]
        return {"id": mid, "oracle": "valid", "diverge": -1, "total": -1, "density": None,
                "reach_hint": m.get("reach"), "detail": f"crashes on some input ({why})"}
    diverge, total = (int(x) for x in run.stdout.split())
    return {"id": mid, "oracle": "equivalent" if diverge == 0 else "valid",
            "diverge": diverge, "total": total,
            "density": round(diverge / total, 4) if total else 0.0,
            "reach_hint": m.get("reach")}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--muts", required=True)
    ap.add_argument("--json", default=None)
    ap.add_argument("--workdir", default="/tmp/claude-1000/-home-yunzez-c2rust-testing/"
                    "1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/mutorc")
    args = ap.parse_args()
    workdir = Path(args.workdir)
    workdir.mkdir(parents=True, exist_ok=True)
    muts = json.loads(Path(args.muts).read_text())
    rows = []
    print(f"{'mutation':22} {'oracle':12} {'diverge/total':>16} {'density':>9} reach")
    print("-" * 72)
    for m in muts:
        r = run_oracle(m, workdir)
        rows.append(r)
        dt = f"{r.get('diverge','-')}/{r.get('total','-')}"
        print(f"{r['id']:22} {r['oracle']:12} {dt:>16} {str(r.get('density','-')):>9} "
              f"{str(r.get('reach_hint','-'))}  {r.get('detail','')}")
    valid = sum(1 for r in rows if r["oracle"] == "valid")
    equiv = sum(1 for r in rows if r["oracle"] == "equivalent")
    print("-" * 72)
    print(f"valid (non-equivalent): {valid}   equivalent: {equiv}   other: {len(rows)-valid-equiv}")
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(rows, indent=1))
        print(f"wrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
