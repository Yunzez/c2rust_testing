pub unsafe fn quadtree_node_ispointer(node: *mut quadtree_node) -> ::core::ffi::c_int {
    if !(*node).nw.is_null()
        && !(*node).ne.is_null()
        && !(*node).sw.is_null()
        && !(*node).se.is_null()
        && quadtree_node_isleaf(node) == 0
    {
        1
    } else {
        0
    }
}
