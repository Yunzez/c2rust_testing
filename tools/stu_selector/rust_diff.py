#!/usr/bin/env python3
"""Auto base-vs-lifted Rust differential generator (C2SaferRust-style artifacts).

C2SaferRust (and Laertes/CROWN) ship BOTH a faithful base c2rust module and a lifted WIP module for the
SAME source, function names preserved. This generates a cargo-fuzz differential that calls the base
function (the faithful oracle) and the WIP function (the translation) on the SAME logical inputs and
compares return VALUES -- catching SILENT semantic diffs (no crash needed). It handles the common
reshaping: a buffer that the lift turned `*const T` -> `&[T]` (or `*mut T` -> `&mut [T]`), with scalar
args passed through. This is the auto-sweep engine for finding crc32-class silent bugs at scale.

Usage: rust_diff.py --base base.rs --wip wip.rs --entry NAME --out DIR [--extra-drop fn1,fn2]
The two .rs are self-contained modules (types already concrete; #[no_mangle] stripped is fine).
"""
from __future__ import annotations
import argparse, re, sys
from pathlib import Path

SCALARS = {"u8","u16","u32","u64","usize","i8","i16","i32","i64","isize","bool","c_int","c_uint",
           "c_long","c_ulong","c_char","c_uchar","f32","f64"}

def norm_ty(t: str) -> str:
    t = t.strip()
    t = re.sub(r"\bstd::os::raw::", "", t)
    t = re.sub(r"\bstd::ffi::", "", t)
    return t.strip()

def find_sig(src: str, entry: str):
    """Return (params:list[(name,ty)], ret:str|None) for `fn entry(...)` in src, else None."""
    m = re.search(rf'\bfn\s+{re.escape(entry)}\s*(?:<[^>]*>)?\s*\(', src)
    if not m: return None
    i = m.end() - 1
    depth = 0; j = i
    while j < len(src):
        c = src[j]
        if c == '(': depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0: break
        j += 1
    params_str = src[i+1:j]
    rest = src[j+1:j+120]
    rm = re.match(r'\s*->\s*([^{;]+)', rest)
    ret = norm_ty(rm.group(1)) if rm else None
    params = []
    if params_str.strip():
        for p in split_top(params_str):
            p = p.strip()
            if not p: continue
            nm, _, ty = p.partition(':')
            params.append((nm.strip().lstrip('mut ').strip(), norm_ty(ty)))
    return params, ret

def split_top(s: str):
    out=[]; d=0; cur=""
    for c in s:
        if c in "<([": d+=1
        elif c in ">)]": d-=1
        if c=="," and d==0: out.append(cur); cur=""
        else: cur+=c
    if cur.strip(): out.append(cur)
    return out

def parse_aliases(src: str) -> dict:
    """`pub type X = Y;` -> {X: Y} (Y normalized)."""
    m = {}
    for a in re.finditer(r'\btype\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([^;]+);', src):
        m[a.group(1)] = norm_ty(a.group(2))
    return m

def resolve(ty: str, aliases: dict, depth=0) -> str:
    """Follow type aliases until scalar/ptr/slice or fixpoint; recurse into pointee/element."""
    ty = ty.strip()
    if depth > 12: return ty
    # recurse into structure, resolving the inner element type
    m = re.match(r'^(\*const |\*mut )(.+)$', ty)
    if m: return m.group(1) + resolve(m.group(2), aliases, depth+1)
    m = re.match(r'^&(mut )?\[\s*(.+?)\s*\]$', ty)
    if m: return f"&{'mut ' if m.group(1) else ''}[{resolve(m.group(2), aliases, depth+1)}]"
    m = re.match(r'^&(mut )?(.+)$', ty)
    if m and (m.group(2).strip() in aliases): return f"&{'mut ' if m.group(1) else ''}{resolve(m.group(2), aliases, depth+1)}"
    if ty in aliases and aliases[ty] != ty:
        return resolve(aliases[ty], aliases, depth+1)
    return ty

