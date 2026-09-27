#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn prefix_sum_excl(
    mut src: *const uint32_t,
    mut n: size_t,
    mut dst: *mut uint64_t,
) {
    let mut acc: uint64_t = 0 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        *dst.offset(i as isize) = acc;
        acc = acc.wrapping_add(*src.offset(i as isize) as uint64_t);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rle_encode(
    mut src: *const uint8_t,
    mut n: size_t,
    mut out: *mut uint8_t,
    mut cap: size_t,
) -> size_t {
    let mut w: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        let mut v: uint8_t = *src.offset(i as isize);
        let mut run: size_t = 1 as size_t;
        while i.wrapping_add(run) < n
            && *src.offset(i.wrapping_add(run) as isize) as ::core::ffi::c_int
                == v as ::core::ffi::c_int
            && run < 255 as size_t
        {
            run = run.wrapping_add(1);
        }
        if w.wrapping_add(2 as size_t) > cap {
            return -(1 as ::core::ffi::c_int) as size_t;
        }
        let fresh0 = w;
        w = w.wrapping_add(1);
        *out.offset(fresh0 as isize) = run as uint8_t;
        let fresh1 = w;
        w = w.wrapping_add(1);
        *out.offset(fresh1 as isize) = v;
        i = i.wrapping_add(run);
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn rolling_checksum(mut src: *const uint8_t, mut n: size_t) -> uint32_t {
    let mut a: uint32_t = 1 as uint32_t;
    let mut b: uint32_t = 0 as uint32_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        a = a
            .wrapping_add(*src.offset(i as isize) as uint32_t)
            .wrapping_rem(65521 as uint32_t);
        b = b.wrapping_add(a).wrapping_rem(65521 as uint32_t);
        i = i.wrapping_add(1);
    }
    return b << 16 as ::core::ffi::c_int | a;
}
