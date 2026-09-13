pub type z_crc_t = u32; pub type z_size_t = u64; pub type ptrdiff_t = i64;

include!["/input/crc32_z_packet_raw.inc"];

#[no_mangle]
pub fn crc32_z_packet(packet: &mut [u8; 25]) -> u64 {
    let crc = u64::from_ne_bytes(packet[0..8].try_into().unwrap());
    let len = (packet[24] % 17) as u64;
    unsafe { crc32_z(crc, packet[8..].as_ptr(), len) }
}
