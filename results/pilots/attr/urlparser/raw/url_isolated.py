# (c) isolated ASan+UBSan C oracle: each input -> fresh process; sanitizer report / rc!=0 / timeout => excluded.
# Admissible inputs are then replayed through the (a) in-process differential binary (C+Rust) for the divergence count.
import os, subprocess, json, sys, time, glob
corpus_dirs = sys.argv[1:]
files = sorted(set(f for d in corpus_dirs for f in glob.glob(d + "/*")))
res = []; t0 = time.time(); first_div = None
env = dict(os.environ, ASAN_OPTIONS="detect_leaks=0:abort_on_error=0:symbolize=0", UBSAN_OPTIONS="print_stacktrace=0")
for f in files:
    data = open(f, "rb").read()
    if not data: continue
    try:
        pr = subprocess.run(["./url_oracle_asan"], input=data, capture_output=True, timeout=10, env=env)
        err = pr.stderr.decode(errors="replace")
        san = ("Sanitizer" in err) or ("runtime error" in err)
        kind = None
        if "heap-buffer-overflow" in err: kind = "asan:heap-buffer-overflow"
        elif "AddressSanitizer" in err: kind = "asan:" + (err.split("AddressSanitizer: ")[1].split()[0] if "AddressSanitizer: " in err else "?")
        elif "runtime error" in err: kind = "ubsan:" + err.split("runtime error: ")[1].split("\n")[0][:60]
        r = {"file": os.path.basename(f), "len": len(data), "rc": pr.returncode, "sanitizer": san, "kind": kind, "admissible": (pr.returncode == 0 and not san)}
    except subprocess.TimeoutExpired:
        r = {"file": os.path.basename(f), "len": len(data), "rc": None, "sanitizer": False, "kind": "timeout", "admissible": False}
    res.append(r)
t_oracle = time.time() - t0
adm = [r for r in res if r["admissible"]]
# differential replay on admissible inputs through the (a) binary, one input per process so a divergence panic is attributable
div = []
for r in adm:
    f = [x for x in files if os.path.basename(x) == r["file"]][0]
    pr = subprocess.run(["./urlparser_none/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_target_1", f], capture_output=True, timeout=30)
    out = pr.stderr.decode(errors="replace")
    if pr.returncode != 0:
        why = [l for l in out.splitlines() if "divergence" in l or "panicked" in l or "ERROR" in l or "deadly signal" in l]
        div.append({"file": r["file"], "rc": pr.returncode, "why": why[:2]})
        if first_div is None: first_div = time.time() - t0
summary = {"inputs": len(res), "excluded": len(res) - len(adm), "admissible": len(adm), "divergences_on_admissible": len(div),
           "exclusion_kinds": {}, "t_oracle_s": round(t_oracle, 2), "ttfd_s": first_div, "divergences": div}
for r in res:
    if not r["admissible"]: summary["exclusion_kinds"][r["kind"] or f"rc={r['rc']}"] = summary["exclusion_kinds"].get(r["kind"] or f"rc={r['rc']}", 0) + 1
json.dump({"summary": summary, "records": res}, open("url_isolated.json", "w"), indent=1)
print(json.dumps(summary, indent=1))
