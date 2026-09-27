include!["/input/optimize_cmf_packet_raw.inc"];

#[no_mangle]
pub fn optimize_cmf_packet(data: &mut [u8; 2], data_size: usize) {
    optimize_cmf(data, data_size);
}
