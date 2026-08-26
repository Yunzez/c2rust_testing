#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, internal_features, unused_imports)]
pub mod lil;
#[no_mangle] pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }
#[no_mangle] pub static mut __stderrp: *mut std::os::raw::c_void = std::ptr::null_mut();
#[no_mangle] pub extern "C" fn __maskrune(_c: std::os::raw::c_int, _f: std::os::raw::c_ulong) -> std::os::raw::c_int { 0 }
// E3 harness shim for the macOS-transpiled ctype path: `_DefaultRuneLocale` with a POPULATED
// `__runetype` (the 2026-08-25 lil pilot: a zeroed table makes isspace/ispunct/isdigit always false
// -> every script returns [] = 313 false divergences; fill ported from
// results/pilots/attr/lil/raw/rune_fill.rs). Layout = the leading fields of lil::_RuneLocale
// (magic[8], encoding[32], 2 fn ptrs, invalid_rune, __runetype[256], __maplower[256],
// __mapupper[256]); the zero tail keeps the object as large as the original 64 KiB shim.
#[repr(C)]
pub struct E3RuneLocale {
    pub __magic: [u8; 8],
    pub __encoding: [u8; 32],
    pub __sgetrune: usize,
    pub __sputrune: usize,
    pub __invalid_rune: i32,
    pub __runetype: [u32; 256],
    pub __maplower: [i32; 256],
    pub __mapupper: [i32; 256],
    pub __tail: [u8; 65536 - 60 - 3 * 1024],
}
impl E3RuneLocale {
    pub const fn filled() -> Self {
        let mut t = [0u32; 256];
        let mut lo = [0i32; 256];
        let mut up = [0i32; 256];
        let mut i = 0usize;
        while i < 256 {
            let c = i as u8;
            let mut f = 0u32;
            if c.is_ascii_alphabetic() { f |= 0x100; }
            if c.is_ascii_control() { f |= 0x200; }
            if c.is_ascii_digit() { f |= 0x400; }
            if c.is_ascii_graphic() { f |= 0x800; }
            if c.is_ascii_lowercase() { f |= 0x1000; }
            if c.is_ascii_punctuation() { f |= 0x2000; }
            if c.is_ascii_whitespace() || c == 0x0b { f |= 0x4000; }
            if c.is_ascii_uppercase() { f |= 0x8000; }
            if c.is_ascii_hexdigit() { f |= 0x10000; }
            if c == b' ' || c == b'\t' { f |= 0x20000; }
            if c.is_ascii_graphic() || c == b' ' { f |= 0x40000; }
            t[i] = f;
            lo[i] = c.to_ascii_lowercase() as i32;
            up[i] = c.to_ascii_uppercase() as i32;
            i += 1;
        }
        E3RuneLocale { __magic: *b"RuneMagA", __encoding: [0; 32], __sgetrune: 0, __sputrune: 0,
                       __invalid_rune: 0xFFFD, __runetype: t, __maplower: lo, __mapupper: up,
                       __tail: [0; 65536 - 60 - 3 * 1024] }
    }
}
#[no_mangle] pub static _DefaultRuneLocale: E3RuneLocale = E3RuneLocale::filled();
