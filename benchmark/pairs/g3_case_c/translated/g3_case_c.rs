#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ring {
    pub buf: [int32_t; 16],
    pub head: size_t,
}
pub const CAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ring_at(mut r: *const Ring) -> int32_t {
    return (*r).buf[(*r).head as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ring_get(mut r: *mut Ring) -> int32_t {
    (*r).head = (*r).head.wrapping_rem(CAP as size_t);
    return ring_at(r);
}
