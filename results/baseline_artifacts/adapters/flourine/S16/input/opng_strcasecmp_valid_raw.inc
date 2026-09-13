fn opng_strcasecmp(str1: *const std::os::raw::c_char, str2: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ch1: std::os::raw::c_int;
    let mut ch2: std::os::raw::c_int;

    let c_str1 = unsafe { std::ffi::CStr::from_ptr(str1) };
    let c_str2 = unsafe { std::ffi::CStr::from_ptr(str2) };

    let str1_lossy = c_str1.to_string_lossy();
    let str2_lossy = c_str2.to_string_lossy();

    let mut iter1 = str1_lossy.chars();
    let mut iter2 = str2_lossy.chars();

    loop {
        ch1 = iter1.next().map_or(0, |c| c.to_ascii_lowercase() as std::os::raw::c_int);
        ch2 = iter2.next().map_or(0, |c| c.to_ascii_lowercase() as std::os::raw::c_int);

        if ch1 != ch2 {
            return ch1 - ch2;
        }
        if ch1 == 0 {
            return 0;
        }
    }
}
