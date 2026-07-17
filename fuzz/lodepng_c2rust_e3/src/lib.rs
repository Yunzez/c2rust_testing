#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, internal_features, unused_imports)]
pub mod src { pub mod lodepng; }
#[no_mangle] pub extern "C" fn __assert_rtn(_a: *const std::os::raw::c_char,_b: *const std::os::raw::c_char,_c: std::os::raw::c_int,_d: *const std::os::raw::c_char)->!{std::process::abort()}
