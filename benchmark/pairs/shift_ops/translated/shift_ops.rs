#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn shl_u32(mut v: uint32_t, mut count: uint32_t) -> uint32_t {
    return v << count;
}
#[no_mangle]
pub unsafe extern "C" fn shl_i32(mut v: int32_t, mut count: uint32_t) -> int32_t {
    return v << count;
}
unsafe extern "C" fn shl_u32_masked(mut v: uint32_t, mut count: uint32_t) -> uint32_t {
    return v << (count & 31 as uint32_t);
}
unsafe extern "C" fn shl_i32_safe(mut v: int32_t, mut count: uint32_t) -> int32_t {
    let mut u: uint32_t = v as uint32_t;
    return (u << (count & 31 as uint32_t)) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn shift_ops_safe(mut v: uint32_t, mut count: uint32_t) -> uint32_t {
    return shl_u32_masked(v, count) ^ shl_i32_safe(v as int32_t, count) as uint32_t;
}
