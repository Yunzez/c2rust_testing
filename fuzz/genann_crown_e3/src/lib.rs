#![feature(extern_types)]
#![feature(register_tool)]
#![feature(core_intrinsics)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, internal_features, unused_imports)]
pub mod src {
    pub mod example1;
    pub mod example2;
    pub mod example3;
    pub mod example4;
    pub mod genann;
    pub mod test;
}
#[no_mangle]
pub extern "C" fn __assert_rtn(_f: *const std::os::raw::c_char, _fl: *const std::os::raw::c_char, _l: std::os::raw::c_int, _e: *const std::os::raw::c_char) -> ! { std::process::abort() }
