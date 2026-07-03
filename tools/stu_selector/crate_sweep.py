#!/usr/bin/env python3
"""Corpus-scale auto-sweep: link a base c2rust crate + C2SaferRust WIP crate and differential-test
every value-shaped function present in both, using a PLAIN cargo binary (no cargo-fuzz) -- the full
c2rust crates build under plain cargo but not under cargo-fuzz's sanitizer instrumentation, and these
small-input value functions do not need coverage guidance. For each function it aligns the base/WIP
signatures (reusing rust_diff's logic), emits a randomized differential test (catch_unwind around each
side), and records CLEAN / DIVERGENCE / CRASH / UNSUPPORTED. Finds crc32-class silent diffs at scale.

Usage: crate_sweep.py --base-crate DIR --wip-crate DIR --base-name c2rust_out --wip-name wip_out \
                      --outdir DIR [--iters 200000]
(the WIP crate's package name must already differ from the base's to link both.)
"""
import argparse, re, subprocess, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
from rust_diff import find_sig, parse_aliases, resolve, role_of

LENISH = {"len","n","size","count","cap","length","sz","num","nbytes","outlen"}

def modpath(rel: Path) -> str:
    return "::".join(rel.with_suffix("").parts)

def plan_fn(base_src, wip_src, entry):
    bp = find_sig(base_src, entry); wp = find_sig(wip_src, entry)
    if not bp or not wp: raise ValueError("sig-not-found")
    bparams, bret = bp; wparams, wret = wp
    balias = parse_aliases(base_src); walias = parse_aliases(wip_src)
    if len(bparams) != len(wparams): raise ValueError("arg-count")
    for who, rt, al in (("base",bret,balias),("wip",wret,walias)):
        if rt is None: continue
        rr = resolve(rt, al)
        if rr.startswith("*const") or rr.startswith("*mut") or re.match(r'^&(mut )?(?!\[)', rr):
            raise ValueError(f"{who}-nonvalue-ret")
    plan=[]; seen=False
    for (bn,bt),(wn,wt) in zip(bparams, wparams):
        br=role_of(bt,balias); wr=role_of(wt,walias)
        if br[0]=="scalar" and wr[0]=="scalar":
            nm=(wn or bn or "").lower().strip("_")
            plan.append(("buflen" if (seen and nm in LENISH) else "scalar", bt, wt))
        elif br[0] in ("ptr","slice") and wr[0] in ("ptr","slice"):
            # only BYTE/scalar buffers -- reject struct pointers (png_struct_def, etc.)
            from rust_diff import SCALARS
            be=resolve(br[1],balias).split("::")[-1]; we=resolve(wr[1],walias).split("::")[-1]
            if be not in SCALARS or we not in SCALARS:
                raise ValueError(f"non-scalar-buffer-{br[1]}/{wr[1]}")
            plan.append(("buf",bt,wt)); seen=True
        else: raise ValueError(f"param-{br[0]}/{wr[0]}")
    if sum(1 for p in plan if p[0]=="buf")>1: raise ValueError(">1-buffer")
    return plan, balias, walias

