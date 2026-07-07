pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = (args.len() - 1) as std::os::raw::c_int;
    let arg_ptrs: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut raw_args: Vec<*mut std::os::raw::c_char> = arg_ptrs.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    raw_args.push(std::ptr::null_mut());
    
    unsafe {
        let exit_code = main_0(arg_count, raw_args.as_mut_ptr());
        ::std::process::exit(exit_code as i32);
    }
}

