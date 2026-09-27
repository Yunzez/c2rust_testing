pub unsafe fn quadtree_bounds_free(bounds: *mut quadtree_bounds) {
    extern "C" {
        fn free(ptr: *mut libc::c_void);
    }
    if bounds.is_null() {
        return;
    }
    quadtree_point_free((*bounds).nw);
    quadtree_point_free((*bounds).se);
    free(bounds as *mut libc::c_void);
}
