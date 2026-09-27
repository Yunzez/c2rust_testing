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
pub unsafe extern "C" fn negate_i32(mut x: int32_t) -> int32_t {
    return -x;
}
#[no_mangle]
pub unsafe extern "C" fn abs_i32(mut x: int32_t) -> int32_t {
    if x < 0 as int32_t {
        return -x;
    }
    return x;
}
unsafe extern "C" fn negate_u_safe(mut x: int32_t) -> int32_t {
    let mut u: uint32_t = x as uint32_t;
    let mut neg: uint32_t = (!u).wrapping_add(1 as uint32_t);
    return neg as int32_t;
}
unsafe extern "C" fn abs_magnitude_u32(mut x: int32_t) -> uint32_t {
    if x < 0 as int32_t {
        return (0 as ::core::ffi::c_int as uint32_t).wrapping_sub(x as uint32_t);
    }
    return x as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn negate_abs_safe(mut x: int32_t) -> uint32_t {
    return (negate_u_safe(x) as uint32_t).wrapping_add(abs_magnitude_u32(x));
}
