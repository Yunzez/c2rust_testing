// SACTOR × quadtree (run 2026-09-02, PARTIAL): verbatim concatenation of the UNIDIOMATIC-phase outputs
// run1_result/{point.c__9ec7d037,bounds.c__ea3019ab,node.c__baf530fc}/translated_code_unidiomatic/functions/*.rs
// (12 functions, all SACTOR-verified against the 12 samples) plus the struct definitions from
// node.c__baf530fc/translated_code_unidiomatic/structs/quadtree_node.rs. quadtree.c was refused
// (circular dependencies) and driver.c failed to link, so no Rust exists for those TUs. The idiomatic
// phase reached only point.c (2 fns); see RUN.md. Not a building crate (libc paths unresolved).
#![allow(non_camel_case_types, unused)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point,
    pub se: *mut quadtree_point,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type size_t = usize;
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

// --- point.c__9ec7d037/quadtree_point_free.rs
pub unsafe fn quadtree_point_free(point: *mut quadtree_point) {
    libc::free(point.cast());
}

// --- point.c__9ec7d037/quadtree_point_new.rs
pub unsafe fn quadtree_point_new(x: f64, y: f64) -> *mut quadtree_point {
    let point = libc::malloc(::core::mem::size_of::<quadtree_point>()) as *mut quadtree_point;
    if point.is_null() {
        return ::core::ptr::null_mut();
    }
    (*point).x = x;
    (*point).y = y;
    point
}

// --- bounds.c__ea3019ab/quadtree_bounds_extend.rs
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

// --- bounds.c__ea3019ab/quadtree_bounds_free.rs
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

// --- bounds.c__ea3019ab/quadtree_bounds_new.rs
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

// --- node.c__baf530fc/quadtree_node_free.rs
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

// --- node.c__baf530fc/quadtree_node_isempty.rs
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

// --- node.c__baf530fc/quadtree_node_isleaf.rs
pub unsafe fn quadtree_node_isleaf(node: *mut quadtree_node) -> ::core::ffi::c_int {
    ((*node).point != core::ptr::null_mut()) as ::core::ffi::c_int
}

// --- node.c__baf530fc/quadtree_node_ispointer.rs
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

// --- node.c__baf530fc/quadtree_node_new.rs
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

// --- node.c__baf530fc/quadtree_node_reset.rs
pub unsafe fn quadtree_node_reset(
    node: *mut quadtree_node,
    key_free: unsafe extern "C" fn(*mut ::core::ffi::c_void),
) {
    quadtree_point_free((*node).point);
    key_free((*node).key);
}

// --- node.c__baf530fc/quadtree_node_with_bounds.rs
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
