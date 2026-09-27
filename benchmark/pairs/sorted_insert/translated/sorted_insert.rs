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
unsafe extern "C" fn lower_bound(
    mut a: *const int32_t,
    mut len: size_t,
    mut key: int32_t,
) -> size_t {
    let mut lo: size_t = 0 as size_t;
    let mut hi: size_t = len;
    while lo < hi {
        let mut mid: size_t = lo.wrapping_add(hi.wrapping_sub(lo).wrapping_div(2 as size_t));
        if *a.offset(mid as isize) < key {
            lo = mid.wrapping_add(1 as size_t);
        } else {
            hi = mid;
        }
    }
    return lo;
}
#[no_mangle]
pub unsafe extern "C" fn sorted_insert(
    mut a: *mut int32_t,
    mut len: size_t,
    mut cap: size_t,
    mut key: int32_t,
) -> size_t {
    let mut pos: size_t = lower_bound(a, len, key);
    let mut i: size_t = len;
    while i > pos {
        *a.offset(i as isize) = *a.offset(i.wrapping_sub(1 as size_t) as isize);
        i = i.wrapping_sub(1);
    }
    *a.offset(pos as isize) = key;
    return len.wrapping_add(1 as size_t);
}
