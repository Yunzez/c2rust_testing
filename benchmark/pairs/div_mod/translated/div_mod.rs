#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn mod_signed_i32(mut a: int32_t, mut b: int32_t) -> int32_t {
    return a % b;
}
#[no_mangle]
pub unsafe extern "C" fn div_signed_i32(mut a: int32_t, mut b: int32_t) -> int32_t {
    return a / b;
}
unsafe extern "C" fn mod_signed_guarded(mut a: int32_t, mut b: int32_t) -> int32_t {
    if b == 0 as int32_t {
        return a;
    }
    if a == INT32_MIN as int32_t && b == -(1 as int32_t) {
        return 0 as int32_t;
    }
    return a % b;
}
unsafe extern "C" fn div_signed_guarded(mut a: int32_t, mut b: int32_t) -> int32_t {
    if b == 0 as int32_t {
        return 0 as int32_t;
    }
    if a == INT32_MIN as int32_t && b == -(1 as int32_t) {
        return INT32_MIN as int32_t;
    }
    return a / b;
}
#[no_mangle]
pub unsafe extern "C" fn div_mod_safe(mut a: int32_t, mut b: int32_t) -> int32_t {
    return div_signed_guarded(a, b) + mod_signed_guarded(a, b);
}
