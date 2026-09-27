pub unsafe fn HuffmanTree_init(tree: *mut HuffmanTree) {
    if !tree.is_null() {
        (*tree).codes = core::ptr::null_mut();
        (*tree).lengths = core::ptr::null_mut();
        (*tree).table_len = core::ptr::null_mut();
        (*tree).table_value = core::ptr::null_mut();
    }
}
