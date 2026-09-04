#![allow(unused_imports, unused_variables, dead_code)]

pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub unsafe fn quadtree_point_free(point: *mut quadtree_point) {
    libc::free(point.cast());
}
pub unsafe fn quadtree_point_new(x: f64, y: f64) -> *mut quadtree_point {
    let point = libc::malloc(::core::mem::size_of::<quadtree_point>()) as *mut quadtree_point;
    if point.is_null() {
        return ::core::ptr::null_mut();
    }
    (*point).x = x;
    (*point).y = y;
    point
}
