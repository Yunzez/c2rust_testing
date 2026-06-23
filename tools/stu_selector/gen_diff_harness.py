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
import json
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


# C param names that are Rust keywords/reserved would produce invalid Rust in the harness.
RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "async", "await", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen",
}


def safe_name(name: str, idx: int) -> str:
    if not name:
        return f"arg{idx}"
    return f"{name}_" if name in RUST_KEYWORDS else name


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
            for idx, a in enumerate(cur.get_arguments()):
                t = a.type
                pname = safe_name(a.spelling, idx)
                if t.kind == TypeKind.POINTER:
                    pointee = t.get_pointee()
                    is_const = pointee.is_const_qualified()
                    base = map_scalar(pointee.spelling)
                    if base is None:
                        raise SystemExit(f"unsupported pointer element type: {pointee.spelling}")
                    params.append({"kind": "ptr", "const": is_const,
                                   "elem": base[0], "elem_w": base[1], "name": pname})
                else:
                    sc = map_scalar(t.spelling)
                    if sc is None:
                        raise SystemExit(f"unsupported scalar param type: {t.spelling}")
                    params.append({"kind": "scalar", "rust": sc[0], "w": sc[1], "name": pname})
            rt = cur.result_type
            if rt.kind == TypeKind.VOID:
                ret = "void"
            else:
                sc = map_scalar(rt.spelling)
                ret = sc[0] if sc else "i32"
    return params, ret, sorted(set(all_fns))


