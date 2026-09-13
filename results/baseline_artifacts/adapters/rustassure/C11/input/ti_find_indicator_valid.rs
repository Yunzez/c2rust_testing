#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]

include!["/input/ti_find_indicator_valid_raw.inc"];

#[no_mangle]
pub fn ti_find_indicator_valid(name: &mut [i8; 16]) -> i32 {
    name[15] = 0;
    unsafe { (!ti_find_indicator(name.as_ptr()).is_null()) as i32 }
}
