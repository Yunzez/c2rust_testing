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

pub mod src {
pub mod src {
pub mod bounds {
use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    
    
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
impl Default for quadtree_point {fn default() -> Self {Self {
x: Default::default(),
y: Default::default(),
}}}

pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: Option<Box<quadtree_point_t>>,
    pub se: Option<Box<quadtree_point_t>>,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
impl Default for quadtree_bounds {fn default() -> Self {Self {
nw: None,
se: None,
width: Default::default(),
height: Default::default(),
}}}
impl quadtree_bounds {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_bounds_t = quadtree_bounds;
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_extend(
    mut bounds: *mut quadtree_bounds_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    (*(*bounds).nw.as_deref_mut().unwrap()).x= fmin(x, (*(*bounds).nw.as_deref().unwrap()).x);
    (*(*bounds).nw.as_deref_mut().unwrap()).y= fmax(y, (*(*bounds).nw.as_deref().unwrap()).y);
    (*(*bounds).se.as_deref_mut().unwrap()).x= fmax(x, (*(*bounds).se.as_deref().unwrap()).x);
    (*(*bounds).se.as_deref_mut().unwrap()).y= fmin(y, (*(*bounds).se.as_deref().unwrap()).y);
    (*bounds).width= fabs((*(*bounds).nw.as_deref().unwrap()).x - (*(*bounds).se.as_deref().unwrap()).x);
    (*bounds).height= fabs((*(*bounds).nw.as_deref().unwrap()).y - (*(*bounds).se.as_deref().unwrap()).y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_free(mut bounds: Option<Box<quadtree_bounds_t>>) {
    crate::src::src::point::quadtree_point_free((*bounds.as_deref_mut().unwrap()).nw.take());
    crate::src::src::point::quadtree_point_free((*bounds.as_deref_mut().unwrap()).se.take());
    ();
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_new() -> Option<Box<quadtree_bounds_t>> {
    let mut bounds = None;
    bounds= Some(Box::new(<crate::src::src::bounds::quadtree_bounds as Default>::default()));
    if bounds.as_deref().is_none() {();
        return None;
    }
    (*bounds.as_deref_mut().unwrap()).nw= crate::src::src::point::quadtree_point_new(
        ::std::f32::INFINITY as libc::c_double,
        -::std::f32::INFINITY as libc::c_double,
    );
    (*bounds.as_deref_mut().unwrap()).se= crate::src::src::point::quadtree_point_new(
        -::std::f32::INFINITY as libc::c_double,
        ::std::f32::INFINITY as libc::c_double,
    );
    (*bounds.as_deref_mut().unwrap()).width= 0 as libc::c_int as libc::c_double;
    (*bounds.as_deref_mut().unwrap()).height= 0 as libc::c_int as libc::c_double;
    return bounds;
}

}

pub mod node {
use ::libc;
extern "C" {
    
    
    
    
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
impl Default for ErasedByPreprocessor0 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
impl Default for ErasedByPreprocessor1 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer1;
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut /* owning */ quadtree_node,
    pub nw: *mut /* owning */ quadtree_node,
    pub se: *mut /* owning */ quadtree_node,
    pub sw: *mut /* owning */ quadtree_node,
    pub bounds: *mut /* owning */ quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut libc::c_void,
}
impl Default for quadtree_node {fn default() -> Self {Self {
ne: std::ptr::null_mut(),
nw: std::ptr::null_mut(),
se: std::ptr::null_mut(),
sw: std::ptr::null_mut(),
bounds: std::ptr::null_mut(),
point: std::ptr::null_mut(),
key: std::ptr::null_mut(),
}}}
impl quadtree_node {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_node_t = quadtree_node;
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_ispointer(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return (!(*node).nw.is_null() && !(*node).ne.is_null() && !(*node).sw.is_null()
        && !(*node).se.is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isempty(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return ((*node).nw.is_null() && (*node).ne.is_null() && (*node).sw.is_null()
        && (*node).se.is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isleaf(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return ((*node).point != 0 as *mut libc::c_void as *mut quadtree_point_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_reset(
    mut node: *mut quadtree_node_t,
    mut key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    crate::src::src::point::quadtree_point_free(Some(Box::from_raw((*node).point)));
    (Some(key_free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*node).key);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_new() -> Option<Box<quadtree_node_t>> {
    let mut node = None;
    node= Some(Box::new(<crate::src::src::node::quadtree_node as Default>::default()));
    if node.as_deref().is_none() {();
        return None;
    }
    (*node.as_deref_mut().unwrap()).ne= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).nw= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).se= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).sw= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).point= 0 as *mut quadtree_point_t;
    (*node.as_deref_mut().unwrap()).bounds= 0 as *mut quadtree_bounds_t;
    (*node.as_deref_mut().unwrap()).key= 0 as *mut libc::c_void;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_with_bounds(
    mut minx: libc::c_double,
    mut miny: libc::c_double,
    mut maxx: libc::c_double,
    mut maxy: libc::c_double,
) -> *mut quadtree_node_t {
    let mut node = 0 as *mut quadtree_node_t;
    node= quadtree_node_new().map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if node.is_null() {();
        return 0 as *mut quadtree_node_t;
    }
    (*node).bounds= crate::src::src::bounds::quadtree_bounds_new().map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if (*node).bounds.is_null() {();
        return 0 as *mut quadtree_node_t;
    }
    crate::src::src::bounds::quadtree_bounds_extend((*node).bounds, maxx, maxy);
    crate::src::src::bounds::quadtree_bounds_extend((*node).bounds, minx, miny);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_free(
    mut node: Option<Box<quadtree_node_t>>,
    mut key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if !(*node.as_deref().unwrap()).nw.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).nw)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).ne.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).ne)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).sw.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).sw)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).se.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).se)), key_free);
    }else { (); }
    crate::src::src::bounds::quadtree_bounds_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).bounds)));
    quadtree_node_reset(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), key_free);
    ();
}

}

