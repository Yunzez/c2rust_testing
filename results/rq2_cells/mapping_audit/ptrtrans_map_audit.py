"""Audit PtrTrans's shipped function-correspondence map (rust_definition_name) against
the C function names, on lodepng. Reproduces the 143/255 mismatch + 102 airtight scrambles."""
import json, sys
F = sys.argv[1] if len(sys.argv) > 1 else \
    "tools/frameworks/ptrtrans_rebuild/PtrTrans-C2Rust/dataset/PA_trans_projects/lodepng_Trans_PA_trans_metadata.jsonl"
cnames, pairs = set(), []
for line in open(F):
    d = json.loads(line)
    if d.get("map_tag") != "code":
        continue
    c = d["source_c_code_id"][0].split("#")[0]
    rdn = d.get("rust_definition_name")
    r = rdn[0] if isinstance(rdn, list) and rdn else ""
    cnames.add(c); pairs.append((c, r))
n = len(pairs)
exact = sum(1 for c, r in pairs if c == r)
mism = [(c, r) for c, r in pairs if c != r and r]
scramble = [(c, r) for c, r in mism if r in cnames]   # claimed Rust name IS another C function
print(f"function-level records : {n}")
print(f"self-consistent (r==c) : {exact} ({exact/n:.0%})")
print(f"mismatches (r!=c)      : {len(mism)} ({len(mism)/n:.0%})")
print(f"AIRTIGHT scrambles     : {len(scramble)} ({len(scramble)/n:.0%})  (claimed Rust target is a distinct C fn)")
print("smoking guns:")
for c, r in scramble[:6]:
    print(f"  {c} -> claims '{r}' (itself a distinct C function)")
