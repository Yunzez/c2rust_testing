

include!["/input/ti_adx_start_valid_raw.inc"];

#[no_mangle]
pub fn ti_adx_start_valid(options: &mut [f64; 1]) -> i32 { ti_adx_start(options.as_ptr()) }