def role_of(ty: str, aliases: dict = None):
    ty = resolve(ty, aliases or {})
    ty = ty.strip()
    if re.match(r'&(mut )?\[', ty): return ("slice", re.search(r'\[\s*([a-z0-9_]+)', ty).group(1))
    if ty.startswith("*const") or ty.startswith("*mut"):
        return ("ptr", ty.split()[-1])
    base = re.sub(r"^&(mut )?", "", ty).strip()
    if base in SCALARS: return ("scalar", base)
    return ("other", ty)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True); ap.add_argument("--wip", required=True)
    ap.add_argument("--entry", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--crate-name", default="rdiff")
    a = ap.parse_args()
    base_src = Path(a.base).read_text(); wip_src = Path(a.wip).read_text()
    bp = find_sig(base_src, a.entry); wp = find_sig(wip_src, a.entry)
    if not bp: sys.exit(f"entry {a.entry} not found in base")
    if not wp: sys.exit(f"entry {a.entry} not found in wip")
    bparams, bret = bp; wparams, wret = wp
    balias = parse_aliases(base_src); walias = parse_aliases(wip_src)
    if len(bparams) != len(wparams):
        sys.exit(f"UNSUPPORTED: arg count differs base={len(bparams)} wip={len(wparams)} "
                 f"(needs matcher-level alignment)")
    # classify each position by (base_role, wip_role)
    LENISH = {"len","n","size","count","cap","length","sz","num","nbytes","outlen"}
    plan = []
    seen_buf = False
    for (bn,bt),(wn,wt) in zip(bparams, wparams):
        br = role_of(bt, balias); wr = role_of(wt, walias)
        if br[0]=="scalar" and wr[0]=="scalar":
            # a length-like scalar following a buffer must be BOUND to the buffer length,
            # not fuzzed independently (else len != buf.len() manufactures a false divergence).
            nm = (wn or bn or "").lower().strip("_")
            if seen_buf and nm in LENISH:
                plan.append(("buflen", bt, wt))
            else:
                plan.append(("scalar", bt, wt))
        elif br[0] in ("ptr","slice") and wr[0] in ("ptr","slice"):
            plan.append(("buf", bt, wt)); seen_buf = True
        else:
            sys.exit(f"UNSUPPORTED param: base {bt} / wip {wt} (role {br[0]}/{wr[0]})")
    # need exactly one buffer for the simple generator; scalars carry any lengths
    nbuf = sum(1 for p in plan if p[0]=="buf")
    if nbuf > 1: sys.exit("UNSUPPORTED: >1 buffer (extend generator)")

    out = Path(a.out); (out/"src").mkdir(parents=True, exist_ok=True); (out/"fuzz"/"fuzz_targets").mkdir(parents=True, exist_ok=True)
    (out/"src"/"base.rs").write_text(base_src)
    (out/"src"/"wip.rs").write_text(wip_src)
    (out/"src"/"lib.rs").write_text(
        "#![allow(warnings)]\npub mod base;\npub mod wip;\n")

    # build the driver. Cast to RESOLVED scalar types (driver can't see module-local aliases).
    dec, base_args, wip_args = [], [], []
    for idx,(kind,bt,wt) in enumerate(plan):
        btr = resolve(bt, balias); wtr = resolve(wt, walias)
        if kind in ("scalar","buflen"):
            wbase = re.sub(r"^&(mut )?", "", wtr).strip()
            src_val = "vbuf.len() as" if kind=="buflen" else "cur.take_u64() as"
            note = "   // bound to buffer length" if kind=="buflen" else ""
            dec.append(f"    let a{idx} = {src_val} {wbase};{note}")
            base_args.append(f"a{idx} as {re.sub(r'^&(mut )?','',btr).strip()}")
            wip_args.append(f"a{idx}")
        else:  # buf
            welem = role_of(wt, walias)[1]
            dec.append(f"    let n = (cur.byte() as usize) % 65;")
            dec.append(f"    let mut vbuf: Vec<{welem}> = (0..n).map(|_| cur.byte() as {welem}).collect();")
            if role_of(bt, balias)[0]=="ptr":
                cst = "as_ptr" if btr.startswith("*const") else "as_mut_ptr"
                base_args.append(f"vbuf.{cst}() as {btr}")
            else:
                base_args.append("&mut vbuf[..]" if btr.startswith("&mut") else "&vbuf[..]")
            if role_of(wt, walias)[0]=="slice":
                wip_args.append("&mut vbuf[..]" if wtr.startswith("&mut") else "&vbuf[..]")
            else:
                cst = "as_ptr" if wtr.startswith("*const") else "as_mut_ptr"
                wip_args.append(f"vbuf.{cst}() as {wtr}")
    def cmp_expr(ret):
        if ret and ret.startswith("Option"):
            return ('format!("{:?}", {X})')
        return '{X}'
    base_call = f"unsafe {{ base::{a.entry}({', '.join(base_args)}) }}"
    wip_call  = f"unsafe {{ wip::{a.entry}({', '.join(wip_args)}) }}"
    # normalize returns to a comparable String
    def norm_ret(expr, ret):
        return f'format!("{{:?}}", {expr})'
    driver = f'''#![no_main]
#![allow(warnings)]
use libfuzzer_sys::fuzz_target;
use std::os::raw::*;
use {a.crate_name} as t;
struct Cur<'a> {{ d: &'a [u8], p: usize }}
impl<'a> Cur<'a> {{
    fn new(d: &'a [u8]) -> Self {{ Cur {{ d, p: 0 }} }}
    fn byte(&mut self) -> u8 {{ let b = if self.p < self.d.len() {{ self.d[self.p] }} else {{ 0 }}; self.p += 1; b }}
    fn take_u64(&mut self) -> u64 {{ let mut v=0u64; for i in 0..8 {{ v |= (self.byte() as u64) << (8*i); }} v }}
}}
fuzz_target!(|data: &[u8]| {{
    let mut cur = Cur::new(data);
{chr(10).join(dec)}
    let bres = {base_call.replace("base::", "t::base::")};
    let wres = {wip_call.replace("wip::", "t::wip::")};
    let b = {norm_ret("bres", bret)};
    let w = {norm_ret("wres", wret)};
    if b != w {{
        panic!("divergence: base={{:?}} wip={{:?}}", b, w);
    }}
}});
'''
    (out/"fuzz"/"fuzz_targets"/f"{a.entry}_ft.rs").write_text(driver)
    (out/"Cargo.toml").write_text(f'''[package]
name="{a.crate_name}"
version="0.0.0"
edition="2018"
[lib]
name="{a.crate_name}"
path="src/lib.rs"
[profile.release]
opt-level=3
[workspace]
''')
    (out/"fuzz"/"Cargo.toml").write_text(f'''[package]
name="{a.crate_name}-fuzz"
version="0.0.0"
edition="2018"
[package.metadata]
cargo-fuzz=true
[dependencies]
libfuzzer-sys="0.4"
[dependencies.{a.crate_name}]
path=".."
[[bin]]
name="{a.entry}_ft"
path="fuzz_targets/{a.entry}_ft.rs"
test=false
doc=false
[workspace]
''')
    print(f"OK entry={a.entry} base_sig={bparams}->{bret} wip_sig={wparams}->{wret}")
    print(f"  plan={[p[0] for p in plan]}")
    print(f"  run: cd {out} && cargo +nightly-2025-09-01 fuzz run {a.entry}_ft")

if __name__ == "__main__":
    main()
