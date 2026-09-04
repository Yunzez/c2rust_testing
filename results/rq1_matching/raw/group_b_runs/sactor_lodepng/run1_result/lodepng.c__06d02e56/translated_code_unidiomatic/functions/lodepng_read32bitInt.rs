pub fn lodepng_read32bitInt(buffer: *const u8) -> u32 {
    unsafe {
        ((*buffer.offset(0) as u32) << 24)
            | ((*buffer.offset(1) as u32) << 16)
            | ((*buffer.offset(2) as u32) << 8)
            | (*buffer.offset(3) as u32)
    }
}
