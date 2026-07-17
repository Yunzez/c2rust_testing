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
