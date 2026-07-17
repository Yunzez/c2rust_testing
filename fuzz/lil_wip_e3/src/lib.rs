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
#[no_mangle] pub static _DefaultRuneLocale: [u8; 65536] = [0u8; 65536];
