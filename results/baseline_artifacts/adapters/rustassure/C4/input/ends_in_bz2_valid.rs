

include!["/input/ends_in_bz2_valid_raw.inc"];

#[no_mangle]
pub fn ends_in_bz2_valid(value: &mut [i8; 16]) -> i32 { value[15] = 0; endsInBz2(value.as_mut_ptr()) }
