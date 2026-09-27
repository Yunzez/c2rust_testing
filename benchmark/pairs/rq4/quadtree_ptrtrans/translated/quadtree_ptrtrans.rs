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

pub mod bounds {
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
}

pub mod node {
use crate::*;
pub fn quadtree_node_new() -> Option<Box<QuadtreeNode>> {
    let node = Box::new(QuadtreeNode {
        ne: None,
        nw: None,
        se: None,
        sw: None,
        bounds: None,
        point: None,
        key: None,
    });
    Some(node)
}
pub fn quadtree_node_isleaf(node: Option<&QuadtreeNode>) -> bool {
    match node {
        Some(n) => n.point.is_some(),
        None => false,
    }
}
pub fn quadtree_node_with_bounds(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Option<Box<QuadtreeNode>> {
    // Attempt to create a new QuadtreeNode
    let mut node = quadtree_node_new()?;
    // Attempt to create new bounds for the node
    let mut bounds = quadtree_bounds_new()?;
    // Extend the bounds with the provided coordinates
    quadtree_bounds_extend(Some(&mut bounds), maxx, maxy);
    quadtree_bounds_extend(Some(&mut bounds), minx, miny);
    // Assign the bounds to the node
    node.bounds = Some(bounds);
    // Return the constructed node
    Some(node)
}
pub fn quadtree_node_ispointer(node: Option<&QuadtreeNode>) -> bool {
    if let Some(node) = node {
        node.nw.is_some()
            && node.ne.is_some()
            && node.sw.is_some()
            && node.se.is_some()
            && !quadtree_node_isleaf(Some(node))
    } else {
        false
    }
}
pub fn quadtree_node_isempty(node: Option<&QuadtreeNode>) -> bool {
    match node {
        Some(n) => {
            n.nw.is_none()
                && n.ne.is_none()
                && n.sw.is_none()
                && n.se.is_none()
                && !quadtree_node_isleaf(Some(n))
        }
        None => false, // If the node is None (null), it cannot be empty.
    }
}
}

pub mod point {
use std::boxed::Box;
use crate::*;
pub fn quadtree_point_new(x: f64, y: f64) -> Option<Box<QuadtreePoint>> {
    let point = Box::new(QuadtreePoint { x, y });
    Some(point)
}
}

pub mod quadtree {
use crate::*;
use core::any::Any;
pub fn quadtree_walk(
    root: Option<&QuadtreeNode>, 
    descent: Option<&dyn Fn(&QuadtreeNode)>, 
    ascent: Option<&dyn Fn(&QuadtreeNode)>
) {
    if let Some(node) = root {
        if let Some(descent_fn) = descent {
            descent_fn(node);
        }
        if let Some(nw) = node.nw.as_deref() {
            quadtree_walk(Some(nw), descent, ascent);
        }
        if let Some(ne) = node.ne.as_deref() {
            quadtree_walk(Some(ne), descent, ascent);
        }
        if let Some(sw) = node.sw.as_deref() {
            quadtree_walk(Some(sw), descent, ascent);
        }
        if let Some(se) = node.se.as_deref() {
            quadtree_walk(Some(se), descent, ascent);
        }
        if let Some(ascent_fn) = ascent {
            ascent_fn(node);
        }
    }
}
pub fn node_contains_(outer: Option<&QuadtreeNode>, it: Option<&QuadtreePoint>) -> bool {
    if let (Some(outer_node), Some(it_point)) = (outer, it) {
        if let Some(bounds) = &outer_node.bounds {
            if let (Some(nw), Some(se)) = (&bounds.nw, &bounds.se) {
                return nw.x < it_point.x && nw.y > it_point.y && se.x > it_point.x && se.y < it_point.y;
            }
        }
    }
    false
}
pub fn get_quadrant_<'a>(
    root: Option<&'a mut QuadtreeNode>, 
    point: Option<&QuadtreePoint>
) -> Option<&'a mut QuadtreeNode> {
    if let Some(root_node) = root {
        if node_contains_(root_node.nw.as_deref(), point) {
            return root_node.nw.as_deref_mut();
        }
        if node_contains_(root_node.ne.as_deref(), point) {
            return root_node.ne.as_deref_mut();
        }
        if node_contains_(root_node.sw.as_deref(), point) {
            return root_node.sw.as_deref_mut();
        }
        if node_contains_(root_node.se.as_deref(), point) {
            return root_node.se.as_deref_mut();
        }
    }
    None
}
pub fn quadtree_new<'a>(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Option<Box<Quadtree<'a>>> {
    // Attempt to create a new Quadtree instance
    let mut tree = Box::new(Quadtree {
        root: None, // Initialize root as None
        key_free: None, // Initialize key_free as None
        length: 0, // Initialize length to 0
    });
    // Attempt to create a root node with the given bounds
    if let Some(root_node) = quadtree_node_with_bounds(minx, miny, maxx, maxy) {
        tree.root = Some(root_node); // Assign the root node
        Some(tree) // Return the constructed Quadtree wrapped in Some
    } else {
        None // Return None if root node creation failed
    }
}
pub fn find_<'a>(
    node: Option<&'a QuadtreeNode>, 
    x: f64, 
    y: f64
) -> Option<&'a QuadtreePoint> {
    if let Some(node_ref) = node {
        if quadtree_node_isleaf(Some(node_ref)) {
            if let Some(point) = &node_ref.point {
                if point.x == x && point.y == y {
                    return Some(point);
                }
            }
        } else {
            let test = QuadtreePoint { x, y };
            if let Some(next_node) = get_quadrant_(None, Some(&test)) {
                return find_(Some(next_node), x, y);
            }
        }
    }
    None
}
pub fn insert_(
    key_free: Option<& dyn FnMut(&dyn std::any::Any)>,
    root: Option<&mut QuadtreeNode>, // Nullable, Borrowed and Mutable pointer
    point: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    key: Option<Box<dyn std::any::Any>>, // Nullable, Owning pointer
) -> bool {
    if let Some(root) = root {
        if quadtree_node_isempty(Some(root)) {
            root.point = point;
            root.key = key;
            return true;
        } else if quadtree_node_isleaf(Some(root)) {
            if let Some(existing_point) = &root.point {
                if let Some(new_point) = &point {
                    if existing_point.x == new_point.x && existing_point.y == new_point.y {
                        root.point = point;
                        root.key = key;
                        return false;
                    }
                }
            }
            if !split_node_(key_free, Some(root)) {
                return false;
            }
            return insert_(key_free, Some(root), point, key);
        } else if quadtree_node_ispointer(Some(root)) {
            // Convert `Option<&Box<QuadtreePoint>>` to `Option<&QuadtreePoint>`
            let point_ref = point.as_ref().map(|b| b.as_ref());
            if let Some(quadrant) = get_quadrant_(Some(root), point_ref) {
                return insert_(key_free, Some(quadrant), point, key);
            }
            return false;
        }
    }
    false
}
pub fn split_node_<'a>(
    key_free: Option<&'a dyn FnMut(&dyn std::any::Any)>,  // Nullable, Borrowed and Immutable pointer
    node: Option<&'a mut QuadtreeNode>, // Nullable, Borrowed and Mutable pointer
) -> bool { // Changed return type from `i32` to `bool`
    if let Some(node) = node {
        let bounds = node.bounds.as_ref();
        if let Some(bounds) = bounds {
            let x = bounds.nw.as_ref().map_or(0.0, |nw| nw.x);
            let y = bounds.nw.as_ref().map_or(0.0, |nw| nw.y);
            let hw = bounds.width / 2.0;
            let hh = bounds.height / 2.0;
            let nw = quadtree_node_with_bounds(x, y - hh, x + hw, y);
            let ne = quadtree_node_with_bounds(x + hw, y - hh, x + hw * 2.0, y);
            let sw = quadtree_node_with_bounds(x, y - hh * 2.0, x + hw, y - hh);
            let se = quadtree_node_with_bounds(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);
            if nw.is_none() || ne.is_none() || sw.is_none() || se.is_none() {
                return false; // Changed return value from `0` to `false`
            }
            node.nw = nw;
            node.ne = ne;
            node.sw = sw;
            node.se = se;
            let old_point = node.point.take();
            let old_key = node.key.take();
            return insert_(key_free, Some(node), old_point, old_key);
        }
    }
    false // Changed return value from `0` to `false`
}
pub fn quadtree_search<'a>(tree: Option<&'a Quadtree<'a>>, x: f64, y: f64) -> Option<&'a QuadtreePoint> {
    find_(tree.and_then(|t| t.root.as_deref()), x, y)
}
pub fn quadtree_insert(
    tree: Option<&mut Quadtree>, // Nullable, Borrowed and Mutable pointer
    x: f64, 
    y: f64, 
    key: Option<Box<dyn Any>>,
) -> i32 {
    if let Some(tree) = tree {
        let point = Box::new(QuadtreePoint { x, y });
        if !node_contains_(tree.root.as_deref(), Some(&point)) {
            return 0;
        }
        if !insert_(tree.key_free, tree.root.as_deref_mut(), Some(point), key) {
            return 0;
        }
        tree.length += 1;
        
        return 1;
    }
    0
}
}

