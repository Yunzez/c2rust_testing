pub unsafe fn quadtree_point_new(x: f64, y: f64) -> *mut quadtree_point {
    let point = libc::malloc(::core::mem::size_of::<quadtree_point>()) as *mut quadtree_point;
    if point.is_null() {
        return ::core::ptr::null_mut();
    }
    (*point).x = x;
    (*point).y = y;
    point
}
