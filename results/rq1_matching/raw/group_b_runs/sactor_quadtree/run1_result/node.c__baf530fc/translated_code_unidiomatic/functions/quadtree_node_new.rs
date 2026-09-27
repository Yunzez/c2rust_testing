pub unsafe fn quadtree_node_new() -> *mut quadtree_node {
    use core::mem;
    use libc::malloc;
    let node = malloc(mem::size_of::<quadtree_node>()) as *mut quadtree_node;
    if node.is_null() {
        return core::ptr::null_mut();
    }
    (*node).ne = core::ptr::null_mut();
    (*node).nw = core::ptr::null_mut();
    (*node).se = core::ptr::null_mut();
    (*node).sw = core::ptr::null_mut();
    (*node).point = core::ptr::null_mut();
    (*node).bounds = core::ptr::null_mut();
    (*node).key = core::ptr::null_mut();
    node
}
