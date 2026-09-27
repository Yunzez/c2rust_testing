#!/usr/bin/env python3
"""Seed IR round-trip regression (docs/seeding_policy_plan.md section 2): for boundaries covering every node
shape, ENCODE requested values with seed_ir, run the real harness built with --decode-dump, and require the
values it decodes to equal the requested ones exactly (floats by bits), and seed_ir.decode() to agree.
usage: test_seed_ir_roundtrip.py [--work DIR] [--only case,...]   (exit 1 on any mismatch)"""
import argparse, json, os, pathlib, random, re, shutil, struct, subprocess, sys
ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector")); sys.path.insert(0, str(ROOT / "scripts")); sys.path.insert(0, str(ROOT / "scripts" / "rq4"))
import seed_ir as I, c2r_funnel as F, cell as CELL

CASES = [  # (pair, boundary, c_source): the shapes named in the plan
    ("tulip_c2rust", "ti_bbands", "tulip.c"),            # bounded scalar; 1 fuzz row 4096 f64; options Repeat 2; zero rows; inout row
    ("tulip_c2rust", "ti_sma", "tulip.c"),               # bounded scalar; row; options Repeat 1
    ("bzip2_c2rust", "BZ2_bzBuffToBuffCompress", "bzip2lib.c"),  # 3 bounded ints first, then opaque input_buffer, out_buf_cap
    ("lodepng_c2rust", "lodepng_chunk_length", "lodepng.c"),      # Repeat const 4 of u8
    ("qsort_c2rust", "quickSort", "qsort.c"),            # bounded scalars + param-extent Repeat
    ("quadtree_c2rust", "quadtree_insert", "quadtree_all.c"),     # f64 scalars then null then opaque produced object
    ("genann_c2rust", "genann_run", "genann.c"),         # opaque produced object FIRST: nothing placeable (negative case)
]

def build(pair, b, c_source, work):
    class A: pass
    a = A(); a.c_source = c_source; a.plugins = None; a.shim = str(ROOT / "benchmark" / "pairs" / "rq4" / "darwin_shims.c")
    dj = next(pair.glob("translated/*.defs.json"), None); a.defs = str(dj) if dj else None; defs = json.load(open(dj)) if dj else {}
    out = work / b; shutil.rmtree(out, ignore_errors=True)
    cmd = [sys.executable, str(ROOT / "tools/stu_selector/gen_diff_harness.py"), "--pair", str(pair), "--entry", b, "--rust-entry", F.rust_name(pair, b),
           "--plan", "--ub-free", "--c-source", c_source, "--out", str(out), "--decode-dump"]
    r = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True); assert r.returncode == 0, r.stderr[-500:]
    err = F.fixups(a, pair, out, b, False, defs); assert not err, err
    env = dict(os.environ, CARGO_TARGET_DIR=str(work / "target"), RUSTUP_TOOLCHAIN=CELL.TOOLCHAIN)
    rb = subprocess.run(["cargo", "fuzz", "build"], cwd=str(out), env=env, capture_output=True, text=True); assert rb.returncode == 0, "\n".join(l for l in rb.stderr.splitlines() if l.startswith("error"))[:800]
    return next((work / "target" / "x86_64-unknown-linux-gnu" / "release").glob("*_ft"))

def dump(binp, data, work):
    f = work / "in.bin"; f.write_bytes(data)
    r = subprocess.run([str(binp), "-runs=1", str(f)], env=dict(os.environ, C2R_MODE="rust-only", ASAN_OPTIONS="detect_leaks=0"), capture_output=True, text=True, timeout=60)
    out = {}
    for m in re.finditer(r"^DUMP (\S+) (.*)$", r.stderr, re.M):
        out[m.group(1)] = m.group(2)
    assert "DUMP_END" in r.stderr, r.stderr[-600:]
    return out

