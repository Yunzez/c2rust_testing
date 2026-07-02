#!/usr/bin/env python3
"""Out-of-process differential harness generator (the DEFAULT harness path).

Unlike the in-process generator (gen_diff_harness.py), which links C and Rust into one binary and
calls C via FFI (requiring C-ABI-shaped `extern "C"` boundaries), this emits:

  <out>/oracle/<name>_oracle.c   a standalone C program: read stdin bytes -> decode args -> call the
                                 real C entry -> serialize outputs to stdout. Built with UBSan+ASan;
                                 a nonzero exit == C hit UB on this input (the UB gate == attribution).
  <out>/ (cargo-fuzz crate)      lib.rs = the Rust translation (any signature shape, called NATIVELY,
                                 no FFI); fuzz target decodes the SAME byte format, calls the Rust
                                 entry, runs the C oracle subprocess, gates on its exit, compares.

Out-of-process is the general default: each side is a normal program, so ANY translation works
(c2rust / CROWN / C2SaferRust / SACTOR / hand-ports). In-process stays as a speed specialization
for C-ABI boundaries. Reuses gen_diff_harness's libclang signature parsing + role classification.

v1 param roles (shared with in-process): scalar, input_buffer (const T*+len), inout_buffer (T*+len),
out_scalar (T*), plus the scalar return value. The RUST call adapts to the translation's signature:
  - raw-ptr shape  (c2rust/CROWN):  translated::foo(v.as_mut_ptr(), n, ...)
  - slice shape    (idiomatic):     translated::foo(&mut v, ...)     [detected from the .rs sig]
Anything outside these roles is reported UNSUPPORTED (recorded, not silently dropped).

Usage: gen_oop_harness.py --pair <dir> --entry <fn> [--rust-entry <fn>] --out <dir> [--cap N]
"""
from __future__ import annotations
import argparse, re, sys, shutil
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import gen_diff_harness as gdh  # reuse parse_entry_signature / classify / map_scalar

# Rust scalar -> C type (for the oracle's extern-free direct call) and byte width
RUST2C = {"i8": "int8_t", "u8": "uint8_t", "i16": "int16_t", "u16": "uint16_t",
          "i32": "int32_t", "u32": "uint32_t", "i64": "int64_t", "u64": "uint64_t",
          "usize": "size_t", "isize": "ssize_t", "void": "void",
          "f32": "float", "f64": "double", "bool": "int"}
FLOAT_BITS = {"f32": ("uint32_t", "u32", 4), "f64": ("uint64_t", "u64", 8)}  # rust -> (C uint, rust uint, w)
CAP = 64  # max buffer elements decoded from the fuzz input


def rust_entry_sig(rs_text: str, entry: str):
    """Return ('raw_ptr'|'slice'|None, raw_sig_str). Detect whether the Rust entry takes raw
    pointers (c2rust shape) or slices (idiomatic), to pick the native call form."""
    m = re.search(rf'fn {re.escape(entry)}\s*\(([^)]*)\)', rs_text)
    if not m:
        return None, None
    sig = m.group(1)
    if "&mut [" in sig or "&[" in sig:
        return "slice", sig
    if "*mut" in sig or "*const" in sig:
        return "raw_ptr", sig
    return "scalar_only", sig  # only scalars (e.g. morton)


