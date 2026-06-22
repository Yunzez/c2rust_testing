#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
unsafe extern "C" fn popcount32(mut x: uint32_t) -> uint32_t {
    let mut count: uint32_t = 0 as uint32_t;
    while x != 0 {
        x = (x as ::core::ffi::c_uint & x.wrapping_sub(1 as uint32_t) as ::core::ffi::c_uint)
            as uint32_t;
        count = count.wrapping_add(1);
    }
    return count;
}
unsafe extern "C" fn reverse32(mut x: uint32_t) -> uint32_t {
    x = (x & 0xaaaaaaaa as uint32_t) >> 1 as ::core::ffi::c_int
        | (x & 0x55555555 as uint32_t) << 1 as ::core::ffi::c_int;
    x = (x & 0xcccccccc as uint32_t) >> 2 as ::core::ffi::c_int
        | (x & 0x33333333 as uint32_t) << 2 as ::core::ffi::c_int;
    x = (x & 0xf0f0f0f0 as uint32_t) >> 4 as ::core::ffi::c_int
        | (x & 0xf0f0f0f as uint32_t) << 4 as ::core::ffi::c_int;
    x = (x & 0xff00ff00 as uint32_t) >> 8 as ::core::ffi::c_int
        | (x & 0xff00ff as uint32_t) << 8 as ::core::ffi::c_int;
    x = x >> 16 as ::core::ffi::c_int | x << 16 as ::core::ffi::c_int;
    return x;
}
unsafe extern "C" fn pack4(
    mut a: uint32_t,
    mut b: uint32_t,
    mut c: uint32_t,
    mut d: uint32_t,
) -> uint32_t {
    return (a & 0xff as uint32_t) << 24 as ::core::ffi::c_int
        | (b & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | (c & 0xff as uint32_t) << 8 as ::core::ffi::c_int
        | d & 0xff as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn bitutils_eval(
    mut op: ::core::ffi::c_int,
    mut x: uint32_t,
    mut y: uint32_t,
) -> uint32_t {
    match op {
        0 => return popcount32(x),
        1 => return reverse32(x),
        2 => {
            let mut packed: uint32_t = pack4(
                x >> 24 as ::core::ffi::c_int,
                x >> 16 as ::core::ffi::c_int,
                x >> 8 as ::core::ffi::c_int,
                x,
            );
            return packed ^ reverse32(y);
        }
        3 => return popcount32(x ^ y),
        _ => return 0 as uint32_t,
    };
}
