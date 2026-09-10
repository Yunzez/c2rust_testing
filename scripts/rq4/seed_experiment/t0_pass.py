"""t=0 coverage: replay ONLY each arm's initial seeds through the instrumented harness (one coverage
build per harness, reused for the three arms). Output: E/t0/<arm>/ours/<b>.json + t0_status.json."""
import json, os, pathlib, shutil, subprocess, sys
sys.path.insert(0, "/home/yunzez/c2rust_testing/scripts/rq4")
import cell as C
E = pathlib.Path(sys.argv[1]); ARMS = ["base", "random", "grid"]
man = json.load(open(E / "seeds" / "manifest.json")); seeded = set(man["seeded"])
status = {}
for hd in sorted((E / "base" / "harnesses").iterdir()):
    if not hd.is_dir(): continue
    b = hd.name
    name = next(p.stem for p in (hd / "fuzz" / "fuzz_targets").glob("*.rs"))
    env = dict(os.environ, RUSTUP_TOOLCHAIN=C.TOOLCHAIN, C2R_MODE="rust-only", ASAN_OPTIONS="detect_leaks=0")
    covdir = hd / "target" / "x86_64-unknown-linux-gnu" / "coverage"
    cov = covdir / "x86_64-unknown-linux-gnu" / "release" / name
    res = {}
    try:
        for arm in ARMS:
            outj = E / "t0" / arm / "ours" / f"{b}.json"; outj.parent.mkdir(parents=True, exist_ok=True)
            corpus = E / "t0" / arm / "corpus" / b
            if arm != "base" and b not in seeded:
                shutil.copy(E / "t0" / "base" / "ours" / f"{b}.json", outj); res[arm] = "same as base (unseeded)"; continue
            if arm == "base":  # build + run through cargo fuzz coverage, exactly as collect() does
                r = subprocess.run(["cargo", "fuzz", "coverage", name, str(corpus), "--", "-timeout=25"],
                                   cwd=str(hd), env=env, capture_output=True, text=True, errors="replace", timeout=900)
                pd = hd / "fuzz" / "coverage" / name / "coverage.profdata"
                if r.returncode or not cov.exists() or not pd.exists():
                    res[arm] = f"failed rc={r.returncode}"; break
            else:  # same binary, the arm's seeds, -runs=0 executes every seed once and exits
                per = hd / f"t0_{arm}"; shutil.rmtree(per, ignore_errors=True); per.mkdir()
                e2 = dict(env, LLVM_PROFILE_FILE=str(per / "%m-%p.profraw"))
                rr = subprocess.run([str(cov), "-runs=0", "-timeout=25", str(corpus)], env=e2, cwd=str(C.sandbox_dir(hd)),
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=300)
                raws = list(per.glob("*.profraw"))
                if not raws: res[arm] = f"no profile rc={rr.returncode}"; continue
                pd = per / "coverage.profdata"
                subprocess.run([str(C.TC / "llvm-profdata"), "merge", "-sparse", *map(str, raws), "-o", str(pd)],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
            with open(outj, "w") as fh:
                subprocess.run([str(C.TC / "llvm-cov"), "export", str(cov), f"-instr-profile={pd}"], stdout=fh,
                               stderr=subprocess.DEVNULL, timeout=1800)
            res[arm] = "ok" if outj.stat().st_size > 0 else "empty-export"
            if arm != "base": shutil.rmtree(per, ignore_errors=True)
    finally:
        shutil.rmtree(covdir, ignore_errors=True)
    status[b] = res; print(b, res, flush=True)
(E / "t0" / "t0_status.json").write_text(json.dumps(status, indent=1) + "\n")
print("T0_PASS_DONE")
