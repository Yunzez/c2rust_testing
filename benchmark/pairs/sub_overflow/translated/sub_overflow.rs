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
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
unsafe extern "C" fn sub_wrap_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return a.wrapping_sub(b);
}
#[no_mangle]
pub unsafe extern "C" fn sub_signed_i32(mut a: int32_t, mut b: int32_t) -> int32_t {
    return a - b;
}
unsafe extern "C" fn sub_signed_sat(mut a: int32_t, mut b: int32_t) -> int32_t {
    if b < 0 as int32_t && a > INT32_MAX as int32_t + b {
        return INT32_MAX as int32_t;
    }
    if b > 0 as int32_t && a < INT32_MIN as int32_t + b {
        return INT32_MIN as int32_t;
    }
    return a - b;
}
#[no_mangle]
pub unsafe extern "C" fn sub_overflow_safe(mut a: int32_t, mut b: int32_t) -> int32_t {
    let mut w: uint32_t = sub_wrap_u32(a as uint32_t, b as uint32_t);
    return sub_signed_sat(a, b) ^ w as int32_t;
}
