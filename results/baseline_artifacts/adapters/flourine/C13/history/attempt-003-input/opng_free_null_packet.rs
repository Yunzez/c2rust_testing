

include!["/input/opng_free_null_packet_raw.inc"];

#[no_mangle]
pub fn opng_free_null_packet(_ignored: &mut [u8; 1]) { opng_free(std::ptr::null_mut()); }
