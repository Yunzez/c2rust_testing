#[derive(Clone, Copy, Debug, PartialEq)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
impl quadtree_point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
