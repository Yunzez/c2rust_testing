import random, subprocess
random.seed(42)
trials=50000
cases=[]
for t in range(trials):
    n=random.choice([0,1,2,3,5,8,16,64,256])
    vals=[random.choice([random.randint(-2**31,2**31-1), random.randint(-100,100), 0, 2**31-1, -2**31]) for _ in range(n)]
    cases.append((n,vals))
inp=f"{trials}\n"+"\n".join(f"{n}\n"+" ".join(map(str,v)) for n,v in cases)+"\n"
import os
env=dict(os.environ); env["ASAN_OPTIONS"]="detect_leaks=0"
rc=subprocess.run(["./c_batch"],input=inp,capture_output=True,text=True,env=env,timeout=600)
assert rc.returncode==0, f"C batch UB/abort: {rc.stderr[-300:]}"   # whole-batch UB gate
rr=subprocess.run(["./rs_driver/target/release/rs_driver"],input=inp,capture_output=True,text=True,timeout=600)
assert rr.returncode==0, f"Rust batch crashed: {rr.stderr[-300:]}"
c_lines=rc.stdout.strip("\n").split("\n"); r_lines=rr.stdout.strip("\n").split("\n")
assert len(c_lines)==trials and len(r_lines)==trials, (len(c_lines),len(r_lines))
diffs=0; shown=0
for t,(cl,rl) in enumerate(zip(c_lines,r_lines)):
    if cl.split()!=rl.split():
        diffs+=1
        if shown<3:
            shown+=1
            n,vals=cases[t]
            print(f"DIFF t={t} n={n} in={vals[:8]}\n  C : {cl[:100]}\n  RS: {rl[:100]}")
# also: how many Rust outputs are not even sorted?
unsorted=0
for rl in r_lines:
    xs=[int(x) for x in rl.split()]
    if any(xs[i]>xs[i+1] for i in range(len(xs)-1)): unsorted+=1
print(f"FINAL: {trials} trials, {diffs} diffs ({100*diffs/trials:.2f}%), rust-output-unsorted={unsorted}")
