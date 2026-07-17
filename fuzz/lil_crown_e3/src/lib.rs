#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#![feature(core_intrinsics)]



extern crate libc;
pub mod src {
pub mod lil;
pub mod main;
} // mod src

#[no_mangle] pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }
#[no_mangle] pub static mut __stderrp: *mut std::os::raw::c_void = std::ptr::null_mut();
#[no_mangle] pub extern "C" fn readline(_p: *const std::os::raw::c_char) -> *mut std::os::raw::c_char { std::ptr::null_mut() }
#[no_mangle] pub extern "C" fn add_history(_l: *const std::os::raw::c_char) {}
