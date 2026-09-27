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
