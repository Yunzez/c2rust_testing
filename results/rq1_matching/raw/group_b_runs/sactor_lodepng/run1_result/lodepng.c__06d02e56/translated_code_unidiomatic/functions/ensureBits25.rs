#[inline]
pub unsafe fn ensureBits25(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    let reader_ref: &mut LodePNGBitReader = &mut *reader;
    let start: libc::size_t = reader_ref.bp >> 3usize;
    let size: libc::size_t = reader_ref.size;
    unsafe fn get_data_byte(ptr: *const ::core::ffi::c_uchar, idx: libc::size_t) -> u32 {
        *ptr.add(idx) as u32
    }
    if start + 3usize < size {
        let b0 = get_data_byte(reader_ref.data, start + 0);
        let b1 = get_data_byte(reader_ref.data, start + 1);
        let b2 = get_data_byte(reader_ref.data, start + 2);
        let b3 = get_data_byte(reader_ref.data, start + 3);
        reader_ref.buffer = b0 | (b1 << 8u32) | (b2 << 16u32) | (b3 << 24u32);
        reader_ref.buffer >>= (reader_ref.bp & 7usize) as u32;
    } else {
        reader_ref.buffer = 0;
        if start + 0usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 0);
        }
        if start + 1usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 1) << 8u32;
        }
        if start + 2usize < size {
            reader_ref.buffer |= get_data_byte(reader_ref.data, start + 2) << 16u32;
        }
        reader_ref.buffer >>= (reader_ref.bp & 7usize) as u32;
    }
    let _ = nbits;
}
