// SACTOR unidiomatic translation of `lodepng_filesize` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:36:05; attempt 1). Verification verdict: rust compiled
fn lodepng_filesize(filename: *const libc::c_char) -> libc::c_long {
    unsafe {
        let mode = b"rb\0";
        let file: *mut libc::FILE = libc::fopen(
            filename,
            mode.as_ptr() as *const libc::c_char,
        );
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
