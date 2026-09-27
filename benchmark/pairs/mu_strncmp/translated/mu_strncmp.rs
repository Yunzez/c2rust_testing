#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn mu_strncmp(
    mut _l: *const ::core::ffi::c_char,
    mut _r: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut l: *const ::core::ffi::c_uchar =
        _l as *mut ::core::ffi::c_void as *const ::core::ffi::c_uchar;
    let mut r: *const ::core::ffi::c_uchar =
        _r as *mut ::core::ffi::c_void as *const ::core::ffi::c_uchar;
    let fresh0 = n;
    n = n.wrapping_sub(1);
    if fresh0 == 0 {
        return 0 as ::core::ffi::c_int;
    }
    while *l as ::core::ffi::c_int != 0
        && *r as ::core::ffi::c_int != 0
        && n != 0
        && *l as ::core::ffi::c_int == *r as ::core::ffi::c_int
    {
        l = l.offset(1);
        r = r.offset(1);
        n = n.wrapping_sub(1);
    }
    return *l as ::core::ffi::c_int - *r as ::core::ffi::c_int;
}
