pub unsafe fn quadtree_bounds_new() -> *mut quadtree_bounds {
    type Bounds = quadtree_bounds;
    let bounds: *mut Bounds = libc::malloc(::core::mem::size_of::<Bounds>()) as *mut Bounds;
    if bounds.is_null() {
        return core::ptr::null_mut();
    }
    (*bounds).nw = quadtree_point_new(1e308f64, -1e308f64);
    (*bounds).se = quadtree_point_new(-1e308f64, 1e308f64);
    (*bounds).width = 0.0f64;
    (*bounds).height = 0.0f64;
    bounds
}
