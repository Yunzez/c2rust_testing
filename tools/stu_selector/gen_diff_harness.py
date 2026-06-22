#!/usr/bin/env python3

"""Generate a differential fuzz harness for a selected STU (entry function).

Given a c2rust pair (C source + translated .rs) and an entry function name, emit a
cargo-fuzz project that:
  - compiles the C source as an oracle, renaming every C function `f` -> `c_f`
    (sanitizer-coverage instrumented), so it does not collide with the #[no_mangle]
    Rust translation;
  - uses the translated .rs as the lib crate;
  - decodes the fuzz bytes into the entry's parameters (via a byte cursor), calls both
    `c_<entry>` and `translated::<entry>`, and panics on any divergence (return value or
    mutated/out buffers).

This is the harness side of the STU pipeline; used for G1 validation (on equivalent c2rust
output the false-divergence rate should be ~0). Supported parameter shapes: scalars,
`const T* + len` input buffers, `T* + len` in/out buffers, and `T*` out-scalars. Function
pointers / nested pointers are not supported (those boundaries are hard-gated by the STU
selector anyway).

Usage:
  python3 tools/stu_selector/gen_diff_harness.py --pair benchmark/pairs/intmath --entry intmath_eval
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402
import clang.cindex  # noqa: E402
from clang.cindex import CursorKind, Index, TypeKind  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]

# c2rust type spelling -> (rust_ffi_type, byte_width). size_t -> usize to match translation.
SCALAR_MAP = {
    "size_t": ("usize", 8),
    "uint64_t": ("u64", 8), "int64_t": ("i64", 8),
    "uint32_t": ("u32", 4), "int32_t": ("i32", 4),
    "uint16_t": ("u16", 2), "int16_t": ("i16", 2),
    "uint8_t": ("u8", 1), "int8_t": ("i8", 1),
    "int": ("i32", 4), "unsigned int": ("u32", 4), "unsigned": ("u32", 4),
    "long": ("i64", 8), "unsigned long": ("u64", 8),
    "long long": ("i64", 8), "unsigned long long": ("u64", 8),
    "char": ("i8", 1), "signed char": ("i8", 1), "unsigned char": ("u8", 1),
    "_Bool": ("bool", 1),
}


def map_scalar(spelling: str) -> tuple[str, int] | None:
    s = spelling.replace("const ", "").strip()
    return SCALAR_MAP.get(s)


def parse_entry_signature(cc_dir: Path, entry: str) -> tuple[list[dict], str, list[str]]:
    """Return (params, ret_rust_type, all_function_names) for the entry via libclang."""
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    import os
    cwd0 = os.getcwd()
    params: list[dict] = []
    ret = "i32"
    all_fns: list[str] = []
    for cmd in cdb.getAllCompileCommands():
        src_abs = str((Path(cmd.directory) / cmd.filename).resolve())
        names = {cmd.filename, src_abs, Path(cmd.filename).name}
        args = cgmod._filter_compile_args(list(cmd.arguments), names)
        os.chdir(cmd.directory if Path(cmd.directory).exists() else cc_dir)
        try:
            tu = index.parse(src_abs, args=args)
        finally:
            os.chdir(cwd0)
        for cur in tu.cursor.walk_preorder():
            if cur.kind != CursorKind.FUNCTION_DECL or not cur.is_definition():
                continue
            f = cur.location.file.name if cur.location and cur.location.file else None
            if not f or f.startswith(("/usr/", "/lib/")):
                continue
            all_fns.append(cur.spelling)
            if cur.spelling != entry:
                continue
            for a in cur.get_arguments():
                t = a.type
                if t.kind == TypeKind.POINTER:
                    pointee = t.get_pointee()
                    is_const = pointee.is_const_qualified()
                    base = map_scalar(pointee.spelling)
                    if base is None:
                        raise SystemExit(f"unsupported pointer element type: {pointee.spelling}")
                    params.append({"kind": "ptr", "const": is_const,
                                   "elem": base[0], "elem_w": base[1], "name": a.spelling})
                else:
                    sc = map_scalar(t.spelling)
                    if sc is None:
                        raise SystemExit(f"unsupported scalar param type: {t.spelling}")
                    params.append({"kind": "scalar", "rust": sc[0], "w": sc[1], "name": a.spelling})
            rt = cur.result_type
            if rt.kind == TypeKind.VOID:
                ret = "void"
            else:
                sc = map_scalar(rt.spelling)
                ret = sc[0] if sc else "i32"
    return params, ret, sorted(set(all_fns))


def classify(params: list[dict]) -> list[dict]:
    """Pair pointers with a following integer length param; tag in/out/out-scalar."""
    out = []
    i = 0
    while i < len(params):
        p = params[i]
        if p["kind"] == "ptr":
            nxt = params[i + 1] if i + 1 < len(params) else None
            if nxt and nxt["kind"] == "scalar":
                role = "in_buf" if p["const"] else "io_buf"
                out.append({**p, "role": role, "len_name": nxt["name"]})
                i += 2
                continue
            out.append({**p, "role": "out_scalar"})
        else:
            out.append({**p, "role": "scalar"})
        i += 1
    return out


def gen_target(entry: str, items: list[dict], ret: str) -> str:
    """Generate the fuzz_target body."""
    decode, c_args, r_args, post = [], [], [], []
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            decode.append(f"    let {n} = cur.take_{it['rust']}();")
            c_args.append(n)
            r_args.append(n)
        elif it["role"] == "in_buf":
            decode.append(f"    let {n}_buf: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_buf.len();")
            c_args += [f"{n}_buf.as_ptr()", it["len_name"]]
            r_args += [f"{n}_buf.as_ptr()", it["len_name"]]
        elif it["role"] == "io_buf":
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_c.len();")
            decode.append(f"    let mut {n}_r = {n}_c.clone();")
            c_args += [f"{n}_c.as_mut_ptr()", it["len_name"]]
            r_args += [f"{n}_r.as_mut_ptr()", it["len_name"]]
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: buffer {n}"); }}')
        elif it["role"] == "out_scalar":
            decode.append(f"    let mut {n}_c: {it['elem']} = 0 as {it['elem']};")
            decode.append(f"    let mut {n}_r: {it['elem']} = 0 as {it['elem']};")
            c_args.append(f"&mut {n}_c")
            r_args.append(f"&mut {n}_r")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: out param {n}"); }}')

    # length params are scalars too in the C signature but provided by buffers above;
    # ensure any scalar that is actually a consumed length is not double-decoded.
    len_names = {it["len_name"] for it in items if "len_name" in it}
    decode = [d for d in decode
              if not any(d.strip().startswith(f"let {ln} =") and "cur.take_" in d for ln in len_names)]

    call_c = f"c_{entry}({', '.join(c_args)})"
    call_r = f"translated::{entry}({', '.join(r_args)})"
    if ret == "void":
        body_call = f"        {call_c};\n        {call_r};"
        ret_cmp = ""
    else:
        body_call = f"        let c_ret = {call_c};\n        let r_ret = {call_r};"
        ret_cmp = '        if c_ret != r_ret { panic!("divergence: return value"); }'

    return "\n".join([
        "#![no_main]",
        'use libfuzzer_sys::fuzz_target;',
        "struct Cur<'a> { d: &'a [u8], p: usize }",
        "impl<'a> Cur<'a> {",
        "    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }",
        "    fn byte(&mut self) -> u8 { let b = if self.p < self.d.len() { self.d[self.p] } else { 0 }; self.p += 1; b }",
        *[f"    fn take_{t}(&mut self) -> {t} {{ let mut v = [0u8; {w}]; for i in 0..{w} {{ v[i] = self.byte(); }} {t}::from_le_bytes(v) }}"
          for t, w in [("u8", 1), ("i8", 1), ("u16", 2), ("i16", 2), ("u32", 4), ("i32", 4),
                       ("u64", 8), ("i64", 8), ("usize", 8)]],
        *[f"    fn take_vec_{t}(&mut self) -> Vec<{t}> {{ let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_{t}()).collect() }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"]],
        "}",
        "",
        "fn cd() -> i8 { 0 }  // silence unused on some shapes",
        "",
        f"use {ENTRY_CRATE} as translated;",
        "extern \"C\" {",
        f"    fn c_{entry}({EXTERN_ARGS}) {EXTERN_RET};",
        "}",
        "",
        "fuzz_target!(|data: &[u8]| {",
        "    let _ = cd();",
        "    let mut cur = Cur::new(data);",
        *decode,
        "    unsafe {",
        body_call,
        ret_cmp,
        *post,
        "    }",
        "});",
        "",
    ])


# placeholders filled in main (kept module-level for the f-string above)
ENTRY_CRATE = "CRATE"
EXTERN_ARGS = ""
EXTERN_RET = ""


def main() -> int:
    global ENTRY_CRATE, EXTERN_ARGS, EXTERN_RET
    ap = argparse.ArgumentParser(description="Generate a differential fuzz harness for an STU")
    ap.add_argument("--pair", required=True, help="benchmark/pairs/<name>")
    ap.add_argument("--entry", required=True)
    ap.add_argument("--out", default=None, help="output project dir (default fuzz_gen/<name>)")
    args = ap.parse_args()

    pair = Path(args.pair)
    name = pair.name
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"))
    c_src = next((pair / "source").glob("*.c"))

    params, ret, all_fns = parse_entry_signature(cc, args.entry)
    items = classify(params)

    # extern "C" decl types/args for c_<entry>
    decl_parts = []
    for it in items:
        if it["role"] == "scalar":
            decl_parts.append(f"{it['name']}: {it['rust']}")
        elif it["role"] in ("in_buf",):
            decl_parts.append(f"{it['name']}: *const {it['elem']}")
        elif it["role"] in ("io_buf",):
            decl_parts.append(f"{it['name']}: *mut {it['elem']}")
        elif it["role"] == "out_scalar":
            decl_parts.append(f"{it['name']}: *mut {it['elem']}")
        if "len_name" in it:
            # the length scalar param keeps its place in the C signature
            ln = next(p for p in params if p["name"] == it["len_name"])
            decl_parts.append(f"{it['len_name']}: {ln['rust']}")
    ENTRY_CRATE = name.replace("-", "_")
    EXTERN_ARGS = ", ".join(decl_parts)
    EXTERN_RET = "" if ret == "void" else f"-> {ret}"

    out = Path(args.out) if args.out else (ROOT / "fuzz_gen" / name)
    out.mkdir(parents=True, exist_ok=True)
    (out / "src").mkdir(exist_ok=True)
    (out / "c").mkdir(exist_ok=True)
    (out / "fuzz" / "fuzz_targets").mkdir(parents=True, exist_ok=True)

    (out / "c" / c_src.name).write_text(c_src.read_text(), encoding="utf-8")
    (out / "src" / "lib.rs").write_text(rs.read_text(), encoding="utf-8")

    (out / "Cargo.toml").write_text(
        f'[package]\nname = "{ENTRY_CRATE}"\nversion = "0.1.0"\nedition = "2021"\n\n'
        f'[build-dependencies]\ncc = "1"\n\n[dependencies]\n', encoding="utf-8")

    defines = "\n".join(f'        .define("{fn}", "c_{fn}")' for fn in all_fns)
    (out / "build.rs").write_text(f'''fn main() {{
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=trace-pc-guard,trace-cmp").warnings(false);
    build
{defines};
    build.file("c/{c_src.name}");
    build.compile("c_oracle");
    let rd = std::process::Command::new("clang").arg("--print-resource-dir").output().unwrap();
    let rd = String::from_utf8(rd.stdout).unwrap().trim().to_string();
    let lib_dir = std::path::Path::new(&rd).join("lib").join("linux");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".into());
    println!("cargo:rustc-link-search=native={{}}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=clang_rt.profile-{{}}", arch);
}}
''', encoding="utf-8")

    (out / "fuzz" / "Cargo.toml").write_text(f'''[package]
name = "{ENTRY_CRATE}-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = {{ version = "0.15.4", package = "libafl_libfuzzer" }}

[dependencies.{ENTRY_CRATE}]
path = ".."

[workspace]
members = ["."]

[profile.release]
debug = 1

[[bin]]
name = "{ENTRY_CRATE}_ft"
path = "fuzz_targets/{ENTRY_CRATE}_ft.rs"
test = false
doc = false
''', encoding="utf-8")

    (out / "fuzz" / "fuzz_targets" / f"{ENTRY_CRATE}_ft.rs").write_text(
        gen_target(args.entry, items, ret), encoding="utf-8")

    print(f"generated harness at {out}")
    print(f"  entry: {args.entry}({EXTERN_ARGS}) {EXTERN_RET}")
    print(f"  roles: {[(it['name'], it['role']) for it in items]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
