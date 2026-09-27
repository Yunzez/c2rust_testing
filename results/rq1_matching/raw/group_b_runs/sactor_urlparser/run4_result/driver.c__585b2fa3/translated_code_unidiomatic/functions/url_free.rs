pub unsafe fn url_free(data: *mut url_data) {
    if data.is_null() {
        return;
    }
    use libc::free;
    unsafe fn free_if_not_null(ptr: *mut ::core::ffi::c_char) {
        if !ptr.is_null() {
            free(ptr as *mut libc::c_void);
        }
    }
    free_if_not_null((*data).auth);
    free_if_not_null((*data).protocol);
    free_if_not_null((*data).hostname);
    free_if_not_null((*data).host);
    free_if_not_null((*data).pathname);
    free_if_not_null((*data).path);
    free_if_not_null((*data).hash);
    free_if_not_null((*data).search);
    free_if_not_null((*data).query);
}
