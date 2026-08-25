"""Fallback cJSON x PtrTrans map audit: no cjson *_trans_metadata.jsonl survives on disk, so audit the
crate's own self-declared map ('/// Translated from:' doc comment -> following fn name) and the
name-set overlap between cJSON.c function names and crate fn names."""
import re, sys, json
rs=open(sys.argv[1]).read(); c=open(sys.argv[2]).read()
cfn=set(re.findall(r'^(?:static\s+)?(?:CJSON_PUBLIC\([^)]*\)|[A-Za-z_][\w \*]*?)\s*\*?\s*([A-Za-z_]\w*)\s*\([^;{]*\)\s*\{', c, re.M))
cfn-= {'if','while','for','switch'}
rsfn=re.findall(r'^\s*pub(?:\(crate\))?\s+(?:unsafe\s+)?fn\s+([A-Za-z_]\w*)', rs, re.M)
decl=[]
for m in re.finditer(r'/// Translated from:\n((?:///.*\n)+?)\s*pub(?:\(crate\))?\s+(?:unsafe\s+)?fn\s+([A-Za-z_]\w*)', rs):
    sig=" ".join(l.strip('/ ') for l in m.group(1).strip().split("\n"))
    cname=re.search(r'([A-Za-z_]\w*)\s*\(', sig).group(1)
    decl.append((cname, m.group(2), sig[:70]))
mism=[(a,b,s) for a,b,s in decl if a!=b]
print(json.dumps({"c_fns":len(cfn),"rust_fns":len(rsfn),"rust_fns_unique":len(set(rsfn)),
 "name_overlap":len(cfn&set(rsfn)),"c_only":sorted(cfn-set(rsfn)),"rust_only":sorted(set(rsfn)-cfn),
 "doc_declared_records":len(decl),"doc_mismatches":mism},indent=1))
