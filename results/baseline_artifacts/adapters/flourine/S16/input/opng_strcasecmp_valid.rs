

include!["/input/opng_strcasecmp_valid_raw.inc"];

#[no_mangle]
pub fn opng_strcasecmp_valid(left: &mut [i8; 16], right: &mut [i8; 16]) -> i32 { left[15]=0; right[15]=0; opng_strcasecmp(left.as_ptr(),right.as_ptr()) }
