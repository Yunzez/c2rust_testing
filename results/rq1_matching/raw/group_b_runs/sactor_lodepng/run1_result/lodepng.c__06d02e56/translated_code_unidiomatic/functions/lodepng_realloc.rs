use libc::{c_void, realloc, size_t};
#[inline]
pub unsafe fn lodepng_realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void {
    #[cfg(LODEPNG_MAX_ALLOC)]
    {
        extern "C" {
            static LODEPNG_MAX_ALLOC: size_t;
        }
        if new_size > LODEPNG_MAX_ALLOC {
            return core::ptr::null_mut();
        }
    }
    realloc(ptr, new_size)
}
