#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = usize;
pub type uintptr_t = usize;
pub type word = size_t;
pub const ALIGN: usize = ::core::mem::size_of::<size_t>();
pub const ONES: size_t = (-(1 as ::core::ffi::c_int) as size_t).wrapping_div(UCHAR_MAX as size_t);
pub const HIGHS: size_t =
    ONES.wrapping_mul((UCHAR_MAX / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t);
#[no_mangle]
pub unsafe extern "C" fn mu_strlen(mut s: *const ::core::ffi::c_char) -> size_t {
    let mut a: *const ::core::ffi::c_char = s;
    let mut w: *const word = ::core::ptr::null::<word>();
    while (s as uintptr_t).wrapping_rem(ALIGN as uintptr_t) != 0 {
        if *s == 0 {
            return s.offset_from(a) as ::core::ffi::c_long as size_t;
        }
        s = s.offset(1);
    }
    w = s as *const ::core::ffi::c_void as *const word;
    while (*w).wrapping_sub(ONES) & !*w & HIGHS == 0 {
        w = w.offset(1);
    }
    s = w as *const ::core::ffi::c_void as *const ::core::ffi::c_char;
    while *s != 0 {
        s = s.offset(1);
    }
    return s.offset_from(a) as ::core::ffi::c_long as size_t;
}
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