def items_from_schema(schema: dict) -> list[dict]:
    """Reproduce classify()'s items from an explicit schema (schemas/<prog>.json).

    Roles come from the schema, not adjacency. The mapping is exact so that switching the
    generator from classify() to the schema is byte-identical (the schema was derived from
    classify()). output_buffer maps to the same io_buf decode as today (capacity used as the
    vector length) — the richer semantics are deferred to keep byte-identity.
    """
    items = []
    for p in schema["params"]:
        role = p["role"]
        if role == "scalar":
            items.append({"kind": "scalar", "role": "scalar", "name": p["name"],
                          "rust": p["rust"], "w": p["width"]})
        elif role == "input_buffer":
            items.append({"kind": "ptr", "role": "in_buf", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "len_name": p["length_param"]})
        elif role in ("inout_buffer", "output_buffer"):
            ln = p.get("length_param") or p.get("capacity_param")
            items.append({"kind": "ptr", "role": "io_buf", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "len_name": ln})
        elif role == "out_scalar":
            items.append({"kind": "ptr", "role": "out_scalar", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"]})
        elif role == "input_string":
            items.append({"kind": "ptr", "role": "in_str", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"]})
        # length / capacity params are consumed by their owning buffer (not standalone items)
    return items


def _infer_abi(params: list[dict]) -> list[dict]:
    """Synthesize an ABI-ordered schema-param list from inference (the --infer-schema fallback)."""
    its = classify(params)
    by = {p["name"]: p for p in params}
    buf_role, len_owner = {}, {}
    for it in its:
        if it["role"] == "in_buf":
            buf_role[it["name"]] = "input_buffer"; len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "io_buf":
            buf_role[it["name"]] = "inout_buffer"; len_owner[it["len_name"]] = (it["name"], "length")
    out_scalars = {it["name"] for it in its if it["role"] == "out_scalar"}
    abi = []
    for p in params:
        n = p["name"]
        if n in buf_role:
            it = next(i for i in its if i["name"] == n)
            abi.append({"name": n, "role": buf_role[n], "decode": "vector",
                        "elem": it["elem"], "elem_width": it["elem_w"], "length_param": it["len_name"]})
        elif n in len_owner:
            buf, kind = len_owner[n]
            abi.append({"name": n, "role": kind, "decode": "derived_from_buffer",
                        "of_buffer": buf, "rust": p["rust"], "width": p["w"]})
        elif n in out_scalars:
            it = next(i for i in its if i["name"] == n)
            role = "input_string" if by[n].get("const") else "out_scalar"
            decode = "nul_string" if by[n].get("const") else "out_scalar_zero"
            abi.append({"name": n, "role": role, "decode": decode,
                        "elem": it["elem"], "elem_width": it["elem_w"]})
        else:
            abi.append({"name": n, "role": "scalar", "decode": "scalar",
                        "rust": p["rust"], "width": p["w"]})
    return abi


def resolve(name: str, cc_dir: Path, entry: str, infer: bool = False):
    """(params, ret, all_fns, items, abi). Require + strongly validate the schema; --infer to fall back.

    items drive byte-decode (role-based, order preserved per buffer); abi is the schema params in
    ABI order and drives the call arguments + extern signature.
    """
    params, ret, all_fns = parse_entry_signature(cc_dir, entry)
    schema_path = ROOT / "schemas" / f"{name}.json"
    if schema_path.exists():
        import harness_schema as hs
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        errs = hs.validate(schema)
        if schema.get("entry") != entry:
            errs.append(f"schema entry {schema.get('entry')!r} != requested {entry!r}")
        if schema.get("program") != name:
            errs.append(f"schema program {schema.get('program')!r} != pair {name!r}")
        errs += hs.validate_against_signature(schema, params, ret)
        if errs:
            raise SystemExit(f"schema {schema_path} invalid: {errs}")
        return params, ret, all_fns, items_from_schema(schema), schema["params"]
    if infer:
        return params, ret, all_fns, classify(params), _infer_abi(params)
    raise SystemExit(f"no schema for '{name}' at {schema_path}; pass --infer-schema to fall back")


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


def _decode_and_post(items: list[dict]) -> tuple[list[str], list[str]]:
    """Storage + byte-decode + comparison, driven by item ROLE (not order).

    Decode text is identical to the pre-ABI-refactor generator (so the 12 migration entries stay
    byte-identical): each buffer emits its take + its derived-length binding right after it.
    """
    decode, post = [], []
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            decode.append(f"    let {n} = cur.take_{it['rust']}();")
        elif it["role"] == "in_buf":
            decode.append(f"    let {n}_buf: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_buf.len();")
        elif it["role"] == "io_buf":
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_c.len();")
            decode.append(f"    let mut {n}_r = {n}_c.clone();")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: buffer {n}"); }}')
        elif it["role"] == "out_scalar":
            decode.append(f"    let mut {n}_c: {it['elem']} = 0 as {it['elem']};")
            decode.append(f"    let mut {n}_r: {it['elem']} = 0 as {it['elem']};")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: out param {n}"); }}')
        elif it["role"] == "in_str":
            decode.append(f"    let mut {n}_buf: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    {n}_buf.push(0 as {it['elem']});")
    return decode, post


def _call_and_decl(abi: list[dict]) -> tuple[list[str], list[str], list[str]]:
    """Call arguments and extern signature in STRICT schema (ABI) order.

    Iterating the schema params in declaration order is what makes a length param that PRECEDES
    its buffer (e.g. f(size_t n, T* buf, ...)) come out in the right ABI position.
    """
    c_args, r_args, decl = [], [], []
    for p in abi:
        role, n = p["role"], p["name"]
        if role in ("scalar", "length", "capacity"):
            c_args.append(n); r_args.append(n); decl.append(f"{n}: {p['rust']}")
        elif role == "input_buffer":
            c_args.append(f"{n}_buf.as_ptr()"); r_args.append(f"{n}_buf.as_ptr()")
            decl.append(f"{n}: *const {p['elem']}")
        elif role in ("inout_buffer", "output_buffer"):
            c_args.append(f"{n}_c.as_mut_ptr()"); r_args.append(f"{n}_r.as_mut_ptr()")
            decl.append(f"{n}: *mut {p['elem']}")
        elif role == "out_scalar":
            c_args.append(f"&mut {n}_c"); r_args.append(f"&mut {n}_r")
            decl.append(f"{n}: *mut {p['elem']}")
        elif role == "input_string":
            c_args.append(f"{n}_buf.as_ptr()"); r_args.append(f"{n}_buf.as_ptr()")
            decl.append(f"{n}: *const {p['elem']}")
    return c_args, r_args, decl


def gen_target(entry: str, items: list[dict], abi: list[dict], ret: str, crate: str) -> str:
    """Generate the fuzz_target source: decode from items, call/signature in ABI order."""
    decode, post = _decode_and_post(items)
    c_args, r_args, decl = _call_and_decl(abi)
    extern_args = ", ".join(decl)
    extern_ret = "" if ret == "void" else f"-> {ret}"

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
        f"use {crate} as translated;",
        "extern \"C\" {",
        f"    fn c_{entry}({extern_args}) {extern_ret};",
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


def main() -> int:
    ap = argparse.ArgumentParser(description="Generate a differential fuzz harness for an STU")
    ap.add_argument("--pair", required=True, help="benchmark/pairs/<name>")
    ap.add_argument("--entry", required=True)
    ap.add_argument("--out", default=None, help="output project dir (default fuzz_gen/<name>)")
    ap.add_argument("--infer-schema", action="store_true",
                    help="fall back to signature inference when no schema exists (default: require schema)")
    args = ap.parse_args()

    pair = Path(args.pair)
    name = pair.name
    crate = name.replace("-", "_")
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"))
    c_src = next((pair / "source").glob("*.c"))

    params, ret, all_fns, items, abi = resolve(name, cc, args.entry, infer=args.infer_schema)

    out = Path(args.out) if args.out else (ROOT / "fuzz_gen" / name)
    out.mkdir(parents=True, exist_ok=True)
    (out / "src").mkdir(exist_ok=True)
    (out / "c").mkdir(exist_ok=True)
    (out / "fuzz" / "fuzz_targets").mkdir(parents=True, exist_ok=True)

    (out / "c" / c_src.name).write_text(c_src.read_text(), encoding="utf-8")
    (out / "src" / "lib.rs").write_text(rs.read_text(), encoding="utf-8")

    (out / "Cargo.toml").write_text(
        f'[package]\nname = "{crate}"\nversion = "0.1.0"\nedition = "2021"\n\n'
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
name = "{crate}-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = {{ version = "0.15.4", package = "libafl_libfuzzer" }}

[dependencies.{crate}]
path = ".."

[workspace]
members = ["."]

[profile.release]
debug = 1

[[bin]]
name = "{crate}_ft"
path = "fuzz_targets/{crate}_ft.rs"
test = false
doc = false
''', encoding="utf-8")

    (out / "fuzz" / "fuzz_targets" / f"{crate}_ft.rs").write_text(
        gen_target(args.entry, items, abi, ret, crate), encoding="utf-8")

    print(f"generated harness at {out}")
    print(f"  entry: {args.entry} -> {ret}")
    print(f"  abi roles: {[(p['name'], p['role']) for p in abi]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
