#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BQueue {
    pub data: [int32_t; 16],
    pub head: size_t,
    pub len: size_t,
}
pub const BQ_CAP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn bq_front(mut q: *const BQueue) -> int32_t {
    return (*q).data[(*q).head as usize];
}
#[no_mangle]
pub unsafe extern "C" fn bq_back_trusting(mut q: *const BQueue) -> int32_t {
    return (*q).data[(*q).head.wrapping_add((*q).len).wrapping_sub(1 as size_t) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn bq_sum_masked(mut q: *const BQueue) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut live: size_t = (*q)
        .len
        .wrapping_rem((BQ_CAP + 1 as ::core::ffi::c_int) as size_t);
    let mut h: size_t = (*q).head.wrapping_rem(BQ_CAP as size_t);
    let mut i: size_t = 0 as size_t;
    while i < live {
        acc += (*q).data[h.wrapping_add(i).wrapping_rem(BQ_CAP as size_t) as usize] as int64_t;
        i = i.wrapping_add(1);
    }
    return acc;
}
