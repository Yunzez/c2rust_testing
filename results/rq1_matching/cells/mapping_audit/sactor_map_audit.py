import json, os, re, glob
B="tools/frameworks/sactor/tests/c_examples"
def c_funcs(paths):
    names=set()
    for path in paths:
        txt=open(path,errors='ignore').read()
        txt=re.sub(r'//.*','',txt); txt=re.sub(r'/\*.*?\*/','',txt,flags=re.S)
        names|=set(re.findall(r'\b([A-Za-z_]\w*)\s*\([^;{)]*\)\s*\{', txt))
    return {n for n in names if n not in ('if','for','while','switch','sizeof','return')}
def rust_funcs(path):
    return set(re.findall(r'\bfn\s+([A-Za-z_]\w*)', open(path,errors='ignore').read()))
print(f"{'example':16s} {'map':>4} {'Cfn':>4} {'Rfn':>4}  bad_target(map->missing Rust)   incomplete(C not in map)")
for mp in sorted(glob.glob(f"{B}/*/**/function_name_map.json", recursive=True)):
    ex=mp.split('/c_examples/')[1].split('/')[0]
    exroot=f"{B}/{ex}"
    comb=os.path.join(os.path.dirname(os.path.dirname(mp)),"combined.rs")
    if not os.path.exists(comb): continue
    m=json.load(open(mp))
    rf=rust_funcs(comb)
    # this example's OWN C source only (test_task / c_for_analyzer / top-level .c)
    cs=[c for c in glob.glob(f"{exroot}/**/*.c",recursive=True)
        if 'target' not in c and 'test_harness' not in c and 'rust_crate' not in c and '/result' not in c]
    cf=c_funcs(cs)
    bad_val=[f"{k}->{v}" for k,v in m.items() if v not in rf]
    incomplete=[k for k in cf if k not in m]
    flag=" <==" if (bad_val or incomplete) else ""
    print(f"{ex:16s} {len(m):4d} {len(cf):4d} {len(rf):4d}  {str(bad_val)[:30]:30s}  {str(sorted(incomplete))[:38]}{flag}")
