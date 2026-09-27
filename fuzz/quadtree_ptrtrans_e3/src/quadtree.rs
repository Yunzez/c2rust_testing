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