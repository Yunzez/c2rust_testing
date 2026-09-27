#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn is_space(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == ' ' as i32
        || c as ::core::ffi::c_int == '\t' as i32
        || c as ::core::ffi::c_int == '\r' as i32
        || c as ::core::ffi::c_int == '\n' as i32
        || c as ::core::ffi::c_int == '\u{c}' as i32
        || c as ::core::ffi::c_int == '\u{b}' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn skip_ws(mut p: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    while *p as ::core::ffi::c_int != 0 && is_space(*p) != 0 {
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn rtrim(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    while end > start && is_space(*end.offset(-(1 as ::core::ffi::c_int) as isize)) != 0 {
        end = end.offset(-1);
    }
    return end;
}
unsafe extern "C" fn copy_span(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut cap: size_t,
) -> size_t {
    let mut n: size_t = end.offset_from(start) as ::core::ffi::c_long as size_t;
    if n.wrapping_add(1 as size_t) > cap {
        return -(1 as ::core::ffi::c_int) as size_t;
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        n,
    );
    *dst.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn kv_parse(
    mut line: *const ::core::ffi::c_char,
    mut key: *mut ::core::ffi::c_char,
    mut key_cap: size_t,
    mut val: *mut ::core::ffi::c_char,
    mut val_cap: size_t,
) -> ::core::ffi::c_int {
    if line.is_null() || key.is_null() || val.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut p: *const ::core::ffi::c_char = skip_ws(line);
    if *p as ::core::ffi::c_int == '\0' as i32 || *p as ::core::ffi::c_int == '#' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    let mut eq: *const ::core::ffi::c_char = strchr(p, '=' as i32);
    if eq.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut key_start: *const ::core::ffi::c_char = p;
    let mut key_end: *const ::core::ffi::c_char = rtrim(key_start, eq);
    if key_end == key_start {
        return -(1 as ::core::ffi::c_int);
    }
    let mut val_start: *const ::core::ffi::c_char =
        skip_ws(eq.offset(1 as ::core::ffi::c_int as isize));
    let mut val_end: *const ::core::ffi::c_char =
        rtrim(val_start, val_start.offset(strlen(val_start) as isize));
    if val_end.offset_from(val_start) as ::core::ffi::c_long >= 2 as ::core::ffi::c_long
        && *val_start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32
        && *val_end.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '"' as i32
    {
        val_start = val_start.offset(1);
        val_end = val_end.offset(-1);
    }
    if copy_span(key_start, key_end, key, key_cap) == -(1 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if copy_span(val_start, val_end, val, val_cap) == -(1 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
