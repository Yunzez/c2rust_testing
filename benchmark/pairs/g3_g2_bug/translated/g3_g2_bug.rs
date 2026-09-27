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
    return x * 10 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_ratio(mut pct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if pct < 0 as ::core::ffi::c_int {
        pct = 0 as ::core::ffi::c_int;
    }
    if pct > 100 as ::core::ffi::c_int {
        pct = 100 as ::core::ffi::c_int;
    }
    return scale(pct);
}
#[no_mangle]
pub unsafe extern "C" fn report(
    mut pct: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut a: ::core::ffi::c_int = safe_ratio(pct);
    let mut b: ::core::ffi::c_int = y * y * y;
    return a + b;
}
