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
pub struct BStack {
    pub data: [int32_t; 16],
    pub top: size_t,
}
pub const BS_CAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn bs_peek(mut s: *const BStack) -> int32_t {
    return (*s).data[(*s).top.wrapping_sub(1 as size_t) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn bs_pop_trusting(mut s: *mut BStack, mut out: *mut int32_t) -> int32_t {
    (*s).top = (*s).top.wrapping_sub(1 as size_t);
    *out = (*s).data[(*s).top as usize];
    return *out;
}
#[no_mangle]
pub unsafe extern "C" fn bs_sum_masked(mut s: *const BStack) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut live: size_t = (*s)
        .top
        .wrapping_rem((BS_CAP + 1 as ::core::ffi::c_int) as size_t);
    let mut i: size_t = 0 as size_t;
    while i < live {
        acc += (*s).data[i.wrapping_rem(BS_CAP as size_t) as usize] as int64_t;
        i = i.wrapping_add(1);
    }
    return acc;
}
