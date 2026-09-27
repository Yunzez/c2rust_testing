#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Config {
    pub gains: [int32_t; 8],
    pub offset: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub lo: [int32_t; 8],
    pub hi: [int32_t; 8],
}
pub const CFG_CHANNELS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn cfg_total_gain(mut c: *const Config) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut i: size_t = 0 as size_t;
    while i < CFG_CHANNELS as size_t {
        acc += (*c).gains[i as usize] as int64_t + (*c).offset as int64_t;
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn range_count_inverted(mut r: *const Range) -> int32_t {
    let mut n: int32_t = 0 as int32_t;
    let mut i: size_t = 0 as size_t;
    while i < CFG_CHANNELS as size_t {
        if (*r).hi[i as usize] < (*r).lo[i as usize] {
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    return n;
}
