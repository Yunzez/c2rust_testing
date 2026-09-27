fn main() {
    let mut argv: [*mut std::os::raw::c_char; 2] = [b"test\0".as_ptr() as *mut _, std::ptr::null_mut()];
    let rc = unsafe { genann_crown::src::test::main_0(1, argv.as_mut_ptr()) };
    std::process::exit(rc);
}
