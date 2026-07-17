#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use std::any::Any;
use quadtree::{quadtree_new, quadtree_insert, quadtree_search};
fuzz_target!(|data: &[u8]| {
    let mut tree = quadtree_new(0.0, 0.0, 1000.0, 1000.0);
    for ch in data.chunks_exact(16).take(64) {
        let x = (f64::from_le_bytes(ch[0..8].try_into().unwrap()) % 1000.0).abs();
        let y = (f64::from_le_bytes(ch[8..16].try_into().unwrap()) % 1000.0).abs();
        if !x.is_finite() || !y.is_finite() { continue; }
        let _ = quadtree_insert(tree.as_deref_mut(), x, y, Some(Box::new(0u8) as Box<dyn Any>));
        let _ = quadtree_search(tree.as_deref(), x, y);
    }
});
