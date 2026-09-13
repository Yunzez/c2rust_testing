

include!["/input/mmed3_packet_raw.inc"];

#[no_mangle]
pub fn mmed3_packet(values: &mut [u8; 3]) -> u8 { mmed3(values[0], values[1], values[2]) }
