#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn encode_run(
    mut src: *const uint8_t,
    mut avail: size_t,
    mut out: *mut uint8_t,
) -> size_t {
    let mut value: uint8_t = *src.offset(0 as ::core::ffi::c_int as isize);
    let mut run: size_t = 1 as size_t;
    while run < avail
        && run < 255 as size_t
        && *src.offset(run as isize) as ::core::ffi::c_int == value as ::core::ffi::c_int
    {
        run = run.wrapping_add(1);
    }
    *out.offset(0 as ::core::ffi::c_int as isize) = run as uint8_t;
    *out.offset(1 as ::core::ffi::c_int as isize) = value;
    return run;
}
#[no_mangle]
pub unsafe extern "C" fn rle_encode(
    mut src: *const uint8_t,
    mut len: size_t,
    mut dst: *mut uint8_t,
    mut dst_cap: size_t,
) -> size_t {
    let mut in_0: size_t = 0 as size_t;
    let mut out: size_t = 0 as size_t;
    if src.is_null() || dst.is_null() {
        return 0 as size_t;
    }
    while in_0 < len {
        let mut pair: [uint8_t; 2] = [0; 2];
        let mut consumed: size_t = encode_run(
            src.offset(in_0 as isize),
            len.wrapping_sub(in_0),
            &raw mut pair as *mut uint8_t,
        );
        if out.wrapping_add(2 as size_t) > dst_cap {
            return 0 as size_t;
        }
        let fresh0 = out;
        out = out.wrapping_add(1);
        *dst.offset(fresh0 as isize) = pair[0 as ::core::ffi::c_int as usize];
        let fresh1 = out;
        out = out.wrapping_add(1);
        *dst.offset(fresh1 as isize) = pair[1 as ::core::ffi::c_int as usize];
        in_0 = in_0.wrapping_add(consumed);
    }
    return out;
}
