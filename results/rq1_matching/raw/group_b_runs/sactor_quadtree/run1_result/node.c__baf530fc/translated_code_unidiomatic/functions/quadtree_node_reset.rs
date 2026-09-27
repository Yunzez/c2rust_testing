pub unsafe fn quadtree_node_reset(
    node: *mut quadtree_node,
    key_free: unsafe extern "C" fn(*mut ::core::ffi::c_void),
) {
    quadtree_point_free((*node).point);
    key_free((*node).key);
}
