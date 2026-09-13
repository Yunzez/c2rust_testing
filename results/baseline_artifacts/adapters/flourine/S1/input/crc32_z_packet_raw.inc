pub fn crc32_z(crc: u64, buf: &[u8], len: usize) -> u64 {
    if buf.is_empty() { return 0 }

    let mut crc = crc ^ 0xffffffff;
    let mut len = len;

    let mut index = 0;

    while len >= 8 {
        for _ in 0..8 {
            crc = unsafe { crc_table[0][((crc as u32 ^ buf[index] as u32) & 0xff) as usize] as u64 ^ (crc >> 8) };
            index += 1;
        }
        len -= 8;
    }

    for _ in 0..len {
        crc = unsafe { crc_table[0][((crc as u32 ^ buf[index] as u32) & 0xff) as usize] as u64 ^ (crc >> 8) };
        index += 1;
    }

    crc ^ 0xffffffff
}
