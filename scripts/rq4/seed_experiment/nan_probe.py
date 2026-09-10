"""For every boundary that has divergence inputs, rebuild its harness with the row/array compares instrumented to
print each differing element (both sides, with bits), replay the divergence inputs in combined mode, and classify
each input: nan_only (every differing element is NaN on both sides), numeric (some differing element is a real
number on at least one side), or no_diff_seen (the harness diverged elsewhere).
usage: nan_probe.py <harness tarball> <divergences dir> <workdir> <out.json>"""
import json, os, pathlib, re, subprocess, sys, tarfile, shutil
tarball, divroot, work, outj = map(pathlib.Path, sys.argv[1:5]); work.mkdir(parents=True, exist_ok=True)
ONLY = set(sys.argv[5].split(",")) if len(sys.argv) > 5 else None
env = dict(os.environ, RUSTUP_TOOLCHAIN="nightly-2025-09-01", CARGO_TARGET_DIR=str(work / "target"), ASAN_OPTIONS="detect_leaks=0", C2R_MODE="combined")
res = {}
for bd in sorted(p for p in divroot.iterdir() if p.is_dir()):
    inputs = sorted(f for f in bd.iterdir() if f.is_file() and not f.name.startswith("_"))
    if not inputs: continue
    b = bd.name; hd = work / "harnesses" / b
    if ONLY and b not in ONLY: continue
    shutil.rmtree(hd, ignore_errors=True)
    with tarfile.open(tarball) as t:
        members = [m for m in t.getmembers() if m.name.startswith(f"harnesses/{b}/")]
        t.extractall(work, members=members)
    ft = next((hd / "fuzz" / "fuzz_targets").glob("*.rs")); s = ft.read_text(); patched = 0
    for line in list(s.split("\n")):
        m = re.search(r'c2r_div\("(table (\w+) row (\d+)|array (\w+)|buffer (\w+)|out array (\w+))"\)', line)
        if not m or "if " not in line: continue
        if m.group(2): cv, rv = f"{m.group(2)}__{m.group(3)}_c", f"{m.group(2)}__{m.group(3)}_r"
        else: nm = m.group(4) or m.group(5) or m.group(6); cv, rv = f"{nm}_c", f"{nm}_r"
        tag = m.group(1)
        probe = (f'    {{ let n = {cv}.len().min({rv}.len()); let mut nd = 0usize; let mut nn = 0usize; for i in 0..n {{ '
                 f'if {cv}[i].to_bits() != {rv}[i].to_bits() {{ nd += 1; if {cv}[i].is_nan() && {rv}[i].is_nan() {{ nn += 1; }} '
                 f'else if nd - nn <= 4 {{ eprintln!("PROBE {tag} [{{}}] c={{:?}} r={{:?}}", i, {cv}[i], {rv}[i]); }} }} }} '
                 f'if {cv}.len() != {rv}.len() {{ eprintln!("PROBE {tag} lenc={{}} lenr={{}}", {cv}.len(), {rv}.len()); }} '
                 f'eprintln!("PROBE {tag} diff={{}} nan_nan={{}}", nd, nn); }}\n')
        s = s.replace(line, probe + line, 1); patched += 1
    ft.write_text(s)
    r = subprocess.run(["cargo", "fuzz", "build", "-O", "--debug-assertions", ft.stem], cwd=str(hd), env=env, capture_output=True, text=True)
    binp = work / "target" / "x86_64-unknown-linux-gnu" / "release" / ft.stem
    if r.returncode or not binp.exists():
        res[b] = {"error": "build failed", "patched": patched, "stderr": r.stderr[-400:]}; print(b, "BUILD FAILED", flush=True); continue
    rows = {}
    for f in inputs:
        rr = subprocess.run([str(binp), "-runs=1", str(f)], env=env, capture_output=True, text=True, timeout=60, cwd=str(hd / "sandbox") if (hd / "sandbox").is_dir() else str(hd))
        lines = [l for l in rr.stderr.split("\n") if l.startswith("PROBE") or "C2R_OUTCOME" in l]
        stats = re.findall(r"PROBE (.+?) diff=(\d+) nan_nan=(\d+)", rr.stderr)
        diffs = [(t, int(d), int(n)) for t, d, n in stats if int(d)]
        numeric = [l for l in lines if l.startswith("PROBE") and "] c=" in l]
        outcome = next((l.split("C2R_OUTCOME")[1].strip() for l in lines if "C2R_OUTCOME" in l), "")
        cls = "no_diff_seen" if not diffs else ("numeric" if any(d != n for _, d, n in diffs) else "nan_only")
        rows[f.name] = {"class": cls, "outcome": outcome, "diffs": diffs, "examples": numeric[:4]}
    tally = {}
    for v in rows.values(): tally[v["class"]] = tally.get(v["class"], 0) + 1
    res[b] = {"patched_compares": patched, "inputs": len(inputs), "tally": tally, "rows": rows}
    print(b, patched, "compares patched;", tally, flush=True)
    shutil.rmtree(hd, ignore_errors=True)
outj.write_text(json.dumps(res, indent=1) + "\n"); print("NAN_PROBE_DONE")
