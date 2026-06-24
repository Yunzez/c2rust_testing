#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn mu_llabs(mut a: ::core::ffi::c_longlong) -> ::core::ffi::c_longlong {
    return if a > 0 as ::core::ffi::c_longlong {
        a
    } else {
        -a
    };
}
