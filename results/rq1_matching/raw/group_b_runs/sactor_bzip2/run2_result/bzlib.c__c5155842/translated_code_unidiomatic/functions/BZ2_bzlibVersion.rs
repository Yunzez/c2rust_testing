pub unsafe fn BZ2_bzlibVersion() -> *const libc::c_char {
    b"1.0.8, 13-Jul-2019\0".as_ptr() as *const libc::c_char
}
