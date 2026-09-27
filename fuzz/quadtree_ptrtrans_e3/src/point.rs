use std::boxed::Box;
use crate::*;
pub fn quadtree_point_new(x: f64, y: f64) -> Option<Box<QuadtreePoint>> {
    let point = Box::new(QuadtreePoint { x, y });
    Some(point)
}