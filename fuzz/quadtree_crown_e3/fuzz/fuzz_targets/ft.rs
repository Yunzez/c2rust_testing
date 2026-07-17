#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use quadtree_crown_e3::src::src::quadtree as translated;
fuzz_target!(|data: &[u8]| {
    unsafe {
        let tree = translated::quadtree_new(0.0, 0.0, 1000.0, 1000.0);
        if tree.is_null() { return; }
        let mut key: u8 = 0;
        for ch in data.chunks_exact(16).take(64) {
            let x = f64::from_le_bytes(ch[0..8].try_into().unwrap()) % 1000.0;
            let y = f64::from_le_bytes(ch[8..16].try_into().unwrap()) % 1000.0;
            translated::quadtree_insert(tree, x.abs(), y.abs(), &mut key as *mut u8 as *mut std::os::raw::c_void);
            translated::quadtree_search(tree, x.abs(), y.abs());
        }
        translated::quadtree_free(tree);
    }
});