# ---------------------------------------------------------------------------- C oracle emitter
def emit_oracle_c(entry, items, ret, csrcs, cap):
    """Standalone C: stdin bytes -> decode -> call entry -> serialize (ret + inout/out params)."""
    incl = "".join(f'#include "{c.name}"\n' for c in csrcs)
    decode, call_args, ser = [], [], []
    for it in items:
        r = it["role"]
        nm = it["name"]
        if r == "scalar":
            cty = RUST2C.get(it["rust"], "long"); w = it["w"]
            if it["rust"] in FLOAT_BITS:  # bit-reinterpret raw bytes into the float (not int->float cast)
                ubits, _, fw = FLOAT_BITS[it["rust"]]
                decode.append(f"    {ubits} {nm}_b = ({ubits})take_uint({fw}); {cty} {nm}; "
                              f"__builtin_memcpy(&{nm}, &{nm}_b, {fw});")
            else:
                decode.append(f"    {cty} {nm} = ({cty})take_uint({w});")
            call_args.append(nm)
        elif r in ("in_buf", "io_buf"):
            if it["elem"] in FLOAT_BITS:  # float buffers deferred (need per-elem bit handling)
                raise SystemExit(f"oop-unsupported: float buffer {nm} (scalar floats supported; buffers deferred)")
            ety = RUST2C.get(it["elem"], "long"); ew = it["elem_w"]; ln = it["len_name"]
            decode.append(f"    size_t {ln} = (size_t)(next_byte() % ({cap}+1));")
            decode.append(f"    {ety} {nm}[{cap}]; for (size_t i=0;i<{ln};i++) {nm}[i]=({ety})take_uint({ew});")
            call_args.append(nm)
            call_args.append(ln)   # the C signature has (ptr, len) adjacent -> pass BOTH
            if r == "io_buf":
                ser.append(f'    printf(" buf:{nm}:%zu", {ln}); '
                           f'for (size_t i=0;i<{ln};i++) printf(":%lld",(long long){nm}[i]);')
        elif r == "out_scalar":
            elem = it["elem"]  # ptr-to-scalar: the pointee type is in "elem", not "rust"
            if elem in FLOAT_BITS:  # float out-scalar deferred
                raise SystemExit(f"oop-unsupported: float out-scalar {nm} (deferred)")
            ety = "_Bool" if elem == "bool" else RUST2C.get(elem, "long")  # bool* needs _Bool*, not int*
            decode.append(f"    {ety} {nm}_cell = 0; {ety}* {nm} = &{nm}_cell;")
            call_args.append(nm)
            ser.append(f'    printf(" out:{nm}:%lld",(long long){nm}_cell);')
        else:
            raise SystemExit(f"oop-unsupported role {r} for {nm}")
    ret_c = RUST2C.get(ret, "void")
    if ret == "void":
        callline = f"    {entry}({', '.join(call_args)});"
        retser = ""
    elif ret in FLOAT_BITS:  # canonical-NaN + bit compare (finite/inf are bit-exact with -ffp-contract=off)
        ubits, _, fw = FLOAT_BITS[ret]
        callline = f"    {ret_c} _ret = {entry}({', '.join(call_args)});"
        retser = (f'    if (__builtin_isnan(_ret)) {{ printf("ret:nan"); }} else {{ '
                  f'{ubits} _rb; __builtin_memcpy(&_rb, &_ret, {fw}); '
                  f'printf("ret:%llu",(unsigned long long)_rb); }}')
    else:
        callline = f"    {ret_c} _ret = {entry}({', '.join(call_args)});"
        retser = '    printf("ret:%lld",(long long)_ret);'
    return f"""/* AUTO-GENERATED out-of-process C oracle for `{entry}`.
 * stdin bytes -> decode -> call the real C -> serialize (ret + inout/out params) to stdout.
 * Build with -fsanitize=undefined,address -fno-sanitize-recover=all: nonzero exit == C UB (gate). */
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
{incl}
static unsigned char _buf[1<<16]; static size_t _got, _pos;
static unsigned char next_byte(void){{ return _pos < _got ? _buf[_pos++] : (_pos++, 0); }}
static long long take_uint(int w){{ unsigned long long v=0; for(int b=0;b<w;b++) v |= (unsigned long long)next_byte()<<(8*b); return (long long)v; }}

int main(void){{
    _got = fread(_buf,1,sizeof _buf,stdin); _pos=0;
{chr(10).join(decode)}
{callline}
{retser}
{chr(10).join(ser)}
    printf("\\n");
    return 0;
}}
"""


