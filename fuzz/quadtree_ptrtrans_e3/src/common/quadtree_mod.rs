/// Represents a point in a quadtree with x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadtreePoint {
    pub x: f64,
    pub y: f64,
}
pub struct QuadtreeBounds {
    pub nw: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub se: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub width: f64,
    pub height: f64,
}
pub struct QuadtreeNode {
    pub ne: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub nw: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub se: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub sw: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub bounds: Option<Box<QuadtreeBounds>>, // Nullable, Owning pointer
    pub point: Option<Box<QuadtreePoint>>, // Nullable, Owning pointer
    pub key: Option<Box<dyn std::any::Any>>, // Nullable, Owning pointer
}
pub struct Quadtree<'a> {
    pub root: Option<Box<QuadtreeNode>>, // Nullable, Owning pointer
    pub key_free: Option<&'a dyn FnMut(&dyn std::any::Any)>,// Nullable, Borrowed and Mutable pointer with Lifetime Annotation
    pub length: u32, // Unsigned integer
}