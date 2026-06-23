#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GapBuf {
    pub text: [::core::ffi::c_char; 32],
    pub gap_start: size_t,
    pub gap_end: size_t,
}
pub const GB_CAP: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn gb_char_before_gap(mut g: *const GapBuf) -> ::core::ffi::c_char {
    return (*g).text[(*g).gap_start.wrapping_sub(1 as size_t) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn gb_char_after_gap(mut g: *const GapBuf) -> ::core::ffi::c_char {
    return (*g).text[(*g).gap_end as usize];
}
#[no_mangle]
pub unsafe extern "C" fn gb_live_checksum(mut g: *const GapBuf) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut gs: size_t = (*g)
        .gap_start
        .wrapping_rem((GB_CAP + 1 as ::core::ffi::c_int) as size_t);
    let mut ge: size_t = (*g)
        .gap_end
        .wrapping_rem((GB_CAP + 1 as ::core::ffi::c_int) as size_t);
    if ge < gs {
        ge = gs;
    }
    let mut i: size_t = 0 as size_t;
    while i < GB_CAP as size_t {
        if !(i >= gs && i < ge) {
            acc += (*g).text[i as usize] as ::core::ffi::c_uchar as int64_t;
        }
        i = i.wrapping_add(1);
    }
    return acc;
}