# ---------------------------------------------------------------------------- Rust fuzz emitter
def emit_fuzz_rs(entry, rust_entry, items, ret, call_kind, oracle_rel, crate_name):
    dec, call_args, cmp_ser = [], [], []
    for it in items:
        r = it["role"]; nm = it["name"]
        if r == "scalar":
            if it["rust"] in FLOAT_BITS:  # bit-reinterpret (not int->float cast)
                _, uty, fw = FLOAT_BITS[it["rust"]]
                dec.append(f"    let {nm} = {it['rust']}::from_bits(cur.take_{uty}() as {uty});")
            elif it["rust"] == "bool":  # `x as bool` is invalid Rust
                dec.append(f"    let {nm} = cur.byte() != 0;")
            else:
                dec.append(f"    let {nm} = cur.take_{_takef(it['rust'], it['w'])}() as {it['rust']};")
            call_args.append(nm)
        elif r in ("in_buf", "io_buf"):
            if it["elem"] in FLOAT_BITS:
                raise SystemExit(f"oop-unsupported: float buffer {nm} (deferred)")
            ety = it["elem"]; ln = it["len_name"]
            dec.append(f"    let {ln} = (cur.byte() as usize) % ({CAP}+1);")
            dec.append(f"    let mut {nm}: Vec<{ety}> = (0..{ln}).map(|_| cur.take_{_takef(ety, it['elem_w'])}() as {ety}).collect();")
            if call_kind == "slice":
                call_args.append(f"&mut {nm}[..]")
            else:
                call_args.append(f"{nm}.as_mut_ptr()")
                # raw-ptr form also needs the length arg passed positionally (handled by scalar len below)
            if r == "io_buf":
                cmp_ser.append(f'    r_out.push_str(&format!(" buf:{nm}:{{}}", {ln}));'
                               f' for &x in &{nm}[..{ln}] {{ r_out.push_str(&format!(":{{}}", x as i64)); }}')
        elif r == "out_scalar":
            elem = it["elem"]
            if elem in FLOAT_BITS:
                raise SystemExit(f"oop-unsupported: float out-scalar {nm} (deferred)")
            dec.append(f"    let mut {nm}_cell: {elem} = {'false' if elem == 'bool' else '0'};")
            call_args.append(f"&mut {nm}_cell" if call_kind == "slice" else f"&mut {nm}_cell as *mut {elem}")
            cmp_ser.append(f'    r_out.push_str(&format!(" out:{nm}:{{}}", {nm}_cell as i64));')
        else:
            raise SystemExit(f"oop-unsupported role {r}")
    # length params for raw-ptr calls: c2rust sig has (ptr, len) so pass len positionally after ptr.
    # Reconstruct the positional call: walk items, for buffers in raw_ptr mode inject the len value.
    if call_kind == "raw_ptr":
        call_args = _raw_ptr_call_args(items)
    is_unsafe = call_kind == "raw_ptr"
    callexpr = f"translated::{rust_entry}({', '.join(call_args)})"
    if is_unsafe:
        callexpr = f"unsafe {{ {callexpr} }}"
    if ret == "void":
        callblock = f"    {callexpr};\n    let mut r_out = String::new();"
    elif ret in FLOAT_BITS:  # canonical-NaN + bit compare (matches the C oracle)
        callblock = (f"    let _ret = {callexpr};\n    let mut r_out = if _ret.is_nan() "
                     f"{{ \"ret:nan\".to_string() }} else {{ format!(\"ret:{{}}\", _ret.to_bits()) }};")
    else:
        callblock = f"    let _ret = {callexpr};\n    let mut r_out = format!(\"ret:{{}}\", _ret as i64);"
    return f"""#![no_main]
//! AUTO-GENERATED out-of-process differential harness for `{entry}`.
//! Rust entry called NATIVELY ({call_kind}); C runs as a subprocess oracle (UB gate = its exit code).
use libfuzzer_sys::fuzz_target;
use std::io::Write;
use std::process::{{Command, Stdio}};

const CAP: usize = {CAP};
const ORACLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/{oracle_rel}");

struct Cur<'a> {{ d: &'a [u8], p: usize }}
impl<'a> Cur<'a> {{
    fn new(d: &'a [u8]) -> Self {{ Cur {{ d, p: 0 }} }}
    fn byte(&mut self) -> u8 {{ let b = if self.p < self.d.len() {{ self.d[self.p] }} else {{ 0 }}; self.p += 1; b }}
    fn take(&mut self, w: usize) -> u64 {{ let mut v = 0u64; for i in 0..w {{ v |= (self.byte() as u64) << (8*i); }} v }}
    fn take_u8(&mut self)->u64{{self.take(1)}} fn take_u16(&mut self)->u64{{self.take(2)}}
    fn take_u32(&mut self)->u64{{self.take(4)}} fn take_u64(&mut self)->u64{{self.take(8)}}
}}

fn run_oracle(data: &[u8]) -> Option<String> {{
    let mut ch = Command::new(ORACLE).stdin(Stdio::piped()).stdout(Stdio::piped())
        .stderr(Stdio::null()).spawn().ok()?;
    ch.stdin.take()?.write_all(data).ok();
    let out = ch.wait_with_output().ok()?;
    if !out.status.success() {{ return None; }}   // C UB / crash -> gate: discard this input
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}}

fuzz_target!(|data: &[u8]| {{
    let c_out = match run_oracle(data) {{ Some(s) => s, None => return }};
    let mut cur = Cur::new(data);
{chr(10).join(dec)}
{callblock}
{chr(10).join(cmp_ser)}
    let r_out = r_out.trim().to_string();
    assert_eq!(c_out, r_out, "divergence: C={{:?}} Rust={{:?}}", c_out, r_out);
}});

use {crate_name} as translated;
"""


