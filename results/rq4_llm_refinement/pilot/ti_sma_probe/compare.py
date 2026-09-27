import json, sys, pathlib, glob
P = pathlib.Path(sys.argv[1])
def stats(cond):
    f = P / cond / "ours" / "ti_sma.json"
    if not f.exists(): return None
    d = json.load(open(f))
    out = {}
    for fn in d["data"][0]["functions"]:
        n = fn["name"]
        if "ti_sma" not in n or "start" in n or "fuzz" in n.lower() or "harness" in n.lower(): continue
        regs = fn["regions"]; cov = sum(1 for r in regs if r[4] > 0)
        out[n[-40:]] = (cov, len(regs), fn.get("count"))
    corpus = len(list((P / cond / "corpus" / "ti_sma").glob("*")))
    return out, corpus
for c in ("base", "hint"):
    s = stats(c)
    print(c, "corpus", s[1] if s else None)
    if s:
        for k, v in s[0].items(): print(f"   {k}: {v[0]}/{v[1]} regions, entry count {v[2]}")
