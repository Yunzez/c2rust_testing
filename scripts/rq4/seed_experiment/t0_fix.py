"""Re-do the t=0 replay for harnesses whose batch -runs=0 replay died (a seed panics -> abort -> no profile),
with the per-input fallback collect() uses: one process per seed, LLVM_PROFILE_FILE per run, merge survivors.
A seed that panics contributes nothing (same convention as the archived campaigns). usage: t0_fix.py E [arms]"""
import json, os, pathlib, shutil, subprocess, sys
sys.path.insert(0, "/home/yunzez/c2rust_testing/scripts/rq4"); import cell as C
E = pathlib.Path(sys.argv[1]); ARMS = (sys.argv[2] if len(sys.argv) > 2 else "base,random,grid").split(",")
st = json.load(open(E / "t0" / "t0_status.json"))
todo = {b: [a for a in ARMS if str(v.get(a, "")).startswith("no profile")] for b, v in st.items()}
todo = {b: arms for b, arms in todo.items() if arms}
print("harnesses to redo:", len(todo), flush=True)
for b, arms in sorted(todo.items()):
    hd = E / "base" / "harnesses" / b
    name = next(p.stem for p in (hd / "fuzz" / "fuzz_targets").glob("*.rs"))
    env = dict(os.environ, RUSTUP_TOOLCHAIN=C.TOOLCHAIN, C2R_MODE="rust-only", ASAN_OPTIONS="detect_leaks=0")
    covdir = hd / "target" / "x86_64-unknown-linux-gnu" / "coverage"; cov = covdir / "x86_64-unknown-linux-gnu" / "release" / name
    try:
        r = subprocess.run(["cargo", "fuzz", "coverage", name, str(E / "t0" / "base" / "corpus" / b), "--", "-timeout=25"],
                           cwd=str(hd), env=env, capture_output=True, text=True, errors="replace", timeout=900)
        if r.returncode or not cov.exists(): st[b]["fix"] = f"build failed rc={r.returncode}"; print(b, st[b]["fix"], flush=True); continue
        for arm in arms:
            per = hd / f"t0fix_{arm}"; shutil.rmtree(per, ignore_errors=True); per.mkdir(); ok = 0; files = sorted((E / "t0" / arm / "corpus" / b).iterdir())
            for f in files:
                e2 = dict(env, LLVM_PROFILE_FILE=str(per / "%m-%p.profraw"))
                try:
                    if subprocess.run([str(cov), "-runs=1", "-timeout=25", str(f)], env=e2, cwd=str(C.sandbox_dir(hd)),
                                      stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=60).returncode == 0: ok += 1
                except subprocess.TimeoutExpired: pass
            raws = list(per.glob("*.profraw")); outj = E / "t0" / arm / "ours" / f"{b}.json"
            if raws:
                pd = per / "coverage.profdata"
                subprocess.run([str(C.TC / "llvm-profdata"), "merge", "-sparse", *map(str, raws), "-o", str(pd)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
                with open(outj, "w") as fh: subprocess.run([str(C.TC / "llvm-cov"), "export", str(cov), f"-instr-profile={pd}"], stdout=fh, stderr=subprocess.DEVNULL, timeout=1800)
                st[b][arm] = f"per-input ({ok}/{len(files)} completed)"
            else: st[b][arm] = f"per-input (0/{len(files)} completed, no profile)"
            shutil.rmtree(per, ignore_errors=True); print(b, arm, st[b][arm], flush=True)
    finally: shutil.rmtree(covdir, ignore_errors=True)
(E / "t0" / "t0_status.json").write_text(json.dumps(st, indent=1) + "\n"); print("T0_FIX_DONE")
