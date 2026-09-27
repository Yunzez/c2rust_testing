#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn merge_runs(
    mut a: *mut ::core::ffi::c_int,
    mut tmp: *mut ::core::ffi::c_int,
    mut lo: size_t,
    mut mid: size_t,
    mut hi: size_t,
) {
    let mut i: size_t = lo;
    let mut j: size_t = mid;
    let mut k: size_t = 0 as size_t;
    while i < mid && j < hi {
        if *a.offset(i as isize) <= *a.offset(j as isize) {
            let fresh0 = i;
            i = i.wrapping_add(1);
            let fresh1 = k;
            k = k.wrapping_add(1);
            *tmp.offset(fresh1 as isize) = *a.offset(fresh0 as isize);
        } else {
            let fresh2 = j;
            j = j.wrapping_add(1);
            let fresh3 = k;
            k = k.wrapping_add(1);
            *tmp.offset(fresh3 as isize) = *a.offset(fresh2 as isize);
        }
    }
    while i < mid {
        let fresh4 = i;
        i = i.wrapping_add(1);
        let fresh5 = k;
        k = k.wrapping_add(1);
        *tmp.offset(fresh5 as isize) = *a.offset(fresh4 as isize);
    }
    while j < hi {
        let fresh6 = j;
        j = j.wrapping_add(1);
        let fresh7 = k;
        k = k.wrapping_add(1);
        *tmp.offset(fresh7 as isize) = *a.offset(fresh6 as isize);
    }
    let mut t: size_t = 0 as size_t;
    while t < k {
        *a.offset(lo.wrapping_add(t) as isize) = *tmp.offset(t as isize);
        t = t.wrapping_add(1);
    }
}
unsafe extern "C" fn msort_range(
    mut a: *mut ::core::ffi::c_int,
    mut tmp: *mut ::core::ffi::c_int,
    mut lo: size_t,
    mut hi: size_t,
) {
    if hi.wrapping_sub(lo) < 2 as size_t {
        return;
    }
    let mut mid: size_t = lo.wrapping_add(hi.wrapping_sub(lo).wrapping_div(2 as size_t));
    msort_range(a, tmp, lo, mid);
    msort_range(a, tmp, mid, hi);
    merge_runs(a, tmp, lo, mid, hi);
}
unsafe extern "C" fn lower_bound(
    mut a: *const ::core::ffi::c_int,
    mut n: size_t,
    mut key: ::core::ffi::c_int,
) -> size_t {
    let mut lo: size_t = 0 as size_t;
    let mut hi: size_t = n;
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
pub unsafe extern "C" fn sort_and_find(
    mut a: *mut ::core::ffi::c_int,
    mut n: size_t,
    mut key: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    let mut tmp: *mut ::core::ffi::c_int =
        malloc(n.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t))
            as *mut ::core::ffi::c_int;
    if tmp.is_null() {
        return -(2 as ::core::ffi::c_int);
    }
    msort_range(a, tmp, 0 as size_t, n);
    free(tmp as *mut ::core::ffi::c_void);
    let mut idx: size_t = lower_bound(a, n, key);
    if idx < n && *a.offset(idx as isize) == key {
        return idx as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
