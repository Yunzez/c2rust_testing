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
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn reduce_sum_i32(mut a: *const int32_t, mut n: size_t) -> int32_t {
    let mut acc: int32_t = 0 as int32_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        acc += *a.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn reduce_prod_i32(mut a: *const int32_t, mut n: size_t) -> int32_t {
    let mut acc: int32_t = 1 as int32_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        acc *= *a.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return acc;
}
unsafe extern "C" fn reduce_sum_wrap(mut a: *const int32_t, mut n: size_t) -> int32_t {
    let mut acc: uint32_t = 0 as uint32_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        acc = acc.wrapping_add(*a.offset(i as isize) as uint32_t);
        i = i.wrapping_add(1);
    }
    return acc as int32_t;
}
unsafe extern "C" fn reduce_prod_wrap(mut a: *const int32_t, mut n: size_t) -> int32_t {
    let mut acc: uint32_t = 1 as uint32_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < n {
        acc = acc.wrapping_mul(*a.offset(i as isize) as uint32_t);
        i = i.wrapping_add(1);
    }
    return acc as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn reduce_overflow_safe(mut a: *const int32_t, mut n: size_t) -> int32_t {
    return reduce_sum_wrap(a, n) ^ reduce_prod_wrap(a, n);
}
