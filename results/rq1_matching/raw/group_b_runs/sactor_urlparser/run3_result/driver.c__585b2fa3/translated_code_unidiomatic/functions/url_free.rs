pub unsafe fn url_free(data: *mut url_data) {
    if data.is_null() {
        return;
    }
    extern "C" {
        fn free(ptr: *mut ::core::ffi::c_void);
    }
    if !(*data).auth.is_null() {
        free((*data).auth.cast());
    }
    if !(*data).protocol.is_null() {
        free((*data).protocol.cast());
    }
    if !(*data).hostname.is_null() {
        free((*data).hostname.cast());
    }
    if !(*data).host.is_null() {
        free((*data).host.cast());
    }
    if !(*data).pathname.is_null() {
        free((*data).pathname.cast());
    }
    if !(*data).path.is_null() {
        free((*data).path.cast());
    }
    if !(*data).hash.is_null() {
        free((*data).hash.cast());
    }
    if !(*data).search.is_null() {
        free((*data).search.cast());
    }
    if !(*data).query.is_null() {
        free((*data).query.cast());
    }
}
