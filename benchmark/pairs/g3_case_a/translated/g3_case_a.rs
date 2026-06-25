#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn scale(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return x * 100 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scale_pct(mut pct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if pct < 0 as ::core::ffi::c_int {
        pct = 0 as ::core::ffi::c_int;
    }
    if pct > 100 as ::core::ffi::c_int {
        pct = 100 as ::core::ffi::c_int;
    }
    return scale(pct);
}
