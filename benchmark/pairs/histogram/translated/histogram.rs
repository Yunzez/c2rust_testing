#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Histogram {
    pub bins: [uint32_t; 16],
    pub nbins: size_t,
}
pub const HG_BINS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn hg_total_trusting(mut h: *const Histogram) -> uint64_t {
    let mut acc: uint64_t = 0 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < (*h).nbins {
        acc = acc.wrapping_add((*h).bins[i as usize] as uint64_t);
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn hg_last_bin(mut h: *const Histogram) -> uint32_t {
    return (*h).bins[(*h).nbins.wrapping_sub(1 as size_t) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn hg_total_masked(mut h: *const Histogram) -> uint64_t {
    let mut acc: uint64_t = 0 as uint64_t;
    let mut active: size_t = (*h)
        .nbins
        .wrapping_rem((HG_BINS + 1 as ::core::ffi::c_int) as size_t);
    let mut i: size_t = 0 as size_t;
    while i < active {
        acc = acc.wrapping_add((*h).bins[i as usize] as uint64_t);
        i = i.wrapping_add(1);
    }
    return acc;
}
