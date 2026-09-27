pub fn lodepng_filesize(filename: *const libc::c_char) -> libc::c_long {
    unsafe {
        let mode = b"rb\0";
        let file: *mut libc::FILE = libc::fopen(filename, mode.as_ptr() as *const libc::c_char);
        if file.is_null() {
            return -1;
        }
        if libc::fseek(file, 0, 2) != 0 {
            libc::fclose(file);
            return -1;
        }
        let mut size: libc::c_long = libc::ftell(file);
        if size == libc::c_long::MAX {
            size = -1;
        }
        libc::fclose(file);
        size
    }
}