pub mod point {
use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor2 { dummy: () }
impl Default for ErasedByPreprocessor2 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> Option<Box<quadtree_point_t>> {
    let mut point = None;
    point= Some(Box::new(<crate::src::src::bounds::quadtree_point as Default>::default()));
    if point.as_deref().is_none() {();
        return None;
    }
    (*point.as_deref_mut().unwrap()).x= x;
    (*point.as_deref_mut().unwrap()).y= y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: Option<Box<quadtree_point_t>>) {
    ();
}

}

pub mod quadtree {
use ::libc;
extern "C" {
    
    
    
    
    
    
    
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor3 { dummy: () }
impl Default for ErasedByPreprocessor3 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor4 { dummy: () }
impl Default for ErasedByPreprocessor4 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
impl Default for ErasedByPreprocessor5 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer2;
#[repr(C)]
pub struct quadtree {
    pub root: Option<Box<quadtree_node_t>>,
    pub key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub length: libc::c_uint,
}
impl Default for quadtree {fn default() -> Self {Self {
root: None,
key_free: Default::default(),
length: Default::default(),
}}}
impl quadtree {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_t = quadtree;
unsafe extern "C" fn node_contains_(
    mut outer: *mut quadtree_node_t,
    mut it: *mut quadtree_point_t,
) -> libc::c_int {
    return (!(*outer).bounds.is_null() && (*(*(*outer).bounds).nw.as_deref().unwrap()).x < (*it).x
        && (*(*(*outer).bounds).nw.as_deref().unwrap()).y > (*it).y && (*(*(*outer).bounds).se.as_deref().unwrap()).x > (*it).x
        && (*(*(*outer).bounds).se.as_deref().unwrap()).y < (*it).y) as libc::c_int;
}
unsafe extern "C" fn elision_(mut key: *mut libc::c_void) {}
unsafe extern "C" fn reset_node_(
    mut tree: *mut quadtree_t,
    mut node: Option<&mut quadtree_node_t>,
) {
    if (*tree).key_free.is_some() {
        crate::src::src::node::quadtree_node_reset(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), (*tree).key_free);
    } else {
        crate::src::src::node::quadtree_node_reset(
            node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
            Some(elision_ as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    };
}
unsafe extern "C" fn get_quadrant_(
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
) -> *mut quadtree_node_t {
    if node_contains_((*root).nw, point) != 0 {
        return (*root).nw;
    }
    if node_contains_((*root).ne, point) != 0 {
        return (*root).ne;
    }
    if node_contains_((*root).sw, point) != 0 {
        return (*root).sw;
    }
    if node_contains_((*root).se, point) != 0 {
        return (*root).se;
    }
    return 0 as *mut quadtree_node_t;
}
unsafe extern "C" fn split_node_(
    mut tree: *mut quadtree_t,
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    let mut nw = 0 as *mut quadtree_node_t;
    let mut ne = 0 as *mut quadtree_node_t;
    let mut sw = 0 as *mut quadtree_node_t;
    let mut se = 0 as *mut quadtree_node_t;
    let mut x = (*(*(*node).bounds).nw.as_deref().unwrap()).x;
    let mut y = (*(*(*node).bounds).nw.as_deref().unwrap()).y;
    let mut hw = (*(*node).bounds).width / 2 as libc::c_int as libc::c_double;
    let mut hh = (*(*node).bounds).height / 2 as libc::c_int as libc::c_double;
    nw= crate::src::src::node::quadtree_node_with_bounds(x, y - hh, x + hw, y);
    if nw.is_null() {();
        return 0 as libc::c_int;
    }
    ne= crate::src::src::node::quadtree_node_with_bounds(
        x + hw,
        y - hh,
        x + hw * 2 as libc::c_int as libc::c_double,
        y,
    );
    if ne.is_null() {();
        return 0 as libc::c_int;
    }
    sw= crate::src::src::node::quadtree_node_with_bounds(
        x,
        y - hh * 2 as libc::c_int as libc::c_double,
        x + hw,
        y - hh,
    );
    if sw.is_null() {();
        return 0 as libc::c_int;
    }
    se= crate::src::src::node::quadtree_node_with_bounds(
        x + hw,
        y - hh * 2 as libc::c_int as libc::c_double,
        x + hw * 2 as libc::c_int as libc::c_double,
        y - hh,
    );
    if se.is_null() {();
        return 0 as libc::c_int;
    }
    (*node).nw= nw;
    (*node).ne= ne;
    (*node).sw= sw;
    (*node).se= se;
    let mut old = (*node).point;
    let mut key = (*node).key;
    (*node).point= 0 as *mut quadtree_point_t;
    (*node).key= 0 as *mut libc::c_void;
    return insert_(tree, node, old, key);
}
unsafe extern "C" fn find_(
    mut node: Option<&mut quadtree_node_t>,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut quadtree_point_t {
    if crate::src::src::node::quadtree_node_isleaf(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
        if (*(*node.as_deref().unwrap()).point).x == x && (*(*node.as_deref().unwrap()).point).y == y {
            return (*node.as_deref().unwrap()).point;
        }
    } else {
        let mut test = quadtree_point_t { x: 0., y: 0. };
        test.x= x;
        test.y= y;
        return find_(get_quadrant_(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of_mut!(test)).as_mut(), x, y);
    }
    return 0 as *mut quadtree_point_t;
}
unsafe extern "C" fn insert_(
    mut tree: *mut quadtree_t,
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    if crate::src::src::node::quadtree_node_isempty(root) != 0 {
        (*root).point= point;
        (*root).key= key;
        return 1 as libc::c_int;
    } else {
        if crate::src::src::node::quadtree_node_isleaf(root) != 0 {
            if (*(*root).point).x == (*point).x && (*(*root).point).y == (*point).y {
                reset_node_(tree, root.as_mut());
                (*root).point= point;
                (*root).key= key;
                return 0 as libc::c_int;
            } else {
                if split_node_(tree, root) == 0 {
                    return 0 as libc::c_int;
                }
                return insert_(tree, root, point, key);
            }
        } else {
            if crate::src::src::node::quadtree_node_ispointer(root) != 0 {
                let mut quadrant = get_quadrant_(root, point);
                return if quadrant.is_null() {();
                    0 as libc::c_int
                } else {
                    insert_(tree, quadrant, point, key)
                };
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_new(
    mut minx: libc::c_double,
    mut miny: libc::c_double,
    mut maxx: libc::c_double,
    mut maxy: libc::c_double,
) -> Option<Box<quadtree_t>> {
    let mut tree = None;
    tree= Some(Box::new(<crate::src::src::quadtree::quadtree as Default>::default()));
    if tree.as_deref().is_none() {();
        return None;
    }
    (*tree.as_deref_mut().unwrap()).root= Some(Box::from_raw(crate::src::src::node::quadtree_node_with_bounds(minx, miny, maxx, maxy)));
    if (*tree.as_deref_mut().unwrap()).root.as_deref().is_none() {();
        ();
        return None;
    }
    (*tree.as_deref_mut().unwrap()).key_free= None;
    (*tree.as_deref_mut().unwrap()).length= 0 as libc::c_int as libc::c_uint;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_insert(
    mut tree: *mut quadtree_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    let mut point = 0 as *mut quadtree_point_t;
    point= crate::src::src::point::quadtree_point_new(x, y).map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if point.is_null() {();
        return 0 as libc::c_int;
    }
    if node_contains_((*tree).root.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), point) == 0 {
        return 0 as libc::c_int;
    }
    if insert_(tree, (*tree).root.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), point, key) == 0 {
        return 0 as libc::c_int;
    }
    (*tree).length= (*tree).length.wrapping_add(1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_search(
    mut tree: Option<&mut quadtree_t>,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut quadtree_point_t {
    return find_((*tree.as_deref_mut().unwrap()).root.as_deref_mut(), x, y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_free(mut tree: Option<Box<quadtree_t>>) {
    if (*tree.as_deref().unwrap()).key_free.is_some() {
        crate::src::src::node::quadtree_node_free((*tree.as_deref_mut().unwrap()).root.take(), (*tree.as_deref().unwrap()).key_free);
    } else {
        crate::src::src::node::quadtree_node_free(
            (*tree.as_deref_mut().unwrap()).root.take(),
            Some(elision_ as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    ();
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_walk(
    mut root: *mut quadtree_node_t,
    mut descent: Option::<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
    mut ascent: Option::<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
) {
    (Some(descent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
    if !(*root).nw.is_null() {
        quadtree_walk((*root).nw, descent, ascent);
    }else { (); }
    if !(*root).ne.is_null() {
        quadtree_walk((*root).ne, descent, ascent);
    }else { (); }
    if !(*root).sw.is_null() {
        quadtree_walk((*root).sw, descent, ascent);
    }else { (); }
    if !(*root).se.is_null() {
        quadtree_walk((*root).se, descent, ascent);
    }else { (); }
    (Some(ascent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
}

}

} // mod src
pub mod test {
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
impl Default for ErasedByPreprocessor6 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
impl Default for ErasedByPreprocessor7 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8 { dummy: () }
impl Default for ErasedByPreprocessor8 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9 { dummy: () }
impl Default for ErasedByPreprocessor9 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_t = crate::src::src::quadtree::quadtree;
#[no_mangle]
pub unsafe extern "C" fn descent(mut node: *mut quadtree_node_t) {
    if !(*node).bounds.is_null() {
        printf(
            b"{ nw.x:%f, nw.y:%f, se.x:%f, se.y:%f }: \0" as *const u8
                as *const libc::c_char,
            (*(*(*node).bounds).nw.as_deref().unwrap()).x,
            (*(*(*node).bounds).nw.as_deref().unwrap()).y,
            (*(*(*node).bounds).se.as_deref().unwrap()).x,
            (*(*(*node).bounds).se.as_deref().unwrap()).y,
        );
    }else { (); }
}
#[no_mangle]
pub unsafe extern "C" fn ascent(mut node: *mut quadtree_node_t) {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn test_node() {
    let mut node = crate::src::src::node::quadtree_node_new().map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if crate::src::src::node::quadtree_node_isleaf(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_isleaf(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::node::quadtree_node_isempty(node) != 0 {} else {
        __assert_fail(
            b"quadtree_node_isempty(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::node::quadtree_node_ispointer(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_ispointer(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn test_bounds() {
    let mut bounds = crate::src::src::bounds::quadtree_bounds_new();
    if !bounds.as_deref().is_none() {} else {();
        __assert_fail(
            b"bounds\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).nw.as_deref().unwrap()).x == ::std::f32::INFINITY as libc::c_double {} else {
        __assert_fail(
            b"bounds->nw->x == INFINITY\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).se.as_deref().unwrap()).x == -::std::f32::INFINITY as libc::c_double {} else {
        __assert_fail(
            b"bounds->se->x == -INFINITY\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    crate::src::src::bounds::quadtree_bounds_extend(bounds.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), 5.0f64, 5.0f64);
    if (*(*bounds.as_deref().unwrap()).nw.as_deref().unwrap()).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->nw->x == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).se.as_deref().unwrap()).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->x == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    crate::src::src::bounds::quadtree_bounds_extend(bounds.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), 10.0f64, 10.0f64);
    if (*(*bounds.as_deref().unwrap()).nw.as_deref().unwrap()).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).nw.as_deref().unwrap()).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).se.as_deref().unwrap()).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*bounds.as_deref().unwrap()).se.as_deref().unwrap()).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*bounds.as_deref().unwrap()).width == 5.0f64 {} else {
        __assert_fail(
            b"bounds->width == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*bounds.as_deref().unwrap()).height == 5.0f64 {} else {
        __assert_fail(
            b"bounds->height == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const libc::c_char,
        );
    };
    crate::src::src::bounds::quadtree_bounds_free(bounds);
}
unsafe extern "C" fn test_tree() {
    let mut val = 10 as libc::c_int;
    let mut tree = crate::src::src::quadtree::quadtree_new(
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    if (*(*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).bounds).nw.as_deref().unwrap()).x == 1 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->nw->x == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).bounds).nw.as_deref().unwrap()).y == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).bounds).se.as_deref().unwrap()).x == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->se->x == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).bounds).se.as_deref().unwrap()).y == 1 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->se->y == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 0, 0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        10 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 10, 10, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        110.0f64,
        110.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 110.0, 110.0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        8.0f64,
        2.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 8.0, 2.0, &val) != 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*tree.as_deref().unwrap()).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).point).x == 8.0f64 {} else {
        __assert_fail(
            b"tree->root->point->x == 8.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).point).y == 2.0f64 {} else {
        __assert_fail(
            b"tree->root->point->y == 2.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        2.0f64,
        3.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) != 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        2.0f64,
        3.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*tree.as_deref().unwrap()).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 2\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*(*tree.as_deref().unwrap()).root.as_deref().unwrap()).point.is_null() {();} else {
        __assert_fail(
            b"tree->root->point == NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        3.0f64,
        1.1f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 3.0, 1.1, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*tree.as_deref().unwrap()).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 3\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*crate::src::src::quadtree::quadtree_search(tree.as_deref_mut(), 3.0f64, 1.1f64)).x == 3.0f64 {} else {
        __assert_fail(
            b"quadtree_search(tree, 3.0, 1.1)->x == 3.0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const libc::c_char,
        );
    };
    crate::src::src::quadtree::quadtree_walk(
        (*tree.as_deref_mut().unwrap()).root.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        Some(ascent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
        Some(descent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
    );
    crate::src::src::quadtree::quadtree_free(tree);
}
unsafe extern "C" fn test_points() {
    let mut point = crate::src::src::point::quadtree_point_new(
        5 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    );
    if (*point.as_deref().unwrap()).x == 5 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"point->x == 5\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            b"void test_points()\0" as *const u8 as *const libc::c_char,
        );
    };
    if (*point.as_deref().unwrap()).y == 6 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"point->y == 6\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            b"void test_points()\0" as *const u8 as *const libc::c_char,
        );
    };
    crate::src::src::point::quadtree_point_free(point);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    printf(
        b"\nquadtree_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_node_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_node_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_bounds_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_bounds_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_point_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_point_t>() as libc::c_ulong,
    );
    printf(b"\x1B[33mtree\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_tree();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mnode\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_node();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mbounds\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_bounds();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mpoints\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_points();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    return 0;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *const libc::c_char,
//             ) as i32,
//         )
//     }
// }

}

} // mod src

// root re-exports so the generated harness's `translated::<entry>` resolves
// elision_: private in src/quadtree — exposed per-entry by --expose-entry
// find_: private in src/quadtree — exposed per-entry by --expose-entry
// get_quadrant_: private in src/quadtree — exposed per-entry by --expose-entry
// insert_: private in src/quadtree — exposed per-entry by --expose-entry
// node_contains_: private in src/quadtree — exposed per-entry by --expose-entry
pub use crate::src::src::bounds::quadtree_bounds_extend;
pub use crate::src::src::bounds::quadtree_bounds_free;
pub use crate::src::src::bounds::quadtree_bounds_new;
pub use crate::src::src::quadtree::quadtree_free;
pub use crate::src::src::quadtree::quadtree_insert;
pub use crate::src::src::quadtree::quadtree_new;
pub use crate::src::src::node::quadtree_node_free;
pub use crate::src::src::node::quadtree_node_isempty;
pub use crate::src::src::node::quadtree_node_isleaf;
pub use crate::src::src::node::quadtree_node_ispointer;
pub use crate::src::src::node::quadtree_node_new;
pub use crate::src::src::node::quadtree_node_reset;
pub use crate::src::src::node::quadtree_node_with_bounds;
pub use crate::src::src::point::quadtree_point_free;
pub use crate::src::src::point::quadtree_point_new;
pub use crate::src::src::quadtree::quadtree_search;
pub use crate::src::src::quadtree::quadtree_walk;
// reset_node_: private in src/quadtree — exposed per-entry by --expose-entry
// split_node_: private in src/quadtree — exposed per-entry by --expose-entry
