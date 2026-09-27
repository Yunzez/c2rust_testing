use libc::{c_void, malloc, size_t};
pub unsafe fn lodepng_malloc(size: size_t) -> *mut c_void {
    malloc(size)
}
