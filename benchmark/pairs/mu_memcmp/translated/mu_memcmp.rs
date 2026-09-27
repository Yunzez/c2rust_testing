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
pub unsafe extern "C" fn mu_memcmp(
    mut vl: *const ::core::ffi::c_void,
    mut vr: *const ::core::ffi::c_void,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut l: *const ::core::ffi::c_uchar = vl as *const ::core::ffi::c_uchar;
    let mut r: *const ::core::ffi::c_uchar = vr as *const ::core::ffi::c_uchar;
    while n != 0 && *l as ::core::ffi::c_int == *r as ::core::ffi::c_int {
        n = n.wrapping_sub(1);
        l = l.offset(1);
        r = r.offset(1);
    }
    return if n != 0 {
        *l as ::core::ffi::c_int - *r as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