pub mod common {
pub mod quadtree_mod {
/// Represents a point in a quadtree with x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadtreePoint {
    pub x: f64,
    pub y: f64,
}
pub struct QuadtreeBounds {
    pub nw: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub se: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub width: f64,
    pub height: f64,
}
pub struct QuadtreeNode {
    pub ne: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub nw: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub se: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub sw: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub bounds: Option<Box<QuadtreeBounds>>, // Nullable, Owning pointer
    pub point: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub key: Option<Box<dyn std::any::Any>>, // Nullable, Owning pointer
}
pub struct Quadtree<'a> {
    pub root: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub key_free: Option<&'a dyn FnMut(&dyn std::any::Any)>,// Nullable, Borrowed and Mutable pointer with Lifetime Annotation
    pub length: u32, // Unsigned integer
}
}

} // mod common
// root re-exports so the generated harness's `translated::<entry>` resolves
pub use crate::quadtree::find_;
pub use crate::quadtree::get_quadrant_;
pub use crate::quadtree::insert_;
pub use crate::quadtree::node_contains_;
pub use crate::bounds::quadtree_bounds_extend;
pub use crate::bounds::quadtree_bounds_new;
pub use crate::quadtree::quadtree_insert;
pub use crate::quadtree::quadtree_new;
pub use crate::node::quadtree_node_isempty;
pub use crate::node::quadtree_node_isleaf;
pub use crate::node::quadtree_node_ispointer;
pub use crate::node::quadtree_node_new;
pub use crate::node::quadtree_node_with_bounds;
pub use crate::point::quadtree_point_new;
pub use crate::quadtree::quadtree_search;
pub use crate::quadtree::quadtree_walk;
pub use crate::quadtree::split_node_;
// support modules carried verbatim from the translation's own crate root
pub use crate::node::*;
pub use crate::bounds::*;
pub use crate::point::*;
pub use crate::quadtree::*;
pub use crate::common::quadtree_mod::*;

