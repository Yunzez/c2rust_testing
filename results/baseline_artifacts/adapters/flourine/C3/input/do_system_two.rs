include!["/input/do_system_two_raw.inc"];

#[no_mangle]
pub fn do_system_two(first: &mut [i8; 16], second: &mut [i8; 16]) -> *mut std::os::raw::c_char {
    first[15] = 0;
    second[15] = 0;
    let mut argv = [first.as_mut_ptr(), second.as_mut_ptr()];
    do_system(2, argv.as_mut_ptr())
}
