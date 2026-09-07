// GENERATED for the RQ4 coverage experiment by scratchpad/rq4_cov/flatten_rust.py.
// Module bodies are copied byte-for-byte from
// tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/ (== fuzz/bzip2_c2rust_e3/src/).
// Only the module wrappers and the root re-exports below are added.
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut, internal_features,
         unused_imports, unpredictable_function_pointer_comparisons)]

pub mod bounds {
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
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmax(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fmin(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn quadtree_point_new(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> *mut quadtree_point_t;
    fn quadtree_point_free(point: *mut quadtree_point_t);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_extend(
    mut bounds: *mut quadtree_bounds_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) {
    (*(*bounds).nw).x = fmin(x, (*(*bounds).nw).x);
    (*(*bounds).nw).y = fmax(y, (*(*bounds).nw).y);
    (*(*bounds).se).x = fmax(x, (*(*bounds).se).x);
    (*(*bounds).se).y = fmin(y, (*(*bounds).se).y);
    (*bounds).width = fabs((*(*bounds).nw).x - (*(*bounds).se).x);
    (*bounds).height = fabs((*(*bounds).nw).y - (*(*bounds).se).y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_free(mut bounds: *mut quadtree_bounds_t) {
    quadtree_point_free((*bounds).nw);
    quadtree_point_free((*bounds).se);
    free(bounds as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_new() -> *mut quadtree_bounds_t {
    let mut bounds: *mut quadtree_bounds_t = ::core::ptr::null_mut::<quadtree_bounds_t>();
    bounds =
        malloc(::core::mem::size_of::<quadtree_bounds_t>() as size_t) as *mut quadtree_bounds_t;
    if bounds.is_null() {
        return ::core::ptr::null_mut::<quadtree_bounds_t>();
    }
    (*bounds).nw = quadtree_point_new(
        ::core::f32::INFINITY as ::core::ffi::c_double,
        -::core::f32::INFINITY as ::core::ffi::c_double,
    );
    (*bounds).se = quadtree_point_new(
        -::core::f32::INFINITY as ::core::ffi::c_double,
        ::core::f32::INFINITY as ::core::ffi::c_double,
    );
    (*bounds).width = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*bounds).height = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    return bounds;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

}

pub mod node {
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
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn quadtree_bounds_new() -> *mut quadtree_bounds_t;
    fn quadtree_bounds_extend(
        bounds: *mut quadtree_bounds_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn quadtree_bounds_free(bounds: *mut quadtree_bounds_t);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut quadtree_node,
    pub nw: *mut quadtree_node,
    pub se: *mut quadtree_node,
    pub sw: *mut quadtree_node,
    pub bounds: *mut quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut ::core::ffi::c_void,
}
pub type quadtree_node_t = quadtree_node;
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_ispointer(
    mut node: *mut quadtree_node_t,
) -> ::core::ffi::c_int {
    return (!(*node).nw.is_null()
        && !(*node).ne.is_null()
        && !(*node).sw.is_null()
        && !(*node).se.is_null()
        && quadtree_node_isleaf(node) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isempty(
    mut node: *mut quadtree_node_t,
) -> ::core::ffi::c_int {
    return ((*node).nw.is_null()
        && (*node).ne.is_null()
        && (*node).sw.is_null()
        && (*node).se.is_null()
        && quadtree_node_isleaf(node) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isleaf(
    mut node: *mut quadtree_node_t,
) -> ::core::ffi::c_int {
    return ((*node).point != NULL as *mut quadtree_point_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_reset(
    mut node: *mut quadtree_node_t,
    mut key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    quadtree_point_free((*node).point);
    Some(key_free.expect("non-null function pointer")).expect("non-null function pointer")(
        (*node).key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_new() -> *mut quadtree_node_t {
    let mut node: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    node = malloc(::core::mem::size_of::<quadtree_node_t>() as size_t) as *mut quadtree_node_t;
    if node.is_null() {
        return ::core::ptr::null_mut::<quadtree_node_t>();
    }
    (*node).ne = ::core::ptr::null_mut::<quadtree_node>();
    (*node).nw = ::core::ptr::null_mut::<quadtree_node>();
    (*node).se = ::core::ptr::null_mut::<quadtree_node>();
    (*node).sw = ::core::ptr::null_mut::<quadtree_node>();
    (*node).point = ::core::ptr::null_mut::<quadtree_point_t>();
    (*node).bounds = ::core::ptr::null_mut::<quadtree_bounds_t>();
    (*node).key = NULL;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_with_bounds(
    mut minx: ::core::ffi::c_double,
    mut miny: ::core::ffi::c_double,
    mut maxx: ::core::ffi::c_double,
    mut maxy: ::core::ffi::c_double,
) -> *mut quadtree_node_t {
    let mut node: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    node = quadtree_node_new();
    if node.is_null() {
        return ::core::ptr::null_mut::<quadtree_node_t>();
    }
    (*node).bounds = quadtree_bounds_new();
    if (*node).bounds.is_null() {
        return ::core::ptr::null_mut::<quadtree_node_t>();
    }
    quadtree_bounds_extend((*node).bounds, maxx, maxy);
    quadtree_bounds_extend((*node).bounds, minx, miny);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_free(
    mut node: *mut quadtree_node_t,
    mut key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    if !(*node).nw.is_null() {
        quadtree_node_free((*node).nw as *mut quadtree_node_t, key_free);
    }
    if !(*node).ne.is_null() {
        quadtree_node_free((*node).ne as *mut quadtree_node_t, key_free);
    }
    if !(*node).sw.is_null() {
        quadtree_node_free((*node).sw as *mut quadtree_node_t, key_free);
    }
    if !(*node).se.is_null() {
        quadtree_node_free((*node).se as *mut quadtree_node_t, key_free);
    }
    quadtree_bounds_free((*node).bounds);
    quadtree_node_reset(node, key_free);
    free(node as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

}

pub mod point {
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
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> *mut quadtree_point_t {
    let mut point: *mut quadtree_point_t = ::core::ptr::null_mut::<quadtree_point_t>();
    point = malloc(::core::mem::size_of::<quadtree_point_t>() as size_t) as *mut quadtree_point_t;
    if point.is_null() {
        return ::core::ptr::null_mut::<quadtree_point_t>();
    }
    (*point).x = x;
    (*point).y = y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: *mut quadtree_point_t) {
    free(point as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

}

pub mod quadtree {
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
    fn quadtree_point_new(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> *mut quadtree_point_t;
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn quadtree_node_free(
        node: *mut quadtree_node_t,
        value_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn quadtree_node_ispointer(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_node_isempty(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_node_isleaf(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_node_reset(
        node: *mut quadtree_node_t,
        key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn quadtree_node_with_bounds(
        minx: ::core::ffi::c_double,
        miny: ::core::ffi::c_double,
        maxx: ::core::ffi::c_double,
        maxy: ::core::ffi::c_double,
    ) -> *mut quadtree_node_t;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut quadtree_node,
    pub nw: *mut quadtree_node,
    pub se: *mut quadtree_node,
    pub sw: *mut quadtree_node,
    pub bounds: *mut quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut ::core::ffi::c_void,
}
pub type quadtree_node_t = quadtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree {
    pub root: *mut quadtree_node_t,
    pub key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub length: ::core::ffi::c_uint,
}
pub type quadtree_t = quadtree;
unsafe extern "C" fn node_contains_(
    mut outer: *mut quadtree_node_t,
    mut it: *mut quadtree_point_t,
) -> ::core::ffi::c_int {
    return (!(*outer).bounds.is_null()
        && (*(*(*outer).bounds).nw).x <= (*it).x
        && (*(*(*outer).bounds).nw).y >= (*it).y
        && (*(*(*outer).bounds).se).x >= (*it).x
        && (*(*(*outer).bounds).se).y <= (*it).y) as ::core::ffi::c_int;
}
unsafe extern "C" fn elision_(mut key: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn reset_node_(mut tree: *mut quadtree_t, mut node: *mut quadtree_node_t) {
    if (*tree).key_free.is_some() {
        quadtree_node_reset(node, (*tree).key_free);
    } else {
        quadtree_node_reset(
            node,
            Some(elision_ as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    };
}
unsafe extern "C" fn get_quadrant_(
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
) -> *mut quadtree_node_t {
    if node_contains_((*root).nw as *mut quadtree_node_t, point) != 0 {
        return (*root).nw as *mut quadtree_node_t;
    }
    if node_contains_((*root).ne as *mut quadtree_node_t, point) != 0 {
        return (*root).ne as *mut quadtree_node_t;
    }
    if node_contains_((*root).sw as *mut quadtree_node_t, point) != 0 {
        return (*root).sw as *mut quadtree_node_t;
    }
    if node_contains_((*root).se as *mut quadtree_node_t, point) != 0 {
        return (*root).se as *mut quadtree_node_t;
    }
    return ::core::ptr::null_mut::<quadtree_node_t>();
}
unsafe extern "C" fn split_node_(
    mut tree: *mut quadtree_t,
    mut node: *mut quadtree_node_t,
) -> ::core::ffi::c_int {
    let mut nw: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    let mut ne: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    let mut sw: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    let mut se: *mut quadtree_node_t = ::core::ptr::null_mut::<quadtree_node_t>();
    let mut old: *mut quadtree_point_t = ::core::ptr::null_mut::<quadtree_point_t>();
    let mut key: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut x: ::core::ffi::c_double = (*(*(*node).bounds).nw).x;
    let mut y: ::core::ffi::c_double = (*(*(*node).bounds).nw).y;
    let mut hw: ::core::ffi::c_double =
        (*(*node).bounds).width / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut hh: ::core::ffi::c_double =
        (*(*node).bounds).height / 2 as ::core::ffi::c_int as ::core::ffi::c_double;
    nw = quadtree_node_with_bounds(x, y - hh, x + hw, y);
    if nw.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ne = quadtree_node_with_bounds(
        x + hw,
        y - hh,
        x + hw * 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        y,
    );
    if ne.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    sw = quadtree_node_with_bounds(
        x,
        y - hh * 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        x + hw,
        y - hh,
    );
    if sw.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    se = quadtree_node_with_bounds(
        x + hw,
        y - hh * 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        x + hw * 2 as ::core::ffi::c_int as ::core::ffi::c_double,
        y - hh,
    );
    if se.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*node).nw = nw as *mut quadtree_node;
    (*node).ne = ne as *mut quadtree_node;
    (*node).sw = sw as *mut quadtree_node;
    (*node).se = se as *mut quadtree_node;
    old = (*node).point;
    key = (*node).key;
    (*node).point = ::core::ptr::null_mut::<quadtree_point_t>();
    (*node).key = NULL;
    return insert_(tree, node, old, key);
}
unsafe extern "C" fn find_(
    mut node: *mut quadtree_node_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> *mut quadtree_point_t {
    if node.is_null() {
        return ::core::ptr::null_mut::<quadtree_point_t>();
    }
    if quadtree_node_isleaf(node) != 0 {
        if (*(*node).point).x == x && (*(*node).point).y == y {
            return (*node).point;
        }
    } else if quadtree_node_ispointer(node) != 0 {
        let mut test: quadtree_point_t = quadtree_point { x: 0., y: 0. };
        test.x = x;
        test.y = y;
        return find_(get_quadrant_(node, &raw mut test), x, y);
    }
    return ::core::ptr::null_mut::<quadtree_point_t>();
}
unsafe extern "C" fn insert_(
    mut tree: *mut quadtree_t,
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
    mut key: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if quadtree_node_isempty(root) != 0 {
        (*root).point = point;
        (*root).key = key;
        return 1 as ::core::ffi::c_int;
    } else if quadtree_node_isleaf(root) != 0 {
        if (*(*root).point).x == (*point).x && (*(*root).point).y == (*point).y {
            reset_node_(tree, root);
            (*root).point = point;
            (*root).key = key;
            return 2 as ::core::ffi::c_int;
        } else {
            if split_node_(tree, root) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            return insert_(tree, root, point, key);
        }
    } else if quadtree_node_ispointer(root) != 0 {
        let mut quadrant: *mut quadtree_node_t = get_quadrant_(root, point);
        return if quadrant.is_null() {
            0 as ::core::ffi::c_int
        } else {
            insert_(tree, quadrant, point, key)
        };
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_new(
    mut minx: ::core::ffi::c_double,
    mut miny: ::core::ffi::c_double,
    mut maxx: ::core::ffi::c_double,
    mut maxy: ::core::ffi::c_double,
) -> *mut quadtree_t {
    let mut tree: *mut quadtree_t = ::core::ptr::null_mut::<quadtree_t>();
    tree = malloc(::core::mem::size_of::<quadtree_t>() as size_t) as *mut quadtree_t;
    if tree.is_null() {
        return ::core::ptr::null_mut::<quadtree_t>();
    }
    (*tree).root = quadtree_node_with_bounds(minx, miny, maxx, maxy);
    if (*tree).root.is_null() {
        return ::core::ptr::null_mut::<quadtree_t>();
    }
    (*tree).key_free = None;
    (*tree).length = 0 as ::core::ffi::c_uint;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_insert(
    mut tree: *mut quadtree_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut key: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut point: *mut quadtree_point_t = ::core::ptr::null_mut::<quadtree_point_t>();
    let mut insert_status: ::core::ffi::c_int = 0;
    point = quadtree_point_new(x, y);
    if point.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if node_contains_((*tree).root, point) == 0 {
        quadtree_point_free(point);
        return 0 as ::core::ffi::c_int;
    }
    insert_status = insert_(tree, (*tree).root, point, key);
    if insert_status == 0 {
        quadtree_point_free(point);
        return 0 as ::core::ffi::c_int;
    }
    if insert_status == 1 as ::core::ffi::c_int {
        (*tree).length = (*tree).length.wrapping_add(1);
    }
    return insert_status;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_search(
    mut tree: *mut quadtree_t,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> *mut quadtree_point_t {
    return find_((*tree).root, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_free(mut tree: *mut quadtree_t) {
    if (*tree).key_free.is_some() {
        quadtree_node_free((*tree).root, (*tree).key_free);
    } else {
        quadtree_node_free(
            (*tree).root,
            Some(elision_ as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
    free(tree as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_walk(
    mut root: *mut quadtree_node_t,
    mut descent: Option<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
    mut ascent: Option<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
) {
    Some(descent.expect("non-null function pointer")).expect("non-null function pointer")(root);
    if !(*root).nw.is_null() {
        quadtree_walk((*root).nw as *mut quadtree_node_t, descent, ascent);
    }
    if !(*root).ne.is_null() {
        quadtree_walk((*root).ne as *mut quadtree_node_t, descent, ascent);
    }
    if !(*root).sw.is_null() {
        quadtree_walk((*root).sw as *mut quadtree_node_t, descent, ascent);
    }
    if !(*root).se.is_null() {
        quadtree_walk((*root).se as *mut quadtree_node_t, descent, ascent);
    }
    Some(ascent.expect("non-null function pointer")).expect("non-null function pointer")(root);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

}

pub mod test {
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn quadtree_point_new(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> *mut quadtree_point_t;
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn quadtree_bounds_new() -> *mut quadtree_bounds_t;
    fn quadtree_bounds_extend(
        bounds: *mut quadtree_bounds_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    );
    fn quadtree_bounds_free(bounds: *mut quadtree_bounds_t);
    fn quadtree_node_new() -> *mut quadtree_node_t;
    fn quadtree_node_ispointer(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_node_isempty(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_node_isleaf(node: *mut quadtree_node_t) -> ::core::ffi::c_int;
    fn quadtree_new(
        minx: ::core::ffi::c_double,
        miny: ::core::ffi::c_double,
        maxx: ::core::ffi::c_double,
        maxy: ::core::ffi::c_double,
    ) -> *mut quadtree_t;
    fn quadtree_free(tree: *mut quadtree_t);
    fn quadtree_search(
        tree: *mut quadtree_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
    ) -> *mut quadtree_point_t;
    fn quadtree_insert(
        tree: *mut quadtree_t,
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        key: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn quadtree_walk(
        root: *mut quadtree_node_t,
        descent_0: Option<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
        ascent_0: Option<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut quadtree_node,
    pub nw: *mut quadtree_node,
    pub se: *mut quadtree_node,
    pub sw: *mut quadtree_node,
    pub bounds: *mut quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut ::core::ffi::c_void,
}
pub type quadtree_node_t = quadtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree {
    pub root: *mut quadtree_node_t,
    pub key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub length: ::core::ffi::c_uint,
}
pub type quadtree_t = quadtree;
#[no_mangle]
pub unsafe extern "C" fn descent(mut node: *mut quadtree_node_t) {}
#[no_mangle]
pub unsafe extern "C" fn ascent(mut node: *mut quadtree_node_t) {}
unsafe extern "C" fn test_node() {
    let mut node: *mut quadtree_node_t = quadtree_node_new();
    '_c2rust_label: {
        if quadtree_node_isleaf(node) == 0 {
        } else {
            __assert_fail(
                b"!quadtree_node_isleaf(node)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                20 as ::core::ffi::c_uint,
                b"void test_node(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if quadtree_node_isempty(node) != 0 {
        } else {
            __assert_fail(
                b"quadtree_node_isempty(node)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                21 as ::core::ffi::c_uint,
                b"void test_node(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if quadtree_node_ispointer(node) == 0 {
        } else {
            __assert_fail(
                b"!quadtree_node_ispointer(node)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                22 as ::core::ffi::c_uint,
                b"void test_node(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    free(node as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn test_bounds() {
    let mut bounds: *mut quadtree_bounds_t = quadtree_bounds_new();
    '_c2rust_label: {
        if !bounds.is_null() {
        } else {
            __assert_fail(
                b"bounds\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                30 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*(*bounds).nw).x == ::core::f32::INFINITY as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"bounds->nw->x == INFINITY\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                31 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*(*bounds).se).x == -::core::f32::INFINITY as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"bounds->se->x == -INFINITY\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                32 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    quadtree_bounds_extend(bounds, 5.0f64, 5.0f64);
    '_c2rust_label_2: {
        if (*(*bounds).nw).x == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->nw->x == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                35 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*(*bounds).se).x == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->se->x == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                36 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    quadtree_bounds_extend(bounds, 10.0f64, 10.0f64);
    '_c2rust_label_4: {
        if (*(*bounds).nw).y == 10.0f64 {
        } else {
            __assert_fail(
                b"bounds->nw->y == 10.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                39 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*(*bounds).nw).y == 10.0f64 {
        } else {
            __assert_fail(
                b"bounds->nw->y == 10.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                40 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*(*bounds).se).y == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->se->y == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                41 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if (*(*bounds).se).y == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->se->y == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                42 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if (*bounds).width == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->width == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if (*bounds).height == 5.0f64 {
        } else {
            __assert_fail(
                b"bounds->height == 5.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                45 as ::core::ffi::c_uint,
                b"void test_bounds(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    quadtree_bounds_free(bounds);
}
unsafe extern "C" fn test_tree() {
    let mut val: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut tree: *mut quadtree_t = quadtree_new(
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        10 as ::core::ffi::c_int as ::core::ffi::c_double,
        10 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    '_c2rust_label: {
        if (*(*(*(*tree).root).bounds).nw).x == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"tree->root->bounds->nw->x == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                56 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*(*(*(*tree).root).bounds).nw).y == 10.0f64 {
        } else {
            __assert_fail(
                b"tree->root->bounds->nw->y == 10.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*(*(*(*tree).root).bounds).se).x == 10.0f64 {
        } else {
            __assert_fail(
                b"tree->root->bounds->se->x == 10.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*(*(*(*tree).root).bounds).se).y == 1 as ::core::ffi::c_int as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"tree->root->bounds->se->y == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if quadtree_insert(
            tree,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 0, 0, &val) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if quadtree_insert(
            tree,
            110.0f64,
            110.0f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 110.0, 110.0, &val) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if quadtree_insert(
            tree,
            8.0f64,
            2.0f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) != 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 8.0, 2.0, &val) != 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*tree).length == 1 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"tree->length == 1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if (*(*(*tree).root).point).x == 8.0f64 {
        } else {
            __assert_fail(
                b"tree->root->point->x == 8.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if (*(*(*tree).root).point).y == 2.0f64 {
        } else {
            __assert_fail(
                b"tree->root->point->y == 2.0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if quadtree_insert(
            tree,
            0.0f64,
            1.0f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 0.0, 1.0, &val) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if quadtree_insert(
            tree,
            2.0f64,
            3.0f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 1 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 2.0, 3.0, &val) == 1\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_11: {
        if quadtree_insert(
            tree,
            2.0f64,
            3.0f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 2 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 2.0, 3.0, &val) == 2\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_12: {
        if (*tree).length == 2 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"tree->length == 2\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_13: {
        if (*(*tree).root).point.is_null() {
        } else {
            __assert_fail(
                b"tree->root->point == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                74 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_14: {
        if quadtree_insert(
            tree,
            3.0f64,
            1.1f64,
            &raw mut val as *mut ::core::ffi::c_void,
        ) == 1 as ::core::ffi::c_int
        {
        } else {
            __assert_fail(
                b"quadtree_insert(tree, 3.0, 1.1, &val) == 1\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                76 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_15: {
        if (*tree).length == 3 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"tree->length == 3\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                77 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_16: {
        if (*quadtree_search(tree, 3.0f64, 1.1f64)).x == 3.0f64 {
        } else {
            __assert_fail(
                b"quadtree_search(tree, 3.0, 1.1)->x == 3.0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_uint,
                b"void test_tree(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    quadtree_walk(
        (*tree).root,
        Some(ascent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
        Some(descent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
    );
    quadtree_free(tree);
}
unsafe extern "C" fn test_points() {
    let mut point: *mut quadtree_point_t = quadtree_point_new(
        5 as ::core::ffi::c_int as ::core::ffi::c_double,
        6 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    '_c2rust_label: {
        if (*point).x == 5 as ::core::ffi::c_int as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"point->x == 5\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                b"void test_points(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*point).y == 6 as ::core::ffi::c_int as ::core::ffi::c_double {
        } else {
            __assert_fail(
                b"point->y == 6\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yunzez/c2rust_testing/tools/c2rust_crustbench/out/quadtree/test.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                87 as ::core::ffi::c_uint,
                b"void test_points(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    quadtree_point_free(point);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    printf(b"\x1B[33mtree\x1B[0m \0" as *const u8 as *const ::core::ffi::c_char);
    test_tree();
    puts(b"\x1B[1;32m\xE2\x9C\x93\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"\x1B[33mnode\x1B[0m \0" as *const u8 as *const ::core::ffi::c_char);
    test_node();
    puts(b"\x1B[1;32m\xE2\x9C\x93\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"\x1B[33mbounds\x1B[0m \0" as *const u8 as *const ::core::ffi::c_char);
    test_bounds();
    puts(b"\x1B[1;32m\xE2\x9C\x93\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    printf(b"\x1B[33mpoints\x1B[0m \0" as *const u8 as *const ::core::ffi::c_char);
    test_points();
    puts(b"\x1B[1;32m\xE2\x9C\x93\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    return 0;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *const ::core::ffi::c_char,
        ) as i32)
    }
}

}

// root re-exports so the generated harness's `translated::<entry>` resolves
// elision_: private in quadtree — exposed per-entry by --expose-entry
// find_: private in quadtree — exposed per-entry by --expose-entry
// get_quadrant_: private in quadtree — exposed per-entry by --expose-entry
// insert_: private in quadtree — exposed per-entry by --expose-entry
// node_contains_: private in quadtree — exposed per-entry by --expose-entry
pub use crate::bounds::quadtree_bounds_extend;
pub use crate::bounds::quadtree_bounds_free;
pub use crate::bounds::quadtree_bounds_new;
pub use crate::quadtree::quadtree_free;
pub use crate::quadtree::quadtree_insert;
pub use crate::quadtree::quadtree_new;
pub use crate::node::quadtree_node_free;
pub use crate::node::quadtree_node_isempty;
pub use crate::node::quadtree_node_isleaf;
pub use crate::node::quadtree_node_ispointer;
pub use crate::node::quadtree_node_new;
pub use crate::node::quadtree_node_reset;
pub use crate::node::quadtree_node_with_bounds;
pub use crate::point::quadtree_point_free;
pub use crate::point::quadtree_point_new;
pub use crate::quadtree::quadtree_search;
pub use crate::quadtree::quadtree_walk;
// reset_node_: private in quadtree — exposed per-entry by --expose-entry
// split_node_: private in quadtree — exposed per-entry by --expose-entry
