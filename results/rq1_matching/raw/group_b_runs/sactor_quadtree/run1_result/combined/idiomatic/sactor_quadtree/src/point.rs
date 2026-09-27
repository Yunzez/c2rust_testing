#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
pub fn quadtree_point_free(_point: Box<quadtree_point>) {}
pub fn quadtree_point_new(x: f64, y: f64) -> quadtree_point {
    quadtree_point { x, y }
}
