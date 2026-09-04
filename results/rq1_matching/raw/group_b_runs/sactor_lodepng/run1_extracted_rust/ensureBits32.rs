// SACTOR unidiomatic translation of `ensureBits32` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:38:46; attempt 1). Verification verdict: rust compiled
#[inline]
pub unsafe fn ensureBits32(reader: *mut LodePNGBitReader, nbits: libc::size_t) {
    #[inline]
    unsafe fn get_byte(base: *const ::core::ffi::c_uchar, idx: libc::size_t) -> u32 {
        *base.add(idx) as u32
    }
    let reader_ref: &mut LodePNGBitReader = &mut *reader;
    let start: libc::size_t = reader_ref.bp >> 3usize;
    let size: libc::size_t = reader_ref.size;
    if start + 4usize < size {
        let base = reader_ref.data;
        let mut buffer: u32 = get_byte(base, start + 0usize)
            | (get_byte(base, start + 1usize) << 8u32)
            | (get_byte(base, start + 2usize) << 16u32)
            | (get_byte(base, start + 3usize) << 24u32);
        buffer >>= (reader_ref.bp & 7usize) as u32;
        buffer
            |= (get_byte(base, start + 4usize) << 24u32)
                << (8u32 - (reader_ref.bp & 7usize) as u32);
        reader_ref.buffer = buffer as ::core::ffi::c_uint;
    } else {
        let base = reader_ref.data;
        let mut buffer: u32 = 0;
        if start + 0usize < size {
            buffer |= get_byte(base, start + 0usize);
        }
        if start + 1usize < size {
            buffer |= get_byte(base, start + 1usize) << 8u32;
        }
        if start + 2usize < size {
            buffer |= get_byte(base, start + 2usize) << 16u32;
        }
        if start + 3usize < size {
            buffer |= get_byte(base, start + 3usize) << 24u32;
        }
        buffer >>= (reader_ref.bp & 7usize) as u32;
        reader_ref.buffer = buffer as ::core::ffi::c_uint;
    }
    let _ = nbits;
}
