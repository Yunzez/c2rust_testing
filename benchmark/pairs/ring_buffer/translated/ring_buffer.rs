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
pub struct RingBuf {
    pub buf: [int32_t; 16],
    pub head: size_t,
    pub tail: size_t,
    pub count: size_t,
}
pub const RB_CAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn rb_first(mut rb: *const RingBuf) -> int32_t {
    return (*rb).buf[(*rb).head as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rb_sum_masked(mut rb: *const RingBuf) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut live: size_t = (*rb)
        .count
        .wrapping_rem((RB_CAP + 1 as ::core::ffi::c_int) as size_t);
    let mut i: size_t = 0 as size_t;
    while i < live {
        acc += (*rb).buf[(*rb).head.wrapping_add(i).wrapping_rem(RB_CAP as size_t) as usize]
            as int64_t;
        i = i.wrapping_add(1);
    }
    return acc;
}
