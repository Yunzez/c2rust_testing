fn endsInBz2(name: *mut std::os::raw::c_char) -> i32 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let str_slice = c_str.to_str().unwrap();
    let n = str_slice.len();
    if n <= 4 { return 0; }
    if str_slice.ends_with(".bz2") { return 1; }
    0
}
