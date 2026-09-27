include!["/input/opng_strcasecmp_packet_raw.inc"];

#[no_mangle]
pub fn opng_strcasecmp_packet(str1: &mut [i8; 16], str2: &mut [i8; 16]) -> i32 {
    str1[15] = 0;
    str2[15] = 0;
    opng_strcasecmp(str1.as_ptr(), str2.as_ptr())
}
