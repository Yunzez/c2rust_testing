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
unsafe extern "C" fn opener_for(mut c: uint8_t) -> uint8_t {
    if c as ::core::ffi::c_int == ')' as i32 as uint8_t as ::core::ffi::c_int {
        return '(' as i32 as uint8_t;
    }
    if c as ::core::ffi::c_int == ']' as i32 as uint8_t as ::core::ffi::c_int {
        return '[' as i32 as uint8_t;
    }
    if c as ::core::ffi::c_int == '}' as i32 as uint8_t as ::core::ffi::c_int {
        return '{' as i32 as uint8_t;
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn brackets_balanced(
    mut buf: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut stack: [uint8_t; 256] = [0; 256];
    let mut sp: size_t = 0 as size_t;
    let mut i: size_t = 0;
    if buf.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as size_t;
    while i < len {
        let mut c: uint8_t = *buf.offset(i as isize);
        if c as ::core::ffi::c_int == '(' as i32 as uint8_t as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '[' as i32 as uint8_t as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '{' as i32 as uint8_t as ::core::ffi::c_int
        {
            if sp >= ::core::mem::size_of::<[uint8_t; 256]>() as usize {
                return 0 as ::core::ffi::c_int;
            }
            let fresh0 = sp;
            sp = sp.wrapping_add(1);
            stack[fresh0 as usize] = c;
        } else {
            let mut want: uint8_t = opener_for(c);
            if want as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if sp == 0 as size_t
                    || stack[sp.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                        != want as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                sp = sp.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return if sp == 0 as size_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn fnv1a_checksum(mut buf: *const uint8_t, mut len: size_t) -> uint32_t {
    let mut h: uint32_t = 2166136261 as uint32_t;
    let mut i: size_t = 0;
    if buf.is_null() {
        return h;
    }
    i = 0 as size_t;
    while i < len {
        h ^= *buf.offset(i as isize) as uint32_t;
        h = (h as ::core::ffi::c_uint).wrapping_mul(16777619 as ::core::ffi::c_uint) as uint32_t
            as uint32_t;
        i = i.wrapping_add(1);
    }
    return h;
}
