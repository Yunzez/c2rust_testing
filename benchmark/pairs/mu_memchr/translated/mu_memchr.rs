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
pub const SS: usize = ::core::mem::size_of::<size_t>();
pub const ALIGN: usize = (::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize);
pub const ONES: size_t = (-(1 as ::core::ffi::c_int) as size_t).wrapping_div(UCHAR_MAX as size_t);
pub const HIGHS: size_t =
    ONES.wrapping_mul((UCHAR_MAX / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t);
#[no_mangle]
pub unsafe extern "C" fn mu_memchr(
    mut src: *const ::core::ffi::c_void,
    mut c: ::core::ffi::c_int,
    mut n: size_t,
) -> *mut ::core::ffi::c_void {
    let mut s: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    c = c as ::core::ffi::c_uchar as ::core::ffi::c_int;
    while s as uintptr_t & ALIGN as uintptr_t != 0 && n != 0 && *s as ::core::ffi::c_int != c {
        s = s.offset(1);
        n = n.wrapping_sub(1);
    }
    if n != 0 && *s as ::core::ffi::c_int != c {
        let mut w: *const word = ::core::ptr::null::<word>();
        let mut k: size_t = ONES.wrapping_mul(c as size_t);
        w = s as *const ::core::ffi::c_void as *const word;
        while n >= SS && (*w ^ k as word).wrapping_sub(ONES) & !(*w ^ k as word) & HIGHS == 0 {
            w = w.offset(1);
            n = (n as ::core::ffi::c_ulong).wrapping_sub(SS as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
        s = w as *const ::core::ffi::c_void as *const ::core::ffi::c_uchar;
    }
    while n != 0 && *s as ::core::ffi::c_int != c {
        s = s.offset(1);
        n = n.wrapping_sub(1);
    }
    return if n != 0 {
        s as *mut ::core::ffi::c_void
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
