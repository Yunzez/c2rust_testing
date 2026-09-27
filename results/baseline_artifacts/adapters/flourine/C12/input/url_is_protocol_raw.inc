#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: * mut std::os::raw::c_char) -> bool {
    let mut count: i32 =
        (::std::mem::size_of::<[*mut std::os::raw::c_char; 177]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut i: i32 = 0 as std::os::raw::c_int;
    while i < count {
        if 0 as std::os::raw::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as std::os::raw::c_int != 0
        }
        i += 1
    }
    return 0 as std::os::raw::c_int != 0;
}
