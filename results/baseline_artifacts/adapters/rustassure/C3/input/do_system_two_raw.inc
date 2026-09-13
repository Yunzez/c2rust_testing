fn do_system(argc: usize, argv: *mut *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let mut cmd = String::new();
    let mut i: usize = 0;

    while i < argc {
        let arg = unsafe { std::ffi::CStr::from_ptr(*argv.offset(i as isize)) };
        if i != 0 {
            cmd.push(' ');
        }
        cmd.push_str(arg.to_str().unwrap());
        i += 1;
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    let retval = String::from_utf8(output.stdout).expect("Failed to convert output to String");
    let c_string = std::ffi::CString::new(retval).expect("Failed to create CString");
    c_string.into_raw()
}
