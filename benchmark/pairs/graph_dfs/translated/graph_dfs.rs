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
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub to: size_t,
    pub next: *mut Node,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn alloc_buckets(mut n: size_t) -> *mut *mut Node {
    let mut b: *mut *mut Node =
        malloc(n.wrapping_mul(::core::mem::size_of::<*mut Node>() as size_t)) as *mut *mut Node;
    if b.is_null() {
        return ::core::ptr::null_mut::<*mut Node>();
    }
    let mut i: size_t = 0 as size_t;
    while i < n {
        let ref mut fresh0 = *b.offset(i as isize);
        *fresh0 = ::core::ptr::null_mut::<Node>();
        i = i.wrapping_add(1);
    }
    return b;
}
unsafe extern "C" fn add_edge(
    mut buckets: *mut *mut Node,
    mut from: size_t,
    mut to: size_t,
) -> ::core::ffi::c_int {
    let mut e: *mut Node = malloc(::core::mem::size_of::<Node>() as size_t) as *mut Node;
    if e.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*e).to = to;
    (*e).next = *buckets.offset(from as isize) as *mut Node;
    let ref mut fresh1 = *buckets.offset(from as isize);
    *fresh1 = e;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn free_buckets(mut buckets: *mut *mut Node, mut n: size_t) {
    if buckets.is_null() {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < n {
        let mut e: *mut Node = *buckets.offset(i as isize);
        while !e.is_null() {
            let mut nx: *mut Node = (*e).next as *mut Node;
            free(e as *mut ::core::ffi::c_void);
            e = nx;
        }
        i = i.wrapping_add(1);
    }
    free(buckets as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn dfs(
    mut buckets: *mut *mut Node,
    mut seen: *mut ::core::ffi::c_uchar,
    mut v: size_t,
) -> int64_t {
    if *seen.offset(v as isize) != 0 {
        return 0 as int64_t;
    }
    *seen.offset(v as isize) = 1 as ::core::ffi::c_uchar;
    let mut acc: int64_t = 1 as int64_t;
    let mut e: *mut Node = *buckets.offset(v as isize);
    while !e.is_null() {
        acc += dfs(buckets, seen, (*e).to);
        e = (*e).next as *mut Node;
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn count_reachable(
    mut n: size_t,
    mut edges: *const [size_t; 2],
    mut m: size_t,
) -> int64_t {
    if n == 0 as size_t {
        return 0 as int64_t;
    }
    let mut buckets: *mut *mut Node = alloc_buckets(n);
    if buckets.is_null() {
        return 0 as int64_t;
    }
    let mut i: size_t = 0 as size_t;
    while i < m {
        let mut from: size_t = (*edges.offset(i as isize))[0 as ::core::ffi::c_int as usize];
        let mut to: size_t = (*edges.offset(i as isize))[1 as ::core::ffi::c_int as usize];
        if from < n && to < n {
            if add_edge(buckets, from, to) != 0 as ::core::ffi::c_int {
                free_buckets(buckets, n);
                return 0 as int64_t;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut seen: *mut ::core::ffi::c_uchar =
        calloc(n, ::core::mem::size_of::<::core::ffi::c_uchar>() as size_t)
            as *mut ::core::ffi::c_uchar;
    if seen.is_null() {
        free_buckets(buckets, n);
        return 0 as int64_t;
    }
    let mut reachable: int64_t = dfs(buckets, seen, 0 as size_t);
    free(seen as *mut ::core::ffi::c_void);
    free_buckets(buckets, n);
    return reachable;
}
