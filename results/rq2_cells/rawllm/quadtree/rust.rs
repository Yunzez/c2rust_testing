use std::f64::{INFINITY, NEG_INFINITY};

/// Version string equivalent to `QUADTREE_VERSION` in C.
pub const QUADTREE_VERSION: &str = "0.1.0";

/// 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadPoint {
    pub x: f64,
    pub y: f64,
}

impl QuadPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Bounding box of a quadtree node.
#[derive(Clone, Debug)]
pub struct QuadBounds {
    pub nw: QuadPoint,
    pub se: QuadPoint,
    pub width: f64,
    pub height: f64,
}

impl QuadBounds {
    /// Create bounds equivalent to `quadtree_bounds_new` (infinite, will be
    /// tightened via `extend`).
    pub fn new() -> Self {
        let nw = QuadPoint::new(INFINITY, NEG_INFINITY);
        let se = QuadPoint::new(NEG_INFINITY, INFINITY);
        Self {
            nw,
            se,
            width: 0.0,
            height: 0.0,
        }
    }

    /// Extend bounds to include the given point (equivalent to
    /// `quadtree_bounds_extend`).
    pub fn extend(&mut self, x: f64, y: f64) {
        use std::cmp::Ordering;

        // nw.x = fmin(x, nw.x)
        self.nw.x = match x.partial_cmp(&self.nw.x) {
            Some(Ordering::Less) => x,
            _ => self.nw.x,
        };
        // nw.y = fmax(y, nw.y)
        self.nw.y = match y.partial_cmp(&self.nw.y) {
            Some(Ordering::Greater) => y,
            _ => self.nw.y,
        };
        // se.x = fmax(x, se.x)
        self.se.x = match x.partial_cmp(&self.se.x) {
            Some(Ordering::Greater) => x,
            _ => self.se.x,
        };
        // se.y = fmin(y, se.y)
        self.se.y = match y.partial_cmp(&self.se.y) {
            Some(Ordering::Less) => y,
            _ => self.se.y,
        };

        self.width = (self.nw.x - self.se.x).abs();
        self.height = (self.nw.y - self.se.y).abs();
    }

    /// Convenience constructor equivalent to how C code sets up bounds via
    /// two `extend` calls.
    pub fn from_corners(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Self {
        let mut b = QuadBounds::new();
        b.extend(maxx, maxy);
        b.extend(minx, miny);
        b
    }

    /// Check if a point is strictly inside the bounds, matching `node_contains_`.
    fn contains_point(&self, p: &QuadPoint) -> bool {
        self.nw.x < p.x
            && self.nw.y > p.y
            && self.se.x > p.x
            && self.se.y < p.y
    }
}

/// A node in the quadtree.
#[derive(Debug)]
pub struct QuadNode<K> {
    pub ne: Option<Box<QuadNode<K>>>,
    pub nw: Option<Box<QuadNode<K>>>,
    pub se: Option<Box<QuadNode<K>>>,
    pub sw: Option<Box<QuadNode<K>>>,
    pub bounds: Option<QuadBounds>,
    pub point: Option<QuadPoint>,
    pub key: Option<K>,
}

impl<K> QuadNode<K> {
    /// Equivalent to `quadtree_node_new`.
    pub fn new() -> Self {
        Self {
            ne: None,
            nw: None,
            se: None,
            sw: None,
            bounds: None,
            point: None,
            key: None,
        }
    }

