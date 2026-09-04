pub unsafe fn quadtree_bounds_extend(bounds: *mut quadtree_bounds, x: f64, y: f64) {
    use core::cmp::Ordering;
    fn fmin(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }
    fn fmax(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }
    fn fabs(a: f64) -> f64 {
        a.abs()
    }
    if bounds.is_null() {
        return;
    }
    let b = &mut *bounds;
    let nw = &mut *b.nw;
    let se = &mut *b.se;
    nw.x = fmin(x, nw.x);
    nw.y = fmax(y, nw.y);
    se.x = fmax(x, se.x);
    se.y = fmin(y, se.y);
    b.width = fabs(nw.x - se.x);
    b.height = fabs(nw.y - se.y);
}
