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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub value: ::core::ffi::c_int,
    pub next: *mut Node,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn ll_push_front(
    mut head: *mut Node,
    mut value: ::core::ffi::c_int,
) -> *mut Node {
    let mut n: *mut Node = malloc(::core::mem::size_of::<Node>() as size_t) as *mut Node;
    if n.is_null() {
        return head;
    }
    (*n).value = value;
    (*n).next = head as *mut Node;
    return n;
}
unsafe extern "C" fn ll_reverse_rec(mut cur: *mut Node, mut prev: *mut Node) -> *mut Node {
    if cur.is_null() {
        return prev;
    }
    let mut next: *mut Node = (*cur).next as *mut Node;
    (*cur).next = prev as *mut Node;
    return ll_reverse_rec(next, cur);
}
unsafe extern "C" fn ll_sum(mut head: *const Node) -> ::core::ffi::c_long {
    let mut s: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut p: *const Node = head;
    while !p.is_null() {
        s += (*p).value as ::core::ffi::c_long;
        p = (*p).next;
    }
    return s;
}
unsafe extern "C" fn ll_free(mut head: *mut Node) {
    while !head.is_null() {
        let mut next: *mut Node = (*head).next as *mut Node;
        free(head as *mut ::core::ffi::c_void);
        head = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ll_run(
    mut vals: *const ::core::ffi::c_int,
    mut n: size_t,
) -> ::core::ffi::c_long {
    let mut head: *mut Node = ::core::ptr::null_mut::<Node>();
    let mut i: size_t = 0 as size_t;
    while i < n {
        head = ll_push_front(head, *vals.offset(i as isize));
        i = i.wrapping_add(1);
    }
    head = ll_reverse_rec(head, ::core::ptr::null_mut::<Node>());
    let mut result: ::core::ffi::c_long = ll_sum(head);
    ll_free(head);
    return result;
}
