#![allow(unused_imports, unused_variables, dead_code)]

use crate::point::quadtree_point_free;
use crate::point::quadtree_point_new;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point,
    pub se: *mut quadtree_point,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub unsafe fn quadtree_bounds_extend(bounds: *mut quadtree_bounds, x: f64, y: f64) {
    use core::cmp::Ordering;
    fn fmin(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }
    fn fmax(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }
    fn fabs(a: f64) -> f64 {
        a.abs()
    }
    if bounds.is_null() {
        return;
    }
    let b = &mut *bounds;
    let nw = &mut *b.nw;
    let se = &mut *b.se;
    nw.x = fmin(x, nw.x);
    nw.y = fmax(y, nw.y);
    se.x = fmax(x, se.x);
    se.y = fmin(y, se.y);
    b.width = fabs(nw.x - se.x);
    b.height = fabs(nw.y - se.y);
}
pub unsafe fn quadtree_bounds_free(bounds: *mut quadtree_bounds) {
    extern "C" {
        fn free(ptr: *mut libc::c_void);
    }
    if bounds.is_null() {
        return;
    }
    quadtree_point_free((*bounds).nw);
    quadtree_point_free((*bounds).se);
    free(bounds as *mut libc::c_void);
}
pub unsafe fn quadtree_bounds_new() -> *mut quadtree_bounds {
    type Bounds = quadtree_bounds;
    let bounds: *mut Bounds = libc::malloc(::core::mem::size_of::<Bounds>()) as *mut Bounds;
    if bounds.is_null() {
        return core::ptr::null_mut();
    }
    (*bounds).nw = quadtree_point_new(1e308f64, -1e308f64);
    (*bounds).se = quadtree_point_new(-1e308f64, 1e308f64);
    (*bounds).width = 0.0f64;
    (*bounds).height = 0.0f64;
    bounds
}
