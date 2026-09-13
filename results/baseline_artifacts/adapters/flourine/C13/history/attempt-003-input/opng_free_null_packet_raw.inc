fn opng_free(ptr: *mut std::os::raw::c_void) {
    // This deallocator must be compatible with libpng's memory allocation
    // routines, png_malloc() and png_free().
    // If those routines change, this one must be changed accordingly.
    unsafe {
        drop(Box::from_raw(ptr as *mut _));
    }
}
