pub unsafe fn lodepng_crc32(data: *const u8, length: usize) -> u32 {
    let mut r: u32 = 0xffffffffu32;
    let mut i: usize = 0;
    while i < length {
        let byte = *data.add(i);
        let index = ((r ^ byte as u32) & 0xffu32) as usize;
        r = lodepng_crc32_table[index] ^ (r >> 8);
        i += 1;
    }
    r ^ 0xffffffffu32
}
