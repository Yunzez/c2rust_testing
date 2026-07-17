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