def expected_repr(nd, v):
    if isinstance(nd, I.Scalar):
        return str(struct.unpack("<Q", struct.pack("<d", float(v)))[0] if nd.width == 8 else struct.unpack("<I", struct.pack("<f", float(v)))[0]) if nd.float else str(int(v))
    vals = [struct.unpack("<Q", struct.pack("<d", float(x)))[0] if nd.width == 8 else struct.unpack("<I", struct.pack("<f", float(x)))[0] for x in v] if nd.float else [int(x) for x in v]
    return "[" + ", ".join(str(x) for x in vals) + "]"

def trials(nodes):
    """assignment sets covering min/max/mid of bounded scalars, 0/-1/odd values, arrays of every extent"""
    pre = I.placeable(nodes); sets = []
    for k in range(4):
        a = {}
        for nd in pre:
            if isinstance(nd, I.Scalar):
                if nd.float: a[nd.name] = [0.0, -1.5, 3.25, float("nan")][k]
                elif nd.bounds: lo, hi = nd.bounds; a[nd.name] = [lo, hi, (lo + hi) // 2, lo + 1 if lo + 1 <= hi else hi][k]
                else: a[nd.name] = [0, -1 if nd.signed else 1, 7, 2 ** (8 * nd.width - 1) - 1][k]
            elif isinstance(nd, I.Repeat) and nd.count.kind == "const" and nd.count.value <= 8:
                n = nd.count.value; a[nd.name] = [[(i * 3 + k) * (0.5 if nd.float else 1) for i in range(n)], [(-1 if nd.signed or nd.float else 1)] * n, [0] * n, [(9 if not nd.float else 9.5)] * n][k]
        sets.append(a)
    return sets

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--work", default="/tmp/seed_ir_rt"); ap.add_argument("--only"); a = ap.parse_args()
    work = pathlib.Path(a.work); work.mkdir(parents=True, exist_ok=True); bad = 0; total = 0
    for pair_name, b, c_source in CASES:
        if a.only and b not in a.only.split(","): continue
        pair = ROOT / "benchmark" / "pairs" / "rq4" / pair_name
        plans = json.load(open(work / f"{pair_name}.plans.json")) if (work / f"{pair_name}.plans.json").exists() else None
        if plans is None:
            (work / f"plan_{pair_name}").mkdir(parents=True, exist_ok=True); plans = F.plan_all(pair, work / f"plan_{pair_name}"); (work / f"{pair_name}.plans.json").write_text(json.dumps(plans))
        entry = next(p for p in plans if p["boundary"] == b); nodes = I.lower(entry, I.load_aliases(pair), I.param_order(pair, b)); pre = I.placeable(nodes)
        binp = build(pair, b, c_source, work)
        print(f"== {pair_name}/{b}: {len(pre)} placeable of {len(nodes)} nodes; first opaque: {next((n.name+' ('+n.why.split(' (')[0]+')' for n in nodes if isinstance(n, I.Opaque)), None)}")
        for k, asg in enumerate(trials(nodes)):
            rng = random.Random(f"rt:{b}:{k}"); data, layout = I.encode(nodes, asg, rng); got = dump(binp, data, work); ref = I.decode(nodes, data)
            for nd in pre:
                if not isinstance(nd, (I.Scalar, I.Repeat)) or nd.name not in asg: continue
                total += 1; exp = expected_repr(nd, asg[nd.name]); g = got.get(nd.name)
                refv = ref.get(nd.name); ref_ok = (struct.pack("<d", refv) == struct.pack("<d", float(asg[nd.name])) if isinstance(nd, I.Scalar) and nd.float else refv == asg[nd.name]) if not isinstance(refv, list) else len(refv) == len(asg[nd.name])
                if g != exp or not ref_ok:
                    bad += 1; print(f"   MISMATCH trial {k} {nd.name}: requested {asg[nd.name]!r} -> harness {g!r} (expected {exp!r}); seed_ir.decode {refv!r}")
        print(f"   {len(trials(nodes))} trials ok" if not bad else "")
    print(f"{'OK' if not bad else 'FAIL'}: {total - bad}/{total} field checks passed"); return 1 if bad else 0

if __name__ == "__main__":
    raise SystemExit(main())
