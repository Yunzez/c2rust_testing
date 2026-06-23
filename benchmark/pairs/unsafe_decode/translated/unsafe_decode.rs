#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn char_at(mut s: *const ::core::ffi::c_char, mut pos: size_t) -> uint8_t {
    return *s.offset(pos as isize) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sum_declared(mut data: *const uint8_t, mut len: size_t) -> uint32_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut n: size_t = 0;
    let mut i: size_t = 0;
    n = *data.offset(0 as ::core::ffi::c_int as isize) as size_t;
    i = 0 as size_t;
    while i < n {
        sum = sum.wrapping_add(*data.offset((1 as size_t).wrapping_add(i) as isize) as uint32_t);
        i = i.wrapping_add(1);
    }
    return sum;
}
