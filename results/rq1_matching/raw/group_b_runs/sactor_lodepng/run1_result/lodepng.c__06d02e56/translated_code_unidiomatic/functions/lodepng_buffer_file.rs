pub unsafe fn lodepng_buffer_file(
    out: *mut u8,
    size: libc::size_t,
    filename: *const libc::c_char,
) -> u32 {
    let mut file: *mut libc::FILE;
    file = libc::fopen(filename, b"rb\0".as_ptr() as *const libc::c_char);
    if file.is_null() {
        return 78;
    }
    let readsize = libc::fread(out as *mut libc::c_void, 1, size, file);
    libc::fclose(file);
    if readsize != size {
        return 78;
    }
    0
}
