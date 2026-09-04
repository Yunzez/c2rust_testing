use core::ptr;
use std::ffi;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
pub type size_t = usize;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Cquadtree_point {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
unsafe fn Cquadtree_point_to_quadtree_point_mut(
    input: *mut Cquadtree_point,
) -> &'static mut quadtree_point {
    assert!(!input.is_null());
    let c_struct = &*input;
    let idiom_struct = quadtree_point {
        x: c_struct.x,
        y: c_struct.y,
    };
    Box::leak(Box::new(idiom_struct))
}
unsafe fn quadtree_point_to_Cquadtree_point_mut(
    idiom_struct: &mut quadtree_point,
) -> *mut Cquadtree_point {
    let _x = idiom_struct.x;
    let _y = idiom_struct.y;
    let c_struct = Cquadtree_point { x: _x, y: _y };
    Box::into_raw(Box::new(c_struct))
}
pub fn quadtree_point_free_idiomatic(_point: Box<quadtree_point>) {}
fn quadtree_point_free(point: *mut Cquadtree_point) {
    if point.is_null() {
        return;
    }
    unsafe {
        let idiom_ref: &mut quadtree_point = Cquadtree_point_to_quadtree_point_mut(point);
        let boxed_point: Box<quadtree_point> = Box::from_raw(idiom_ref as *mut quadtree_point);
        quadtree_point_free_idiomatic(boxed_point);
        drop(Box::from_raw(point));
    }
}
