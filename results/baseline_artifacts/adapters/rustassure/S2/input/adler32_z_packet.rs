pub type uLong = u64; pub type Bytef = u8; pub type z_size_t = u64;

include!["/input/adler32_z_packet_raw.inc"];

#[no_mangle]
pub fn adler32_z_packet(packet: &mut [u8; 25]) -> u64 {
    let adler = u64::from_ne_bytes(packet[0..8].try_into().unwrap());
    let len = (packet[24] % 16) as u64;
    unsafe { adler32_z(adler, packet[8..].as_ptr(), len) }
}
