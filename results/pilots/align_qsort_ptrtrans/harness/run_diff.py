import subprocess, os, sys, time, json
op=sys.argv[1]; inp=open(f"in_{op}.txt").read(); trials=int(inp.split("\n",1)[0])
env=dict(os.environ); env["ASAN_OPTIONS"]="detect_leaks=0"; env["UBSAN_OPTIONS"]="print_stacktrace=1"
t0=time.time()
rc=subprocess.run(["./c_pairs",op],input=inp,capture_output=True,text=True,env=env,timeout=600)
open(f"log_{op}_c.stderr","w").write(rc.stderr)
if rc.returncode!=0: print(json.dumps({"op":op,"c_gate":"FAIL","rc":rc.returncode,"stderr":rc.stderr[-400:]})); sys.exit(0)
t1=time.time()
rr=subprocess.run(["./rs_driver/target/release/rs_driver",op],input=inp,capture_output=True,text=True,timeout=600)
open(f"log_{op}_rs.stderr","w").write(rr.stderr)
t2=time.time()
if rr.returncode!=0: print(json.dumps({"op":op,"rust_failure":True,"rc":rr.returncode,"stderr":rr.stderr[-400:]})); sys.exit(0)
c=rc.stdout.strip("\n").split("\n"); r=rr.stdout.strip("\n").split("\n")
assert len(c)==trials==len(r),(len(c),len(r))
open(f"out_{op}_c.txt","w").write(rc.stdout); open(f"out_{op}_rs.txt","w").write(rr.stdout)
diffs=[t for t in range(trials) if c[t].split()!=r[t].split()]
first=None
if diffs:
    # TTFD: wall-clock until the first diverging record was compared, approximated as the fraction of the
    # batch run wall-clock up to that record (batch mode; records are processed sequentially)
    first=diffs[0]; ttfd=round((t2-t0)*(first+1)/trials,4)
else: ttfd="none"
ex=[]
for t in diffs[:3]:
    ex.append({"t":t,"C":c[t][:120],"RS":r[t][:120]})
print(json.dumps({"op":op,"trials":trials,"c_gate":"clean","valid_records":trials,"divergences":len(diffs),
  "first_div_index":first,"ttfd_s":ttfd,"wall_s":round(t2-t0,2),"examples":ex}))
