use libc::{c_char, c_uchar, size_t, FILE};
pub unsafe fn lodepng_save_file(
    buffer: *const c_uchar,
    buffersize: size_t,
    filename: *const c_char,
) -> u32 {
    extern "C" {
        fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
        fn fwrite(
            ptr: *const libc::c_void,
            size: size_t,
            nmemb: size_t,
            stream: *mut FILE,
        ) -> size_t;
        fn fclose(stream: *mut FILE) -> libc::c_int;
    }
    let mode = b"wb\0" as *const u8 as *const c_char;
    let file = fopen(filename, mode);
    if file.is_null() {
        return 79;
    }
    fwrite(buffer as *const libc::c_void, 1, buffersize, file);
    fclose(file);
    0
}
