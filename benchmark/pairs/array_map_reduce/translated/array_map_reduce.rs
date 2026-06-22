#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = usize;
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn map_array(
    mut src: *const int32_t,
    mut dst: *mut int32_t,
    mut n: size_t,
    mut fn_0: Option<unsafe extern "C" fn(int32_t) -> int32_t>,
) {
    let mut i: size_t = 0 as size_t;
    while i < n {
        *dst.offset(i as isize) = fn_0.expect("non-null function pointer")(*src.offset(i as isize));
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn fold_array(
    mut src: *const int32_t,
    mut n: size_t,
    mut acc0: int32_t,
    mut op: Option<unsafe extern "C" fn(int32_t, int32_t) -> int32_t>,
) -> int32_t {
    let mut acc: int32_t = acc0;
    let mut i: size_t = 0 as size_t;
    while i < n {
        acc = op.expect("non-null function pointer")(acc, *src.offset(i as isize));
        i = i.wrapping_add(1);
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn map_then_reduce(
    mut src: *const int32_t,
    mut scratch: *mut int32_t,
    mut n: size_t,
    mut map_fn: Option<unsafe extern "C" fn(int32_t) -> int32_t>,
    mut reduce_fn: Option<unsafe extern "C" fn(int32_t, int32_t) -> int32_t>,
    mut init: int32_t,
) -> int32_t {
    if src.is_null() || scratch.is_null() || map_fn.is_none() || reduce_fn.is_none() {
        return init;
    }
    map_array(src, scratch, n, map_fn);
    return fold_array(scratch, n, init, reduce_fn);
}
