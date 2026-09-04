#![allow(unused_imports, unused_variables, dead_code)]

use crate::bounds::quadtree_bounds_extend;
use crate::bounds::quadtree_bounds_free;
use crate::bounds::quadtree_bounds_new;
use crate::point::quadtree_point_free;

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
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut quadtree_node,
    pub nw: *mut quadtree_node,
    pub se: *mut quadtree_node,
    pub sw: *mut quadtree_node,
    pub bounds: *mut quadtree_bounds,
    pub point: *mut quadtree_point,
    pub key: *mut ::core::ffi::c_void,
}
pub unsafe fn quadtree_node_free(
    node: *mut quadtree_node,
    key_free: unsafe extern "C" fn(*mut ::core::ffi::c_void),
) {
    if node.is_null() {
        return;
    }
    if !(*node).nw.is_null() {
        quadtree_node_free((*node).nw, key_free);
    }
    if !(*node).ne.is_null() {
        quadtree_node_free((*node).ne, key_free);
    }
    if !(*node).sw.is_null() {
        quadtree_node_free((*node).sw, key_free);
    }
    if !(*node).se.is_null() {
        quadtree_node_free((*node).se, key_free);
    }
    quadtree_bounds_free((*node).bounds);
    quadtree_node_reset(node, key_free);
    libc::free(node.cast());
}
pub unsafe fn quadtree_node_isempty(node: *mut quadtree_node) -> ::core::ffi::c_int {
    if (*node).nw.is_null()
        && (*node).ne.is_null()
        && (*node).sw.is_null()
        && (*node).se.is_null()
        && quadtree_node_isleaf(node) == 0
    {
        1
    } else {
        0
    }
}
pub unsafe fn quadtree_node_isleaf(node: *mut quadtree_node) -> ::core::ffi::c_int {
    ((*node).point != core::ptr::null_mut()) as ::core::ffi::c_int
}
pub unsafe fn quadtree_node_ispointer(node: *mut quadtree_node) -> ::core::ffi::c_int {
    if !(*node).nw.is_null()
        && !(*node).ne.is_null()
        && !(*node).sw.is_null()
        && !(*node).se.is_null()
        && quadtree_node_isleaf(node) == 0
    {
        1
    } else {
        0
    }
}
pub unsafe fn quadtree_node_new() -> *mut quadtree_node {
    use core::mem;
    use libc::malloc;
    let node = malloc(mem::size_of::<quadtree_node>()) as *mut quadtree_node;
    if node.is_null() {
        return core::ptr::null_mut();
    }
    (*node).ne = core::ptr::null_mut();
    (*node).nw = core::ptr::null_mut();
    (*node).se = core::ptr::null_mut();
    (*node).sw = core::ptr::null_mut();
    (*node).point = core::ptr::null_mut();
    (*node).bounds = core::ptr::null_mut();
    (*node).key = core::ptr::null_mut();
    node
}
pub unsafe fn quadtree_node_reset(
    node: *mut quadtree_node,
    key_free: unsafe extern "C" fn(*mut ::core::ffi::c_void),
) {
    quadtree_point_free((*node).point);
    key_free((*node).key);
}
pub unsafe fn quadtree_node_with_bounds(
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
) -> *mut quadtree_node {
    let mut node: *mut quadtree_node;
    node = quadtree_node_new();
    if node.is_null() {
        return core::ptr::null_mut();
    }
    (*node).bounds = quadtree_bounds_new();
    if (*node).bounds.is_null() {
        return core::ptr::null_mut();
    }
    quadtree_bounds_extend((*node).bounds, maxx, maxy);
    quadtree_bounds_extend((*node).bounds, minx, miny);
    node
}
