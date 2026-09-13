extern "C" {
    fn strlen(value: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strcpy(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn opng_path_make_backup(mut buffer: * mut std::os::raw::c_char,
                                               mut bufsize: std::os::raw::c_ulong,
                                               mut path: * const std::os::raw::c_char)
 -> * mut std::os::raw::c_char {
    static mut bak_extname: [std::os::raw::c_char; 5] =
        [0,0,0,0,0,]; unsafe fn laertes_init_bak_extname() {
bak_extname = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b".bak\x00")
        };}//;
    if strlen(path).wrapping_add(::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
                                     std::os::raw::c_ulong) > bufsize {
        return 0 as *mut std::os::raw::c_char
    }
    /* OPNG_OS_UNIX and others */
    strcpy(buffer, path);
    strcat(buffer, bak_extname.as_ptr());
    return buffer;
}