def emit_test(idx, entry, mp, plan, balias, walias, base_name, wip_name, iters):
    dec=[]; ba=[]; wa=[]
    for i,(kind,bt,wt) in enumerate(plan):
        btr=resolve(bt,balias); wtr=resolve(wt,walias)
        if kind in ("scalar","buflen"):
            wbase=re.sub(r"^&(mut )?","",wtr).strip()
            src="vbuf.len() as" if kind=="buflen" else "rng.next() as"
            dec.append(f"        let a{i} = {src} {wbase};")
            ba.append(f"a{i} as {re.sub(r'^&(mut )?','',btr).strip()}"); wa.append(f"a{i}")
        else:
            welem=role_of(wt,walias)[1]
            dec.append(f"        let n=(rng.next() as usize)%65;")
            dec.append(f"        let mut vbuf: Vec<{welem}> = (0..n).map(|_| rng.next() as {welem}).collect();")
            if role_of(bt,balias)[0]=="ptr":
                ba.append(f"vbuf.{'as_ptr' if btr.startswith('*const') else 'as_mut_ptr'}() as {btr}")
            else: ba.append("&mut vbuf[..]" if btr.startswith("&mut") else "&vbuf[..]")
            if role_of(wt,walias)[0]=="slice":
                wa.append("&mut vbuf[..]" if wtr.startswith("&mut") else "&vbuf[..]")
            else: wa.append(f"vbuf.{'as_ptr' if wtr.startswith('*const') else 'as_mut_ptr'}() as {wtr}")
    body="\n".join(dec)
    pfx = f"{mp}::" if mp else ""
    bcall=f"{base_name}::{pfx}{entry}({', '.join(ba)})"
    wcall=f"{wip_name}::{pfx}{entry}({', '.join(wa)})"
    return f'''
fn test_{idx}(rng:&mut R)->&'static str{{
    for _ in 0..{iters} {{
{body}
        let b=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||format!("{{:?}}", unsafe{{ {bcall} }})));
        let w=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||format!("{{:?}}", unsafe{{ {wcall} }})));
        match (b,w) {{
            (Ok(bs),Ok(ws)) => if bs!=ws {{ eprintln!("DIVERGENCE {mp}::{entry} base={{}} wip={{}}", bs, ws); return "DIVERGENCE"; }},
            (Ok(_),Err(_)) => {{ eprintln!("CRASH(wip-only) {mp}::{entry}"); return "CRASH"; }},
            (Err(_),Ok(_)) => {{ eprintln!("CRASH(base-only) {mp}::{entry}"); return "CRASH"; }},
            _=>{{}}
        }}
    }}
    "CLEAN"
}}'''

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--base-crate",required=True); ap.add_argument("--wip-crate",required=True)
    ap.add_argument("--base-name",default="c2rust_out"); ap.add_argument("--wip-name",default="wip_out")
    ap.add_argument("--src-subdir",default="src"); ap.add_argument("--outdir",required=True)
    ap.add_argument("--iters",type=int,default=200000); ap.add_argument("--toolchain",default="nightly-2025-09-01")
    a=ap.parse_args()
    bcr=Path(a.base_crate); wcr=Path(a.wip_crate)
    out=Path(a.outdir); (out/"src").mkdir(parents=True,exist_ok=True)
    # figure out the lib entry file + which top-level modules are declared pub in it
    libpath=None
    m=re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"', wcr.joinpath("Cargo.toml").read_text(), re.S)
    libpath=m.group(1) if m else "c2rust-lib.rs"
    libfile=wcr/libpath
    libtxt=libfile.read_text() if libfile.exists() else ""
    decl_mods=set(re.findall(r'pub\s+mod\s+([A-Za-z_][A-Za-z0-9_]*)', libtxt))
    single_file = bool(re.search(r'\bpub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+', libtxt))  # lib file has fns
    SKIP={"build.rs","main.rs"}
    # choose the file set: src/ crates walk src recursively; root-module crates walk ONLY the crate
    # root (c2rust emits nested duplicate sub-crate dirs like bzip2/bzip2/ that we must NOT descend).
    if (wcr/"src").is_dir():
        files=[f for f in sorted((wcr/"src").rglob("*.rs")) if "target" not in f.parts]
    elif single_file:
        files=[libfile]
    else:
        files=[f for f in sorted(wcr.glob("*.rs"))]   # crate root only, non-recursive
    cands=[]
    for wf in files:
        rel=wf.relative_to(wcr)
        if wf==libfile:
            if not single_file: continue      # c2rust-lib.rs = declarations only
            mp=""                               # single-file crate: fns at crate root
        else:
            if rel.name in SKIP: continue
            top=rel.parts[0].replace(".rs","")
            if top not in decl_mods and (wcr/"src").is_dir()==False: continue
            mp=modpath(rel)
        bf=bcr/rel
        if not bf.exists(): continue
        wsrc=wf.read_text(); bsrc=bf.read_text()
        for fn in re.findall(r'\bpub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)', wsrc):
            if fn in ("main",): continue
            if re.search(rf'\bfn\s+{re.escape(fn)}\b', bsrc): cands.append((fn,mp,bsrc,wsrc))
    print(f"discovered {len(cands)} candidate fns (single_file={single_file}, mods={len(decl_mods)})")
    tests=[]; meta=[]; nun=0
    seen=set()
    for fn,mp,bsrc,wsrc in cands:
        if (fn,mp) in seen: continue
        seen.add((fn,mp))
        try: plan,ba,wa=plan_fn(bsrc,wsrc,fn)
        except ValueError: nun+=1; continue
        idx=len(tests)
        tests.append(emit_test(idx,fn,mp,plan,ba,wa,a.base_name,a.wip_name,a.iters))
        meta.append((idx,fn,mp))
    print(f"{len(tests)} value-shaped tests; {nun} unsupported")
    arms="\n".join(f'        {idx} => test_{idx}(&mut rng),' for idx,fn,mp in meta)
    src=f'''#![allow(warnings)]
use std::os::raw::*;
extern crate {a.base_name}; extern crate {a.wip_name};
struct R(u64); impl R {{ fn next(&mut self)->u64 {{ self.0^=self.0<<13; self.0^=self.0>>7; self.0^=self.0<<17; self.0 }} }}
{"".join(tests)}
fn main() {{
    std::panic::set_hook(Box::new(|_|{{}}));
    let mut rng=R(0x9e3779b97f4a7c15);
    let idx: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
    let r = match idx {{
{arms}
        _ => "BADIDX",
    }};
    println!("{{}}", r);
}}'''
    (out/"src"/"main.rs").write_text(src)
    (out/"Cargo.toml").write_text(f'''[package]
name="cratesweep"
version="0.0.0"
edition="2018"
[[bin]]
name="cratesweep"
path="src/main.rs"
[profile.release]
opt-level=2
[dependencies]
{a.base_name} = {{ path = "{bcr.resolve()}", package = "{a.base_name}" }}
{a.wip_name} = {{ path = "{wcr.resolve()}", package = "{a.wip_name}" }}
[workspace]
''')
    print("building (plain cargo) ...")
    b=subprocess.run(["cargo",f"+{a.toolchain}","build","--release"],cwd=out,capture_output=True,text=True)
    if b.returncode!=0:
        errs=[l for l in b.stderr.splitlines() if l.startswith("error")][:5]
        print("BUILD FAILED:\n  "+"\n  ".join(errs)); return
    print("running each test in its own process (isolates segfaults) ...")
    binp=str(out/"target"/"release"/"cratesweep")
    from collections import Counter
    c=Counter(); results=[]
    for idx,fn,mp in meta:
        try:
            r=subprocess.run([binp,str(idx)],capture_output=True,text=True,timeout=120)
        except subprocess.TimeoutExpired:
            results.append((fn,mp,"HANG")); c["HANG"]+=1; print(f"[HANG]  {mp}::{fn}"); continue
        div=[l for l in r.stderr.splitlines() if l.startswith("DIVERGENCE")]
        st = r.stdout.strip().split("\n")[-1] if r.stdout.strip() else ""
        if r.returncode < 0:            # killed by signal (segfault, etc.)
            results.append((fn,mp,"CRASH-SEGV")); c["CRASH-SEGV"]+=1; print(f"[SEGV]  {mp}::{fn}")
        elif st=="DIVERGENCE":
            note=div[0][:90] if div else ""
            results.append((fn,mp,"DIVERGENCE",note)); c["DIVERGENCE"]+=1; print(f"*** DIVERGENCE {mp}::{fn}  {note}")
        elif st=="CRASH":
            results.append((fn,mp,"CRASH-PANIC")); c["CRASH-PANIC"]+=1; print(f"[panic] {mp}::{fn}")
        elif st=="CLEAN":
            results.append((fn,mp,"CLEAN")); c["CLEAN"]+=1
        else:
            results.append((fn,mp,"OTHER")); c["OTHER"]+=1
    print("\n==== SWEEP SUMMARY ====", dict(c))
    for row in results:
        if row[2]=="DIVERGENCE": print("  DIVERGENCE ", row[1]+"::"+row[0], "  ", row[3] if len(row)>3 else "")

if __name__=="__main__":
    main()