    /// Equivalent to `quadtree_node_with_bounds`.
    pub fn with_bounds(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Self {
        let mut node = QuadNode::new();
        node.bounds = Some(QuadBounds::from_corners(minx, miny, maxx, maxy));
        node
    }

    /// Equivalent to `quadtree_node_isleaf`.
    fn is_leaf(&self) -> bool {
        self.point.is_some()
    }

    /// Equivalent to `quadtree_node_isempty`.
    fn is_empty(&self) -> bool {
        self.nw.is_none()
            && self.ne.is_none()
            && self.sw.is_none()
            && self.se.is_none()
            && !self.is_leaf()
    }

    /// Equivalent to `quadtree_node_ispointer`.
    fn is_pointer(&self) -> bool {
        self.nw.is_some()
            && self.ne.is_some()
            && self.sw.is_some()
            && self.se.is_some()
            && !self.is_leaf()
    }

    /// Equivalent to `node_contains_` for a node.
    fn node_contains(&self, p: &QuadPoint) -> bool {
        match &self.bounds {
            Some(b) => b.contains_point(p),
            None => false,
        }
    }

    /// Get the child quadrant that contains the point, equivalent to
    /// `get_quadrant_`.
    fn get_quadrant_mut(&mut self, p: &QuadPoint) -> Option<&mut Box<QuadNode<K>>> {
        if let Some(nw) = self.nw.as_mut() {
            if nw.node_contains(p) {
                return Some(nw);
            }
        }
        if let Some(ne) = self.ne.as_mut() {
            if ne.node_contains(p) {
                return Some(ne);
            }
        }
        if let Some(sw) = self.sw.as_mut() {
            if sw.node_contains(p) {
                return Some(sw);
            }
        }
        if let Some(se) = self.se.as_mut() {
            if se.node_contains(p) {
                return Some(se);
            }
        }
        None
    }

    /// Immutable version for search.
    fn get_quadrant(&self, p: &QuadPoint) -> Option<&QuadNode<K>> {
        if let Some(nw) = self.nw.as_ref() {
            if nw.node_contains(p) {
                return Some(nw);
            }
        }
        if let Some(ne) = self.ne.as_ref() {
            if ne.node_contains(p) {
                return Some(ne);
            }
        }
        if let Some(sw) = self.sw.as_ref() {
            if sw.node_contains(p) {
                return Some(sw);
            }
        }
        if let Some(se) = self.se.as_ref() {
            if se.node_contains(p) {
                return Some(se);
            }
        }
        None
    }

    /// Equivalent to `split_node_` but purely in safe Rust; returns whether the
    /// old point was successfully reinserted.
    fn split_node(&mut self) -> bool {
        let bounds = match &self.bounds {
            Some(b) => b.clone(),
            None => return false,
        };

        let x = bounds.nw.x;
        let y = bounds.nw.y;
        let hw = bounds.width / 2.0;
        let hh = bounds.height / 2.0;

        let nw = QuadNode::with_bounds(x, y - hh, x + hw, y);
        let ne = QuadNode::with_bounds(x + hw, y - hh, x + hw * 2.0, y);
        let sw = QuadNode::with_bounds(x, y - hh * 2.0, x + hw, y - hh);
        let se = QuadNode::with_bounds(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);

        self.nw = Some(Box::new(nw));
        self.ne = Some(Box::new(ne));
        self.sw = Some(Box::new(sw));
        self.se = Some(Box::new(se));

        // Move the old point + key out and reinsert.
        let old_point = self.point.take();
        let old_key = self.key.take();

        if let (Some(p), Some(k)) = (old_point, old_key) {
            self.insert_here(p, k).0
        } else {
            true
        }
    }

    /// Recursive insert equivalent to `insert_` but working only within this subtree.
    /// Returns (success, replaced_old) where `replaced_old` is true if a point
    /// with the same coordinates existed and was replaced.
    fn insert_here(&mut self, point: QuadPoint, key: K) -> (bool, bool) {
        if self.is_empty() {
            self.point = Some(point);
            self.key = Some(key);
            return (true, false);
        } else if self.is_leaf() {
            if let Some(existing) = self.point {
                if existing.x == point.x && existing.y == point.y {
                    // Replace existing key.
                    self.point = Some(point);
                    self.key = Some(key);
                    return (true, true);
                } else {
                    if !self.split_node() {
                        return (false, false);
                    }
                    return self.insert_here(point, key);
                }
            }
            (false, false)
        } else if self.is_pointer() {
            if let Some(child) = self.get_quadrant_mut(&point) {
                return child.insert_here(point, key);
            }
            (false, false)
        } else {
            (false, false)
        }
    }

    /// Recursive search equivalent to `find_`.
    fn find_point(&self, x: f64, y: f64) -> Option<QuadPoint> {
        if self.is_leaf() {
            if let Some(p) = self.point {
                if p.x == x && p.y == y {
                    return Some(p);
                }
            }
            None
        } else {
            let test = QuadPoint::new(x, y);
            self.get_quadrant(&test).and_then(|q| q.find_point(x, y))
        }
    }
}

/// Quadtree structure.
#[derive(Debug)]
pub struct QuadTree<K> {
    pub root: QuadNode<K>,
    pub length: u32,
}

impl<K> QuadTree<K> {
    /// Equivalent to `quadtree_new` (without the raw callback).
    pub fn new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> Self {
        Self {
            root: QuadNode::with_bounds(minx, miny, maxx, maxy),
            length: 0,
        }
    }

    /// Equivalent to `quadtree_insert`.
    ///
    /// Returns true on success, false on failure (out of bounds or structural).
    pub fn insert(&mut self, x: f64, y: f64, key: K) -> bool {
        let p = QuadPoint::new(x, y);
        if !self.root.node_contains(&p) {
            return false;
        }
        let (ok, _replaced) = self.root.insert_here(p, key);
        if ok {
            self.length = self.length.saturating_add(1);
        }
        ok
    }

    /// Equivalent to `quadtree_search` (returns the point, not the key, as in C).
    pub fn search_point(&self, x: f64, y: f64) -> Option<QuadPoint> {
        self.root.find_point(x, y)
    }

    /// Walk the tree in depth-first order, invoking `descent` when entering
    /// a node and `ascent` when leaving it, equivalent to `quadtree_walk`.
    pub fn walk<F, G>(&self, mut descent: F, mut ascent: G)
    where
        F: FnMut(&QuadNode<K>),
        G: FnMut(&QuadNode<K>),
    {
        fn dfs<K, F, G>(node: &QuadNode<K>, descent: &mut F, ascent: &mut G)
        where
            F: FnMut(&QuadNode<K>),
            G: FnMut(&QuadNode<K>),
        {
            descent(node);
            if let Some(ref n) = node.nw {
                dfs(n, descent, ascent);
            }
            if let Some(ref n) = node.ne {
                dfs(n, descent, ascent);
            }
            if let Some(ref n) = node.sw {
                dfs(n, descent, ascent);
            }
            if let Some(ref n) = node.se {
                dfs(n, descent, ascent);
            }
            ascent(node);
        }

        dfs(&self.root, &mut descent, &mut ascent);
    }
}
