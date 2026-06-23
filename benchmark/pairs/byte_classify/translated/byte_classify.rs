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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn is_digit(mut b: uint8_t) -> ::core::ffi::c_int {
    return if b as ::core::ffi::c_int >= '0' as i32 as uint8_t as ::core::ffi::c_int
        && b as ::core::ffi::c_int <= '9' as i32 as uint8_t as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn is_space(mut b: uint8_t) -> ::core::ffi::c_int {
    return if b as ::core::ffi::c_int == ' ' as i32 as uint8_t as ::core::ffi::c_int
        || b as ::core::ffi::c_int == '\t' as i32 as uint8_t as ::core::ffi::c_int
        || b as ::core::ffi::c_int == '\n' as i32 as uint8_t as ::core::ffi::c_int
        || b as ::core::ffi::c_int == '\r' as i32 as uint8_t as ::core::ffi::c_int
        || b as ::core::ffi::c_int == '\u{c}' as i32 as uint8_t as ::core::ffi::c_int
        || b as ::core::ffi::c_int == '\u{b}' as i32 as uint8_t as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn count_class(
    mut buf: *const uint8_t,
    mut len: size_t,
    mut class_id: uint8_t,
) -> uint32_t {
    let mut n: uint32_t = 0 as uint32_t;
    let mut i: size_t = 0;
    if buf.is_null() {
        return 0 as uint32_t;
    }
    i = 0 as size_t;
    while i < len {
        let mut b: uint8_t = *buf.offset(i as isize);
        let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if class_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            hit = is_digit(b);
        } else if class_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            hit = is_space(b);
        } else if class_id as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            hit = if b as ::core::ffi::c_int >= 'A' as i32 as uint8_t as ::core::ffi::c_int
                && b as ::core::ffi::c_int <= 'Z' as i32 as uint8_t as ::core::ffi::c_int
                || b as ::core::ffi::c_int >= 'a' as i32 as uint8_t as ::core::ffi::c_int
                    && b as ::core::ffi::c_int <= 'z' as i32 as uint8_t as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if hit != 0 {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return n;
}
