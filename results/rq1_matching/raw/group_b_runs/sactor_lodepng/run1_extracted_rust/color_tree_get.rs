// SACTOR unidiomatic translation of `color_tree_get` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:51:11; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn color_tree_get(
    tree: *mut ColorTree,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> ::core::ffi::c_int {
    let mut tree = tree;
    let mut bit: ::core::ffi::c_int = 0;
    while bit < 8 {
        let i = 8 * (((r >> bit) & 1) as ::core::ffi::c_int)
            + 4 * (((g >> bit) & 1) as ::core::ffi::c_int)
            + 2 * (((b >> bit) & 1) as ::core::ffi::c_int)
            + 1 * (((a >> bit) & 1) as ::core::ffi::c_int);
        if tree.is_null() {
            return -1;
        }
        let child = (*tree).children[i as usize];
        if child.is_null() {
            return -1;
        } else {
            tree = child;
        }
        bit += 1;
    }
    if tree.is_null() { -1 } else { (*tree).index }
}
