use libc::{c_char, c_int, malloc, strcpy, strlen};
pub unsafe fn strdup(str_ptr: *const c_char) -> *mut c_char {
    let n: c_int = strlen(str_ptr) as c_int + 1;
    let dup: *mut c_char = malloc(n as usize) as *mut c_char;
    if !dup.is_null() {
        strcpy(dup, str_ptr);
    }
    dup
}
