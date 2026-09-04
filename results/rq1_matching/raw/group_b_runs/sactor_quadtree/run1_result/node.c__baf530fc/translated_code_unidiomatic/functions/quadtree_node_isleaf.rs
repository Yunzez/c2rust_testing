pub unsafe fn quadtree_node_isleaf(node: *mut quadtree_node) -> ::core::ffi::c_int {
    ((*node).point != core::ptr::null_mut()) as ::core::ffi::c_int
}
