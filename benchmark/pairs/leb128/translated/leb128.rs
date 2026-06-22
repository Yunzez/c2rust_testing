#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn leb128_encode(
    mut value: uint64_t,
    mut out: *mut uint8_t,
    mut cap: size_t,
) -> size_t {
    let mut n: size_t = 0 as size_t;
    loop {
        let mut byte: uint8_t = (value & 0x7f as uint64_t) as uint8_t;
        value >>= 7 as ::core::ffi::c_int;
        if value != 0 as uint64_t {
            byte = (byte as ::core::ffi::c_uint | 0x80 as ::core::ffi::c_uint) as uint8_t;
        }
        if n >= cap {
            return 0 as size_t;
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        *out.offset(fresh0 as isize) = byte;
        if !(value != 0 as uint64_t) {
            break;
        }
    }
    return n;
}
unsafe extern "C" fn leb128_decode(
    mut in_0: *const uint8_t,
    mut len: size_t,
    mut value_out: *mut uint64_t,
) -> size_t {
    let mut result: uint64_t = 0 as uint64_t;
    let mut shift: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: size_t = 0 as size_t;
    while i < len {
        let mut byte: uint8_t = *in_0.offset(i as isize);
        let mut low7: uint64_t =
            (byte as ::core::ffi::c_uint & 0x7f as ::core::ffi::c_uint) as uint64_t;
        if shift >= 64 as ::core::ffi::c_uint {
            return 0 as size_t;
        }
        if shift == 63 as ::core::ffi::c_uint && low7 > 1 as uint64_t {
            return 0 as size_t;
        }
        result |= low7 << shift;
        i = i.wrapping_add(1);
        if byte as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
            *value_out = result;
            return i;
        }
        shift = shift.wrapping_add(7 as ::core::ffi::c_uint);
    }
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn leb128_roundtrip(
    mut in_0: *const uint8_t,
    mut len: size_t,
    mut out: *mut uint8_t,
    mut out_cap: size_t,
) -> ::core::ffi::c_int {
    let mut value: uint64_t = 0;
    let mut consumed: size_t = 0;
    let mut written: size_t = 0;
    if in_0.is_null() || out.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    consumed = leb128_decode(in_0, len, &raw mut value);
    if consumed == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    written = leb128_encode(value, out, out_cap);
    if written == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    return written as ::core::ffi::c_int;
}
