pub fn readBitFromReversedStream(bitpointer: *mut libc::size_t, bitstream: *const u8) -> u8 {
    unsafe {
        let byte_index = (*bitpointer >> 3) as isize;
        let bit_index = 7 - (*bitpointer & 0x7);
        let byte = *bitstream.offset(byte_index);
        let result = ((byte >> bit_index) & 1) as u8;
        *bitpointer = *bitpointer + 1;
        result
    }
}
