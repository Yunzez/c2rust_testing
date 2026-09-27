include!["/input/genann_cached_initialized_raw.inc"];

#[no_mangle]
pub fn genann_cached_initialized(input: &mut [f64; 1]) -> f64 {
    let value = if input[0].is_nan() { 0.0 } else { input[0] };
    unsafe {
        genann_init_sigmoid_lookup(std::ptr::null());
        genann_act_sigmoid_cached(std::ptr::null(), value)
    }
}
