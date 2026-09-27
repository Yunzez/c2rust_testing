pub unsafe fn quadtree_point_free(point: *mut quadtree_point) {
    libc::free(point.cast());
}
