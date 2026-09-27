"""Rust-crate coverage census for the raw-LLM lil translation.
Merges profraws, exports llvm-cov JSON, then checks each of the 117 analyzer-known
functions (leaf + type-qualified names matched against mangled symbols) for count>0."""
import json, re, subprocess, sys, glob, os
SYS = subprocess.run(["rustc","--print","sysroot"],capture_output=True,text=True).stdout.strip()
BIN = f"{SYS}/lib/rustlib/x86_64-unknown-linux-gnu/bin"
subprocess.run([f"{BIN}/llvm-profdata","merge","-sparse","-o","merged.profdata"]+glob.glob("prof/*.profraw"),check=True)
exp = json.loads(subprocess.run([f"{BIN}/llvm-cov","export","./target/debug/cov",
      "-instr-profile=merged.profdata","-summary-only" if False else "-skip-expansions"],
      capture_output=True,text=True).stdout or "{}")
funcs = exp["data"][0]["functions"]
# executed mangled symbols
hot = [f["name"] for f in funcs if f.get("count",0) > 0]
cold= [f["name"] for f in funcs if f.get("count",0) == 0]
# the analyzer's 117 function names for this crate
known=[f["name"] for f in json.load(open("../../../results/rq1_matching/cells/rawllm/lil/lil_r.json"))["functions"]]
def hit(name):
    parts = name.split("::")           # e.g. LilInterpreter::fnc_append -> both parts must appear
    return any(all(p in sym for p in parts) for sym in hot)
covered=[n for n in known if hit(n)]
missed =[n for n in known if not hit(n)]
fnc=[n for n in known if n.split("::")[-1].startswith("fnc_")]
fnc_cov=[n for n in fnc if n in covered]
print(f"Rust crate functions covered : {len(covered)}/{len(known)} = {len(covered)/len(known):.1%}")
print(f"fnc_* handlers covered       : {len(fnc_cov)}/{len(fnc)}")
print(f"missed ({len(missed)}):")
for i in range(0,len(missed),3):
    print("   "+"  ".join(f"{m:34s}" for m in missed[i:i+3]))
