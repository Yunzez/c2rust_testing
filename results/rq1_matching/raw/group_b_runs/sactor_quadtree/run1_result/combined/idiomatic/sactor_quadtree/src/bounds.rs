#![allow(unused_imports, unused_variables, dead_code)]

use crate::point::quadtree_point_free;
use crate::point::quadtree_point_new;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
impl quadtree_point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
