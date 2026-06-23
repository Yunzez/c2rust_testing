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
unsafe extern "C" fn reverse_range(mut a: *mut int32_t, mut lo: size_t, mut hi: size_t) {
    while lo.wrapping_add(1 as size_t) < hi.wrapping_add(1 as size_t) && lo < hi {
        let mut j: size_t = hi.wrapping_sub(1 as size_t);
        let mut t: int32_t = *a.offset(lo as isize);
        *a.offset(lo as isize) = *a.offset(j as isize);
        *a.offset(j as isize) = t;
        lo = lo.wrapping_add(1);
        hi = hi.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rotate_left(mut a: *mut int32_t, mut n: size_t, mut k: size_t) {
    if n == 0 as size_t {
        return;
    }
    k = k.wrapping_rem(n);
    reverse_range(a, 0 as size_t, k);
    reverse_range(a, k, n);
    reverse_range(a, 0 as size_t, n);
}
#[no_mangle]
pub unsafe extern "C" fn partition_even_odd(mut a: *mut int32_t, mut n: size_t) -> size_t {
    let mut w: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        if *a.offset(i as isize) & 1 as int32_t == 0 as int32_t {
            let mut t: int32_t = *a.offset(i as isize);
            let mut j: size_t = i;
            while j > w {
                *a.offset(j as isize) = *a.offset(j.wrapping_sub(1 as size_t) as isize);
                j = j.wrapping_sub(1);
            }
            *a.offset(w as isize) = t;
            w = w.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return w;
}
