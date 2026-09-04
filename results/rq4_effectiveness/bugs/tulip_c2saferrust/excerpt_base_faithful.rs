unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    if argc < 2 as std::os::raw::c_int {
        printf(b"No indicator given.\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        printf(b"Example:\n\x00" as *const u8 as *const std::os::raw::c_char);
        printf(b"\tsample ma 5\x00" as *const u8 as *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int
    }
    if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
