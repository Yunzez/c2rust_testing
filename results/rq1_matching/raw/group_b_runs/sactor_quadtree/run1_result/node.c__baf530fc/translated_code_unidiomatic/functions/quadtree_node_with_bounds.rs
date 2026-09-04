pub unsafe fn quadtree_node_with_bounds(
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
) -> *mut quadtree_node {
    let mut node: *mut quadtree_node;
    node = quadtree_node_new();
    if node.is_null() {
        return core::ptr::null_mut();
    }
    (*node).bounds = quadtree_bounds_new();
    if (*node).bounds.is_null() {
        return core::ptr::null_mut();
    }
    quadtree_bounds_extend((*node).bounds, maxx, maxy);
    quadtree_bounds_extend((*node).bounds, minx, miny);
    node
}
