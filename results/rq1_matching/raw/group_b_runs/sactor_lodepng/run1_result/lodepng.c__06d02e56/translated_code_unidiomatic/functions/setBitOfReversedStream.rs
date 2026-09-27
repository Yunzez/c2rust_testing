use libc::size_t;
pub fn setBitOfReversedStream(bitpointer: *mut size_t, bitstream: *mut u8, bit: u8) {
    unsafe {
        let index = (*bitpointer >> 3) as isize;
        let ptr = bitstream.offset(index);
        let bit_pos = 7u8 - (((*bitpointer) & 7) as u8);
        if bit == 0 {
            *ptr &= !(1u8 << bit_pos);
        } else {
            *ptr |= 1u8 << bit_pos;
        }
        *bitpointer += 1;
    }
}
