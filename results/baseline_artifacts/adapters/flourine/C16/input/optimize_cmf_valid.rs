

include!["/input/optimize_cmf_valid_raw.inc"];

#[no_mangle]
pub fn optimize_cmf_valid(data: &mut [u8; 2]) { optimize_cmf(&mut data[..], 2); }
