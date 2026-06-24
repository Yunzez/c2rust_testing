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
pub unsafe extern "C" fn mu_strspn(
    mut s: *const ::core::ffi::c_char,
    mut c: *const ::core::ffi::c_char,
) -> size_t {
    let mut a: *const ::core::ffi::c_char = s;
    let mut byteset: [size_t; 4] = [0 as ::core::ffi::c_int as size_t, 0, 0, 0];
    if *c.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return 0 as size_t;
    }
    if *c.offset(1 as ::core::ffi::c_int as isize) == 0 {
        while *s as ::core::ffi::c_int == *c as ::core::ffi::c_int {
            s = s.offset(1);
        }
        return s.offset_from(a) as ::core::ffi::c_long as size_t;
    }
    while *c as ::core::ffi::c_int != 0 && {
        byteset[(*(c as *mut ::core::ffi::c_uchar) as size_t)
            .wrapping_div((8 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t))
            as usize] |= (1 as ::core::ffi::c_int as size_t)
            << (*(c as *mut ::core::ffi::c_uchar) as size_t).wrapping_rem(
                (8 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
            );
        byteset[(*(c as *mut ::core::ffi::c_uchar) as size_t)
            .wrapping_div((8 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t))
            as usize]
            != 0
    } {
        c = c.offset(1);
    }
    while *s as ::core::ffi::c_int != 0
        && byteset[(*(s as *mut ::core::ffi::c_uchar) as size_t)
            .wrapping_div((8 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t))
            as usize]
            & (1 as ::core::ffi::c_int as size_t)
                << (*(s as *mut ::core::ffi::c_uchar) as size_t).wrapping_rem(
                    (8 as size_t).wrapping_mul(::core::mem::size_of::<size_t>() as size_t),
                )
            != 0
    {
        s = s.offset(1);
    }
    return s.offset_from(a) as ::core::ffi::c_long as size_t;
}
