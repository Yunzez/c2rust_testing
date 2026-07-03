#!/usr/bin/env python3
"""Corpus-scale auto-sweep: link a base c2rust crate + C2SaferRust WIP crate and differential-fuzz
every value-shaped function present in both. No per-module extraction -- both crates build as rlibs and
expose their functions at `<crate>::<mod path>::<fn>`. For each function it aligns the base/WIP signatures
(reusing rust_diff's logic), generates a fuzz target comparing return VALUES, builds, fuzzes briefly, and
records CLEAN / DIVERGENCE / UNSUPPORTED / BUILD-FAIL. This is how we find crc32-class silent diffs at scale.

Usage: crate_sweep.py --base-crate DIR --wip-crate DIR --base-name c2rust_out --wip-name wip_out \
                      --src-subdir src --outdir DIR [--secs 12] [--limit N]
(the WIP crate's package name must already differ from the base's to link both.)
"""
import argparse, re, subprocess, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
from rust_diff import find_sig, parse_aliases, resolve, role_of  # reuse the aligned-signature logic

LENISH = {"len","n","size","count","cap","length","sz","num","nbytes","outlen"}

def modpath(rel: Path) -> str:
    parts = list(rel.with_suffix("").parts)
    return "::".join(parts)

def build_driver(base_src, wip_src, entry, base_ref, wip_ref):
    """Return (driver_str) or raise ValueError(reason)."""
    bp = find_sig(base_src, entry); wp = find_sig(wip_src, entry)
    if not bp or not wp: raise ValueError("sig-not-found")
    bparams, bret = bp; wparams, wret = wp
    balias = parse_aliases(base_src); walias = parse_aliases(wip_src)
    if len(bparams) != len(wparams): raise ValueError("arg-count-differs")
    for who, rt, al in (("base",bret,balias),("wip",wret,walias)):
        if rt is None: continue
        rr = resolve(rt, al)
        if rr.startswith("*const") or rr.startswith("*mut") or re.match(r'^&(mut )?(?!\[)', rr):
            raise ValueError(f"{who}-nonvalue-return")
    plan=[]; seen=False
    for (bn,bt),(wn,wt) in zip(bparams, wparams):
        br=role_of(bt,balias); wr=role_of(wt,walias)
        if br[0]=="scalar" and wr[0]=="scalar":
            nm=(wn or bn or "").lower().strip("_")
            plan.append(("buflen" if (seen and nm in LENISH) else "scalar", bt, wt))
        elif br[0] in ("ptr","slice") and wr[0] in ("ptr","slice"):
            plan.append(("buf",bt,wt)); seen=True
        else: raise ValueError(f"param {br[0]}/{wr[0]}")
    if sum(1 for p in plan if p[0]=="buf")>1: raise ValueError(">1-buffer")
    dec=[]; ba=[]; wa=[]
    for idx,(kind,bt,wt) in enumerate(plan):
        btr=resolve(bt,balias); wtr=resolve(wt,walias)
        if kind in ("scalar","buflen"):
            wbase=re.sub(r"^&(mut )?","",wtr).strip()
            src="vbuf.len() as" if kind=="buflen" else "cur.take_u64() as"
            dec.append(f"    let a{idx} = {src} {wbase};")
            ba.append(f"a{idx} as {re.sub(r'^&(mut )?','',btr).strip()}"); wa.append(f"a{idx}")
        else:
            welem=role_of(wt,walias)[1]
            dec.append(f"    let n=(cur.byte() as usize)%65;")
            dec.append(f"    let mut vbuf: Vec<{welem}> = (0..n).map(|_| cur.byte() as {welem}).collect();")
            if role_of(bt,balias)[0]=="ptr":
                ba.append(f"vbuf.{'as_ptr' if btr.startswith('*const') else 'as_mut_ptr'}() as {btr}")
            else: ba.append("&mut vbuf[..]" if btr.startswith("&mut") else "&vbuf[..]")
            if role_of(wt,walias)[0]=="slice":
                wa.append("&mut vbuf[..]" if wtr.startswith("&mut") else "&vbuf[..]")
            else: wa.append(f"vbuf.{'as_ptr' if wtr.startswith('*const') else 'as_mut_ptr'}() as {wtr}")
    body = "\n".join(dec)
    return f'''#![no_main]
#![allow(warnings)]
use libfuzzer_sys::fuzz_target;
use std::os::raw::*;
struct Cur<'a>{{d:&'a[u8],p:usize}}
impl<'a> Cur<'a>{{fn new(d:&'a[u8])->Self{{Cur{{d,p:0}}}}
 fn byte(&mut self)->u8{{let b=if self.p<self.d.len(){{self.d[self.p]}}else{{0}};self.p+=1;b}}
 fn take_u64(&mut self)->u64{{let mut v=0u64;for i in 0..8{{v|=(self.byte() as u64)<<(8*i);}}v}}}}
fuzz_target!(|data:&[u8]|{{
    let mut cur=Cur::new(data);
{body}
    let b=format!("{{:?}}", unsafe {{ {base_ref}{entry}({', '.join(ba)}) }});
    let w=format!("{{:?}}", unsafe {{ {wip_ref}{entry}({', '.join(wa)}) }});
    if b!=w {{ panic!("divergence: base={{:?}} wip={{:?}}", b, w); }}
}});
'''

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--base-crate",required=True); ap.add_argument("--wip-crate",required=True)
    ap.add_argument("--base-name",default="c2rust_out"); ap.add_argument("--wip-name",default="wip_out")
    ap.add_argument("--src-subdir",default="src"); ap.add_argument("--outdir",required=True)
    ap.add_argument("--secs",type=int,default=12); ap.add_argument("--limit",type=int,default=0)
    ap.add_argument("--toolchain",default="nightly-2025-09-01")
    a=ap.parse_args()
    bcr=Path(a.base_crate); wcr=Path(a.wip_crate)
    out=Path(a.outdir); (out/"fuzz"/"fuzz_targets").mkdir(parents=True,exist_ok=True); (out/"src").mkdir(exist_ok=True)
    # discover candidate (fn, modpath, base_file, wip_file)
    cands=[]
    for wf in sorted((wcr/a.src_subdir).rglob("*.rs")):
        rel=wf.relative_to(wcr)
        bf=bcr/rel
        if not bf.exists(): continue
        wsrc=wf.read_text(); bsrc=bf.read_text()
        mp=modpath(rel)
        for fn in re.findall(r'\bpub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)', wsrc):
            if f"fn {fn}" in bsrc:
                cands.append((fn, mp, bsrc, wsrc))
    print(f"discovered {len(cands)} candidate fns in {a.wip_crate}")
    targets=[]; results=[]
    for fn, mp, bsrc, wsrc in cands:
        base_ref=f"{a.base_name}::{mp}::"; wip_ref=f"{a.wip_name}::{mp}::"
        try:
            drv=build_driver(bsrc, wsrc, fn, base_ref, wip_ref)
        except ValueError as e:
            results.append((fn,mp,"UNSUPPORTED",str(e))); continue
        tname=f"t_{len(targets)}_{fn}"[:60]
        (out/"fuzz"/"fuzz_targets"/f"{tname}.rs").write_text(drv)
        targets.append((tname, fn, mp))
        if a.limit and len(targets)>=a.limit: break
    print(f"{len(targets)} value-shaped targets generated; {sum(1 for r in results if r[2]=='UNSUPPORTED')} unsupported")
    # write crate manifests
    (out/"src"/"lib.rs").write_text("")
    (out/"Cargo.toml").write_text(f'''[package]
name="cratesweep"
version="0.0.0"
edition="2018"
[lib]
path="src/lib.rs"
[dependencies]
[workspace]
''')
    deps=f'''{a.base_name} = {{ path = "{bcr.resolve()}", package = "{a.base_name}" }}
{a.wip_name} = {{ path = "{wcr.resolve()}", package = "{a.wip_name}" }}'''
    bins="\n".join(f'[[bin]]\nname="{t}"\npath="fuzz_targets/{t}.rs"\ntest=false\ndoc=false' for t,_,_ in targets)
    (out/"fuzz"/"Cargo.toml").write_text(f'''[package]
name="cratesweep-fuzz"
version="0.0.0"
edition="2018"
[package.metadata]
cargo-fuzz=true
[dependencies]
libfuzzer-sys="0.4"
{deps}
{bins}
[workspace]
''')
    # build once (all targets), then fuzz each
    print("building all targets ...")
    b=subprocess.run(["cargo",f"+{a.toolchain}","fuzz","build"],cwd=out,capture_output=True,text=True)
    built=set(re.findall(r'Compiling|Finished', b.stdout+b.stderr))
    for tname, fn, mp in targets:
        # check this target's binary exists
        r=subprocess.run(["cargo",f"+{a.toolchain}","fuzz","run",tname,"--",
                          f"-max_total_time={a.secs}","-timeout=12"],cwd=out,capture_output=True,text=True)
        txt=r.stdout+r.stderr
        div=re.search(r'divergence: (base=.*)',txt)
        if div: results.append((fn,mp,"DIVERGENCE",div.group(1)[:70])); print(f"*** DIVERGENCE {mp}::{fn}  {div.group(1)[:60]}")
        elif "error[" in txt or "could not compile" in txt: results.append((fn,mp,"BUILD-FAIL",""))
        else: results.append((fn,mp,"CLEAN",f"~{a.secs}s"))
    print("\n==== CRATE SWEEP SUMMARY ====")
    for fn,mp,st,note in sorted(results,key=lambda x:x[2]):
        if st in ("DIVERGENCE","CLEAN"): print(f"  {st:11} {mp}::{fn}  {note}")
    for st in ("DIVERGENCE","CLEAN","UNSUPPORTED","BUILD-FAIL"):
        print(f"  {st}: {sum(1 for r in results if r[2]==st)}")

if __name__=="__main__":
    main()
