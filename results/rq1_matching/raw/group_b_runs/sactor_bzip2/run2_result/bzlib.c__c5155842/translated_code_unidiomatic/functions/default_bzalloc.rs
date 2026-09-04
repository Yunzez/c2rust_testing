use libc::{c_int, c_void, malloc, size_t};
pub unsafe fn default_bzalloc(opaque: *mut c_void, items: c_int, size: c_int) -> *mut c_void {
    unsafe fn mul_to_size_t(a: c_int, b: c_int) -> size_t {
        (a as size_t).wrapping_mul(b as size_t)
    }
    let total_size: size_t = mul_to_size_t(items, size);
    let v = malloc(total_size);
    v
}
