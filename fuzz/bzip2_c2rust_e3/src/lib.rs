#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, internal_features, unused_imports)]
// #![feature(const_transmute)]







pub mod bzip2;
pub mod blocksort;
pub mod bzlib;
pub mod compress;
pub mod crctable;
pub mod decompress;
pub mod huffman;
pub mod randtable;


#[no_mangle]
pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }

// macOS libc shims (c2rust transpiled on macOS references these; used only by dead CLI code here)
#[no_mangle] pub static mut __stderrp: *mut std::os::raw::c_void = std::ptr::null_mut();
#[no_mangle] pub extern "C" fn __maskrune(_c: std::os::raw::c_int, _f: std::os::raw::c_ulong) -> std::os::raw::c_int { 0 }
#[no_mangle] pub static _DefaultRuneLocale: [u8; 65536] = [0u8; 65536];
