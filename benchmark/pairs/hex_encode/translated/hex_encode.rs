#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn nibble_to_hex(mut n: uint8_t) -> uint8_t {
    static mut digits: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    return digits[(n as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize] as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn hex_encode(
    mut src: *const uint8_t,
    mut len: size_t,
    mut dst: *mut uint8_t,
    mut dst_cap: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut out: size_t = 0 as size_t;
    if src.is_null() || dst.is_null() {
        return 0 as size_t;
    }
    if len > dst_cap.wrapping_div(2 as size_t) {
        return 0 as size_t;
    }
    i = 0 as size_t;
    while i < len {
        let mut b: uint8_t = *src.offset(i as isize);
        let fresh0 = out;
        out = out.wrapping_add(1);
        *dst.offset(fresh0 as isize) =
            nibble_to_hex((b as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t);
        let fresh1 = out;
        out = out.wrapping_add(1);
        *dst.offset(fresh1 as isize) =
            nibble_to_hex((b as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t);
        i = i.wrapping_add(1);
    }
    return out;
}
