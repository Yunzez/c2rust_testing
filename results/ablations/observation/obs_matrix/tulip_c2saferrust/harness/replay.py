#!/usr/bin/env python3
"""OBS matrix generic replay (derived from results/ablations/observation/obs_qsort_ptrtrans/harness/replay.py).
usage: replay.py CORPUS OUTDIR C_BIN R_BIN
Driver protocol (both sides): argv[1] = silent|print, argv[2] = state-file path, stdin = input bytes.
State file is written AFTER the call returns, never to stdout; first line MUST be 'ret:<value>' (or 'ret:void').
Channels: O-R = ret line (NO-RETURN if no state file); O-P = (exit code, stdout); O-S = (O-R, whole state file);
O-F = (O-S, O-P).  UB gate: C driver built with ASan+UBSan, halt_on_error; input excluded if exit!=0 or a sanitizer
report is on stderr in either driver mode; second C replay -> C-unstable check.
"""
import os, sys, json, time, subprocess, hashlib
CORPUS, OUT, C_BIN, R_BIN = sys.argv[1:5]
TO = float(os.environ.get("OBS_TIMEOUT", "30"))
env = dict(os.environ); env["ASAN_OPTIONS"] = "detect_leaks=0:abort_on_error=1"; env["UBSAN_OPTIONS"] = "print_stacktrace=1:halt_on_error=1"
env.setdefault("RUST_BACKTRACE", "0")
files = sorted(os.listdir(CORPUS))
CH = ["O-R", "O-P", "O-S", "O-F"]; DR = ["silent", "print"]
os.makedirs(OUT, exist_ok=True)
def run(binp, mode, data, sf):
    if os.path.exists(sf): os.remove(sf)
    try:
        p = subprocess.run([binp, mode, sf], input=data, capture_output=True, env=env, timeout=TO)
        rc, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        rc, out, err = -999, (e.stdout or b""), (e.stderr or b"") + b"\n[TIMEOUT]"
    st = open(sf, "rb").read().decode("utf-8", "replace") if os.path.exists(sf) else None
    return dict(rc=rc, out=out.decode("utf-8", "replace"), err=err.decode("utf-8", "replace"), state=st)
def project(r):
    ret = r["state"].split("\n", 1)[0] if r["state"] is not None else "NO-RETURN"
    o_p = (r["rc"], r["out"]); o_s = (ret, r["state"])
    return {"O-R": ret, "O-P": o_p, "O-S": o_s, "O-F": (o_s, o_p)}
cells = {d: {c: dict(divergences=0, first_div_s=None, first_div_input=None, first_div_idx=None) for c in CH} for d in DR}
cls = dict(valid=0, c_ub=0, c_unstable=0, rust_failure=0, semantic_difference=0, abstention=0, agree=0, total=len(files))
records = []; t0 = time.time(); sf_c = os.path.join(OUT, "_st_c"); sf_r = os.path.join(OUT, "_st_r")
for idx, f in enumerate(files):
    data = open(os.path.join(CORPUS, f), "rb").read()
    rec = dict(idx=idx, input=f, len=len(data), sha=hashlib.sha1(data).hexdigest()[:12])
    c = {d: run(C_BIN, d, data, sf_c) for d in DR}
    ub = any(x["rc"] < 0 or x["rc"] >= 128 or x["rc"] == 99 or "runtime error" in x["err"] or "Sanitizer" in x["err"] or "[TIMEOUT]" in x["err"] for x in c.values())  # CELL PATCH: C exit 1 (usage error) is not UB; gate on sanitizer report / signal only
    if ub:
        cls["c_ub"] += 1; rec["class"] = "C-UB"; rec["c_rc"] = c["silent"]["rc"]; rec["c_err"] = c["silent"]["err"][-400:]; records.append(rec); continue
    c2 = run(C_BIN, "silent", data, sf_c)
    if c2["state"] != c["silent"]["state"] or c["silent"]["state"] != c["print"]["state"] or c2["out"] != c["silent"]["out"]:
        cls["c_unstable"] += 1; rec["class"] = "C-unstable"; records.append(rec); continue
    cls["valid"] += 1
    r = {d: run(R_BIN, d, data, sf_r) for d in DR}
    rec["rust_rc"] = {d: r[d]["rc"] for d in DR}
    rust_fail = any(x["rc"] < 0 or x["rc"] >= 128 or x["rc"] == 101 for x in r.values())  # CELL PATCH: CLI exit 1 is a normal return, not a Rust failure; signal/panic only
    diverged_any = False; rec["div"] = {}
    for d in DR:
        pc = project(c[d]); pr = project(r[d])
        for ch in CH:
            dv = pc[ch] != pr[ch]; rec["div"][f"{d}/{ch}"] = dv
            if dv:
                diverged_any = True; cell = cells[d][ch]; cell["divergences"] += 1
                if cell["first_div_s"] is None:
                    cell["first_div_s"] = round(time.time() - t0, 3); cell["first_div_input"] = f; cell["first_div_idx"] = idx
    if rust_fail: cls["rust_failure"] += 1; rec["class"] = "Rust-failure"; rec["rust_err"] = r["silent"]["err"][-300:]
    elif diverged_any: cls["semantic_difference"] += 1; rec["class"] = "semantic-difference"
    else: cls["agree"] += 1; rec["class"] = "agree"
    rec["c_state"] = (c["silent"]["state"] or "")[:160]; rec["r_state"] = (r["silent"]["state"] or "")[:160]
    rec["c_out"] = c["print"]["out"][:160]; rec["r_out"] = r["print"]["out"][:160]
    records.append(rec)
elapsed = round(time.time() - t0, 3)
res = dict(cells=cells, classification=cls, replay_wall_s=elapsed, corpus_files=len(files), corpus=CORPUS, c_bin=C_BIN, r_bin=R_BIN)
json.dump(res, open(os.path.join(OUT, "replay_summary.json"), "w"), indent=1)
with open(os.path.join(OUT, "replay_records.jsonl"), "w") as fh:
    for rec in records: fh.write(json.dumps(rec) + "\n")
print(json.dumps(res, indent=1))
