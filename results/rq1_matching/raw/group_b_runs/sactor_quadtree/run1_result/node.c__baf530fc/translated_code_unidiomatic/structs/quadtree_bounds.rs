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
