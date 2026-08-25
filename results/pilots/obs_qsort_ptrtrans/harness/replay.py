#!/usr/bin/env python3
"""OBS pilot replay: run every corpus input through C (ASan+UBSan) and PtrTrans-Rust drivers,
two driver variants (silent / print), and project each execution onto 4 observation channels:
  O-R = return value only (quickSort is void -> 'void' iff the call returned; process abort/panic => no return)
  O-P = process stdout + exit code
  O-S = boundary state = return + designated output memory (array) + globals (qsort: none)
  O-F = O-S U O-P
UB gate: C driver under ASan+UBSan (-fno-sanitize-recover=all); an input is admissible only if the C
process exits 0 with no sanitizer report on stderr. Inadmissible inputs are excluded from all cells.
"""
import os, sys, json, time, subprocess, hashlib
S=os.path.dirname(os.path.abspath(__file__))
CORPUS=sys.argv[1]; OUT=sys.argv[2]
C_BIN=os.path.join(S,"c","driver_c"); R_BIN=os.path.join(S,"rs","target","release","obs_rs_driver")
env=dict(os.environ); env["ASAN_OPTIONS"]="detect_leaks=0:abort_on_error=1"; env["UBSAN_OPTIONS"]="print_stacktrace=1:halt_on_error=1"
files=sorted(os.listdir(CORPUS))
CH=["O-R","O-P","O-S","O-F"]; DR=["silent","print"]
def run(binp, mode, data, sf):
    if os.path.exists(sf): os.remove(sf)
    p=subprocess.run([binp,mode,sf],input=data,capture_output=True,env=env,timeout=30)
    st=open(sf).read() if os.path.exists(sf) else None
    return dict(rc=p.returncode, out=p.stdout.decode("utf-8","replace"), err=p.stderr.decode("utf-8","replace"), state=st)
def project(r):
    # returns the observation per channel
    ret = "void" if (r["state"] is not None) else "NO-RETURN"   # state file only written after the call returned
    o_p = (r["rc"], r["out"])
    o_s = (ret, r["state"])
    return {"O-R":ret, "O-P":o_p, "O-S":o_s, "O-F":(o_s,o_p)}
cells={d:{c:dict(divergences=0,first_div_s=None,first_div_input=None) for c in CH} for d in DR}
cls=dict(valid=0,c_ub=0,c_unstable=0,rust_failure=0,semantic_difference=0,abstention=0,total=len(files))
records=[]; t0=time.time(); sf_c=os.path.join(S,"_st_c"); sf_r=os.path.join(S,"_st_r")
for idx,f in enumerate(files):
    data=open(os.path.join(CORPUS,f),"rb").read()
    n=min(len(data)//4,256)
    rec=dict(input=f,n=n,sha=hashlib.sha1(data).hexdigest()[:12])
    # C reference, both drivers; UB gate: both must be clean and mutually consistent
    c={d:run(C_BIN,d,data,sf_c) for d in DR}
    ub=any(x["rc"]!=0 or "runtime error" in x["err"] or "Sanitizer" in x["err"] for x in c.values())
    if ub:
        cls["c_ub"]+=1; rec["class"]="C-UB"; rec["c_err"]=c["silent"]["err"][-400:]; records.append(rec); continue
    c2=run(C_BIN,"silent",data,sf_c)   # repeated C replay for C-unstable
    if c2["state"]!=c["silent"]["state"] or c["silent"]["state"]!=c["print"]["state"]:
        cls["c_unstable"]+=1; rec["class"]="C-unstable"; records.append(rec); continue
    cls["valid"]+=1
    r={d:run(R_BIN,d,data,sf_r) for d in DR}
    rec["rust_rc"]={d:r[d]["rc"] for d in DR}
    rust_fail=any(x["rc"]!=0 for x in r.values())
    diverged_any=False; rec["div"]={}
    for d in DR:
        pc=project(c[d]); pr=project(r[d])
        for ch in CH:
            dv = pc[ch]!=pr[ch]
            rec["div"][f"{d}/{ch}"]=dv
            if dv:
                diverged_any=True
                cell=cells[d][ch]; cell["divergences"]+=1
                if cell["first_div_s"] is None:
                    cell["first_div_s"]=round(time.time()-t0,3); cell["first_div_input"]=f; cell["first_div_idx"]=idx
    if rust_fail: cls["rust_failure"]+=1; rec["class"]="Rust-failure"; rec["rust_err"]=r["silent"]["err"][-300:]
    elif diverged_any: cls["semantic_difference"]+=1; rec["class"]="semantic-difference"
    else: rec["class"]="agree"
    rec["c_arr"]=c["silent"]["state"].split("arr:")[1].strip()[:80]; rec["r_arr"]=r["silent"]["state"].split("arr:")[1].strip()[:80]
    records.append(rec)
elapsed=round(time.time()-t0,3)
res=dict(cells=cells,classification=cls,replay_wall_s=elapsed,corpus_files=len(files),corpus=CORPUS)
os.makedirs(OUT,exist_ok=True)
json.dump(res,open(os.path.join(OUT,"replay_summary.json"),"w"),indent=1)
with open(os.path.join(OUT,"replay_records.jsonl"),"w") as fh:
    for rec in records: fh.write(json.dumps(rec)+"\n")
print(json.dumps(res,indent=1))
