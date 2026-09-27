#[inline]
pub unsafe fn peekBits(reader: *mut LodePNGBitReader, nbits: libc::size_t) -> libc::c_uint {
    (*reader).buffer & ((1u32 << nbits) - 1u32)
}
