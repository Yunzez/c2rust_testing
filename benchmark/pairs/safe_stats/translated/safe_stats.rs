#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn sum_i32(mut a: *const int32_t, mut n: size_t) -> int32_t {
    let mut acc: int32_t = 0 as int32_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        acc += *a.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn idiv(mut a: int32_t, mut b: int32_t) -> int32_t {
    return a / b;
}
#[no_mangle]
pub unsafe extern "C" fn xorshift_fold(mut data: *const uint8_t, mut n: size_t) -> uint64_t {
    let mut h: uint64_t = 1469598103934665603 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        h ^= *data.offset(i as isize) as uint64_t;
        h = (h as ::core::ffi::c_ulonglong).wrapping_mul(1099511628211 as ::core::ffi::c_ulonglong)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn count_above(
    mut a: *const int32_t,
    mut n: size_t,
    mut threshold: int32_t,
) -> size_t {
    let mut c: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        if *a.offset(i as isize) > threshold {
            c = c.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return c;
}
