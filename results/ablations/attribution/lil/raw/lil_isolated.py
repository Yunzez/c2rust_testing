# (c) isolated ASan+UBSan oracle per record + (d) repeated C replay (fresh process alone / batch fwd / batch reversed)
import struct, subprocess, json, sys, time
S="."
data=open("lil_corpus.bin","rb").read(); recs=[]; p=0
while p+2<=len(data):
    n=struct.unpack_from("<H",data,p)[0]; p+=2; recs.append(data[p:p+n]); p+=n
def batch(binary, order):
    blob=b"".join(struct.pack("<H",len(recs[i]))+recs[i] for i in order)
    out=subprocess.run([binary],input=blob,capture_output=True,timeout=120).stdout.decode(errors="replace").splitlines()
    res=[None]*len(recs)
    for k,i in enumerate(order):
        res[i]=out[k] if k<len(out) else "<MISSING>"
    return res
N=len(recs)
t0=time.time()
iso=[]
for i,r in enumerate(recs):
    try:
        pr=subprocess.run(["./lil_c_asan","--single"],input=r,capture_output=True,timeout=10,env={"ASAN_OPTIONS":"detect_leaks=0","UBSAN_OPTIONS":"print_stacktrace=0"})
        err=pr.stderr.decode(errors="replace")
        san=("runtime error" in err) or ("Sanitizer" in err)
        iso.append({"i":i,"rc":pr.returncode,"sanitizer":san,"out":pr.stdout.decode(errors="replace").strip(),"err":err.strip().splitlines()[0] if err.strip() else ""})
    except subprocess.TimeoutExpired:
        iso.append({"i":i,"rc":None,"sanitizer":False,"timeout":True,"out":"","err":"timeout"})
t_iso=time.time()-t0
# (d) repeated replay on the plain binary
alone=[subprocess.run(["./lil_c_none","--single"],input=r,capture_output=True,timeout=10).stdout.decode(errors="replace").strip() for r in recs]
fwd=batch("./lil_c_none",list(range(N)))
rev=batch("./lil_c_none",list(reversed(range(N))))
# also: each record run right after a shorter record (fn-15 trigger: "any shorter record precedes it")
short_first=batch("./lil_c_none",[0]+list(range(N)))  # rec0 is 'expr 1 << 64' (12 bytes)
short_first=short_first  # positions map by index (rec0 duplicated; second run kept)
unstable=[i for i in range(N) if len({alone[i],fwd[i],rev[i]})>1]
json.dump({"n":N,"iso":iso,"alone":alone,"fwd":fwd,"rev":rev,"unstable":unstable,"t_isolated_s":t_iso},open("lil_isolated.json","w"),indent=1)
print("N",N,"isolated-excluded",sum(1 for x in iso if x["sanitizer"] or (x["rc"] not in (0,))),"unstable",unstable,"t_iso",round(t_iso,1))
for i in unstable: print(i, recs[i], "| alone:",alone[i],"| fwd:",fwd[i],"| rev:",rev[i])
