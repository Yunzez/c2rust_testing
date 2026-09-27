extern "C" {
    fn strlen(value: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strcpy(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(dest: *mut std::os::raw::c_char, src: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}

include!["/input/opng_path_make_backup_raw.inc"];

#[no_mangle]
pub fn opng_path_make_backup_valid(buffer: &mut [i8; 32], path: &mut [i8; 16]) {
    buffer[0] = 0;
    path[15] = 0;
    unsafe {
        opng_path_make_backup(buffer.as_mut_ptr(), 32, path.as_ptr());
    }
}
