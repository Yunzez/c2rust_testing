pub unsafe fn quadtree_node_free(
    node: *mut quadtree_node,
    key_free: unsafe extern "C" fn(*mut ::core::ffi::c_void),
) {
    if node.is_null() {
        return;
    }
    if !(*node).nw.is_null() {
        quadtree_node_free((*node).nw, key_free);
    }
    if !(*node).ne.is_null() {
        quadtree_node_free((*node).ne, key_free);
    }
    if !(*node).sw.is_null() {
        quadtree_node_free((*node).sw, key_free);
    }
    if !(*node).se.is_null() {
        quadtree_node_free((*node).se, key_free);
    }
    quadtree_bounds_free((*node).bounds);
    quadtree_node_reset(node, key_free);
    libc::free(node.cast());
}
