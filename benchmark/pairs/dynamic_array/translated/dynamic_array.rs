#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(raw_ref_op)]
extern "C" {
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynArray {
    pub data: *mut ::core::ffi::c_int,
    pub len: size_t,
    pub cap: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn da_init(mut a: *mut DynArray) -> ::core::ffi::c_int {
    (*a).data = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*a).len = 0 as size_t;
    (*a).cap = 0 as size_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn da_grow(mut a: *mut DynArray) -> ::core::ffi::c_int {
    if (*a).len < (*a).cap {
        return 0 as ::core::ffi::c_int;
    }
    let mut new_cap: size_t = if (*a).cap == 0 as size_t {
        4 as size_t
    } else {
        (*a).cap.wrapping_mul(2 as size_t)
    };
    let mut p: *mut ::core::ffi::c_int = realloc(
        (*a).data as *mut ::core::ffi::c_void,
        new_cap.wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*a).data = p;
    (*a).cap = new_cap;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn da_push(
    mut a: *mut DynArray,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if da_grow(a) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let fresh0 = (*a).len;
    (*a).len = (*a).len.wrapping_add(1);
    *(*a).data.offset(fresh0 as isize) = value;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn da_pop(
    mut a: *mut DynArray,
    mut out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*a).len == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    (*a).len = (*a).len.wrapping_sub(1);
    *out = *(*a).data.offset((*a).len as isize);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn da_sum(mut a: *const DynArray) -> ::core::ffi::c_long {
    let mut s: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: size_t = 0 as size_t;
    while i < (*a).len {
        s += *(*a).data.offset(i as isize) as ::core::ffi::c_long;
        i = i.wrapping_add(1);
    }
    return s;
}
unsafe extern "C" fn da_free(mut a: *mut DynArray) {
    free((*a).data as *mut ::core::ffi::c_void);
    (*a).data = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*a).len = 0 as size_t;
    (*a).cap = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn da_run(
    mut ops: *const ::core::ffi::c_int,
    mut n: size_t,
) -> ::core::ffi::c_long {
    let mut a: DynArray = DynArray {
        data: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        len: 0,
        cap: 0,
    };
    da_init(&raw mut a);
    let mut pop_acc: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: size_t = 0 as size_t;
    while i < n {
        let fresh1 = i;
        i = i.wrapping_add(1);
        let mut opcode: ::core::ffi::c_int = *ops.offset(fresh1 as isize);
        if opcode & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if i < n {
                let fresh2 = i;
                i = i.wrapping_add(1);
                value = *ops.offset(fresh2 as isize);
            }
            da_push(&raw mut a, value);
        } else {
            let mut out: ::core::ffi::c_int = 0;
            if da_pop(&raw mut a, &raw mut out) == 0 as ::core::ffi::c_int {
                pop_acc -= out as ::core::ffi::c_long;
            }
        }
    }
    let mut result: ::core::ffi::c_long = da_sum(&raw mut a) + pop_acc;
    da_free(&raw mut a);
    return result;
}
