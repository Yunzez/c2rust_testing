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
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn alloc_matrix(mut r: size_t, mut c: size_t) -> *mut *mut ::core::ffi::c_int {
    let mut m: *mut *mut ::core::ffi::c_int =
        malloc(r.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t))
            as *mut *mut ::core::ffi::c_int;
    if m.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    }
    let mut i: size_t = 0 as size_t;
    while i < r {
        let ref mut fresh0 = *m.offset(i as isize);
        *fresh0 = malloc(c.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t))
            as *mut ::core::ffi::c_int;
        if (*m.offset(i as isize)).is_null() {
            let mut k: size_t = 0 as size_t;
            while k < i {
                free(*m.offset(k as isize) as *mut ::core::ffi::c_void);
                k = k.wrapping_add(1);
            }
            free(m as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
        }
        let mut j: size_t = 0 as size_t;
        while j < c {
            *(*m.offset(i as isize)).offset(j as isize) = 0 as ::core::ffi::c_int;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return m;
}
unsafe extern "C" fn free_matrix(mut m: *mut *mut ::core::ffi::c_int, mut r: size_t) {
    if m.is_null() {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < r {
        free(*m.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(m as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn transpose(
    mut src: *mut *mut ::core::ffi::c_int,
    mut r: size_t,
    mut c: size_t,
) -> *mut *mut ::core::ffi::c_int {
    let mut t: *mut *mut ::core::ffi::c_int = alloc_matrix(c, r);
    if t.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    }
    let mut i: size_t = 0 as size_t;
    while i < r {
        let mut j: size_t = 0 as size_t;
        while j < c {
            *(*t.offset(j as isize)).offset(i as isize) =
                *(*src.offset(i as isize)).offset(j as isize);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return t;
}
unsafe extern "C" fn total_sum(
    mut m: *mut *mut ::core::ffi::c_int,
    mut r: size_t,
    mut c: size_t,
) -> int64_t {
    let mut acc: int64_t = 0 as int64_t;
    let mut i: size_t = 0 as size_t;
    while i < r {
        let mut j: size_t = 0 as size_t;
        while j < c {
            acc += *(*m.offset(i as isize)).offset(j as isize) as int64_t;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn matrix_transpose_and_sum(
    mut mat: *mut *mut ::core::ffi::c_int,
    mut rows: size_t,
    mut cols: size_t,
) -> int64_t {
    if mat.is_null() || rows == 0 as size_t || cols == 0 as size_t {
        return 0 as int64_t;
    }
    let mut s1: int64_t = total_sum(mat, rows, cols);
    let mut t: *mut *mut ::core::ffi::c_int = transpose(mat, rows, cols);
    if t.is_null() {
        return s1;
    }
    let mut s2: int64_t = total_sum(t, cols, rows);
    free_matrix(t, cols);
    return s1 + s2;
}
