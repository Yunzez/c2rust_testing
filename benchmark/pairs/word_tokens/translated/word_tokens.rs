#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn hash_word(mut w: *const ::core::ffi::c_char) -> uint64_t {
    let mut h: uint64_t = 1469598103934665603 as uint64_t;
    let mut p: *const ::core::ffi::c_char = w;
    while *p != 0 {
        h ^= *p as ::core::ffi::c_uchar as uint64_t;
        h = (h as ::core::ffi::c_ulonglong).wrapping_mul(1099511628211 as ::core::ffi::c_ulonglong)
            as uint64_t as uint64_t;
        p = p.offset(1);
    }
    return h;
}
unsafe extern "C" fn sort_words(mut words: *mut *mut ::core::ffi::c_char, mut n: size_t) {
    let mut i: size_t = 1 as size_t;
    while i < n {
        let mut key: *mut ::core::ffi::c_char = *words.offset(i as isize);
        let mut j: size_t = i;
        while j > 0 as size_t
            && strcmp(*words.offset(j.wrapping_sub(1 as size_t) as isize), key)
                > 0 as ::core::ffi::c_int
        {
            let ref mut fresh0 = *words.offset(j as isize);
            *fresh0 = *words.offset(j.wrapping_sub(1 as size_t) as isize);
            j = j.wrapping_sub(1);
        }
        let ref mut fresh1 = *words.offset(j as isize);
        *fresh1 = key;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn copy_view(
    mut words: *mut *mut ::core::ffi::c_char,
    mut n: size_t,
) -> *mut *mut ::core::ffi::c_char {
    let mut v: *mut *mut ::core::ffi::c_char =
        malloc(n.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t))
            as *mut *mut ::core::ffi::c_char;
    if v.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    let mut i: size_t = 0 as size_t;
    while i < n {
        let ref mut fresh2 = *v.offset(i as isize);
        *fresh2 = *words.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn fold_unique_words(
    mut words: *mut *mut ::core::ffi::c_char,
    mut count: size_t,
) -> uint64_t {
    if words.is_null() || count == 0 as size_t {
        return 0 as uint64_t;
    }
    let mut view: *mut *mut ::core::ffi::c_char = copy_view(words, count);
    if view.is_null() {
        return 0 as uint64_t;
    }
    sort_words(view, count);
    let mut acc: uint64_t = 0 as uint64_t;
    let mut prev: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: size_t = 0 as size_t;
    while i < count {
        if !(!prev.is_null() && strcmp(prev, *view.offset(i as isize)) == 0 as ::core::ffi::c_int) {
            acc = acc.wrapping_mul(31 as uint64_t) ^ hash_word(*view.offset(i as isize));
            prev = *view.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    free(view as *mut ::core::ffi::c_void);
    return acc;
}
