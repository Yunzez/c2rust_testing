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
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn to_upper_inplace(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut changed: size_t = 0 as size_t;
    if buf.is_null() {
        return 0 as size_t;
    }
    i = 0 as size_t;
    while i < len {
        let mut b: uint8_t = *buf.offset(i as isize) as uint8_t;
        if b as ::core::ffi::c_int >= 'a' as i32 as uint8_t as ::core::ffi::c_int
            && b as ::core::ffi::c_int <= 'z' as i32 as uint8_t as ::core::ffi::c_int
        {
            *buf.offset(i as isize) =
                (b as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_char;
            changed = changed.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn csv_field_count(
    mut s: *const ::core::ffi::c_char,
    mut delim: ::core::ffi::c_char,
) -> uint32_t {
    let mut fields: uint32_t = 1 as uint32_t;
    let mut i: size_t = 0 as size_t;
    if s.is_null() {
        return 0 as uint32_t;
    }
    while *s.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
        if *s.offset(i as isize) as ::core::ffi::c_int == delim as ::core::ffi::c_int {
            fields = fields.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return fields;
}
