pub extern "C" fn ti_adx_start(options: *const f64) -> i32 {
    unsafe {
        (options.offset(0) as i32 - 1) * 2
    }
}
