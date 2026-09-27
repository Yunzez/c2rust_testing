#[derive(Copy, Clone, Debug, PartialEq)]
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

use core::ptr;
use std::ffi;

unsafe fn Cquadtree_point_to_quadtree_point_mut(
    input: *mut Cquadtree_point,
) -> &'static mut quadtree_point {
    assert!(!input.is_null());
    let c_struct = &*input;
    let idiom_struct = quadtree_point {
        // Field 'x' -> 'x' (C -> idiomatic)
        x: c_struct.x,
        // Field 'y' -> 'y' (C -> idiomatic)
        y: c_struct.y,
    };
    Box::leak(Box::new(idiom_struct))
}

unsafe fn quadtree_point_to_Cquadtree_point_mut(
    idiom_struct: &mut quadtree_point,
) -> *mut Cquadtree_point {
    // Field 'x' -> 'x' (idiomatic -> C)
    let _x = idiom_struct.x;
    // Field 'y' -> 'y' (idiomatic -> C)
    let _y = idiom_struct.y;

    let c_struct = Cquadtree_point { x: _x, y: _y };
    Box::into_raw(Box::new(c_struct))
}
