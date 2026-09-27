use std::f64;
use crate::*;
pub fn quadtree_bounds_new() -> Option<Box<QuadtreeBounds>> {
    let mut bounds = Box::new(QuadtreeBounds {
        nw: None,
        se: None,
        width: 0.0,
        height: 0.0,
    });
    bounds.nw = quadtree_point_new(f64::INFINITY, -f64::INFINITY);
    bounds.se = quadtree_point_new(-f64::INFINITY, f64::INFINITY);
    Some(bounds)
}
pub fn quadtree_bounds_extend(bounds: Option<&mut QuadtreeBounds>, x: f64, y: f64) {
    if let Some(bounds) = bounds {
        if let Some(nw) = bounds.nw.as_mut() {
            nw.x = f64::min(x, nw.x);
            nw.y = f64::max(y, nw.y);
        }
        if let Some(se) = bounds.se.as_mut() {
            se.x = f64::max(x, se.x);
            se.y = f64::min(y, se.y);
        }
        if let (Some(nw), Some(se)) = (bounds.nw.as_ref(), bounds.se.as_ref()) {
            bounds.width = (nw.x - se.x).abs();
            bounds.height = (nw.y - se.y).abs();
        }
    }
}