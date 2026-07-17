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