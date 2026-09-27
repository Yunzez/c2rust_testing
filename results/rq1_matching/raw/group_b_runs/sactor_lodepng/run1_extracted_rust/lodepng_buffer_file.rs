// SACTOR unidiomatic translation of `lodepng_buffer_file` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:36:25; attempt 1). Verification verdict: rust compiled
unsafe fn lodepng_buffer_file(
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
