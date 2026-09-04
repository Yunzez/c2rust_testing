// SACTOR unidiomatic translation of `HuffmanTree_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:40:04; attempt 1). Verification verdict: Error: Function signature not found in the translated code for function `searchCodeIndex`. Got functions: ['search_code_index'], check if you have the correct function name., you should **NOT** change
pub unsafe fn HuffmanTree_init(tree: *mut HuffmanTree) {
    if !tree.is_null() {
        (*tree).codes = core::ptr::null_mut();
        (*tree).lengths = core::ptr::null_mut();
        (*tree).table_len = core::ptr::null_mut();
        (*tree).table_value = core::ptr::null_mut();
    }
}