def _takef(rust, w):
    return {1: "u8", 2: "u16", 4: "u32", 8: "u64"}[w]


def _raw_ptr_call_args(items):
    """Positional args for a c2rust-shape call: buffer -> (ptr, len) adjacent; scalars as-is."""
    args = []
    for it in items:
        r = it["role"]; nm = it["name"]
        if r == "scalar":
            args.append(nm)
        elif r in ("in_buf", "io_buf"):
            args.append(f"{nm}.as_mut_ptr()")
            args.append(f"{it['len_name']} as {it['len_rust']}")
        elif r == "out_scalar":
            args.append(f"&mut {nm}_cell as *mut {it['elem']}")
    # dedup a len that also appears as its own scalar item (buffer owns it)
    return args


def apply_bitfields(lines: list[str]) -> list[str]:
    """Insert the c2rust-bitfields import after the leading inner-attr block, if the crate needs it."""
    if not any("BitfieldStruct" in l for l in lines):
        return lines
    last = -1
    for idx, l in enumerate(lines):
        if l.strip().startswith("#!"):
            depth = l.count("(") - l.count(")"); last = idx; j = idx
            while depth > 0 and j + 1 < len(lines):
                j += 1; depth += lines[j].count("(") - lines[j].count(")"); last = j
        elif last >= 0 and idx > last:
            break
    if "c2rust_bitfields" not in "\n".join(lines[:max(last + 3, 3)]):
        lines.insert(last + 1, "#[macro_use] extern crate c2rust_bitfields;")
    return lines


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--pair", required=True)
    ap.add_argument("--entry", required=True)
    ap.add_argument("--rust-entry", default=None)
    ap.add_argument("--out", required=True)
    ap.add_argument("--cap", type=int, default=CAP)
    args = ap.parse_args()
    pair = Path(args.pair); out = Path(args.out)
    rust_entry = args.rust_entry or args.entry
    cc_dir = pair / "build"
    params, ret, _ = gdh.parse_entry_signature(cc_dir, args.entry)
    items = gdh.classify(params)
    param_rust = {p["name"]: p.get("rust") for p in params if p.get("kind") == "scalar"}
    # attach rust element types / len rust types for buffers (classify uses C-side)
    for it in items:
        if it.get("role") in ("in_buf", "io_buf"):
            it.setdefault("elem", it.get("elem", "u8"))
            it.setdefault("elem_w", it.get("elem_w", 1))
            # the length param's real Rust type (e.g. u32), NOT a hardcoded usize
            it["len_rust"] = param_rust.get(it.get("len_name")) or "usize"
    rs = sorted((pair / "translated").glob("*.rs"))[0]
    call_kind, _sig = rust_entry_sig(rs.read_text(), rust_entry)
    call_kind = "raw_ptr" if call_kind in ("raw_ptr", "scalar_only") else "slice"

    out.mkdir(parents=True, exist_ok=True)
    (out / "oracle").mkdir(exist_ok=True)

    # --- C oracle: sources + headers + generated driver, built with UBSan+ASan ---
    csrcs = sorted((pair / "source").glob("*.c"))
    for c in csrcs:
        shutil.copy2(c, out / "oracle" / c.name)
    for h in (pair / "source").glob("*.h"):
        shutil.copy2(h, out / "oracle" / h.name)
    (out / "oracle" / f"{args.entry}_oracle.c").write_text(emit_oracle_c(args.entry, items, ret, csrcs, args.cap))
    import subprocess
    obin = out / "oracle" / f"{args.entry}_oracle"
    cc = subprocess.run(["clang", "-fsanitize=undefined,address", "-fno-sanitize-recover=all",
                         "-ffp-contract=off",  # match Rust: no FMA fusion, so finite floats are bit-exact
                         "-g", "-O1", "-I", str(out / "oracle"),
                         str(out / "oracle" / f"{args.entry}_oracle.c"), "-o", str(obin)],
                        text=True, capture_output=True)
    if cc.returncode != 0:
        print("ORACLE_BUILD_FAIL:", cc.stderr[-400:]); return 1

    # --- Rust crate: lib.rs = the translation (native call), fuzz target = the OOP harness ---
    crate = re.sub(r"[^a-zA-Z0-9_]", "_", f"oop_{pair.name}")
    (out / "src").mkdir(exist_ok=True)
    lib_lines = rs.read_text().splitlines()
    lib_lines = apply_bitfields(lib_lines)
    lib_text = "\n".join(lib_lines)
    # Expose the entry: static C fns become private Rust fns (`extern "C" fn NAME`), uncallable as
    # translated::NAME. Add `pub` so the harness can call the boundary natively (the needs_expose case).
    lib_text = re.sub(rf'(?m)^(\s*)((?:unsafe )?extern "C" fn {re.escape(rust_entry)}\b)',
                      r'\1pub \2', lib_text)
    (out / "src" / "lib.rs").write_text(lib_text + "\n")
    needs_bf = any("BitfieldStruct" in l for l in lib_lines)
    (out / "Cargo.toml").write_text(
        f'[package]\nname = "{crate}"\nversion = "0.1.0"\nedition = "2021"\n'
        f'[lib]\nname = "{crate}"\npath = "src/lib.rs"\n'
        + ('[dependencies]\nc2rust-bitfields = "0.22"\n' if needs_bf else "")
        + "[workspace]\n")
    fz = out / "fuzz"; (fz / "fuzz_targets").mkdir(parents=True, exist_ok=True)
    oracle_rel = f"oracle/{args.entry}_oracle"
    (fz / "fuzz_targets" / f"{crate}_ft.rs").write_text(
        emit_fuzz_rs(args.entry, rust_entry, items, ret, call_kind, "../" + oracle_rel, crate))
    (fz / "Cargo.toml").write_text(
        f'[package]\nname = "{crate}-fuzz"\nversion = "0.0.0"\nedition = "2021"\n'
        f'[package.metadata]\ncargo-fuzz = true\n[dependencies]\nlibfuzzer-sys = "0.4"\n'
        f'{crate} = {{ path = ".." }}\n'
        + ('c2rust-bitfields = "0.22"\n' if needs_bf else "")
        + f'[[bin]]\nname = "{crate}_ft"\npath = "fuzz_targets/{crate}_ft.rs"\ntest = false\ndoc = false\n'
          f'[workspace]\n')

    print(f"entry={args.entry} ret={ret} call_kind={call_kind} crate={crate} bitfields={needs_bf}")
    print("roles:", [(it['name'], it['role']) for it in items])
    print(f"oracle built: {obin}")
    print(f"fuzz target: {crate}_ft")
    print(f"run: cargo +{gdh.__dict__.get('TOOLCHAIN','nightly-2025-09-01')} fuzz run {crate}_ft  (cwd {out})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
